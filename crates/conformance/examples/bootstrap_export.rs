//! One persistent bootstrap export through the public consumer; no broad export sweep.
//! Build with LEXLEAN_DIAGNOSTIC_SOURCE equal to the clean source checkpoint.
use camino::Utf8PathBuf;
use repo_conformance::{production_consumer, production_export};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

type Failure = Box<dyn std::error::Error>;

fn baked_source() -> Result<&'static str, Failure> {
    option_env!("LEXLEAN_DIAGNOSTIC_SOURCE").ok_or_else(|| {
        "build requires LEXLEAN_DIAGNOSTIC_SOURCE set to the clean source checkpoint".into()
    })
}

fn sha(path: &Path) -> Result<String, Failure> {
    let mut file = std::fs::File::open(path)?;
    let mut hash = Sha256::new();
    let mut buffer = [0_u8; 65536];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hash.update(&buffer[..count]);
    }
    Ok(format!("{:x}", hash.finalize()))
}

fn git(root: &Path, arguments: &[&str]) -> Result<String, Failure> {
    let result = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(arguments)
        .output()?;
    if !result.status.success() {
        return Err(format!(
            "Git identity query failed: {}",
            String::from_utf8_lossy(&result.stderr)
        )
        .into());
    }
    Ok(String::from_utf8(result.stdout)?.trim().to_owned())
}

fn save(path: &Path, value: &Value) -> Result<(), Failure> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    std::fs::write(path, bytes)?;
    Ok(())
}

// Only top-level generated directories are omitted. Walk errors and special files
// fail closed rather than silently making an incomplete persistent source copy.
fn copy_project(from: &Path, to: &Path) -> Result<(), Failure> {
    for entry in walkdir::WalkDir::new(from).follow_links(false).into_iter().filter_entry(|entry| {
        let relative = entry.path().strip_prefix(from).expect("walk is below source");
        !matches!(relative.components().next(), Some(Component::Normal(name)) if name == "expected" || name == ".lexlean")
    }) {
        let entry = entry?;
        let destination = to.join(entry.path().strip_prefix(from)?);
        if entry.file_type().is_dir() {
            std::fs::create_dir_all(&destination)?;
        } else if entry.file_type().is_file() {
            std::fs::copy(entry.path(), &destination)?;
        } else {
            return Err(format!("source copy refuses special file: {}", entry.path().display()).into());
        }
    }
    Ok(())
}

fn inventory(root: &Path) -> Result<BTreeMap<String, String>, Failure> {
    let mut files = BTreeMap::new();
    for entry in walkdir::WalkDir::new(root).follow_links(false) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let relative = entry
                .path()
                .strip_prefix(root)?
                .to_str()
                .ok_or("non-UTF-8 source path")?;
            files.insert(
                relative.replace(std::path::MAIN_SEPARATOR, "/"),
                sha(entry.path())?,
            );
        } else if !entry.file_type().is_dir() {
            return Err("special file in persistent project".into());
        }
    }
    Ok(files)
}

fn export(root: &Path, source: &Path) -> Result<(), Failure> {
    git(
        source,
        &["ls-files", "--error-unmatch", "bootstrap/lexlean.lock"],
    )?;
    let project = root.join("workspace");
    std::fs::create_dir(&project)?;
    for member in ["bootstrap", "stdlib"] {
        copy_project(&source.join(member), &project.join(member))?;
    }
    std::fs::copy(
        source.join("lexlean-workspace.toml"),
        project.join("lexlean-workspace.toml"),
    )?;
    let inputs = inventory(&project)?;
    save(&root.join("source-inputs.json"), &json!(inputs))?;
    let application_path = source.join("tests/production-export/bootstrap/roots.json");
    std::fs::copy(&application_path, root.join("application.json"))?;
    let application = production_export::application("bootstrap");
    let config = Utf8PathBuf::from_path_buf(project.join("bootstrap/lexlean.toml"))
        .map_err(|_| "non-UTF-8 project path")?;
    // setup checks the committed lock with allow_network=false; it never relocks.
    let (_engine, pins) = production_consumer::setup(&config)?;
    let loaded = lexlean::project::Project::load(&config)?;
    let toolchain = lexlean::verify::toolchain::preflight(&loaded.config.limits)
        .map_err(|diagnostic| format!("toolchain preflight: {diagnostic:?}"))?;
    let tool = |tool: &lexlean::verify::toolchain::Tool| {
        json!({
            "path": tool.path, "sha256": tool.sha256.to_hex(), "version_output": tool.version_output
        })
    };
    let source_checkpoint = git(source, &["rev-parse", "HEAD"])?;
    if source_checkpoint != baked_source()?
        || !git(
            source,
            &["status", "--porcelain", "--untracked-files=normal"],
        )?
        .is_empty()
    {
        return Err("source checkout moved before export".into());
    }
    let receipt = json!({
        "status": "preflight-passed-export-not-started", "compiler_accepted": false,
        "source_checkpoint": source_checkpoint, "source_tree": git(source, &["rev-parse", "HEAD^{tree}"])?,
        "executable_sha256": sha(&std::env::current_exe()?)?,
        "compiler_semantics": lexlean::compiler_semantics_id_for("1.2").to_hex(),
        "lean_toolchain": lexlean::LEAN_TOOLCHAIN,
        "tools": {"lean": tool(&toolchain.lean), "lake": tool(&toolchain.lake), "leanchecker": tool(&toolchain.leanchecker)},
        "source_inputs_sha256": sha(&root.join("source-inputs.json"))?,
        "application_sha256": sha(&root.join("application.json"))?,
        "lock_sha256": sha(&project.join("bootstrap/lexlean.lock"))?,
        "request": production_consumer::request(&application, &pins, &[]),
        "telemetry_directory": root.join("telemetry"),
        "scope": "bootstrap-only public production export; no compiler adoption"
    });
    save(&root.join("preflight.json"), &receipt)?;
    // The public consumer calls production::build (including complete verify)
    // then verify_artifact. Do not run standalone verify or replay either again.
    match production_consumer::export(&config, &application, &[]) {
        Ok((output, replay)) => {
            std::fs::write(root.join("production-result.json"), output.result.to_json())?;
            save(
                &root.join("bundle-replay.json"),
                &serde_json::to_value(&replay)?,
            )?;
            for (relative, expected) in &inputs {
                if sha(&project.join(relative))? != *expected {
                    return Err(format!("source input changed during export: {relative}").into());
                }
            }
            save(
                &root.join("outcome.json"),
                &json!({
                    "status": "bootstrap-production-export-and-bundle-replay-passed",
                    "compiler_accepted": false, "preflight_sha256": sha(&root.join("preflight.json"))?,
                    "result_sha256": sha(&root.join("production-result.json"))?,
                    "replay_sha256": sha(&root.join("bundle-replay.json"))?,
                    "bundle": project.join("bootstrap").join(output.bundle),
                    "attestation_id": output.result.verification.attestation_id.to_hex(),
                    "not_qualified": ["other exported projects", "full resulting-head CI", "compiler release or downstream pin adoption"]
                }),
            )?;
            Ok(())
        }
        Err(error) => {
            save(
                &root.join("export-failure.json"),
                &json!({
                    "status": "bootstrap-production-export-failed", "compiler_accepted": false,
                    "class": error.class.as_str(), "exit_code": error.class.exit_code(),
                    "diagnostics": format!("{:?}", error.diagnostics),
                    "preflight_sha256": sha(&root.join("preflight.json"))?
                }),
            )?;
            Err(error.into())
        }
    }
}

fn main() -> Result<(), Failure> {
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("crate parent")?
        .parent()
        .ok_or("repository parent")?
        .canonicalize()?;
    if !git(
        &source,
        &["status", "--porcelain", "--untracked-files=normal"],
    )?
    .is_empty()
        || git(&source, &["rev-parse", "HEAD"])? != baked_source()?
    {
        return Err("export requires clean checkout at the exact baked source checkpoint".into());
    }
    let mut args = std::env::args_os().skip(1);
    let requested = PathBuf::from(
        args.next()
            .ok_or("new absolute output directory required")?,
    );
    if args.next().is_some() || !requested.is_absolute() {
        return Err("one absolute output directory required".into());
    }
    let output = requested
        .parent()
        .ok_or("output parent")?
        .canonicalize()?
        .join(requested.file_name().ok_or("output name")?);
    if output.starts_with(&source) {
        return Err("persistent output must be outside source checkout".into());
    }
    std::fs::create_dir(&output)?;
    std::fs::create_dir(output.join("telemetry"))?;
    // This standalone executable starts no other threads before configuring telemetry.
    std::env::set_var(
        lexlean::verify::operational::TELEMETRY_VARIABLE,
        output.join("telemetry"),
    );
    save(
        &output.join("started.json"),
        &json!({
            "status": "started-no-acceptance", "source_checkpoint": baked_source()?,
            "executable_sha256": sha(&std::env::current_exe()?)?, "compiler_accepted": false
        }),
    )?;
    match export(&output, &source) {
        Ok(()) => Ok(()),
        Err(error) => {
            save(
                &output.join("outcome.json"),
                &json!({
                    "status": "failed", "compiler_accepted": false, "error": error.to_string(),
                    "preserved_workspace": output.join("workspace"), "telemetry_directory": output.join("telemetry")
                }),
            )?;
            Err(error)
        }
    }
}

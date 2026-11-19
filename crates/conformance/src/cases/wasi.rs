//! The `cli-api` suite, `check` on WASI: CL-25 (SPEC.md §8.3).
//!
//! The executable is built for `wasm32-wasip1` from this tree and run by
//! Node's `node:wasi`, a WASI host over the real file system, so a project
//! is read through the same paths, confinement checks, and file identities
//! as on the native host. The native side is `lexlean::cli::run`, the
//! function `main` calls, as the fixture runner runs it (§28.2).

use std::process::Command;

use camino::{Utf8Path, Utf8PathBuf};

use crate::fixtures;
use crate::support::{self, P};

const TARGET: &str = "wasm32-wasip1";

/// The WASI host: the module, the directory it sees as `/`, then the
/// arguments after `lexlean`. The project is the root of the instance's file
/// system because a WASI program starts in `/` and LexLean looks for
/// `lexlean.toml` from where it starts (§23.2). The environment is empty, as
/// it is for a native run that sets nothing (§25.4).
const HOST: &str = r#"import { readFile } from "node:fs/promises";
import { WASI } from "node:wasi";
const [module, root, ...args] = process.argv.slice(2);
const wasi = new WASI({
  version: "preview1",
  args: ["lexlean", ...args],
  env: {},
  preopens: { "/": root },
  returnOnExit: true,
});
const instance = await WebAssembly.instantiate(
  await WebAssembly.compile(await readFile(module)),
  wasi.getImportObject(),
);
process.exitCode = wasi.start(instance);
"#;

/// What one run produced: the exit code, stdout, and stderr.
type Run = (i32, String, String);

/// Is there a WASI host and the pinned toolchain's standard library for the
/// target? On the normative host both must be present (`rust-toolchain.toml`
/// names the target, and CI installs Node's `node:wasi` with the runner); on
/// any other host the case says it did not run, never passes vacuously.
fn wasi_backed(id: &str) -> bool {
    let node = Command::new("node")
        .args(["--input-type=module", "-e", "await import('node:wasi')"])
        .output()
        .is_ok_and(|output| output.status.success());
    let target = Command::new("rustc")
        .args(["--print", "target-libdir", "--target", TARGET])
        .current_dir(support::repo_root().as_std_path())
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .is_some_and(|dir| Utf8Path::new(dir.trim()).is_dir());
    if node && target {
        return true;
    }
    assert!(
        !support::normative_host(),
        "{id}: the normative host (Linux x86-64) must have `node` with `node:wasi` (found: {node}) and the pinned toolchain's {TARGET} standard library (found: {target}); this case never passes vacuously here (§8.3)"
    );
    eprintln!(
        "{id}: no WASI host or no {TARGET} standard library on this host; nothing was run (§8.3)"
    );
    false
}

/// Build the executable for the target from this tree. It is the release
/// profile the executable ships with, in the workspace's target directory,
/// where nothing else builds for this target.
fn build() -> Utf8PathBuf {
    let target_dir = std::env::var("CARGO_TARGET_DIR")
        .map_or_else(|_| support::repo_root().join("target"), Utf8PathBuf::from);
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let output = Command::new(cargo)
        .args([
            "build",
            "--locked",
            "--release",
            "--target",
            TARGET,
            "-p",
            "lexlean",
            "--bin",
            "lexlean",
        ])
        .current_dir(support::repo_root().as_std_path())
        .output()
        .expect("cargo runs");
    assert!(
        output.status.success(),
        "CL-25: lexlean does not build for {TARGET}:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let module = target_dir.join(TARGET).join("release/lexlean.wasm");
    assert!(module.is_file(), "CL-25: {module} was built");
    module
}

/// Run the module on `root` under the WASI host.
fn wasi(module: &Utf8Path, host: &Utf8Path, root: &Utf8Path, arguments: &[&str]) -> Run {
    // Node 22 collects garbage inside its fast call of WASI's
    // `path_filestat_get` and dies with SIGSEGV on a language-1.2 project;
    // the ordinary call path does not.
    let output = Command::new("node")
        .args(["--no-warnings", "--no-turbo-fast-api-calls"])
        .arg(host.as_str())
        .arg(module.as_str())
        .arg(root.as_str())
        .args(arguments)
        .output()
        .expect("node runs");
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

/// Run the executable natively on `root`, holding the environment lock as
/// the fixture runner does.
fn native(root: &Utf8Path, arguments: &[&str]) -> Run {
    let _guard = support::env_shared();
    support::cli_in(root, arguments)
}

/// A run with the project's own location written as `$PROJECT`: the native
/// run sees its temporary directory, the instance sees `/`.
fn normalized((exit, stdout, stderr): Run, root: &str) -> Run {
    let root = root.trim_end_matches('/');
    let replace = |text: &str| {
        if root.is_empty() {
            text.replace("\"/", "\"$PROJECT/")
        } else {
            text.replace(root, "$PROJECT")
        }
    };
    (exit, replace(&stdout), replace(&stderr))
}

/// A copy of `source` for one side of a comparison, symlinks kept.
fn copy(source: &Utf8Path) -> P {
    let project = P::empty();
    fixtures::copy_tree_preserving_links(source.as_std_path(), project.root.as_std_path())
        .expect("the project copies");
    for generated in [".lexlean", "expected"] {
        let path = project.root.join(generated);
        if path.is_dir() {
            std::fs::remove_dir_all(path.as_std_path()).expect("remove a generated directory");
        }
    }
    project
}

/// One project to check on both hosts: its source and the `check` arguments.
struct Subject {
    name: String,
    source: Utf8PathBuf,
    args: Vec<String>,
}

/// Every single-step `check` fixture that needs no toolchain, overlay, or
/// environment of its own, and every example project.
fn subjects() -> Vec<Subject> {
    let root = support::repo_root();
    let mut subjects = Vec::new();
    for dir in fixtures::discover(&root) {
        let case = fixtures::load_case(&dir).expect("a fixture loads");
        let [invocation] = case.invocations.as_slice() else {
            continue;
        };
        if invocation.command != "check"
            || !case.environment.is_empty()
            || dir.join("toolchain").exists()
            || dir.join("overlay").exists()
        {
            continue;
        }
        subjects.push(Subject {
            name: dir.strip_prefix(&root).unwrap_or(&dir).to_string(),
            source: dir.join("project"),
            args: invocation.args.clone(),
        });
    }
    let fixtures = subjects.len();
    assert!(
        fixtures >= 100,
        "CL-25: {fixtures} check fixtures were found"
    );
    let mut examples: Vec<Utf8PathBuf> = std::fs::read_dir(root.join("examples").as_std_path())
        .expect("examples")
        .flatten()
        .filter(|entry| entry.path().join("lexlean.toml").is_file())
        .filter_map(|entry| Utf8PathBuf::from_path_buf(entry.path()).ok())
        .collect();
    examples.sort();
    for source in examples {
        subjects.push(Subject {
            name: source.strip_prefix(&root).unwrap_or(&source).to_string(),
            source,
            args: Vec::new(),
        });
    }
    assert!(subjects.len() > fixtures, "CL-25: the examples were found");
    subjects
}

/// Check `subject` on both hosts and report any difference.
fn compare(module: &Utf8Path, host: &Utf8Path, subject: &Subject) -> Option<String> {
    let mut arguments = vec!["--diagnostic-format", "json", "check"];
    arguments.extend(subject.args.iter().map(String::as_str));
    let left = copy(&subject.source);
    let right = copy(&subject.source);
    let on_native = normalized(native(&left.root, &arguments), left.root.as_str());
    let on_wasi = normalized(wasi(module, host, &right.root, &arguments), "/");
    (on_native != on_wasi).then(|| {
        format!(
            "{}: native exit {} stdout {} stderr {}\n  wasi exit {} stdout {} stderr {}",
            subject.name, on_native.0, on_native.1, on_native.2, on_wasi.0, on_wasi.1, on_wasi.2
        )
    })
}

/// CL-25.
pub(crate) fn cli() {
    if !wasi_backed("CL-25") {
        return;
    }
    let module = build();
    let scratch = tempfile::Builder::new()
        .prefix("lexlean-wasi-host-")
        .tempdir()
        .expect("tempdir");
    let host = Utf8PathBuf::from_path_buf(scratch.path().join("host.mjs")).expect("utf8 path");
    std::fs::write(host.as_std_path(), HOST).expect("write the host");

    // The same exit code, command result, and diagnostics on every project.
    let subjects = subjects();
    let next = std::sync::atomic::AtomicUsize::new(0);
    let differences = std::sync::Mutex::new(Vec::new());
    let workers = std::thread::available_parallelism()
        .map_or(1, usize::from)
        .min(8);
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| loop {
                let at = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let Some(subject) = subjects.get(at) else {
                    break;
                };
                if let Some(difference) = compare(&module, &host, subject) {
                    differences
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .push(difference);
                }
            });
        }
    });
    let differences = differences
        .into_inner()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    assert!(
        differences.is_empty(),
        "CL-25: {} of {} projects differ between the native and the WASI executable:\n{}",
        differences.len(),
        subjects.len(),
        differences.join("\n")
    );
    eprintln!(
        "CL-25: {} projects agree between the native and the WASI executable",
        subjects.len()
    );

    // Two names of one file are refused on WASI too (§25.1): WASI reports the
    // device and inode that tell them apart.
    let example = support::repo_root().join("examples/nat-add-zero");
    let linked = |project: &P| {
        std::fs::hard_link(
            project.root.join("src/Main.lex.tex").as_std_path(),
            project.root.join("src/Twin.lex.tex").as_std_path(),
        )
        .expect("a hard link");
    };
    let left = copy(&example);
    let right = copy(&example);
    linked(&left);
    linked(&right);
    let arguments = ["--diagnostic-format", "json", "check"];
    let on_native = normalized(native(&left.root, &arguments), left.root.as_str());
    let on_wasi = normalized(wasi(&module, &host, &right.root, &arguments), "/");
    assert!(
        on_native.1.contains("LLC0104") && on_native.1.contains("are the same file"),
        "CL-25: the native executable refuses a hard-linked module: {on_native:?}"
    );
    assert_eq!(on_wasi, on_native, "CL-25: a hard-linked module on WASI");

    // A command that changes the project is refused before it touches it:
    // WASI has no file lock (§21.8).
    let project = copy(&example);
    let before = support::file_set(&project.root);
    let (exit, stdout, stderr) = wasi(
        &module,
        &host,
        &project.root,
        &["--diagnostic-format", "json", "fmt"],
    );
    assert_eq!(exit, 3, "CL-25: fmt on WASI exits 3: {stdout}{stderr}");
    assert!(
        stdout.contains("LLV7010") && stdout.contains("this host has no file lock"),
        "CL-25: fmt on WASI is refused with LLV7010: {stdout}{stderr}"
    );
    assert_eq!(
        support::file_set(&project.root),
        before,
        "CL-25: a refused fmt leaves the project as it was"
    );

    // The depth the linked stack hosts is enforced as on every host: input
    // within it checks as it does natively, and input beyond it is LLS8002,
    // never a trap, however deep the project allows (§8.3, §25.5).
    let hosted = lexlean::config::Limits {
        max_scope_depth: u64::MAX,
        ..support::limits_of(&P::example())
    };
    let hosted = usize::try_from(hosted.compile_stack_plan().effective_max_scope_depth)
        .expect("the native plan hosts a depth");
    assert!(
        hosted > 1024,
        "CL-25: the native host hosts more than WASI's 1,024"
    );
    for (nesting, refused) in [(1000, false), (1040, true)] {
        let project = copy(&example);
        let nested = format!("{}n{}", "(".repeat(nesting), ")".repeat(nesting));
        for (file, from, to) in [
            (
                "lexlean.toml",
                "max_scope_depth = 1024".to_owned(),
                "max_scope_depth = 100000000".to_owned(),
            ),
            (
                "src/Main.lex.tex",
                r"\(n + 0 = n\)".to_owned(),
                format!(r"\({nested} + 0 = n\)"),
            ),
        ] {
            let path = project.root.join(file);
            let text = std::fs::read_to_string(path.as_std_path()).expect("a project file");
            assert!(text.contains(&from), "CL-25: {file} holds {from}");
            std::fs::write(path.as_std_path(), text.replace(&from, &to)).expect("edit");
        }
        let locked = native(&project.root, &["lock"]);
        assert_eq!(
            locked.0, 0,
            "CL-25: the deep project locks natively: {locked:?}"
        );
        let arguments = ["--diagnostic-format", "json", "check"];
        let on_native = normalized(native(&project.root, &arguments), project.root.as_str());
        let on_wasi = normalized(wasi(&module, &host, &project.root, &arguments), "/");
        assert_eq!(
            on_native.0, 0,
            "CL-25: nesting {nesting} checks natively: {on_native:?}"
        );
        if refused {
            assert_eq!(
                on_wasi.0, 4,
                "CL-25: nesting {nesting} on WASI exits 4: {on_wasi:?}"
            );
            assert!(
                on_wasi.1.contains("LLS8002") && on_wasi.1.contains("max_scope_depth exceeded"),
                "CL-25: nesting {nesting} on WASI is LLS8002: {on_wasi:?}"
            );
        } else {
            assert_eq!(on_wasi, on_native, "CL-25: nesting {nesting} on WASI");
        }
    }
}

/// The example projects that check: every directory of `examples/` with a
/// `lexlean.toml` whose native `check` exits 0.
fn checking_examples() -> Vec<Utf8PathBuf> {
    let root = support::repo_root();
    let mut examples: Vec<Utf8PathBuf> = std::fs::read_dir(root.join("examples").as_std_path())
        .expect("examples")
        .flatten()
        .filter(|entry| entry.path().join("lexlean.toml").is_file())
        .filter_map(|entry| Utf8PathBuf::from_path_buf(entry.path()).ok())
        .collect();
    examples.sort();
    examples
        .into_iter()
        .filter(|source| native(&copy(source).root, &["check"]).0 == 0)
        .collect()
}

/// CL-26.
pub(crate) fn snapshot() {
    use sha2::{Digest, Sha256};

    let examples = checking_examples();
    assert!(
        examples.len() >= 3,
        "CL-26: {} example projects check",
        examples.len()
    );
    let mut native_bytes = Vec::new();
    for source in &examples {
        let project = copy(source);
        let before = support::file_set(&project.root);
        // Human mode: stdout is exactly the snapshot's canonical file bytes,
        // the bytes the public API serializes, and nothing else.
        let (exit, stdout, stderr) = native(&project.root, &["snapshot"]);
        assert_eq!(exit, 0, "CL-26: {source}: snapshot exits 0: {stderr}");
        assert!(
            stderr.is_empty(),
            "CL-26: {source}: nothing on stderr: {stderr}"
        );
        let engine = project.engine();
        let expected = engine
            .snapshot(lexlean::api::CheckRequest {
                selection: lexlean::api::Selection::Entrypoints,
            })
            .expect("the snapshot of a project that checks");
        assert_eq!(
            stdout.as_bytes(),
            expected.canonical_bytes().as_slice(),
            "CL-26: {source}: stdout is the snapshot's canonical bytes"
        );
        assert_eq!(
            Sha256::digest(stdout.as_bytes()).as_slice(),
            expected.snapshot_id().0.as_slice(),
            "CL-26: {source}: the snapshot ID is the SHA-256 of stdout"
        );
        // The same IDs `check` reports, and the selection's own spec.
        let (_, checked, _) = native(&project.root, &["--diagnostic-format", "json", "check"]);
        let checked: serde_json::Value = serde_json::from_str(&checked).expect("check JSON");
        let document: serde_json::Value = serde_json::from_str(&stdout).expect("snapshot JSON");
        assert_eq!(
            document["semantic_id"], checked["semantic_id"],
            "CL-26: {source}: semantic ID"
        );
        assert_eq!(
            document["source_id"], checked["source_id"],
            "CL-26: {source}: source ID"
        );
        assert_eq!(
            document["spec"],
            serde_json::Value::from(lexlean::artifact::snapshot::snapshot_spec(
                expected.language()
            )),
            "CL-26: {source}: the snapshot spec of the project's language"
        );
        // JSON mode: one command result, as `check` reports it, under `snapshot`.
        let (exit, result, _) = native(&project.root, &["--diagnostic-format", "json", "snapshot"]);
        let result: serde_json::Value = serde_json::from_str(&result).expect("command result");
        assert_eq!(exit, 0, "CL-26: {source}: json snapshot exits 0");
        assert_eq!(
            result["command"], "snapshot",
            "CL-26: {source}: the command result names snapshot"
        );
        assert_eq!(
            result["semantic_id"], checked["semantic_id"],
            "CL-26: {source}: json semantic ID"
        );
        assert_eq!(
            result["modules"], checked["modules"],
            "CL-26: {source}: json modules"
        );
        // It reads the project and writes nothing, like `check`.
        assert_eq!(
            support::file_set(&project.root),
            before,
            "CL-26: {source}: nothing is written"
        );
        native_bytes.push((source.clone(), stdout));
    }

    // A project `check` refuses gives no snapshot: the same exit code and
    // diagnostics, and nothing on stdout in human mode.
    let refused = copy(&support::repo_root().join("examples/nat-add-zero"));
    std::fs::write(
        refused.root.join("src/Main.lex.tex").as_std_path(),
        "\\begin{lexlean}{Main}\n",
    )
    .expect("write a broken module");
    let (check_exit, check_json, _) =
        native(&refused.root, &["--diagnostic-format", "json", "check"]);
    let (exit, stdout, stderr) = native(&refused.root, &["snapshot"]);
    assert_ne!(
        check_exit, 0,
        "CL-26: the broken module is refused by check"
    );
    assert_eq!(
        exit, check_exit,
        "CL-26: snapshot exits as check does on a refused project"
    );
    assert!(
        stdout.is_empty(),
        "CL-26: a refused project prints no snapshot: {stdout}"
    );
    assert!(
        !stderr.is_empty(),
        "CL-26: a refused project's diagnostics are on stderr"
    );
    let (_, json, _) = native(&refused.root, &["--diagnostic-format", "json", "snapshot"]);
    let json: serde_json::Value = serde_json::from_str(&json).expect("command result");
    let check_json: serde_json::Value = serde_json::from_str(&check_json).expect("command result");
    assert_eq!(
        json["diagnostics"], check_json["diagnostics"],
        "CL-26: the same diagnostics as check"
    );

    // On WASI the snapshot is the same bytes: it holds no path or host data
    // (§24.1), so a browser worker gets what the native executable prints.
    if !wasi_backed("CL-26") {
        return;
    }
    let module = build();
    let scratch = tempfile::Builder::new()
        .prefix("lexlean-wasi-snapshot-")
        .tempdir()
        .expect("tempdir");
    let host = Utf8PathBuf::from_path_buf(scratch.path().join("host.mjs")).expect("utf8 path");
    std::fs::write(host.as_std_path(), HOST).expect("write the host");
    for (source, bytes) in &native_bytes {
        let project = copy(source);
        let (exit, stdout, stderr) = wasi(&module, &host, &project.root, &["snapshot"]);
        assert_eq!(
            exit, 0,
            "CL-26: {source}: snapshot on WASI exits 0: {stderr}"
        );
        assert!(
            stdout == *bytes,
            "CL-26: {source}: the WASI snapshot is the native snapshot, byte for byte"
        );
    }
    eprintln!(
        "CL-26: {} snapshots agree between the native and the WASI executable",
        native_bytes.len()
    );
}

/// The Lean module names a generated Lean text imports.
fn lean_imports(text: &str) -> Vec<&str> {
    text.lines()
        .filter_map(|line| {
            line.strip_prefix("public import ")
                .or_else(|| line.strip_prefix("import "))
                .map(str::trim)
        })
        .collect()
}

/// CL-27.
#[allow(clippy::too_many_lines)]
pub(crate) fn lean_modules() {
    use lexlean::artifact::content_id::Sha256Digest;

    let examples = checking_examples();
    assert!(
        examples.len() >= 3,
        "CL-27: {} example projects check",
        examples.len()
    );
    let mut native_bytes = Vec::new();
    let mut runtime_seen = false;
    for source in &examples {
        let project = copy(source);
        let before = support::file_set(&project.root);
        // Human mode: one canonical document and nothing else; nothing is
        // written, so no lock is taken and no build root is made.
        let (exit, stdout, stderr) = native(&project.root, &["lean-modules"]);
        assert_eq!(exit, 0, "CL-27: {source}: lean-modules exits 0: {stderr}");
        assert!(
            stderr.is_empty(),
            "CL-27: {source}: nothing on stderr: {stderr}"
        );
        assert_eq!(
            support::file_set(&project.root),
            before,
            "CL-27: {source}: nothing is written"
        );
        let document: serde_json::Value = serde_json::from_str(&stdout).expect("document JSON");
        assert_eq!(
            lexlean::artifact::canonical_json::Json::parse(stdout.as_bytes())
                .expect("canonical JSON")
                .to_file_bytes(),
            stdout.as_bytes(),
            "CL-27: {source}: the document is canonical JSON"
        );
        assert_eq!(
            document["spec"], "lexlean/lean-modules/1",
            "CL-27: {source}"
        );

        // What `build` publishes for the same selection: the same build ID,
        // and for each Lean output of its manifest exactly the bytes and the
        // module the document gives, and the toolchain and compiler it pins.
        let (exit, built, stderr) =
            native(&project.root, &["--diagnostic-format", "json", "build"]);
        assert_eq!(exit, 0, "CL-27: {source}: build exits 0: {stderr}");
        let built: serde_json::Value = serde_json::from_str(&built).expect("build result");
        assert_eq!(
            document["build_id"], built["build_id"],
            "CL-27: {source}: the build ID build publishes"
        );
        let build_dir = project
            .root
            .join(built["artifacts"][0].as_str().expect("the build directory"));
        let manifest: serde_json::Value = serde_json::from_slice(
            &std::fs::read(build_dir.join("manifest.json").as_std_path()).expect("manifest"),
        )
        .expect("manifest JSON");
        for key in [
            "language",
            "lean_toolchain",
            "compiler",
            "source_id",
            "semantic_id",
        ] {
            assert_eq!(
                document[key], manifest[key],
                "CL-27: {source}: `{key}` is the manifest's"
            );
        }
        let rows = document["modules"].as_array().expect("modules");
        let project_rows: Vec<&serde_json::Value> =
            rows.iter().filter(|row| row["kind"] == "project").collect();
        let lean_outputs: Vec<&serde_json::Value> = manifest["outputs"]
            .as_array()
            .expect("outputs")
            .iter()
            .filter(|row| row["kind"] == "lean")
            .collect();
        assert_eq!(
            project_rows.len(),
            lean_outputs.len(),
            "CL-27: {source}: one project row per Lean file build writes"
        );
        for row in &project_rows {
            let lean_module = row["lean_module"].as_str().expect("lean_module");
            let path = format!("modules/{}.lean", lean_module.replace('.', "/"));
            let written =
                std::fs::read(build_dir.join(&path).as_std_path()).expect("the built Lean file");
            let text = row["text"].as_str().expect("text");
            assert!(
                text.as_bytes() == written.as_slice(),
                "CL-27: {source}: {lean_module} is the bytes build writes at {path}"
            );
            let output = lean_outputs
                .iter()
                .find(|output| output["path"] == path.as_str())
                .unwrap_or_else(|| panic!("CL-27: {source}: the manifest lists {path}"));
            assert_eq!(output["sha256"], row["sha256"], "CL-27: {source}: {path}");
            let module = row["module"].as_str().expect("module");
            assert!(
                manifest["modules"]
                    .as_array()
                    .expect("manifest modules")
                    .iter()
                    .any(|m| m["module"] == module && m["lean_module"] == lean_module),
                "CL-27: {source}: {module} is {lean_module} in the manifest"
            );
        }
        // The shipped modules are the compiler's own, and every module comes
        // after every module of the document it imports.
        for (index, row) in rows.iter().enumerate() {
            let lean_module = row["lean_module"].as_str().expect("lean_module");
            let text = row["text"].as_str().expect("text");
            assert_eq!(
                row["sha256"].as_str().expect("sha256"),
                Sha256Digest::of(text.as_bytes()).to_hex(),
                "CL-27: {source}: {lean_module}'s digest"
            );
            match row["kind"].as_str().expect("kind") {
                "project" => {}
                "runtime" => {
                    runtime_seen = true;
                    let shipped = lexlean::backend::runtime::modules()
                        .expect("the runtime")
                        .iter()
                        .find(|module| module.name == lean_module)
                        .unwrap_or_else(|| panic!("CL-27: {lean_module} is a runtime module"));
                    assert!(shipped.text == text, "CL-27: {source}: {lean_module}");
                }
                "text-laws" => assert_eq!(
                    lexlean::text_laws::lean_text(lean_module).expect("a text-laws module"),
                    text,
                    "CL-27: {source}: {lean_module}"
                ),
                other => panic!("CL-27: {source}: kind `{other}`"),
            }
            for import in lean_imports(text) {
                if let Some(at) = rows.iter().position(|other| other["lean_module"] == import) {
                    assert!(
                        at < index,
                        "CL-27: {source}: {import} precedes {lean_module}, which imports it"
                    );
                }
            }
        }

        // JSON mode: one command result with check's IDs and the build ID.
        let fresh = copy(source);
        let (_, checked, _) = native(&fresh.root, &["--diagnostic-format", "json", "check"]);
        let checked: serde_json::Value = serde_json::from_str(&checked).expect("check JSON");
        let (exit, result, _) = native(
            &fresh.root,
            &["--diagnostic-format", "json", "lean-modules"],
        );
        let result: serde_json::Value = serde_json::from_str(&result).expect("command result");
        assert_eq!(exit, 0, "CL-27: {source}: json lean-modules exits 0");
        assert_eq!(result["command"], "lean-modules", "CL-27: {source}");
        for key in ["source_id", "semantic_id", "modules"] {
            assert_eq!(result[key], checked[key], "CL-27: {source}: json `{key}`");
        }
        assert_eq!(
            result["build_id"], document["build_id"],
            "CL-27: {source}: json build ID"
        );
        assert_eq!(
            result["artifacts"],
            serde_json::json!([]),
            "CL-27: {source}: no artifact is named"
        );
        native_bytes.push((source.clone(), stdout));
    }
    assert!(
        runtime_seen,
        "CL-27: some example imports the runtime, so its modules are compared"
    );

    // A project `check` refuses prints no document: the same exit code and
    // diagnostics, and nothing on stdout in human mode.
    let refused = copy(&support::repo_root().join("examples/nat-add-zero"));
    std::fs::write(
        refused.root.join("src/Main.lex.tex").as_std_path(),
        "\\begin{lexlean}{Main}\n",
    )
    .expect("write a broken module");
    let (check_exit, check_json, _) =
        native(&refused.root, &["--diagnostic-format", "json", "check"]);
    let (exit, stdout, stderr) = native(&refused.root, &["lean-modules"]);
    assert_ne!(
        check_exit, 0,
        "CL-27: the broken module is refused by check"
    );
    assert_eq!(exit, check_exit, "CL-27: lean-modules exits as check does");
    assert!(
        stdout.is_empty(),
        "CL-27: a refused project prints nothing: {stdout}"
    );
    assert!(!stderr.is_empty(), "CL-27: its diagnostics are on stderr");
    let (_, json, _) = native(
        &refused.root,
        &["--diagnostic-format", "json", "lean-modules"],
    );
    let json: serde_json::Value = serde_json::from_str(&json).expect("command result");
    let check_json: serde_json::Value = serde_json::from_str(&check_json).expect("command result");
    assert_eq!(
        json["diagnostics"], check_json["diagnostics"],
        "CL-27: the same diagnostics as check"
    );

    // On WASI, where `build` is refused for want of a lock, the same bytes:
    // the document holds no path or host data.
    if !wasi_backed("CL-27") {
        return;
    }
    let module = build();
    let scratch = tempfile::Builder::new()
        .prefix("lexlean-wasi-lean-modules-")
        .tempdir()
        .expect("tempdir");
    let host = Utf8PathBuf::from_path_buf(scratch.path().join("host.mjs")).expect("utf8 path");
    std::fs::write(host.as_std_path(), HOST).expect("write the host");
    for (source, bytes) in &native_bytes {
        let project = copy(source);
        let before = support::file_set(&project.root);
        let (exit, stdout, stderr) = wasi(&module, &host, &project.root, &["lean-modules"]);
        assert_eq!(
            exit, 0,
            "CL-27: {source}: lean-modules on WASI exits 0: {stderr}"
        );
        assert!(
            stdout == *bytes,
            "CL-27: {source}: the WASI document is the native document, byte for byte"
        );
        assert_eq!(
            support::file_set(&project.root),
            before,
            "CL-27: {source}: nothing is written on WASI"
        );
        // `build` stays refused there, so this command is the way to the
        // bytes it would publish.
        let (exit, _, _) = wasi(&module, &host, &project.root, &["build"]);
        assert_eq!(exit, 3, "CL-27: {source}: build is refused on WASI");
    }
    eprintln!(
        "CL-27: {} documents agree between the native and the WASI executable",
        native_bytes.len()
    );
}

/// The policy rows a verification attests for a `lexlean/lean-modules/1`
/// document, in its order: one per entry, a model's members in its place, a
/// `core` entry only when the attestation holds it (stage 11 skips a core
/// name the audit does not report). Each is `(rule, name, kind, axioms)`.
fn attested_rows(document: &serde_json::Value) -> Vec<(String, String, String, serde_json::Value)> {
    // The stage reads the project's modules in module order, then the
    // runtime modules; the document lists them in elaboration order.
    let mut modules: Vec<&serde_json::Value> = document["modules"]
        .as_array()
        .expect("modules")
        .iter()
        .filter(|module| module["kind"] == "project")
        .collect();
    modules.sort_by_key(|module| module["module"].as_str().unwrap_or_default().to_owned());
    modules.extend(
        document["modules"]
            .as_array()
            .expect("modules")
            .iter()
            .filter(|module| module["kind"] == "runtime"),
    );
    let mut rows = Vec::new();
    for module in modules {
        for policy in module["axiom_policies"].as_array().expect("axiom_policies") {
            let rule = policy["rule"].as_str().expect("rule").to_owned();
            let axioms = policy["axioms"].clone();
            if rule == "model" {
                for member in policy["members"].as_array().expect("members") {
                    rows.push((
                        rule.clone(),
                        member.as_str().expect("member").to_owned(),
                        "allow".to_owned(),
                        axioms.clone(),
                    ));
                }
            } else {
                rows.push((
                    rule,
                    policy["declaration"]
                        .as_str()
                        .expect("declaration")
                        .to_owned(),
                    policy["kind"].as_str().expect("kind").to_owned(),
                    axioms,
                ));
            }
        }
    }
    rows
}

/// CL-28.
pub(crate) fn lean_module_policies() {
    let root = support::repo_root();
    let mut rules: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    // Without Lean: the table of every example that checks names every rule
    // in the shape a host reads, and a model names the members it governs.
    for source in checking_examples() {
        let project = copy(&source);
        let (exit, stdout, stderr) = native(&project.root, &["lean-modules"]);
        assert_eq!(exit, 0, "CL-28: {source}: {stderr}");
        let document: serde_json::Value = serde_json::from_str(&stdout).expect("document");
        for module in document["modules"].as_array().expect("modules") {
            let policies = module["axiom_policies"].as_array().expect("axiom_policies");
            if module["kind"] == "text-laws" {
                assert!(
                    policies.is_empty(),
                    "CL-28: {source}: stage 11 holds no text-laws declaration"
                );
            }
            for policy in policies {
                let rule = policy["rule"].as_str().expect("rule");
                let kind = policy["kind"].as_str().expect("kind");
                assert!(
                    ["none", "allow", "exact"].contains(&kind),
                    "CL-28: {source}: kind {kind}"
                );
                let axioms = policy["axioms"].as_array().expect("axioms");
                if kind == "none" {
                    assert!(axioms.is_empty(), "CL-28: {source}: {policy}");
                }
                match rule {
                    "document" | "core" | "semantic" => assert!(policy.get("members").is_none()),
                    // A model that generates nothing governs no member, and
                    // then its union is empty.
                    "model" => assert!(
                        policy["members"].is_array(),
                        "CL-28: {source}: a model names the declarations it governs"
                    ),
                    "runtime" => {
                        assert_eq!(
                            kind, "exact",
                            "CL-28: {source}: a runtime declaration is exact"
                        );
                        assert_eq!(module["kind"], "runtime", "CL-28: {source}");
                    }
                    other => panic!("CL-28: {source}: rule `{other}`"),
                }
                rules.insert(rule.to_owned());
            }
        }
    }
    for rule in ["document", "core", "semantic", "model", "runtime"] {
        assert!(
            rules.contains(rule),
            "CL-28: some example has a `{rule}` entry"
        );
    }

    // With Lean: what `verify` attests is exactly the table, row for row in
    // its order, on examples that between them use every rule but `core`,
    // whose rule is the document rule but for an unreported name.
    for name in ["peano-arithmetic", "list-induction", "models"] {
        let project = copy(&root.join("examples").join(name));
        let (exit, stdout, stderr) = native(&project.root, &["lean-modules"]);
        assert_eq!(exit, 0, "CL-28: {name}: {stderr}");
        let document: serde_json::Value = serde_json::from_str(&stdout).expect("document");
        let Some(verified) = support::verify_ok_backed("CL-28", &project) else {
            return;
        };
        let attestation: serde_json::Value = serde_json::from_slice(
            &std::fs::read(verified.root.join("attestation.json").as_std_path())
                .expect("attestation"),
        )
        .expect("attestation JSON");
        let attested: Vec<(String, String, serde_json::Value)> = attestation["declarations"]
            .as_array()
            .expect("declarations")
            .iter()
            .map(|row| {
                (
                    row["name"].as_str().expect("name").to_owned(),
                    row["policy"]["kind"].as_str().expect("kind").to_owned(),
                    row["policy"]["axioms"].clone(),
                )
            })
            .collect();
        let names: std::collections::BTreeSet<&str> =
            attested.iter().map(|row| row.0.as_str()).collect();
        let expected: Vec<(String, String, serde_json::Value)> = attested_rows(&document)
            .into_iter()
            .filter(|(rule, name, _, _)| rule != "core" || names.contains(name.as_str()))
            .map(|(_, name, kind, axioms)| (name, kind, axioms))
            .collect();
        assert!(!expected.is_empty(), "CL-28: {name} has policies");
        assert_eq!(
            attested.len(),
            expected.len(),
            "CL-28: {name}: verify attests one row per printed row"
        );
        for (index, (attested, printed)) in attested.iter().zip(&expected).enumerate() {
            assert_eq!(
                attested, printed,
                "CL-28: {name}: row {index}: verify attests the printed row"
            );
        }
        eprintln!(
            "CL-28: {name}: {} attested rows are the printed table",
            attested.len()
        );
    }
}

//! `cargo xtask kernel-terms stage|produce`: the two steps of kernel-term
//! production evidence (SPEC.md §18.10) that need the compiler; the steps
//! that need Lean (compiling, exporting, judging) are
//! `docs/kernel-terms/run.sh`.
//!
//! `stage <lexlean.toml> <out>` builds the project and writes, under `out`,
//! every Lean module the build generated and every runtime module they
//! import (`src/`), the modules in dependency order (`order.txt`, one
//! `module<TAB>path` row each), the constants the library export must hold
//! (`roots.txt`), and the project's semantic snapshot (`snapshot.json`).
//!
//! `produce <out> [<library.ndjson>]` reads the snapshot `stage` wrote and
//! the library export, and writes the produced declarations
//! (`suffix.ndjson`), each produced theorem's own part (`parts/`), and what
//! became of each theorem (`production.json`). Without a library the
//! declarations are self-contained, for `crates/conformance/lean/KernelCheck.lean`.
//!
//! `declare <out> [<library.ndjson>]` reads the same snapshot and writes the
//! project's own declarations and the theorems produced over them
//! (`declarations.ndjson`), and each theorem's statement, form, proof,
//! axioms, and outcome, with every declaration refused (`declarations.json`):
//! what a checker holding only the library proves the project from.
//!
//! `roots` prints every constant the lowering and the producers may name in
//! any project: the roots of the library such a checker holds.

use std::fmt::Write as _;
use std::path::Path;

use serde_json::json;

use crate::Fail;

pub fn kernel_terms(arguments: &[String]) -> Result<(), Fail> {
    match arguments {
        [step, project, out] if step == "stage" => stage(Path::new(project), Path::new(out)),
        [step, out, library] if step == "produce" => produce(Path::new(out), Some(Path::new(library))),
        [step, out] if step == "produce" => produce(Path::new(out), None),
        [step, out, library] if step == "declare" => declare(Path::new(out), Some(Path::new(library))),
        [step, out] if step == "declare" => declare(Path::new(out), None),
        [step] if step == "roots" => {
            for root in lexlean::backend::kernel::library_roots() {
                println!("{root}");
            }
            Ok(())
        }
        _ => Err("cargo xtask kernel-terms stage <lexlean.toml> <out> | produce <out> [<library.ndjson>] | declare <out> [<library.ndjson>] | roots".into()),
    }
}

fn stage(project: &Path, out: &Path) -> Result<(), Fail> {
    let path = camino::Utf8Path::from_path(project).ok_or("the project path is UTF-8")?;
    let engine = lexlean::Engine::load(path)?;
    let built = engine.build(lexlean::BuildRequest {
        selection: lexlean::Selection::All,
    })?;
    let build_id = built.build_id.ok_or("a build has a build ID")?;
    let directory = project.parent().ok_or("the project file has a directory")?;
    let build = directory
        .join(".lexlean")
        .join("build")
        .join(build_id.to_hex());
    let source = out.join("src");
    std::fs::create_dir_all(&source)?;
    let mut texts: Vec<(String, String)> = Vec::new();
    for unit in built.units.values() {
        for artifact in &unit.artifacts.paths {
            let Some(relative) = artifact
                .strip_prefix("modules/")
                .and_then(|p| p.strip_suffix(".lean"))
            else {
                continue;
            };
            let text = std::fs::read_to_string(build.join(artifact))?;
            texts.push((relative.replace('/', "."), text));
        }
    }
    let generated: Vec<String> = texts.iter().map(|(name, _)| name.clone()).collect();
    let all = lexlean::backend::kernel::lean_sources(texts)
        .map_err(|diagnostic| diagnostic.message.clone())?;
    let mut order = String::new();
    for (name, text) in &all {
        let file = source.join(format!("{}.lean", name.replace('.', "/")));
        if let Some(parent) = file.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&file, text)?;
        writeln!(order, "{name}\tsrc/{}.lean", name.replace('.', "/"))?;
    }
    std::fs::write(out.join("order.txt"), order)?;
    let snapshot = engine.snapshot(lexlean::CheckRequest {
        selection: lexlean::Selection::All,
    })?;
    let snapshot = serde_json::to_value(&snapshot)?;
    let modules = lexlean::backend::kernel::modules_from_snapshot(&snapshot)
        .map_err(|diagnostic| diagnostic.message.clone())?;
    let runtime: Vec<&str> = all
        .iter()
        .map(|(name, _)| name.as_str())
        .filter(|name| !generated.iter().any(|g| g == name))
        .collect();
    let mut roots = lexlean::backend::kernel::export_roots(&modules, &runtime).join("\n");
    roots.push('\n');
    std::fs::write(out.join("roots.txt"), roots)?;
    std::fs::write(out.join("snapshot.json"), serde_json::to_string(&snapshot)?)?;
    println!(
        "kernel-terms stage: {} modules written to {}",
        all.len(),
        out.display()
    );
    Ok(())
}

fn produce(out: &Path, library: Option<&Path>) -> Result<(), Fail> {
    let snapshot: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(out.join("snapshot.json"))?)?;
    let modules = lexlean::backend::kernel::modules_from_snapshot(&snapshot)
        .map_err(|diagnostic| diagnostic.message.clone())?;
    let library = library.map(std::fs::read_to_string).transpose()?;
    let started = std::time::Instant::now();
    let production = lexlean::backend::kernel::produce(&modules, library.as_deref())
        .map_err(|diagnostic| diagnostic.message.clone())?;
    let elapsed = started.elapsed();
    std::fs::write(out.join("suffix.ndjson"), &production.suffix)?;
    let parts = out.join("parts");
    let _ = std::fs::remove_dir_all(&parts);
    std::fs::create_dir_all(&parts)?;
    let started_parts = std::time::Instant::now();
    let written_parts = production
        .parts()
        .map_err(|diagnostic| diagnostic.message.clone())?;
    let parts_elapsed = started_parts.elapsed();
    for (index, (theorem, part)) in written_parts.iter().enumerate() {
        std::fs::write(parts.join(format!("{index:05}.ndjson")), part)?;
        std::fs::write(parts.join(format!("{index:05}.name")), theorem)?;
    }
    let rows: Vec<serde_json::Value> = production
        .rows
        .iter()
        .map(|row| match &row.outcome {
            lexlean::backend::kernel::Outcome::Produced { constant, bound } => json!({
                "theorem": row.theorem, "form": row.form, "axioms": row.axioms,
                "produced": constant, "bound": bound,
            }),
            lexlean::backend::kernel::Outcome::Refused { reason } => json!({
                "theorem": row.theorem, "form": row.form, "axioms": row.axioms, "refused": reason,
            }),
        })
        .collect();
    let produced = rows
        .iter()
        .filter(|row| row.get("produced").is_some())
        .count();
    std::fs::write(
        out.join("production.json"),
        serde_json::to_string_pretty(&json!({
            "spec": "lexlean/kernel-production/1",
            "produce_ms": elapsed.as_secs_f64() * 1000.0,
            "parts_ms": parts_elapsed.as_secs_f64() * 1000.0,
            "suffix_bytes": production.suffix.len(),
            "rows": rows,
        }))?,
    )?;
    println!(
        "kernel-terms produce: {produced} of {} theorems produced in {:.1} ms, {} suffix bytes",
        production.rows.len(),
        elapsed.as_secs_f64() * 1000.0,
        production.suffix.len()
    );
    Ok(())
}

fn declare(out: &Path, library: Option<&Path>) -> Result<(), Fail> {
    // A byte literal is a list thousands of cells deep, and writing it recurses through each cell.
    let (out, library) = (out.to_path_buf(), library.map(Path::to_path_buf));
    std::thread::Builder::new()
        .stack_size(1 << 30)
        .spawn(move || declare_in(&out, library.as_deref()).map_err(|error| error.to_string()))?
        .join()
        .map_err(|_| "declaring panicked")??;
    Ok(())
}

fn declare_in(out: &Path, library: Option<&Path>) -> Result<(), Fail> {
    let snapshot: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(out.join("snapshot.json"))?)?;
    let modules = lexlean::backend::kernel::modules_from_snapshot(&snapshot)
        .map_err(|diagnostic| diagnostic.message.clone())?;
    let library = library.map(std::fs::read_to_string).transpose()?;
    // The one document `lexlean declare` prints (§23.4): its suffix is
    // declarations.ndjson, the rest declarations.json.
    let document = lexlean::backend::kernel::declarations_document(&modules, library.as_deref())
        .map_err(|diagnostic| diagnostic.message.clone())?;
    let mut value: serde_json::Value = serde_json::from_slice(&document.to_file_bytes())?;
    let suffix = value
        .as_object_mut()
        .and_then(|object| object.remove("suffix"))
        .and_then(|suffix| suffix.as_str().map(str::to_owned))
        .ok_or("the declarations document has a suffix")?;
    std::fs::write(out.join("declarations.ndjson"), &suffix)?;
    std::fs::write(out.join("declarations.json"), serde_json::to_string_pretty(&value)?)?;
    let theorems = value["theorems"].as_array().map_or(0, Vec::len);
    let stated = value["theorems"].as_array().map_or(0, |rows| rows.iter().filter(|row| row.get("statement").is_some()).count());
    let produced = value["theorems"].as_array().map_or(0, |rows| rows.iter().filter(|row| row.get("produced").is_some()).count());
    let refused = value["refused"].as_array().map_or(0, Vec::len);
    println!("kernel-terms declare: {theorems} theorems, {stated} stated, {produced} produced, {refused} declarations refused");
    Ok(())
}

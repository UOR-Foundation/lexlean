//! `cargo xtask proof-census [--write] [lexlean.toml ...]`: how many theorem
//! occurrences use each semantic proof form, read from the linked IR of every
//! LexLean project of the repository (and of any further project named on the
//! command line), so the choice of which proof forms get a kernel-term
//! producer (SPEC.md §18.10) rests on measured frequency, not a guess.
//!
//! A proof form is counted twice over: once per theorem whose whole proof is
//! that form (`whole`), and once per node of the proof tree, branches of
//! `constructor`, `cases`, `induction`, and `nested_induction` included
//! (`nodes`). A theorem is *phase-0 producible* when every node of its tree is
//! a form `lexlean::backend::kernel` produces. Theorems a reasoner elaborates
//! to are counted apart (`generated`): they have no semantic proof, only a
//! fixed template. A model's obligations name theorems the author writes, so
//! those are already counted as written.
//!
//! A project outside the repository may pin another Lean (a Prism app pins
//! the toolchain its SDK carries). Linking does not read the pin, so such a
//! project is copied and its pin set to the one this compiler requires; the
//! copy is the project in every other byte. A repository project without a
//! lock (a library the compiler embeds) is copied and locked the same way.
//! The written census is `docs/kernel-terms/census.json`. Without `--write`
//! the census is recomputed and compared with it, so a stale file fails.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::{json, Map, Value};

use crate::Fail;

/// The census file, relative to the repository root.
pub const CENSUS: &str = "docs/kernel-terms/census.json";

/// The repository directories whose projects are counted. Negative fixtures
/// and committed oracles (`expected/`) are not projects anyone proves.
const ROOTS: &[&str] = &[
    "examples",
    "compiler",
    "stdlib",
    "std-models",
    "std-reasoning",
    "bootstrap",
    "language",
];

/// Counts of one population of theorems.
#[derive(Default)]
struct Tally {
    theorems: usize,
    producible: usize,
    whole: BTreeMap<String, usize>,
    nodes: BTreeMap<String, usize>,
}

impl Tally {
    fn add(&mut self, proof: &Value) {
        self.theorems += 1;
        let kind = kind_of(proof);
        *self.whole.entry(kind).or_default() += 1;
        let mut all_producible = true;
        walk(proof, &mut |node| {
            let kind = kind_of(node);
            all_producible &= lexlean::backend::kernel::PRODUCED_FORMS.contains(&kind.as_str());
            *self.nodes.entry(kind).or_default() += 1;
        });
        if all_producible {
            self.producible += 1;
        }
    }

    fn merge(&mut self, other: &Self) {
        self.theorems += other.theorems;
        self.producible += other.producible;
        for (kind, count) in &other.whole {
            *self.whole.entry(kind.clone()).or_default() += count;
        }
        for (kind, count) in &other.nodes {
            *self.nodes.entry(kind.clone()).or_default() += count;
        }
    }

    fn json(&self) -> Value {
        json!({
            "theorems": self.theorems,
            "phase0_producible": self.producible,
            "whole": self.whole,
            "nodes": self.nodes,
        })
    }
}

fn kind_of(proof: &Value) -> String {
    proof
        .get("kind")
        .and_then(Value::as_str)
        .unwrap_or("unknown")
        .to_owned()
}

/// Every node of a semantic proof tree, the root first.
fn walk(proof: &Value, visit: &mut dyn FnMut(&Value)) {
    visit(proof);
    for branch in proof
        .get("branches")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        // `constructor` holds proofs; the case forms hold `{constructor, binders, proof}`.
        walk(branch.get("proof").unwrap_or(branch), visit);
    }
    for member in proof
        .get("members")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        for branch in member
            .get("branches")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            if let Some(inner) = branch.get("proof") {
                walk(inner, visit);
            }
        }
    }
}

/// Every project file under the repository roots, sorted.
fn repository_projects(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for top in ROOTS {
        for entry in walkdir::WalkDir::new(root.join(top))
            .follow_links(false)
            .into_iter()
            .filter_entry(|entry| {
                let name = entry.file_name().to_string_lossy();
                name != "expected" && name != ".lexlean" && name != ".lake" && name != "target"
            })
            .flatten()
        {
            if entry.file_type().is_file() && entry.file_name() == "lexlean.toml" {
                out.push(entry.path().to_path_buf());
            }
        }
    }
    out.sort();
    out
}

/// The census of one project: its tallies, or why it could not be linked.
fn project_census(file: &Path) -> Result<(Tally, usize, usize, usize), String> {
    let path = camino::Utf8Path::from_path(file).ok_or("the project path is not UTF-8")?;
    let engine = lexlean::Engine::load(path).map_err(|error| error.to_string())?;
    let snapshot = engine
        .snapshot(lexlean::CheckRequest {
            selection: lexlean::Selection::All,
        })
        .map_err(|error| error.to_string())?;
    let snapshot = serde_json::to_value(&snapshot).map_err(|error| error.to_string())?;
    let mut written = Tally::default();
    let mut generated = 0;
    let mut other_theorems = 0;
    let modules = snapshot["modules"].as_array().cloned().unwrap_or_default();
    for module in &modules {
        if let Some(semantic) = module.get("semantic") {
            for declaration in semantic["declarations"].as_array().into_iter().flatten() {
                if declaration["kind"] == "theorem" {
                    written.add(&declaration["proof"]);
                }
            }
        } else {
            // A prose or core module: its theorems have no semantic proof form.
            other_theorems += module["declarations"]
                .as_array()
                .into_iter()
                .flatten()
                .filter(|row| {
                    matches!(
                        row["kind"].as_str(),
                        Some("theorem" | "lemma" | "corollary")
                    )
                })
                .count();
        }
        for row in module["declarations"].as_array().into_iter().flatten() {
            generated += row
                .pointer("/elaboration/theorems")
                .and_then(Value::as_array)
                .map_or(0, Vec::len);
        }
    }
    Ok((written, generated, other_theorems, modules.len()))
}

/// The Lean pin this compiler requires of a project (§10.1).
const PIN: &str = "leanprover/lean4:v4.32.2";

/// A copy of a project outside the repository, or of one without a lock,
/// with its Lean pin set to [`PIN`] and a fresh lock: the temporary directory
/// (removed when dropped) and the copy's project file. Build and Lake
/// directories are not copied.
fn retargeted(file: &Path) -> Result<(tempfile::TempDir, PathBuf), String> {
    let source = file.parent().ok_or("the project file has no directory")?;
    let copy = tempfile::tempdir().map_err(|error| error.to_string())?;
    for entry in walkdir::WalkDir::new(source)
        .follow_links(false)
        .into_iter()
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            name != ".lexlean" && name != ".lake" && name != "target" && name != ".git"
        })
        .flatten()
    {
        let relative = entry
            .path()
            .strip_prefix(source)
            .map_err(|error| error.to_string())?;
        let target = copy.path().join(relative);
        if entry.file_type().is_dir() {
            std::fs::create_dir_all(&target).map_err(|error| error.to_string())?;
        } else if entry.file_type().is_file() {
            std::fs::copy(entry.path(), &target).map_err(|error| error.to_string())?;
        }
    }
    let project = copy.path().join("lexlean.toml");
    let text = std::fs::read_to_string(&project).map_err(|error| error.to_string())?;
    let text = text
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("lean_toolchain") {
                format!("lean_toolchain = \"{PIN}\"")
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join(
            "
",
        )
        + "
";
    std::fs::write(&project, text).map_err(|error| error.to_string())?;
    let toolchain = copy.path().join("lean-toolchain");
    if toolchain.is_file() {
        std::fs::write(
            &toolchain,
            format!(
                "{PIN}
"
            ),
        )
        .map_err(|error| error.to_string())?;
    }
    // The lock records the pin as well; a fresh lock of the same sources
    // resolves the same packages.
    let _ = std::fs::remove_file(copy.path().join("lexlean.lock"));
    let path = camino::Utf8Path::from_path(&project).ok_or("the copy's path is not UTF-8")?;
    lexlean::Engine::load(path)
        .and_then(|engine| {
            engine.lock(lexlean::LockRequest {
                check_only: false,
                allow_network: false,
            })
        })
        .map_err(|error| error.to_string())?;
    Ok((copy, project))
}

/// A project's label: its directory relative to the repository, or, for a
/// project outside it, `external:` and its last two directory names, so no
/// checkout path is recorded.
fn label(root: &Path, file: &Path) -> String {
    let directory = file.parent().unwrap_or(file);
    if let Ok(relative) = directory.strip_prefix(root) {
        return relative.display().to_string().replace('\\', "/");
    }
    let names: Vec<String> = directory
        .components()
        .rev()
        .take(2)
        .map(|part| part.as_os_str().to_string_lossy().into_owned())
        .collect();
    format!(
        "external:{}",
        names.into_iter().rev().collect::<Vec<_>>().join("/")
    )
}

pub fn census(root: &Path, write: bool, extra: &[String]) -> Result<(), Fail> {
    let mut files = repository_projects(root);
    files.extend(extra.iter().map(PathBuf::from));
    let mut projects = Map::new();
    let mut written = Tally::default();
    let mut generated_total = 0;
    let mut other = 0;
    let mut refused = Map::new();
    for file in &files {
        let name = label(root, file);
        let locked = file
            .parent()
            .is_some_and(|directory| directory.join("lexlean.lock").is_file());
        let linked = if file.starts_with(root) && locked {
            project_census(file)
        } else {
            retargeted(file).and_then(|(_copy, copied)| project_census(&copied))
        };
        match linked {
            Ok((own, generated, prose, modules)) => {
                written.merge(&own);
                generated_total += generated;
                other += prose;
                projects.insert(
                    name,
                    json!({
                        "modules": modules,
                        "written": own.json(),
                        "generated_template_theorems": generated,
                        "prose_or_core_theorems": prose,
                    }),
                );
            }
            Err(reason) => {
                // A project that does not link (a stale lock, a sibling that
                // moved) is reported, never silently dropped from the totals.
                refused.insert(
                    name,
                    Value::String(reason.lines().next().unwrap_or("").to_owned()),
                );
            }
        }
    }
    let nodes: usize = written.nodes.values().sum();
    let produced_nodes: usize = written
        .nodes
        .iter()
        .filter(|(kind, _)| lexlean::backend::kernel::PRODUCED_FORMS.contains(&kind.as_str()))
        .map(|(_, count)| count)
        .sum();
    let report = json!({
        "spec": "lexlean/proof-census/1",
        "produced_forms": lexlean::backend::kernel::PRODUCED_FORMS,
        "totals": {
            "written": written.json(),
            "generated_template_theorems": generated_total,
            "prose_or_core_theorems": other,
            "written_nodes": nodes,
            "written_nodes_in_produced_forms": produced_nodes,
        },
        "projects": projects,
        "not_linked": refused,
    });
    let mut text = serde_json::to_string_pretty(&report)?;
    text.push('\n');
    let path = root.join(CENSUS);
    if write {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, &text)?;
        println!(
            "proof-census: {} written theorems in {} projects, {} phase-0 producible; wrote {CENSUS}",
            written.theorems,
            projects.len(),
            written.producible
        );
        return Ok(());
    }
    let committed = std::fs::read_to_string(&path).unwrap_or_default();
    if committed != text {
        return Err(format!(
            "§18.10: {CENSUS} is not the census of the tree; run `cargo xtask proof-census --write` with the same project arguments"
        )
        .into());
    }
    println!("proof-census: {CENSUS} is current");
    Ok(())
}

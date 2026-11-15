//! Exported functions carry `#[target_feature]` for their feature set
//! (SPEC.md §17.13, §17.16, decision D4 of #109): `RB-25`.
//!
//! The safe-Rust rule excludes an `unsafe` dispatch and leaving instruction
//! selection to rustc excludes `-C target-feature`, so the exported function
//! itself carries the attribute and its caller holds the obligation to check
//! the CPU. The case holds the renderer to that: the attribute is printed on
//! the export and nowhere else, the validator refuses every malformed feature
//! set, and the pinned rustc refuses a caller without the feature (E0133),
//! which it accepts once the attribute is dropped.

use lexlean::calculus::rust::ast::{Ident, ItemDef};
use lexlean::calculus::rust::package::{self, Manifest};
use lexlean::calculus::rust::{validate, Profile};
use lexlean::calculus::{interp, Outcome, Program, Value};
use serde_json::Value as Json;

use crate::rust_build::{cargo_in, workspace, Built};
use crate::rust_packages;
use crate::support::repo_root;

/// `run(a, b) = a + b` over `Int`: an export that can overflow, so both a
/// value and the failure cross the boundary.
const PROGRAM: &str = r#"{"adts":[],"functions":[{"body":{"kind":"prim","operands":[{"kind":"var","name":0},{"kind":"var","name":1}],"operation":{"kind":"int_add"}},"parameters":[0,1],"result":{"kind":"int"},"types":[{"kind":"int"},{"kind":"int"}]}],"spec":"lexlean/target-program/1"}"#;

/// The vectors the harness runs: a sum, a negative sum, and an overflow.
const VECTORS: [(i64, i64); 3] = [(3, 4), (-9, 2), (i64::MAX, 1)];

fn program() -> Program {
    Program::parse(PROGRAM.as_bytes()).expect("the case's program")
}

fn manifest(name: &str, features: &[&str]) -> Manifest {
    let mut manifest = rust_packages::manifest(name, &program(), 0, Profile::Std);
    manifest.exports[0].features = features.iter().map(|f| (*f).to_owned()).collect();
    manifest
}

fn library(manifest: &Manifest) -> String {
    let rendered = package::package(manifest).expect("the package renders");
    String::from_utf8(rendered.files["src/lib.rs"].clone()).expect("UTF-8")
}

fn refused(manifest: &Manifest, expected: &str) {
    let error = package::package(manifest).expect_err(expected);
    assert!(error.contains(expected), "{expected}: {error}");
}

/// The registry's features are D4's, each on its one architecture and triple.
fn registry() {
    let registry = lexlean::production::registry().expect("the registry");
    let rows: Vec<(&str, &str, &str)> = registry
        .features
        .values()
        .map(|row| (row.id.as_str(), row.arch.as_str(), row.triple.as_str()))
        .collect();
    assert_eq!(
        rows,
        [
            ("avx2", "x86_64", "x86_64-unknown-linux-gnu"),
            ("neon", "aarch64", "aarch64-unknown-linux-gnu"),
            ("simd128", "wasm32", "wasm32-unknown-unknown"),
        ],
        "the registered target features are those decided in D4"
    );
    for id in registry.features.keys() {
        assert!(
            !registry.targets.contains_key(id),
            "a feature `{id}` is not a target"
        );
    }
    // The schemas of the manifest and the provenance admit exactly the
    // registered features.
    let registered: Vec<Json> = registry
        .features
        .keys()
        .map(|id| Json::from(id.as_str()))
        .collect();
    for name in ["rust-package.schema.json", "rust-provenance.schema.json"] {
        let schema: Json = serde_json::from_slice(
            &std::fs::read(repo_root().join("schemas").join(name).as_std_path()).expect("schema"),
        )
        .expect("schema JSON");
        assert_eq!(
            schema["properties"]["exports"]["items"]["properties"]["features"]["items"]["enum"],
            Json::Array(registered.clone()),
            "{name} admits exactly the registered target features"
        );
    }
}

/// The attribute is printed once, on the export, and the rest of the library
/// is the baseline's byte for byte; the provenance names the features.
fn rendering() {
    let baseline = library(&manifest("featured", &[]));
    assert!(
        !baseline.contains("#[target_feature"),
        "a baseline export carries no target feature"
    );
    let baseline_provenance = package::package(&manifest("featured", &[]))
        .expect("the package renders")
        .files["provenance.json"]
        .clone();
    let baseline_provenance: Json =
        serde_json::from_slice(&baseline_provenance).expect("provenance JSON");
    assert!(
        baseline_provenance["exports"][0].get("features").is_none(),
        "a baseline export's provenance is written as before"
    );
    for feature in ["avx2", "neon", "simd128"] {
        let featured = manifest("featured", &[feature]);
        let text = library(&featured);
        let attribute = format!("#[target_feature(enable = \"{feature}\")]\n");
        assert_eq!(
            text.matches("#[target_feature").count(),
            1,
            "{feature}: one attribute in the library"
        );
        assert!(
            text.contains(&format!("{attribute}pub fn run(")),
            "{feature}: the attribute is on the export:\n{text}"
        );
        assert_eq!(
            text.replacen(&attribute, "", 1),
            baseline,
            "{feature}: the attribute is the only difference from the baseline"
        );
        let provenance: Json = serde_json::from_slice(
            &package::package(&featured)
                .expect("the package renders")
                .files["provenance.json"],
        )
        .expect("provenance JSON");
        assert_eq!(
            provenance["exports"][0]["features"],
            serde_json::json!([feature]),
            "{feature}: the provenance names the feature its caller checks"
        );
    }
}

/// Every malformed feature set is refused by the validator, never repaired.
fn plants() {
    refused(
        &manifest("planted", &["avx512f"]),
        "`avx512f` of `run` is not a registered target feature",
    );
    refused(
        &manifest("planted", &["dsp"]),
        "`dsp` of `run` is not a registered target feature",
    );
    refused(
        &manifest("planted", &["simd128", "avx2"]),
        "the features of `run` are not strictly ascending",
    );
    refused(
        &manifest("planted", &["avx2", "avx2"]),
        "the features of `run` are not strictly ascending",
    );
    refused(
        &manifest("planted", &["avx2", "neon"]),
        "the features of `run` are of more than one architecture (aarch64, x86_64)",
    );
    // A lowered function never carries one: only the export's caller can
    // hold the obligation to check the CPU.
    let (mut krate, _, _) = package::lower(&manifest("planted", &[])).expect("the crate");
    let planted = krate.items.iter_mut().find_map(|item| match item {
        ItemDef::Function {
            name: Ident::Function(_),
            features,
            ..
        } => {
            features.push("avx2".to_owned());
            Some(())
        }
        _ => None,
    });
    assert!(planted.is_some(), "the plant site exists");
    let error = validate::validate(&krate).expect_err("a feature on a lowered function");
    assert!(
        error.contains("`f0` is not an export and carries target features"),
        "{error}"
    );
}

/// The feature of the host's architecture, with the macro that detects it.
fn host_feature() -> Option<(&'static str, &'static str)> {
    if cfg!(target_arch = "x86_64") {
        Some(("avx2", "is_x86_feature_detected"))
    } else if cfg!(target_arch = "aarch64") {
        Some(("neon", "is_aarch64_feature_detected"))
    } else {
        None
    }
}

fn expected(a: i64, b: i64) -> String {
    let arguments = [
        Value::Int {
            value: a.to_string(),
        },
        Value::Int {
            value: b.to_string(),
        },
    ];
    match interp::run(&program(), 1_000, 0, &arguments) {
        Outcome::Value {
            value: Value::Int { value },
            ..
        } => format!("Ok({value})"),
        Outcome::Overflow { .. } => "Err(Overflow)".to_owned(),
        other => panic!("the evaluator gives {other:?}"),
    }
}

/// Build `manifest` with a harness whose body is `harness`, in a fresh
/// workspace, and return cargo's output and the harness's run, if it built.
fn build(
    manifest: &Manifest,
    harness: String,
) -> (std::process::Output, Option<std::process::Output>) {
    let dir = tempfile::Builder::new()
        .prefix("lexlean-rust-target-feature-")
        .tempdir()
        .expect("tempdir");
    let name = manifest.name.clone();
    workspace(
        dir.path(),
        &[Built {
            name: name.clone(),
            files: package::package(manifest)
                .expect("the package renders")
                .files,
            harness,
        }],
    );
    let built = cargo_in(dir.path(), &["build", "--offline", "--workspace"]);
    if !built.status.success() {
        return (built, None);
    }
    let binary = dir
        .path()
        .join("target/debug")
        .join(format!("run_{name}{}", std::env::consts::EXE_SUFFIX));
    let ran = std::process::Command::new(&binary)
        .output()
        .expect("the harness runs");
    (built, Some(ran))
}

/// Under the pinned rustc: a caller with the feature builds and agrees with
/// the evaluator where the CPU has it; a caller without it is refused with
/// E0133; and the same caller builds once the attribute is dropped, so the
/// refusal is the attribute's.
fn rustc() {
    let Some((feature, detect)) = host_feature() else {
        eprintln!(
            "RB-25: no registered feature is of this host's architecture; the package is not built"
        );
        return;
    };
    let featured = manifest("target_feature", &[feature]);
    let name = featured.name.clone();
    let vectors: Vec<String> = VECTORS
        .iter()
        .map(|(a, b)| format!("({a}i64, {b}i64)"))
        .collect();
    let checked = format!(
        "#[target_feature(enable = \"{feature}\")]\nfn call(a: i64, b: i64) -> String {{\n    format!(\"{{:?}}\", {name}::run(a, b))\n}}\n\nfn main() {{\n    if !std::arch::{detect}!(\"{feature}\") {{\n        println!(\"unavailable\");\n        return;\n    }}\n    for (a, b) in [{}] {{\n        // The CPU was checked above: the caller's obligation (D4).\n        let line = unsafe {{ call(a, b) }};\n        println!(\"{{line}}\");\n    }}\n}}\n",
        vectors.join(", ")
    );
    let (built, ran) = build(&featured, checked);
    let ran = ran.unwrap_or_else(|| {
        panic!(
            "RB-25: a caller with `{feature}` does not build:\n{}",
            String::from_utf8_lossy(&built.stderr)
        )
    });
    assert!(
        ran.status.success(),
        "RB-25: the harness fails: {}",
        String::from_utf8_lossy(&ran.stderr)
    );
    let stdout = String::from_utf8(ran.stdout).expect("UTF-8");
    if stdout.trim() == "unavailable" {
        eprintln!("RB-25: this host's CPU has no `{feature}`; the package is built and not run");
    } else {
        let want: Vec<String> = VECTORS.iter().map(|(a, b)| expected(*a, *b)).collect();
        let got: Vec<&str> = stdout.lines().collect();
        assert_eq!(
            got, want,
            "RB-25: the export under `{feature}` agrees with the evaluator"
        );
    }
    let unchecked = format!("fn main() {{\n    println!(\"{{:?}}\", {name}::run(3, 4));\n}}\n");
    let (built, _) = build(&featured, unchecked.clone());
    let stderr = String::from_utf8_lossy(&built.stderr);
    assert!(
        !built.status.success() && stderr.contains("E0133"),
        "RB-25: a caller without `{feature}` is refused with E0133:\n{stderr}"
    );
    // The plant: the attribute dropped. The caller without the feature now
    // builds, so the refusal above is the attribute's and nothing else's.
    let (built, ran) = build(&manifest("target_feature", &[]), unchecked);
    assert!(
        built.status.success() && ran.is_some_and(|ran| ran.status.success()),
        "RB-25: the baseline export is callable without the feature:\n{}",
        String::from_utf8_lossy(&built.stderr)
    );
}

/// Run the case of `RB-25`.
pub fn run(id: &str) {
    assert_eq!(id, "RB-25");
    registry();
    rendering();
    plants();
    rustc();
}

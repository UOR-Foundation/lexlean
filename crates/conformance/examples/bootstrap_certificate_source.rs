//! Source-only investigation of the retained bootstrap R6 timeout.
//! This executable neither invokes Lean nor exports or accepts packages.

use std::path::PathBuf;

use lexlean::production::{certificate::certificate, lower};
use repo_conformance::support::{self, P};
use sha2::{Digest, Sha256};

fn main() {
    let mut args = std::env::args_os().skip(1);
    let output = PathBuf::from(args.next().expect("new output directory argument"));
    assert!(args.next().is_none(), "exactly one output directory");
    std::fs::create_dir(&output).expect("output must not already exist");

    // Read the retained producer's lock; computing a new one would hide drift.
    let project = P::bootstrap();
    let checked = support::checked_project(&project);
    let modules = lower::linked_modules(&checked);
    let roots = lower::roots(&checked).expect("bootstrap root reports");
    let identity = checked.semantic_id.to_hex();
    let root = roots.get(6).expect("bootstrap root R6");
    let module = format!("LexLeanPreserve.C{}.R6", &identity[..32]);
    assert_eq!(
        module, "LexLeanPreserve.C6ec49c97f5334c1cfee5e969876350b6.R6",
        "diagnostic must describe the retained v5 timeout"
    );
    let limits = support::limits(&project);
    let lowered = lower::lower_root(&modules, &root.module, &root.name, root.report, &limits)
        .expect("R6 lowers under its configured limits");
    let cert = certificate(
        &modules,
        &root.module,
        &root.name,
        root.report,
        &lowered,
        &module,
        limits.max_file_bytes,
    )
    .expect("R6 certificate source generates under its configured limit");
    let mut files = serde_json::Map::new();
    let mut write = |name: &str, bytes: &[u8]| {
        std::fs::write(output.join(name), bytes).expect("write diagnostic source");
        files.insert(
            name.to_owned(),
            serde_json::json!({"bytes": bytes.len(), "sha256": format!("{:x}", Sha256::digest(bytes))}),
        );
    };
    write("R6.lean", cert.text.as_bytes());
    if let Some(source) = &cert.source {
        write("R6.lex.tex", source.as_bytes());
    }
    write(
        "program.json",
        &serde_json::to_vec(&lowered.program).expect("program JSON"),
    );
    let summary = serde_json::json!({
        "kind": "source-generation-only",
        "accepted": false,
        "producer": "9fd01c31e40bb29577833825f902fafcceedb49a",
        "retained_run": "43620506158",
        "semantic_id": identity,
        "module": module,
        "root": root.report.root,
        "theorem": cert.theorem,
        "imports": cert.imports,
        "files": files,
        "roots": roots.iter().enumerate().map(|(i, r)| serde_json::json!({"index": i, "root": r.report.root})).collect::<Vec<_>>(),
    });
    std::fs::write(
        output.join("source-receipt.json"),
        serde_json::to_vec_pretty(&summary).expect("receipt JSON"),
    )
    .expect("write source receipt");
    println!("{}", summary);
}

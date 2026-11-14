//! Source-only R6 candidate using the owner's conditional-branch proof repair.
//! This executable neither invokes Lean nor exports or accepts packages.

use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

use lexlean::production::{certificate::certificate, lower};
use repo_conformance::support::{self, P};
use sha2::{Digest, Sha256};

const BASE: &str = "9fd01c31e40bb29577833825f902fafcceedb49a";
const OWNER_FIX: &str = "89b8f7738ed4781ba75bcc62ebb09cfcc7a42c40";

fn git(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .expect("read Git source identity");
    assert!(output.status.success(), "Git identity query failed");
    String::from_utf8(output.stdout)
        .expect("Git identity is UTF-8")
        .trim()
        .to_owned()
}

fn main() {
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates directory")
        .parent()
        .expect("repository directory")
        .to_owned();
    assert!(
        git(
            &repository,
            &["status", "--porcelain", "--untracked-files=normal"]
        )
        .is_empty(),
        "source generation requires a clean checkout"
    );
    let source_checkpoint = git(&repository, &["rev-parse", "HEAD"]);
    let compiled_checkpoint = option_env!("LEXLEAN_DIAGNOSTIC_SOURCE")
        .expect("build with LEXLEAN_DIAGNOSTIC_SOURCE set to the clean Git HEAD");
    assert_eq!(
        source_checkpoint, compiled_checkpoint,
        "generator must match the source checkout"
    );
    let source_tree = git(&repository, &["rev-parse", "HEAD^{tree}"]);
    let proof_path = "crates/lexlean/src/production/certificate/data_a/prove.rs";
    let actual_proof = git(&repository, &["rev-parse", &format!("HEAD:{proof_path}")]);
    let owner_proof = git(
        &repository,
        &["rev-parse", &format!("{OWNER_FIX}:{proof_path}")],
    );
    assert_eq!(
        actual_proof, owner_proof,
        "use the owner's exact proof repair"
    );
    git(&repository, &["merge-base", "--is-ancestor", BASE, "HEAD"]);
    let executable = std::env::current_exe().expect("generator executable");
    let mut executable_file = std::fs::File::open(&executable).expect("read generator executable");
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 65536];
    loop {
        let count = executable_file
            .read(&mut buffer)
            .expect("hash generator executable");
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    let generator_sha256 = format!("{:x}", hasher.finalize());

    let mut args = std::env::args_os().skip(1);
    let output = PathBuf::from(args.next().expect("new output directory argument"));
    assert!(args.next().is_none(), "exactly one output directory");
    std::fs::create_dir(&output).expect("output must not already exist");
    let compiler_identity = lexlean::compiler_semantics_id_for("1.2").to_hex();
    assert_eq!(
        compiler_identity, "59f64efe727898bcf6a337630c38eb4ea7ffc08cd83a49a0ea669c61c3b23587",
        "retained producer compiler identity"
    );

    // Read the retained base lock; computing a new one would hide drift.
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
        "compiler_semantics": compiler_identity,
        "not_exercised": ["aggregate production budget", "Lean elaboration", "kernel replay", "axiom audit", "certificates B and E"],
        "producer_base": BASE,
        "owner_fix_source_commit": OWNER_FIX,
        "source_checkpoint": source_checkpoint,
        "source_tree": source_tree,
        "proof_source_blob": actual_proof,
        "generator_sha256": generator_sha256,
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

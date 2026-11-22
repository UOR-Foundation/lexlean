//! Regression for issue #562: a rejected Lean module must fail the actual
//! lexlean process, not merely return a diagnostic through the library API.

use std::path::Path;
use std::process::Command;

#[test]
fn lean_rejection_has_a_failing_process_exit() {
    // The supported-host jobs run non-Lean tests without the Lean toolchain.
    // The normative rest leg has it; missing Lean there must never pass silently.
    let normative_rest = std::env::var("LEXLEAN_BDD_PART").is_ok_and(|part| part == "rest");
    let pinned_lean = Command::new("lean")
        .arg("--version")
        .output()
        .ok()
        .is_some_and(|output| {
            output.status.success()
                && String::from_utf8_lossy(&output.stdout).contains("4.32.2")
        });
    if !pinned_lean {
        assert!(
            !normative_rest,
            "the normative rest lane must have the pinned Lean 4.32.2 toolchain"
        );
        eprintln!("no pinned Lean available on this host; Lean process regression not run");
        return;
    }

    let repository = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .filter(|root| root.join("examples/nat-add-zero/lexlean.toml").is_file());
    let Some(repository) = repository else {
        assert!(
            !normative_rest,
            "the normative rest lane must have the committed example project"
        );
        eprintln!("standalone packaged crate: repository example unavailable");
        return;
    };
    let original = repository.join("examples/nat-add-zero");
    let directory = tempfile::tempdir().expect("isolated project");
    let project = directory.path();
    for file in [
        "NatAddZeroHost.lean",
        "lake-manifest.json",
        "lakefile.toml",
        "lean-toolchain",
        "lexlean.lock",
        "lexlean.toml",
    ] {
        std::fs::copy(original.join(file), project.join(file))
            .unwrap_or_else(|error| panic!("copy fixture {file}: {error}"));
    }
    std::fs::create_dir(project.join("src")).expect("source directory");
    let source = std::fs::read_to_string(original.join("src/Main.lex.tex"))
        .expect("committed proof source");
    let true_statement = r"\(n + 0 = n\)";
    let false_statement = r"\(n + 0 = 0\)";
    assert_eq!(
        source.matches(true_statement).count(),
        1,
        "the planted defect must target exactly one theorem"
    );
    std::fs::write(
        project.join("src/Main.lex.tex"),
        source.replacen(true_statement, false_statement, 1),
    )
    .expect("write false theorem");

    let binary = env!("CARGO_BIN_EXE_lexlean");
    let check = Command::new(binary)
        .current_dir(project)
        .args(["check", "src/Main.lex.tex"])
        .output()
        .expect("run the real check executable");
    assert_eq!(
        check.status.code(),
        Some(0),
        "the negative fixture must pass source checking before Lean: {}",
        String::from_utf8_lossy(&check.stderr)
    );

    // This is the originally reported process invocation: the default human
    // diagnostic path must fail closed even when Lean rejects after checking.
    let human = Command::new(binary)
        .current_dir(project)
        .args(["verify", "src/Main.lex.tex"])
        .output()
        .expect("run the default CLI verify executable");
    assert_eq!(
        human.status.code(),
        Some(1),
        "default CLI must exit 1 on Lean rejection; stdout: {}; stderr: {}",
        String::from_utf8_lossy(&human.stdout),
        String::from_utf8_lossy(&human.stderr)
    );
    assert!(
        String::from_utf8_lossy(&human.stderr).contains("LLV7002"),
        "default CLI must report the registered Lean rejection diagnostic"
    );
    assert!(
        !String::from_utf8_lossy(&human.stdout).contains("verified "),
        "default CLI cannot print a success verdict for a rejected proof"
    );

    let verify = Command::new(binary)
        .current_dir(project)
        .args(["--diagnostic-format", "json", "verify", "src/Main.lex.tex"])
        .output()
        .expect("run the real verify executable");
    assert_eq!(
        verify.status.code(),
        Some(1),
        "a Lean-rejected theorem must exit 1, not qualify a downstream gate; stdout: {}; stderr: {}",
        String::from_utf8_lossy(&verify.stdout),
        String::from_utf8_lossy(&verify.stderr)
    );
    assert!(verify.stderr.is_empty(), "JSON diagnostics belong on stdout, not stderr");
    let result: serde_json::Value =
        serde_json::from_slice(&verify.stdout).expect("one command-result JSON document");
    assert_eq!(result["command"], "verify");
    assert_eq!(result["exit_code"], 1);
    assert_eq!(result["success"], false);
    let has_rejection = result["diagnostics"]
        .as_array()
        .expect("diagnostics array")
        .iter()
        .any(|diagnostic| diagnostic["code"] == "LLV7002");
    assert!(has_rejection, "the failure must be the registered Lean rejection: {result}");
    assert!(
        result.get("attestation_id").is_none(),
        "a rejected proof cannot report a successful attestation"
    );
    let verified = project.join(".lexlean/verified");
    if verified.exists() {
        assert!(
            std::fs::read_dir(verified)
                .expect("verified directory")
                .next()
                .is_none(),
            "a rejected proof cannot publish a verified artifact"
        );
    }
}

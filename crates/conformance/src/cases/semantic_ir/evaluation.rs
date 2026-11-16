//! SM-44: a language-1.2 `evaluation` proof, an equality checked by the
//! kernel alone (SPEC.md §17.10).

use crate::support::{self, P};

fn source(statement: &str, proof: &str, axioms: &str) -> String {
    format!(
        "\\begin{{lexlean}}{{Main}}\n\\useglossary{{lexlean.std.nat@1.2.0}}\n\\title{{Natural number addition}}\n\\begin{{semanticmodule}}\ndefinition{{\n  name: double,\n  parameters: [n : Nat],\n  result: Nat,\n  body: (n + n)\n}}\n\ntheorem{{\n  name: doubled,\n  parameters: [],\n  statement: {statement},\n  proof: {proof},\n  axioms: [{axioms}]\n}}\n\\end{{semanticmodule}}\n\\end{{lexlean}}\n"
    )
}

fn project_of(text: &str, language: &str) -> P {
    let project = P::example();
    project.edit(
        "lexlean.toml",
        "language = \"1.0\"",
        &format!("language = \"{language}\""),
    );
    project.write("src/Main.lex.tex", text);
    project.relock();
    project
}

fn lean_text(project: &P) -> String {
    let built = project.build_ok();
    let build_dir = project.build_dir(&built.build_id.expect("a build id"));
    std::fs::read_dir(build_dir.join("modules").as_std_path())
        .expect("the modules")
        .flatten()
        .flat_map(|entry| {
            std::fs::read_dir(entry.path())
                .into_iter()
                .flatten()
                .flatten()
        })
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "lean"))
        .map(|entry| std::fs::read_to_string(entry.path()).expect("Lean text"))
        .collect()
}

/// Run the case of `SM-44`.
pub fn run() {
    let holds = "((double 21) = 42)";
    let accepted = project_of(&source(holds, "evaluation{}", ""), "1.2");
    let checked = support::checked_project(&accepted);
    let core = serde_json::to_value(
        checked.modules["Main"]
            .document
            .semantic
            .as_ref()
            .expect("a semantic module"),
    )
    .expect("module JSON");
    support::assert_schema("semantic-module-v2", "an evaluation proof", &core);
    assert!(
        core.to_string()
            .contains(r#""proof":{"kind":"evaluation"}"#),
        "the proof is the closed zero-argument form: {core}"
    );
    // The formatter keeps the proof, and the canonical source is a fixed
    // point that links to the same module; the notation reads back to it.
    let (code, _, error) = accepted.cli(&["fmt"]);
    assert_eq!(code, 0, "{error}");
    let formatted = accepted.read("src/Main.lex.tex");
    assert!(
        formatted.contains(r#""proof":{"kind":"evaluation"}"#),
        "the formatter keeps the proof: {formatted}"
    );
    let notation = lexlean::sdk::to_notation(&formatted).expect("the notation");
    assert!(notation.contains("evaluation{}"), "{notation}");
    assert_eq!(
        lexlean::sdk::read_module(&notation).expect("reads back"),
        lexlean::sdk::read_module(&formatted).expect("reads"),
        "the notation is the same module"
    );
    accepted.relock();
    accepted.fmt_check_ok();
    assert_eq!(
        serde_json::to_value(
            &support::checked_project(&accepted).modules["Main"]
                .document
                .semantic
        )
        .expect("JSON"),
        serde_json::to_value(&checked.modules["Main"].document.semantic).expect("JSON"),
        "the formatted source is the same module"
    );
    // The emitted proof is the fixed lowering, and the module imports, as
    // `meta`, exactly the two modules it runs.
    let lean = lean_text(&accepted);
    assert!(
        lean.contains("run_tac _root_.Lean.Elab.Tactic.liftMetaFinishingTactic fun llGoal => _root_.Lean.MVarId.refl llGoal (check := false)")
            && lean.contains("meta import Lean.Elab.Tactic.Basic\n")
            && lean.contains("meta import Lean.Meta.Tactic.Refl\n")
            && lean.matches("meta import").count() == 2,
        "{lean}"
    );
    if support::verify_ok_backed("SM-44", &accepted).is_some() {
        // The kernel decides the statement: a false equation is refused at
        // verification, and the axiom policy is exact.
        let false_equation = project_of(&source("((double 21) = 43)", "evaluation{}", ""), "1.2");
        false_equation.check_ok();
        false_equation.verify_fails_with("LLV7002");
        let widened = project_of(&source(holds, "evaluation{}", "propext"), "1.2");
        widened.check_ok();
        widened.verify_fails_with("LLV7005");
    }
    // Language 1.1 has no evaluation proof.
    let earlier = formatted
        .replace("@1.2.0", "@1.1.0")
        .replace("lexlean/semantic-module/2", "lexlean/semantic-module/1");
    let shown = format!("{:?}", project_of(&earlier, "1.1").check_err());
    assert!(shown.contains("evaluation proof"), "language 1.1: {shown}");
}

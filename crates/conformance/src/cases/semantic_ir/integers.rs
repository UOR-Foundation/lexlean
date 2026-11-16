//! SM-39: the language-1.2 integer conversions `int_of_nat`, `nat_abs`, and
//! `int_to_nat`, and `gcd` (SPEC.md §17.12, *Integer conversions and gcd*).

use crate::support::{self, P};

/// One theorem decided by the kernel over literals.
fn theorem(name: &str, statement: &str) -> String {
    format!(
        "theorem{{\n  name: {name},\n  parameters: [],\n  statement: {statement},\n  proof: reflexivity{{}},\n  axioms: []\n}}\n"
    )
}

/// A literal beyond every fixed width, so that the conversions are seen to be
/// over unbounded values: 2^70.
const WIDE: &str = "1180591620717411303424";

/// A language-1.2 module whose definitions apply each operation, and whose
/// theorems state Lean's values for them at the edges of their definitions:
/// zero, a negative value, and a value no fixed width holds.
fn module(extra: &str) -> String {
    let mut out = String::from(
        "\\begin{lexlean}{Main}\n\\useglossary{lexlean.std.nat@1.2.0}\n\\title{Natural number addition}\n\\begin{semanticmodule}\n",
    );
    out.push_str("definition{\n  name: widen,\n  parameters: [n : Nat],\n  result: Int,\n  body: (prim int_of_nat n : Int)\n}\n\n");
    out.push_str("definition{\n  name: magnitude,\n  parameters: [i : Int],\n  result: Nat,\n  body: (prim nat_abs i : Nat)\n}\n\n");
    out.push_str("definition{\n  name: clamp,\n  parameters: [i : Int],\n  result: Nat,\n  body: (prim int_to_nat i : Nat)\n}\n\n");
    // The gcd of two integers is the gcd of their magnitudes (Lean's
    // `Int.gcd`).
    out.push_str("definition{\n  name: divisor,\n  parameters: [a : Int, b : Int],\n  result: Nat,\n  body: (prim gcd (prim nat_abs a : Nat) (prim nat_abs b : Nat) : Nat)\n}\n\n");
    for (name, statement) in [
        (
            "ofNatZero",
            "((prim int_of_nat 0 : Int) = (0 : Int))".to_owned(),
        ),
        (
            "ofNatWide",
            format!("((prim int_of_nat {WIDE} : Int) = ({WIDE} : Int))"),
        ),
        (
            "absNegative",
            "((prim nat_abs (-7 : Int) : Nat) = 7)".to_owned(),
        ),
        (
            "absPositive",
            "((prim nat_abs (7 : Int) : Nat) = 7)".to_owned(),
        ),
        (
            "absWide",
            format!("((prim nat_abs (-{WIDE} : Int) : Nat) = {WIDE})"),
        ),
        (
            "clampNegative",
            "((prim int_to_nat (-3 : Int) : Nat) = 0)".to_owned(),
        ),
        (
            "clampPositive",
            "((prim int_to_nat (4 : Int) : Nat) = 4)".to_owned(),
        ),
        ("gcdBoth", "((prim gcd 12 18 : Nat) = 6)".to_owned()),
        ("gcdLeftZero", "((prim gcd 0 5 : Nat) = 5)".to_owned()),
        ("gcdRightZero", "((prim gcd 5 0 : Nat) = 5)".to_owned()),
        ("gcdZero", "((prim gcd 0 0 : Nat) = 0)".to_owned()),
        (
            "gcdWide",
            format!("((prim gcd {WIDE} 1152921504606846976 : Nat) = 1152921504606846976)"),
        ),
        (
            "gcdSigned",
            "((divisor (-12 : Int) (18 : Int)) = 6)".to_owned(),
        ),
    ] {
        out.push_str(&theorem(name, &statement));
        out.push('\n');
    }
    out.push_str(extra);
    out.push_str("\\end{semanticmodule}\n\\end{lexlean}\n");
    out
}

fn project_of(text: &str) -> P {
    let project = P::language_1_2_example();
    project.write("src/Main.lex.tex", text);
    project.relock();
    project
}

/// Run the case of `SM-39`.
pub fn run() {
    let accepted = project_of(&module(""));
    let checked = support::checked_project(&accepted);
    let core = serde_json::to_value(
        checked.modules["Main"]
            .document
            .semantic
            .as_ref()
            .expect("a semantic module"),
    )
    .expect("module JSON");
    support::assert_schema("semantic-module-v2", "the integer conversions", &core);

    // The generated Lean writes Lean's own definitions by their full names,
    // and nothing elaborates them.
    let rendered = support::rendered(&accepted);
    let lean = support::lean_text(&rendered, "Main");
    for expected in [
        "(Int.ofNat (n) : Int)",
        "(Int.natAbs (i) : Nat)",
        "(Int.toNat (i) : Nat)",
        "(Nat.gcd ((Int.natAbs (a) : Nat)) ((Int.natAbs (b) : Nat)) : Nat)",
    ] {
        assert!(lean.contains(expected), "missing {expected:?} in:\n{lean}");
    }
    for absent in ["int_of_nat", "nat_abs", "int_to_nat", "__widen"] {
        assert!(!lean.contains(absent), "the Lean names `{absent}`:\n{lean}");
    }
    let tex = support::tex_text(&rendered, "Main");
    assert!(
        tex.contains("Int.natAbs") && tex.contains("Nat.gcd"),
        "the document states the operations"
    );
    // The kernel decides every theorem with no axiom.
    let _ = support::verify_ok_backed("SM-39", &accepted);

    // A production root whose closure reaches one is refused: the operations
    // are formal-only (§17.13).
    let production = P::copy_example("production");
    production.write(
        "src/Main.lex.tex",
        "\\begin{lexlean}{Main}\n\\useglossary{lexlean.std.nat@1.2.0}\n\\title{Natural number addition}\n\\begin{semanticmodule}\ndefinition{\n  name: reduced,\n  parameters: [a : Nat, b : Nat],\n  result: Nat,\n  body: (prim gcd a b : Nat),\n  executable: true,\n  production: {effects: [overflow], targets: [rust-core, rust-std]}\n}\n\\end{semanticmodule}\n\\end{lexlean}\n",
    );
    production.relock();
    let error = production.check_fails_with("LLT4005").to_string();
    assert!(
        error.contains("construct `primitive.gcd` is formal-only"),
        "{error}"
    );
    production.assert_no_backend_output_with("LLT4005");

    // Misuse fails in linking, before any build output.
    for (what, body, message) in [
        (
            "int_of_nat over an Int",
            "(prim int_of_nat (1 : Int) : Int)",
            "primitive IntOfNat argument 0 has type Int, expected Nat",
        ),
        (
            "nat_abs over a Nat",
            "(prim nat_abs 1 : Nat)",
            "primitive NatAbs argument 0 has type Nat, expected Int",
        ),
        (
            "int_to_nat with an Int result",
            "(prim int_to_nat (1 : Int) : Int)",
            "primitive IntToNat result is Int, expected Nat",
        ),
        (
            "gcd over Int",
            "(prim gcd (1 : Int) (2 : Int) : Nat)",
            "primitive Gcd argument 0 has type Int, expected Nat",
        ),
        (
            "gcd of one operand",
            "(prim gcd 1 : Nat)",
            "primitive Gcd expects 2 argument(s), received 1",
        ),
        (
            "nat_abs over a fixed-width integer",
            "(prim nat_abs (1 : Int64) : Nat)",
            "primitive NatAbs argument 0 has type Int64, expected Int",
        ),
    ] {
        let result = if body.ends_with(": Int)") {
            "Int"
        } else {
            "Nat"
        };
        let extra = format!(
            "definition{{\n  name: misuse,\n  parameters: [],\n  result: {result},\n  body: {body}\n}}\n"
        );
        let planted = project_of(&module(&extra));
        let error = planted.check_fails_with("LLT4001");
        let shown = format!("{error:?}");
        assert!(shown.contains(message), "{what}: {shown}");
        planted.assert_no_backend_output_with("LLT4001");
    }

    // Language 1.1 has none of them: the data form of a 1.2 module that
    // applies one, read as a 1.1 module, is refused.
    let small = "\\begin{lexlean}{Main}\n\\useglossary{lexlean.std.nat@1.2.0}\n\\title{Natural number addition}\n\\begin{semanticmodule}\ndefinition{\n  name: magnitude,\n  parameters: [i : Int],\n  result: Nat,\n  body: (prim nat_abs i : Nat)\n}\n\\end{semanticmodule}\n\\end{lexlean}\n";
    let newer = project_of(small);
    let (code, _, error) = newer.cli(&["fmt"]);
    assert_eq!(code, 0, "{error}");
    let formatted = newer.read("src/Main.lex.tex");
    assert!(
        formatted.contains("\"operation\":\"nat_abs\""),
        "the formatter keeps the operation: {formatted}"
    );
    let older = P::example();
    older.edit("lexlean.toml", "language = \"1.0\"", "language = \"1.1\"");
    older.write(
        "src/Main.lex.tex",
        &formatted
            .replace("@1.2.0", "@1.1.0")
            .replace("lexlean/semantic-module/2", "lexlean/semantic-module/1"),
    );
    older.relock();
    let shown = format!("{:?}", older.check_err());
    assert!(shown.contains("integer conversion primitive"), "{shown}");
}

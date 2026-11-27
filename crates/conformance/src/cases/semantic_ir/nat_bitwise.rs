//! SM-42: bitwise primitives and shifts on unbounded `Nat` (SPEC.md §17.11,
//! §17.12, #317).

use crate::support::{self, P};

const THEOREM_XOR: &str = "theorem{\n  name: bitOfXor,\n  parameters: [a : Nat, b : Nat, k : Nat],\n  statement: ((prim remainder (prim shift_right (prim bit_xor a b : Nat) k : Nat) 2 0 : Nat) = (prim bit_xor (prim remainder (prim shift_right a k : Nat) 2 0 : Nat) (prim remainder (prim shift_right b k : Nat) 2 0 : Nat) : Nat)),\n  proof: law{name: nat_bit_xor, arguments: [a, b, k]},\n  axioms: [Quot.sound, propext]\n}\n";

/// One theorem applying each bitwise law but `nat_bit_xor` (whose theorem is
/// [`THEOREM_XOR`]), with `nat_bit_ext` discharged by a premise that names its
/// bit variable; each statement is the law's Lean conclusion at its arguments.
const THEOREMS_BITS: &str = "theorem{\n  name: bitOfAnd,\n  parameters: [a : Nat, b : Nat, k : Nat],\n  statement: ((prim remainder (prim shift_right (prim bit_and a b : Nat) k : Nat) 2 0 : Nat) = (prim bit_and (prim remainder (prim shift_right a k : Nat) 2 0 : Nat) (prim remainder (prim shift_right b k : Nat) 2 0 : Nat) : Nat)),\n  proof: law{name: nat_bit_and, arguments: [a, b, k]},\n  axioms: [Quot.sound, propext]\n}\n\ntheorem{\n  name: bitOfOr,\n  parameters: [a : Nat, b : Nat, k : Nat],\n  statement: ((prim remainder (prim shift_right (prim bit_or a b : Nat) k : Nat) 2 0 : Nat) = (prim bit_or (prim remainder (prim shift_right a k : Nat) 2 0 : Nat) (prim remainder (prim shift_right b k : Nat) 2 0 : Nat) : Nat)),\n  proof: law{name: nat_bit_or, arguments: [a, b, k]},\n  axioms: [Quot.sound, propext]\n}\n\ntheorem{\n  name: bitOfShiftRight,\n  parameters: [a : Nat, m : Nat, k : Nat],\n  statement: ((prim remainder (prim shift_right (prim shift_right a m : Nat) k : Nat) 2 0 : Nat) = (prim remainder (prim shift_right a (m + k) : Nat) 2 0 : Nat)),\n  proof: law{name: nat_bit_shift_right, arguments: [a, m, k]},\n  axioms: [propext]\n}\n\ntheorem{\n  name: bitOfShiftLeft,\n  parameters: [a : Nat, m : Nat, k : Nat],\n  statement: ((prim remainder (prim shift_right (prim shift_left a m : Nat) (m + k) : Nat) 2 0 : Nat) = (prim remainder (prim shift_right a k : Nat) 2 0 : Nat)),\n  proof: law{name: nat_bit_shift_left, arguments: [a, m, k]},\n  axioms: [Quot.sound, propext]\n}\n\ntheorem{\n  name: orOfXorAnd,\n  parameters: [a : Nat, b : Nat],\n  statement: ((prim bit_or a b : Nat) = (prim bit_xor (prim bit_xor a b : Nat) (prim bit_and a b : Nat) : Nat)),\n  proof: law{name: nat_or_xor_and, arguments: [a, b]},\n  axioms: [Quot.sound, propext]\n}\n\ntheorem{\n  name: wittStep,\n  parameters: [y : Nat],\n  statement: ((prim bit_xor y (prim bit_or (prim shift_left y 1 : Nat) (prim shift_left y 2 : Nat) : Nat) : Nat) = (prim bit_xor (prim bit_xor (prim bit_xor y (prim shift_left y 1 : Nat) : Nat) (prim shift_left y 2 : Nat) : Nat) (prim shift_left (prim bit_and y (prim shift_left y 1 : Nat) : Nat) 1 : Nat) : Nat)),\n  proof: law{name: nat_witt_step, arguments: [y]},\n  axioms: [Quot.sound, propext]\n}\n\ntheorem{\n  name: bitsOfFive,\n  parameters: [k : Nat],\n  statement: ((prim remainder (prim shift_right 5 k : Nat) 2 0 : Nat) = (prim remainder (prim shift_right 5 k : Nat) 2 0 : Nat)),\n  proof: reflexivity{},\n  axioms: []\n}\n\ntheorem{\n  name: extOfFive,\n  parameters: [],\n  statement: (5 = 5),\n  proof: law{name: nat_bit_ext, arguments: [5, 5], premises: [bitsOfFive]},\n  axioms: [Quot.sound, propext]\n}\n";

/// The exact axioms each bitwise law pins, sorted: the shift through a shift
/// is arithmetic on the amounts alone and needs no quotient soundness.
const BIT_LAW_AXIOMS: [(&str, &[&str]); 8] = [
    ("nat_bit_and", &["Quot.sound", "propext"]),
    ("nat_bit_or", &["Quot.sound", "propext"]),
    ("nat_bit_xor", &["Quot.sound", "propext"]),
    ("nat_bit_shift_right", &["propext"]),
    ("nat_bit_shift_left", &["Quot.sound", "propext"]),
    ("nat_or_xor_and", &["Quot.sound", "propext"]),
    ("nat_witt_step", &["Quot.sound", "propext"]),
    ("nat_bit_ext", &["Quot.sound", "propext"]),
];

fn source(extra: &str, production: bool) -> String {
    let production = if production {
        "definition{\n  name: probe,\n  parameters: [y : Nat],\n  result: Nat,\n  body: (prim bit_xor y y : Nat),\n  axioms: [propext],\n  executable: true,\n  production: {effects: [], targets: [rust-core]}\n}\n\n"
    } else {
        ""
    };
    format!(
        "\\begin{{lexlean}}{{Main}}\n\\useglossary{{lexlean.std.nat@1.2.0}}\n\\title{{Natural number addition}}\n\\begin{{semanticmodule}}\ndefinition{{\n  name: step,\n  parameters: [y : Nat],\n  result: Nat,\n  body: (prim bit_xor (y + (y + (y + y))) (prim bit_or (y + y) y : Nat) : Nat),\n  axioms: [propext],\n  executable: true\n}}\n\ndefinition{{\n  name: lifted,\n  parameters: [y : Nat, m : Nat],\n  result: Nat,\n  body: (prim bit_and (prim shift_left y m : Nat) (prim shift_right y m : Nat) : Nat),\n  axioms: [propext]\n}}\n\n{extra}{production}\\end{{semanticmodule}}\n\\end{{lexlean}}\n"
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

/// Run the case of `SM-42`.
pub fn run() {
    let accepted = project_of(&source(THEOREM_XOR, false), "1.2");
    let checked = support::checked_project(&accepted);
    let core = serde_json::to_value(
        checked.modules["Main"]
            .document
            .semantic
            .as_ref()
            .expect("a semantic module"),
    )
    .expect("module JSON");
    support::assert_schema("semantic-module-v2", "bitwise primitives on Nat", &core);
    // The formatter keeps the form, and the canonical source is a fixed
    // point that links to the same module.
    let (code, _, error) = accepted.cli(&["fmt"]);
    assert_eq!(code, 0, "{error}");
    accepted.relock();
    accepted.fmt_check_ok();
    // The generated Lean is the evaluable Lean function, not a runtime helper
    // that returns an option.
    let built = accepted.build_ok();
    let build_dir = accepted.build_dir(&built.build_id.expect("a build id"));
    let lean = std::fs::read_dir(build_dir.join("modules").as_std_path())
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
        .collect::<String>();
    for function in [
        "Nat.xor",
        "Nat.lor",
        "Nat.land",
        "Nat.shiftLeft",
        "Nat.shiftRight",
    ] {
        assert!(lean.contains(function), "{function} in {lean}");
    }
    assert!(
        !lean.contains("LexLeanRuntime.bitXor") && !lean.contains("LexLeanRuntime.shiftLeft"),
        "a Nat form is not the fixed-width runtime helper: {lean}"
    );
    // The misuses are refused by the linker or the analysis.
    let false_law = THEOREM_XOR.replace(
        "prim bit_xor (prim remainder (prim shift_right a k",
        "prim bit_or (prim remainder (prim shift_right a k",
    );
    let complement = "definition{\n  name: flipped,\n  parameters: [y : Nat],\n  result: Nat,\n  body: (prim bit_not y : Nat)\n}\n\n";
    let option_shift = "definition{\n  name: narrow,\n  parameters: [y : Nat],\n  result: (Option Nat),\n  body: (prim shift_left y (3 : UInt32) : (Option Nat))\n}\n\n";
    let mixed = "definition{\n  name: mixed,\n  parameters: [y : Nat, z : Int],\n  result: Nat,\n  body: (prim bit_and y z : Nat)\n}\n\n";
    // Language 1.1 reads the data form only.
    let plain = project_of(&source("", false), "1.2");
    let (code, _, error) = plain.cli(&["fmt"]);
    assert_eq!(code, 0, "{error}");
    let legacy = plain
        .read("src/Main.lex.tex")
        .replace("@1.2.0", "@1.1.0")
        .replace("lexlean/semantic-module/3", "lexlean/semantic-module/1");
    for (what, text, language, message) in [
        ("language 1.1", legacy, "1.1", "bitwise primitive on Nat"),
        (
            "a complement of a Nat",
            source(complement, false),
            "1.2",
            "requires a fixed-width operand",
        ),
        (
            "a shift of a Nat with a fixed-width amount",
            source(option_shift, false),
            "1.2",
            "",
        ),
        (
            "mixed Nat and Int operands",
            source(mixed, false),
            "1.2",
            "",
        ),
        (
            "a production root over a Nat form",
            source("", true),
            "1.2",
            "no bounded-width production realization",
        ),
    ] {
        let planted = project_of(&text, language);
        assert!(!planted.check_err_or_ok(), "{what} checks");
        let error = planted.check_err();
        let shown = format!("{error:?}");
        assert!(shown.contains(message), "{what}: {shown}");
    }

    // Each registered bitwise law is a declaration of the runtime's laws
    // module pinning exactly the axioms of its row in [`BIT_LAW_AXIOMS`], the policy its
    // theorems are held to; the registry holds no other bitwise law.
    let families = crate::cases::proofs::registered_nat_families();
    let modules = lexlean::backend::runtime::modules().expect("the shipped runtime");
    let laws = modules
        .iter()
        .find(|module| module.name == lexlean::backend::runtime::LAWS)
        .expect("the laws module is a runtime module");
    for name in crate::cases::proofs::NAT_BITWISE_LAWS {
        let (_, lean) = families
            .iter()
            .find(|(registered, _)| registered == name)
            .unwrap_or_else(|| panic!("`{name}` is registered"));
        let (_, axioms) = laws
            .declarations
            .iter()
            .find(|(declared, _)| declared == lean)
            .unwrap_or_else(|| panic!("`{lean}` is a declaration of the laws module"));
        let (_, policy) = BIT_LAW_AXIOMS
            .iter()
            .find(|(law, _)| *law == name)
            .unwrap_or_else(|| panic!("`{name}` has a policy row"));
        let mut axioms = axioms.clone();
        axioms.sort();
        assert_eq!(axioms, *policy, "`{name}` pins {axioms:?}");
        assert!(
            name == "nat_bit_xor" || THEOREMS_BITS.contains(&format!("law{{name: {name},")),
            "`{name}` is applied by a theorem"
        );
    }

    // The registered laws and the evaluation of the step verify under the
    // exact axioms their policies state, and a false bit equation is refused.
    let evaluated = format!(
        "{THEOREM_XOR}\n{THEOREMS_BITS}\ntheorem{{\n  name: stepOne,\n  parameters: [],\n  statement: ((step 1) = 7),\n  proof: reflexivity{{}},\n  axioms: [propext]\n}}\n\n"
    );
    let evaluated = project_of(&source(&evaluated, false), "1.2");
    support::verify_ok_backed("SM-42", &evaluated);
    let wrong = "theorem{\n  name: stepWrong,\n  parameters: [],\n  statement: ((step 1) = 6),\n  proof: reflexivity{},\n  axioms: []\n}\n\n";
    let wrong = project_of(&source(wrong, false), "1.2");
    let false_law = project_of(&source(&false_law, false), "1.2");
    // Both are well formed and link: only Lean refuses them.
    wrong.check_ok();
    false_law.check_ok();
    if support::lean_backed("SM-42") {
        wrong.verify_fails_with("LLV7002");
        false_law.verify_fails_with("LLV7002");
    }
}

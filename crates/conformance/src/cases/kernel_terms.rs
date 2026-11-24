//! LN-13: kernel-term production (SPEC.md §18.10).
//!
//! A module whose theorems use reflexivity, decide, cases, induction with a
//! generalized parameter, and apply is produced with no library, and the
//! fold-laws example supplies a law. Platform-independently, every theorem is
//! produced and bound, production is deterministic, and production refuses a
//! proof that omits a case and a proof form outside the table. Where the
//! pinned toolchain is installed, Lean's own kernel
//! (`crates/conformance/lean/KernelCheck.lean`, `Kernel.Environment.addDeclCore`)
//! judges the produced declarations over the project's compiled modules and
//! must accept every one with exactly its declared axioms, and must refuse
//! the produced declarations of an altered statement, a swapped case branch,
//! and a false decide.

use lexlean::backend::kernel::{self, Outcome, Production};
use serde_json::Value;

use crate::support::{self, P};

/// The fixture: one theorem for each produced form but law.
const FIXTURE: &str = r#"\begin{lexlean}{Main}
\useglossary{lexlean.std.nat@1.2.0}
\useglossary{languagetwelve.titles@1.0.0}
\title{Typed let terms}
\begin{semanticmodule}
definition{
  name: flip,
  parameters: [b : Bool],
  result: Bool,
  body: (match b | Bool.true => false | Bool.false => true)
}

definition{
  name: double,
  parameters: [n : Nat],
  result: Nat,
  body: (match n | Nat.zero => 0 | Nat.succ k => ((double k) + 2)),
  recursive_argument: n
}

definition{name: leftOf, parameters: [n : Nat, m : Nat], result: Nat, body: n}

theorem{name: double_two, parameters: [], statement: ((double 2) = 4), proof: reflexivity{}}

theorem{name: double_three, parameters: [], statement: ((double 3) = 6), proof: decide{}}

theorem{name: flip_false, parameters: [], statement: ((flip (flip false)) = false), proof: reflexivity{}}

theorem{name: flip_true, parameters: [], statement: ((flip (flip true)) = true), proof: reflexivity{}}

theorem{
  name: flip_flip,
  parameters: [b : Bool],
  statement: ((flip (flip b)) = b),
  proof: cases{
    branches: [
      {proof: apply{arguments: [], theorem: flip_false}, binders: [], constructor: "false"},
      {proof: apply{arguments: [], theorem: flip_true}, binders: [], constructor: "true"}
    ],
    scrutinee: b
  }
}

theorem{
  name: left_of,
  parameters: [n : Nat, m : Nat],
  statement: ((leftOf n m) = n),
  proof: induction{
    branches: [
      {proof: reflexivity{}, binders: [], constructor: zero},
      {proof: reflexivity{}, binders: [k, ih], constructor: succ}
    ],
    generalizing: [m],
    scrutinee: n
  }
}
\end{semanticmodule}
\end{lexlean}
"#;

fn snapshot(project: &P) -> Value {
    let snapshot = project
        .engine()
        .snapshot(lexlean::CheckRequest {
            selection: lexlean::Selection::All,
        })
        .expect("the project links");
    serde_json::to_value(&snapshot).expect("a snapshot is JSON")
}

fn produce(snapshot: &Value) -> Production {
    let modules = kernel::modules_from_snapshot(snapshot).expect("a snapshot this compiler wrote");
    kernel::produce(&modules, None).expect("self-contained production")
}

/// The theorem `name`'s declaration in the snapshot's semantic data.
fn theorem<'a>(snapshot: &'a mut Value, name: &str) -> &'a mut Value {
    snapshot["modules"]
        .as_array_mut()
        .expect("modules")
        .iter_mut()
        .filter_map(|module| module.get_mut("semantic"))
        .flat_map(|semantic| {
            semantic["declarations"]
                .as_array_mut()
                .expect("declarations")
                .iter_mut()
        })
        .find(|declaration| declaration["name"] == name)
        .unwrap_or_else(|| panic!("the fixture declares {name}"))
}

fn outcome<'a>(production: &'a Production, theorem: &str) -> &'a Outcome {
    &production
        .rows
        .iter()
        .find(|row| row.theorem.ends_with(&format!(".{theorem}")))
        .unwrap_or_else(|| panic!("a row for {theorem}"))
        .outcome
}

fn lean() -> std::path::PathBuf {
    support::real_elan_home()
        .join("toolchains")
        .join(support::mangled_toolchain_name())
        .join("bin")
        .join("lean")
}

/// The project's generated modules and the runtime modules they import,
/// compiled by the pinned Lean with every olean part (`lexlean verify` keeps
/// only the `.olean`, and an importing checker reads them all): the
/// directory to put on `LEAN_PATH`.
fn compiled(project: &P) -> tempfile::TempDir {
    let built = project.build_ok();
    let build = project.build_dir(&built.build_id.expect("a build ID"));
    let mut generated = Vec::new();
    for unit in built.units.values() {
        for artifact in &unit.artifacts.paths {
            if let Some(module) = artifact
                .strip_prefix("modules/")
                .and_then(|p| p.strip_suffix(".lean"))
            {
                let text = std::fs::read_to_string(build.join(artifact).as_std_path())
                    .expect("generated Lean");
                generated.push((module.replace('/', "."), text));
            }
        }
    }
    let work = tempfile::tempdir().expect("tempdir");
    let source = work.path().join("src");
    let oleans = work.path().join("oleans");
    for (module, text) in kernel::lean_sources(generated).expect("the generated modules order") {
        let relative = module.replace('.', "/");
        let file = source.join(format!("{relative}.lean"));
        let target = oleans.join(format!("{relative}.olean"));
        std::fs::create_dir_all(file.parent().expect("a directory")).expect("mkdir");
        std::fs::create_dir_all(target.parent().expect("a directory")).expect("mkdir");
        std::fs::write(&file, text).expect("write a module");
        let output = std::process::Command::new(lean())
            .arg("-R")
            .arg(&source)
            .arg("-o")
            .arg(&target)
            .arg(&file)
            .env("LEAN_PATH", &oleans)
            .output()
            .expect("run the pinned Lean");
        assert!(
            output.status.success(),
            "{module} compiles: {}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    work
}

/// Lean's kernel over a self-contained suffix and the project's compiled
/// modules: each declaration's verdict line, by name.
fn judge(compiled: &tempfile::TempDir, modules: &[String], suffix: &str) -> Vec<String> {
    let work = tempfile::tempdir().expect("tempdir");
    let file = work.path().join("suffix.ndjson");
    std::fs::write(&file, suffix).expect("write the suffix");
    let checker = support::repo_root().join("crates/conformance/lean/KernelCheck.lean");
    let output = std::process::Command::new(lean())
        .arg("--run")
        .arg(checker.as_std_path())
        .arg(&file)
        .args(modules)
        .env("LEAN_PATH", compiled.path().join("oleans"))
        .output()
        .expect("run the pinned Lean");
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    assert!(
        !stdout.is_empty()
            && stdout
                .lines()
                .all(|line| line.starts_with("accepted ") || line.starts_with("refused ")),
        "KernelCheck printed something else:\n{stdout}\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    stdout.lines().map(str::to_owned).collect()
}

fn verdict<'a>(lines: &'a [String], constant: &str) -> &'a str {
    lines
        .iter()
        .find(|line| {
            line.strip_prefix("accepted ")
                .or_else(|| line.strip_prefix("refused "))
                .is_some_and(|rest| {
                    rest.starts_with(&format!("{constant} "))
                        || rest.starts_with(&format!("{constant}:"))
                })
        })
        .map_or("absent", String::as_str)
}

fn lean_modules(snapshot: &Value) -> Vec<String> {
    snapshot["modules"]
        .as_array()
        .expect("modules")
        .iter()
        .map(|module| {
            module["lean_module"]
                .as_str()
                .expect("a Lean module")
                .to_owned()
        })
        .collect()
}

#[allow(clippy::too_many_lines)]
pub(crate) fn ln_13() {
    assert_eq!(
        kernel::PRODUCED_FORMS,
        [
            "reflexivity",
            "decide",
            "cases",
            "induction",
            "apply",
            "law",
            "term_cases"
        ],
        "the produced forms are the §18.10 table"
    );
    let fixture = P::copy_example("language-1.2");
    fixture.write("src/Main.lex.tex", FIXTURE);
    fixture.check_ok();
    let original = snapshot(&fixture);
    let production = produce(&original);
    assert_eq!(production.rows.len(), 6, "{:?}", production.rows);
    for row in &production.rows {
        let Outcome::Produced { constant, bound } = &row.outcome else {
            panic!("{} is not produced: {:?}", row.theorem, row.outcome);
        };
        assert_eq!(*constant, format!("{}.{}", kernel::NAMESPACE, row.theorem));
        assert_eq!(
            bound.as_deref(),
            Some(format!("{}.bound.{}", kernel::NAMESPACE, row.theorem).as_str()),
            "self-contained production binds every theorem"
        );
    }
    assert_eq!(
        produce(&original).suffix,
        production.suffix,
        "production is deterministic"
    );
    assert!(
        production
            .suffix
            .lines()
            .all(|line| serde_json::from_str::<Value>(line).is_ok()),
        "every produced line is JSON"
    );

    // An omitted case is refused by production, with the constructor named.
    let mut omitted = original.clone();
    theorem(&mut omitted, "flip_flip")["proof"]["branches"]
        .as_array_mut()
        .expect("branches")
        .retain(|branch| branch["constructor"] != "true");
    let refused = produce(&omitted);
    assert!(
        matches!(outcome(&refused, "flip_flip"), Outcome::Refused { reason } if reason.contains("no case for constructor `true`")),
        "{:?}",
        outcome(&refused, "flip_flip")
    );

    // A proof form outside the table is refused by name, never approximated.
    let laws = P::copy_example("fold-laws");
    laws.check_ok();
    let laws_snapshot = snapshot(&laws);
    let laws_production = produce(&laws_snapshot);
    assert!(
        matches!(outcome(&laws_production, "parity_step"), Outcome::Refused { reason } if reason == "proof form `linear_arithmetic`"),
        "{:?}",
        outcome(&laws_production, "parity_step")
    );
    assert!(
        matches!(
            outcome(&laws_production, "statistics_one_pass"),
            Outcome::Produced { .. }
        ),
        "the registered law list_fold_pair is produced: {:?}",
        outcome(&laws_production, "statistics_one_pass")
    );

    // The planted faults, each in the IR production reads.
    let mut altered = original.clone();
    let statement = &mut theorem(&mut altered, "double_two")["statement"];
    statement["left"]["arguments"][0]["value"] = Value::from("3");
    statement["right"]["value"] = Value::from("6");
    let mut swapped = original.clone();
    for branch in theorem(&mut swapped, "flip_flip")["proof"]["branches"]
        .as_array_mut()
        .expect("branches")
    {
        let other = if branch["constructor"] == "true" {
            "false"
        } else {
            "true"
        };
        branch["constructor"] = Value::from(other);
    }
    let mut false_decide = original.clone();
    theorem(&mut false_decide, "double_three")["statement"]["right"]["value"] = Value::from("7");
    let (altered, swapped, false_decide) =
        (produce(&altered), produce(&swapped), produce(&false_decide));
    for (name, planted) in [
        ("double_two", &altered),
        ("flip_flip", &swapped),
        ("double_three", &false_decide),
    ] {
        assert!(
            matches!(outcome(planted, name), Outcome::Produced { .. }),
            "the producer writes the planted {name}; the kernel must refuse it"
        );
    }

    if !support::lean_backed("LN-13") {
        return;
    }
    let verified = compiled(&fixture);
    let modules = lean_modules(&original);
    let lines = judge(&verified, &modules, &production.suffix);
    for row in &production.rows {
        for constant in [
            format!("{}.{}", kernel::NAMESPACE, row.theorem),
            format!("{}.bound.{}", kernel::NAMESPACE, row.theorem),
        ] {
            assert_eq!(
                verdict(&lines, &constant),
                format!("accepted {constant} axioms: {}", row.axioms.join(", ")),
                "{lines:#?}"
            );
        }
    }
    let prefix = |name: &str| {
        production
            .rows
            .iter()
            .find(|row| row.theorem.ends_with(&format!(".{name}")))
            .map(|row| row.theorem.clone())
            .expect("a row")
    };
    // The altered statement is still proved by reflexivity; binding it to
    // the theorem Lean elaborated is what the kernel refuses.
    let lines = judge(&verified, &modules, &altered.suffix);
    let double_two = prefix("double_two");
    assert!(
        verdict(&lines, &format!("{}.{double_two}", kernel::NAMESPACE)).starts_with("accepted "),
        "{lines:#?}"
    );
    assert!(
        verdict(&lines, &format!("{}.bound.{double_two}", kernel::NAMESPACE))
            .starts_with("refused "),
        "{lines:#?}"
    );
    let lines = judge(&verified, &modules, &swapped.suffix);
    let flip_flip = prefix("flip_flip");
    assert!(
        verdict(&lines, &format!("{}.{flip_flip}", kernel::NAMESPACE)).starts_with("refused "),
        "{lines:#?}"
    );
    let lines = judge(&verified, &modules, &false_decide.suffix);
    let double_three = prefix("double_three");
    assert!(
        verdict(&lines, &format!("{}.{double_three}", kernel::NAMESPACE)).starts_with("refused "),
        "{lines:#?}"
    );

    // The law, over the fold-laws example's compiled modules.
    let laws_verified = compiled(&laws);
    let lines = judge(
        &laws_verified,
        &lean_modules(&laws_snapshot),
        &laws_production.suffix,
    );
    for row in laws_production
        .rows
        .iter()
        .filter(|row| matches!(row.outcome, Outcome::Produced { .. }))
    {
        let constant = format!("{}.{}", kernel::NAMESPACE, row.theorem);
        assert_eq!(
            verdict(&lines, &constant),
            format!("accepted {constant} axioms: {}", row.axioms.join(", ")),
            "{lines:#?}"
        );
    }
}

/// The `LN-14` fixture: `term_cases` over a split term its Boolean fixes.
const ENUMERATION: &str = r#"\begin{lexlean}{Main}
\useglossary{lexlean.std.nat@1.2.0}
\useglossary{languagetwelve.titles@1.0.0}
\title{Typed let terms}
\begin{semanticmodule}
inductive{name: Shade, type_parameters: [], parameters: [], constructors: [{name: light, fields: []}, {name: dark, fields: []}]}

definition{name: shadeOf, parameters: [flag : Bool], result: Shade, body: (if flag then (ctor Shade.light) else (ctor Shade.dark))}

definition{
  name: weight,
  parameters: [flag : Bool, n : Nat],
  result: Nat,
  body: (match (shadeOf flag) | Shade.light => n | Shade.dark => n)
}

definition{
  name: code,
  parameters: [flag : Bool],
  result: Nat,
  body: (match (shadeOf flag) | Shade.light => 0 | Shade.dark => 1)
}

theorem{
  name: weight_is,
  parameters: [flag : Bool, n : Nat],
  statement: ((weight flag n) = n),
  proof: term_cases{definitions: [weight], scrutinee: (shadeOf flag)},
  axioms: [propext]
}

theorem{
  name: code_small,
  parameters: [flag : Bool],
  statement: ((code flag) <= 1),
  proof: term_cases{definitions: [code], scrutinee: (shadeOf flag)},
  axioms: [propext]
}
\end{semanticmodule}
\end{lexlean}
"#;

pub(crate) fn ln_14() {
    assert!(
        kernel::PRODUCED_FORMS.contains(&"term_cases"),
        "term_cases is a produced form"
    );
    let fixture = P::copy_example("language-1.2");
    fixture.write("src/Main.lex.tex", ENUMERATION);
    fixture.check_ok();
    let original = snapshot(&fixture);
    let production = produce(&original);
    for name in ["weight_is", "code_small"] {
        assert!(
            matches!(
                outcome(&production, name),
                Outcome::Produced { bound: Some(_), .. }
            ),
            "{name}: {:?}",
            outcome(&production, name)
        );
    }
    // False only when the Boolean is `false`: `code false` is 1.
    let mut planted = original.clone();
    theorem(&mut planted, "code_small")["statement"]["right"]["value"] = Value::from("0");
    let planted = produce(&planted);
    assert!(
        matches!(outcome(&planted, "code_small"), Outcome::Produced { .. }),
        "the producer writes the planted code_small; the kernel must refuse it"
    );
    if !support::lean_backed("LN-14") {
        return;
    }
    let compiled_project = compiled(&fixture);
    let modules = lean_modules(&original);
    let lines = judge(&compiled_project, &modules, &production.suffix);
    for row in &production.rows {
        // Lean's own proof keeps exactly the declared axioms; the produced
        // proof, by evaluation where Lean rewrites, rests on no more.
        let bound = format!("{}.bound.{}", kernel::NAMESPACE, row.theorem);
        assert_eq!(
            verdict(&lines, &bound),
            format!("accepted {bound} axioms: {}", row.axioms.join(", ")),
            "{lines:#?}"
        );
        let produced = format!("{}.{}", kernel::NAMESPACE, row.theorem);
        let line = verdict(&lines, &produced);
        let rests_on = line
            .strip_prefix(&format!("accepted {produced} axioms: "))
            .unwrap_or_else(|| panic!("{produced} is accepted: {lines:#?}"));
        for axiom in rests_on.split(", ").filter(|axiom| !axiom.is_empty()) {
            assert!(
                row.axioms.iter().any(|declared| declared == axiom),
                "{produced} rests on {axiom}, which is not declared"
            );
        }
    }
    let lines = judge(&compiled_project, &modules, &planted.suffix);
    let code_small = production
        .rows
        .iter()
        .find(|row| row.theorem.ends_with(".code_small"))
        .map(|row| row.theorem.clone())
        .expect("a row");
    assert!(
        verdict(&lines, &format!("{}.{code_small}", kernel::NAMESPACE)).starts_with("refused "),
        "{lines:#?}"
    );
}

fn declare(snapshot: &Value) -> kernel::Declarations {
    let modules = kernel::modules_from_snapshot(snapshot).expect("a snapshot this compiler wrote");
    kernel::declare(&modules, None).expect("self-contained declarations")
}

/// What each line of an export declares, by its kind (`def`, `inductive`, `thm`).
fn declared_kinds(lines: &str) -> Vec<&'static str> {
    lines
        .lines()
        .filter_map(|line| {
            let value: Value = serde_json::from_str(line).expect("every line is JSON");
            ["def", "inductive", "thm"]
                .into_iter()
                .find(|kind| value.get(*kind).is_some())
        })
        .collect()
}

pub(crate) fn ln_15() {
    let mut judged = Vec::new();
    for example in ["higher-order", "recursion"] {
        let fixture = P::copy_example(example);
        fixture.check_ok();
        let original = snapshot(&fixture);
        let declared = declare(&original);
        assert!(
            declared.refused.is_empty(),
            "{example}: {:?}",
            declared.refused
        );
        assert!(!declared.production.rows.is_empty());
        for row in &declared.production.rows {
            assert_eq!(
                declared.statements.get(&row.theorem),
                Some(&format!("{}.{}", kernel::declare::STATEMENT, row.theorem)),
                "{} has its statement",
                row.theorem
            );
            if let Outcome::Produced { bound, .. } = &row.outcome {
                assert_eq!(
                    *bound, None,
                    "nothing is bound where no Lean compiled the project"
                );
            }
        }
        let kinds = declared_kinds(&declared.declarations);
        for kind in ["inductive", "def"] {
            assert!(
                kinds.contains(&kind),
                "{example} declares a {kind}: {kinds:?}"
            );
        }
        assert!(
            declared
                .production
                .suffix
                .starts_with(&declared.declarations),
            "production continues the declarations"
        );
        assert_eq!(
            declare(&original).production.suffix,
            declared.production.suffix,
            "declarations are deterministic"
        );
        judged.push((fixture, declared, kinds.len()));
    }

    // What is not written is refused with its reason, and so is what names it.
    let classes = P::copy_example("semantic-1.1");
    classes.check_ok();
    let refused = declare(&snapshot(&classes)).refused;
    for kind in ["a class", "a instance"] {
        assert!(
            refused.iter().any(|r| r.reason.starts_with(kind)),
            "{kind} is refused: {refused:?}"
        );
    }
    // A type given a value parameter is refused, and so is every declaration that names it.
    let (recursion, _, _) = &judged[1];
    let mut planted = snapshot(recursion);
    let rose = planted["modules"]
        .as_array_mut()
        .expect("modules")
        .iter_mut()
        .filter_map(|module| module.get_mut("semantic"))
        .flat_map(|semantic| {
            semantic["declarations"]
                .as_array_mut()
                .expect("declarations")
                .iter_mut()
        })
        .find(|declaration| declaration["name"] == "Rose")
        .expect("the recursion example declares Rose");
    rose["parameters"] = serde_json::json!([{"name": "bound", "type": {"kind": "nat"}}]);
    let refused = declare(&planted).refused;
    assert!(
        refused
            .iter()
            .any(|r| r.name.ends_with(".Rose") && r.reason.contains("value parameters")),
        "{refused:?}"
    );
    assert!(
        refused.iter().any(|r| r.name.ends_with(".roseSize")),
        "a declaration naming a refused one is refused: {refused:?}"
    );

    if !support::lean_backed("LN-15") {
        return;
    }
    // Lean's kernel with its own library and the runtime, and nothing of the project.
    for (fixture, declared, declarations) in &judged {
        let compiled = compiled(fixture);
        let mut modules = vec!["Init".to_owned()];
        for entry in walkdir::WalkDir::new(compiled.path().join("oleans"))
            .into_iter()
            .flatten()
        {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "olean") {
                let relative = path
                    .strip_prefix(compiled.path().join("oleans"))
                    .expect("under oleans")
                    .with_extension("");
                let module = relative.to_string_lossy().replace(['/', '\\'], ".");
                if module.starts_with("LexLeanRuntime.") {
                    modules.push(module);
                }
            }
        }
        let lines = judge(&compiled, &modules, &declared.production.suffix);
        assert!(
            lines.iter().all(|line| line.starts_with("accepted ")),
            "{lines:#?}"
        );
        let produced = declared
            .production
            .rows
            .iter()
            .filter(|row| matches!(row.outcome, Outcome::Produced { .. }))
            .count();
        assert_eq!(lines.len(), declarations + produced, "{lines:#?}");
        for row in &declared.production.rows {
            if matches!(row.outcome, Outcome::Produced { .. }) {
                let constant = format!("{}.{}", kernel::NAMESPACE, row.theorem);
                assert!(
                    verdict(&lines, &constant).starts_with(&format!("accepted {constant} ")),
                    "{lines:#?}"
                );
            }
        }
    }
}

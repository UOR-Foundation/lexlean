//! `BS-35` to `BS-38` (SPEC.md §17.23, §18.10): kernel-term production's
//! producers as language-1.2 source, `LexLeanBoot.KernelTerms`.
//!
//! `BS-35` holds the three checks the producers decide by (`checkFrom`,
//! `memberCheck`, `lawCheck`) to the stage-0 producer's
//! `lexlean::backend::kernel::select`, under the calculus evaluator and
//! rustc. `BS-36`, `BS-37`, and `BS-38` are the binding, totality, and
//! refusal theorems the module proves: the module is verified alone under
//! the pinned Lean, with the exact axioms each theorem declares, and each
//! planted defect makes Lean refuse it.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use lexlean::backend::kernel::select::{self, BranchHead, CtorShape, Refusal};
use lexlean::calculus::Value;
use serde_json::Value as Json2;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::{bool_val, list, nat_val, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root, P};

const MODULE: &str = "LexLeanBoot.KernelTerms";
const SOURCE: &str = "src/KernelTerms.lex.tex";
const ROOTS: [(&str, &[&str]); 3] = [
    ("checkFrom", &["allocation", "overflow", "recursion"]),
    ("memberCheck", &["allocation"]),
    ("lawCheck", &["allocation"]),
];
/// Seeded runs of each root under the evaluator, and under rustc.
const EVALUATOR_SEEDED: usize = 300;
const RUSTC_SEEDED: usize = 2_000;

// ------------------------------------------------------------- the encoders

fn adt(constructor: u64, fields: Vec<Value>) -> Value {
    Value::Adt {
        constructor,
        fields,
    }
}

fn enc_ctor(shape: CtorShape) -> Value {
    adt(0, vec![nat_val(shape.fields), nat_val(shape.recursive)])
}

/// A branch; its proof is `Term.const` of the branch's position, which the
/// checks never read.
fn enc_branch(head: BranchHead, at: usize) -> Value {
    adt(
        0,
        vec![
            nat_val(head.ctor),
            nat_val(head.binders),
            adt(2, vec![nat_val(at)]),
        ],
    )
}

fn enc_names(names: &[usize]) -> Value {
    list(names.iter().map(|name| nat_val(*name)).collect())
}

/// `Check`: `pass`, or `fail` of a `Refusal` in the model's constructor
/// order.
fn enc_check(outcome: Result<(), Refusal<usize>>) -> Value {
    let refusal = match outcome {
        Ok(()) => return adt(0, Vec::new()),
        Err(refusal) => refusal,
    };
    let (constructor, fields) = match refusal {
        Refusal::NotAnEquation => (0, Vec::new()),
        Refusal::MissingCase(index) => (1, vec![nat_val(index)]),
        Refusal::RepeatedCase(index) => (2, vec![nat_val(index)]),
        Refusal::BinderCount(index) => (3, vec![nat_val(index)]),
        Refusal::UnknownCase(index) => (4, vec![nat_val(index)]),
        Refusal::NotProduced(name) => (5, vec![nat_val(name)]),
        Refusal::UnregisteredLaw(name) => (6, vec![nat_val(name)]),
    };
    adt(1, vec![adt(constructor, fields)])
}

fn json(value: &Value) -> Json2 {
    serde_json::to_value(value).expect("a value is JSON")
}

fn check_run(ctors: &[CtorShape], index: usize, branches: &[BranchHead], induct: bool) -> Run {
    (
        vec![
            list(ctors.iter().map(|shape| enc_ctor(*shape)).collect()),
            nat_val(index),
            list(
                branches
                    .iter()
                    .enumerate()
                    .map(|(at, head)| enc_branch(*head, at))
                    .collect(),
            ),
            bool_val(induct),
        ],
        json(&enc_check(select::check_from(
            ctors, index, branches, induct,
        ))),
    )
}

fn member_run(names: &[usize], name: usize) -> Run {
    (
        vec![enc_names(names), nat_val(name)],
        json(&enc_check(select::member_check(names, &name))),
    )
}

fn law_run(law: usize, registered: &[usize], premises: &[usize], produced: &[usize]) -> Run {
    (
        vec![
            nat_val(law),
            enc_names(registered),
            enc_names(premises),
            enc_names(produced),
        ],
        json(&enc_check(select::law_check(
            &law, registered, premises, produced,
        ))),
    )
}

// ------------------------------------------------------------------ vectors

fn shape(fields: usize, recursive: usize) -> CtorShape {
    CtorShape { fields, recursive }
}

fn head(ctor: usize, binders: usize) -> BranchHead {
    BranchHead { ctor, binders }
}

/// The hand-built vectors of each root, by label: every refusal, the order
/// in which they are found, and the boundaries of the index.
fn named() -> BTreeMap<&'static str, (&'static str, Run)> {
    let bool_shape = [shape(0, 0), shape(0, 0)];
    let nat_shape = [shape(0, 0), shape(1, 1)];
    let list_shape = [shape(0, 0), shape(2, 1)];
    let mut out = BTreeMap::new();
    let mut add = |label, root, run| {
        assert!(
            out.insert(label, (root, run)).is_none(),
            "{label} is named once"
        );
    };
    add(
        "cases bool",
        "checkFrom",
        check_run(&bool_shape, 0, &[head(0, 0), head(1, 0)], false),
    );
    add(
        "cases reversed",
        "checkFrom",
        check_run(&bool_shape, 0, &[head(1, 0), head(0, 0)], false),
    );
    add(
        "cases missing",
        "checkFrom",
        check_run(&bool_shape, 0, &[head(0, 0)], false),
    );
    add(
        "cases missing first",
        "checkFrom",
        check_run(&bool_shape, 0, &[head(1, 0)], false),
    );
    add(
        "cases repeated",
        "checkFrom",
        check_run(&bool_shape, 0, &[head(0, 0), head(0, 0), head(1, 0)], false),
    );
    add(
        "cases binders",
        "checkFrom",
        check_run(&nat_shape, 0, &[head(0, 0), head(1, 2)], false),
    );
    add(
        "cases unknown",
        "checkFrom",
        check_run(&bool_shape, 0, &[head(0, 0), head(1, 0), head(5, 0)], false),
    );
    add(
        "cases unknown at the end",
        "checkFrom",
        check_run(&bool_shape, 0, &[head(0, 0), head(1, 0), head(2, 0)], false),
    );
    add(
        "cases empty type",
        "checkFrom",
        check_run(&[], 0, &[], false),
    );
    add(
        "cases empty type stray",
        "checkFrom",
        check_run(&[], 0, &[head(0, 0)], false),
    );
    add(
        "induction nat",
        "checkFrom",
        check_run(&nat_shape, 0, &[head(0, 0), head(1, 2)], true),
    );
    add(
        "induction nat without hypothesis",
        "checkFrom",
        check_run(&nat_shape, 0, &[head(0, 0), head(1, 1)], true),
    );
    add(
        "induction list",
        "checkFrom",
        check_run(&list_shape, 0, &[head(1, 3), head(0, 0)], true),
    );
    add(
        "cases from an index",
        "checkFrom",
        check_run(&bool_shape, 3, &[head(3, 0), head(4, 0)], false),
    );
    add(
        "cases from an index below",
        "checkFrom",
        check_run(&bool_shape, 3, &[head(3, 0), head(4, 0), head(1, 0)], false),
    );
    add("member present", "memberCheck", member_run(&[3, 7, 9], 7));
    add("member first", "memberCheck", member_run(&[3, 7, 9], 3));
    add("member absent", "memberCheck", member_run(&[3, 7, 9], 8));
    add("member of none", "memberCheck", member_run(&[], 0));
    add(
        "law cited",
        "lawCheck",
        law_run(4, &[1, 4], &[7, 9], &[9, 7, 2]),
    );
    add("law no premises", "lawCheck", law_run(4, &[4], &[], &[]));
    add(
        "law unregistered",
        "lawCheck",
        law_run(5, &[1, 4], &[7], &[7]),
    );
    add(
        "law premise missing",
        "lawCheck",
        law_run(4, &[4], &[7, 8, 9], &[7, 9]),
    );
    out
}

fn seeded_check(seeded: &mut Seeded) -> Run {
    let ctors: Vec<CtorShape> = (0..seeded.below(5))
        .map(|_| shape(seeded.below(4), seeded.below(3)))
        .collect();
    let index = *seeded.pick(&[0, 0, 0, 1, 7]);
    let induct = seeded.below(2) == 1;
    let mut branches = Vec::new();
    for (at, ctor) in ctors.iter().enumerate() {
        let wanted = ctor.fields + if induct { ctor.recursive } else { 0 };
        let copies = *seeded.pick(&[1, 1, 1, 1, 0, 2]);
        for _ in 0..copies {
            let binders = if seeded.below(6) == 0 {
                seeded.below(5)
            } else {
                wanted
            };
            branches.push(head(index + at, binders));
        }
    }
    if seeded.below(5) == 0 {
        branches.push(head(index + ctors.len() + seeded.below(3), 0));
    }
    // Branches in any order: the check reads them by constructor.
    for at in (1..branches.len()).rev() {
        branches.swap(at, seeded.below(at + 1));
    }
    check_run(&ctors, index, &branches, induct)
}

fn names(seeded: &mut Seeded) -> Vec<usize> {
    (0..seeded.below(5)).map(|_| seeded.below(8)).collect()
}

fn seeded_runs(count: usize, seed: u64) -> BTreeMap<&'static str, Vec<Run>> {
    let mut seeded = Seeded(seed);
    let mut out: BTreeMap<&'static str, Vec<Run>> = BTreeMap::new();
    for _ in 0..count {
        out.entry("checkFrom")
            .or_default()
            .push(seeded_check(&mut seeded));
        let pool = names(&mut seeded);
        let name = seeded.below(8);
        out.entry("memberCheck")
            .or_default()
            .push(member_run(&pool, name));
        let registered = names(&mut seeded);
        let premises = names(&mut seeded);
        let produced = names(&mut seeded);
        let law = seeded.below(8);
        out.entry("lawCheck")
            .or_default()
            .push(law_run(law, &registered, &premises, &produced));
    }
    out
}

// --------------------------------------------------------- planted defects

struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
    vector: &'static str,
}

const PLANTS: [Plant; 5] = [
    Plant {
        label: "a constructor with no branch passes",
        site: "(if ((countCase index branches) =? 0) then (ctor Check.fail (ctor Refusal.missingCase index))",
        replacement: "(if ((countCase index branches) =? 9) then (ctor Check.fail (ctor Refusal.missingCase index))",
        vector: "cases missing",
    },
    Plant {
        label: "a repeated branch passes",
        site: "(if ((countCase index branches) =? 1) then",
        replacement: "(if (1 <=? (countCase index branches)) then",
        vector: "cases repeated",
    },
    Plant {
        label: "the hypotheses of an induction are not counted",
        site: "body: (if induct then ((project shape fields) + (project shape recursive)) else (project shape fields)),",
        replacement: "body: (project shape fields),",
        vector: "induction nat",
    },
    Plant {
        label: "a branch past the last constructor passes",
        site: "(if ((project head ctor) <? count) then (strayFrom count rest)",
        replacement: "(if ((project head ctor) <? (count + 9)) then (strayFrom count rest)",
        vector: "cases unknown",
    },
    Plant {
        label: "an unregistered law is reported as an unproduced premise",
        site: "| Check.fail refusal => (ctor Check.fail (ctor Refusal.unregisteredLaw law))),",
        replacement: "| Check.fail refusal => (ctor Check.fail refusal)),",
        vector: "law unregistered",
    },
];

fn plants(vectors: &BTreeMap<&'static str, (&'static str, Run)>) {
    let source = std::fs::read_to_string(repo_root().join("bootstrap").join(SOURCE).as_std_path())
        .expect("the committed source");
    let mut missed = Vec::new();
    for plant in &PLANTS {
        assert!(
            source.contains(plant.site),
            "{}: the site is in the source",
            plant.label
        );
        let (root, run) = &vectors[plant.vector];
        let planted = P::bootstrap();
        planted.write(SOURCE, &source.replacen(plant.site, plant.replacement, 1));
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, root);
        match super::evaluate(&low, std::slice::from_ref(run), 1).first() {
            Some(first) => eprintln!(
                "BS-35: plant detected: {}: {}",
                plant.label,
                first.chars().take(160).collect::<String>()
            ),
            None => missed.push(plant.label),
        }
    }
    assert!(
        missed.is_empty(),
        "planted defects their own vector does not detect: {missed:?}"
    );
}

fn export(lowered: &BTreeMap<&str, super::Lowered>) {
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        let Some((member, low)) = exported_root
            .root
            .strip_prefix(&format!("{MODULE}."))
            .and_then(|member| lowered.get(member).map(|low| (member, low)))
        else {
            continue;
        };
        for output in &exported_root.outputs {
            let package: Json2 = serde_json::from_slice(
                &std::fs::read(
                    bundle
                        .join(&output.package.directory)
                        .join("package.json")
                        .as_std_path(),
                )
                .expect("package.json"),
            )
            .expect("package JSON");
            assert_eq!(
                package["program"],
                serde_json::to_value(&low.program).expect("a program is JSON"),
                "{member}: the exported package realizes the program the differential ran"
            );
            realized += 1;
        }
    }
    assert_eq!(realized, ROOTS.len(), "one certified package per root");
    let packages = harness::tree(&bundle.join("packages"));
    let committed = harness::tree(&harness::fixture_dir("bootstrap").join("packages"));
    assert_eq!(
        packages.keys().collect::<Vec<_>>(),
        committed.keys().collect::<Vec<_>>(),
        "the exported package files are the committed ones"
    );
    for (relative, content) in &packages {
        assert!(
            committed.get(relative) == Some(content),
            "packages/{relative} differs from the committed package"
        );
    }
}

pub(super) fn bs_35() {
    let started = std::time::Instant::now();
    let project = P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("KernelTerms"),
        "KernelTerms is a linked module of the project"
    );
    eligibility_of(&project, "KernelTerms", MODULE, &ROOTS);
    let vectors = named();
    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();
    let normative = support::normative_host();
    let mut by_root: BTreeMap<&str, Vec<Run>> = BTreeMap::new();
    for (root, run) in vectors.values() {
        by_root.entry(root).or_default().push(run.clone());
    }
    if normative {
        let seeded = seeded_runs(EVALUATOR_SEEDED, 0x0B55_0035);
        for (root, _) in &ROOTS {
            let mut runs = by_root[root].clone();
            runs.extend(seeded[root].iter().cloned());
            let disagreements = super::evaluate(&lowered[root], &runs, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the stage-0 check disagree: {disagreements:?}"
            );
            eprintln!("BS-35: {root}: the evaluator agrees on {} runs", runs.len());
        }
        plants(&vectors);
    } else {
        eprintln!("BS-35: platform-bound host: the evaluator, its seeded runs, and the planted defects run on the normative host");
    }
    let rustc_seeded = if normative {
        seeded_runs(RUSTC_SEEDED, 0x0B55_1035)
    } else {
        BTreeMap::new()
    };
    let compiled: Vec<(&str, Vec<Run>)> = ROOTS
        .iter()
        .map(|(root, _)| {
            let mut runs = by_root[root].clone();
            runs.extend(rustc_seeded.get(root).into_iter().flatten().cloned());
            (*root, runs)
        })
        .collect();
    let roots: Vec<(&str, &super::Lowered, &[Run])> = compiled
        .iter()
        .map(|(root, runs)| (*root, &lowered[root], runs.as_slice()))
        .collect();
    let total = rustc_differential("BS-35", &roots, "stage-0 check");
    eprintln!("BS-35: {total} rustc runs agree ({:?})", started.elapsed());
    if !support::lean_backed("BS-35") {
        return;
    }
    export(&lowered);
    eprintln!("BS-35: passed in {:?}", started.elapsed());
}

// ------------------------------------------------------------- the theorems

/// The committed module as the one module of a language-1.2 project of its
/// own: it imports nothing, so it is verified alone.
fn alone(planted: Option<(&str, &str)>) -> P {
    let mut source =
        std::fs::read_to_string(repo_root().join("bootstrap").join(SOURCE).as_std_path())
            .expect("the committed source");
    if let Some((site, replacement)) = planted {
        assert!(
            source.contains(site),
            "the planted site {site:?} is in the source"
        );
        source = source.replacen(site, replacement, 1);
    }
    let header = "\\begin{lexlean}{KernelTerms}\n\\useglossary{lexlean.std.nat@1.2.0}\n\\title{Natural number addition}\n";
    assert!(source.starts_with(header), "the module's header");
    let project = P::copy_example("language-1.2");
    project.write(
        "src/Main.lex.tex",
        &source.replacen(
            header,
            "\\begin{lexlean}{Main}\n\\useglossary{lexlean.std.nat@1.2.0}\n\\useglossary{languagetwelve.titles@1.0.0}\n\\title{Typed let terms}\n",
            1,
        ),
    );
    project
}

/// The theorems of the module and their declared axioms, from its linked IR.
fn declared() -> BTreeMap<String, Vec<String>> {
    let project = alone(None);
    let snapshot = serde_json::to_value(
        project
            .engine()
            .snapshot(lexlean::CheckRequest {
                selection: lexlean::Selection::All,
            })
            .expect("the module links"),
    )
    .expect("a snapshot is JSON");
    snapshot["modules"][0]["semantic"]["declarations"]
        .as_array()
        .expect("declarations")
        .iter()
        .filter(|declaration| declaration["kind"] == "theorem")
        .map(|declaration| {
            (
                declaration["name"].as_str().expect("a name").to_owned(),
                declaration["axioms"]
                    .as_array()
                    .map(|axioms| {
                        axioms
                            .iter()
                            .map(|a| a.as_str().expect("an axiom").to_owned())
                            .collect()
                    })
                    .unwrap_or_default(),
            )
        })
        .collect()
}

/// The module verified alone: each declaration's observed axioms, once.
fn observed() -> &'static BTreeMap<String, Vec<String>> {
    static OBSERVED: OnceLock<BTreeMap<String, Vec<String>>> = OnceLock::new();
    OBSERVED.get_or_init(|| {
        // The project is kept until its attestation is read: its directory is
        // removed with it.
        let project = alone(None);
        let verified = support::verify_ok(&project);
        let attestation: Json2 = serde_json::from_slice(
            &std::fs::read(verified.root.join("attestation.json").as_std_path())
                .expect("the attestation"),
        )
        .expect("attestation JSON");
        attestation["declarations"]
            .as_array()
            .expect("declarations")
            .iter()
            .map(|row| {
                assert_eq!(row["result"], "ok", "{row}");
                (
                    row["name"]
                        .as_str()
                        .expect("a name")
                        .trim_start_matches("LanguageTwelve.Main.")
                        .to_owned(),
                    row["observed"]
                        .as_array()
                        .expect("observed")
                        .iter()
                        .map(|a| a.as_str().expect("an axiom").to_owned())
                        .collect(),
                )
            })
            .collect()
    })
}

/// Each theorem is declared with `axioms` and, under the pinned Lean, proved
/// with exactly those; then the planted module fails verification.
fn theorems(id: &str, names: &[&str], axioms: &[&str], plant: (&str, &str)) {
    let declared = declared();
    for name in names {
        let wanted: Vec<String> = if *name == "decide_total" || *name == "decide_never_refuses" {
            Vec::new()
        } else {
            axioms.iter().map(|a| (*a).to_owned()).collect()
        };
        assert_eq!(
            declared.get(*name),
            Some(&wanted),
            "{id}: `{name}` is a theorem of the module with its declared axioms"
        );
    }
    if !support::lean_backed(id) {
        return;
    }
    for name in names {
        assert_eq!(
            observed().get(*name),
            declared.get(*name),
            "{id}: `{name}` is proved with exactly its declared axioms"
        );
    }
    let _guard = support::env_shared();
    let planted = alone(Some(plant));
    let error = planted.verify_fails_with("LLV7002");
    eprintln!(
        "{id}: the planted module is refused: {}",
        error.to_string().lines().next().unwrap_or("")
    );
}

pub(super) fn bs_36() {
    theorems(
        "BS-36",
        &["declare_binds_statement"],
        &[],
        (
            "statement := statement, value := value",
            "statement := value, value := value",
        ),
    );
}

pub(super) fn bs_37() {
    theorems(
        "BS-37",
        &[
            "reflexivity_total",
            "decide_total",
            "cases_total",
            "induction_total",
            "apply_total",
            "law_total",
        ],
        &["propext"],
        (
            "| Check.pass => (ctor Produced.made (apps (ctor Term.app (ctor Term.app (apps (ctor Term.const casesOn) params) motive) scrutinee) (minorsFrom ctors 0 branches)))",
            "| Check.pass => (ctor Produced.refused (ctor Refusal.notAnEquation))",
        ),
    );
}

pub(super) fn bs_38() {
    theorems(
        "BS-38",
        &[
            "reflexivity_refuses_exactly",
            "decide_never_refuses",
            "cases_refuses_exactly",
            "induction_refuses_exactly",
            "apply_refuses_exactly",
            "law_refuses_exactly",
        ],
        &["propext"],
        (
            "(if ((countCase index branches) =? 0) then (ctor Check.fail (ctor Refusal.missingCase index))",
            "(if ((countCase index branches) =? 9) then (ctor Check.fail (ctor Refusal.missingCase index))",
        ),
    );
    // The named refusals, each with its own `Refusal`, prove with no axiom.
    theorems(
        "BS-38",
        &[
            "missing_case_refused",
            "repeated_case_refused",
            "binder_count_refused",
            "unknown_case_refused",
            "not_an_equation_refused",
            "unproduced_theorem_refused",
            "unregistered_law_refused",
            "cases_selects_by_constructor",
            "reflexivity_term_of_equation",
        ],
        &[],
        (
            "| Equation.other => (ctor Produced.refused (ctor Refusal.notAnEquation))),",
            "| Equation.other => (ctor Produced.refused (ctor Refusal.missingCase 0))),",
        ),
    );
}

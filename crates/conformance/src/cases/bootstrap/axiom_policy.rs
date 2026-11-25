//! `BS-74` (SPEC.md §17.23, §22.6): the axiom policy of a declaration and the
//! rule token of the policy table, `LexLeanBoot.AxiomPolicy`, held to the byte
//! to `lexlean::verify::policy`: `Policy::kind`, `Policy::axioms`,
//! `Policy::permits`, and `Rule::token`. Unlike the earlier bootstrap members,
//! the module was not written by hand: it is the mechanical translation of
//! `crates/lexlean/src/verify/policy.rs`, cut to these four roots and the
//! declarations they reach, with only the roots renamed. Each planted defect is
//! detected by the one vector it names.

use std::collections::BTreeMap;

use lexlean::calculus::Value;
use lexlean::verify::policy::{Policy, Rule};
use serde_json::Value as Json2;

use super::rust_validate::{eligibility_of, rustc_differential};
use super::{text, texts, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.AxiomPolicy";
const FILE: &str = "src/AxiomPolicy.lex.tex";
const ROOTS: [(&str, &[&str]); 4] = [
    ("policyKind", &["allocation"]),
    ("policyAxioms", &["allocation"]),
    ("policyPermits", &["allocation", "overflow"]),
    ("ruleToken", &["allocation"]),
];
const KIND: usize = 0;
const AXIOMS: usize = 1;
const PERMITS: usize = 2;
const TOKEN: usize = 3;

const CERTIFICATE_BUDGET: u64 = 3_145_728;
const EVALUATOR_SEEDED: usize = 300;
const RUSTC_SEEDED: usize = 2_000;

/// Axioms that occur in practice, and two that sort around them.
const POOL: &[&str] = &[
    "propext",
    "Classical.choice",
    "Quot.sound",
    "Lean.ofReduceBool",
    "Lean.trustCompiler",
    "sorryAx",
    "",
    "Z",
];

fn encode_policy(policy: &Policy) -> Value {
    match policy {
        Policy::None => Value::Adt {
            constructor: 0,
            fields: vec![],
        },
        Policy::Allow(names) => Value::Adt {
            constructor: 1,
            fields: vec![texts(names)],
        },
        Policy::Exact(names) => Value::Adt {
            constructor: 2,
            fields: vec![texts(names)],
        },
    }
}

fn encode_rule(rule: &Rule) -> Value {
    match rule {
        Rule::Document => Value::Adt {
            constructor: 0,
            fields: vec![],
        },
        Rule::Core => Value::Adt {
            constructor: 1,
            fields: vec![],
        },
        Rule::Semantic => Value::Adt {
            constructor: 2,
            fields: vec![],
        },
        Rule::Model { members } => Value::Adt {
            constructor: 3,
            fields: vec![texts(members)],
        },
        Rule::Runtime => Value::Adt {
            constructor: 4,
            fields: vec![],
        },
    }
}

fn json(value: Value) -> serde_json::Value {
    serde_json::to_value(value).expect("a value is JSON")
}

fn kind_run(policy: &Policy) -> Run {
    (vec![encode_policy(policy)], json(text(policy.kind())))
}

fn axioms_run(policy: &Policy) -> Run {
    let axioms: Vec<String> = policy.axioms().to_vec();
    (vec![encode_policy(policy)], json(texts(&axioms)))
}

fn permits_run(policy: &Policy, observed: &[String]) -> Run {
    (
        vec![encode_policy(policy), texts(observed)],
        json(Value::Bool {
            value: policy.permits(observed),
        }),
    )
}

fn token_run(rule: &Rule) -> Run {
    (vec![encode_rule(rule)], json(text(rule.token())))
}

fn owned(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

// ------------------------------------------------------------------ vectors

struct Vector {
    label: &'static str,
    root: usize,
    run: Run,
}

#[allow(clippy::too_many_lines)]
fn named() -> Vec<Vector> {
    let none = Policy::None;
    let allow = |items: &[&str]| Policy::Allow(owned(items));
    let exact = |items: &[&str]| Policy::Exact(owned(items));
    let table = vec![
        ("the kind of none", KIND, kind_run(&none)),
        ("the kind of allow", KIND, kind_run(&allow(&["propext"]))),
        ("the kind of an empty allow", KIND, kind_run(&allow(&[]))),
        ("the kind of exact", KIND, kind_run(&exact(&["propext"]))),
        ("the axioms of none", AXIOMS, axioms_run(&none)),
        (
            "the axioms of allow, in its order",
            AXIOMS,
            axioms_run(&allow(&["sorryAx", "propext"])),
        ),
        (
            "the axioms of exact, repeats kept",
            AXIOMS,
            axioms_run(&exact(&["propext", "propext"])),
        ),
        (
            "a none policy permits nothing observed",
            PERMITS,
            permits_run(&none, &[]),
        ),
        (
            "a none policy refuses any observed axiom",
            PERMITS,
            permits_run(&none, &owned(&["propext"])),
        ),
        (
            "an allow policy permits a subset",
            PERMITS,
            permits_run(&allow(&["propext", "Quot.sound"]), &owned(&["Quot.sound"])),
        ),
        (
            "an allow policy permits repeats of a named axiom",
            PERMITS,
            permits_run(&allow(&["propext"]), &owned(&["propext", "propext"])),
        ),
        (
            "an allow policy refuses an axiom it does not name",
            PERMITS,
            permits_run(
                &allow(&["propext"]),
                &owned(&["propext", "Classical.choice"]),
            ),
        ),
        (
            "an empty allow policy permits nothing observed",
            PERMITS,
            permits_run(&allow(&[]), &[]),
        ),
        (
            "an empty allow policy refuses an observed axiom",
            PERMITS,
            permits_run(&allow(&[]), &owned(&["sorryAx"])),
        ),
        (
            "an exact policy permits the same set",
            PERMITS,
            permits_run(
                &exact(&["Quot.sound", "propext"]),
                &owned(&["Quot.sound", "propext"]),
            ),
        ),
        (
            "an exact policy sorts the observed set",
            PERMITS,
            permits_run(
                &exact(&["Classical.choice", "Quot.sound", "propext"]),
                &owned(&["Quot.sound", "propext", "Classical.choice"]),
            ),
        ),
        (
            "an exact policy ignores a repeated observation",
            PERMITS,
            permits_run(
                &exact(&["Quot.sound", "propext"]),
                &owned(&["propext", "Quot.sound", "propext"]),
            ),
        ),
        (
            "an exact policy refuses a subset",
            PERMITS,
            permits_run(&exact(&["Quot.sound", "propext"]), &owned(&["propext"])),
        ),
        (
            "an exact policy refuses a superset",
            PERMITS,
            permits_run(&exact(&["propext"]), &owned(&["propext", "sorryAx"])),
        ),
        (
            "an exact policy written out of order permits nothing",
            PERMITS,
            permits_run(
                &exact(&["propext", "Quot.sound"]),
                &owned(&["Quot.sound", "propext"]),
            ),
        ),
        (
            "an exact policy with a repeat permits nothing",
            PERMITS,
            permits_run(&exact(&["propext", "propext"]), &owned(&["propext"])),
        ),
        (
            "an empty exact policy permits nothing observed",
            PERMITS,
            permits_run(&exact(&[]), &[]),
        ),
        (
            "axioms compare by bytes, not by case",
            PERMITS,
            permits_run(&exact(&["Z", "propext"]), &owned(&["propext", "Z"])),
        ),
        ("the document token", TOKEN, token_run(&Rule::Document)),
        ("the core token", TOKEN, token_run(&Rule::Core)),
        ("the semantic token", TOKEN, token_run(&Rule::Semantic)),
        (
            "the model token",
            TOKEN,
            token_run(&Rule::Model {
                members: owned(&["A.b", "A.c"]),
            }),
        ),
        ("the runtime token", TOKEN, token_run(&Rule::Runtime)),
    ];
    table
        .into_iter()
        .map(|(label, root, run)| Vector { label, root, run })
        .collect()
}

// --------------------------------------------------------------- committed

/// Every `axioms: [...]` list a committed `.lex.tex` source declares.
fn committed() -> Vec<Vec<String>> {
    let root = repo_root();
    let walker = walkdir::WalkDir::new(root.as_std_path())
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            !(entry.file_type().is_dir()
                && (matches!(name.as_ref(), "target" | ".git" | ".lexlean")
                    || support::holds_cache_tag(entry)))
        });
    let mut out = Vec::new();
    for entry in walker.flatten() {
        if !entry.file_type().is_file() || !entry.path().to_string_lossy().ends_with(".lex.tex") {
            continue;
        }
        let Ok(content) = std::fs::read_to_string(entry.path()) else {
            continue;
        };
        let mut rest = content.as_str();
        while let Some(at) = rest.find("axioms: [") {
            rest = &rest[at + "axioms: [".len()..];
            let Some(close) = rest.find(']') else { break };
            let names: Vec<String> = rest[..close]
                .split(',')
                .map(str::trim)
                .filter(|name| !name.is_empty())
                .map(str::to_owned)
                .collect();
            out.push(names);
            rest = &rest[close..];
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Each committed list as an allow and an exact policy, held against every
/// committed list as the observed set.
fn committed_runs(lists: &[Vec<String>]) -> Vec<(usize, Run)> {
    let mut out = Vec::new();
    for declared in lists {
        for policy in [
            Policy::Allow(declared.clone()),
            Policy::Exact(declared.clone()),
        ] {
            out.push((KIND, kind_run(&policy)));
            out.push((AXIOMS, axioms_run(&policy)));
            for observed in lists {
                out.push((PERMITS, permits_run(&policy, observed)));
            }
        }
    }
    out
}

// -------------------------------------------------------------------- seeds

fn seeded_list(seeded: &mut Seeded) -> Vec<String> {
    let len = seeded.below(5);
    (0..len).map(|_| (*seeded.pick(POOL)).to_owned()).collect()
}

fn seeded_policy(seeded: &mut Seeded) -> Policy {
    match seeded.below(3) {
        0 => Policy::None,
        1 => Policy::Allow(seeded_list(seeded)),
        _ => {
            // Half the exact policies are written sorted without repeats, as
            // the compiler writes them, so the permitting branch is reached.
            let mut names = seeded_list(seeded);
            if seeded.chance(50) {
                names.sort();
                names.dedup();
            }
            Policy::Exact(names)
        }
    }
}

fn seeded_runs(count: usize) -> Vec<(usize, Run)> {
    let mut seeded = Seeded(0x0B55_0074);
    (0..count)
        .map(|_| {
            let policy = seeded_policy(&mut seeded);
            match seeded.below(4) {
                0 => (KIND, kind_run(&policy)),
                1 => (AXIOMS, axioms_run(&policy)),
                _ => {
                    // An observed set drawn from the policy's own axioms
                    // half the time, so both outcomes occur.
                    let observed = if seeded.chance(50) && !policy.axioms().is_empty() {
                        let mut picked: Vec<String> = (0..1 + seeded.below(4))
                            .map(|_| seeded.pick(policy.axioms()).clone())
                            .collect();
                        if seeded.chance(30) {
                            picked.push((*seeded.pick(POOL)).to_owned());
                        }
                        picked
                    } else {
                        seeded_list(&mut seeded)
                    };
                    (PERMITS, permits_run(&policy, &observed))
                }
            }
        })
        .collect()
}

// ------------------------------------------------------------------- plants

struct Plant {
    label: &'static str,
    root: usize,
    site: &'static str,
    replacement: &'static str,
    vector: &'static str,
}

const PLANTS: &[Plant] = &[
    Plant {
        label: "a none policy permits anything",
        root: PERMITS,
        site: "Policy.None => ((AxiomOutput::listLength [Bytes] observed) =? 0)",
        replacement: "Policy.None => true",
        vector: "a none policy refuses any observed axiom",
    },
    Plant {
        label: "an allow policy never checks membership",
        root: PERMITS,
        site: "(if acc2 then (memListOfBytesEnd got3 item2) else false)",
        // The capture stays used, so the planted source still links.
        replacement:
            "(if acc2 then (if (memListOfBytesEnd got3 item2) then true else true) else false)",
        vector: "an allow policy refuses an axiom it does not name",
    },
    Plant {
        label: "an exact policy does not sort the observed set",
        root: PERMITS,
        site: "(insertsortedListOfBytesEnd acc x)",
        replacement: "(cons x acc)",
        vector: "an exact policy sorts the observed set",
    },
    Plant {
        label: "an exact policy keeps repeats",
        root: PERMITS,
        site: "(dedupListOfBytesEnd sorted5)",
        replacement: "sorted5",
        vector: "an exact policy ignores a repeated observation",
    },
    Plant {
        label: "the allow kind is spelled exact",
        root: KIND,
        site: "Policy.Allow got3 => (AxiomOutput::text \"allow\")",
        replacement: "Policy.Allow got3 => (AxiomOutput::text \"exact\")",
        vector: "the kind of allow",
    },
    Plant {
        label: "the model token is spelled semantic",
        root: TOKEN,
        site: "(AxiomOutput::text \"model\")",
        replacement: "(AxiomOutput::text \"semantic\")",
        vector: "the model token",
    },
];

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let vector = vectors
            .iter()
            .find(|vector| vector.label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
        assert_eq!(
            vector.root, plant.root,
            "{}: the vector is a run of the root planted",
            plant.label
        );
        let planted = support::P::bootstrap();
        let source = planted.read_raw(FILE);
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(FILE, &source.replacen(plant.site, plant.replacement, 1));
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[plant.root].0);
        match super::evaluate(&low, std::slice::from_ref(&vector.run), 1).first() {
            Some(first) => eprintln!(
                "BS-74: plant detected: {}: {}",
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

// --------------------------------------------------------------------- case

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
        let largest = exported_root
            .outputs
            .iter()
            .flat_map(|output| &output.certification.certificates)
            .map(|certificate| certificate.file.byte_length)
            .max()
            .expect("a certificate");
        eprintln!(
            "BS-74: {}: largest certificate part {largest} bytes",
            exported_root.root
        );
        assert!(
            largest <= CERTIFICATE_BUDGET,
            "{}: a certificate part of {largest} bytes is beyond {CERTIFICATE_BUDGET}",
            exported_root.root
        );
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
    let committed_packages = harness::tree(&harness::fixture_dir("bootstrap").join("packages"));
    assert_eq!(
        packages.keys().collect::<Vec<_>>(),
        committed_packages.keys().collect::<Vec<_>>(),
        "the exported package files are the committed ones"
    );
    for (relative, content) in &packages {
        assert!(
            committed_packages.get(relative) == Some(content),
            "packages/{relative} differs from the committed package"
        );
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("AxiomPolicy"),
        "AxiomPolicy is a linked module of the project"
    );
    eligibility_of(&project, "AxiomPolicy", MODULE, &ROOTS);

    let vectors = named();
    let labels: std::collections::BTreeSet<&str> =
        vectors.iter().map(|vector| vector.label).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let refused = vectors
        .iter()
        .filter(|vector| vector.run.1 == json(Value::Bool { value: false }))
        .count();
    assert!(refused >= 6, "at least six vectors are refused: {refused}");

    let lists = committed();
    assert!(
        lists.len() >= 3,
        "the committed sources declare {} distinct axiom lists",
        lists.len()
    );
    let committed = committed_runs(&lists);
    eprintln!(
        "BS-74: {} vectors, {} committed axiom lists, {} committed runs ({:?})",
        vectors.len(),
        lists.len(),
        committed.len(),
        started.elapsed()
    );

    let normative = support::normative_host();
    let evaluator_seeded = if normative {
        seeded_runs(EVALUATOR_SEEDED)
    } else {
        Vec::new()
    };
    let rustc_seeded = if normative {
        seeded_runs(RUSTC_SEEDED)
    } else {
        Vec::new()
    };

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();

    let per_root = |extra: &[(usize, Run)]| -> Vec<Vec<Run>> {
        let mut out: Vec<Vec<Run>> = vec![Vec::new(); ROOTS.len()];
        for vector in &vectors {
            out[vector.root].push(vector.run.clone());
        }
        for (root, run) in &committed {
            out[*root].push(run.clone());
        }
        for (root, run) in extra {
            out[*root].push(run.clone());
        }
        out
    };

    if normative {
        for (index, runs) in per_root(&evaluator_seeded).iter().enumerate() {
            let root = ROOTS[index].0;
            let disagreements = super::evaluate(&lowered[root], runs, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-74: {root}: the evaluator agrees on {} runs ({:?})",
                runs.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-74: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let compiled = per_root(&rustc_seeded);
    let suites: Vec<(&str, &super::Lowered, &[Run])> = ROOTS
        .iter()
        .enumerate()
        .map(|(index, (root, _))| (*root, &lowered[root], compiled[index].as_slice()))
        .collect();
    let total = rustc_differential("BS-74", &suites, "verify::policy");
    eprintln!("BS-74: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-74") {
        return;
    }
    export(&lowered);
    eprintln!("BS-74: passed in {:?}", started.elapsed());
}

//! `SL-15` (SPEC.md §17.22): a balanced ordered map in the standard library,
//! the `LexLeanStd.OrdMap` module, an AVL tree keyed by `Bytes` in the byte
//! order of `Order` (`SL-10`), against the `BTreeMap<Vec<u8>, Vec<u8>>` the
//! compiler (stage 0) keeps its tables in, under the calculus evaluator and
//! under the pinned rustc.
//!
//! `Map` (`SL-14`) keeps a map as an ascending list, so a lookup or an update
//! walks the list. `OrdMap` exists so that the compiler's own tables, once they
//! are source, cost what a `BTreeMap` costs: every operation descends one path
//! of a tree whose height stays logarithmic. Two roots hold it to that. The
//! first, `ordMapRun`, applies a sequence of insertions, removals, and lookups
//! and returns what each one finds before it acts and the entries left at the
//! end; its oracle is the same sequence on a `BTreeMap`. The second,
//! `ordMapBalanced`, applies the same sequence and checks the tree it leaves:
//! every stored height is right, the two sides of every node differ in height
//! by at most one, and the keys ascend strictly. Its oracle is `true`, so a
//! defect that keeps every answer right but lets the tree degrade into a list
//! is still caught.

use std::collections::BTreeMap;

use lexlean::calculus::rust::package::Passing;
use lexlean::calculus::rust::{self, Profile};
use lexlean::calculus::{self as target, interp, Value};
use lexlean::production::lower::Lowered;
use serde_json::Value as Json;

use super::stdlib::{lower_in, Run};
use crate::production_export as harness;
use crate::rust_build::{cargo_in, workspace, Built};
use crate::rust_harness::{self, Calls};
use crate::rust_packages;
use crate::support::{self, P};

/// The module.
const MODULE: &str = "LexLeanStd.OrdMap";
/// The module's source, for the plants.
const FILE: &str = "src/OrdMap.lex.tex";

/// The roots in the order the module declares them, each with the effects its
/// declaration admits, which are exactly the effects its closure realizes.
const ROOTS: [(&str, &[&str]); 2] = [
    ("ordMapRun", &["allocation", "overflow", "recursion"]),
    ("ordMapBalanced", &["allocation", "overflow", "recursion"]),
];

/// Operation sequences per root under the evaluator.
const EVALUATOR_RUNS: usize = 300;
/// Operation sequences per root under rustc. Each run is a literal of the
/// harness rustc compiles, and a sequence is long, so fewer runs than a
/// member of `Map` takes keep the harness within a runner's memory.
const RUSTC_RUNS: usize = 600;
/// The evaluator's fuel; a root that exhausts it fails the case.
const FUEL: u64 = 200_000_000;
/// The most operations a seeded sequence holds.
const MAX_OPS: usize = 32;

/// The bytes a key or a value is drawn from: few enough that keys repeat and
/// one is often a prefix of another, with both ends of the byte range.
const KEY_BYTES: &[u8] = &[0x00, 0x01, 0x61, 0x62, 0x7f, 0x80, 0xff];

/// The operation codes the roots read.
const INSERT: u64 = 0;
const REMOVE: u64 = 1;
const GET: u64 = 2;

/// One operation: its code, the key, and the value an insertion stores.
type Op = (u64, Vec<u8>, Vec<u8>);

/// A deterministic generator (xorshift64*), so the corpus is the same on
/// every host.
struct Seeded(u64);

impl Seeded {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn below(&mut self, bound: usize) -> usize {
        usize::try_from(self.next() % u64::try_from(bound).expect("a bound")).expect("below")
    }
    /// A key or a value of at most three bytes over `KEY_BYTES`.
    fn key(&mut self) -> Vec<u8> {
        let length = self.below(4);
        (0..length)
            .map(|_| KEY_BYTES[self.below(KEY_BYTES.len())])
            .collect()
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
fn bytes(value: &[u8]) -> Value {
    Value::Bytes { hex: hex(value) }
}
fn pair(left: Value, right: Value) -> Value {
    Value::Pair {
        left: Box::new(left),
        right: Box::new(right),
    }
}
fn list(items: impl IntoIterator<Item = Value>) -> Value {
    Value::List {
        items: items.into_iter().collect(),
    }
}
fn option(value: Option<&Vec<u8>>) -> Value {
    match value {
        Some(value) => Value::Some {
            value: Box::new(bytes(value)),
        },
        None => Value::None,
    }
}
fn json(value: &Value) -> Json {
    serde_json::to_value(value).expect("a value is JSON")
}

/// The argument of both roots: the operations as `(code, (key, value))`.
fn ops_value(ops: &[Op]) -> Value {
    list(ops.iter().map(|(code, key, value)| {
        pair(
            Value::Nat {
                value: code.to_string(),
            },
            pair(bytes(key), bytes(value)),
        )
    }))
}

/// What `ordMapRun` returns on `ops`: what the map holds at each operation's
/// key just before it acts, and the entries left at the end, as a
/// `BTreeMap` gives them.
fn run_oracle(ops: &[Op]) -> Value {
    let mut map: BTreeMap<Vec<u8>, Vec<u8>> = BTreeMap::new();
    let mut seen = Vec::new();
    for (code, key, value) in ops {
        seen.push(option(map.get(key)));
        match *code {
            INSERT => {
                map.insert(key.clone(), value.clone());
            }
            REMOVE => {
                map.remove(key);
            }
            _ => {}
        }
    }
    pair(
        list(seen),
        list(
            map.iter()
                .map(|(key, value)| pair(bytes(key), bytes(value))),
        ),
    )
}

fn run_of(root: &str, ops: &[Op]) -> Run {
    let expected = match root {
        "ordMapRun" => run_oracle(ops),
        "ordMapBalanced" => Value::Bool { value: true },
        other => panic!("no root {other}"),
    };
    (vec![ops_value(ops)], json(&expected))
}

fn op(code: u64, key: &str, value: &str) -> Op {
    (code, key.as_bytes().to_vec(), value.as_bytes().to_vec())
}
fn inserts(keys: &[&str]) -> Vec<Op> {
    keys.iter().map(|key| op(INSERT, key, key)).collect()
}
/// `count` keys of two bytes that ascend, the order that unbalances a plain
/// binary tree on the right.
fn ascending(count: u8) -> Vec<Op> {
    (0..count)
        .map(|index| (INSERT, vec![0x61, index], vec![index]))
        .collect()
}

/// The named sequences: the edge cases of both roots.
fn named() -> Vec<(&'static str, Vec<Op>)> {
    let mut descending = ascending(40);
    descending.reverse();
    let mut drained = ascending(40);
    drained.extend((0..40u8).map(|index| (REMOVE, vec![0x61, index], Vec::new())));
    let mut thinned = ascending(40);
    thinned.extend(
        (0..40u8)
            .step_by(3)
            .map(|index| (REMOVE, vec![0x61, index], Vec::new())),
    );
    thinned.extend((0..40u8).map(|index| (GET, vec![0x61, index], Vec::new())));
    vec![
        ("nothing", Vec::new()),
        ("a lookup in the empty map", vec![op(GET, "a", "")]),
        ("a removal from the empty map", vec![op(REMOVE, "a", "")]),
        (
            "a lookup to the left of the root",
            [inserts(&["b", "a", "c"]), vec![op(GET, "a", "")]].concat(),
        ),
        (
            "an insertion replaces the value of an equal key",
            vec![op(INSERT, "a", "1"), op(INSERT, "a", "2"), op(GET, "a", "")],
        ),
        (
            "the empty key and its extensions",
            [inserts(&["", "a", "ab", "\u{0}"]), vec![op(GET, "", "")]].concat(),
        ),
        (
            "removing a node with two children keeps both sides",
            [
                inserts(&["b", "a", "c"]),
                vec![op(REMOVE, "b", ""), op(GET, "c", "")],
            ]
            .concat(),
        ),
        (
            "descending insertions rotate right",
            inserts(&["c", "b", "a"]),
        ),
        (
            "ascending insertions rotate left",
            inserts(&["a", "b", "c"]),
        ),
        (
            "a left-right zigzag needs a double rotation",
            inserts(&["c", "a", "b"]),
        ),
        (
            "a right-left zigzag needs a double rotation",
            inserts(&["a", "c", "b"]),
        ),
        ("two insertions", inserts(&["a", "b"])),
        ("forty ascending insertions", ascending(40)),
        ("forty descending insertions", descending),
        ("forty insertions then forty removals", drained),
        ("forty insertions, every third removed", thinned),
    ]
}

/// The corpus of one root: the named sequences, then seeded ones up to
/// `runs`, each with its oracle's outcome.
fn corpus(root: &str, runs: usize) -> Vec<Run> {
    let mut out: Vec<Run> = named().iter().map(|(_, ops)| run_of(root, ops)).collect();
    let mut seeded = Seeded(0x51_15_0D_AB);
    while out.len() < runs {
        let length = seeded.below(MAX_OPS + 1);
        let ops: Vec<Op> = (0..length)
            .map(|_| {
                // Insertions outnumber the rest, so the trees grow.
                let code = match seeded.below(6) {
                    0..=2 => INSERT,
                    3 | 4 => REMOVE,
                    _ => GET,
                };
                (code, seeded.key(), seeded.key())
            })
            .collect();
        out.push(run_of(root, &ops));
    }
    out
}

/// A defect planted in the source of `OrdMap`, caught on a named sequence of
/// its own run through one root.
struct Plant {
    label: &'static str,
    root: &'static str,
    site: &'static str,
    replacement: &'static str,
    vector: &'static str,
}

const PLANTS: [Plant; 6] = [
    Plant {
        label: "a lookup searches the wrong side",
        root: "ordMapRun",
        site: "(mapGet ['Value] left key) else (mapGet ['Value] right key)",
        replacement: "(mapGet ['Value] right key) else (mapGet ['Value] left key)",
        vector: "a lookup to the left of the root",
    },
    Plant {
        label: "an insertion keeps the old value of an equal key",
        root: "ordMapRun",
        site: "then (ctor OrdMap.node ['Value] left key value height right)",
        replacement: "then (ctor OrdMap.node ['Value] left here held height right)",
        vector: "an insertion replaces the value of an equal key",
    },
    Plant {
        label: "a removal drops the right side of a node with two children",
        root: "ordMapRun",
        site: "(balanced ['Value] left (first least) (second least) (second taken))",
        replacement: "left",
        vector: "removing a node with two children keeps both sides",
    },
    Plant {
        label: "a left-heavy node is never rotated",
        root: "ordMapBalanced",
        site: "(if ((hr + 1) <? hl) then (rotateFromLeft ['Value] left key value right)",
        replacement: "(if ((hr + 1) <? hl) then (joinNode ['Value] left key value right)",
        vector: "descending insertions rotate right",
    },
    Plant {
        label: "a left-right zigzag is rotated once, not twice",
        root: "ordMapBalanced",
        site: "(if ((heightOf ['Value] inner) <? ((heightOf ['Value] outer) + 1)) then (joinNode ['Value] outer lkey lvalue",
        replacement: "(if true then (joinNode ['Value] outer lkey lvalue",
        vector: "a left-right zigzag needs a double rotation",
    },
    Plant {
        label: "a node's stored height is one too many",
        root: "ordMapBalanced",
        site: "(heightOf ['Value] right)) + 1) right)",
        replacement: "(heightOf ['Value] right)) + 2) right)",
        vector: "two insertions",
    },
];

/// Run one lowered root on one argument list under the evaluator.
fn observe(lowered: &Lowered, arguments: &[Value]) -> Json {
    let outcome = interp::run(&lowered.program, FUEL, lowered.entry(), arguments);
    rust::observable(&outcome).unwrap_or_else(|| panic!("on {arguments:?}: {outcome:?}"))
}

/// Every run of `corpus` on which the evaluator and the oracle disagree.
fn evaluate(lowered: &Lowered, corpus: &[Run]) -> Vec<String> {
    corpus
        .iter()
        .enumerate()
        .filter_map(|(run, (arguments, expected))| {
            let observed = observe(lowered, arguments);
            (&observed != expected).then(|| format!("run {run}: {observed} != {expected}"))
        })
        .collect()
}

fn snake(root: &str) -> String {
    let mut out = String::new();
    for character in root.chars() {
        if character.is_ascii_uppercase() {
            out.push('_');
            out.push(character.to_ascii_lowercase());
        } else {
            out.push(character);
        }
    }
    out
}

fn eligibility(project: &P) {
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report: Json = serde_json::from_slice(
        &std::fs::read(
            project
                .build_dir(&build_id)
                .join("production/LexLeanStd/OrdMap.eligibility.json")
                .as_std_path(),
        )
        .expect("the eligibility report"),
    )
    .expect("the report is JSON");
    let reported = report["roots"].as_array().expect("roots");
    assert_eq!(
        reported
            .iter()
            .map(|row| row["root"].as_str().expect("root").to_owned())
            .collect::<Vec<_>>(),
        ROOTS
            .iter()
            .map(|(root, _)| format!("{MODULE}.{root}"))
            .collect::<Vec<_>>(),
        "the two roots are the module's production roots, in order"
    );
    for (row, (root, effects)) in reported.iter().zip(ROOTS) {
        let declared: Vec<&str> = row["declared_effects"]
            .as_array()
            .expect("declared effects")
            .iter()
            .map(|effect| effect.as_str().expect("an effect"))
            .collect();
        assert_eq!(declared, effects, "{root}: the declared effects");
        let targets = row["targets"].as_array().expect("targets");
        assert_eq!(targets.len(), 1, "{root}: one target");
        let target = &targets[0];
        assert_eq!(target["target"], "rust-std", "{root}: the target");
        assert_eq!(target["status"], "eligible", "{root}: eligible");
        let realized: Vec<&str> = target["effects"]
            .as_array()
            .expect("effects")
            .iter()
            .map(|effect| effect["effect"].as_str().expect("an effect"))
            .collect();
        assert_eq!(
            realized, effects,
            "{root}: the declaration admits exactly the realized effects"
        );
    }
}

/// Run the case for `SL-15`.
///
/// # Panics
///
/// Panics when the case's assertion fails.
#[allow(clippy::too_many_lines)]
pub fn run(id: &str) {
    assert_eq!(id, "SL-15", "the case of {MODULE}");
    let project = P::stdlib();
    project.check_ok();
    eligibility(&project);
    let names: std::collections::BTreeSet<&str> = named().iter().map(|(name, _)| *name).collect();
    assert_eq!(
        names.len(),
        named().len(),
        "each named sequence has its own name"
    );

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, lower_in(&checked, &project, MODULE, root)))
        .collect();
    for (root, _) in ROOTS {
        let disagreements = evaluate(&lowered[root], &corpus(root, EVALUATOR_RUNS));
        assert!(
            disagreements.is_empty(),
            "{root}: the evaluator and the oracle disagree on {} of {EVALUATOR_RUNS} runs: {:?}",
            disagreements.len(),
            &disagreements[..disagreements.len().min(5)]
        );
        eprintln!("SL-15: {root}: the evaluator agrees on {EVALUATOR_RUNS} runs");
    }

    // The rendered rust-std package of each root, compiled by the pinned
    // rustc, runs the larger corpus.
    let dir = tempfile::Builder::new()
        .prefix("lexlean-ordmap-")
        .tempdir()
        .expect("tempdir");
    let mut crates = Vec::new();
    let mut expected: BTreeMap<String, Vec<Json>> = BTreeMap::new();
    for (root, _) in ROOTS {
        let low = &lowered[root];
        let manifest = rust_packages::manifest(
            &format!("ordmap_{}", snake(root)),
            &low.program,
            low.entry(),
            Profile::Std,
        );
        let package = target::package(&manifest.to_file_bytes().expect("canonical manifest"))
            .unwrap_or_else(|failure| panic!("{root}: {failure:?}"));
        let runs = corpus(root, RUSTC_RUNS);
        let arguments: Vec<Vec<Value>> = runs
            .iter()
            .map(|(arguments, _)| arguments.clone())
            .collect();
        let arity = low.program.functions[usize::try_from(low.entry()).expect("entry")]
            .types
            .len();
        let passing = vec![Passing::Own; arity];
        let harness_source = rust_harness::render_calls(
            &low.program,
            Profile::Std,
            &manifest.name,
            &[Calls {
                entry: low.entry(),
                function: "run",
                passing: &passing,
                arguments: &arguments,
            }],
        )
        .expect("the differential harness");
        expected.insert(
            manifest.name.clone(),
            runs.into_iter().map(|(_, expected)| expected).collect(),
        );
        crates.push(Built {
            name: manifest.name,
            files: package.files,
            harness: harness_source,
        });
    }
    workspace(dir.path(), &crates);
    let build = cargo_in(
        dir.path(),
        &["build", "--offline", "--workspace", "--quiet"],
    );
    assert!(
        build.status.success(),
        "a root's package does not build:\n{}",
        String::from_utf8_lossy(&build.stderr)
    );
    let mut binaries = Vec::new();
    for (name, outcomes) in &expected {
        let binary = dir
            .path()
            .join("target/debug")
            .join(format!("run_{name}{}", std::env::consts::EXE_SUFFIX));
        let ran = std::process::Command::new(&binary)
            .output()
            .expect("the differential runs");
        assert!(
            ran.status.success(),
            "{name}: {}",
            String::from_utf8_lossy(&ran.stderr)
        );
        binaries.push((name.clone(), binary));
        let stdout = String::from_utf8(ran.stdout).expect("utf8");
        let printed: Vec<Json> = stdout
            .lines()
            .filter(|line| !line.starts_with("work "))
            .map(|line| {
                serde_json::from_str(line).unwrap_or_else(|error| panic!("{name}: {error}: {line}"))
            })
            .collect();
        assert_eq!(printed.len(), outcomes.len(), "{name}: one outcome per run");
        let disagreements: Vec<String> = printed
            .iter()
            .zip(outcomes)
            .enumerate()
            .filter(|(_, (printed, expected))| printed != expected)
            .map(|(run, (printed, expected))| format!("run {run}: {printed} != {expected}"))
            .collect();
        assert!(
            disagreements.is_empty(),
            "{name}: the rendering and the oracle disagree on {} of {} runs: {:?}",
            disagreements.len(),
            outcomes.len(),
            &disagreements[..disagreements.len().min(5)]
        );
        eprintln!("SL-15: {name}: rustc agrees on {} runs", outcomes.len());
    }

    // The gate is armed: each plant is a defect the named sequence of its own
    // catches through its root, on a module the committed one agrees with on
    // that sequence.
    let sequences: BTreeMap<&str, Vec<Op>> = named().into_iter().collect();
    let mut missed = Vec::new();
    for plant in &PLANTS {
        let ops = sequences
            .get(plant.vector)
            .unwrap_or_else(|| panic!("{}: no sequence `{}`", plant.label, plant.vector));
        let (arguments, expected) = run_of(plant.root, ops);
        assert_eq!(
            observe(&lowered[plant.root], &arguments),
            expected,
            "{}: the committed module agrees with its sequence",
            plant.label
        );
        let planted = P::stdlib();
        let text = planted.read_raw(FILE);
        assert_eq!(
            text.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(FILE, &text.replacen(plant.site, plant.replacement, 1));
        let checked = support::checked_project(&planted);
        let broken = lower_in(&checked, &planted, MODULE, plant.root);
        let observed = observe(&broken, &arguments);
        if observed == expected {
            missed.push(plant.label);
        } else {
            eprintln!(
                "SL-15: plant detected: {}: {observed} != {expected}",
                plant.label
            );
        }
    }
    assert!(
        missed.is_empty(),
        "planted defects their sequences do not detect: {missed:?}"
    );
    // The measured stack bound of every package comes last, so that a bound
    // the record lacks hides none of the results above.
    for (name, binary) in &binaries {
        crate::stack_bounds::observe(&crate::stack_bounds::subject("stdlib", name), binary);
    }
    // Under the pinned Lean, the production export certifies both roots; its
    // packages are the committed ones and realize the programs the
    // differential ran.
    if !support::lean_backed(id) {
        return;
    }
    let exported = harness::exported("stdlib");
    let bundle = exported.bundle();
    let packages = harness::tree(&bundle.join("packages"));
    let committed = harness::tree(&harness::fixture_dir("stdlib").join("packages"));
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
    let mut realized = 0usize;
    for root in &exported.output.result.roots {
        let Some(member) = root.root.strip_prefix(&format!("{MODULE}.")) else {
            continue;
        };
        for output in &root.outputs {
            let package: Json = serde_json::from_slice(
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
                serde_json::to_value(&lowered[member].program).expect("a program is JSON"),
                "{member}: the exported package realizes the program the differential ran"
            );
            realized += 1;
        }
    }
    assert_eq!(realized, ROOTS.len(), "one certified package per root");
}

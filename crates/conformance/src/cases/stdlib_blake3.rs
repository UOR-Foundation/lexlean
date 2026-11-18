//! BLAKE3 in the standard library (SPEC.md §17.22, `SL-16`, #440): the
//! `LexLeanStd.Blake3` module against two oracles written independently of
//! it, the official BLAKE3 test vectors (`tests/stdlib/blake3/`) and the
//! `blake3` crate on a seeded corpus, under the calculus evaluator and under
//! the pinned rustc; and its piece members against the `blake3` crate's own
//! subtree API (`hazmat`), so that the chaining values of the 1 MiB pieces of
//! an input, merged by `pieceRoot`, are the input's BLAKE3 hash.
//!
//! An oracle is evidence, not authority: a disagreement fails the case and
//! is a defect of the module or of the pipeline, and nothing here adopts an
//! oracle's digest.

use std::collections::BTreeMap;

use blake3::hazmat::HasherExt;
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
const MODULE: &str = "LexLeanStd.Blake3";

/// The members in the order the module declares them, each with the effects
/// its declaration admits, which are exactly the effects its closure
/// realizes: `Bytes` and the stack of chaining values allocate, and the
/// positions and counters are `Nat` arithmetic realized in 64 bits.
const MEMBERS: [(&str, &[&str]); 4] = [
    ("blake3", &["allocation", "overflow"]),
    ("blake3Hex", &["allocation", "overflow"]),
    ("pieceChain", &["allocation", "overflow"]),
    ("pieceRoot", &["allocation", "overflow"]),
];

/// Bytes per piece, as `pieceChain` counts them: 1 MiB, 1024 chunks.
const PIECE: usize = 1 << 20;

/// Seeded messages per whole-input member under the evaluator, beside every
/// vector.
const EVALUATOR_RUNS: usize = 120;
/// Seeded messages per whole-input member under rustc, beside every vector.
const RUSTC_RUNS: usize = 600;
/// The evaluator's fuel: the longest official vector (102400 bytes, 100
/// chunks) finishes well within it, and a member that exhausts it fails the
/// case.
const FUEL: u64 = 4_000_000_000;
/// The longest official vector the evaluator runs; longer ones run under
/// rustc.
const EVALUATOR_LONGEST: usize = 8_193;
/// Whole inputs whose pieces the piece members are checked on: one piece, a
/// piece and one byte, two whole pieces, and an odd count of pieces with a
/// partial last piece, so the stack of `pieceRoot` both merges and carries.
const PIECED: [usize; 5] = [
    PIECE - 1,
    PIECE + 1,
    2 * PIECE,
    3 * PIECE + 4_097,
    5 * PIECE - 7,
];

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
    fn message(&mut self, length: usize) -> Vec<u8> {
        (0..length)
            .map(|_| u8::try_from(self.below(256)).expect("a byte"))
            .collect()
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn bytes(value: &[u8]) -> Value {
    Value::Bytes { hex: hex(value) }
}

fn nat(value: usize) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}

/// The `Digest` record of a 32-byte digest: the structure's one constructor
/// with its eight little-endian `UInt32` words.
fn digest(value: &[u8; 32]) -> Value {
    Value::Adt {
        constructor: 0,
        fields: value
            .chunks(4)
            .map(|word| Value::U32 {
                value: u32::from_le_bytes(word.try_into().expect("four bytes")).to_string(),
            })
            .collect(),
    }
}

fn json(value: &Value) -> Json {
    serde_json::to_value(value).expect("a value is JSON")
}

/// A whole-input member's expected outcome for a 32-byte digest.
fn outcome(member: &str, value: &[u8; 32]) -> Json {
    match member {
        "blake3" => json(&digest(value)),
        "blake3Hex" => json(&bytes(hex(value).as_bytes())),
        other => panic!("`{other}` is not a whole-input member of {MODULE}"),
    }
}

/// The chaining value of piece `index` of `total` with content `piece`, by
/// the `blake3` crate's subtree API: a one-piece input's only piece is its
/// root, any other piece a non-root subtree that begins at its byte offset.
fn piece_oracle(piece: &[u8], index: usize, total: usize) -> [u8; 32] {
    if total == 1 {
        return *blake3::hash(piece).as_bytes();
    }
    blake3::Hasher::new()
        .set_input_offset(u64::try_from(index * PIECE).expect("an offset"))
        .update(piece)
        .finalize_non_root()
}

/// The official vectors: the input is the repeating sequence 0, 1, ..., 250
/// of the given length, and the hash mode's output begins with the 32-byte
/// digest.
fn official() -> Vec<(Vec<u8>, [u8; 32])> {
    let path = support::repo_root().join("tests/stdlib/blake3/test_vectors.json");
    let text = std::fs::read_to_string(path.as_std_path()).expect("the official vectors");
    let parsed: Json = serde_json::from_str(&text).expect("the vectors are JSON");
    let cases = parsed["cases"].as_array().expect("cases");
    assert!(cases.len() >= 35, "{} official vectors", cases.len());
    cases
        .iter()
        .map(|case| {
            let length =
                usize::try_from(case["input_len"].as_u64().expect("input_len")).expect("a length");
            let message: Vec<u8> = (0..length)
                .map(|at| u8::try_from(at % 251).expect("a byte"))
                .collect();
            let text = case["hash"].as_str().expect("hash");
            let mut value = [0_u8; 32];
            for (at, byte) in value.iter_mut().enumerate() {
                *byte = u8::from_str_radix(&text[2 * at..2 * at + 2], 16).expect("hexadecimal");
            }
            // The vector's digest is one oracle and the crate another; they
            // agree with each other before either judges the module.
            assert_eq!(
                blake3::hash(&message).as_bytes(),
                &value,
                "the official vector of {length} bytes and the `blake3` crate disagree"
            );
            (message, value)
        })
        .collect()
}

/// The corpus of a whole-input member: every official vector up to
/// `longest` bytes, then `runs` seeded messages whose lengths cover every
/// block boundary of the first chunk and the chunk boundaries after it, then
/// random lengths up to `ceiling`, each with the digest the `blake3` crate
/// computes.
fn corpus(member: &str, runs: usize, longest: usize, ceiling: usize) -> Vec<Run> {
    let mut out: Vec<Run> = official()
        .into_iter()
        .filter(|(message, _)| message.len() <= longest)
        .map(|(message, value)| (vec![bytes(&message)], outcome(member, &value)))
        .collect();
    let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
    for character in member.bytes() {
        seed = seed.rotate_left(8) ^ u64::from(character);
    }
    let mut seeded = Seeded(seed);
    let boundaries = [
        63, 64, 65, 127, 128, 129, 1023, 1024, 1025, 2047, 2048, 2049, 3071, 3072, 3073, 4095,
        4096, 4097,
    ];
    let mut index = 0usize;
    while index < runs {
        let length = if index < boundaries.len() {
            boundaries[index]
        } else {
            seeded.below(ceiling)
        };
        let message = seeded.message(length);
        out.push((
            vec![bytes(&message)],
            outcome(member, blake3::hash(&message).as_bytes()),
        ));
        index += 1;
    }
    out
}

/// The seeded inputs of `PIECED`, each cut into its 1 MiB pieces.
fn pieced() -> Vec<(Vec<u8>, Vec<Vec<u8>>)> {
    let mut seeded = Seeded(0x0b1a_3e3e_5eed_0001_u64);
    PIECED
        .iter()
        .map(|length| {
            let message = seeded.message(*length);
            let pieces = message.chunks(PIECE).map(<[u8]>::to_vec).collect();
            (message, pieces)
        })
        .collect()
}

/// The runs of `pieceChain`: every piece of every pieced input when
/// `whole_pieces`, else only the last piece of each (which is shorter than a
/// piece and so cheap for the evaluator), each against `piece_oracle`.
fn piece_chain_runs(whole_pieces: bool) -> Vec<Run> {
    let mut out = Vec::new();
    for (_, pieces) in pieced() {
        let total = pieces.len();
        for (index, piece) in pieces.iter().enumerate() {
            if !whole_pieces && index + 1 != total {
                continue;
            }
            if !whole_pieces && piece.len() == PIECE {
                continue;
            }
            out.push((
                vec![bytes(piece), nat(index), nat(total)],
                json(&digest(&piece_oracle(piece, index, total))),
            ));
        }
    }
    out
}

/// The runs of `pieceRoot`: the chaining values of every piece of every
/// pieced input, by the `blake3` crate's subtree API, against the crate's
/// hash of the whole input. The merge is a handful of compressions, so the
/// evaluator runs these as rustc does.
fn piece_root_runs() -> Vec<Run> {
    pieced()
        .into_iter()
        .map(|(message, pieces)| {
            let total = pieces.len();
            let chains = pieces
                .iter()
                .enumerate()
                .map(|(index, piece)| digest(&piece_oracle(piece, index, total)))
                .collect();
            (
                vec![Value::List { items: chains }],
                json(&digest(blake3::hash(&message).as_bytes())),
            )
        })
        .collect()
}

/// A member's runs under the evaluator.
fn evaluator_runs(member: &str) -> Vec<Run> {
    match member {
        "blake3" | "blake3Hex" => corpus(member, EVALUATOR_RUNS, EVALUATOR_LONGEST, 2_100),
        "pieceChain" => piece_chain_runs(false),
        "pieceRoot" => piece_root_runs(),
        other => panic!("`{other}` is not a member of {MODULE}"),
    }
}

/// A member's runs under rustc.
fn rustc_runs(member: &str) -> Vec<Run> {
    match member {
        "blake3" | "blake3Hex" => corpus(member, RUSTC_RUNS, usize::MAX, 16_384),
        "pieceChain" => piece_chain_runs(true),
        "pieceRoot" => piece_root_runs(),
        other => panic!("`{other}` is not a member of {MODULE}"),
    }
}

/// Run the case for `SL-16`.
///
/// # Panics
///
/// Panics when the case's assertion fails.
pub(super) fn run(id: &str) {
    assert_eq!(id, "SL-16");
    // The evaluator recurses once per step of a compression, so the case
    // runs on a stack sized for the longest vector it evaluates.
    let outcome = std::thread::Builder::new()
        .stack_size(256 << 20)
        .spawn(run_on_stack)
        .expect("thread")
        .join();
    if let Err(panic) = outcome {
        std::panic::resume_unwind(panic);
    }
}

#[allow(clippy::too_many_lines)]
fn run_on_stack() {
    let project = P::stdlib();
    project.check_ok();
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report: Json = serde_json::from_slice(
        &std::fs::read(
            project
                .build_dir(&build_id)
                .join("production/LexLeanStd/Blake3.eligibility.json")
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
        MEMBERS
            .iter()
            .map(|(member, _)| format!("{MODULE}.{member}"))
            .collect::<Vec<_>>(),
        "the four members are the module's production roots, in order"
    );
    for (row, (member, effects)) in reported.iter().zip(MEMBERS) {
        let declared: Vec<&str> = row["declared_effects"]
            .as_array()
            .expect("declared effects")
            .iter()
            .map(|effect| effect.as_str().expect("an effect"))
            .collect();
        assert_eq!(declared, effects, "{member}: the declared effects");
        let targets = row["targets"].as_array().expect("targets");
        assert_eq!(targets.len(), 1, "{member}: one target");
        let target = &targets[0];
        assert_eq!(target["target"], "rust-std", "{member}: the target");
        assert_eq!(target["status"], "eligible", "{member}: eligible");
        let realized: Vec<&str> = target["effects"]
            .as_array()
            .expect("effects")
            .iter()
            .map(|effect| effect["effect"].as_str().expect("an effect"))
            .collect();
        assert_eq!(
            realized, effects,
            "{member}: the declaration admits exactly the realized effects"
        );
    }
    // Every member lowers once; the evaluator runs the vectors, the seeded
    // corpus, and the piece runs.
    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, Lowered> = MEMBERS
        .iter()
        .map(|(member, _)| (*member, lower_in(&checked, &project, MODULE, member)))
        .collect();
    for (member, _) in MEMBERS {
        let runs = evaluator_runs(member);
        let disagreements = evaluate(&lowered[member], &runs);
        assert!(
            disagreements.is_empty(),
            "{member}: the evaluator and the oracles disagree on {} of {} runs: {:?}",
            disagreements.len(),
            runs.len(),
            &disagreements[..disagreements.len().min(3)]
        );
    }
    // The rendered rust-std package of every member, compiled by the pinned
    // rustc, runs every vector, the larger corpus, and every whole piece.
    let dir = tempfile::Builder::new()
        .prefix("lexlean-stdlib-blake3-")
        .tempdir()
        .expect("tempdir");
    let mut crates = Vec::new();
    let mut expected: BTreeMap<String, (&str, Vec<Json>)> = BTreeMap::new();
    for (member, _) in MEMBERS {
        let low = &lowered[member];
        let manifest = rust_packages::manifest(
            &format!("blake3_{}", member.to_ascii_lowercase()),
            &low.program,
            low.entry(),
            Profile::Std,
        );
        let package = target::package(&manifest.to_file_bytes().expect("canonical manifest"))
            .unwrap_or_else(|failure| panic!("{member}: {failure:?}"));
        let runs = rustc_runs(member);
        let arguments: Vec<Vec<Value>> = runs
            .iter()
            .map(|(arguments, _)| arguments.clone())
            .collect();
        let passing = vec![Passing::Own; arguments.first().map_or(1, Vec::len)];
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
            (
                member,
                runs.into_iter().map(|(_, expected)| expected).collect(),
            ),
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
        &["build", "--offline", "--release", "--workspace", "--quiet"],
    );
    assert!(
        build.status.success(),
        "a member's package does not build:\n{}",
        String::from_utf8_lossy(&build.stderr)
    );
    for (name, (member, outcomes)) in &expected {
        let binary = dir
            .path()
            .join("target/release")
            .join(format!("run_{name}{}", std::env::consts::EXE_SUFFIX));
        let ran = std::process::Command::new(&binary)
            .output()
            .expect("the differential runs");
        assert!(
            ran.status.success(),
            "{name}: {}",
            String::from_utf8_lossy(&ran.stderr)
        );
        crate::stack_bounds::observe(&format!("stdlib/{name}"), &binary);
        let stdout = String::from_utf8(ran.stdout).expect("utf8");
        let printed: Vec<Json> = stdout
            .lines()
            .filter(|line| !line.starts_with("work "))
            .map(|line| {
                serde_json::from_str(line).unwrap_or_else(|error| panic!("{name}: {error}: {line}"))
            })
            .collect();
        assert_eq!(
            printed.len(),
            outcomes.len(),
            "{member}: one outcome per run"
        );
        let disagreements: Vec<String> = printed
            .iter()
            .zip(outcomes)
            .enumerate()
            .filter(|(_, (printed, expected))| printed != expected)
            .map(|(run, (printed, expected))| format!("run {run}: {printed} != {expected}"))
            .collect();
        assert!(
            disagreements.is_empty(),
            "{member}: the rendering and the oracles disagree on {} of {} runs: {:?}",
            disagreements.len(),
            outcomes.len(),
            &disagreements[..disagreements.len().min(3)]
        );
    }
    // The gate is armed: one bit of the quarter-round flipped (the last
    // word `a` it returns, exclusive-or 1) and the empty input's vector
    // disagrees.
    let planted = P::stdlib();
    let text = planted.read_raw("src/Blake3.lex.tex");
    let site = "(record Quad {a := a2,";
    assert_eq!(text.matches(site).count(), 1, "the plant site exists once");
    planted.write(
        "src/Blake3.lex.tex",
        &text.replacen(
            site,
            "(record Quad {a := (prim bit_xor a2 (1 : UInt32) : UInt32),",
            1,
        ),
    );
    let checked = support::checked_project(&planted);
    let hex_root = lower_in(&checked, &planted, MODULE, "blake3Hex");
    let first: Vec<Run> = corpus("blake3Hex", 0, 0, 1);
    assert_eq!(
        first.len(),
        1,
        "the empty input is the one vector of length 0"
    );
    let disagreements = evaluate(&hex_root, &first);
    assert_eq!(
        disagreements.len(),
        1,
        "a planted one-bit error in the quarter-round is detected on the empty input"
    );
    // Under the pinned Lean, the production export certifies the four
    // members; their packages are the committed ones and realize the
    // programs the differential ran.
    if !support::lean_backed("SL-16") {
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
    assert_eq!(realized, MEMBERS.len(), "one certified package per member");
}

/// Every run of `corpus` on which the evaluator and the oracle disagree.
fn evaluate(lowered: &Lowered, corpus: &[Run]) -> Vec<String> {
    corpus
        .iter()
        .enumerate()
        .filter_map(|(run, (arguments, expected))| {
            let outcome = interp::run(&lowered.program, FUEL, lowered.entry(), arguments);
            let observed =
                rust::observable(&outcome).unwrap_or_else(|| panic!("run {run}: {outcome:?}"));
            (&observed != expected).then(|| format!("run {run}: {observed} != {expected}"))
        })
        .collect()
}

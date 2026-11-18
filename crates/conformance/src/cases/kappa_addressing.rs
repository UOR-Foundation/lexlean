//! κ-addressing (SPEC.md §17.22, decision D7 of #110, #127): the `kappa`
//! project, whose κ-label and canonical JSON form are held to uor-addr, the
//! crate that defines them, as an oracle written independently of the
//! package. An oracle is evidence, not authority: a disagreement fails the
//! case and is a defect of the package, of the pipeline, or of the oracle, and
//! nothing here adopts the oracle's output.
//!
//! The package writes the JSON realization, all five hash axes, the five
//! categorical compositions on each axis (`KA-02`..`KA-06`), and the GGUF v3
//! and ONNX realizations (`KA-07`, `KA-08`), so every committed fixture of
//! uor-addr is labelled by the package.

use std::collections::BTreeMap;

use lexlean::calculus::rust::package::Passing;
use lexlean::calculus::rust::{self, Profile};
use lexlean::calculus::{self as target, interp, Value};
use lexlean::production::lower::Lowered;
use serde_json::Value as Json;

use super::stdlib::{lower_in, Run};
use crate::rust_build::{cargo_in, workspace, Built};
use crate::rust_harness::{self, Calls};
use crate::rust_packages;
use crate::support::{self, repo_root, P};

/// The members of the workspace the project needs.
const MEMBERS: [&str; 2] = ["kappa", "stdlib"];
/// The three modules of the package.
const LABEL: &str = "LexLeanKappa.Label";
const JSON: &str = "LexLeanKappa.Json";
const COMPOSE: &str = "LexLeanKappa.Compose";
const AXIS: &str = "LexLeanKappa.Axis";
const GGUF: &str = "LexLeanKappa.Gguf";
const ONNX: &str = "LexLeanKappa.Onnx";

/// The roots in the order their modules declare them, each with the effects
/// its declaration admits, which are exactly the effects its closure
/// realizes.
const ROOTS: [(&str, &str, &[&str]); 24] = [
    (LABEL, "kappaLabel", &["allocation", "overflow"]),
    (LABEL, "isLabel", &["allocation", "overflow"]),
    (LABEL, "handleWire", &["allocation", "overflow"]),
    (JSON, "canonicalJson", &["allocation", "overflow"]),
    (JSON, "jsonLabel", &["allocation", "overflow"]),
    (AXIS, "labelOn", &["allocation", "overflow"]),
    (AXIS, "isLabelOn", &["allocation", "overflow"]),
    (AXIS, "jsonLabelOn", &["allocation", "overflow"]),
    (COMPOSE, "composeG2On", &["allocation", "overflow"]),
    (COMPOSE, "composeF4On", &["allocation", "overflow"]),
    (COMPOSE, "composeE6On", &["allocation", "overflow"]),
    (COMPOSE, "composeE7On", &["allocation", "overflow"]),
    (COMPOSE, "composeE8On", &["allocation", "overflow"]),
    (COMPOSE, "composeG2", &["allocation", "overflow"]),
    (COMPOSE, "composeF4", &["allocation", "overflow"]),
    (COMPOSE, "composeE6", &["allocation", "overflow"]),
    (COMPOSE, "composeE7", &["allocation", "overflow"]),
    (COMPOSE, "composeE8", &["allocation", "overflow"]),
    (GGUF, "canonicalGguf", &["allocation", "overflow"]),
    (GGUF, "ggufLabel", &["allocation", "overflow"]),
    (GGUF, "ggufLabelOn", &["allocation", "overflow"]),
    (ONNX, "canonicalOnnx", &["allocation", "overflow"]),
    (ONNX, "onnxLabel", &["allocation", "overflow"]),
    (ONNX, "onnxLabelOn", &["allocation", "overflow"]),
];

/// The σ-axes the package writes, with the width of their raw digests in
/// bytes: the five of uor-addr.
const AXES: [(&str, usize); 5] = [
    ("sha256", 32),
    ("blake3", 32),
    ("sha3-256", 32),
    ("keccak256", 32),
    ("sha512", 64),
];

/// Seeded documents under the evaluator.
const EVALUATOR_RUNS: usize = 120;
/// Seeded documents under rustc.
const RUSTC_RUNS: usize = 600;
/// Seeded operands of each composition, beside the fixed ones.
const COMPOSE_RUNS: usize = 40;
/// The evaluator's fuel; a root that exhausts it fails the case.
const FUEL: u64 = 400_000_000;

/// The twelve JSON fixtures of uor-addr's byte-identity test, harvested from
/// its reference endpoint: the raw input and the committed address.
const FIXTURES: [(&str, &str, &str); 12] = [
    (
        "simple_object",
        r#"{"foo": "bar"}"#,
        "sha256:7a38bf81f383f69433ad6e900d35b3e2385593f76a7b7ab5d4355b8ba41ee24b",
    ),
    (
        "empty_object",
        "{}",
        "sha256:44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a",
    ),
    (
        "empty_array",
        "[]",
        "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945",
    ),
    (
        "key_sort_test",
        r#"{"b": 1, "a": 2}"#,
        "sha256:d3626ac30a87e6f7a6428233b3c68299976865fa5508e4267c5415c76af7a772",
    ),
    (
        "unicode_muller",
        "{\"name\": \"M\u{fc}ller\"}",
        "sha256:5e92260d138e28f7118a40c3c44be922d4569a39f9f1de676ca334cf19c3a37c",
    ),
    (
        "unicode_sao_paulo",
        "{\"city\": \"S\u{e3}o Paulo\"}",
        "sha256:0906524dcbec3bdb6aa4d7c22ed65d671ba32177b10823b6f256546280aa526b",
    ),
    (
        "unicode_cafe_composed",
        "{\"name\": \"caf\u{e9}\"}",
        "sha256:645fa443126a8954fc6d871912b8fc67bc2ee8feae417efe55546251962ca74d",
    ),
    (
        "unicode_cafe_decomposed",
        "{\"name\": \"cafe\u{301}\"}",
        "sha256:645fa443126a8954fc6d871912b8fc67bc2ee8feae417efe55546251962ca74d",
    ),
    (
        "mixed_types",
        r#"{"int": 42, "bool": true, "null_val": null}"#,
        "sha256:0966918f3851b97071b0e04d2576a2a86a197e71072b07061c8bfdaa6b6a5d2c",
    ),
    (
        "nested",
        r#"{"nested": {"deep": {"value": "found"}}}"#,
        "sha256:b18dce7d3cbf2ac3b908ad75c8420c4a42077192df69dffdbf786755724eff1d",
    ),
    (
        "string_array",
        r#"["a", "b", "c"]"#,
        "sha256:fa1844c2988ad15ab7b49e0ece09684500fad94df916859fb9a43ff85f5bb477",
    ),
    (
        "number_array",
        "[1, 2, 3]",
        "sha256:a615eeaee21de5179de080de8c3052c8da901138406ba71c38c032845f7d54f4",
    ),
];

/// The five committed `.kappa-label` files of uor-addr's GGUF and ONNX
/// fixtures (v0.2.0, `tests/fixtures/`), vendored under `tests/kappa/`, read
/// for label form by `KA-02` and the fixtures of `KA-07` and `KA-08`.
const COMMITTED_LABELS: [&str; 5] = [
    "gguf/aligned-256.kappa-label",
    "gguf/empty-metadata.kappa-label",
    "gguf/synthetic-f32.kappa-label",
    "onnx/synthetic-typed.kappa-label",
    "onnx/synthetic.kappa-label",
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
    fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.below(items.len())]
    }
    fn chance(&mut self, percent: usize) -> bool {
        self.below(100) < percent
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
fn bytes(value: &[u8]) -> Value {
    Value::Bytes { hex: hex(value) }
}
fn option_bytes(value: Option<&[u8]>) -> Value {
    value.map_or(Value::None, |value| Value::Some {
        value: Box::new(bytes(value)),
    })
}

/// The expected observable of a run.
fn record(out: &mut Vec<Run>, arguments: Vec<Value>, expected: &Value) {
    out.push((
        arguments,
        serde_json::to_value(expected).expect("a value is JSON"),
    ));
}

/// What the oracle says of a document: its canonical form, or `None` for a
/// refusal.
fn oracle_canonical(raw: &[u8]) -> Option<Vec<u8>> {
    uor_addr::json::canonicalize(raw).ok()
}

/// What the oracle says of a document: its κ-label, or `None` for a refusal.
fn oracle_label(raw: &[u8]) -> Option<String> {
    uor_addr::json::address(raw)
        .ok()
        .map(|outcome| outcome.address.as_str().to_owned())
}

/// What the oracle says of a document on a σ-axis: its κ-label there, or
/// `None` for a refusal.
fn oracle_address(axis: &str, raw: &[u8]) -> Option<String> {
    use uor_addr::json as j;
    match axis {
        "sha256" => j::address(raw)
            .ok()
            .map(|outcome| outcome.address.as_str().to_owned()),
        "blake3" => j::address_blake3(raw)
            .ok()
            .map(|outcome| outcome.address.as_str().to_owned()),
        "sha3-256" => j::address_sha3_256(raw)
            .ok()
            .map(|outcome| outcome.address.as_str().to_owned()),
        "keccak256" => j::address_keccak256(raw)
            .ok()
            .map(|outcome| outcome.address.as_str().to_owned()),
        "sha512" => j::address_sha512(raw)
            .ok()
            .map(|outcome| outcome.address.as_str().to_owned()),
        other => panic!("`{other}` is not an axis the oracle is held to"),
    }
}

/// A generated document: its text, and whether it uses something the
/// package refuses by design (a number that is not an integer, a string
/// outside the subset whose NFC form it computes, or a repeated member name),
/// so that the oracle accepting it is not a disagreement.
struct Document {
    text: String,
    outside: bool,
}

/// Pieces of a string the package computes: ASCII, the escapes, the
/// characters that need escaping, Latin-1 in composed and decomposed form.
const INSIDE: &[&str] = &[
    "a",
    "Z",
    "0",
    " ",
    "-",
    "/",
    "\u{7f}",
    "\\\"",
    "\\\\",
    "\\/",
    "\\b",
    "\\f",
    "\\n",
    "\\r",
    "\\t",
    "\\u0001",
    "\\u001f",
    "\\u0041",
    "\\u00e9",
    "\\u00E9",
    "\\u0065\\u0301",
    "\u{e9}",
    "\u{c5}",
    "\u{ff}",
    "\u{a0}",
    "\u{df}",
    "e\u{301}",
    "A\u{30a}",
    "n\u{303}",
    "u\u{308}",
    "c\u{327}",
    "O\u{302}",
    "y\u{301}",
    "a\u{300}",
];
/// Pieces the package refuses: outside Latin-1, a combining mark with no
/// letter it composes with, and two marks.
const OUTSIDE: &[&str] = &[
    "\u{65e5}",
    "\u{1f600}",
    "\\u65e5",
    "\\ud83d\\ude00",
    "z\u{301}",
    "\u{e9}\u{301}",
    "e\u{304}",
    "\u{301}",
    "\u{153}",
];

impl Seeded {
    fn string(&mut self, outside: &mut bool) -> String {
        let mut text = String::new();
        for _ in 0..self.below(6) {
            if self.chance(4) {
                *outside = true;
                text.push_str(self.pick(OUTSIDE));
            } else {
                text.push_str(self.pick(INSIDE));
            }
        }
        text
    }

    fn gap(&mut self) -> &'static str {
        self.pick(&["", "", "", " ", "\n", "\t", "\r\n  "])
    }

    fn number(&mut self, outside: &mut bool) -> String {
        if self.chance(3) {
            *outside = true;
            return (*self.pick(&["1.5", "-0", "1e3", "2E-2", "0.0", "-0.5", "10.25e1"]))
                .to_owned();
        }
        let digits = [
            "0",
            "1",
            "7",
            "42",
            "100",
            "-1",
            "-17",
            "9007199254740993",
            "18446744073709551616",
            "-9223372036854775808",
        ];
        (*self.pick(&digits)).to_owned()
    }

    fn value(&mut self, depth: usize, outside: &mut bool) -> String {
        let kind = if depth == 0 {
            self.below(5)
        } else {
            self.below(7)
        };
        match kind {
            0 => "null".to_owned(),
            1 => (*self.pick(&["true", "false"])).to_owned(),
            2 | 3 => self.number(outside),
            4 => format!("\"{}\"", self.string(outside)),
            5 => {
                let count = self.below(4);
                let items: Vec<String> = (0..count)
                    .map(|_| {
                        format!(
                            "{}{}{}",
                            self.gap(),
                            self.value(depth - 1, outside),
                            self.gap()
                        )
                    })
                    .collect();
                format!("[{}{}]", self.gap(), items.join(","))
            }
            _ => {
                let count = self.below(5);
                let mut used: Vec<String> = Vec::new();
                let mut members = Vec::new();
                while members.len() < count {
                    let key = self.string(outside);
                    // Two raw keys that are one key once escapes and NFC are
                    // applied are a repeated name, which the package refuses.
                    let canon = oracle_canonical(format!("[\"{key}\"]").as_bytes());
                    let normal = canon.map_or_else(
                        || key.clone(),
                        |bytes| String::from_utf8_lossy(&bytes).into_owned(),
                    );
                    if used.contains(&normal) {
                        *outside = true;
                    }
                    used.push(normal);
                    members.push(format!(
                        "{}\"{key}\"{}:{}{}",
                        self.gap(),
                        self.gap(),
                        self.gap(),
                        self.value(depth - 1, outside)
                    ));
                }
                format!("{{{}{}}}", members.join(","), self.gap())
            }
        }
    }

    fn document(&mut self) -> Document {
        let mut outside = false;
        // The top level is an object or an array, so that no proper prefix
        // of a document is a document.
        let inner = self.value(3, &mut outside);
        let text = if inner.starts_with('{') || inner.starts_with('[') {
            inner
        } else {
            format!("[{inner}]")
        };
        Document {
            text: format!("{}{}{}", self.gap(), text, self.gap()),
            outside,
        }
    }
}

/// The width in bytes of the raw digest of an axis.
fn digest_bytes(axis: &str) -> usize {
    AXES.iter()
        .find(|(name, _)| *name == axis)
        .map(|(_, width)| *width)
        .unwrap_or_else(|| panic!("`{axis}` is not an axis of the package"))
}

/// The corpus of canonical JSON runs: the fixtures, edge cases, malformed
/// documents, and seeded documents up to `runs`.
fn json_corpus(runs: usize, seed: u64, label: Option<&str>) -> Vec<Run> {
    let mut seeded = Seeded(seed);
    let mut out = Vec::new();
    let expect = |out: &mut Vec<Run>, raw: &[u8], outside: bool| {
        let canonical = oracle_canonical(raw).filter(|_| !outside);
        let expected = match label {
            Some(axis) => canonical.map(|_| {
                let label = oracle_address(axis, raw).expect("a label").into_bytes();
                assert_eq!(
                    label.len(),
                    axis.len() + 1 + 2 * digest_bytes(axis),
                    "a label is its axis, a colon, and the digits"
                );
                label
            }),
            None => canonical,
        };
        record(out, vec![bytes(raw)], &option_bytes(expected.as_deref()));
    };
    for (_, raw, _) in FIXTURES {
        expect(&mut out, raw.as_bytes(), false);
    }
    for raw in [
        "{\"a\":1,\"a\":2}",
        "[1.5]",
        "[-0]",
        "[1e3]",
        "[\"\\u65e5\"]",
        "",
        " ",
        "[",
        "{",
        "[1,]",
        "{\"a\":}",
        "{\"a\" 1}",
        "{,}",
        "[1] x",
        "[01]",
        "[\"\u{1}\"]",
        "[\"\\x\"]",
        "[\"\\u12\"]",
        "[tru]",
        "[nul]",
        "[-]",
        "{\"a\":1 \"b\":2}",
        "[1 2]",
        "\"x\"",
        "7",
        "null",
    ] {
        // The oracle and the package agree on a scalar at the top level and
        // on every malformed text; a repeated name and a number or string
        // outside the subset the oracle accepts are refused by design.
        let outside = matches!(
            raw,
            "{\"a\":1,\"a\":2}" | "[1.5]" | "[-0]" | "[1e3]" | "[\"\\u65e5\"]"
        );
        expect(&mut out, raw.as_bytes(), outside);
    }
    while out.len() < runs {
        let document = seeded.document();
        let raw = document.text.as_bytes();
        expect(&mut out, raw, document.outside);
        // Its proper prefixes and extensions are not documents.
        if !document.outside && out.len() < runs {
            let trimmed = document.text.trim_end();
            let cut = (1..trimmed.len())
                .filter(|at| trimmed.is_char_boundary(*at))
                .nth(seeded.below(trimmed.len().max(2) - 1))
                .unwrap_or(1);
            let prefix = &trimmed.as_bytes()[..cut];
            assert!(
                oracle_canonical(prefix).is_none(),
                "the oracle refuses a proper prefix of a document: {:?}",
                String::from_utf8_lossy(prefix)
            );
            expect(&mut out, prefix, false);
            let extended = format!("{trimmed},");
            expect(&mut out, extended.as_bytes(), false);
        }
    }
    out.truncate(runs);
    out
}

/// The runs of `kappaLabel`: the canonical bytes of every fixture and some
/// bytes of every length around the block size.
fn label_corpus() -> Vec<Run> {
    let mut out = Vec::new();
    let mut seeded = Seeded(0x6b61_7070_6100_0001);
    let mut subjects: Vec<Vec<u8>> = FIXTURES
        .iter()
        .map(|(_, raw, _)| oracle_canonical(raw.as_bytes()).expect("a fixture"))
        .collect();
    subjects.push(Vec::new());
    for length in [1, 55, 56, 63, 64, 65, 119, 120, 128, 1000] {
        subjects.push((0..length).map(|_| seeded.below(256) as u8).collect());
    }
    for subject in subjects {
        let label = format!("sha256:{}", {
            use sha2::{Digest, Sha256};
            hex(&Sha256::digest(&subject))
        });
        record(&mut out, vec![bytes(&subject)], &bytes(label.as_bytes()));
    }
    out
}

/// The runs of `isLabel`: the twelve fixture addresses, the six committed
/// label files, and strings that are one byte from a label.
fn is_label_corpus() -> Vec<Run> {
    let mut out = Vec::new();
    let boolean = |value: bool| Value::Bool { value };
    let mut accepted: Vec<String> = FIXTURES
        .iter()
        .map(|(_, _, label)| (*label).to_owned())
        .collect();
    let root = repo_root().join("tests/kappa");
    for file in COMMITTED_LABELS {
        let text = std::fs::read_to_string(root.join(file).as_std_path())
            .unwrap_or_else(|error| panic!("{file}: {error}"));
        accepted.push(text.trim_end().to_owned());
    }
    for label in &accepted {
        record(&mut out, vec![bytes(label.as_bytes())], &boolean(true));
        let mut upper = label.clone().into_bytes();
        upper[8] = upper[8].to_ascii_uppercase();
        let flipped = upper != label.as_bytes();
        let rejected = [
            label[..label.len() - 1].to_owned(),
            format!("{label}0"),
            label.replacen("sha256:", "sha512:", 1),
            label.replacen("sha256:", "sha256", 1),
            label.replacen("sha256:", "sha256;", 1),
            label.replacen(&label[7..8], "g", 1),
        ];
        for text in rejected {
            record(&mut out, vec![bytes(text.as_bytes())], &boolean(false));
        }
        if flipped {
            record(&mut out, vec![bytes(&upper)], &boolean(false));
        }
    }
    record(&mut out, vec![bytes(b"")], &boolean(false));
    out
}

/// The runs of `handleWire`: valid and invalid kinds against a canonical
/// form.
fn handle_corpus() -> Vec<Run> {
    let mut out = Vec::new();
    let canonical = oracle_canonical(br#"{"foo":"bar"}"#).expect("a fixture");
    let label = oracle_label(br#"{"foo":"bar"}"#).expect("a label");
    for kind in ["json", "onnx-model", "gguf", "int8-tensor", "a0", "0", "-"] {
        let wire = format!("{kind}@{label}");
        record(
            &mut out,
            vec![bytes(kind.as_bytes()), bytes(&canonical)],
            &option_bytes(Some(wire.as_bytes())),
        );
    }
    for kind in [
        "", "Json", "json ", "json@x", "on_nx", "\u{e9}", "json\n", "a:b",
    ] {
        record(
            &mut out,
            vec![bytes(kind.as_bytes()), bytes(&canonical)],
            &Value::None,
        );
    }
    out
}

/// The oracle's composition of the operands on an axis: the label it mints,
/// or `None` for any error (an operand of another axis, a malformed one, or
/// one of the wrong width). `member` is the composition's name without the
/// axis suffix: `composeG2`, `composeF4`, `composeE6`, `composeE7`, `composeE8`.
fn oracle_compose(axis: &str, member: &str, operands: &[Vec<u8>]) -> Option<String> {
    use uor_addr::composition as c;
    use uor_addr::KappaLabel;
    macro_rules! on {
        ($width:literal, $g2:path, $f4:path, $e6:path, $e7:path, $e8:path) => {{
            let operand = |index: usize| KappaLabel::<$width>::from_bytes(&operands[index]).ok();
            let outcome = match member {
                "composeG2" => $g2(&operand(0)?, &operand(1)?),
                "composeF4" => $f4(&operand(0)?),
                "composeE6" => $e6(&operand(0)?),
                "composeE7" => $e7(&operand(0)?),
                "composeE8" => $e8(&operand(0)?),
                other => panic!("`{other}` is not a composition"),
            };
            outcome
                .ok()
                .map(|outcome| outcome.address.as_str().to_owned())
        }};
    }
    match axis {
        "sha256" => on!(
            71,
            c::compose_g2_product,
            c::compose_f4_quotient,
            c::compose_e6_filtration,
            c::compose_e7_augmentation,
            c::compose_e8_embedding
        ),
        "blake3" => on!(
            71,
            c::compose_g2_product_blake3,
            c::compose_f4_quotient_blake3,
            c::compose_e6_filtration_blake3,
            c::compose_e7_augmentation_blake3,
            c::compose_e8_embedding_blake3
        ),
        "sha3-256" => on!(
            73,
            c::compose_g2_product_sha3_256,
            c::compose_f4_quotient_sha3_256,
            c::compose_e6_filtration_sha3_256,
            c::compose_e7_augmentation_sha3_256,
            c::compose_e8_embedding_sha3_256
        ),
        "keccak256" => on!(
            74,
            c::compose_g2_product_keccak256,
            c::compose_f4_quotient_keccak256,
            c::compose_e6_filtration_keccak256,
            c::compose_e7_augmentation_keccak256,
            c::compose_e8_embedding_keccak256
        ),
        "sha512" => on!(
            135,
            c::compose_g2_product_sha512,
            c::compose_f4_quotient_sha512,
            c::compose_e6_filtration_sha512,
            c::compose_e7_augmentation_sha512,
            c::compose_e8_embedding_sha512
        ),
        other => panic!("`{other}` is not an axis the oracle is held to"),
    }
}

/// Operand labels of an axis: the extremes, digests whose first byte modulo
/// nine is eight (the degree-6 class of E6), a digest and its mirror, digests
/// whose quarters are in every order, and seeded digests.
fn valid_operands(axis: &str, seeded: &mut Seeded, random: usize) -> Vec<Vec<u8>> {
    let width = digest_bytes(axis);
    let mut digests: Vec<Vec<u8>> = vec![
        vec![0; width],
        vec![0xff; width],
        (1..=width).map(|byte| byte as u8).collect(),
    ];
    for first in [0x08, 0x11, 0x1a, 0xf8, 0xfb, 0x00, 0x07, 0x09] {
        let mut digest: Vec<u8> = (0..width).map(|_| seeded.below(256) as u8).collect();
        digest[0] = first;
        digests.push(digest);
    }
    // Quarters in every one of the 24 orders of four distinct blocks.
    let quarter = width / 4;
    let quarters: Vec<Vec<u8>> = (0..4u8)
        .map(|index| vec![0x10 * (index + 1) + 3; quarter])
        .collect();
    for a in 0..4usize {
        for b in (0..4usize).filter(|b| *b != a) {
            for c in (0..4usize).filter(|c| *c != a && *c != b) {
                let d = 6 - a - b - c;
                digests.push(
                    [a, b, c, d]
                        .iter()
                        .flat_map(|index| quarters[*index].clone())
                        .collect(),
                );
            }
        }
    }
    // Equal leading quarters, so that the order is decided late.
    digests.push(
        [
            vec![7u8; quarter],
            vec![7; quarter],
            vec![3; quarter],
            vec![9; quarter],
        ]
        .concat(),
    );
    for _ in 0..random {
        digests.push((0..width).map(|_| seeded.below(256) as u8).collect());
    }
    let mirrors: Vec<Vec<u8>> = digests
        .iter()
        .take(12)
        .map(|digest| digest.iter().map(|byte| !byte).collect())
        .collect();
    digests.extend(mirrors);
    digests
        .iter()
        .map(|digest| format!("{axis}:{}", hex(digest)).into_bytes())
        .collect()
}

/// Operands the package refuses on an axis: another axis, the case of the
/// prefix, a digest of the wrong width, upper case, and characters that are
/// not digits.
fn invalid_operands(axis: &str, valid: &[u8]) -> Vec<Vec<u8>> {
    let text = String::from_utf8(valid.to_vec()).expect("a label");
    let digits = &text[axis.len() + 1..];
    let mut out: Vec<Vec<u8>> = vec![
        format!("{}:{digits}", axis.to_uppercase()).into_bytes(),
        format!("{axis}:{}", &digits[1..]).into_bytes(),
        format!("{axis}:{digits}0").into_bytes(),
        format!("{axis}:{}", digits.to_uppercase()).into_bytes(),
        format!("{axis}:g{}", &digits[1..]).into_bytes(),
        format!("{axis};{digits}").into_bytes(),
        format!("{axis}:{digits}\n").into_bytes(),
        format!("{axis}:{}", &digits[..digits.len() / 2]).into_bytes(),
        Vec::new(),
        format!("{axis}:").into_bytes(),
    ];
    // The same digits under each other axis.
    for (other, _) in AXES.iter().filter(|(other, _)| *other != axis) {
        out.push(format!("{other}:{digits}").into_bytes());
    }
    // A label of another axis whose width is this axis's own.
    for (other, _) in AXES.iter().filter(|(other, _)| *other != axis) {
        let fill = "0".repeat(2 * digest_bytes(axis) + axis.len() - other.len());
        out.push(format!("{other}:{fill}").into_bytes());
    }
    // A label with a non-ASCII byte in a digit position.
    let mut high = valid.to_vec();
    high[axis.len() + 4] = 0xe9;
    out.push(high);
    out
}

/// The runs of one composition on an axis: valid operands (the pairs in both
/// orders for the product) and the refused ones, with the oracle's label.
/// `named` puts the axis first among the arguments, as the roots over an axis
/// take it.
fn compose_corpus(member: &str, axis: &str, named: bool, runs: usize) -> Vec<Run> {
    let mut seeded = Seeded(0x6b61_0004_0000_0001);
    let valid = valid_operands(axis, &mut seeded, runs);
    let mut out = Vec::new();
    let mut run = |operands: Vec<Vec<u8>>| {
        let expected = option_bytes(
            oracle_compose(axis, member, &operands)
                .as_deref()
                .map(str::as_bytes),
        );
        let mut arguments: Vec<Value> = Vec::new();
        if named {
            arguments.push(bytes(axis.as_bytes()));
        }
        arguments.extend(operands.iter().map(|operand| bytes(operand)));
        record(&mut out, arguments, &expected);
    };
    if member == "composeG2" {
        for (index, left) in valid.iter().enumerate() {
            let right = &valid[(index * 7 + 3) % valid.len()];
            run(vec![left.clone(), right.clone()]);
            run(vec![right.clone(), left.clone()]);
            run(vec![left.clone(), left.clone()]);
        }
        for bad in invalid_operands(axis, &valid[3]) {
            run(vec![bad.clone(), valid[3].clone()]);
            run(vec![valid[3].clone(), bad]);
        }
    } else {
        for operand in &valid {
            run(vec![operand.clone()]);
        }
        for bad in invalid_operands(axis, &valid[3]) {
            run(vec![bad]);
        }
    }
    out
}

/// The runs of a composition over an axis, for each axis of the package: the
/// axis named first, then the operands. A name that is no axis of the package
/// has no composition, so the oracle is not asked: the root gives no output.
fn compose_on_corpus(member: &str, runs: usize) -> Vec<Run> {
    let base = member
        .strip_suffix("On")
        .expect("a composition over an axis");
    let mut out = Vec::new();
    for (axis, _) in AXES {
        out.extend(compose_corpus(base, axis, true, runs));
    }
    let valid = valid_operands("sha512", &mut Seeded(0x6b61_0006_0000_0001), 0);
    let operands = if base == "composeG2" { 2 } else { 1 };
    for name in ["", "md5", "sha384", "SHA512", "sha512:", "blake2b"] {
        let mut arguments = vec![bytes(name.as_bytes())];
        arguments.extend((0..operands).map(|_| bytes(&valid[3])));
        record(&mut out, arguments, &Value::None);
    }
    out
}

/// The canonical bytes of the JSON documents the hash runs hash: the
/// fixtures, documents whose canonical form has a length around the block of
/// SHA-512 and the rate of Keccak, and seeded documents. Each is paired with
/// the raw text the oracle addresses.
fn hash_subjects(runs: usize) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut seeded = Seeded(0x6b61_0005_0000_0001);
    let mut out: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
    for (_, raw, _) in FIXTURES {
        out.push((
            oracle_canonical(raw.as_bytes()).expect("a fixture"),
            raw.as_bytes().to_vec(),
        ));
    }
    // A string in an array: four bytes of structure and as many letters as
    // bring the canonical form to the length asked.
    for length in [
        4, 5, 55, 56, 63, 64, 65, 110, 111, 112, 113, 127, 128, 129, 134, 135, 136, 137, 239, 240,
        241, 255, 256, 257, 271, 272, 273, 1000, 1023, 1024, 1025, 2047, 2048, 2049, 3071, 3072,
        3073, 4096, 5000,
    ] {
        let letters: String = (0..length - 4)
            .map(|_| char::from(b'a' + u8::try_from(seeded.below(26)).expect("a letter")))
            .collect();
        let raw = format!("[\"{letters}\"]").into_bytes();
        let canonical = oracle_canonical(&raw).expect("a document");
        assert_eq!(
            canonical.len(),
            length,
            "the canonical form has the length asked"
        );
        out.push((canonical, raw));
    }
    while out.len() < runs {
        let document = seeded.document();
        if document.outside {
            continue;
        }
        let raw = document.text.into_bytes();
        let canonical = oracle_canonical(&raw).expect("a document");
        out.push((canonical, raw));
    }
    out
}

/// The runs of `labelOn`: the canonical forms of `hash_subjects` on each axis
/// of the package, and the names that are no axis of the package.
fn label_on_corpus(runs: usize) -> Vec<Run> {
    let mut out = Vec::new();
    for (axis, _) in AXES {
        for (canonical, raw) in hash_subjects(runs) {
            let label = oracle_address(axis, &raw).expect("the oracle labels a document");
            record(
                &mut out,
                vec![bytes(axis.as_bytes()), bytes(&canonical)],
                &option_bytes(Some(label.as_bytes())),
            );
        }
    }
    for name in ["", "md5", "sha384", "SHA512", "sha512:", "blake2b"] {
        record(
            &mut out,
            vec![bytes(name.as_bytes()), bytes(b"[]")],
            &Value::None,
        );
    }
    out
}

/// The runs of `isLabelOn`: the oracle's addresses of the fixtures on each
/// axis, and strings that are one byte from a label or a label of another
/// axis.
fn is_label_on_corpus() -> Vec<Run> {
    let mut out = Vec::new();
    let boolean = |value: bool| Value::Bool { value };
    for (axis, _) in AXES {
        for (_, raw, _) in FIXTURES {
            let label = oracle_address(axis, raw.as_bytes()).expect("a label");
            let mut ask = |candidate: &[u8], expected: bool| {
                record(
                    &mut out,
                    vec![bytes(axis.as_bytes()), bytes(candidate)],
                    &boolean(expected),
                );
            };
            ask(label.as_bytes(), true);
            let digits = &label[axis.len() + 1..];
            ask(&label.as_bytes()[..label.len() - 1], false);
            ask(format!("{label}0").as_bytes(), false);
            ask(format!("{axis};{digits}").as_bytes(), false);
            ask(format!("{axis}{digits}").as_bytes(), false);
            ask(format!("{axis}:g{}", &digits[1..]).as_bytes(), false);
            ask(
                format!("{}:{digits}", axis.to_uppercase()).as_bytes(),
                false,
            );
            ask(
                format!("{axis}:{}", digits.to_uppercase()).as_bytes(),
                digits.to_uppercase() == digits,
            );
            for (other, _) in AXES.iter().filter(|(other, _)| *other != axis) {
                ask(format!("{other}:{digits}").as_bytes(), false);
                let foreign = oracle_address(other, raw.as_bytes()).expect("a label");
                ask(foreign.as_bytes(), false);
            }
        }
    }
    for name in ["", "md5", "blake2b"] {
        let label = oracle_address("sha256", b"[]").expect("a label");
        record(
            &mut out,
            vec![bytes(name.as_bytes()), bytes(label.as_bytes())],
            &boolean(false),
        );
    }
    out
}

/// The runs of `jsonLabelOn`: documents on each axis of the package.
fn json_label_on_corpus(runs: usize) -> Vec<Run> {
    let mut out = Vec::new();
    for (axis, _) in AXES {
        for (arguments, expected) in json_corpus(runs, 0x4b41_0005, Some(axis)) {
            let mut named = vec![bytes(axis.as_bytes())];
            named.extend(arguments);
            out.push((named, expected));
        }
    }
    out
}

/// The corpus of one root, `runs` documents long where it is seeded.
fn corpus(member: &str, runs: usize) -> Vec<Run> {
    match member {
        "kappaLabel" => label_corpus(),
        "isLabel" => is_label_corpus(),
        "handleWire" => handle_corpus(),
        "canonicalJson" => json_corpus(runs, 0x4b41_0003, None),
        "jsonLabel" => json_corpus(runs, 0x4b41_0002, Some("sha256")),
        "composeG2" | "composeF4" | "composeE6" | "composeE7" | "composeE8" => {
            compose_corpus(member, "sha256", false, runs)
        }
        "labelOn" => label_on_corpus(runs),
        "isLabelOn" => is_label_on_corpus(),
        "jsonLabelOn" => json_label_on_corpus(runs),
        "composeG2On" | "composeF4On" | "composeE6On" | "composeE7On" | "composeE8On" => {
            compose_on_corpus(member, runs)
        }
        "canonicalGguf" | "ggufLabel" | "ggufLabelOn" => gguf_corpus(member, runs),
        "canonicalOnnx" | "onnxLabel" | "onnxLabelOn" => onnx_corpus(member, runs),
        other => panic!("`{other}` is not a root of the package"),
    }
}

/// Every run on which the evaluator and the oracle disagree.
fn evaluate(lowered: &Lowered, corpus: &[Run]) -> Vec<String> {
    corpus
        .iter()
        .enumerate()
        .filter_map(|(run, (arguments, expected))| {
            let outcome = interp::run(&lowered.program, FUEL, lowered.entry(), arguments);
            let observed = rust::observable(&outcome)
                .unwrap_or_else(|| panic!("run {run} on {arguments:?}: {outcome:?}"));
            (&observed != expected)
                .then(|| format!("run {run} on {arguments:?}: {observed} != {expected}"))
        })
        .collect()
}

fn lowered_roots(project: &P, members: &[&str]) -> BTreeMap<&'static str, Lowered> {
    let checked = support::checked_project(project);
    ROOTS
        .iter()
        .filter(|(_, member, _)| members.contains(member))
        .map(|(module, member, _)| (*member, lower_in(&checked, project, module, member)))
        .collect()
}

fn snake(member: &str) -> String {
    let mut out = String::new();
    for character in member.chars() {
        if character.is_ascii_uppercase() {
            out.push('_');
            out.push(character.to_ascii_lowercase());
        } else {
            out.push(character);
        }
    }
    out
}

/// The rendered `rust-std` package of each root named, compiled by the
/// pinned rustc and run on the root's corpus; every run is compared with the
/// oracle's outcome.
fn rustc_differential(lowered: &BTreeMap<&'static str, Lowered>, members: &[&str]) {
    let dir = tempfile::Builder::new()
        .prefix("lexlean-kappa-")
        .tempdir()
        .expect("tempdir");
    let mut crates = Vec::new();
    let mut expected: BTreeMap<String, Vec<Json>> = BTreeMap::new();
    for member in members {
        let low = &lowered[member];
        let manifest = rust_packages::manifest(
            &format!("kappa_{}", snake(member)),
            &low.program,
            low.entry(),
            Profile::Std,
        );
        let package = target::package(&manifest.to_file_bytes().expect("canonical manifest"))
            .unwrap_or_else(|failure| panic!("{member}: {failure:?}"));
        let runs = corpus(member, rustc_runs(member));
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
            "{name}: the rendering and uor-addr disagree on {} of {} runs: {:?}",
            disagreements.len(),
            outcomes.len(),
            &disagreements[..disagreements.len().min(5)]
        );
    }
}

/// A copy of the project with one edit to one module.
fn planted(module: &str, from: &str, to: &str) -> P {
    let project = P::workspace_project("kappa", &MEMBERS);
    // `Blake3` is the standard library's module, the one the compiler's
    // identities are computed by, so a plant in it is a plant in the source
    // package, and the copy is relocked over the planted package.
    let in_stdlib = module == "Blake3";
    let path = if in_stdlib {
        format!("../stdlib/src/{module}.lex.tex")
    } else {
        format!("src/{module}.lex.tex")
    };
    let text = project.read_raw(&path);
    assert!(
        text.contains(from),
        "the plant site `{from}` exists in {module}"
    );
    project.write(&path, &text.replacen(from, to, 1));
    if in_stdlib {
        let (exit, _, stderr) = project.cli(&["lock"]);
        assert_eq!(exit, 0, "the planted copy relocks: {stderr}");
    }
    project
}

/// The pinned `compiler_semantics` of a committed lock.
fn pinned(lock: &str) -> String {
    let text = std::fs::read_to_string(repo_root().join(lock).as_std_path())
        .unwrap_or_else(|error| panic!("{lock}: {error}"));
    text.lines()
        .find_map(|line| {
            line.strip_prefix("compiler_semantics = \"")
                .and_then(|rest| rest.strip_suffix('"'))
        })
        .unwrap_or_else(|| panic!("{lock}: no compiler_semantics"))
        .to_owned()
}

/// Run the case for one `KA` conformance ID.
///
/// # Panics
///
/// Panics when the case's assertion fails.
pub fn run(id: &str) {
    match id {
        "KA-01" => package(id),
        "KA-02" => labels(id),
        "KA-03" => canonical_json(),
        "KA-04" => composition(),
        "KA-05" => hash_axes(),
        "KA-06" => axis_composition(),
        "KA-07" => gguf(),
        "KA-08" => onnx(),
        other => panic!("no kappa-addressing case is wired for {other}"),
    }
}

/// `KA-01`: the project, its roots, and the identities it does not move.
fn package(id: &str) {
    let project = P::workspace_project("kappa", &MEMBERS);
    let manifest = project.read_raw("lexlean.toml");
    assert!(
        manifest.contains("module_prefix = \"LexLeanKappa\""),
        "{id}: the module prefix"
    );
    assert_eq!(
        manifest.matches("[[source_package]]").count(),
        1,
        "{id}: one source package"
    );
    assert!(
        manifest.contains("package = \"std\"") && manifest.contains("path = \"stdlib\""),
        "{id}: the source package is the standard library"
    );
    for forbidden in ["[[artifact", "[[glossary"] {
        assert!(!manifest.contains(forbidden), "{id}: no {forbidden}");
    }
    let workspace = lexlean::source_package::Workspace::read(&repo_root())
        .expect("the workspace file reads")
        .expect("the repository has a workspace file");
    assert!(
        workspace.members.iter().any(|member| member == "kappa")
            && workspace.members.iter().any(|member| member == "stdlib"),
        "{id}: kappa is a member of the workspace beside stdlib"
    );
    project.check_ok();
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    for module in ["Label", "Json", "Axis", "Compose", "Gguf", "Onnx"] {
        let report: Json = serde_json::from_slice(
            &std::fs::read(
                project
                    .build_dir(&build_id)
                    .join(format!("production/LexLeanKappa/{module}.eligibility.json"))
                    .as_std_path(),
            )
            .expect("the eligibility report"),
        )
        .expect("the report is JSON");
        let reported = report["roots"].as_array().expect("roots");
        let wanted: Vec<_> = ROOTS
            .iter()
            .filter(|(owner, _, _)| owner.ends_with(module))
            .collect();
        assert_eq!(
            reported
                .iter()
                .map(|row| row["root"].as_str().expect("root").to_owned())
                .collect::<Vec<_>>(),
            wanted
                .iter()
                .map(|(owner, member, _)| format!("{owner}.{member}"))
                .collect::<Vec<_>>(),
            "{id}: the production roots of {module}, in order"
        );
        for (row, (_, member, effects)) in reported.iter().zip(wanted) {
            let declared: Vec<&str> = row["declared_effects"]
                .as_array()
                .expect("declared effects")
                .iter()
                .map(|effect| effect.as_str().expect("an effect"))
                .collect();
            assert_eq!(declared, *effects, "{id}: {member}: the declared effects");
            let targets = row["targets"].as_array().expect("targets");
            assert_eq!(targets.len(), 1, "{id}: {member}: one target");
            assert_eq!(targets[0]["target"], "rust-std", "{id}: {member}: target");
            assert_eq!(targets[0]["status"], "eligible", "{id}: {member}: eligible");
            let realized: Vec<&str> = targets[0]["effects"]
                .as_array()
                .expect("effects")
                .iter()
                .map(|effect| effect["effect"].as_str().expect("an effect"))
                .collect();
            assert_eq!(
                realized, *effects,
                "{id}: {member}: the declaration admits exactly the realized effects"
            );
        }
    }
    // The package lives outside `language/` and `schemas/`: no file of either
    // names it, and the identities of the three languages are the ones the
    // committed locks pin.
    for tree in ["language", "schemas"] {
        for entry in walkdir::WalkDir::new(repo_root().join(tree).as_std_path())
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
        {
            if let Ok(text) = std::fs::read_to_string(entry.path()) {
                assert!(
                    !text.contains("LexLeanKappa"),
                    "{id}: {} names the package",
                    entry.path().display()
                );
            }
        }
    }
    for (language, lock) in [
        ("1.0", "examples/nat-add-zero/lexlean.lock"),
        ("1.1", "examples/semantic-1.1/lexlean.lock"),
        ("1.2", "examples/language-1.2/lexlean.lock"),
    ] {
        if repo_root().join(lock).as_std_path().exists() {
            assert_eq!(
                lexlean::compiler_semantics_id_for(language).label(),
                pinned(lock),
                "{id}: the compiler-semantics ID of language {language} is the one {lock} pins"
            );
        }
    }
    assert_eq!(
        pinned("kappa/lexlean.lock"),
        pinned("stdlib/lexlean.lock"),
        "{id}: the package pins the identity of the standard library beside it"
    );
    support::verify_ok_backed(id, &project);
    // The gate is armed: a package that drops the declared source package
    // does not check, since its modules import the standard library.
    let broken = P::workspace_project("kappa", &MEMBERS);
    let dropped = manifest.replace(
        "[[source_package]]\npackage = \"std\"\nversion = \"1.0.0\"\nkind = \"path\"\npath = \"stdlib\"\n\n",
        "",
    );
    assert_ne!(dropped, manifest, "{id}: the plant site exists");
    broken.write_raw("lexlean.toml", &dropped);
    assert!(
        !broken.check_err_or_ok(),
        "{id}: a project without its source package does not check"
    );
    eprintln!("{id}: plant (source package dropped): the project does not check");
}

/// `KA-02`: the label, the handle, and the fixtures.
fn labels(id: &str) {
    let project = P::workspace_project("kappa", &MEMBERS);
    project.check_ok();
    let lowered = lowered_roots(
        &project,
        &["kappaLabel", "isLabel", "handleWire", "jsonLabel"],
    );
    // The twelve fixtures: the committed address, the oracle's, and the
    // package's, as the evaluator computes it.
    let json = &lowered["jsonLabel"];
    for (name, raw, committed) in FIXTURES {
        assert_eq!(
            oracle_label(raw.as_bytes()).as_deref(),
            Some(committed),
            "{id}: {name}: the oracle gives the committed address"
        );
        let outcome = interp::run(&json.program, FUEL, json.entry(), &[bytes(raw.as_bytes())]);
        let observed = rust::observable(&outcome).expect("an observable");
        assert_eq!(
            observed,
            serde_json::to_value(option_bytes(Some(committed.as_bytes()))).expect("json"),
            "{id}: {name}: the package gives the committed address"
        );
    }
    for member in ["kappaLabel", "isLabel", "handleWire", "jsonLabel"] {
        let runs = corpus(member, EVALUATOR_RUNS);
        let disagreements = evaluate(&lowered[member], &runs);
        assert!(
            disagreements.is_empty(),
            "{id}: {member}: the evaluator and uor-addr disagree on {} of {} runs: {:?}",
            disagreements.len(),
            runs.len(),
            &disagreements[..disagreements.len().min(5)]
        );
    }
    rustc_differential(
        &lowered,
        &["kappaLabel", "isLabel", "handleWire", "jsonLabel"],
    );
    // The gate is armed: a label with the wrong prefix, a label whose
    // alphabet admits uppercase, and a handle that forgets its kind are each
    // detected.
    for (module, from, to, member) in [
        (
            "Label",
            "(text \"sha256:\") (LexLeanStd.Sha256::sha256Hex canonical)",
            "(text \"sha512:\") (LexLeanStd.Sha256::sha256Hex canonical)",
            "kappaLabel",
        ),
        (
            "Label",
            "(if (96 <? n) then (n <? 103) else false)",
            "(if (64 <? n) then (n <? 103) else false)",
            "isLabel",
        ),
        (
            "Label",
            "(prim append (project handle kind) (prim append (text \"@\")",
            "(prim append (project handle label) (prim append (text \"@\")",
            "handleWire",
        ),
    ] {
        let defective = planted(module, from, to);
        let checked = support::checked_project(&defective);
        let low = lower_in(&checked, &defective, LABEL, member);
        let runs = corpus(member, EVALUATOR_RUNS);
        let disagreements = evaluate(&low, &runs);
        assert!(
            !disagreements.is_empty(),
            "{id}: a planted defect in `{member}` is detected by the differential"
        );
        eprintln!(
            "{id}: plant in {member}: {} of {} runs disagree, the first {}",
            disagreements.len(),
            runs.len(),
            disagreements[0]
        );
    }
}

/// `KA-03`: the canonical JSON form.
fn canonical_json() {
    let project = P::workspace_project("kappa", &MEMBERS);
    project.check_ok();
    let lowered = lowered_roots(&project, &["canonicalJson"]);
    let runs = corpus("canonicalJson", EVALUATOR_RUNS);
    let refusal = serde_json::to_value(Value::None).expect("a value is JSON");
    assert!(
        runs.iter().any(|(_, expected)| *expected == refusal)
            && runs.iter().any(|(_, expected)| *expected != refusal),
        "KA-03: the corpus holds both accepted and refused documents"
    );
    let refused = runs
        .iter()
        .filter(|(_, expected)| *expected == refusal)
        .count();
    eprintln!(
        "KA-03: {} evaluator runs, {refused} refused by uor-addr or by design, {} accepted",
        runs.len(),
        runs.len() - refused
    );
    let disagreements = evaluate(&lowered["canonicalJson"], &runs);
    assert!(
        disagreements.is_empty(),
        "KA-03: the evaluator and uor-addr disagree on {} of {} runs: {:?}",
        disagreements.len(),
        runs.len(),
        &disagreements[..disagreements.len().min(5)]
    );
    rustc_differential(&lowered, &["canonicalJson"]);
    // The gate is armed: members kept in input order, an escape that writes
    // a raw newline, and a composition that drops the mark are each detected.
    for (from, to) in [
        (
            "(if (lessKey key (first item))",
            "(if (lessKey (first item) key)",
        ),
        (
            "(if (b =? 10) then 0x5c6e",
            "(if (b =? 10) then 0x0a",
        ),
        (
            "(prim append (LexLeanStd.Text::sliceOr text 0 (prim subtract size 1 : Nat)) (encodeHigh replacement) : Bytes)",
            "(prim append (LexLeanStd.Text::sliceOr text 0 (prim subtract size 1 : Nat)) (byteOf base) : Bytes)",
        ),
    ] {
        let defective = planted("Json", from, to);
        let checked = support::checked_project(&defective);
        let low = lower_in(&checked, &defective, JSON, "canonicalJson");
        let runs = corpus("canonicalJson", EVALUATOR_RUNS);
        let disagreements = evaluate(&low, &runs);
        assert!(
            !disagreements.is_empty(),
            "KA-03: a planted defect `{to}` is detected by the differential"
        );
        eprintln!(
            "KA-03: plant `{to}`: {} of {} runs disagree, the first {}",
            disagreements.len(),
            runs.len(),
            disagreements[0]
        );
    }
}

/// The observable of one root on the given operands.
fn observe(low: &Lowered, operands: &[&[u8]]) -> Json {
    let arguments: Vec<Value> = operands.iter().map(|operand| bytes(operand)).collect();
    let outcome = interp::run(&low.program, FUEL, low.entry(), &arguments);
    rust::observable(&outcome).unwrap_or_else(|| panic!("{outcome:?}"))
}

/// `KA-04`: the five compositions of the sha256 axis.
fn composition() {
    let project = P::workspace_project("kappa", &MEMBERS);
    project.check_ok();
    let lowered = lowered_roots(
        &project,
        &[
            "composeG2",
            "composeF4",
            "composeE6",
            "composeE7",
            "composeE8",
        ],
    );
    let members = [
        "composeG2",
        "composeF4",
        "composeE6",
        "composeE7",
        "composeE8",
    ];
    let refusal = serde_json::to_value(Value::None).expect("a value is JSON");
    for member in members {
        let runs = corpus(member, COMPOSE_RUNS);
        assert!(
            runs.iter().any(|(_, expected)| *expected == refusal)
                && runs.iter().any(|(_, expected)| *expected != refusal),
            "KA-04: {member}: the corpus holds both labels and refusals"
        );
        let disagreements = evaluate(&lowered[member], &runs);
        assert!(
            disagreements.is_empty(),
            "KA-04: {member}: the evaluator and uor-addr disagree on {} of {} runs: {:?}",
            disagreements.len(),
            runs.len(),
            &disagreements[..disagreements.len().min(5)]
        );
    }
    // The laws, read from the evaluator: the product is commutative and a
    // label and its mirror compose alike, as the oracle's own tests state.
    let operands = valid_operands("sha256", &mut Seeded(0x6b61_0004_0000_0001), COMPOSE_RUNS);
    for (index, left) in operands.iter().enumerate().take(24) {
        let right = &operands[(index * 5 + 1) % operands.len()];
        assert_eq!(
            observe(&lowered["composeG2"], &[left, right]),
            observe(&lowered["composeG2"], &[right, left]),
            "KA-04: the product is commutative"
        );
    }
    for operand in operands.iter().take(12) {
        let mirror: Vec<u8> = format!(
            "sha256:{}",
            String::from_utf8_lossy(&operand[7..])
                .chars()
                .map(|digit| {
                    let value = digit.to_digit(16).expect("a hexadecimal digit");
                    char::from_digit(15 - value, 16).expect("a digit")
                })
                .collect::<String>()
        )
        .into_bytes();
        assert_eq!(
            observe(&lowered["composeF4"], &[operand]),
            observe(&lowered["composeF4"], &[&mirror]),
            "KA-04: a label and its mirror compose alike"
        );
    }
    rustc_differential(&lowered, &members);
    // The gate is armed: a product that keeps the operands in the order
    // given, a mirror that is not the complement, a degree byte that is
    // never six, and quarters left in place are each detected.
    for (member, from, to) in [
        (
            "composeG2",
            "(prim append (lower left right) (upper left right) : Bytes)",
            "(prim append left right : Bytes)",
        ),
        (
            "composeF4",
            "(prim subtract 15 (Json::hexValue",
            "(prim subtract 14 (Json::hexValue",
        ),
        ("composeE6", "9 0 : Nat) =? 8)", "9 0 : Nat) =? 9)"),
        (
            "composeE7",
            "(prim append a2 (prim append b3 (prim append c3 d2 : Bytes) : Bytes) : Bytes)",
            "(prim append a0 (prim append b0 (prim append c0 d0 : Bytes) : Bytes) : Bytes)",
        ),
    ] {
        let defective = planted("Compose", from, to);
        let checked = support::checked_project(&defective);
        let low = lower_in(&checked, &defective, COMPOSE, member);
        let runs = corpus(member, COMPOSE_RUNS);
        let disagreements = evaluate(&low, &runs);
        assert!(
            !disagreements.is_empty(),
            "KA-04: a planted defect `{to}` in `{member}` is detected by the differential"
        );
        eprintln!(
            "KA-04: plant in {member}: {} of {} runs disagree, the first {}",
            disagreements.len(),
            runs.len(),
            disagreements[0]
        );
    }
}

/// The documents hashed per axis under the evaluator and under rustc.
const HASH_EVALUATOR_RUNS: usize = 55;
const HASH_RUSTC_RUNS: usize = 70;
/// Seeded operands of each composition on an axis under rustc.
const COMPOSE_RUSTC_RUNS: usize = 12;
/// Seeded operands of each composition on an axis under the evaluator.
const COMPOSE_EVALUATOR_RUNS: usize = 6;

/// The number of seeded runs of a root under rustc.
fn rustc_runs(member: &str) -> usize {
    match member {
        "labelOn" | "jsonLabelOn" => HASH_RUSTC_RUNS,
        "composeG2On" | "composeF4On" | "composeE6On" | "composeE7On" | "composeE8On" => {
            COMPOSE_RUSTC_RUNS
        }
        "canonicalGguf" | "ggufLabel" | "ggufLabelOn" => GGUF_RUSTC_RUNS,
        "canonicalOnnx" | "onnxLabel" | "onnxLabelOn" => ONNX_RUSTC_RUNS,
        _ => RUSTC_RUNS,
    }
}

/// A plant: the module, the text, its replacement, and the root whose
/// differential must see it.
type Plant = (&'static str, &'static str, &'static str, &'static str);

/// Run each plant on the corpus of its root and require a disagreement.
fn detect_plants(id: &str, plants: &[Plant], runs: impl Fn(&str) -> Vec<Run>) {
    for (module, from, to, member) in plants {
        let defective = planted(module, from, to);
        let checked = support::checked_project(&defective);
        let owner = ROOTS
            .iter()
            .find(|(_, name, _)| name == member)
            .map(|(owner, _, _)| *owner)
            .expect("a root of the package");
        let low = lower_in(&checked, &defective, owner, member);
        let corpus = runs(member);
        let disagreements = evaluate(&low, &corpus);
        assert!(
            !disagreements.is_empty(),
            "{id}: a planted defect `{to}` in {module} is detected by the differential on `{member}`"
        );
        eprintln!(
            "{id}: plant `{to}` in {module}: {} of {} runs of {member} disagree, the first {}",
            disagreements.len(),
            corpus.len(),
            disagreements[0]
        );
    }
}

/// `KA-05`: the hash axes beside sha256.
fn hash_axes() {
    let project = P::workspace_project("kappa", &MEMBERS);
    project.check_ok();
    let members = ["labelOn", "isLabelOn", "jsonLabelOn"];
    let lowered = lowered_roots(&project, &members);
    let refusal = serde_json::to_value(Value::None).expect("a value is JSON");
    for member in members {
        let runs = corpus(member, HASH_EVALUATOR_RUNS);
        if member != "isLabelOn" {
            assert!(
                runs.iter().any(|(_, expected)| *expected == refusal)
                    && runs.iter().any(|(_, expected)| *expected != refusal),
                "KA-05: {member}: the corpus holds both labels and refusals"
            );
        }
        let disagreements = evaluate(&lowered[member], &runs);
        assert!(
            disagreements.is_empty(),
            "KA-05: {member}: the evaluator and uor-addr disagree on {} of {} runs: {:?}",
            disagreements.len(),
            runs.len(),
            &disagreements[..disagreements.len().min(5)]
        );
    }
    rustc_differential(&lowered, &members);
    // The gate is armed: a rotation of SHA-512 off by one, a padding that
    // closes a block one byte early, the SHA-3 suffix on the Keccak axis, a
    // Keccak without its closing bit, a rate that forgets the byte shared by
    // the suffix and the closing bit, an axis that hashes with another's
    // function, a width of SHA-512 digits one short, and in BLAKE3 a changed
    // rotation, message permutation, root flag, chunk count, tree merge, and
    // chunk start flag are each detected.
    detect_plants(
        "KA-05",
        &[
            (
                "Sha512",
                "(Word::rotateRight value (28 : UInt32) (36 : UInt32))",
                "(Word::rotateRight value (27 : UInt32) (37 : UInt32))",
                "labelOn",
            ),
            (
                "Sha512",
                "(prim quotient (size + 144) 128 0 : Nat)",
                "(prim quotient (size + 143) 128 0 : Nat)",
                "labelOn",
            ),
            (
                "Keccak",
                "(squeezeHex message (6 : UInt64))",
                "(squeezeHex message (1 : UInt64))",
                "labelOn",
            ),
            (
                "Keccak",
                "(prim bit_or body (128 : UInt64) : UInt64)",
                "(prim bit_or body (0 : UInt64) : UInt64)",
                "labelOn",
            ),
            (
                "Keccak",
                "((prim quotient size 136 0 : Nat) + 1)",
                "((prim quotient (size + 1) 136 0 : Nat) + 1)",
                "labelOn",
            ),
            (
                "Axis",
                "(Keccak::keccakHex message)",
                "(Keccak::sha3Hex message)",
                "jsonLabelOn",
            ),
            (
                "Axis",
                "(if (named axis \"sha512\") then 128 else (if",
                "(if (named axis \"sha512\") then 127 else (if",
                "isLabelOn",
            ),
            (
                "Blake3",
                "(LexLeanStd.Sha256::rotateRight (prim bit_xor d a1 : UInt32) (16 : UInt32) (16 : UInt32))",
                "(LexLeanStd.Sha256::rotateRight (prim bit_xor d a1 : UInt32) (8 : UInt32) (24 : UInt32))",
                "labelOn",
            ),
            (
                "Blake3",
                "m0 := (project m m2)",
                "m0 := (project m m3)",
                "labelOn",
            ),
            (
                "Blake3",
                "(prim bit_or (4 : UInt32) (if root then (8 : UInt32) else (0 : UInt32)) : UInt32)",
                "(prim bit_or (4 : UInt32) (if root then (0 : UInt32) else (0 : UInt32)) : UInt32)",
                "labelOn",
            ),
            (
                "Blake3",
                "(prim quotient (size + 1023) 1024 0 : Nat)",
                "(prim quotient (size + 1024) 1024 0 : Nat)",
                "labelOn",
            ),
            (
                "Blake3",
                "(prim remainder (project state total) 2 0 : Nat) =? 0)",
                "(prim remainder (project state total) 2 0 : Nat) =? 1)",
                "labelOn",
            ),
            (
                "Blake3",
                "(if (last =? 0) then (3 : UInt32) else (2 : UInt32))",
                "(if (last =? 0) then (2 : UInt32) else (2 : UInt32))",
                "labelOn",
            ),
        ],
        |member| corpus(member, HASH_EVALUATOR_RUNS),
    );
}

/// `KA-06`: the five compositions on the axes beside sha256.
fn axis_composition() {
    let project = P::workspace_project("kappa", &MEMBERS);
    project.check_ok();
    let members = [
        "composeG2On",
        "composeF4On",
        "composeE6On",
        "composeE7On",
        "composeE8On",
    ];
    let lowered = lowered_roots(&project, &members);
    let refusal = serde_json::to_value(Value::None).expect("a value is JSON");
    for member in members {
        let runs = corpus(member, COMPOSE_EVALUATOR_RUNS);
        assert!(
            runs.iter().any(|(_, expected)| *expected == refusal)
                && runs.iter().any(|(_, expected)| *expected != refusal),
            "KA-06: {member}: the corpus holds both labels and refusals"
        );
        let disagreements = evaluate(&lowered[member], &runs);
        assert!(
            disagreements.is_empty(),
            "KA-06: {member}: the evaluator and uor-addr disagree on {} of {} runs: {:?}",
            disagreements.len(),
            runs.len(),
            &disagreements[..disagreements.len().min(5)]
        );
    }
    // The laws, read from the evaluator on every axis: the product is
    // commutative and a label and its mirror compose alike.
    for (axis, _) in AXES {
        let operands = valid_operands(axis, &mut Seeded(0x6b61_0006_0000_0002), 4);
        for (index, left) in operands.iter().enumerate().take(12) {
            let right = &operands[(index * 5 + 1) % operands.len()];
            assert_eq!(
                observe(&lowered["composeG2On"], &[axis.as_bytes(), left, right]),
                observe(&lowered["composeG2On"], &[axis.as_bytes(), right, left]),
                "KA-06: the product on {axis} is commutative"
            );
        }
        for operand in operands.iter().take(8) {
            let mirror: Vec<u8> = format!(
                "{axis}:{}",
                String::from_utf8_lossy(&operand[axis.len() + 1..])
                    .chars()
                    .map(|digit| {
                        let value = digit.to_digit(16).expect("a hexadecimal digit");
                        char::from_digit(15 - value, 16).expect("a digit")
                    })
                    .collect::<String>()
            )
            .into_bytes();
            assert_eq!(
                observe(&lowered["composeF4On"], &[axis.as_bytes(), operand]),
                observe(&lowered["composeF4On"], &[axis.as_bytes(), &mirror]),
                "KA-06: a label and its mirror compose alike on {axis}"
            );
        }
    }
    rustc_differential(&lowered, &members);
    // The gate is armed: a product that keeps the operands in the order
    // given, a mirror that is not the complement, a degree byte that is never
    // six, quarters of a fixed sixteen digits, and a degree read at the
    // offset of the sha256 prefix are each detected.
    detect_plants(
        "KA-06",
        &[
            (
                "Compose",
                "(prim append (lower left right) (upper left right) : Bytes)",
                "(prim append left right : Bytes)",
                "composeG2On",
            ),
            (
                "Compose",
                "(prim subtract 15 (Json::hexValue",
                "(prim subtract 14 (Json::hexValue",
                "composeF4On",
            ),
            (
                "Compose",
                "9 0 : Nat) =? 8)",
                "9 0 : Nat) =? 9)",
                "composeE6On",
            ),
            (
                "Compose",
                "(prim quotient (Axis::width axis) 4 0 : Nat)",
                "16",
                "composeE7On",
            ),
            (
                "Compose",
                "(degreeTag operand (prim length (Axis::prefixOf axis) : Nat))",
                "(degreeTag operand 7)",
                "composeE6On",
            ),
        ],
        |member| corpus(member, COMPOSE_EVALUATOR_RUNS),
    );
}

/// The three committed GGUF fixtures of uor-addr (v0.2.0,
/// `tests/fixtures/gguf/`), vendored under `tests/kappa/gguf/` beside their
/// `.kappa-label` files.
const GGUF_FIXTURES: [&str; 3] = ["aligned-256", "empty-metadata", "synthetic-f32"];
/// Seeded GGUF files under the evaluator.
const GGUF_EVALUATOR_RUNS: usize = 60;
/// Seeded GGUF files under rustc.
const GGUF_RUSTC_RUNS: usize = 400;

/// The ggml tensor types uor-addr knows: the identifier, the bytes of a
/// block, and the elements of a block.
const GGML_TYPES: [(u32, u64, u64); 29] = [
    (0, 4, 1),
    (1, 2, 1),
    (2, 18, 32),
    (3, 20, 32),
    (6, 22, 32),
    (7, 24, 32),
    (8, 34, 32),
    (9, 36, 32),
    (10, 84, 256),
    (11, 110, 256),
    (12, 144, 256),
    (13, 176, 256),
    (14, 210, 256),
    (15, 292, 256),
    (16, 66, 256),
    (17, 74, 256),
    (18, 98, 256),
    (19, 50, 256),
    (20, 18, 32),
    (21, 110, 256),
    (22, 82, 256),
    (23, 136, 256),
    (24, 1, 1),
    (25, 2, 1),
    (26, 4, 1),
    (27, 8, 1),
    (28, 8, 1),
    (29, 56, 256),
    (30, 2, 1),
];

/// What the oracle says of a GGUF file: its canonical skeleton, or `None` for
/// a refusal.
fn oracle_gguf(raw: &[u8]) -> Option<Vec<u8>> {
    uor_addr::gguf::canonicalize(raw).ok()
}

/// What the oracle says of a GGUF file on a σ-axis: its κ-label there, or
/// `None` for a refusal.
fn oracle_gguf_address(axis: &str, raw: &[u8]) -> Option<String> {
    use uor_addr::gguf as g;
    match axis {
        "sha256" => g::address(raw)
            .ok()
            .map(|outcome| outcome.address.as_str().to_owned()),
        "blake3" => g::address_blake3(raw)
            .ok()
            .map(|outcome| outcome.address.as_str().to_owned()),
        "sha3-256" => g::address_sha3_256(raw)
            .ok()
            .map(|outcome| outcome.address.as_str().to_owned()),
        "keccak256" => g::address_keccak256(raw)
            .ok()
            .map(|outcome| outcome.address.as_str().to_owned()),
        "sha512" => g::address_sha512(raw)
            .ok()
            .map(|outcome| outcome.address.as_str().to_owned()),
        _ => None,
    }
}

fn put_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}
fn put_u64(out: &mut Vec<u8>, value: u64) {
    out.extend_from_slice(&value.to_le_bytes());
}
fn put_text(out: &mut Vec<u8>, text: &[u8]) {
    put_u64(out, u64::try_from(text.len()).expect("a length"));
    out.extend_from_slice(text);
}
fn align_up(offset: u64, alignment: u64) -> u64 {
    offset.div_ceil(alignment) * alignment
}

/// A GGUF v3 header with the counts given.
fn gguf_header(tensors: u64, pairs: u64) -> Vec<u8> {
    let mut out = b"GGUF".to_vec();
    put_u32(&mut out, 3);
    put_u64(&mut out, tensors);
    put_u64(&mut out, pairs);
    out
}

/// A file of one metadata value: `depth` arrays nested in one another around
/// an empty array of `inner` elements.
fn nested_arrays(depth: usize, inner: u32) -> Vec<u8> {
    let mut out = gguf_header(0, 1);
    put_text(&mut out, b"nested");
    put_u32(&mut out, 9);
    for _ in 1..depth {
        put_u32(&mut out, 9);
        put_u64(&mut out, 1);
    }
    put_u32(&mut out, inner);
    put_u64(&mut out, 0);
    out
}

/// A file of one tensor of the type, the dimensions, and the stored offset
/// given, followed by `data` bytes of tensor data.
fn one_tensor(kind: u32, dims: &[u64], offset: u64, data: usize) -> Vec<u8> {
    let mut out = gguf_header(1, 0);
    put_text(&mut out, b"t");
    put_u32(&mut out, u32::try_from(dims.len()).expect("a rank"));
    for dim in dims {
        put_u64(&mut out, *dim);
    }
    put_u32(&mut out, kind);
    put_u64(&mut out, offset);
    let start = usize::try_from(align_up(out.len() as u64, 32)).expect("a size");
    out.resize(start + data, 0x5a);
    out
}

/// Files at the edges of the form: what the oracle accepts and refuses at
/// each bound it checks.
fn gguf_edges() -> Vec<Vec<u8>> {
    let mut edges = vec![
        Vec::new(),
        b"GGUF".to_vec(),
        gguf_header(0, 0)[..23].to_vec(),
        gguf_header(0, 0),
        nested_arrays(64, 0),
        nested_arrays(65, 0),
        nested_arrays(2, 13),
        one_tensor(0, &[4, 2], 0, 32),
        one_tensor(0, &[4, 2], 0, 31),
        one_tensor(0, &[4, 2], 1, 32),
        one_tensor(0, &[4, 2], u64::MAX, 32),
        one_tensor(4, &[4], 0, 64),
        one_tensor(2, &[33], 0, 64),
        one_tensor(2, &[32], 0, 18),
        one_tensor(0, &[1, 1, 1, 1, 1], 0, 64),
        one_tensor(0, &[], 0, 4),
        one_tensor(0, &[0], 0, 0),
        one_tensor(0, &[1 << 32, 1 << 32], 0, 4),
        one_tensor(15, &[1 << 40, 1 << 24], 0, 4),
    ];
    let mut version = gguf_header(0, 0);
    version[4] = 2;
    edges.push(version);
    let mut magic = gguf_header(0, 0);
    magic[0] = b'g';
    edges.push(magic);
    edges.push(gguf_header(0, u64::MAX));
    edges.push(gguf_header(u64::MAX, 0));
    for (tag, value) in [
        (4_u32, 4_u64),
        (4, 8),
        (4, 7),
        (4, 0),
        (4, 48),
        (4, 1 << 31),
        (10, 7),
    ] {
        let mut out = gguf_header(0, 1);
        put_text(&mut out, b"general.alignment");
        put_u32(&mut out, tag);
        if tag == 4 {
            put_u32(&mut out, u32::try_from(value).expect("a u32"));
        } else {
            put_u64(&mut out, value);
        }
        edges.push(out);
    }
    let mut string = gguf_header(0, 1);
    put_text(&mut string, b"s");
    put_u32(&mut string, 8);
    put_u64(&mut string, u64::MAX);
    edges.push(string);
    edges
}

impl Seeded {
    /// One metadata value of the type tag, written at `depth`.
    fn gguf_value(&mut self, out: &mut Vec<u8>, tag: u32, depth: usize) {
        let width = match tag {
            0 | 1 | 7 => 1,
            2 | 3 => 2,
            4..=6 => 4,
            10..=12 => 8,
            8 => {
                let text: Vec<u8> = (0..self.below(12))
                    .map(|_| *self.pick(b"abcz.\x00\xff"))
                    .collect();
                put_text(out, &text);
                return;
            }
            _ => {
                let element = if depth < 2 {
                    *self.pick(&[0_u32, 4, 6, 8, 9, 10, 7])
                } else {
                    *self.pick(&[0_u32, 4, 8])
                };
                let count = self.below(4);
                put_u32(out, element);
                put_u64(out, count as u64);
                for _ in 0..count {
                    self.gguf_value(out, element, depth + 1);
                }
                return;
            }
        };
        for _ in 0..width {
            out.push(self.below(256) as u8);
        }
    }

    /// A seeded GGUF v3 file: metadata of every type tag, keys and tensor
    /// names in any order and repeated, an alignment of the metadata valid or
    /// not, tensors of every ggml type, and, one time in three, one defect.
    fn gguf_document(&mut self) -> Vec<u8> {
        const KEYS: [&[u8]; 7] = [
            b"general.name",
            b"general.architecture",
            b"a",
            b"ab",
            b"b",
            b"tokenizer.ggml.tokens",
            b"",
        ];
        const NAMES: [&[u8]; 5] = [b"t", b"token_embd.weight", b"a", b"blk.0.attn_q", b"t"];
        let alignment: Option<u64> = self
            .chance(40)
            .then(|| *self.pick(&[8_u64, 16, 32, 64, 256, 4, 7, 48]));
        let pairs = self.below(5);
        let alignment = alignment.filter(|_| pairs > 0);
        let mut body = Vec::new();
        let mut count = 0_u64;
        for index in 0..pairs {
            if alignment.is_some() && index == pairs / 2 {
                put_text(&mut body, b"general.alignment");
                put_u32(&mut body, 4);
                put_u32(
                    &mut body,
                    u32::try_from(alignment.unwrap_or(32)).expect("a u32"),
                );
                count += 1;
            }
            put_text(&mut body, self.pick(&KEYS));
            let tag = u32::try_from(self.below(13)).expect("a tag");
            put_u32(&mut body, tag);
            self.gguf_value(&mut body, tag, 0);
            count += 1;
        }
        let effective = alignment
            .filter(|value| *value >= 8 && value.is_power_of_two())
            .unwrap_or(32);
        let tensors = self.below(4);
        let mut end = 0_u64;
        let mut next = 0_u64;
        for _ in 0..tensors {
            let (kind, block, elements) = *self.pick(&GGML_TYPES);
            let rank = self.below(4);
            let mut dims = Vec::new();
            for axis in 0..rank {
                dims.push(if axis == 0 {
                    elements * (1 + self.below(2) as u64)
                } else {
                    1 + self.below(2) as u64
                });
            }
            let product: u64 = dims.iter().product();
            let size = product / elements * block;
            let offset = if self.chance(20) {
                self.below(usize::try_from(next + 1).expect("an offset")) as u64
            } else {
                next
            };
            next = align_up(offset + size, effective);
            end = end.max(offset + size);
            put_text(&mut body, self.pick(&NAMES));
            put_u32(&mut body, u32::try_from(rank).expect("a rank"));
            for dim in dims {
                put_u64(&mut body, dim);
            }
            put_u32(&mut body, kind);
            put_u64(&mut body, offset);
        }
        let mut out = gguf_header(tensors as u64, count);
        out.extend_from_slice(&body);
        let start = usize::try_from(align_up(out.len() as u64, effective)).expect("a start");
        out.resize(start, 0);
        for _ in 0..end {
            out.push(self.below(256) as u8);
        }
        if self.chance(33) && !out.is_empty() {
            match self.below(5) {
                0 => {
                    let cut = self.below(out.len());
                    out.truncate(cut);
                }
                1 => {
                    let at = self.below(out.len());
                    out[at] ^= 1 << self.below(8);
                }
                2 => out.extend_from_slice(b"trailing"),
                3 => out[16] = out[16].wrapping_add(1),
                _ => out[8] = out[8].wrapping_add(1),
            }
        }
        out
    }
}

/// The GGUF files of a corpus: the three committed fixtures, the edges, and
/// `runs` seeded files.
fn gguf_subjects(runs: usize, seed: u64) -> Vec<Vec<u8>> {
    let root = repo_root().join("tests/kappa/gguf");
    let mut subjects: Vec<Vec<u8>> = GGUF_FIXTURES
        .iter()
        .map(|name| {
            std::fs::read(root.join(format!("{name}.gguf")).as_std_path())
                .unwrap_or_else(|error| panic!("{name}.gguf: {error}"))
        })
        .collect();
    subjects.extend(gguf_edges());
    let mut seeded = Seeded(seed);
    subjects.extend((0..runs).map(|_| seeded.gguf_document()));
    subjects
}

/// The corpus of a root of the module `Gguf`: the oracle's skeleton, its
/// κ-label, or its κ-label on each axis and on a name that is no axis.
fn gguf_corpus(member: &str, runs: usize) -> Vec<Run> {
    let mut out = Vec::new();
    match member {
        "canonicalGguf" => {
            for subject in gguf_subjects(runs, 0x4b41_0007_0000_0001) {
                let expected = oracle_gguf(&subject);
                record(
                    &mut out,
                    vec![bytes(&subject)],
                    &option_bytes(expected.as_deref()),
                );
            }
        }
        "ggufLabel" => {
            for subject in gguf_subjects(runs, 0x4b41_0007_0000_0002) {
                let expected = oracle_gguf_address("sha256", &subject);
                record(
                    &mut out,
                    vec![bytes(&subject)],
                    &option_bytes(expected.as_deref().map(str::as_bytes)),
                );
            }
        }
        _ => {
            let subjects = gguf_subjects(runs / 5, 0x4b41_0007_0000_0003);
            for (index, subject) in subjects.iter().enumerate() {
                let axis = if index % 11 == 10 {
                    "sha256x"
                } else {
                    AXES[index % AXES.len()].0
                };
                let expected = oracle_gguf_address(axis, subject);
                record(
                    &mut out,
                    vec![bytes(axis.as_bytes()), bytes(subject)],
                    &option_bytes(expected.as_deref().map(str::as_bytes)),
                );
            }
        }
    }
    out
}

/// `KA-07`: the GGUF realization.
fn gguf() {
    let project = P::workspace_project("kappa", &MEMBERS);
    project.check_ok();
    let members = ["canonicalGguf", "ggufLabel", "ggufLabelOn"];
    let lowered = lowered_roots(&project, &members);
    // The three committed fixtures: the committed label, the oracle's, and
    // the package's, as the evaluator computes it.
    let root = repo_root().join("tests/kappa/gguf");
    let label = &lowered["ggufLabel"];
    for name in GGUF_FIXTURES {
        let raw = std::fs::read(root.join(format!("{name}.gguf")).as_std_path())
            .unwrap_or_else(|error| panic!("{name}.gguf: {error}"));
        let committed =
            std::fs::read_to_string(root.join(format!("{name}.kappa-label")).as_std_path())
                .unwrap_or_else(|error| panic!("{name}.kappa-label: {error}"));
        let committed = committed.trim_end();
        assert_eq!(
            oracle_gguf_address("sha256", &raw).as_deref(),
            Some(committed),
            "KA-07: {name}: the oracle gives the committed label"
        );
        let outcome = interp::run(&label.program, FUEL, label.entry(), &[bytes(&raw)]);
        let observed = rust::observable(&outcome).expect("an observable");
        assert_eq!(
            observed,
            serde_json::to_value(option_bytes(Some(committed.as_bytes()))).expect("json"),
            "KA-07: {name}: the package gives the committed label"
        );
        eprintln!("KA-07: {name}: {committed}");
    }
    let refusal = serde_json::to_value(Value::None).expect("a value is JSON");
    for member in members {
        let runs = corpus(member, GGUF_EVALUATOR_RUNS);
        assert!(
            runs.iter().any(|(_, expected)| *expected == refusal)
                && runs.iter().any(|(_, expected)| *expected != refusal),
            "KA-07: {member}: the corpus holds both skeletons and refusals"
        );
        let disagreements = evaluate(&lowered[member], &runs);
        assert!(
            disagreements.is_empty(),
            "KA-07: {member}: the evaluator and uor-addr disagree on {} of {} runs: {:?}",
            disagreements.len(),
            runs.len(),
            &disagreements[..disagreements.len().min(5)]
        );
    }
    rustc_differential(&lowered, &members);
    // The gate is armed: metadata in reverse key order, tensors in reverse
    // name order, a default alignment of sixteen, an alignment of four
    // admitted, a string digested from its length field, canonical offsets
    // left unaligned, a data section that starts unaligned, and an array
    // depth bound one deeper are each detected.
    detect_plants(
        "KA-07",
        &[
            (
                "Gguf",
                "(if (Json::lessKey key (first item)) then",
                "(if (Json::lessKey (first item) key) then",
                "canonicalGguf",
            ),
            (
                "Gguf",
                "(if (Json::lessKey (project tensor name) (project item name)) then",
                "(if (Json::lessKey (project item name) (project tensor name)) then",
                "canonicalGguf",
            ),
            (
                "Gguf",
                "alignment := 32, entries",
                "alignment := 16, entries",
                "canonicalGguf",
            ),
            (
                "Gguf",
                "(if ((8 <=? alignment) && (isPowerOfTwo alignment))",
                "(if ((4 <=? alignment) && (isPowerOfTwo alignment))",
                "canonicalGguf",
            ),
            (
                "Gguf",
                "(digest (LexLeanStd.Text::sliceOr subject (at + 8)",
                "(digest (LexLeanStd.Text::sliceOr subject (at + 4)",
                "canonicalGguf",
            ),
            (
                "Gguf",
                "offset := (alignUp ((project state offset) + (project tensor size)) alignment)",
                "offset := ((project state offset) + (project tensor size))",
                "canonicalGguf",
            ),
            (
                "Gguf",
                "(let start : Nat := (alignUp (project tensors position) alignment);",
                "(let start : Nat := (project tensors position);",
                "canonicalGguf",
            ),
            (
                "Gguf",
                "(64 <=? (project top depth))",
                "(65 <=? (project top depth))",
                "canonicalGguf",
            ),
        ],
        |member| corpus(member, GGUF_EVALUATOR_RUNS),
    );
}

/// The two committed ONNX fixtures of uor-addr (v0.2.0,
/// `tests/fixtures/onnx/`), vendored under `tests/kappa/onnx/` beside their
/// `.kappa-label` files.
const ONNX_FIXTURES: [&str; 2] = ["synthetic", "synthetic-typed"];
/// Seeded ONNX models under the evaluator.
const ONNX_EVALUATOR_RUNS: usize = 50;
/// Seeded ONNX models under rustc.
const ONNX_RUSTC_RUNS: usize = 300;

/// What the oracle says of an ONNX model: its canonical skeleton, or `None`
/// for a refusal.
fn oracle_onnx(raw: &[u8]) -> Option<Vec<u8>> {
    uor_addr::onnx::canonicalize(raw).ok()
}

/// What the oracle says of an ONNX model on a σ-axis: its κ-label there, or
/// `None` for a refusal.
fn oracle_onnx_address(axis: &str, raw: &[u8]) -> Option<String> {
    use uor_addr::onnx as o;
    match axis {
        "sha256" => o::address(raw)
            .ok()
            .map(|outcome| outcome.address.as_str().to_owned()),
        "blake3" => o::address_blake3(raw)
            .ok()
            .map(|outcome| outcome.address.as_str().to_owned()),
        "sha3-256" => o::address_sha3_256(raw)
            .ok()
            .map(|outcome| outcome.address.as_str().to_owned()),
        "keccak256" => o::address_keccak256(raw)
            .ok()
            .map(|outcome| outcome.address.as_str().to_owned()),
        "sha512" => o::address_sha512(raw)
            .ok()
            .map(|outcome| outcome.address.as_str().to_owned()),
        _ => None,
    }
}

/// A protocol-buffers writer.
#[derive(Default)]
struct Proto(Vec<u8>);

impl Proto {
    fn raw_varint(&mut self, mut value: u64) -> &mut Self {
        loop {
            let byte = (value & 0x7f) as u8;
            value >>= 7;
            if value == 0 {
                self.0.push(byte);
                return self;
            }
            self.0.push(byte | 0x80);
        }
    }
    fn tag(&mut self, number: u64, wire: u64) -> &mut Self {
        self.raw_varint((number << 3) | wire)
    }
    fn varint(&mut self, number: u64, value: u64) -> &mut Self {
        self.tag(number, 0).raw_varint(value)
    }
    fn bytes(&mut self, number: u64, value: &[u8]) -> &mut Self {
        self.tag(number, 2)
            .raw_varint(value.len() as u64)
            .0
            .extend_from_slice(value);
        self
    }
    fn fixed32(&mut self, number: u64, value: u32) -> &mut Self {
        self.tag(number, 5)
            .0
            .extend_from_slice(&value.to_le_bytes());
        self
    }
    fn fixed64(&mut self, number: u64, value: u64) -> &mut Self {
        self.tag(number, 1)
            .0
            .extend_from_slice(&value.to_le_bytes());
        self
    }
    fn done(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.0)
    }
}

/// A packed run of varints.
fn packed_varints(values: &[u64]) -> Vec<u8> {
    let mut out = Proto::default();
    for value in values {
        out.raw_varint(*value);
    }
    out.done()
}

impl Seeded {
    fn onnx_name(&mut self) -> Vec<u8> {
        self.pick(&[&b""[..], b"x", b"y", b"a", b"conv", b"\xc3\xa9"])
            .to_vec()
    }

    /// A message of random fields in any order, whose length-delimited
    /// payloads are messages again or not, to `depth` levels.
    fn onnx_message(&mut self, depth: usize) -> Vec<u8> {
        let mut out = Proto::default();
        for _ in 0..self.below(4) {
            let number = 1 + self.below(6) as u64;
            match self.below(5) {
                0 => {
                    out.varint(number, self.next() >> self.below(64));
                }
                1 => {
                    out.fixed64(number, self.next());
                }
                2 => {
                    out.fixed32(number, self.next() as u32);
                }
                3 if depth > 0 => {
                    let inner = self.onnx_message(depth - 1);
                    out.bytes(number, &inner);
                }
                _ => {
                    let text = self.onnx_name();
                    out.bytes(number, &text);
                }
            }
        }
        out.done()
    }

    /// A tensor of a data type the oracle knows, or one time in ten of one
    /// it does not.
    fn onnx_tensor(&mut self) -> Vec<u8> {
        let mut out = Proto::default();
        let kind = if self.chance(10) {
            *self.pick(&[0_u64, 24, 1 << 32 | 1])
        } else {
            1 + self.below(23) as u64
        };
        let dims: Vec<u64> = (0..self.below(3)).map(|_| self.below(4) as u64).collect();
        if self.chance(50) {
            out.bytes(1, &packed_varints(&dims));
        } else {
            for dim in &dims {
                out.varint(1, *dim);
            }
        }
        out.varint(2, kind);
        let name = self.onnx_name();
        out.bytes(8, &name);
        match self.below(6) {
            0 => {
                let raw: Vec<u8> = (0..self.below(9)).map(|_| self.below(256) as u8).collect();
                out.bytes(9, &raw);
            }
            1 => {
                out.varint(14, 1);
                for _ in 0..self.below(3) {
                    let mut entry = Proto::default();
                    let key = self.onnx_name();
                    let value = self.onnx_name();
                    entry.bytes(1, &key).bytes(2, &value);
                    let entry = entry.done();
                    out.bytes(13, &entry);
                }
            }
            _ => {
                let number = *self.pick(&[4_u64, 5, 6, 7, 10, 11]);
                for _ in 0..self.below(3) {
                    match self.below(4) {
                        0 => {
                            let values: Vec<u64> =
                                (0..self.below(3)).map(|_| self.next() >> 40).collect();
                            out.bytes(number, &packed_varints(&values));
                        }
                        1 => {
                            out.varint(number, self.next() >> self.below(64));
                        }
                        2 => {
                            out.fixed32(number, self.next() as u32);
                        }
                        _ => {
                            let text = self.onnx_name();
                            out.bytes(number, &text);
                        }
                    }
                }
            }
        }
        out.done()
    }

    /// An attribute of any type, with a subgraph `depth` levels deep at most.
    fn onnx_attribute(&mut self, depth: usize) -> Vec<u8> {
        let mut out = Proto::default();
        let name = self.onnx_name();
        out.bytes(1, &name);
        let kind = if self.chance(5) {
            *self.pick(&[0_u64, 15, 1 << 32 | 2])
        } else {
            1 + self.below(14) as u64
        };
        out.varint(20, kind);
        match kind & 0xffff_ffff {
            1 => {
                out.fixed32(2, self.next() as u32);
            }
            2 => {
                out.varint(3, self.next() >> 8);
            }
            3 => {
                let text = self.onnx_name();
                out.bytes(4, &text);
            }
            4 => {
                if self.chance(90) {
                    let tensor = self.onnx_tensor();
                    out.bytes(5, &tensor);
                }
            }
            5 if depth > 0 => {
                let graph = self.onnx_graph(depth - 1);
                out.bytes(6, &graph);
            }
            6 => {
                out.fixed32(7, self.next() as u32);
                out.bytes(7, &(self.next() as u32).to_le_bytes());
            }
            7 => {
                let values: Vec<u64> = (0..self.below(4)).map(|_| self.next() >> 30).collect();
                out.bytes(8, &packed_varints(&values));
                out.varint(8, 7);
            }
            8 => {
                let text = self.onnx_name();
                out.bytes(9, &text).bytes(9, b"z");
            }
            9 => {
                for _ in 0..self.below(3) {
                    let tensor = self.onnx_tensor();
                    out.bytes(10, &tensor);
                }
            }
            10 if depth > 0 => {
                for _ in 0..self.below(3) {
                    let graph = self.onnx_graph(depth - 1);
                    out.bytes(11, &graph);
                }
            }
            11 | 13 => {
                let message = self.onnx_message(2);
                out.bytes(if kind == 11 { 22 } else { 14 }, &message);
            }
            12 | 14 => {
                for _ in 0..self.below(3) {
                    let message = self.onnx_message(2);
                    out.bytes(if kind == 12 { 23 } else { 15 }, &message);
                }
            }
            _ => {}
        }
        out.done()
    }

    /// A graph of nodes over a pool of value names, serialized in any order,
    /// so that its topological order, its ties, and now and then a cycle are
    /// all read.
    fn onnx_graph(&mut self, depth: usize) -> Vec<u8> {
        let mut out = Proto::default();
        let name = self.onnx_name();
        out.bytes(2, &name);
        let count = self.below(5);
        let mut nodes = Vec::new();
        for index in 0..count {
            let mut node = Proto::default();
            for _ in 0..self.below(3) {
                let source = self.below(index + 2);
                let input = if source <= index && self.chance(70) {
                    format!("v{source}").into_bytes()
                } else if self.chance(10) {
                    format!("v{index}").into_bytes()
                } else {
                    self.onnx_name()
                };
                node.bytes(1, &input);
            }
            node.bytes(2, format!("v{index}").as_bytes());
            if self.chance(20) {
                node.bytes(2, b"");
            }
            let name = self.pick(&[&b""[..], b"n", b"m"]).to_vec();
            node.bytes(3, &name);
            let op = self.pick(&[&b"Add"[..], b"Relu", b"Add"]).to_vec();
            node.bytes(4, &op);
            if self.chance(30) {
                let domain = self.pick(&[&b""[..], b"ai.onnx.ml"]).to_vec();
                node.bytes(7, &domain);
            }
            if self.chance(10) {
                node.bytes(8, b"overload");
            }
            for _ in 0..self.below(3) {
                let attribute = self.onnx_attribute(depth);
                node.bytes(5, &attribute);
            }
            nodes.push(node.done());
        }
        for _ in 0..nodes.len() {
            let left = self.below(nodes.len());
            let right = self.below(nodes.len());
            nodes.swap(left, right);
        }
        for node in nodes {
            out.bytes(1, &node);
        }
        for _ in 0..self.below(3) {
            let tensor = self.onnx_tensor();
            out.bytes(5, &tensor);
        }
        for number in [11_u64, 12, 13] {
            for _ in 0..self.below(2) {
                let mut info = Proto::default();
                let name = self.onnx_name();
                let typed = self.onnx_message(3);
                info.bytes(1, &name).bytes(2, &typed);
                let info = info.done();
                out.bytes(number, &info);
            }
        }
        out.done()
    }

    /// A seeded ONNX model: an IR version in range or not, opsets of any
    /// domain and version, a graph with nodes, initializers, value infos,
    /// and attributes of every type with subgraphs, model metadata, and, one
    /// time in three, one defect.
    fn onnx_document(&mut self) -> Vec<u8> {
        let mut out = Proto::default();
        let version = if self.chance(10) {
            *self.pick(&[0_u64, 14, u64::MAX])
        } else {
            1 + self.below(13) as u64
        };
        out.varint(1, version);
        for _ in 0..self.below(3) {
            let mut opset = Proto::default();
            let domain = self
                .pick(&[&b""[..], b"", b"ai.onnx.ml", b"com.x"])
                .to_vec();
            opset.bytes(1, &domain);
            opset.varint(2, *self.pick(&[0_u64, 1, 13, 17, 1 << 63]));
            let opset = opset.done();
            out.bytes(8, &opset);
        }
        let producer = self.onnx_name();
        out.bytes(2, &producer).bytes(3, b"1.0");
        if self.chance(50) {
            out.bytes(4, b"ai.example");
        }
        out.varint(5, self.below(4) as u64);
        let graph = self.onnx_graph(2);
        out.bytes(7, &graph);
        for _ in 0..self.below(3) {
            let mut entry = Proto::default();
            let key = self.onnx_name();
            let value = self.onnx_name();
            entry.bytes(1, &key).bytes(2, &value);
            let entry = entry.done();
            out.bytes(14, &entry);
        }
        let mut out = out.done();
        if self.chance(33) && !out.is_empty() {
            match self.below(4) {
                0 => {
                    let cut = self.below(out.len());
                    out.truncate(cut);
                }
                1 => {
                    let at = self.below(out.len());
                    out[at] ^= 1 << self.below(8);
                }
                2 => out.push(0x00),
                _ => out.extend_from_slice(&[0x0f, 0x01]),
            }
        }
        out
    }
}

/// A model of the IR version and opsets given around the graph given.
fn onnx_model(graph: &[u8], opsets: &[(&[u8], u64)]) -> Vec<u8> {
    let mut out = Proto::default();
    out.varint(1, 9);
    for (domain, version) in opsets {
        let mut opset = Proto::default();
        opset.bytes(1, domain).varint(2, *version);
        let opset = opset.done();
        out.bytes(8, &opset);
    }
    out.bytes(7, graph);
    out.done()
}

/// A graph of one node whose one attribute holds the graph given.
fn onnx_wrapped(inner: &[u8]) -> Vec<u8> {
    let mut attribute = Proto::default();
    attribute.bytes(1, b"body").varint(20, 5).bytes(6, inner);
    let attribute = attribute.done();
    let mut node = Proto::default();
    node.bytes(4, b"If").bytes(5, &attribute);
    let node = node.done();
    let mut graph = Proto::default();
    graph.bytes(2, b"g").bytes(1, &node);
    graph.done()
}

/// Models at the edges of the form: what the oracle accepts and refuses at
/// each bound it checks.
fn onnx_edges() -> Vec<Vec<u8>> {
    let mut graph = Proto::default();
    let simple = graph.bytes(2, b"g").done();
    let mut nested = simple.clone();
    let mut nests = Vec::new();
    for level in 1..=65 {
        nested = onnx_wrapped(&nested);
        if level >= 63 {
            nests.push(onnx_model(&nested, &[(b"", 13)]));
        }
    }
    let mut cycle = Proto::default();
    cycle.bytes(1, b"v0").bytes(2, b"v0").bytes(4, b"Loop");
    let cycle = cycle.done();
    let mut looped = Proto::default();
    looped.bytes(2, b"g").bytes(1, &cycle);
    let looped = looped.done();
    let mut deep = Proto::default();
    let mut message = Proto::default().varint(3, 7).fixed32(1, 9).done();
    for level in 0..34 {
        message = Proto::default().bytes(2, &message).varint(1, level).done();
    }
    let mut info = Proto::default();
    info.bytes(1, b"x").bytes(2, &message);
    let info = info.done();
    deep.bytes(2, b"g").bytes(11, &info);
    let deep = deep.done();
    let mut edges = vec![
        Vec::new(),
        Proto::default().varint(1, 9).done(),
        onnx_model(&simple, &[]),
        onnx_model(&simple, &[(b"", 0)]),
        onnx_model(&simple, &[(b"ai.onnx.ml", 3)]),
        onnx_model(&simple, &[(b"ai.onnx.ml", 3), (b"", 1)]),
        onnx_model(&simple, &[(b"", 1 << 63)]),
        onnx_model(&looped, &[(b"", 13)]),
        onnx_model(&deep, &[(b"", 13)]),
        vec![
            0x08, 0x89, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00,
        ],
        vec![
            0x08, 0x89, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00,
        ],
        vec![0x08, 0x09, 0x03, 0x00],
        vec![0x08, 0x09, 0x3a, 0xff, 0xff, 0xff, 0xff, 0x0f],
    ];
    edges.extend(nests);
    let mut version = onnx_model(&simple, &[(b"", 13)]);
    version[1] = 14;
    edges.push(version);
    edges
}

/// The ONNX models of a corpus: the two committed fixtures, the edges, and
/// `runs` seeded models.
fn onnx_subjects(runs: usize, seed: u64) -> Vec<Vec<u8>> {
    let root = repo_root().join("tests/kappa/onnx");
    let mut subjects: Vec<Vec<u8>> = ONNX_FIXTURES
        .iter()
        .map(|name| {
            std::fs::read(root.join(format!("{name}.onnx")).as_std_path())
                .unwrap_or_else(|error| panic!("{name}.onnx: {error}"))
        })
        .collect();
    subjects.extend(onnx_edges());
    let mut seeded = Seeded(seed);
    subjects.extend((0..runs).map(|_| seeded.onnx_document()));
    subjects
}

/// The corpus of a root of the module `Onnx`: the oracle's skeleton, its
/// κ-label, or its κ-label on each axis and on a name that is no axis.
fn onnx_corpus(member: &str, runs: usize) -> Vec<Run> {
    let mut out = Vec::new();
    match member {
        "canonicalOnnx" => {
            for subject in onnx_subjects(runs, 0x4b41_0008_0000_0001) {
                let expected = oracle_onnx(&subject);
                record(
                    &mut out,
                    vec![bytes(&subject)],
                    &option_bytes(expected.as_deref()),
                );
            }
        }
        "onnxLabel" => {
            for subject in onnx_subjects(runs, 0x4b41_0008_0000_0002) {
                let expected = oracle_onnx_address("sha256", &subject);
                record(
                    &mut out,
                    vec![bytes(&subject)],
                    &option_bytes(expected.as_deref().map(str::as_bytes)),
                );
            }
        }
        _ => {
            let subjects = onnx_subjects(runs / 5, 0x4b41_0008_0000_0003);
            for (index, subject) in subjects.iter().enumerate() {
                let axis = if index % 11 == 10 {
                    "sha256x"
                } else {
                    AXES[index % AXES.len()].0
                };
                let expected = oracle_onnx_address(axis, subject);
                record(
                    &mut out,
                    vec![bytes(axis.as_bytes()), bytes(subject)],
                    &option_bytes(expected.as_deref().map(str::as_bytes)),
                );
            }
        }
    }
    out
}

/// `KA-08`: the ONNX realization.
fn onnx() {
    let project = P::workspace_project("kappa", &MEMBERS);
    project.check_ok();
    let members = ["canonicalOnnx", "onnxLabel", "onnxLabelOn"];
    let lowered = lowered_roots(&project, &members);
    // The two committed fixtures: the committed label, the oracle's, and
    // the package's, as the evaluator computes it.
    let root = repo_root().join("tests/kappa/onnx");
    let label = &lowered["onnxLabel"];
    for name in ONNX_FIXTURES {
        let raw = std::fs::read(root.join(format!("{name}.onnx")).as_std_path())
            .unwrap_or_else(|error| panic!("{name}.onnx: {error}"));
        let committed =
            std::fs::read_to_string(root.join(format!("{name}.kappa-label")).as_std_path())
                .unwrap_or_else(|error| panic!("{name}.kappa-label: {error}"));
        let committed = committed.trim_end();
        assert_eq!(
            oracle_onnx_address("sha256", &raw).as_deref(),
            Some(committed),
            "KA-08: {name}: the oracle gives the committed label"
        );
        let outcome = interp::run(&label.program, FUEL, label.entry(), &[bytes(&raw)]);
        let observed = rust::observable(&outcome).expect("an observable");
        assert_eq!(
            observed,
            serde_json::to_value(option_bytes(Some(committed.as_bytes()))).expect("json"),
            "KA-08: {name}: the package gives the committed label"
        );
        eprintln!("KA-08: {name}: {committed}");
    }
    let refusal = serde_json::to_value(Value::None).expect("a value is JSON");
    for member in members {
        let runs = corpus(member, ONNX_EVALUATOR_RUNS);
        assert!(
            runs.iter().any(|(_, expected)| *expected == refusal)
                && runs.iter().any(|(_, expected)| *expected != refusal),
            "KA-08: {member}: the corpus holds both skeletons and refusals"
        );
        let disagreements = evaluate(&lowered[member], &runs);
        assert!(
            disagreements.is_empty(),
            "KA-08: {member}: the evaluator and uor-addr disagree on {} of {} runs: {:?}",
            disagreements.len(),
            runs.len(),
            &disagreements[..disagreements.len().min(5)]
        );
    }
    rustc_differential(&lowered, &members);
    // The gate is armed: the first of tied nodes instead of the last, an
    // order that ignores readiness, a message digested in wire order, a
    // message digested one level deep only, a subgraph bound one deeper,
    // attributes and initializers in wire order, another external-data
    // marker, and an opset of version zero admitted are each detected.
    detect_plants(
        "KA-08",
        &[
            (
                "Onnx",
                "(if (Json::lessKey (project (project choice best) key) (project node key)) then choice else (record Choice {found := true, best := node}))",
                "(if (Json::lessKey (project node key) (project (project choice best) key)) then (record Choice {found := true, best := node}) else choice)",
                "canonicalOnnx",
            ),
            (
                "Onnx",
                "(if (containsNat emitted (project producer index)) then false else true)",
                "(if ((containsNat emitted (project producer index)) || true) then false else true)",
                "canonicalOnnx",
            ),
            (
                "Onnx",
                "(record Frame {fields := (byNumber (second all)), acc := 0x, depth := 0})",
                "(record Frame {fields := (second all), acc := 0x, depth := 0})",
                "canonicalOnnx",
            ),
            (
                "Onnx",
                "(depth <? 32)",
                "(depth <? 1)",
                "canonicalOnnx",
            ),
            (
                "Onnx",
                "(depth <=? 64)",
                "(depth <=? 65)",
                "canonicalOnnx",
            ),
            (
                "Onnx",
                "(sortedBy attributes (scannedKeys attributes 1))",
                "attributes",
                "canonicalOnnx",
            ),
            (
                "Onnx",
                "(sortedBy tensors (scannedKeys tensors 8))",
                "tensors",
                "canonicalOnnx",
            ),
            (
                "Onnx",
                "\"onnx:external-data:v1\"",
                "\"onnx:external-data:v2\"",
                "canonicalOnnx",
            ),
            (
                "Onnx",
                "((0 <? (varintOf (second version))) &&",
                "((0 <=? (varintOf (second version))) &&",
                "canonicalOnnx",
            ),
        ],
        |member| corpus(member, ONNX_EVALUATOR_RUNS),
    );
}

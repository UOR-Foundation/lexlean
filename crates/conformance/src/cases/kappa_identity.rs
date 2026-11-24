//! LexLean's identities are BLAKE3 κ-labels (SPEC.md §21.1): `AR-15`, the
//! audit that refuses a SHA-256 identity outside the formats that require
//! one, and `AR-16`, the cross-implementation vectors and the three ways a
//! recipe can fail to be canonical.

use lexlean::artifact::content_id::{self, FramedHasher, Kappa};
use serde_json::Value as Json;

use crate::support;

/// The committed vectors, written by `tests/kappa/generate.py`.
const VECTORS: &str = "tests/kappa/vectors.json";

pub(crate) fn run(id: &str) {
    match id {
        "AR-15" => audit(),
        "AR-16" => vectors(),
        other => panic!("no κ-identity case is wired for {other}"),
    }
}

/// The repository's texts with `edit` applied to the file `file`.
fn planted(
    texts: &[(String, String)],
    file: &str,
    edit: impl Fn(&str) -> String,
) -> Vec<(String, String)> {
    let mut found = false;
    let out = texts
        .iter()
        .map(|(path, text)| {
            if path == file {
                found = true;
                let changed = edit(text);
                assert_ne!(&changed, text, "{file}: the plant site exists");
                (path.clone(), changed)
            } else {
                (path.clone(), text.clone())
            }
        })
        .collect();
    assert!(found, "{file} is read by the audit");
    out
}

fn audit() {
    let root = support::repo_root();
    let texts = repo_model::hashes::texts(root.as_std_path()).expect("the audited texts");
    let read = repo_model::hashes::check(&texts).unwrap_or_else(|refused| panic!("{refused}"));
    assert!(read > 500, "{read} files read");

    // The digest type of the old recipes, planted in the compiler's content
    // identity.
    let sha_type = format!("{}{}", "Sha", "256Digest");
    let digest_type = planted(
        &texts,
        "crates/lexlean/src/artifact/content_id.rs",
        |text| format!("{text}\npub struct {sha_type}(pub [u8; 32]);\n"),
    );
    assert!(
        repo_model::hashes::check(&digest_type).is_err(),
        "a SHA-256 digest type in the compiler is refused"
    );

    // A digest field of the old spelling, planted in a lock schema.
    let field = format!("{}{}", "tree_sha", "256");
    let lock_field = planted(&texts, "schemas/lock-v2.schema.json", |text| {
        text.replacen("\"tree_kappa\"", &format!("\"{field}\""), 1)
    });
    assert!(
        repo_model::hashes::check(&lock_field).is_err(),
        "a SHA-256 field in a schema is refused"
    );

    // A pattern that admits a bare digest, planted in a snapshot schema.
    let bare = repo_model::hashes::bare_digest_pattern();
    let snapshot_pattern = planted(&texts, "schemas/semantic-snapshot-v2.schema.json", |text| {
        text.replacen("\"^blake3:[0-9a-f]{64}$\"", &bare, 1)
    });
    assert!(
        repo_model::hashes::check(&snapshot_pattern).is_err(),
        "a bare 64-digit digest pattern in a schema is refused"
    );

    // The SHA-256 crate used in the release tooling, on a line no row
    // sanctions.
    let crate_use = format!("    let _ = {}{}Sha256::digest(b\"x\");", "sha", "2::");
    let tooling = planted(&texts, "xtask/src/main.rs", |text| {
        format!("{text}\n{crate_use}\n")
    });
    assert!(
        repo_model::hashes::check(&tooling).is_err(),
        "the SHA-256 crate outside a sanctioned line of the release tooling is refused"
    );

    // A scan that reads no source of the shipped crate finds nothing, so it
    // is refused rather than passed.
    let without: Vec<(String, String)> = texts
        .iter()
        .filter(|(path, _)| !path.starts_with("crates/lexlean/src/"))
        .cloned()
        .collect();
    assert!(
        repo_model::hashes::check(&without).is_err(),
        "a scan that reads no source of the shipped crate is refused"
    );
}

fn hex(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .map(|at| u8::from_str_radix(&text[at..at + 2], 16).expect("hexadecimal"))
        .collect()
}

fn text(value: &Json) -> &str {
    value.as_str().expect("a string")
}

fn kappa(value: &Json) -> Kappa {
    Kappa::parse(text(value)).expect("a κ-label")
}

/// uor-addr's typed κ-label of `carrier` on its `blake3` σ-axis.
fn uor_addr(carrier: &[u8]) -> String {
    let label = support::oracle_kappa(carrier).label();
    let typed =
        uor_addr::KappaLabel::<71>::from_bytes(label.as_bytes()).expect("uor-addr accepts it");
    assert_eq!(typed.sigma_axis(), Some("blake3"));
    typed.as_str().to_owned()
}

/// A framed recipe as the compiler's `FramedHasher` writes it.
fn framed(domain: &str, frames: &[(&str, &[u8])]) -> (Vec<u8>, Kappa) {
    let mut hasher = FramedHasher::new(domain);
    for (label, bytes) in frames {
        hasher.frame(label, bytes);
    }
    let carrier = hasher.carrier().to_vec();
    (carrier, hasher.finish())
}

/// The compiler's carrier and label of one vector.
fn recompute(vector: &Json) -> (Option<Vec<u8>>, Kappa) {
    let inputs = &vector["inputs"];
    match text(&vector["kind"]) {
        "leaf" => {
            let bytes = hex(text(&inputs["bytes"]));
            (Some(bytes.clone()), Kappa::of(&bytes))
        }
        "entry-signature" => {
            let bytes = text(&inputs["signature"]).as_bytes().to_vec();
            (Some(bytes.clone()), Kappa::of(&bytes))
        }
        "tree" => {
            let rows: Vec<(String, Vec<u8>)> = inputs["rows"]
                .as_array()
                .expect("rows")
                .iter()
                .map(|row| (text(&row["path"]).to_owned(), hex(text(&row["bytes"]))))
                .collect();
            let borrowed: Vec<(&str, &[u8])> = rows
                .iter()
                .map(|(path, bytes)| (path.as_str(), bytes.as_slice()))
                .collect();
            (None, content_id::tree_kappa(&borrowed))
        }
        "source" => {
            let sources: Vec<(String, Vec<u8>)> = inputs["sources"]
                .as_array()
                .expect("sources")
                .iter()
                .map(|row| (text(&row["path"]).to_owned(), hex(text(&row["bytes"]))))
                .collect();
            (
                None,
                content_id::source_id(
                    &hex(text(&inputs["project"])),
                    &hex(text(&inputs["lock"])),
                    &sources,
                ),
            )
        }
        "semantic" => {
            assert_eq!(text(&inputs["toolchain"]), lexlean::LEAN_TOOLCHAIN);
            (
                None,
                content_id::semantic_id_for(
                    kappa(&inputs["compiler_semantics"]),
                    text(&inputs["language"]),
                    text(&inputs["linked_ir"]),
                    text(&inputs["lexicon_closure"]),
                ),
            )
        }
        "build" => (
            None,
            content_id::build_id(kappa(&inputs["source_id"]), kappa(&inputs["semantic_id"])),
        ),
        "declaration-group" => {
            let semantics = kappa(&inputs["compiler_semantics"]);
            let mut frames: Vec<(&str, Vec<u8>)> = vec![
                ("compiler-semantics", semantics.as_bytes().to_vec()),
                ("language", text(&inputs["language"]).as_bytes().to_vec()),
                ("toolchain", text(&inputs["toolchain"]).as_bytes().to_vec()),
            ];
            for declaration in inputs["declarations"].as_array().expect("declarations") {
                frames.push(("declaration", text(declaration).as_bytes().to_vec()));
            }
            for reference in inputs["references"].as_array().expect("references") {
                frames.push(("reference", text(&reference["name"]).as_bytes().to_vec()));
                frames.push(("reference-id", kappa(&reference["id"]).as_bytes().to_vec()));
            }
            let borrowed: Vec<(&str, &[u8])> = frames
                .iter()
                .map(|(label, bytes)| (*label, bytes.as_slice()))
                .collect();
            let (carrier, label) = framed("lexlean-declaration-group-v2", &borrowed);
            (Some(carrier), label)
        }
        "declaration" => {
            let group = kappa(&inputs["group"]);
            let (carrier, label) = framed(
                "lexlean-declaration-v2",
                &[
                    ("group", group.as_bytes()),
                    ("name", text(&inputs["name"]).as_bytes()),
                ],
            );
            (Some(carrier), label)
        }
        other => panic!("{VECTORS}: no recipe for the kind `{other}`"),
    }
}

/// Whether `recipe` changes its label when any one input changes: the
/// recipe covers every field.
fn covers_every_field(recipe: &dyn Fn(&[&[u8]]) -> String, inputs: &[&[u8]]) -> bool {
    let base = recipe(inputs);
    (0..inputs.len()).all(|at| {
        let mut changed: Vec<Vec<u8>> = inputs.iter().map(|bytes| bytes.to_vec()).collect();
        changed[at].push(b'!');
        let borrowed: Vec<&[u8]> = changed.iter().map(Vec::as_slice).collect();
        recipe(&borrowed) != base
    })
}

/// A listing of modules: `(path, source)` rows.
type Listing = [(String, Vec<u8>)];

/// Whether two module listings that differ only in where a path ends and its
/// source begins get different labels.
fn separates_boundaries(recipe: &dyn Fn(&Listing) -> String) -> bool {
    let left = vec![("src/A.lex.tex".to_owned(), b"xy".to_vec())];
    let right = vec![("src/A.lex.texx".to_owned(), b"y".to_vec())];
    recipe(&left) != recipe(&right)
}

/// Whether two different entry signatures get different labels.
fn distinguishes_signatures(recipe: &dyn Fn(&str, &str) -> String) -> bool {
    recipe("nat", "(fun (n : Nat) Nat)") != recipe("nat", "(fun (n : Nat) Int)")
}

fn vectors() {
    let path = support::repo_root().join(VECTORS);
    let document: Json =
        serde_json::from_slice(&std::fs::read(path.as_std_path()).expect("the vectors"))
            .expect("the vectors are JSON");
    assert_eq!(document["axis"], "blake3");
    let vectors = document["vectors"].as_array().expect("vectors");
    let kinds: std::collections::BTreeSet<&str> =
        vectors.iter().map(|vector| text(&vector["kind"])).collect();
    for kind in [
        "leaf",
        "tree",
        "source",
        "semantic",
        "declaration",
        "entry-signature",
    ] {
        assert!(kinds.contains(kind), "{VECTORS} has a `{kind}` vector");
    }
    for vector in vectors {
        let stated_carrier = hex(text(&vector["carrier"]));
        let stated = text(&vector["kappa"]);
        // Both oracles give the stated label from the stated carrier.
        assert_eq!(
            support::oracle_kappa(&stated_carrier).label(),
            stated,
            "the blake3 crate"
        );
        assert_eq!(
            uor_addr(&stated_carrier),
            stated,
            "uor-addr's blake3 σ-axis"
        );
        // The compiler gives the stated label, and its carrier where it
        // exposes one.
        let (carrier, label) = recompute(vector);
        assert_eq!(
            label.as_str(),
            stated,
            "the compiler's {} label",
            text(&vector["kind"])
        );
        if let Some(carrier) = carrier {
            assert_eq!(
                carrier,
                stated_carrier,
                "the compiler's {} carrier",
                text(&vector["kind"])
            );
        }
        assert_eq!(
            Kappa::of(&stated_carrier).as_str(),
            stated,
            "the certified package"
        );
    }

    // The source recipe covers its project, lock, path, and source.
    let source = |inputs: &[&[u8]]| {
        content_id::source_id(
            inputs[0],
            inputs[1],
            &[(
                String::from_utf8(inputs[2].to_vec()).expect("a path"),
                inputs[3].to_vec(),
            )],
        )
        .label()
    };
    let fields: [&[u8]; 4] = [b"project", b"lock", b"src/Main.lex.tex", b"source"];
    assert!(
        covers_every_field(&source, &fields),
        "the source recipe covers every field"
    );
    // Planted: a source recipe that leaves the lock out.
    let without_lock = |inputs: &[&[u8]]| {
        let mut hasher = FramedHasher::new("lexlean-source-v2");
        hasher.frame("project", inputs[0]);
        hasher.frame("path", inputs[2]);
        hasher.frame("source", inputs[3]);
        hasher.finish().label()
    };
    assert!(
        !covers_every_field(&without_lock, &fields),
        "a recipe that leaves out a field it must cover is refused"
    );

    // Two different modules are two different sources.
    let framed_sources =
        |rows: &[(String, Vec<u8>)]| content_id::source_id(b"p", b"l", rows).label();
    assert!(
        separates_boundaries(&framed_sources),
        "framing separates a path from its source"
    );
    // Planted: the same rows written without their length prefixes.
    let unframed = |rows: &[(String, Vec<u8>)]| {
        let mut carrier = b"lexlean-source-v2\0".to_vec();
        for (path, bytes) in rows {
            carrier.extend_from_slice(path.as_bytes());
            carrier.extend_from_slice(bytes);
        }
        Kappa::of(&carrier).label()
    };
    assert!(
        !separates_boundaries(&unframed),
        "two modules whose unframed carriers coincide are refused"
    );

    // A changed lexicon entry changes its label: the signature is the leaf.
    let signature = |_entry: &str, signature: &str| Kappa::of(signature.as_bytes()).label();
    assert!(
        distinguishes_signatures(&signature),
        "an entry's label follows its signature"
    );
    // Planted: a label of the entry's name alone.
    let by_name = |entry: &str, _signature: &str| Kappa::of(entry.as_bytes()).label();
    assert!(
        !distinguishes_signatures(&by_name),
        "a changed lexicon entry whose label does not change is refused"
    );
}

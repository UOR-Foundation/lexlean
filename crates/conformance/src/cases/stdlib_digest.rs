//! The compiler's content identity is the UOR κ-label on the BLAKE3 σ-axis,
//! computed by the standard library's certified `Blake3` package (SPEC.md
//! §17.22, §21.1, `SL-06`), held through the compiler's own `Kappa::of`,
//! `FramedHasher`, and `tree_kappa` to the authority that establishes a BLAKE3
//! digest (the reference team's published test vectors) and to two oracles
//! that may falsify and never establish anything: the reference `blake3` crate
//! and uor-addr's BLAKE3 σ-axis, the derivation Hologram's store re-derives.
//! Every defect the statement lists is planted here, on a copy, and must be
//! found by the same check the real tree passes.

use lexlean::artifact::content_id::{tree_kappa, FramedHasher, Kappa};

use crate::support;

/// The committed export the compiler links (SPEC.md §17.22).
const EXPORT: &str = "tests/production-export/stdlib/packages/rust-std/blake3_label/src/lib.rs";
/// The committed copy of it the crate compiles (a copy, since the build
/// script compiles it too and cannot follow a link a host checks out as text).
const LINK: &str = "crates/lexlean/blake3_package.rs";
/// The reference team's test vectors, vendored at the release the oracles pin.
const VECTORS: &str = "tests/stdlib/blake3/test_vectors.json";

/// The hash crates a shipped crate must not depend on: the identity has one
/// implementation, the certified package, and no crate beside it.
const HASH_CRATES: [&str; 6] = [
    "sha2",
    "blake3",
    "uor-addr",
    "uor-prism",
    "uor-prism-crypto",
    "ring",
];

/// The longest message length every block and chunk boundary is covered up to.
const LENGTHS: usize = 3_100;

/// A deterministic byte stream (xorshift64*), so the corpus is the same on
/// every host.
fn stream(seed: u64, length: usize) -> Vec<u8> {
    let mut state = seed | 1;
    (0..length)
        .map(|_| {
            state ^= state >> 12;
            state ^= state << 25;
            state ^= state >> 27;
            (state.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 56) as u8
        })
        .collect()
}

/// The reference implementation's label.
fn oracle(message: &[u8]) -> String {
    support::oracle_kappa(message).label()
}

/// uor-addr's label on its BLAKE3 σ-axis: the fold Hologram's `address_bytes`
/// performs, written as uor-addr's typed κ-label accepts it.
fn uor_addr_label(message: &[u8]) -> String {
    let label = format!("blake3:{}", blake3::hash(message).to_hex());
    let typed = uor_addr::KappaLabel::<71>::from_bytes(label.as_bytes())
        .expect("uor-addr accepts the label");
    assert_eq!(typed.sigma_axis(), Some("blake3"), "the BLAKE3 σ-axis");
    typed.as_str().to_owned()
}

/// The names `lexlean` depends on, of every kind, read from the output of
/// `cargo metadata --no-deps` for the workspace: the resolved package names,
/// so a `[dependencies.sha2]` table or a renamed `package = "sha2"` counts as
/// the dependency it is.
pub(super) fn shipped_dependencies(metadata: &serde_json::Value) -> Vec<String> {
    let package = metadata["packages"]
        .as_array()
        .and_then(|packages| packages.iter().find(|package| package["name"] == "lexlean"))
        .expect("cargo metadata lists the lexlean package");
    package["dependencies"]
        .as_array()
        .expect("a package lists its dependencies")
        .iter()
        .map(|dependency| dependency["name"].as_str().expect("a name").to_owned())
        .collect()
}

/// The dependency check of the statement: `lexlean` has no hash crate.
pub(super) fn has_no_hash_crate(metadata: &serde_json::Value) -> Result<(), String> {
    let dependencies = shipped_dependencies(metadata);
    assert!(
        dependencies.len() > 5,
        "the shipped crate's dependency list is read ({dependencies:?})"
    );
    match dependencies
        .iter()
        .find(|name| HASH_CRATES.contains(&name.as_str()))
    {
        Some(name) => Err(format!("R6: the shipped crate depends on `{name}`")),
        None => Ok(()),
    }
}

/// `cargo metadata` of this workspace, offline and against the lock.
pub(super) fn workspace_metadata() -> serde_json::Value {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let output = std::process::Command::new(cargo)
        .args([
            "metadata",
            "--no-deps",
            "--offline",
            "--locked",
            "--format-version",
            "1",
        ])
        .current_dir(support::repo_root().as_std_path())
        .output()
        .expect("cargo metadata runs");
    assert!(
        output.status.success(),
        "cargo metadata: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("cargo metadata is JSON")
}

/// The link check: the in-crate link resolves to the export, and the bytes it
/// reads are the export's.
fn link_is_the_export(link: &[u8], export: &[u8]) -> Result<(), String> {
    if link == export {
        Ok(())
    } else {
        Err(format!(
            "the linked package ({} bytes) is not the committed export ({} bytes)",
            link.len(),
            export.len()
        ))
    }
}

/// The authority's cases: an input length, the input (byte `i` is `i` modulo
/// 251), and the stated hash-mode digest (the first 32 bytes of its output).
fn vectors() -> Vec<(Vec<u8>, String)> {
    let path = support::repo_root().join(VECTORS);
    let text = std::fs::read_to_string(path.as_std_path()).expect("the vendored vectors");
    let document: serde_json::Value = serde_json::from_str(&text).expect("the vectors are JSON");
    document["cases"]
        .as_array()
        .expect("the vectors list cases")
        .iter()
        .map(|case| {
            let length = usize::try_from(case["input_len"].as_u64().expect("a length"))
                .expect("a length fits");
            let input: Vec<u8> = (0..length).map(|index| (index % 251) as u8).collect();
            let hash = case["hash"].as_str().expect("a hash");
            (input, format!("blake3:{}", &hash[..64]))
        })
        .collect()
}

/// The first message on which `label` differs from the authority or the
/// oracle, if any: every authority case by its stated digest, every length to
/// `LENGTHS`, and the neighbours of every power of two to 4 MiB by the oracle.
fn first_disagreement(
    label: &dyn Fn(&[u8]) -> String,
    cases: &[(Vec<u8>, String)],
) -> Option<String> {
    for (message, stated) in cases {
        if &label(message) != stated {
            return Some(format!("BLAKE3 test vector of {} bytes", message.len()));
        }
    }
    for length in 0..=LENGTHS {
        let message = stream(length as u64 + 1, length);
        if label(&message) != oracle(&message) {
            return Some(format!("seeded message of {length} bytes"));
        }
    }
    for power in 0..=22u32 {
        for length in [(1usize << power) - 1, 1 << power, (1 << power) + 1] {
            let message = stream(u64::from(power) + 1000, length);
            if label(&message) != oracle(&message) {
                return Some(format!("seeded message of {length} bytes"));
            }
        }
    }
    None
}

/// The §21.1 frame encoding written independently of the compiler, with the
/// length prefixes' byte order chosen by `big_endian`; the oracle labels it.
fn recomputed_frames(domain: &str, frames: &[(&str, &[u8])], big_endian: bool) -> Kappa {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain.as_bytes());
    hasher.update(&[0u8]);
    for (label, bytes) in frames {
        let (label_length, body_length) = (label.len() as u32, bytes.len() as u64);
        if big_endian {
            hasher.update(&label_length.to_be_bytes());
            hasher.update(label.as_bytes());
            hasher.update(&body_length.to_be_bytes());
        } else {
            hasher.update(&label_length.to_le_bytes());
            hasher.update(label.as_bytes());
            hasher.update(&body_length.to_le_bytes());
        }
        hasher.update(bytes);
    }
    support::oracle_finish(&hasher)
}

/// Whether the compiler's framing agrees with an encoding that writes the
/// prefixes in the stated order, on frames that make a wrong order visible.
fn frames_agree(big_endian: bool) -> bool {
    let long = stream(7, 70_000);
    let frames: [(&str, &[u8]); 4] = [
        ("a", b""),
        ("project", b"name = \"x\"\n"),
        ("source", &long),
        ("path", b"src/Main.lex.tex"),
    ];
    let mut hasher = FramedHasher::new("lexlean-source-v2");
    for (label, bytes) in frames {
        hasher.frame(label, bytes);
    }
    hasher.finish() == recomputed_frames("lexlean-source-v2", &frames, big_endian)
}

/// The §11.5 tree encoding, independently.
fn recomputed_tree(files: &[(&str, &[u8])]) -> Kappa {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"lexlean-tree-v2\0");
    for (path, bytes) in files {
        hasher.update(&(path.len() as u32).to_be_bytes());
        hasher.update(path.as_bytes());
        hasher.update(&(bytes.len() as u64).to_be_bytes());
        hasher.update(bytes);
    }
    support::oracle_finish(&hasher)
}

/// A package copy with one bit of the first word of its initialization vector
/// flipped, built with `rustc` and run on the authority's first cases; its
/// labels, one per case.
fn plant_iv(cases: &[(Vec<u8>, String)]) -> Vec<String> {
    let root = support::repo_root();
    let export = std::fs::read_to_string(root.join(EXPORT).as_std_path()).expect("the export");
    assert!(export.contains("1779033703u32"), "IV[0] is in the package");
    let planted = export.replace("1779033703u32", "1779033702u32");
    let directory = tempfile::tempdir().expect("a temporary directory");
    std::fs::write(directory.path().join("package.rs"), planted).expect("the planted package");
    let lengths: Vec<String> = cases
        .iter()
        .map(|(message, _)| message.len().to_string())
        .collect();
    let harness = directory.path().join("main.rs");
    std::fs::write(
        &harness,
        format!(
            r#"#[path = "package.rs"]
#[allow(warnings)]
mod package;
fn main() {{
    for length in [{}usize] {{
        let message: Vec<u8> = (0..length).map(|index| (index % 251) as u8).collect();
        let label = package::run(package::Bytes::lit(&message)).expect("in domain");
        println!("{{}}", String::from_utf8(label.octets().to_vec()).expect("ascii"));
    }}
}}
"#,
            lengths.join("usize, ")
        ),
    )
    .expect("the harness");
    let binary = directory.path().join("planted");
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_owned());
    let built = std::process::Command::new(rustc)
        .args(["--edition", "2021", "-C", "opt-level=1", "-o"])
        .arg(&binary)
        .arg(&harness)
        .output()
        .expect("rustc runs");
    assert!(
        built.status.success(),
        "the planted package builds: {}",
        String::from_utf8_lossy(&built.stderr)
    );
    let ran = std::process::Command::new(&binary)
        .output()
        .expect("the planted harness runs");
    String::from_utf8(ran.stdout)
        .expect("labels")
        .lines()
        .map(str::to_owned)
        .collect()
}

/// Run the case for `SL-06`.
///
/// # Panics
///
/// Panics when the case's assertion fails.
#[allow(clippy::too_many_lines)]
pub(super) fn run(id: &str) {
    assert_eq!(id, "SL-06");
    let root = support::repo_root();

    // R6: the shipped crate's boundary is explicit, it has no second
    // implementation of the identity beside the package, and the package it
    // compiles is the committed export byte for byte (a diverging copy fails).
    let metadata = workspace_metadata();
    has_no_hash_crate(&metadata).unwrap_or_else(|reason| panic!("{reason}"));
    let export = std::fs::read(root.join(EXPORT).as_std_path()).expect("the committed export");
    let link = std::fs::read(root.join(LINK).as_std_path())
        .unwrap_or_else(|error| panic!("{LINK}: the copy of the export is missing: {error}"));
    link_is_the_export(&link, &export).unwrap_or_else(|reason| panic!("{reason}"));

    // The authority is a `some-true` row of the register that vendors the
    // vectors (R2): a file the register does not cite is not an authority.
    let authorities = std::fs::read_to_string(root.join("model/authorities.toml").as_std_path())
        .expect("the authorities");
    assert!(
        authorities.contains(VECTORS),
        "no authority row of model/authorities.toml vendors {VECTORS}"
    );

    // The authority and both oracles first: they must agree before any of
    // them judges the compiler.
    let cases = vectors();
    assert_eq!(cases.len(), 35, "the reference team's 35 cases");
    for (message, stated) in &cases {
        assert_eq!(&oracle(message), stated, "the vector and `blake3` agree");
        assert_eq!(
            &uor_addr_label(message),
            stated,
            "the vector and uor-addr agree"
        );
    }

    // The compiler's own κ-label against all of them.
    let compiler = |message: &[u8]| Kappa::of(message).label();
    if let Some(found) = first_disagreement(&compiler, &cases) {
        panic!("the compiler's κ-label disagrees with its authority or the oracle on a {found}");
    }
    for round in 0..40u64 {
        let message = stream(round + 5000, (round as usize + 1) * 104_729);
        assert_eq!(
            compiler(&message),
            uor_addr_label(&message),
            "the compiler's κ-label of {} seeded bytes is uor-addr's",
            message.len()
        );
    }

    // Frames and tree rows, against the encodings written out independently.
    assert!(frames_agree(true), "FramedHasher writes the §21.1 frames");
    let big = stream(11, 300_000);
    let rows: [(&str, &[u8]); 3] = [("a/b", b"one"), ("a/c", &big), ("z", b"")];
    assert_eq!(
        tree_kappa(&rows),
        recomputed_tree(&rows),
        "tree_kappa writes the §11.5 rows"
    );

    // Each planted defect is found by the check the tree just passed.
    for planted in [
        serde_json::json!({"name": "sha2", "kind": null}),
        serde_json::json!({"name": "blake3", "rename": "digest", "kind": "dev"}),
        serde_json::json!({"name": "uor-addr", "kind": null}),
    ] {
        let mut altered = metadata.clone();
        let lexlean = altered["packages"]
            .as_array_mut()
            .and_then(|packages| {
                packages
                    .iter_mut()
                    .find(|package| package["name"] == "lexlean")
            })
            .expect("lexlean");
        lexlean["dependencies"]
            .as_array_mut()
            .expect("dependencies")
            .push(planted);
        assert!(
            has_no_hash_crate(&altered).is_err(),
            "a hash crate dependency of any form or kind is found"
        );
    }
    let mut diverging = export.clone();
    let middle = diverging.len() / 2;
    diverging[middle] ^= 1;
    assert!(
        link_is_the_export(&diverging, &export).is_err(),
        "a copy that differs from the export by one byte is found"
    );
    let truncated = |message: &[u8]| Kappa::of(&message[..message.len().saturating_sub(1)]).label();
    assert!(
        first_disagreement(&truncated, &cases).is_some(),
        "a message truncated by one byte is found"
    );
    assert!(
        !frames_agree(false),
        "a frame length written little-endian is found"
    );
    let planted = plant_iv(&cases[..8]);
    assert_eq!(planted.len(), 8, "one label per case");
    for ((_, stated), label) in cases.iter().zip(&planted) {
        assert_ne!(
            label, stated,
            "a bit of the first word of the initialization vector is found on every case"
        );
    }
}

//! The hash audit (SPEC.md §21.1, `AR-15`): every identity LexLean computes,
//! names, or pins is a BLAKE3 κ-label, and the 256-bit SHA-2 digest appears
//! only where a format LexLean does not own requires it.
//!
//! The audit reads what computes or declares an identity: the shipped crate,
//! the repository's own tooling (`xtask` and this crate), the language data,
//! every schema, and the model files. A line that names that digest in any
//! spelling, and a schema pattern that admits a bare 64-digit digest, is
//! refused unless an [`ALLOWED`] row names its file and a text the line
//! contains, with the outside format that requires it. The conformance case of
//! `AR-15` plants each defect the statement lists in a copy of these texts and
//! observes the audit refuse it.

use std::path::Path;

/// What the audit reads, relative to the repository root.
pub const SCANNED: [&str; 11] = [
    "crates/lexlean/src",
    "crates/lexlean/build.rs",
    "crates/lexlean/Cargo.toml",
    "crates/model/src",
    "xtask/src",
    "language",
    "schemas",
    "api/schemas",
    "tooling/schemas",
    "model/authorities.toml",
    "model/ledger.toml",
];

/// The spellings of the 256-bit SHA-2 digest the audit looks for,
/// case-insensitively. Each is written in two halves so that this file, which
/// the audit reads, does not match itself; exempting the file would leave a
/// hole where such a digest parked in the gate would sit.
pub fn spellings() -> [String; 3] {
    [
        format!("{}{}", "sha", "256"),
        format!("{}{}", "sha-", "256"),
        format!("{}{}", "sha", "2::"),
    ]
}

/// A schema pattern that admits a bare digest, without the `blake3:` axis.
pub fn bare_digest_pattern() -> String {
    format!("\"^[0-9a-f]{{{}}}$\"", 64)
}

/// One sanctioned use of the digest: the file, a text its line contains, and
/// the outside format that requires the digest there.
pub struct Allowed {
    /// The file, relative to the repository root, with `/` separators.
    pub file: &'static str,
    /// A text the sanctioned line contains.
    pub contains: &'static str,
    /// The format LexLean does not own that requires the digest.
    pub reason: &'static str,
}

/// The sanctioned uses. A row is a decision a reviewer makes, which is why the
/// list is short and every row names the outside format.
pub const ALLOWED: [Allowed; 24] = [
    Allowed {
        file: "crates/model/src/release.rs",
        contains: "",
        reason: "release/checksums.txt and the release manifest are the checksum utility's format, and crates.io checks a crate by the digest it computes (RP-18)",
    },
    Allowed {
        file: "xtask/src/release_manifest.rs",
        contains: "",
        reason: "the release manifest and checksums are the checksum utility's format and crates.io's digest (RP-18)",
    },
    Allowed {
        file: "xtask/src/main.rs",
        contains: "MANIFEST.",
        reason: "the release manifest asset is the checksum utility's format (RP-18)",
    },
    Allowed {
        file: "xtask/src/main.rs",
        contains: "release_manifest::",
        reason: "the crate and manifest digests of the release identity are the ones crates.io and the checksum utility check (RP-18)",
    },
    Allowed {
        file: "xtask/src/main.rs",
        contains: "crate_",
        reason: "the crate digest of the release identity is the one crates.io checks (RP-18)",
    },
    Allowed {
        file: "xtask/src/main.rs",
        contains: "manifest_",
        reason: "the manifest digest of the release identity is the one the checksum utility checks (RP-18)",
    },
    Allowed {
        file: "xtask/src/main.rs",
        contains: ".crate SHA-",
        reason: "the packaged crate's digest is the one crates.io checks (RP-18)",
    },
    Allowed {
        file: "xtask/Cargo.toml",
        contains: "",
        reason: "the release tooling depends on the SHA-2 crate for the formats above",
    },
    Allowed {
        file: "crates/model/src/registry.rs",
        contains: "acquired",
        reason: "the authority row's acquired digest is the one an upstream publisher states for the bytes it releases",
    },
    Allowed {
        file: "crates/model/src/project.rs",
        contains: "acquired",
        reason: "the authority row's acquired digest is the one an upstream publisher states for the bytes it releases",
    },
    Allowed {
        file: "model/authorities.toml",
        contains: "NIST",
        reason: "an authority of the standard library's SHA-2 function (SPEC.md §17.22), which LexLean offers and never uses for an identity",
    },
    Allowed {
        file: "model/authorities.toml",
        contains: "FIPS",
        reason: "an authority of the standard library's SHA-2 function (SPEC.md §17.22), which LexLean offers and never uses for an identity",
    },
    Allowed {
        file: "model/authorities.toml",
        contains: "nist",
        reason: "an authority of the standard library's SHA-2 function (SPEC.md §17.22), which LexLean offers and never uses for an identity",
    },
    Allowed {
        file: "model/authorities.toml",
        contains: concat!("tests/stdlib/sha", "256/"),
        reason: "an authority of the standard library's SHA-2 function (SPEC.md §17.22), which LexLean offers and never uses for an identity",
    },
    Allowed {
        file: "model/ledger.toml",
        contains: "NIST",
        reason: "an authority of the standard library's SHA-2 function (SPEC.md §17.22), which LexLean offers and never uses for an identity",
    },
    Allowed {
        file: "model/ledger.toml",
        contains: "FIPS",
        reason: "an authority of the standard library's SHA-2 function (SPEC.md §17.22), which LexLean offers and never uses for an identity",
    },
    Allowed {
        file: "model/ledger.toml",
        contains: "nist",
        reason: "an authority of the standard library's SHA-2 function (SPEC.md §17.22), which LexLean offers and never uses for an identity",
    },
    Allowed {
        file: "model/ledger.toml",
        contains: concat!("tests/stdlib/sha", "256/"),
        reason: "an authority of the standard library's SHA-2 function (SPEC.md §17.22), which LexLean offers and never uses for an identity",
    },
    Allowed {
        file: "model/authorities.toml",
        contains: "published source archive",
        reason: "the digest the Rust project publishes for its source archive, which identifies the pinned rustc release",
    },
    Allowed {
        file: "crates/model/src/lib.rs",
        contains: "acquired_",
        reason: "the acquired digest is the one an upstream publisher states for the bytes it releases (Lean, rustc), bound to the vendored copy's κ-label",
    },
    Allowed {
        file: "model/authorities.toml",
        contains: "acquired_",
        reason: "the acquired digest is the one an upstream publisher states for the bytes it releases, bound to the κ-label of the vendored copy",
    },
    Allowed {
        file: "crates/model/src/lib.rs",
        contains: "as Acquired",
        reason: "the acquired digest is the one an upstream publisher states for the bytes it releases (Lean, rustc), bound to the vendored copy's κ-label",
    },
    Allowed {
        file: "model/authorities.toml",
        contains: "wasm-gemm-gnaf",
        reason: "the citation quotes the GNAF issuer, whose repository pins the draft by its own digest",
    },
    Allowed {
        file: "crates/model/src/isolation.rs",
        contains: "git_tree",
        reason: "the differential oracle's pin (OR-01) records the digest of an external checkout under validation; it is evidence about the oracle and never an input to an identity (SPEC.md §17.19)",
    },
];

/// The standard library's SHA-2 module is a function LexLean offers, never an
/// identity it computes: a mention of it by these exact names is the function.
/// Only these names are exempt.
pub const FUNCTION_NAMES: [&str; 4] = [
    "LexLeanStd.Sha256",
    "rust-std/sha256",
    "sha256_hex",
    "Sha256Hex",
];

/// One refused line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// The file, relative to the repository root.
    pub file: String,
    /// The 1-based line.
    pub line: usize,
    /// The line's text, trimmed.
    pub text: String,
}

/// Whether `line` of `file` names the digest outside the sanctioned uses.
#[must_use]
pub fn refused(file: &str, line: &str) -> bool {
    let lower = line.to_lowercase();
    let mut rest = lower.clone();
    for name in FUNCTION_NAMES {
        rest = rest.replace(&name.to_lowercase(), "");
    }
    let mentions = spellings()
        .iter()
        .any(|spelling| rest.contains(spelling.as_str()))
        || line.contains(&bare_digest_pattern());
    if !mentions {
        return false;
    }
    !ALLOWED
        .iter()
        .any(|allowed| allowed.file == file && line.contains(allowed.contains))
}

/// Scan `(file, text)` pairs; every refused line.
#[must_use]
pub fn scan_texts(texts: &[(String, String)]) -> Vec<Finding> {
    let mut out = Vec::new();
    for (file, text) in texts {
        for (index, line) in text.lines().enumerate() {
            if refused(file, line) {
                out.push(Finding {
                    file: file.clone(),
                    line: index + 1,
                    text: line.trim().chars().take(160).collect(),
                });
            }
        }
    }
    out
}

/// Every text file the audit reads, as `(relative path, text)`, sorted.
///
/// # Errors
///
/// A root that cannot be read.
pub fn texts(root: &Path) -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();
    for scanned in SCANNED {
        let start = root.join(scanned);
        if !start.exists() {
            return Err(format!("{scanned}: a scanned root is missing"));
        }
        for entry in walkdir::WalkDir::new(&start)
            .sort_by_file_name()
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
        {
            let Ok(text) = std::fs::read_to_string(entry.path()) else {
                continue;
            };
            let relative = entry
                .path()
                .strip_prefix(root)
                .map_err(|error| error.to_string())?
                .to_string_lossy()
                .replace('\\', "/");
            out.push((relative, text));
        }
    }
    Ok(out)
}

/// The audit over `texts`: the number of files read, or every refused line.
/// Armed: texts that hold no source of the shipped crate are refused, since a
/// scan of them could not have found anything.
///
/// # Errors
///
/// The refused lines, one per line of the message.
pub fn check(texts: &[(String, String)]) -> Result<usize, String> {
    if !texts
        .iter()
        .any(|(file, _)| file.starts_with("crates/lexlean/src/"))
    {
        return Err("AR-15: the hash audit read no source of the shipped crate".to_owned());
    }
    let findings = scan_texts(texts);
    if findings.is_empty() {
        Ok(texts.len())
    } else {
        Err(findings
            .iter()
            .map(|finding| {
                format!(
                    "AR-15: {}:{}: a SHA-2 digest outside the formats that require one: {}",
                    finding.file, finding.line, finding.text
                )
            })
            .collect::<Vec<_>>()
            .join("\n"))
    }
}

/// The audit over the repository.
///
/// # Errors
///
/// A root that cannot be read, or the refused lines.
pub fn scan(root: &Path) -> Result<usize, String> {
    check(&texts(root)?)
}

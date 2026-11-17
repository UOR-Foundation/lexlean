//! `cargo xtask semantics-records [--write]`: the digests LexLean's
//! language-1.2 semantics records of the Rust backend (SPEC.md §17.16,
//! `language/semantics-1.2.toml`): `rust_renderer`, the framed digest of the
//! renderer's sources, and `rust_runtime_core` and `rust_runtime_std`, the
//! κ-label of each profile's runtime text. The unit tests of the renderer and
//! the runtime fail when a record differs; this computes the records from the
//! tree, so a change to the backend or its runtime modules writes them rather
//! than having them copied from a test failure. `just relock-identity` runs it
//! first, since the records are part of the compiler-semantics ID.

use std::path::Path;

use lexlean::artifact::content_id::Kappa;
use lexlean::calculus::rust::{renderer_digest, runtime, RENDERER_FILES};

use crate::Fail;

const SEMANTICS: &str = "language/semantics-1.2.toml";

/// Checks, or with `write` rewrites, the three records.
pub fn semantics_records(root: &Path, write: bool) -> Result<(), Fail> {
    let renderer_dir = root.join("crates/lexlean/src/calculus/rust");
    let mut sources = Vec::new();
    for name in RENDERER_FILES {
        sources.push((name, std::fs::read(renderer_dir.join(name))?));
    }
    let files: Vec<(&str, &[u8])> = sources
        .iter()
        .map(|(name, bytes)| (*name, bytes.as_slice()))
        .collect();
    let core = runtime::core().map_err(|error| format!("the core runtime: {error}"))?;
    let std_text = runtime::std().map_err(|error| format!("the std runtime: {error}"))?;
    let records = [
        (
            "rust_runtime_core",
            Kappa::of(core.as_bytes()).label(),
        ),
        ("rust_renderer", renderer_digest(&files).label()),
        (
            "rust_runtime_std",
            Kappa::of(format!("{core}{std_text}").as_bytes()).label(),
        ),
    ];

    let path = root.join(SEMANTICS);
    let text = std::fs::read_to_string(&path)?;
    let mut lines: Vec<String> = text.lines().map(str::to_owned).collect();
    let mut stale = Vec::new();
    for (key, digest) in &records {
        let prefix = format!("{key} = ");
        let line = lines
            .iter_mut()
            .find(|line| line.starts_with(&prefix))
            .ok_or_else(|| format!("{SEMANTICS} records no `{key}`"))?;
        let wanted = format!("{prefix}\"{digest}\"");
        if *line != wanted {
            stale.push(format!("{key}: {} -> {digest}", &line[prefix.len()..]));
            *line = wanted;
        }
    }
    if stale.is_empty() {
        println!("semantics-records: {SEMANTICS} is current");
        return Ok(());
    }
    for change in &stale {
        println!("semantics-records: {change}");
    }
    if !write {
        return Err(format!(
            "{SEMANTICS} records a stale Rust backend digest; run `cargo xtask semantics-records --write`"
        )
        .into());
    }
    let mut out = lines.join("\n");
    if text.ends_with('\n') {
        out.push('\n');
    }
    std::fs::write(&path, out)?;
    println!("semantics-records: wrote {SEMANTICS}");
    Ok(())
}

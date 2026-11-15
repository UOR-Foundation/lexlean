//! `cargo xtask bump-version <X.Y.Z>`: the hand-held sites of the workspace
//! version (SPEC.md §2.3) moved together, for a release cut (§9.4, `cut.yml`).
//!
//! The version is fixed in the workspace manifest, repeated by the §2.3 block
//! of SPEC.md and the RP-01 identity case that holds the two equal, and named
//! by `## Unreleased` in CHANGELOG.md until a release takes it. A cut moved each
//! by hand; this moves all of them or none, and refuses when a site does not
//! read as it expects, so a site that drifted is named rather than skipped.
//!
//! A pull request records its change as a fragment, `changes/<slug>.md`, and
//! never edits CHANGELOG.md: every pull request editing the one `## Unreleased`
//! list made each landing conflict with every open pull request, and each
//! re-merge restarted the gate. The cut folds the entries already under
//! `## Unreleased`, then every fragment in file-name order, into `## X.Y.Z`
//! under a fresh, empty `## Unreleased`, and deletes the fragments.
//! What the version moves downstream (the lock's workspace rows, and every
//! build oracle's `compiler.version`) is written by `cargo update --workspace`
//! here and by the write recipes of `regenerate.yml` after it.

use std::path::Path;

use crate::Fail;

/// One rewrite: the file, the text it must hold `count` times at the old
/// version, and that text at the new one.
struct Site {
    path: &'static str,
    old: String,
    new: String,
    count: usize,
}

fn parse(version: &str) -> Option<(u64, u64, u64)> {
    let mut parts = version.split('.');
    let mut next = || -> Option<u64> {
        let part = parts.next()?;
        // SemVer: no leading zero, digits only.
        if part.is_empty() || (part.len() > 1 && part.starts_with('0')) {
            return None;
        }
        if !part.bytes().all(|byte| byte.is_ascii_digit()) {
            return None;
        }
        part.parse().ok()
    };
    let parsed = (next()?, next()?, next()?);
    parts.next().is_none().then_some(parsed)
}

fn sites(old: &str, new: &str) -> Vec<Site> {
    let site = |path, old: String, new: String, count| Site {
        path,
        old,
        new,
        count,
    };
    vec![
        // `[workspace.package]` and the three path dependencies of the workspace.
        site(
            "Cargo.toml",
            format!("version = \"{old}\""),
            format!("version = \"{new}\""),
            4,
        ),
        site(
            "SPEC.md",
            format!("[workspace.package]\nversion = \"{old}\"\n"),
            format!("[workspace.package]\nversion = \"{new}\"\n"),
            1,
        ),
        site(
            "crates/conformance/src/cases/repository.rs",
            format!("\"version = \\\"{old}\\\"\","),
            format!("\"version = \\\"{new}\\\"\","),
            1,
        ),
        site(
            "crates/conformance/src/cases/repository.rs",
            format!("stdout.starts_with(\"lexlean {old}\\n\")"),
            format!("stdout.starts_with(\"lexlean {new}\\n\")"),
            1,
        ),
        site(
            "crates/conformance/src/cases/repository.rs",
            format!("\"a {old} tree must refuse release"),
            format!("\"a {new} tree must refuse release"),
            1,
        ),
    ]
}

/// The directory of changelog fragments, one `<slug>.md` per change.
pub const FRAGMENTS: &str = "changes";
const CHANGELOG: &str = "CHANGELOG.md";
const UNRELEASED: &str = "\n## Unreleased\n";

/// Why a fragment cannot be folded: it must hold an entry, and a heading of
/// level one or two would open a section of CHANGELOG.md of its own.
fn fragment_fault(text: &str) -> Option<&'static str> {
    if text.trim().is_empty() {
        return Some("is empty");
    }
    let mut fenced = false;
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            fenced = !fenced;
        } else if !fenced
            && (line.starts_with("# ") || line.starts_with("## ") || line == "#" || line == "##")
        {
            return Some("holds a `#` or `##` heading");
        }
    }
    None
}

/// Every fragment under `changes/`, in file-name order, as (published path,
/// text). A missing directory has none. Anything but `*.md` (and `.gitkeep`)
/// is refused, so a misnamed fragment is not silently left out of a release.
pub fn fragments(root: &Path) -> Result<Vec<(String, String)>, Vec<String>> {
    let directory = root.join(FRAGMENTS);
    let entries = match std::fs::read_dir(&directory) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(vec![format!("{FRAGMENTS}/: {error}")]),
    };
    let mut names = Vec::new();
    let mut faults = Vec::new();
    for entry in entries {
        match entry {
            Ok(entry) => names.push(entry.file_name().to_string_lossy().into_owned()),
            Err(error) => faults.push(format!("{FRAGMENTS}/: {error}")),
        }
    }
    names.sort();
    let mut found = Vec::new();
    for name in names {
        let path = format!("{FRAGMENTS}/{name}");
        if name == ".gitkeep" {
            continue;
        }
        if !name.ends_with(".md") || !root.join(&path).is_file() {
            faults.push(format!("{path}: a fragment is a file `<slug>.md`"));
            continue;
        }
        match std::fs::read_to_string(root.join(&path)) {
            Ok(text) => match fragment_fault(&text) {
                Some(fault) => faults.push(format!("{path}: {fault}")),
                None => found.push((path, text)),
            },
            Err(error) => faults.push(format!("{path}: {error}")),
        }
    }
    if faults.is_empty() {
        Ok(found)
    } else {
        Err(faults)
    }
}

/// `validate-model`: every fragment folds, so a bad one fails its own pull
/// request rather than the cut.
pub fn check_fragments(root: &Path) -> Result<(), Fail> {
    match fragments(root) {
        Ok(found) => {
            println!(
                "changelog fragments: {} fold under the next release",
                found.len()
            );
            Ok(())
        }
        Err(faults) => Err(format!(
            "§9.4: the changelog fragments do not fold:\n  {}",
            faults.join("\n  ")
        )
        .into()),
    }
}

/// CHANGELOG.md with its unreleased entries, then each fragment's text in the
/// order given, under `## {version}`, below a fresh, empty `## Unreleased`.
fn fold(changelog: &str, version: &str, fragments: &[(String, String)]) -> Result<String, String> {
    let found = changelog.matches(UNRELEASED).count();
    if found != 1 {
        return Err(format!(
            "{CHANGELOG}: {UNRELEASED:?} occurs {found} times, not 1"
        ));
    }
    let (head, rest) = changelog.split_once(UNRELEASED).expect("counted once");
    let (entries, tail) = match rest.find("\n## ") {
        Some(end) => rest.split_at(end),
        None => (rest, ""),
    };
    let pieces: Vec<&str> = std::iter::once(entries.trim())
        .chain(fragments.iter().map(|(_, text)| text.trim()))
        .filter(|piece| !piece.is_empty())
        .collect();
    let mut out = format!("{head}{UNRELEASED}\n## {version}\n");
    if !pieces.is_empty() {
        out.push('\n');
        out.push_str(&pieces.join("\n\n"));
        out.push('\n');
    }
    out.push_str(tail);
    Ok(out)
}

/// Write `texts`, delete `removed`, then run `last`; if any step fails, put
/// back every file this touched (each of `texts`, each removed fragment, and
/// `also`), so the whole move is all or nothing.
fn commit(
    root: &Path,
    texts: &[(String, String)],
    removed: &[String],
    also: &[&str],
    last: impl FnOnce() -> Result<(), String>,
) -> Result<(), Fail> {
    let originals: Vec<(&str, Vec<u8>)> = texts
        .iter()
        .map(|(path, _)| path.as_str())
        .chain(removed.iter().map(String::as_str))
        .chain(also.iter().copied())
        .map(|path| std::fs::read(root.join(path)).map(|bytes| (path, bytes)))
        .collect::<Result<_, _>>()?;
    let restore = |cause: String| -> Fail {
        if !removed.is_empty() {
            if let Err(error) = std::fs::create_dir_all(root.join(FRAGMENTS)) {
                return format!("{cause}; restoring {FRAGMENTS}/ failed too: {error}").into();
            }
        }
        for (path, bytes) in &originals {
            if let Err(error) = std::fs::write(root.join(path), bytes) {
                return format!("{cause}; restoring {path} failed too: {error}").into();
            }
        }
        format!("{cause}; every file is restored").into()
    };
    for (path, text) in texts {
        if let Err(error) = std::fs::write(root.join(path), text) {
            return Err(restore(format!("writing {path}: {error}")));
        }
    }
    for path in removed {
        if let Err(error) = std::fs::remove_file(root.join(path)) {
            return Err(restore(format!("deleting {path}: {error}")));
        }
    }
    last().map_err(restore)
}

/// Move every site from the workspace version to `new`, then the lock's
/// workspace rows. Nothing is written unless every site reads as expected.
pub fn bump_version(root: &Path, arguments: &[String]) -> Result<(), Fail> {
    let [new] = arguments else {
        return Err("cargo xtask bump-version <X.Y.Z>".into());
    };
    let old = crate::workspace_version(root)?;
    let (Some(from), Some(to)) = (parse(&old), parse(new)) else {
        return Err(
            format!("§2.3: `{old}` to `{new}` is not a move between X.Y.Z versions").into(),
        );
    };
    if to <= from {
        return Err(format!("§2.3: `{new}` does not follow `{old}`").into());
    }
    let mut texts: Vec<(String, String)> = Vec::new();
    let mut unmet = Vec::new();
    for site in sites(&old, new) {
        let index = match texts.iter().position(|(path, _)| *path == site.path) {
            Some(index) => index,
            None => {
                texts.push((
                    site.path.to_owned(),
                    std::fs::read_to_string(root.join(site.path))?,
                ));
                texts.len() - 1
            }
        };
        let text = &mut texts[index].1;
        let found = text.matches(&site.old).count();
        if found == site.count {
            *text = text.replace(&site.old, &site.new);
        } else {
            unmet.push(format!(
                "{}: {:?} occurs {found} times, not {}",
                site.path, site.old, site.count
            ));
        }
    }
    let fragments = match fragments(root) {
        Ok(found) => found,
        Err(faults) => {
            unmet.extend(faults);
            Vec::new()
        }
    };
    match fold(
        &std::fs::read_to_string(root.join(CHANGELOG))?,
        new,
        &fragments,
    ) {
        Ok(text) => texts.push((CHANGELOG.to_owned(), text)),
        Err(fault) => unmet.push(fault),
    }
    if !unmet.is_empty() {
        return Err(format!(
            "bump-version {old} -> {new} wrote nothing; the sites do not read as expected:\n  {}",
            unmet.join("\n  ")
        )
        .into());
    }
    // The lock is moved last and may fail; on failure every file this wrote,
    // every fragment it deleted, and the lock are put back.
    let removed: Vec<String> = fragments.into_iter().map(|(path, _)| path).collect();
    commit(root, &texts, &removed, &["Cargo.lock"], || {
        let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
        match std::process::Command::new(cargo)
            .current_dir(root)
            .args(["update", "--workspace", "--offline"])
            .status()
        {
            Ok(status) if status.success() => Ok(()),
            Ok(status) => Err(format!(
                "cargo update --workspace --offline exited {status}"
            )),
            Err(error) => Err(format!(
                "cargo update --workspace --offline did not run: {error}"
            )),
        }
    })?;
    println!(
        "bump-version: {old} -> {new} in {} files and Cargo.lock; {} changelog fragments folded",
        texts.len(),
        removed.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{commit, fold, fragments, parse, CHANGELOG};

    const BEFORE: &str =
        "# Changelog\n\nPreamble.\n\n## Unreleased\n\n- Existing entry.\n\n## 0.4.0\n\n- Old.\n";
    const AFTER: &str = "# Changelog\n\nPreamble.\n\n## Unreleased\n\n## 0.4.2\n\n- Existing entry.\n\n- Alpha.\n  More of alpha.\n\n- Beta.\n\n## 0.4.0\n\n- Old.\n";

    fn tree() -> tempfile::TempDir {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(dir.path().join(CHANGELOG), BEFORE).expect("write");
        std::fs::create_dir(dir.path().join("changes")).expect("mkdir");
        // Written out of name order: the fold orders by name, not by creation.
        std::fs::write(dir.path().join("changes/b-beta.md"), "- Beta.\n").expect("write");
        std::fs::write(
            dir.path().join("changes/a-alpha.md"),
            "\n- Alpha.\n  More of alpha.\n\n",
        )
        .expect("write");
        dir
    }

    fn fold_tree(root: &std::path::Path, last: Result<(), String>) -> Result<(), crate::Fail> {
        let found = fragments(root).expect("fragments fold");
        let text = fold(
            &std::fs::read_to_string(root.join(CHANGELOG)).unwrap(),
            "0.4.2",
            &found,
        )
        .expect("fold");
        let removed: Vec<String> = found.into_iter().map(|(path, _)| path).collect();
        commit(root, &[(CHANGELOG.to_owned(), text)], &removed, &[], || {
            last
        })
    }

    #[test]
    fn the_cut_folds_unreleased_entries_then_fragments_by_name() {
        let dir = tree();
        fold_tree(dir.path(), Ok(())).expect("the fold lands");
        assert_eq!(
            std::fs::read_to_string(dir.path().join(CHANGELOG)).unwrap(),
            AFTER
        );
        assert_eq!(
            std::fs::read_dir(dir.path().join("changes"))
                .unwrap()
                .count(),
            0,
            "the fragments are gone"
        );
        // The next cut finds the empty `## Unreleased` and moves nothing into it.
        assert_eq!(
            fold(AFTER, "0.4.3", &[]).unwrap(),
            AFTER.replace("## Unreleased\n\n", "## Unreleased\n\n## 0.4.3\n\n")
        );
    }

    #[test]
    fn a_failed_cut_restores_the_changelog_and_every_fragment() {
        let dir = tree();
        let error = fold_tree(dir.path(), Err("lock".to_owned())).expect_err("the last step fails");
        assert!(
            error.to_string().contains("every file is restored"),
            "{error}"
        );
        assert_eq!(
            std::fs::read_to_string(dir.path().join(CHANGELOG)).unwrap(),
            BEFORE
        );
        assert_eq!(
            std::fs::read_to_string(dir.path().join("changes/b-beta.md")).unwrap(),
            "- Beta.\n"
        );
        assert!(dir.path().join("changes/a-alpha.md").is_file());
    }

    #[test]
    fn fragments_hold_an_entry_and_no_section_heading() {
        let dir = tempfile::tempdir().expect("tempdir");
        assert_eq!(
            fragments(dir.path()),
            Ok(Vec::new()),
            "no directory, no fragments"
        );
        let changes = dir.path().join("changes");
        std::fs::create_dir(&changes).unwrap();
        std::fs::write(changes.join(".gitkeep"), "").unwrap();
        std::fs::write(
            changes.join("fine.md"),
            "- Fine.\n\n```\n## inside a fence\n```\n### Sub\n",
        )
        .unwrap();
        assert_eq!(fragments(dir.path()).map(|found| found.len()), Ok(1));
        std::fs::write(changes.join("empty.md"), " \n\n").unwrap();
        std::fs::write(changes.join("heading.md"), "## 0.9.0\n- Smuggled.\n").unwrap();
        std::fs::write(changes.join("typo.txt"), "- Lost.\n").unwrap();
        let faults = fragments(dir.path()).expect_err("three faults");
        assert_eq!(faults.len(), 3, "{faults:?}");
        assert!(faults[0].starts_with("changes/empty.md"));
        assert!(faults[1].starts_with("changes/heading.md"));
        assert!(faults[2].starts_with("changes/typo.txt"));
        assert!(
            fold("# Changelog\n", "0.4.2", &[]).is_err(),
            "no `## Unreleased` refuses"
        );
    }

    #[test]
    fn versions_are_three_plain_numbers() {
        assert_eq!(parse("0.4.1"), Some((0, 4, 1)));
        assert_eq!(parse("1.0.0"), Some((1, 0, 0)));
        for refused in [
            "0.4",
            "0.4.1.2",
            "v0.4.1",
            "0.04.1",
            "0.4.1-rc.1",
            "0.4.x",
            "",
        ] {
            assert_eq!(parse(refused), None, "{refused}");
        }
        assert!(parse("0.5.0") > parse("0.4.9"));
    }
}

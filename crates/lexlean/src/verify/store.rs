//! The process store (SPEC.md §22.13): a verification process is a function
//! of its content identity, so a process whose identity an earlier run on
//! this host already established is not run again.
//!
//! The content identity κ of a Lean module is the digest of its staged text
//! and of the identities of the modules its import header names, by name: a
//! Merkle identity of everything a `lean` or `leanchecker` process of that
//! module can read beyond the pinned toolchain. A module the stage does not
//! hold (the toolchain's own `Init`, `Lean`, `Std`) is identified by its name,
//! which the toolchain digest fixes. The key of one process adds the tool,
//! the digest of its executable, and its normalized argument vector, so the
//! same module compiled, replayed, or compiled without output are three
//! processes.
//!
//! The store is operational, like the width (§22.11): it is selected only by
//! `LEXLEAN_VERIFY_STORE`, is no input of any stage, enters no ID, and
//! changes no published byte. Only a process that succeeded is stored, with
//! its normalized record and the bytes of the files it wrote; a stored entry
//! is used only when its record names this run's digest of the executable
//! and every stored file still has the digest it was stored with. Anything
//! else, including an unreadable entry, is a miss and the process runs.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Mutex, PoisonError};

use camino::{Utf8Path, Utf8PathBuf};

use crate::artifact::canonical_json::Json;
use crate::artifact::content_id::Sha256Digest;
use crate::config::Limits;
use crate::diagnostic::Diagnostic;
use crate::verify::child::{ChildRecord, ChildSpec, Normalizer};

/// The environment variable that names the store directory.
pub const STORE_VARIABLE: &str = "LEXLEAN_VERIFY_STORE";

const DOMAIN: &str = "lexlean/verify-store/1";

/// A process store directory.
#[derive(Debug, Clone)]
pub struct Store {
    root: PathBuf,
}

impl Store {
    /// The store the environment selects, if any: an unset or empty variable
    /// selects none.
    #[must_use]
    pub fn from_env() -> Option<Self> {
        let value = std::env::var_os(STORE_VARIABLE)?;
        if value.is_empty() {
            return None;
        }
        Some(Self {
            root: PathBuf::from(value),
        })
    }

    fn entry(&self, key: &Sha256Digest) -> PathBuf {
        let hex = key.to_hex();
        self.root.join(&hex[..2]).join(hex)
    }

    /// The stored record of the process `key`, after restoring the files it
    /// wrote to `outputs`, or `None` when the store does not hold it whole.
    fn restore(
        &self,
        key: &Sha256Digest,
        outputs: &[Utf8PathBuf],
        executable: Sha256Digest,
    ) -> Option<ChildRecord> {
        let entry = self.entry(key);
        let manifest = std::fs::read_to_string(entry.join("outputs.txt")).ok()?;
        let record = ChildRecord::from_json(
            &Json::parse(&std::fs::read(entry.join("record.json")).ok()?).ok()?,
        )
        .ok()?;
        if record.executable_sha256 != executable || record.exit_code != 0 {
            return None;
        }
        // One line per file written: the output it belongs to, the suffix
        // after that output's name (Lean writes `.olean` with companions), and
        // the file's digest.
        let mut files = Vec::new();
        for (position, line) in manifest.lines().enumerate() {
            let mut fields = line.splitn(3, ' ');
            let index: usize = fields.next()?.parse().ok()?;
            let digest = fields.next()?;
            let suffix = fields.next().unwrap_or("");
            let output = outputs.get(index)?;
            let stored = std::fs::read(entry.join(format!("{position}.out"))).ok()?;
            if Sha256Digest::of(&stored).to_hex() != digest {
                return None;
            }
            files.push((
                Utf8PathBuf::from(format!("{}{suffix}", output.with_extension(""))),
                stored,
            ));
        }
        if outputs.iter().enumerate().any(|(index, _)| {
            !manifest
                .lines()
                .any(|line| line.split(' ').next() == Some(&index.to_string()))
        }) {
            return None;
        }
        for (path, stored) in files {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent.as_std_path()).ok()?;
            }
            std::fs::write(path.as_std_path(), stored).ok()?;
        }
        // The entry was used: its time orders what a pruning keeps.
        let _ = std::fs::File::options()
            .append(true)
            .open(entry.join("outputs.txt"))
            .and_then(|file| file.set_modified(std::time::SystemTime::now()));
        Some(record)
    }

    /// Store a process that succeeded. A failure to store is dropped: the
    /// store never decides a result.
    fn save(&self, key: &Sha256Digest, outputs: &[Utf8PathBuf], record: &ChildRecord) {
        let entry = self.entry(key);
        let Some(parent) = entry.parent() else { return };
        if entry.is_dir() || std::fs::create_dir_all(parent).is_err() {
            return;
        }
        let Ok(scratch) = tempfile::Builder::new()
            .prefix(".entry-")
            .tempdir_in(parent)
        else {
            return;
        };
        let mut digests = String::new();
        let mut position = 0usize;
        for (index, output) in outputs.iter().enumerate() {
            // Lean writes the `.olean` with companions of the same stem
            // (`.olean.server`, `.olean.private`, `.ir`): every file of that
            // stem in the output's directory is what the process wrote.
            let (Some(directory), Some(stem)) = (output.parent(), output.file_stem()) else {
                return;
            };
            let name = format!("{stem}.");
            let name = name.as_str();
            let Ok(entries) = std::fs::read_dir(directory.as_std_path()) else {
                return;
            };
            let mut written: Vec<String> = entries
                .flatten()
                .filter_map(|entry| entry.file_name().into_string().ok())
                .filter(|file| file.starts_with(name))
                .collect();
            written.sort();
            for file in written {
                let Ok(bytes) = std::fs::read(directory.join(&file).as_std_path()) else {
                    return;
                };
                let suffix = &file[stem.len()..];
                digests.push_str(&format!(
                    "{index} {} {suffix}\n",
                    Sha256Digest::of(&bytes).to_hex()
                ));
                if std::fs::write(scratch.path().join(format!("{position}.out")), bytes).is_err() {
                    return;
                }
                position += 1;
            }
        }
        if std::fs::write(
            scratch.path().join("record.json"),
            record.to_json().to_file_bytes(),
        )
        .is_err()
            || std::fs::write(scratch.path().join("outputs.txt"), digests).is_err()
        {
            return;
        }
        let scratch = scratch.keep();
        if std::fs::rename(&scratch, &entry).is_err() {
            // Another run stored the same process first.
            let _ = std::fs::remove_dir_all(&scratch);
        }
    }
}

/// The key of one process: its tool, executable, normalized arguments, and
/// the content identity of the module it is about.
#[must_use]
pub fn key(
    tool: &str,
    executable: Sha256Digest,
    argv: &[String],
    identity: Sha256Digest,
) -> Sha256Digest {
    let mut bytes = Vec::new();
    for part in [DOMAIN, tool, &executable.to_hex(), &identity.to_hex()] {
        bytes.extend_from_slice(part.as_bytes());
        bytes.push(0);
    }
    for argument in argv {
        bytes.extend_from_slice(argument.as_bytes());
        bytes.push(0);
    }
    Sha256Digest::of(&bytes)
}

/// Run `process` unless the store holds the process `key`; a process that
/// succeeded silently is stored with the files `outputs` it wrote.
///
/// # Errors
///
/// What `process` returns.
pub fn stored(
    store: Option<&Store>,
    key: Option<Sha256Digest>,
    outputs: &[Utf8PathBuf],
    executable: Sha256Digest,
    process: impl FnOnce() -> Result<ChildRecord, Diagnostic>,
) -> Result<ChildRecord, Diagnostic> {
    let (Some(store), Some(key)) = (store, key) else {
        return process();
    };
    if let Some(record) = store.restore(&key, outputs, executable) {
        crate::verify::telemetry::reused(record.module.as_deref(), &record.tool);
        return Ok(record);
    }
    let record = process()?;
    if record.exit_code == 0 && outputs.iter().all(|output| output.as_std_path().is_file()) {
        store.save(&key, outputs, &record);
    }
    Ok(record)
}

/// The import header of a Lean source: the module each `import` command of
/// its header names, in order. The header is what precedes the first command
/// that is not an import: blank lines, line comments, the `module` and
/// `prelude` keywords, and imports with their `public`, `private`, `meta`, and
/// `all` modifiers (Lean's module system).
#[must_use]
pub fn imports(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    for line in text.lines().map(str::trim) {
        if line.is_empty() || line.starts_with("--") || line == "module" || line == "prelude" {
            continue;
        }
        let mut words = line.split_whitespace().peekable();
        while matches!(words.peek(), Some(&("public" | "private" | "meta"))) {
            words.next();
        }
        if words.next() != Some("import") {
            break;
        }
        if words.peek() == Some(&"all") {
            words.next();
        }
        found.extend(words.map(str::to_owned));
    }
    found
}

/// The store as one verification uses it: the store, and the staged source
/// roots its modules' identities are read from, in the order of the search
/// path the processes read their imports from.
pub struct Session {
    store: Store,
    roots: Vec<Utf8PathBuf>,
    known: Mutex<BTreeMap<String, Sha256Digest>>,
}

impl Session {
    /// The session of a verification whose sources are staged under `roots`.
    #[must_use]
    pub fn new(store: Store, roots: Vec<Utf8PathBuf>) -> Self {
        Self {
            store,
            roots,
            known: Mutex::new(BTreeMap::new()),
        }
    }

    /// The content identity of the staged module `name`: its text and the
    /// identities of its imports. A module no root holds is the toolchain's
    /// and is identified by its name.
    pub fn module(&self, name: &str) -> Sha256Digest {
        let mut open = Vec::new();
        self.visit(name, &mut open)
    }

    /// The content identity of the source at `path` and its imports.
    pub fn source(&self, path: &Utf8Path) -> Option<Sha256Digest> {
        let text = std::fs::read(path.as_std_path()).ok()?;
        let mut open = Vec::new();
        // Labelled by its file name: the staging directory it lies in is a
        // fresh one in every run.
        Some(self.of_text(path.file_name()?, &text, &mut open))
    }

    fn visit(&self, name: &str, open: &mut Vec<String>) -> Sha256Digest {
        if let Some(identity) = self
            .known
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get(name)
        {
            return *identity;
        }
        let relative = format!("{}.lean", name.replace('.', "/"));
        let Some(text) = self
            .roots
            .iter()
            .find_map(|root| std::fs::read(root.join(&relative).as_std_path()).ok())
        else {
            return Sha256Digest::of(format!("{DOMAIN}\0toolchain\0{name}").as_bytes());
        };
        if open.iter().any(|other| other == name) {
            return Sha256Digest::of(format!("{DOMAIN}\0cycle\0{name}").as_bytes());
        }
        open.push(name.to_owned());
        let identity = self.of_text(name, &text, open);
        open.pop();
        self.known
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(name.to_owned(), identity);
        identity
    }

    fn of_text(&self, label: &str, text: &[u8], open: &mut Vec<String>) -> Sha256Digest {
        let mut bytes = format!(
            "{DOMAIN}\0module\0{label}\0{}\0",
            Sha256Digest::of(text).to_hex()
        )
        .into_bytes();
        for import in imports(&String::from_utf8_lossy(text)) {
            let identity = self.visit(&import, open);
            bytes.extend_from_slice(format!("{import}\0{}\0", identity.to_hex()).as_bytes());
        }
        Sha256Digest::of(&bytes)
    }

    /// Run the process `spec` about the content `identity` unless the store
    /// holds it; `outputs` are the files it writes.
    ///
    /// # Errors
    ///
    /// What the process returns.
    pub fn run(
        &self,
        spec: &ChildSpec<'_>,
        limits: &Limits,
        normalizer: &Normalizer,
        identity: Sha256Digest,
        outputs: &[Utf8PathBuf],
    ) -> Result<ChildRecord, Diagnostic> {
        let argv: Vec<String> = spec
            .argv
            .iter()
            .map(|argument| normalizer.normalize_arg(argument))
            .collect();
        let key = key(spec.tool, spec.executable_sha256, &argv, identity);
        stored(
            Some(&self.store),
            Some(key),
            outputs,
            spec.executable_sha256,
            || crate::verify::child::run(spec, limits, normalizer),
        )
    }
}

/// Run `spec` through the session's store when there is one.
///
/// # Errors
///
/// What the process returns.
pub fn run(
    session: Option<&Session>,
    spec: &ChildSpec<'_>,
    limits: &Limits,
    normalizer: &Normalizer,
    identity: impl FnOnce(&Session) -> Option<Sha256Digest>,
    outputs: &[Utf8PathBuf],
) -> Result<ChildRecord, Diagnostic> {
    match session.and_then(|session| identity(session).map(|identity| (session, identity))) {
        Some((session, identity)) => session.run(spec, limits, normalizer, identity, outputs),
        None => crate::verify::child::run(spec, limits, normalizer),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_import_header_reads_the_module_system_forms() {
        let text = "module\n-- a comment\npublic import Init\nimport all A.B\nmeta import C\nimport D E\n\nset_option x 1\nimport F\n";
        assert_eq!(imports(text), ["Init", "A.B", "C", "D", "E"]);
        assert_eq!(
            imports("prelude\nimport Init.Core\ndef x := 1\n"),
            ["Init.Core"]
        );
    }

    #[test]
    fn an_identity_follows_every_module_an_import_reaches() {
        let root = tempfile::tempdir().expect("scratch");
        let base = Utf8PathBuf::from_path_buf(root.path().to_path_buf()).expect("utf-8");
        let write = |name: &str, text: &str| {
            let path = base.join(format!("{}.lean", name.replace('.', "/")));
            std::fs::create_dir_all(path.parent().expect("parent").as_std_path()).expect("dir");
            std::fs::write(path.as_std_path(), text).expect("write");
        };
        write("P.A", "module\npublic import Init\ndef a := 1\n");
        write("P.B", "module\npublic import P.A\ndef b := 2\n");
        write("P.C", "module\npublic import P.B\ndef c := 3\n");
        let before = Session::new(
            Store {
                root: PathBuf::new(),
            },
            vec![base.clone()],
        );
        let (a, c) = (before.module("P.A"), before.module("P.C"));
        write("P.A", "module\npublic import Init\ndef a := 4\n");
        let after = Session::new(
            Store {
                root: PathBuf::new(),
            },
            vec![base.clone()],
        );
        assert_ne!(after.module("P.A"), a);
        assert_ne!(
            after.module("P.C"),
            c,
            "a change two imports away moves the identity"
        );
        assert_eq!(after.module("Init"), before.module("Init"));
    }
}

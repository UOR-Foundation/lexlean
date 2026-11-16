//! The process store (SPEC.md §22.13): a verification process is a function
//! of its content identity, so a process whose identity an earlier run on
//! this host already established is not run again.
//!
//! The content identity κ of a Lean module is the digest of its staged text
//! and of the identities of the modules its import header names, by name: a
//! Merkle identity of everything a `lean` or `leanchecker` process of that
//! module can read beyond the pinned toolchain. A module the stage does not
//! hold is bound only when the toolchain's library holds it (the toolchain's
//! own `Init`, `Lean`, `Std`); it is then identified by its name, and the
//! toolchain by the digest of every file of it a process loads. A module
//! neither holds is unbound, and a process whose closure reaches it is not
//! answered by the store. The key of one process adds the tool, the digest of
//! its executable, its normalized argument vector, and its context: the
//! digest of the program it starts, the toolchain, the Lake configuration of
//! its working directory, and its normalized environment rows. The same
//! module compiled, replayed, or compiled without output are three
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
    toolchain: Utf8PathBuf,
    known: Mutex<BTreeMap<String, Option<Sha256Digest>>>,
    files: Mutex<BTreeMap<Utf8PathBuf, Option<Sha256Digest>>>,
}

/// The file names under a toolchain or a Lake build directory that a `lean`,
/// `lake`, or `leanchecker` process loads: the executables, the shared
/// libraries, and each module's `.olean` with its companions and `.ir`.
fn loaded(relative: &str) -> bool {
    let name = relative.rsplit('/').next().unwrap_or(relative);
    relative.starts_with("bin/")
        || name.ends_with(".olean")
        || name.ends_with(".olean.server")
        || name.ends_with(".olean.private")
        || name.ends_with(".ir")
        || name.ends_with(".so")
        || name.contains(".so.")
        || name.ends_with(".dylib")
        || name.ends_with(".dll")
}

/// The digest of every loaded file under `root`, by its path relative to
/// `root`, or `None` when the tree cannot be read whole.
///
/// The traversal follows symbolic links as a process reading the tree does,
/// so a library directory linked from elsewhere is bound like one inside it.
/// A link that does not resolve, or a directory reached a second time (a
/// link cycle or an alias), makes the tree unreadable: `None`, a miss.
fn tree(root: &Utf8Path) -> Option<Sha256Digest> {
    let mut files: Vec<String> = Vec::new();
    let mut seen: std::collections::BTreeSet<PathBuf> = std::collections::BTreeSet::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(directory) = stack.pop() {
        if !seen.insert(std::fs::canonicalize(directory.as_std_path()).ok()?) {
            return None;
        }
        for entry in std::fs::read_dir(directory.as_std_path()).ok()? {
            let entry = entry.ok()?;
            let path = Utf8PathBuf::from_path_buf(entry.path()).ok()?;
            let kind = std::fs::metadata(path.as_std_path()).ok()?.file_type();
            if kind.is_dir() {
                stack.push(path);
            } else {
                let relative = path.strip_prefix(root).ok()?.as_str().to_owned();
                if loaded(&relative) {
                    files.push(relative);
                }
            }
        }
    }
    files.sort();
    // The files are read on every core: the toolchain is the largest input a
    // key binds, and the store answers only once it is read.
    let width = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    let chunk = files.len().div_ceil(width).max(1);
    let digests: Vec<Option<Vec<Sha256Digest>>> = std::thread::scope(|scope| {
        files
            .chunks(chunk)
            .map(|part| {
                scope.spawn(move || {
                    part.iter()
                        .map(|relative| {
                            std::fs::read(root.join(relative).as_std_path())
                                .ok()
                                .map(|bytes| Sha256Digest::of(&bytes))
                        })
                        .collect::<Option<Vec<_>>>()
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|handle| handle.join().ok().flatten())
            .collect()
    });
    let mut bytes = format!("{DOMAIN}\0tree\0").into_bytes();
    let mut names = files.iter();
    for part in digests {
        for digest in part? {
            bytes.extend_from_slice(format!("{}\0{}\0", names.next()?, digest.to_hex()).as_bytes());
        }
    }
    Some(Sha256Digest::of(&bytes))
}

impl Session {
    /// The session of a verification whose sources are staged under `roots`.
    #[must_use]
    pub fn new(store: Store, roots: Vec<Utf8PathBuf>, toolchain: Utf8PathBuf) -> Self {
        Self {
            store,
            roots,
            toolchain,
            known: Mutex::new(BTreeMap::new()),
            files: Mutex::new(BTreeMap::new()),
        }
    }

    /// The digest of a file or a tree, read once in a session.
    fn digest_of(
        &self,
        path: &Utf8Path,
        read: impl FnOnce(&Utf8Path) -> Option<Sha256Digest>,
    ) -> Option<Sha256Digest> {
        if let Some(known) = self
            .files
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get(path)
        {
            return *known;
        }
        let digest = read(path);
        self.files
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(path.to_path_buf(), digest);
        digest
    }

    /// The context of the process `spec`, beyond its module's identity: the
    /// program it starts, the toolchain it loads, the Lake configuration and
    /// build directory of its working directory, and its normalized
    /// environment rows. `None` when one of them cannot be read whole, or
    /// when the process does not run under the toolchain's home.
    fn context(&self, spec: &ChildSpec<'_>, normalizer: &Normalizer) -> Option<Sha256Digest> {
        let crate::verify::child::ChildHome::Toolchain { toolchain_bin } = spec.home else {
            return None;
        };
        let program = self.digest_of(spec.program, |path| {
            std::fs::read(path.as_std_path())
                .ok()
                .map(|bytes| Sha256Digest::of(&bytes))
        })?;
        let toolchain = self.digest_of(&self.toolchain, tree)?;
        let mut bytes = format!(
            "{DOMAIN}\0context\0{}\0{}\0{}\0{}\0",
            program.to_hex(),
            toolchain.to_hex(),
            normalizer.normalize_arg(toolchain_bin.as_str()),
            normalizer.normalize_arg(spec.cwd.as_str()),
        )
        .into_bytes();
        // `lake env` reads the workspace's configuration and puts its build
        // directories on the search path.
        for name in [
            "lakefile.lean",
            "lakefile.toml",
            "lake-manifest.json",
            "lean-toolchain",
        ] {
            let path = spec.cwd.join(name);
            let row = match std::fs::read(path.as_std_path()) {
                Ok(text) => Sha256Digest::of(&text).to_hex(),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => "absent".to_owned(),
                Err(_) => return None,
            };
            bytes.extend_from_slice(format!("{name}\0{row}\0").as_bytes());
        }
        let lake = spec.cwd.join(".lake");
        let lake_row = if lake.as_std_path().exists() {
            self.digest_of(&lake, tree)?.to_hex()
        } else {
            "absent".to_owned()
        };
        bytes.extend_from_slice(format!(".lake\0{lake_row}\0").as_bytes());
        for (key, value) in &spec.extra_env {
            bytes.extend_from_slice(
                format!("{key}\0{}\0", normalizer.normalize_arg(value)).as_bytes(),
            );
        }
        Some(Sha256Digest::of(&bytes))
    }

    /// The content identity of the staged module `name`: its text and the
    /// identities of its imports. A module no root holds is bound when the
    /// toolchain's library holds it, and is then identified by its name;
    /// otherwise it, and everything that imports it, is unbound (`None`).
    pub fn module(&self, name: &str) -> Option<Sha256Digest> {
        let mut open = Vec::new();
        self.visit(name, &mut open)
    }

    /// The content identity of what a `leanchecker` replay of the prefix
    /// `name` reads: the module and every staged module inside its namespace,
    /// which the replay checks with it (§22.4).
    pub fn prefix(&self, name: &str) -> Option<Sha256Digest> {
        let mut inside: Vec<String> = Vec::new();
        let relative = name.replace('.', "/");
        for root in &self.roots {
            let mut stack = vec![root.join(&relative)];
            while let Some(directory) = stack.pop() {
                let Ok(entries) = std::fs::read_dir(directory.as_std_path()) else {
                    continue;
                };
                for entry in entries.flatten() {
                    let Ok(path) = Utf8PathBuf::from_path_buf(entry.path()) else {
                        continue;
                    };
                    if path.is_dir() {
                        stack.push(path);
                    } else if path.extension() == Some("lean") {
                        if let Ok(rest) = path.strip_prefix(root) {
                            inside.push(rest.with_extension("").as_str().replace('/', "."));
                        }
                    }
                }
            }
        }
        inside.sort();
        inside.dedup();
        let mut bytes = format!(
            "{DOMAIN}\0prefix\0{name}\0{}\0",
            self.module(name)?.to_hex()
        )
        .into_bytes();
        for module in inside {
            bytes.extend_from_slice(
                format!("{module}\0{}\0", self.module(&module)?.to_hex()).as_bytes(),
            );
        }
        Some(Sha256Digest::of(&bytes))
    }

    /// The content identity of the source at `path` and its imports.
    pub fn source(&self, path: &Utf8Path) -> Option<Sha256Digest> {
        let text = std::fs::read(path.as_std_path()).ok()?;
        let mut open = Vec::new();
        // Labelled by its file name: the staging directory it lies in is a
        // fresh one in every run.
        self.of_text(path.file_name()?, &text, &mut open)
    }

    fn visit(&self, name: &str, open: &mut Vec<String>) -> Option<Sha256Digest> {
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
            // The toolchain's own module: its bytes are bound by the
            // toolchain's digest in every key's context. Any other module
            // the stage does not hold is unbound.
            let library = self
                .toolchain
                .join("lib")
                .join("lean")
                .join(format!("{}.olean", name.replace('.', "/")));
            return library
                .as_std_path()
                .is_file()
                .then(|| Sha256Digest::of(format!("{DOMAIN}\0toolchain\0{name}").as_bytes()));
        };
        if open.iter().any(|other| other == name) {
            return Some(Sha256Digest::of(
                format!("{DOMAIN}\0cycle\0{name}").as_bytes(),
            ));
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

    fn of_text(&self, label: &str, text: &[u8], open: &mut Vec<String>) -> Option<Sha256Digest> {
        let mut bytes = format!(
            "{DOMAIN}\0module\0{label}\0{}\0",
            Sha256Digest::of(text).to_hex()
        )
        .into_bytes();
        for import in imports(&String::from_utf8_lossy(text)) {
            let identity = self.visit(&import, open)?;
            bytes.extend_from_slice(format!("{import}\0{}\0", identity.to_hex()).as_bytes());
        }
        Some(Sha256Digest::of(&bytes))
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
        // A context that cannot be read whole binds nothing: the process
        // runs and is not stored.
        let Some(context) = self.context(spec, normalizer) else {
            return crate::verify::child::run(spec, limits, normalizer);
        };
        let identity = Sha256Digest::of(
            format!(
                "{DOMAIN}\0bound\0{}\0{}\0",
                identity.to_hex(),
                context.to_hex()
            )
            .as_bytes(),
        );
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

    fn session(roots: Vec<Utf8PathBuf>, toolchain: &Utf8Path) -> Session {
        Session::new(
            Store {
                root: PathBuf::new(),
            },
            roots,
            toolchain.to_path_buf(),
        )
    }

    fn write(base: &Utf8Path, relative: &str, text: &str) {
        let path = base.join(relative);
        std::fs::create_dir_all(path.parent().expect("parent").as_std_path()).expect("dir");
        std::fs::write(path.as_std_path(), text).expect("write");
    }

    fn scratch() -> (tempfile::TempDir, Utf8PathBuf) {
        let root = tempfile::tempdir().expect("scratch");
        let base = Utf8PathBuf::from_path_buf(root.path().to_path_buf()).expect("utf-8");
        (root, base)
    }

    #[test]
    fn an_identity_follows_every_module_an_import_reaches() {
        let (_root, base) = scratch();
        let (src, toolchain) = (base.join("src"), base.join("toolchain"));
        write(&toolchain, "lib/lean/Init.olean", "init");
        write(&src, "P/A.lean", "module\npublic import Init\ndef a := 1\n");
        write(&src, "P/B.lean", "module\npublic import P.A\ndef b := 2\n");
        write(&src, "P/C.lean", "module\npublic import P.B\ndef c := 3\n");
        let before = session(vec![src.clone()], &toolchain);
        let (a, c) = (before.module("P.A"), before.module("P.C"));
        assert!(a.is_some() && c.is_some());
        write(&src, "P/A.lean", "module\npublic import Init\ndef a := 4\n");
        let after = session(vec![src.clone()], &toolchain);
        assert_ne!(after.module("P.A"), a);
        assert_ne!(
            after.module("P.C"),
            c,
            "a change two imports away moves the identity"
        );
        assert_eq!(after.module("Init"), before.module("Init"));
    }

    #[test]
    fn a_module_neither_the_stage_nor_the_toolchain_holds_is_unbound() {
        let (_root, base) = scratch();
        let (src, toolchain) = (base.join("src"), base.join("toolchain"));
        write(&toolchain, "lib/lean/Init.olean", "init");
        write(
            &src,
            "P/A.lean",
            "module\npublic import Elsewhere.Built\ndef a := 1\n",
        );
        write(&src, "P/B.lean", "module\npublic import P.A\ndef b := 2\n");
        let session = session(vec![src.clone()], &toolchain);
        assert_eq!(session.module("Elsewhere.Built"), None);
        assert_eq!(session.module("P.A"), None);
        assert_eq!(session.module("P.B"), None, "unbound through every import");
        assert!(session.module("Init").is_some());
    }

    #[cfg(unix)]
    #[test]
    fn a_linked_library_is_bound_and_a_link_cycle_is_unreadable() {
        let (_root, base) = scratch();
        let (toolchain, library) = (base.join("toolchain"), base.join("library"));
        write(&toolchain, "bin/lean", "lean");
        write(&library, "Init.olean", "init");
        std::fs::create_dir_all(toolchain.join("lib").as_std_path()).expect("dir");
        std::os::unix::fs::symlink(
            library.as_std_path(),
            toolchain.join("lib/lean").as_std_path(),
        )
        .expect("link");
        let session = session(Vec::new(), &toolchain);
        assert!(
            session.module("Init").is_some(),
            "the linked library holds Init"
        );
        let before = tree(&toolchain).expect("readable");
        write(&library, "Init.olean", "changed");
        assert_ne!(
            tree(&toolchain),
            Some(before),
            "a file under the linked library is bound"
        );
        std::os::unix::fs::symlink(toolchain.as_std_path(), library.join("back").as_std_path())
            .expect("cycle");
        assert_eq!(tree(&toolchain), None, "a link cycle is a miss");
        std::fs::remove_file(library.join("back").as_std_path()).expect("unlink");
        std::os::unix::fs::symlink(
            base.join("nowhere").as_std_path(),
            library.join("dangling").as_std_path(),
        )
        .expect("dangling");
        assert_eq!(
            tree(&toolchain),
            None,
            "a link that does not resolve is a miss"
        );
    }

    #[test]
    fn the_toolchain_digest_follows_every_file_a_process_loads() {
        let (_root, base) = scratch();
        write(&base, "bin/lean", "lean");
        write(&base, "lib/libleanshared.so", "shared");
        write(&base, "lib/lean/Init/Core.olean", "core");
        write(&base, "lib/lean/Init/Core.ir", "ir");
        write(&base, "lib/lean/Init/Core.ilean", "server index");
        let first = tree(&base).expect("readable");
        write(&base, "lib/lean/Init/Core.ilean", "another index");
        assert_eq!(
            tree(&base),
            Some(first),
            "a file no process loads is not bound"
        );
        for (relative, text) in [
            ("lib/lean/Init/Core.olean", "changed"),
            ("lib/lean/Init/Core.ir", "changed"),
            ("lib/libleanshared.so", "changed"),
            ("bin/lean", "changed"),
            ("lib/lean/Init/New.olean.private", "new"),
        ] {
            let before = tree(&base);
            write(&base, relative, text);
            assert_ne!(tree(&base), before, "{relative} is bound");
        }
    }
}

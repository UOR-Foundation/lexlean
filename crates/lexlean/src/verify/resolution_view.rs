//! Byte custody for a compiler-owned merged object directory.
//!
//! Lake resolves packages before an inherited LEAN_PATH, and Lean selects a
//! whole top-level namespace at the first matching directory. The execution
//! adapter must supply this one view consistently to every consumer. This
//! component takes a complete, independently established object inventory; it
//! does not discover imports or authorize a tool invocation.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};

use crate::artifact::content_id::Sha256Digest;

/// One immutable object or companion bound to its producing evidence.
#[derive(Clone, Debug)]
pub(crate) struct ObjectPin {
    pub root: PathBuf,
    pub relative: PathBuf,
    pub bytes: u64,
    pub sha256: Sha256Digest,
}

/// Imports are copied, never linked or written back to the source library.
/// The owning verifier must record these pins in its evidence and call `check`
/// around each consumer; successful custody alone is not proof acceptance.
pub(crate) struct ResolutionView {
    root: PathBuf,
    originals: Vec<ObjectPin>,
    copies: Vec<ObjectPin>,
    reserved: BTreeSet<PathBuf>,
}

fn refused(message: impl std::fmt::Display) -> String {
    format!("resolution view: {message}")
}

fn relative_object(path: &Path) -> Result<(), String> {
    if path.as_os_str().is_empty()
        || path
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
    {
        return Err(refused("object path is not a confined relative path"));
    }
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| refused("object path is not UTF-8"))?;
    if ![".olean", ".olean.private", ".olean.server", ".ir"]
        .iter()
        .any(|suffix| name.ends_with(suffix))
    {
        return Err(refused("unrecognized object companion"));
    }
    Ok(())
}

fn confined(root: &Path, relative: &Path) -> Result<PathBuf, String> {
    relative_object(relative)?;
    let canonical = std::fs::canonicalize(root).map_err(refused)?;
    if canonical != root || !root.is_dir() {
        return Err(refused("object root is not a canonical real directory"));
    }
    let mut path = root.to_owned();
    for part in relative.components() {
        path.push(part.as_os_str());
        let metadata = std::fs::symlink_metadata(&path).map_err(refused)?;
        if metadata.file_type().is_symlink() {
            return Err(refused("object path contains a symlink"));
        }
    }
    if !std::fs::metadata(&path).map_err(refused)?.is_file() {
        return Err(refused("object is not a regular file"));
    }
    Ok(path)
}

fn capture(pin: &ObjectPin, remaining: u64) -> Result<Vec<u8>, String> {
    if pin.bytes > remaining {
        return Err(refused("object byte budget exceeded"));
    }
    let path = confined(&pin.root, &pin.relative)?;
    let mut file = std::fs::File::open(&path).map_err(refused)?;
    let opened = file.metadata().map_err(refused)?;
    if !opened.is_file() || opened.len() != pin.bytes {
        return Err(refused("object size differs from producing evidence"));
    }
    // The read itself is bounded, even if a file grows after metadata is read.
    use std::io::Read;
    let mut data = Vec::new();
    (&mut file)
        .take(
            pin.bytes
                .checked_add(1)
                .ok_or_else(|| refused("size overflow"))?,
        )
        .read_to_end(&mut data)
        .map_err(refused)?;
    if data.len() as u64 != pin.bytes || Sha256Digest::of(&data) != pin.sha256 {
        return Err(refused("object bytes differ from producing evidence"));
    }
    confined(&pin.root, &pin.relative)?;
    Ok(data)
}

impl ResolutionView {
    /// `reserved` contains every generated physical object's path, including
    /// its companions. A duplicate import is refused even if its bytes match.
    /// `byte_budget` is supplied by the owning phase's admitted storage budget.
    pub(crate) fn create(
        root: &Path,
        imports: &[ObjectPin],
        reserved: &BTreeSet<PathBuf>,
        byte_budget: u64,
    ) -> Result<Self, String> {
        let parent = root.parent().ok_or_else(|| refused("view has no parent"))?;
        if std::fs::canonicalize(parent).map_err(refused)? != parent {
            return Err(refused("view parent is not canonical"));
        }
        let mut paths = BTreeSet::new();
        let mut total = 0_u64;
        for pin in imports {
            relative_object(&pin.relative)?;
            if reserved.contains(&pin.relative) || !paths.insert(pin.relative.clone()) {
                return Err(refused("duplicate full object path"));
            }
            total = total
                .checked_add(pin.bytes)
                .ok_or_else(|| refused("size overflow"))?;
            if total > byte_budget {
                return Err(refused("view byte budget exceeded"));
            }
        }
        for path in reserved {
            relative_object(path)?;
        }
        // Never merge into an existing directory, even an empty one. A failed
        // creation leaves only diagnostic scratch owned by this attempt.
        std::fs::create_dir(root).map_err(refused)?;
        let mut copies = Vec::new();
        for original in imports {
            let data = capture(original, byte_budget)?;
            let target = root.join(&original.relative);
            std::fs::create_dir_all(target.parent().ok_or_else(|| refused("no parent"))?)
                .map_err(refused)?;
            use std::io::Write;
            let mut out = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&target)
                .map_err(refused)?;
            out.write_all(&data)
                .and_then(|()| out.sync_all())
                .map_err(refused)?;
            copies.push(ObjectPin {
                root: root.to_owned(),
                ..original.clone()
            });
        }
        let view = Self {
            root: root.to_owned(),
            originals: imports.to_vec(),
            copies,
            reserved: reserved.clone(),
        };
        view.check(byte_budget)?;
        Ok(view)
    }

    /// Admit exactly one successful compiler output and its companion bytes.
    /// The caller must bind the producer record digest separately in
    /// canonical verification evidence. Imports can never be replaced.
    pub(crate) fn register_generated(
        &mut self,
        pin: ObjectPin,
        byte_budget: u64,
    ) -> Result<(), String> {
        // Refuse a substituted root or intermediate directory before any write.
        self.check(byte_budget)?;
        relative_object(&pin.relative)?;
        if !self.reserved.contains(&pin.relative)
            || self.copies.iter().any(|old| old.relative == pin.relative)
        {
            return Err(refused("unreserved or duplicate generated object"));
        }
        let used = self
            .copies
            .iter()
            .try_fold(0_u64, |n, p| n.checked_add(p.bytes))
            .ok_or_else(|| refused("size overflow"))?;
        let data = capture(
            &pin,
            byte_budget
                .checked_sub(used)
                .ok_or_else(|| refused("budget"))?,
        )?;
        let path = self.root.join(&pin.relative);
        std::fs::create_dir_all(path.parent().ok_or_else(|| refused("no parent"))?)
            .map_err(refused)?;
        use std::io::Write;
        let mut out = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map_err(refused)?;
        out.write_all(&data)
            .and_then(|()| out.sync_all())
            .map_err(refused)?;
        self.copies.push(ObjectPin {
            root: self.root.clone(),
            ..pin.clone()
        });
        self.originals.push(pin);
        self.check(byte_budget)
    }

    pub(crate) fn reserve(&mut self, relative: PathBuf) -> Result<(), String> {
        relative_object(&relative)?;
        if self.copies.iter().any(|old| old.relative == relative) {
            return Err(refused("generated object collides with an existing object"));
        }
        self.reserved.insert(relative);
        Ok(())
    }

    pub(crate) fn inventory(&self) -> &[ObjectPin] {
        &self.originals
    }

    pub(crate) fn check(&self, byte_budget: u64) -> Result<(), String> {
        if std::fs::canonicalize(&self.root).map_err(refused)? != self.root
            || !std::fs::symlink_metadata(&self.root)
                .map_err(refused)?
                .is_dir()
        {
            return Err(refused("view root is not a canonical real directory"));
        }
        let mut expected = BTreeMap::new();
        let mut remaining = byte_budget;
        for (source, copy) in self.originals.iter().zip(&self.copies) {
            capture(source, remaining)?;
            capture(copy, remaining)?;
            remaining = remaining
                .checked_sub(source.bytes)
                .ok_or_else(|| refused("budget"))?;
            expected.insert(copy.relative.clone(), ());
        }
        fn visit(root: &Path, dir: &Path, found: &mut BTreeSet<PathBuf>) -> Result<(), String> {
            for entry in std::fs::read_dir(dir).map_err(refused)? {
                let entry = entry.map_err(refused)?;
                let kind = entry.file_type().map_err(refused)?;
                if kind.is_dir() {
                    visit(root, &entry.path(), found)?;
                } else if kind.is_file() {
                    found.insert(entry.path().strip_prefix(root).map_err(refused)?.to_owned());
                } else {
                    return Err(refused("view contains a symlink or special file"));
                }
            }
            Ok(())
        }
        let mut found = BTreeSet::new();
        visit(&self.root, &self.root, &mut found)?;
        if found != expected.into_keys().collect() {
            return Err(refused("view object inventory changed"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            use std::sync::atomic::{AtomicUsize, Ordering};
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let path = std::env::temp_dir().join(format!(
                "lexlean-view-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir(&path).unwrap();
            Self(std::fs::canonicalize(path).unwrap())
        }
        fn object(&self, name: &str, data: &[u8]) -> ObjectPin {
            let relative = PathBuf::from(name);
            std::fs::create_dir_all(self.0.join(&relative).parent().unwrap()).unwrap();
            std::fs::write(self.0.join(&relative), data).unwrap();
            ObjectPin {
                root: self.0.clone(),
                relative,
                bytes: data.len() as u64,
                sha256: Sha256Digest::of(data),
            }
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn copies_imports_without_changing_originals() {
        let source = Fixture::new();
        let output = Fixture::new();
        let pin = source.object("Rule30UOR/Base.olean", b"base");
        let reserved = [PathBuf::from("Rule30UOR/Generated.olean")]
            .into_iter()
            .collect();
        let view =
            ResolutionView::create(&output.0.join("view"), &[pin.clone()], &reserved, 4).unwrap();
        view.check(4).unwrap();
        assert_eq!(
            std::fs::read(source.0.join(&pin.relative)).unwrap(),
            b"base"
        );
    }
    #[test]
    fn missing_corrupt_and_duplicate_objects_refuse() {
        let source = Fixture::new();
        let output = Fixture::new();
        let pin = source.object("Rule30UOR/Base.olean", b"base");
        assert!(ResolutionView::create(
            &output.0.join("duplicate"),
            &[pin.clone(), pin.clone()],
            &BTreeSet::new(),
            100
        )
        .is_err());
        let reserved = [pin.relative.clone()].into_iter().collect();
        assert!(ResolutionView::create(
            &output.0.join("collision"),
            &[pin.clone()],
            &reserved,
            100
        )
        .is_err());
        std::fs::write(source.0.join(&pin.relative), b"evil").unwrap();
        assert!(ResolutionView::create(
            &output.0.join("corrupt"),
            &[pin.clone()],
            &BTreeSet::new(),
            100
        )
        .is_err());
        std::fs::remove_file(source.0.join(&pin.relative)).unwrap();
        assert!(
            ResolutionView::create(&output.0.join("missing"), &[pin], &BTreeSet::new(), 100)
                .is_err()
        );
    }
    #[test]
    fn changed_companion_and_extra_view_file_refuse() {
        let source = Fixture::new();
        let output = Fixture::new();
        let pins = [
            source.object("Rule30UOR/Base.olean", b"base"),
            source.object("Rule30UOR/Base.olean.private", b"private"),
        ];
        let view =
            ResolutionView::create(&output.0.join("view"), &pins, &BTreeSet::new(), 100).unwrap();
        std::fs::write(view.root.join(&pins[1].relative), b"changed").unwrap();
        assert!(view.check(100).is_err());
        std::fs::write(view.root.join(&pins[1].relative), b"private").unwrap();
        std::fs::write(view.root.join("extra.olean"), b"extra").unwrap();
        assert!(view.check(100).is_err());
    }
    #[test]
    fn traversal_and_over_budget_refuse() {
        let source = Fixture::new();
        let output = Fixture::new();
        let pin = source.object("Rule30UOR/Base.olean", b"base");
        assert!(ResolutionView::create(
            &output.0.join("budget"),
            &[pin.clone()],
            &BTreeSet::new(),
            3
        )
        .is_err());
        let bad = ObjectPin {
            relative: PathBuf::from("../Base.olean"),
            ..pin
        };
        assert!(
            ResolutionView::create(&output.0.join("escape"), &[bad], &BTreeSet::new(), 100)
                .is_err()
        );
    }
    #[cfg(unix)]
    #[test]
    fn symlink_input_refuses() {
        let source = Fixture::new();
        let output = Fixture::new();
        let pin = source.object("Rule30UOR/Base.olean", b"base");
        std::fs::remove_file(source.0.join(&pin.relative)).unwrap();
        let external = source.object("other.olean", b"base");
        std::os::unix::fs::symlink(
            source.0.join(external.relative),
            source.0.join(&pin.relative),
        )
        .unwrap();
        assert!(
            ResolutionView::create(&output.0.join("view"), &[pin], &BTreeSet::new(), 100).is_err()
        );
    }
    #[cfg(unix)]
    #[test]
    fn empty_view_cannot_be_replaced_by_a_symlink() {
        let output = Fixture::new();
        let outside = Fixture::new();
        let view =
            ResolutionView::create(&output.0.join("view"), &[], &BTreeSet::new(), 0).unwrap();
        std::fs::remove_dir(&view.root).unwrap();
        std::os::unix::fs::symlink(&outside.0, &view.root).unwrap();
        assert!(view.check(0).is_err());
    }
    #[test]
    fn generated_and_imported_objects_share_one_checked_root() {
        let imports = Fixture::new();
        let generated = Fixture::new();
        let output = Fixture::new();
        let base = imports.object("Rule30UOR/Base.olean", b"base");
        let main = generated.object("Rule30UOR/Generated.olean", b"generated");
        let companion = generated.object("Rule30UOR/Generated.olean.private", b"companion");
        let reserved = [main.relative.clone(), companion.relative.clone()]
            .into_iter()
            .collect();
        let mut view =
            ResolutionView::create(&output.0.join("view"), &[base], &reserved, 100).unwrap();
        view.register_generated(main.clone(), 100).unwrap();
        view.register_generated(companion.clone(), 100).unwrap();
        view.check(100).unwrap();
        assert_eq!(view.inventory().len(), 3);
        assert!(view.register_generated(main, 100).is_err());
        std::fs::write(generated.0.join(companion.relative), b"tampered!").unwrap();
        assert!(view.check(100).is_err());
    }

    #[test]
    fn generated_registration_cannot_replace_import_or_escape_inventory() {
        let imports = Fixture::new();
        let generated = Fixture::new();
        let output = Fixture::new();
        let base = imports.object("Rule30UOR/Base.olean", b"base");
        let shadow = generated.object("Rule30UOR/Base.olean", b"evil");
        let main = generated.object("Rule30UOR/Generated.olean", b"generated");
        let mut view =
            ResolutionView::create(&output.0.join("view"), &[base], &BTreeSet::new(), 100).unwrap();
        assert!(view.reserve(shadow.relative.clone()).is_err());
        assert!(view.register_generated(shadow, 100).is_err());
        assert!(view.register_generated(main.clone(), 100).is_err());
        view.reserve(main.relative.clone()).unwrap();
        assert!(view.register_generated(main.clone(), 4).is_err());
        view.register_generated(main, 100).unwrap();
        view.check(100).unwrap();
    }
    #[cfg(unix)]
    #[test]
    fn generated_registration_refuses_escape_before_writing() {
        let generated = Fixture::new();
        let output = Fixture::new();
        let outside = Fixture::new();
        let pin = generated.object("Rule30UOR/Generated.olean", b"generated");
        let reserved = [pin.relative.clone()].into_iter().collect();
        let mut view = ResolutionView::create(&output.0.join("view"), &[], &reserved, 100).unwrap();
        std::os::unix::fs::symlink(&outside.0, view.root.join("Rule30UOR")).unwrap();
        assert!(view.register_generated(pin.clone(), 100).is_err());
        assert_eq!(std::fs::read_dir(&outside.0).unwrap().count(), 0);
        std::fs::remove_file(view.root.join("Rule30UOR")).unwrap();
        std::fs::remove_dir(&view.root).unwrap();
        std::os::unix::fs::symlink(&outside.0, &view.root).unwrap();
        assert!(view.register_generated(pin, 100).is_err());
        assert_eq!(std::fs::read_dir(&outside.0).unwrap().count(), 0);
    }
}

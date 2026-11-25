"""Source-bound fault injections for transferred-export environment custody (#489).

These are deterministic safety placement checks, not an executed Lean replay.
The existing Rust integration gate owns runtime qualification.
"""
from pathlib import Path
import unittest


SOURCE = (Path(__file__).resolve().parents[2] /
          "crates/conformance/src/export_custody.rs")
HEAD = "pub fn take(directory: &Utf8Path) -> Result<Exported, String> {"
GUARD = "let _environment = support::env_shared();"


def assert_guarded_take(source):
    if source.count(HEAD) != 1:
        raise ValueError("transferred export function changed")
    block = source.split(HEAD, 1)[1]
    if block.count(GUARD) != 1:
        raise ValueError("transferred export must acquire one shared environment guard")
    for label, item in (
        ("wait completion", "let artifact = directory.join(ARTIFACT_DIR);"),
        ("content digest", "let digest = bundle_digest(&source);"),
        ("toolchain identity", "check(&custody, &identity(&root)?, &digest)?;"),
        ("fresh project", "let project = P::bootstrap();"),
        ("replay", "production::verify_artifact(&bundle)"),
        ("return", "Ok(Exported {"),
    ):
        if block.count(item) != 1:
            raise ValueError("transferred export contract missing: " + label)
    # Explicit use of this RAII binding can release or move it before replay.
    # This is a source check, not a proof of arbitrary Rust lock lifetimes.
    if block.count("_environment") != 1:
        raise ValueError("shared environment guard must remain bound through replay")
    guard = block.index(GUARD)
    if not (block.index("let artifact = directory.join(ARTIFACT_DIR);") <
            block.index("let digest = bundle_digest(&source);") <
            guard <
            block.index("check(&custody, &identity(&root)?, &digest)?;") <
            block.index("let project = P::bootstrap();") <
            block.index("production::verify_artifact(&bundle)") <
            block.index("Ok(Exported {")):
        raise ValueError("environment guard must cover exact-tree identity and replay, not wait")
    return True


class EnvironmentCustodyContract(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        if not SOURCE.is_file():
            raise unittest.SkipTest("full repository checkout unavailable")
        cls.source = SOURCE.read_text(encoding="utf-8")

    def test_real_transfer_source_is_guarded(self):
        self.assertTrue(assert_guarded_take(self.source))

    def test_missing_guard_fails(self):
        source = self.source.replace(GUARD, "", 1)
        with self.assertRaisesRegex(ValueError, "shared environment guard"):
            assert_guarded_take(source)

    def test_guard_before_wait_fails(self):
        source = self.source.replace(
            "let artifact = directory.join(ARTIFACT_DIR);",
            GUARD + "\n    let artifact = directory.join(ARTIFACT_DIR);", 1,
        )
        source = source.replace(
            "    " + GUARD + "\n    let root = support::repo_root();",
            "    let root = support::repo_root();", 1,
        )
        with self.assertRaisesRegex(ValueError, "not wait"):
            assert_guarded_take(source)

    def test_guard_after_identity_fails(self):
        source = self.source.replace(
            "    " + GUARD + "\n    let root = support::repo_root();",
            "    let root = support::repo_root();", 1,
        ).replace(
            "    let project = P::bootstrap();",
            "    " + GUARD + "\n    let project = P::bootstrap();", 1,
        )
        with self.assertRaisesRegex(ValueError, "identity and replay"):
            assert_guarded_take(source)

    def test_early_drop_is_refused(self):
        source = self.source.replace(
            "    " + GUARD + "\n    let root = support::repo_root();",
            "    " + GUARD + "\n    let root = support::repo_root();\n    drop(_environment);",
            1,
        )
        self.assertNotEqual(source, self.source)
        with self.assertRaisesRegex(ValueError, "remain bound"):
            assert_guarded_take(source)

    def test_exclusive_lock_is_refused(self):
        mutated = self.source.replace(GUARD,
                                      "let _environment = support::env_lock();", 1)
        with self.assertRaisesRegex(ValueError, "shared environment guard"):
            assert_guarded_take(mutated)


if __name__ == "__main__":
    unittest.main()

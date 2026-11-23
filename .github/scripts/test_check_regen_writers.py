"""Real Git patch and manifest falsifiers for regenerated writer ownership (#489)."""
import os
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path

from check_regen_writers import WRITERS, inspect, patch_names


@unittest.skipUnless(shutil.which("git"), "requires a local Git executable")
class WriterCollisionTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.repo = self.root / "repo"
        self.repo.mkdir()
        self.parts = self.root / "parts"
        self.parts.mkdir()
        self.git("init", "-q")
        self.git("config", "user.name", "Local Test")
        self.git("config", "user.email", "test@example.invalid")
        self.lines = [f"source line {i}\n" for i in range(1, 45)]
        (self.repo / "shared.txt").write_text("".join(self.lines))
        (self.repo / "unique.txt").write_text("original\n")
        (self.repo / "binary.dat").write_bytes(b"data\x00old\xff")
        self.git("add", "-A")
        self.git("commit", "-qm", "baseline")
        for part in WRITERS:
            (self.parts / (part + ".patch")).write_bytes(b"")
            (self.parts / (part + ".files")).write_bytes(b"")

    def git(self, *args):
        return subprocess.run(
            ["git", *args], cwd=self.repo, check=True, stdout=subprocess.PIPE,
            stderr=subprocess.PIPE, timeout=10,
        ).stdout

    def make_writer(self, writer, changes):
        for path, data in changes.items():
            filename = self.repo / path
            filename.parent.mkdir(parents=True, exist_ok=True)
            if isinstance(data, bytes):
                filename.write_bytes(data)
            elif data is None:
                filename.unlink()
            else:
                filename.write_text(data)
        self.git("add", "-A")
        patch = self.git("diff", "--cached", "--binary")
        paths = self.git("diff", "--cached", "--name-only", "-z")
        (self.parts / (writer + ".patch")).write_bytes(patch)
        (self.parts / (writer + ".files")).write_bytes(paths)
        self.git("reset", "--hard", "-q", "HEAD")
        return patch

    def test_disjoint_text_and_real_binary_patches(self):
        binary = self.make_writer("production-export", {"binary.dat": b"data\x00new\x80"})
        self.assertIn(b"GIT binary patch", binary)
        self.make_writer("records", {"unique.txt": "new value\n"})
        self.assertEqual(inspect(self.parts),
                         [("production-export", 1), ("records", 1), ("oracles", 0)])
        self.assertEqual((self.repo / "binary.dat").read_bytes(), b"data\x00old\xff")

    def test_disjoint_hunks_same_file_both_apply_but_must_be_refused(self):
        a = self.lines.copy()
        a[2] = "changed only near beginning\n"
        b = self.lines.copy()
        b[38] = "changed only near end\n"
        first = self.make_writer("production-export", {"shared.txt": "".join(a)})
        second = self.make_writer("records", {"shared.txt": "".join(b)})
        self.assertNotEqual(first, second)
        # Demonstrate the pre-existing blind spot: both diffs can apply.
        self.git("apply", str(self.parts / "production-export.patch"))
        self.git("apply", str(self.parts / "records.patch"))
        with self.assertRaisesRegex(ValueError, "writer collision.*shared.txt"):
            inspect(self.parts)

    def test_patch_manifest_mismatch_fails_before_apply(self):
        self.make_writer("records", {"unique.txt": "updated\n"})
        (self.parts / "records.files").write_bytes(b"shared.txt\0")
        with self.assertRaisesRegex(ValueError, "inventory disagree"):
            inspect(self.parts)

    def test_duplicate_names_inside_one_writer_fails(self):
        self.make_writer("records", {"unique.txt": "updated\n"})
        (self.parts / "records.files").write_bytes(b"unique.txt\0unique.txt\0")
        with self.assertRaisesRegex(ValueError, "repeated"):
            inspect(self.parts)

    def test_newline_delimiters_are_refused(self):
        self.make_writer("records", {"unique.txt": "updated\n"})
        (self.parts / "records.files").write_bytes(b"unique.txt\n")
        with self.assertRaisesRegex(ValueError, "NUL-delimited"):
            inspect(self.parts)

    def test_missing_file_manifest_fails(self):
        (self.parts / "records.files").unlink()
        with self.assertRaisesRegex(ValueError, "missing"):
            inspect(self.parts)

    def test_missing_patch_fails(self):
        (self.parts / "records.patch").unlink()
        with self.assertRaisesRegex(ValueError, "missing"):
            inspect(self.parts)

    def test_symlink_manifest_fails(self):
        (self.parts / "records.files").unlink()
        try:
            (self.parts / "records.files").symlink_to(self.parts / "oracles.files")
        except OSError:
            self.skipTest("symlinks not available")
        with self.assertRaisesRegex(ValueError, "symlink"):
            inspect(self.parts)

    def test_unsafe_filename_is_refused(self):
        self.make_writer("records", {"unique.txt": "updated\n"})
        (self.parts / "records.files").write_bytes(b"../outside.txt\0")
        with self.assertRaisesRegex(ValueError, "unsafe"):
            inspect(self.parts)

    def test_patch_metadata_rename_is_understood(self):
        (self.repo / "unique.txt").rename(self.repo / "renamed.txt")
        self.git("add", "-A")
        (self.parts / "records.patch").write_bytes(self.git("diff", "--cached", "--binary"))
        (self.parts / "records.files").write_bytes(
            self.git("diff", "--cached", "--name-only", "-z"))
        self.assertEqual(inspect(self.parts)[1], ("records", 1))

    def test_filename_containing_newline_is_preserved(self):
        name = "line\nbreak.txt"
        (self.repo / name).write_text("before\n")
        self.git("add", "-A")
        self.git("commit", "-qm", "add newline path")
        self.make_writer("records", {name: "after\n"})
        self.assertEqual(inspect(self.parts)[1], ("records", 1))

    def test_incomplete_binary_numstat_is_refused(self):
        with self.assertRaisesRegex(ValueError, "truncated"):
            patch_names(b"-\t-\tfile", "test.patch")


if __name__ == "__main__":
    unittest.main()

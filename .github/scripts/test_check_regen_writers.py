"""Real Git patch and manifest falsifiers for regenerated writer ownership (#489)."""
import os
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path

from check_regen_writers import WRITERS, inspect, patch_names, paths_from_nul


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

    def make_writer(self, writer, changes, unstage_locks=False):
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
        if unstage_locks:
            self.git("reset", "-q", "--", "bootstrap/lexlean.lock",
                     "stdlib/lexlean.lock", "compiler/lexlean.lock")
        patch = self.git("diff", "--cached", "--no-renames", "--binary")
        paths = self.git("diff", "--cached", "--no-renames", "--name-only", "-z")
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

    def test_inspection_from_nested_checkout_preserves_root_paths(self):
        self.make_writer("records", {"unique.txt": "updated\n"})
        nested = self.repo / "nested"
        nested.mkdir()
        before = Path.cwd()
        try:
            os.chdir(nested)
            self.assertEqual(inspect(self.parts)[1], ("records", 1))
        finally:
            os.chdir(before)

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

    def test_rename_metadata_refused_and_both_paths_accounted(self):
        (self.repo / "unique.txt").rename(self.repo / "renamed.txt")
        self.git("add", "-A")
        patch = self.parts / "records.patch"
        manifest = self.parts / "records.files"
        patch.write_bytes(self.git("diff", "--cached", "-M", "--binary"))
        manifest.write_bytes(self.git("diff", "--cached", "-M", "--name-only", "-z"))
        self.assertIn(b"rename from ", patch.read_bytes())
        with self.assertRaisesRegex(ValueError, "unexpected rename/copy"):
            inspect(self.parts)
        patch.write_bytes(self.git("diff", "--cached", "--no-renames", "--binary"))
        manifest.write_bytes(self.git("diff", "--cached", "--no-renames", "--name-only", "-z"))
        self.assertEqual(inspect(self.parts)[1], ("records", 2))
        self.git("reset", "--hard", "-q", "HEAD")
        self.make_writer("oracles", {"unique.txt": "conflicting source edit\n"})
        with self.assertRaisesRegex(ValueError, "writer collision.*unique.txt"):
            inspect(self.parts)

    def test_nul_parser_preserves_newline_on_every_host(self):
        name = b"line\nbreak.txt"
        self.assertEqual(paths_from_nul(name + b"\0", "fixture"), [name])

    @unittest.skipIf(os.name == "nt", "Windows cannot create newline filenames")
    def test_filename_containing_newline_is_preserved(self):
        name = "line\nbreak.txt"
        (self.repo / name).write_text("before\n")
        self.git("add", "-A")
        self.git("commit", "-qm", "add newline path")
        self.make_writer("records", {name: "after\n"})
        self.assertEqual(inspect(self.parts)[1], ("records", 1))

    def lock_changes(self):
        locks = ["bootstrap/lexlean.lock", "stdlib/lexlean.lock", "compiler/lexlean.lock"]
        for lock in locks:
            target = self.repo / lock
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text("old\n")
        self.git("add", "-A")
        self.git("commit", "-qm", "register lock fixtures")
        return {lock: "new compiler identity\n" for lock in locks}

    def test_lock_refresh_only_one_writer_publishes_lock_paths(self):
        changes = self.lock_changes()
        self.make_writer("production-export", changes)
        self.make_writer("records", {**changes, "unique.txt": "record written\n"},
                         unstage_locks=True)
        self.make_writer("oracles", {**changes, "shared.txt": "oracle\n"},
                         unstage_locks=True)
        self.assertEqual(inspect(self.parts),
                         [("production-export", 3), ("records", 1), ("oracles", 1)])
        self.assertEqual((self.parts / "records.files").read_bytes(), b"unique.txt\0")

    def test_without_single_lock_owner_collision_is_detected(self):
        changes = self.lock_changes()
        self.make_writer("production-export", changes)
        self.make_writer("records", changes)
        with self.assertRaisesRegex(ValueError, "writer collision.*lexlean.lock"):
            inspect(self.parts)

    @staticmethod
    def check_workflow(source):
        steps = source.split("\n      - ")
        def step(name):
            found = [item for item in steps if item.startswith("name: " + name + "\n")]
            if len(found) != 1:
                raise ValueError("missing or repeated step: " + name)
            return found[0]
        capture = step("Capture what the recipes wrote")
        guard = step("Reject overlapping or unaccounted writer paths")
        apply = step("Apply every part")
        if not all(command in capture for command in (
            "git diff --cached --no-renames --binary",
            "git diff --cached --no-renames --name-only -z",
        )):
            raise ValueError("both capture commands must expose rename source paths")
        expected_guard = (
            "name: Reject overlapping or unaccounted writer paths\n"
            "        shell: bash\n"
            '        run: python3 .github/scripts/check_regen_writers.py --parts-dir "$RUNNER_TEMP/parts"'
        )
        if guard != expected_guard:
            raise ValueError("writer guard must execute unconditionally and fail closed")
        if steps.index(guard) >= steps.index(apply):
            raise ValueError("writer guard must precede patch application")

    def test_workflow_relocks_before_writers_and_checks_before_apply(self):
        workflow = Path(__file__).resolve().parents[1] / "workflows" / "regenerate.yml"
        if not workflow.is_file():
            self.skipTest("workflow unavailable outside full checkout")
        source = workflow.read_text(encoding="utf-8")
        self.check_workflow(source)
        self.assertLess(source.index("- name: Refresh the project locks"),
                        source.index("- name: Run the write recipes"))
        self.assertIn("cargo build -q -p lexlean", source)
        self.assertIn("for p in bootstrap stdlib compiler", source)
        self.assertIn("git reset -q -- bootstrap/lexlean.lock stdlib/lexlean.lock compiler/lexlean.lock", source)
        self.assertLess(source.index("- name: Reject overlapping or unaccounted writer paths"),
                        source.index("- name: Apply every part"))

    def test_workflow_refuses_weakened_writer_guards(self):
        workflow = Path(__file__).resolve().parents[1] / "workflows" / "regenerate.yml"
        if not workflow.is_file():
            self.skipTest("workflow unavailable outside full checkout")
        source = workflow.read_text(encoding="utf-8")
        name = "- name: Reject overlapping or unaccounted writer paths"
        for old, new, reason in (
            (name, name + "\n        continue-on-error: true", "fail closed"),
            (name, name + "\n        if: false", "fail closed"),
            ('run: python3 .github/scripts/check_regen_writers.py --parts-dir "$RUNNER_TEMP/parts"',
             'run: python3 .github/scripts/check_regen_writers.py --parts-dir "$RUNNER_TEMP/parts" || true', "fail closed"),
            ('run: python3 .github/scripts/check_regen_writers.py --parts-dir "$RUNNER_TEMP/parts"',
             'run: python3 .github/scripts/check_regen_writers.py --parts-dir "$RUNNER_TEMP/parts"; exit 0', "fail closed"),
            (name + "\n        shell: bash", name + "\n        shell: bash {0}", "fail closed"),
            ("git diff --cached --no-renames --binary", "git diff --cached --binary", "rename source"),
            ("git diff --cached --no-renames --name-only -z", "git diff --cached --name-only -z", "rename source"),
        ):
            with self.subTest(plant=old, replacement=new):
                self.assertIn(old, source)
                with self.assertRaisesRegex(ValueError, reason):
                    self.check_workflow(source.replace(old, new, 1))

    def test_incomplete_binary_numstat_is_refused(self):
        with self.assertRaisesRegex(ValueError, "truncated"):
            patch_names(b"-\t-\tfile", "test.patch")


if __name__ == "__main__":
    unittest.main()

"""Real Git patch-set and manifest falsifiers for regenerated writer ownership
(#489, #624)."""
import hashlib
import json
import os
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path

import patch_set
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
        self.base = self.git("rev-parse", "HEAD").decode().strip()
        for part in WRITERS:
            patch_set.write(part, self.parts, self.base, repo=self.repo)

    def inspect(self):
        return inspect(self.parts, self.base, repo=self.repo)

    def clear(self, writer):
        for path in self.parts.iterdir():
            if path.name.startswith(writer + "."):
                path.unlink()

    def reseal_raw(self, writer):
        manifest_path = self.parts / (writer + ".patchset.json")
        manifest = json.loads(manifest_path.read_text())
        files = (self.parts / (writer + ".files")).read_bytes()
        manifest["files_sha256"] = hashlib.sha256(files).hexdigest()
        manifest["paths"] = files.count(b"\0")
        for row in manifest["parts"]:
            part = self.parts / row["file"]
            row["bytes"] = part.stat().st_size
            row["sha256"] = patch_set.sha256(part)
        manifest_path.write_text(json.dumps(manifest))

    def reseal(self, writer):
        """Make the manifest agree with tampered files, so that the check
        under test, not a checksum, is what refuses them."""
        manifest_path = self.parts / (writer + ".patchset.json")
        manifest = json.loads(manifest_path.read_text())
        files = (self.parts / (writer + ".files")).read_bytes()
        manifest["files_sha256"] = hashlib.sha256(files).hexdigest()
        manifest["paths"] = files.count(b"\0")
        for row in manifest["parts"]:
            part = self.parts / row["file"]
            row["bytes"] = part.stat().st_size
            row["sha256"] = patch_set.sha256(part)
            found = patch_set.numstat(part)
            row["paths"] = len(found)
            row["paths_sha256"] = patch_set.nul_digest(found)
        manifest_path.write_text(json.dumps(manifest))

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
        self.clear(writer)
        manifest = patch_set.write(writer, self.parts, self.base, repo=self.repo)
        self.git("reset", "--hard", "-q", "HEAD")
        return b"".join((self.parts / row["file"]).read_bytes() for row in manifest["parts"])

    def test_disjoint_text_and_real_binary_patches(self):
        binary = self.make_writer("production-export", {"binary.dat": b"data\x00new\x80"})
        self.assertIn(b"GIT binary patch", binary)
        self.make_writer("records", {"unique.txt": "new value\n"})
        self.assertEqual(self.inspect(),
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
        self.git("apply", str(self.parts / "production-export.0001.patch"))
        self.git("apply", str(self.parts / "records.0001.patch"))
        with self.assertRaisesRegex(ValueError, "writer collision.*shared.txt"):
            self.inspect()

    def test_inspection_from_nested_checkout_preserves_root_paths(self):
        self.make_writer("records", {"unique.txt": "updated\n"})
        nested = self.repo / "nested"
        nested.mkdir()
        before = Path.cwd()
        try:
            os.chdir(nested)
            self.assertEqual(inspect(self.parts, self.base)[1], ("records", 1))
        finally:
            os.chdir(before)

    def test_patch_manifest_mismatch_fails_before_apply(self):
        self.make_writer("records", {"unique.txt": "updated\n"})
        (self.parts / "records.files").write_bytes(b"shared.txt\0")
        with self.assertRaisesRegex(ValueError, "file manifest checksum"):
            self.inspect()
        self.reseal("records")
        with self.assertRaisesRegex(ValueError, "inventory disagree"):
            self.inspect()

    def test_duplicate_names_inside_one_writer_fails(self):
        self.make_writer("records", {"unique.txt": "updated\n"})
        (self.parts / "records.files").write_bytes(b"unique.txt\0unique.txt\0")
        self.reseal("records")
        with self.assertRaisesRegex(ValueError, "repeated"):
            self.inspect()

    def test_newline_delimiters_are_refused(self):
        self.make_writer("records", {"unique.txt": "updated\n"})
        (self.parts / "records.files").write_bytes(b"unique.txt\n")
        self.reseal("records")
        with self.assertRaisesRegex(ValueError, "NUL-delimited"):
            self.inspect()

    def test_missing_file_manifest_fails(self):
        (self.parts / "records.files").unlink()
        with self.assertRaisesRegex(ValueError, "missing"):
            self.inspect()

    def test_missing_patch_fails(self):
        self.make_writer("records", {"unique.txt": "updated\n"})
        (self.parts / "records.0001.patch").unlink()
        with self.assertRaisesRegex(ValueError, "missing"):
            self.inspect()

    def test_symlink_manifest_fails(self):
        (self.parts / "records.files").unlink()
        try:
            (self.parts / "records.files").symlink_to(self.parts / "oracles.files")
        except OSError:
            self.skipTest("symlinks not available")
        with self.assertRaisesRegex(ValueError, "symlink"):
            self.inspect()

    def test_unsafe_filename_is_refused(self):
        self.make_writer("records", {"unique.txt": "updated\n"})
        (self.parts / "records.files").write_bytes(b"../outside.txt\0")
        self.reseal("records")
        with self.assertRaisesRegex(ValueError, "unsafe"):
            self.inspect()

    def test_rename_metadata_refused_and_both_paths_accounted(self):
        (self.repo / "unique.txt").rename(self.repo / "renamed.txt")
        self.git("add", "-A")
        self.clear("records")
        patch_set.write("records", self.parts, self.base, repo=self.repo)
        patch = self.parts / "records.0001.patch"
        manifest = self.parts / "records.files"
        self.assertEqual(self.inspect()[1], ("records", 2))
        patch.write_bytes(self.git("diff", "--cached", "-M", "--binary"))
        manifest.write_bytes(self.git("diff", "--cached", "-M", "--name-only", "-z"))
        self.assertIn(b"rename from ", patch.read_bytes())
        self.reseal_raw("records")
        with self.assertRaisesRegex(ValueError, "unexpected rename/copy"):
            self.inspect()
        self.git("reset", "--hard", "-q", "HEAD")
        self.make_writer("records", {"unique.txt": None,
                                     "renamed.txt": "original\n"})
        self.assertEqual(self.inspect()[1], ("records", 2))
        self.make_writer("oracles", {"unique.txt": "conflicting source edit\n"})
        with self.assertRaisesRegex(ValueError, "writer collision.*unique.txt"):
            self.inspect()

    def test_nul_parser_preserves_newline_on_every_host(self):
        name = b"line\nbreak.txt"
        self.assertEqual(paths_from_nul(name + b"\0", "fixture"), [name])

    @unittest.skipIf(os.name == "nt", "Windows cannot create newline filenames")
    def test_filename_containing_newline_is_preserved(self):
        name = "line\nbreak.txt"
        (self.repo / name).write_text("before\n")
        self.git("add", "-A")
        self.git("commit", "-qm", "add newline path")
        self.rebase()
        self.make_writer("records", {name: "after\n"})
        self.assertEqual(self.inspect()[1], ("records", 1))

    def lock_changes(self):
        locks = ["bootstrap/lexlean.lock", "stdlib/lexlean.lock", "compiler/lexlean.lock"]
        for lock in locks:
            target = self.repo / lock
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text("old\n")
        self.git("add", "-A")
        self.git("commit", "-qm", "register lock fixtures")
        self.rebase()
        return {lock: "new compiler identity\n" for lock in locks}

    def rebase(self):
        """A fixture committed after setUp: the writers regenerate that commit."""
        self.base = self.git("rev-parse", "HEAD").decode().strip()
        for part in WRITERS:
            self.clear(part)
            patch_set.write(part, self.parts, self.base, repo=self.repo)

    def test_lock_refresh_only_one_writer_publishes_lock_paths(self):
        changes = self.lock_changes()
        self.make_writer("production-export", changes)
        self.make_writer("records", {**changes, "unique.txt": "record written\n"},
                         unstage_locks=True)
        self.make_writer("oracles", {**changes, "shared.txt": "oracle\n"},
                         unstage_locks=True)
        self.assertEqual(self.inspect(),
                         [("production-export", 3), ("records", 1), ("oracles", 1)])
        self.assertEqual((self.parts / "records.files").read_bytes(), b"unique.txt\0")

    def test_writer_from_another_start_tree_is_refused(self):
        (self.repo / "unique.txt").write_text("a later commit\n")
        self.git("commit", "-qam", "later")
        self.make_writer("records", {"shared.txt": "written on the later commit\n"})
        with self.assertRaisesRegex(ValueError, "shared start tree"):
            self.inspect()

    def test_writer_tree_that_its_parts_do_not_give_is_refused(self):
        self.make_writer("records", {"unique.txt": "updated\n"})
        manifest_path = self.parts / "records.patchset.json"
        manifest = json.loads(manifest_path.read_text())
        manifest["tree"] = manifest["start_tree"]
        manifest_path.write_text(json.dumps(manifest))
        with self.assertRaisesRegex(ValueError, "the parts give tree"):
            self.inspect()

    def test_old_single_patch_artifact_is_refused(self):
        self.make_writer("records", {"unique.txt": "updated\n"})
        (self.parts / "records.patch").write_bytes(b"")
        with self.assertRaisesRegex(ValueError, "unlisted files claim the set"):
            self.inspect()

    def test_without_single_lock_owner_collision_is_detected(self):
        changes = self.lock_changes()
        self.make_writer("production-export", changes)
        self.make_writer("records", changes)
        with self.assertRaisesRegex(ValueError, "writer collision.*lexlean.lock"):
            self.inspect()

    GUARD = ('run: python3 .github/scripts/check_regen_writers.py '
             '--parts-dir "$RUNNER_TEMP/parts" --base "$COMMIT"')
    CAPTURE = ('python3 .github/scripts/patch_set.py write --name "${{ matrix.writer }}" '
               '--base "$COMMIT" --out "$RUNNER_TEMP/part"')
    COMBINE = ('python3 .github/scripts/patch_set.py combine --dir "$RUNNER_TEMP/parts" '
               '--names production-export,records,oracles --base "$COMMIT"')
    FINISH = ('run: python3 .github/scripts/patch_set.py apply --dir "$RUNNER_TEMP/combined" '
              '--name combined --base "$COMMIT"')
    RETURN = ('python3 .github/scripts/patch_set.py write --name regenerated '
              '--base "$COMMIT" --out "$out"')
    DELIVER = ('python3 .github/scripts/patch_set.py apply --dir "$dir" --name regenerated '
               '--base "$commit"')

    @classmethod
    def check_workflow(cls, source, deliver, tool):
        steps = source.split("\n      - ")
        def step(name):
            found = [item for item in steps if item.startswith("name: " + name + "\n")]
            if len(found) != 1:
                raise ValueError("missing or repeated step: " + name)
            return found[0]
        capture = step("Capture what the recipes wrote")
        guard = step("Reject overlapping or unaccounted writer paths")
        apply = step("Apply every part")
        finish = step("Apply the regenerated tree")
        returned = step("Return the regenerated tree")
        if cls.CAPTURE not in capture or "git diff --cached --" in capture:
            raise ValueError("each writer must return a bounded patch set")
        expected_guard = (
            "name: Reject overlapping or unaccounted writer paths\n"
            "        shell: bash\n"
            "        env:\n"
            "          COMMIT: ${{ inputs.ref || github.sha }}\n"
            "        " + cls.GUARD
        )
        if guard != expected_guard:
            raise ValueError("writer guard must execute unconditionally and fail closed")
        if steps.index(guard) >= steps.index(apply):
            raise ValueError("writer guard must precede patch application")
        if cls.COMBINE not in apply or cls.FINISH not in finish or cls.RETURN not in returned:
            raise ValueError("every patch must pass between jobs as a checked patch set")
        if "git apply" in source or "git apply" in deliver or cls.DELIVER not in deliver:
            raise ValueError("every patch must pass between jobs as a checked patch set")
        # The writers' sets name rename sources: patch_set.py diffs without
        # renames, every time it diffs the index.
        if not all(command in tool for command in (
            '"diff", "--cached", "--no-renames", "--binary"',
            '"diff", "--cached", "--no-renames", "--name-only", "-z"',
        )) or tool.count('"diff", "--cached"') != tool.count('"diff", "--cached", "--no-renames"'):
            raise ValueError("both capture commands must expose rename source paths")

    @staticmethod
    def sources():
        scripts = Path(__file__).resolve().parent
        workflow = scripts.parent / "workflows" / "regenerate.yml"
        if not workflow.is_file():
            raise unittest.SkipTest("workflow unavailable outside full checkout")
        return (workflow.read_text(encoding="utf-8"),
                (scripts / "deliver-regenerated.sh").read_text(encoding="utf-8"),
                (scripts / "patch_set.py").read_text(encoding="utf-8"))

    def test_workflow_relocks_before_writers_and_checks_before_apply(self):
        source, deliver, tool = self.sources()
        self.check_workflow(source, deliver, tool)
        self.assertLess(source.index("- name: Refresh the project locks"),
                        source.index("- name: Run the write recipes"))
        self.assertIn("cargo build -q -p lexlean", source)
        self.assertIn("for p in bootstrap stdlib compiler", source)
        self.assertIn("git reset -q -- bootstrap/lexlean.lock stdlib/lexlean.lock compiler/lexlean.lock", source)
        self.assertLess(source.index("- name: Reject overlapping or unaccounted writer paths"),
                        source.index("- name: Apply every part"))

    def test_workflow_refuses_weakened_writer_guards(self):
        source, deliver, tool = self.sources()
        name = "- name: Reject overlapping or unaccounted writer paths"
        workflow_plants = (
            (name, name + "\n        continue-on-error: true", "fail closed"),
            (name, name + "\n        if: false", "fail closed"),
            (self.GUARD, self.GUARD + " || true", "fail closed"),
            (self.GUARD, self.GUARD + "; exit 0", "fail closed"),
            (self.GUARD, self.GUARD.replace(' --base "$COMMIT"', ""), "fail closed"),
            (name + "\n        shell: bash", name + "\n        shell: bash {0}", "fail closed"),
            (self.CAPTURE, 'git diff --cached --no-renames --binary > "$RUNNER_TEMP/part/records.patch"',
             "bounded patch set"),
            (self.COMBINE, 'git apply --index "$RUNNER_TEMP/parts/records.patch"', "checked patch set"),
            (self.FINISH, 'run: git apply --index "$RUNNER_TEMP/combined/combined.patch"',
             "checked patch set"),
            (self.RETURN, 'git diff --cached --binary > "$out/regenerated.patch"', "checked patch set"),
        )
        for old, new, reason in workflow_plants:
            with self.subTest(plant=old, replacement=new):
                self.assertIn(old, source)
                with self.assertRaisesRegex(ValueError, reason):
                    self.check_workflow(source.replace(old, new, 1), deliver, tool)
        with self.assertRaisesRegex(ValueError, "checked patch set"):
            self.check_workflow(source, deliver.replace(
                self.DELIVER, 'git apply --index "$dir/regenerated.patch"', 1), tool)
        for old in ('"diff", "--cached", "--no-renames", "--binary"',
                    '"diff", "--cached", "--no-renames", "--name-only", "-z"'):
            with self.subTest(plant=old):
                self.assertIn(old, tool)
                with self.assertRaisesRegex(ValueError, "rename source"):
                    self.check_workflow(source, deliver,
                                        tool.replace(old, old.replace(' "--no-renames",', ""), 1))

    def test_incomplete_binary_numstat_is_refused(self):
        with self.assertRaisesRegex(ValueError, "truncated"):
            patch_names(b"-\t-\tfile", "test.patch")


if __name__ == "__main__":
    unittest.main()

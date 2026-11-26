"""Falsifiers for the bounded patch sets of regenerate.yml (#624), on real Git
repositories with thresholds small enough that a handful of files make many
parts."""
import json
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path

import patch_set

TARGET = 600
MAXIMUM = 4000


@unittest.skipUnless(shutil.which("git"), "requires a local Git executable")
class PatchSetTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        root = Path(self.tmp.name)
        self.repo = root / "repo"
        self.repo.mkdir()
        self.out = root / "out"
        self.git("init", "-q")
        self.git("config", "user.name", "Local Test")
        self.git("config", "user.email", "test@example.invalid")
        for i in range(12):
            (self.repo / ("file%02d.txt" % i)).write_text("".join(
                "line %d of file %d\n" % (n, i) for n in range(20)))
        (self.repo / "image.bin").write_bytes(bytes(range(256)) * 2)
        self.git("add", "-A")
        self.git("commit", "-qm", "base")
        self.base = self.git("rev-parse", "HEAD").decode().strip()

    def git(self, *args, input=None):
        return subprocess.run(["git", *args], cwd=self.repo, input=input, check=True,
                              stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                              timeout=60).stdout

    def change(self):
        for i in range(12):
            (self.repo / ("file%02d.txt" % i)).write_text("".join(
                "rewritten line %d of file %d\n" % (n, i) for n in range(20)))
        (self.repo / "image.bin").write_bytes(bytes(reversed(range(256))) * 2)
        (self.repo / "added.txt").write_text("new\n")
        (self.repo / "file11.txt").unlink()
        self.git("add", "-A")

    def write(self, name="records", target=TARGET, maximum=MAXIMUM):
        return patch_set.write(name, self.out, self.base, target, maximum, repo=self.repo)

    def written(self):
        self.change()
        manifest = self.write()
        tree = self.git("write-tree").decode().strip()
        self.git("reset", "-q", "--hard", "HEAD")
        return manifest, tree

    def manifest(self, name="records"):
        return json.loads((self.out / (name + ".patchset.json")).read_text())

    def save(self, manifest, name="records"):
        (self.out / (name + ".patchset.json")).write_text(json.dumps(manifest))

    def test_parts_are_whole_file_diffs_below_the_maximum(self):
        whole = None
        self.change()
        whole = self.git("diff", "--cached", "--no-renames", "--binary")
        manifest = self.write()
        self.git("reset", "-q", "--hard", "HEAD")
        parts = [(self.out / row["file"]).read_bytes() for row in manifest["parts"]]
        self.assertGreater(len(parts), 3)
        self.assertEqual(b"".join(parts), whole)
        for part in parts:
            self.assertTrue(part.startswith(b"diff --git "))
            self.assertLess(len(part), MAXIMUM)
        self.assertIn(b"GIT binary patch", whole)
        self.assertEqual(manifest["paths"], 14)
        self.assertEqual(patch_set.load(self.out, "records", self.base)[1].__len__(), 14)

    def test_the_parts_give_the_recorded_tree_in_place_and_in_a_scratch_index(self):
        manifest, tree = self.written()
        self.assertEqual(manifest["tree"], tree)
        patch_set.check_tree(self.out, "records", manifest, repo=self.repo)
        patch_set.apply(self.out, "records", self.base, repo=self.repo)
        self.assertEqual(self.git("write-tree").decode().strip(), tree)
        self.assertEqual((self.repo / "image.bin").read_bytes(),
                         bytes(reversed(range(256))) * 2)

    def test_writing_is_deterministic(self):
        self.change()
        first = self.write()
        again = Path(self.tmp.name) / "again"
        second = patch_set.write("records", again, self.base, TARGET, MAXIMUM, repo=self.repo)
        self.assertEqual(first, second)
        for row in first["parts"]:
            self.assertEqual((self.out / row["file"]).read_bytes(),
                             (again / row["file"]).read_bytes())

    def test_an_empty_change_is_a_set_of_no_parts(self):
        manifest = self.write()
        self.assertEqual(manifest["parts"], [])
        self.assertEqual(manifest["start_tree"], manifest["tree"])
        patch_set.apply(self.out, "records", self.base, repo=self.repo)

    def test_a_tampered_part_is_refused(self):
        manifest, _ = self.written()
        part = self.out / manifest["parts"][1]["file"]
        data = bytearray(part.read_bytes())
        data[-2] ^= 1
        part.write_bytes(bytes(data))
        with self.assertRaisesRegex(ValueError, "checksum"):
            patch_set.load(self.out, "records", self.base)

    def test_a_missing_part_is_refused(self):
        manifest, _ = self.written()
        (self.out / manifest["parts"][-1]["file"]).unlink()
        with self.assertRaisesRegex(ValueError, "missing"):
            patch_set.load(self.out, "records", self.base)

    def test_a_dropped_part_row_is_refused(self):
        manifest, _ = self.written()
        manifest["parts"].pop()
        self.save(manifest)
        with self.assertRaisesRegex(ValueError, "unlisted files claim the set"):
            patch_set.load(self.out, "records", self.base)
        (self.out / ("records.%04d.patch" % (len(manifest["parts"]) + 1))).unlink()
        with self.assertRaisesRegex(ValueError, "inventory disagree"):
            patch_set.load(self.out, "records", self.base)

    def test_reordered_parts_are_refused(self):
        manifest, _ = self.written()
        manifest["parts"][0], manifest["parts"][1] = manifest["parts"][1], manifest["parts"][0]
        self.save(manifest)
        with self.assertRaisesRegex(ValueError, "part order"):
            patch_set.load(self.out, "records", self.base)

    def test_a_path_written_by_two_parts_is_refused(self):
        manifest, _ = self.written()
        first, second = manifest["parts"][0], manifest["parts"][1]
        duplicate = self.out / second["file"]
        duplicate.write_bytes((self.out / first["file"]).read_bytes())
        for key in ("bytes", "sha256", "paths", "paths_sha256"):
            second[key] = first[key]
        self.save(manifest)
        with self.assertRaisesRegex(ValueError, "another part also writes"):
            patch_set.load(self.out, "records", self.base)

    def test_an_artifact_of_another_commit_is_refused(self):
        self.written()
        (self.repo / "later.txt").write_text("later\n")
        self.git("add", "-A")
        self.git("commit", "-qm", "later")
        later = self.git("rev-parse", "HEAD").decode().strip()
        with self.assertRaisesRegex(ValueError, "made from"):
            patch_set.load(self.out, "records", later)
        with self.assertRaisesRegex(ValueError, "not the start tree"):
            patch_set.apply(self.out, "records", self.base, repo=self.repo)

    def test_a_recorded_tree_the_parts_do_not_give_is_refused(self):
        manifest, _ = self.written()
        manifest["tree"] = manifest["start_tree"]
        self.save(manifest)
        with self.assertRaisesRegex(ValueError, "the parts give tree"):
            patch_set.check_tree(self.out, "records", manifest, repo=self.repo)
        with self.assertRaisesRegex(ValueError, "applied tree"):
            patch_set.apply(self.out, "records", self.base, repo=self.repo)

    def test_one_file_diff_at_the_maximum_is_refused(self):
        (self.repo / "file00.txt").write_text("x" * MAXIMUM + "\n")
        self.git("add", "-A")
        with self.assertRaisesRegex(ValueError, "reaches the part maximum"):
            self.write()

    def test_thresholds_at_gits_limit_are_refused(self):
        with self.assertRaisesRegex(ValueError, "thresholds"):
            self.write(maximum=patch_set.GIT_APPLY_LIMIT)
        self.write(target=TARGET, maximum=MAXIMUM)
        manifest = self.manifest()
        manifest["max_bytes"] = patch_set.GIT_APPLY_LIMIT
        self.save(manifest)
        with self.assertRaisesRegex(ValueError, "thresholds"):
            patch_set.load(self.out, "records", self.base)

    def test_unknown_manifest_keys_are_refused(self):
        self.written()
        manifest = self.manifest()
        manifest["note"] = "trust me"
        self.save(manifest)
        with self.assertRaisesRegex(ValueError, "manifest keys"):
            patch_set.load(self.out, "records", self.base)

    def test_an_unsafe_set_name_is_refused(self):
        with self.assertRaisesRegex(ValueError, "unsafe set name"):
            patch_set.load(self.out, "../records", self.base)

    def test_combining_disjoint_sets_gives_each_tree_on_its_paths(self):
        (self.repo / "file00.txt").write_text("first writer\n")
        self.git("add", "-A")
        patch_set.write("production-export", self.out, self.base, TARGET, MAXIMUM, repo=self.repo)
        self.git("reset", "-q", "--hard", "HEAD")
        self.change()
        self.git("reset", "-q", "--", "file00.txt")
        patch_set.write("records", self.out, self.base, TARGET, MAXIMUM, repo=self.repo)
        self.git("reset", "-q", "--hard", "HEAD")
        start, _, count = patch_set.combine(self.out, ["production-export", "records"],
                                            self.base, repo=self.repo)
        self.assertEqual(start, self.git("rev-parse", "HEAD^{tree}").decode().strip())
        self.assertEqual(count, 14)
        self.assertEqual((self.repo / "file00.txt").read_text(), "first writer\n")

    def test_combining_overlapping_sets_is_refused(self):
        (self.repo / "file00.txt").write_text("first writer\n")
        self.git("add", "-A")
        patch_set.write("production-export", self.out, self.base, TARGET, MAXIMUM, repo=self.repo)
        self.git("reset", "-q", "--hard", "HEAD")
        self.written()
        with self.assertRaisesRegex(ValueError, "writer collision.*file00"):
            patch_set.combine(self.out, ["production-export", "records"], self.base,
                              repo=self.repo)


if __name__ == "__main__":
    unittest.main()

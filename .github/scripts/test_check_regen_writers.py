"""Real Git patch-set and manifest falsifiers for regenerated writer ownership
(#489, #624)."""
import hashlib
import json
import os
import re
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path

import patch_set
import regen_lifecycle
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
    WRITE_COMBINED = ('python3 .github/scripts/patch_set.py write --name combined '
                      '--base "$COMMIT" --out "$RUNNER_TEMP/combined"')
    FINISH = ('run: python3 .github/scripts/patch_set.py apply --dir "$RUNNER_TEMP/combined" '
              '--name combined --base "$COMMIT"')
    RETURN = ('python3 .github/scripts/patch_set.py write --name regenerated '
              '--base "$COMMIT" --out "$out"')
    DELIVER = ('python3 .github/scripts/patch_set.py apply --dir "$dir" --name regenerated '
               '--base "$commit"')
    # The lines of deliver-regenerated.sh, in this order, each whole and once:
    # strict mode, the checksum and coverage checks, the commit binding, the
    # fast-forward check, the set's apply, then the commit, the push and the
    # leased deletion.
    DELIVERY = (
        "set -euo pipefail",
        '(cd "$dir" && sha256sum --check --strict --quiet SHA256SUMS)',
        'if [ "$listed" != "$present" ]; then',
        'if [ "$(cat "$dir/base-commit.txt")" != "$commit" ]; then',
        'remote=$(git ls-remote origin "refs/heads/$branch" | cut -f1)',
        'if [ "$remote" != "$commit" ]; then',
        DELIVER,
        "if git diff --cached --quiet; then",
        '    commit --quiet --file "$RUNNER_TEMP/message.txt"',
        '  git push origin "HEAD:refs/heads/$branch"',
        '    if git push --force-with-lease="refs/heads/$started:$commit" origin ":refs/heads/$started"; then',
    )

    @staticmethod
    def jobs(source):
        """Each job of a workflow: its header up to `steps:`, and its steps,
        each the text after its `- `, without the comments that follow it."""
        if source.count("\njobs:\n") != 1:
            raise ValueError("lifecycle: one jobs map")
        found, current = {}, None
        for line in source.split("\njobs:\n", 1)[1].split("\n"):
            match = re.match(r"^  ([a-z][a-z0-9-]*):$", line)
            if match:
                current = match.group(1)
                if current in found:
                    raise ValueError("lifecycle: repeated job " + current)
                found[current] = []
            if current:
                found[current].append(line)
        jobs = {}
        for name, lines in found.items():
            while lines and (lines[-1] == "" or lines[-1].startswith("  #")):
                lines.pop()
            text = "\n".join(lines) + "\n"
            if text.count("\n    steps:\n") != 1:
                raise ValueError("lifecycle: job " + name + " has no single steps list")
            header, steps = text.split("\n    steps:\n", 1)
            jobs[name] = (header + "\n    steps:\n", re.split(r"(?m)^      - ", steps)[1:])
        return jobs

    @classmethod
    def check_workflow(cls, source, deliver, tool):
        jobs = cls.jobs(source)
        if set(jobs) != set(regen_lifecycle.HEADERS):
            raise ValueError("lifecycle: the jobs are not writer, combine, finish, deliver")
        if "continue-on-error" in source:
            raise ValueError("lifecycle: no step or job may continue on error (fail closed)")
        for name, (header, steps) in jobs.items():
            if header != regen_lifecycle.HEADERS[name]:
                raise ValueError("lifecycle: job " + name + " header changed (fail closed)")
            pinned = regen_lifecycle.STEPS[name]
            if len(steps) != len(pinned):
                raise ValueError("lifecycle: job " + name + " steps added or removed (fail closed)")
            # Every step whole, the producers' too: a skipped or altered write
            # recipe would deliver a tree that was never regenerated.
            for step, pin in zip(steps, pinned):
                if step != pin:
                    raise ValueError("lifecycle: job " + name + " step changed: " +
                                     step.split("\n", 1)[0] + " (fail closed)")
        # The pins themselves name the reviewed commands, so a pin edited to
        # agree with a weakened step still fails here.
        texts = {name: "".join(regen_lifecycle.STEPS[name]) for name in regen_lifecycle.STEPS}
        for job, command in (("writer", cls.CAPTURE), ("combine", cls.GUARD),
                             ("combine", cls.COMBINE), ("combine", cls.WRITE_COMBINED),
                             ("finish", cls.FINISH), ("finish", cls.RETURN)):
            if texts[job].count(command) != 1 or command + " ||" in texts[job]:
                raise ValueError("every patch must pass between jobs as a checked patch set")
        if "git apply" in source or "git apply" in deliver:
            raise ValueError("every patch must pass between jobs as a checked patch set")
        cls.check_delivery(deliver)
        # The writers' sets name rename sources: patch_set.py diffs without
        # renames, every time it diffs the index.
        if not all(command in tool for command in (
            '"diff", "--cached", "--no-renames", "--binary"',
            '"diff", "--cached", "--no-renames", "--name-only", "-z"',
        )) or tool.count('"diff", "--cached"') != tool.count('"diff", "--cached", "--no-renames"'):
            raise ValueError("both capture commands must expose rename source paths")

    @classmethod
    def check_delivery(cls, deliver):
        # The pin must itself hold the delivery's shape, so that a pin edited
        # to agree with a weakened script still fails here; then the script is
        # the pin, whole: a roster of lines in order cannot see a branch that
        # skips one or an exit that was removed (review 6077939760).
        cls.check_delivery_shape(regen_lifecycle.DELIVERY)
        cls.check_delivery_shape(deliver)
        if deliver != regen_lifecycle.DELIVERY:
            raise ValueError("delivery: the script is not the reviewed script (fail closed)")

    @classmethod
    def check_delivery_shape(cls, deliver):
        lines = deliver.split("\n")
        if lines[0] != "#!/usr/bin/env bash":
            raise ValueError("delivery: not a bash script (fail closed)")
        code = [line for line in lines if line.strip() and not line.lstrip().startswith("#")]
        if not code or code[0] != "set -euo pipefail":
            raise ValueError("delivery: strict mode must come first (fail closed)")
        at = []
        for line in cls.DELIVERY:
            if lines.count(line) != 1:
                raise ValueError("delivery: missing or repeated: " + line + " (fail closed)")
            at.append(lines.index(line))
        if at != sorted(at):
            raise ValueError("delivery: its checks, apply, commit and push are out of order "
                             "(fail closed)")
        apply = at[cls.DELIVERY.index(cls.DELIVER)]
        for index, line in enumerate(lines):
            bare = line.split("#", 1)[0] if line.lstrip().startswith("#") else line
            if re.search(r"\bset\s+\+|\bset\s+-o\s+posix|\btrap\b|\|\|\s*(true|:)\s*$|"
                         r"--force(?!-with-lease=)|--no-verify", bare) and \
                    bare != '  count=$(printf \'%s\' "$heads" | grep -c . || true)':
                raise ValueError("delivery: a masked or relaxed command: " + line +
                                 " (fail closed)")
            if re.match(r"\s*(exit|return)\b", bare) and index > apply:
                raise ValueError("delivery: an exit after the set is applied (fail closed)")
        if sum(1 for line in lines if re.match(r"\s*exit 0\s*$", line)) != 2:
            raise ValueError("delivery: an early exit other than the two that push nothing "
                             "(fail closed)")

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
        apply_name = "- name: Apply every part"
        finish_name = "- name: Apply the regenerated tree"
        return_name = "- name: Return the regenerated tree"
        strict = "          set -euo pipefail\n          # Each set is checked whole again"
        workflow_plants = (
            (name, name + "\n        continue-on-error: true", "fail closed"),
            (name, name + "\n        if: false", "fail closed"),
            (self.GUARD, self.GUARD + " || true", "fail closed"),
            (self.GUARD, self.GUARD + "; exit 0", "fail closed"),
            (self.GUARD, self.GUARD.replace(' --base "$COMMIT"', ""), "fail closed"),
            (name + "\n        shell: bash", name + "\n        shell: bash {0}", "fail closed"),
            # Every later lifecycle boundary, not only the writer guard.
            (self.CAPTURE, self.CAPTURE + " || true", "fail closed"),
            (self.COMBINE, "exit 0\n          " + self.COMBINE, "fail closed"),
            (self.COMBINE, self.COMBINE + " || true", "fail closed"),
            (self.WRITE_COMBINED, self.WRITE_COMBINED + " || true", "fail closed"),
            (strict, strict.replace("set -euo pipefail", "set -eu"), "fail closed"),
            (strict, strict.replace("set -euo pipefail", "set +e"), "fail closed"),
            (apply_name, apply_name + "\n        if: false", "fail closed"),
            (apply_name + "\n        shell: bash", apply_name + "\n        shell: sh", "fail closed"),
            (self.FINISH, self.FINISH + " || true", "fail closed"),
            (finish_name, finish_name + "\n        if: false", "fail closed"),
            (finish_name, finish_name + "\n        continue-on-error: true", "fail closed"),
            (self.RETURN, self.RETURN + " || true", "fail closed"),
            (return_name + "\n        if: ${{ !cancelled() }}", return_name, "fail closed"),
            ("    needs: [finish]\n", "    needs: [finish]\n    if: false\n", "fail closed"),
            ("    needs: [combine]\n", "    needs: [writer]\n", "fail closed"),
            # A step moved ahead of the set it depends on.
            (finish_name, "- name: Write the moved counts", "fail closed"),
            (self.CAPTURE, 'git diff --cached --no-renames --binary > "$RUNNER_TEMP/part/records.patch"',
             "fail closed"),
            (self.COMBINE, 'git apply --index "$RUNNER_TEMP/parts/records.patch"', "fail closed"),
            (self.FINISH, 'run: git apply --index "$RUNNER_TEMP/combined/combined.patch"',
             "fail closed"),
            (self.RETURN, 'git diff --cached --binary > "$out/regenerated.patch"', "fail closed"),
            ("! -name SHA256SUMS", "-name 'regenerated.*'", "fail closed"),
        )
        # Every producer of the regenerated tree, skipped: its step is pinned
        # whole as much as the patch-set steps are (review 6077939760).
        producers = [name for name in (
            "Refresh the project locks", "Run the write recipes", "Write the moved counts",
            "Write the model documents",
            "Measure the stack bound of each recursive package the record lacks",
            "Relock and repin the moved identity") if "- name: " + name + "\n" in source]
        self.assertGreaterEqual(len(producers), 5)
        workflow_plants += tuple(
            ("- name: " + name + "\n", "- name: " + name + "\n        if: false\n", "fail closed")
            for name in producers)
        for old, new, reason in workflow_plants:
            with self.subTest(plant=old, replacement=new):
                self.assertIn(old, source)
                with self.assertRaisesRegex(ValueError, reason):
                    self.check_workflow(source.replace(old, new, 1), deliver, tool)
        listed = '(cd "$dir" && sha256sum --check --strict --quiet SHA256SUMS)'
        commit = 'git -c user.name=\'github-actions[bot]\''
        delivery_plants = (
            (self.DELIVER, 'git apply --index "$dir/regenerated.patch"'),
            (self.DELIVER, self.DELIVER + " || true"),
            (self.DELIVER, self.DELIVER + " || :"),
            (self.DELIVER, "exit 0\n" + self.DELIVER),
            (self.DELIVER, self.DELIVER + "\nexit 0"),
            (self.DELIVER, "set +e\n" + self.DELIVER),
            ("set -euo pipefail", "set -eu"),
            ("set -euo pipefail", "set -euo pipefail\nset +o pipefail"),
            ("set -euo pipefail", "set -euo pipefail\ntrap 'exit 0' ERR"),
            (listed, listed.replace(" --strict", "")),
            (listed, listed + " || true"),
            ('if [ "$listed" != "$present" ]; then', "if false; then"),
            ('if [ "$remote" != "$commit" ]; then', 'if false; then'),
            ('--force-with-lease="refs/heads/$started:$commit"', "--force"),
            ('  git push origin "HEAD:refs/heads/$branch"',
             '  git push --force origin "HEAD:refs/heads/$branch"'),
            # The set applied only after the commit is made.
            (self.DELIVER + "\n", ""),
            # Review 6077939760: control flow around lines that all remain.
            ('  echo "::error::SHA256SUMS does not list exactly the files of the artifact"\n  exit 1\n',
             '  echo "::error::SHA256SUMS does not list exactly the files of the artifact"\n'),
            ('if [ "$listed" != "$present" ]; then',
             'listed=$present\nif [ "$listed" != "$present" ]; then'),
            (self.DELIVER + "\n", "if false; then\n" + self.DELIVER + "\nfi\n"),
            ('$(cat "$dir/base-commit.txt"), not $commit"\n  exit 1\n',
             '$(cat "$dir/base-commit.txt"), not $commit"\n  :\n'),
        )
        for old, new in delivery_plants:
            with self.subTest(delivery=old, replacement=new):
                self.assertIn(old, deliver)
                planted = deliver.replace(old, new, 1)
                if old == self.DELIVER + "\n" and new == "":
                    planted = planted.replace("  git push origin", self.DELIVER + "\n  git push origin", 1)
                with self.assertRaisesRegex(ValueError, "fail closed|checked patch set"):
                    self.check_workflow(source, planted, tool)
        for old in ('"diff", "--cached", "--no-renames", "--binary"',
                    '"diff", "--cached", "--no-renames", "--name-only", "-z"'):
            with self.subTest(plant=old):
                self.assertIn(old, tool)
                with self.assertRaisesRegex(ValueError, "rename source"):
                    self.check_workflow(source, deliver,
                                        tool.replace(old, old.replace(' "--no-renames",', ""), 1))

    def test_a_pin_edited_to_a_masked_command_is_still_refused(self):
        source, deliver, tool = self.sources()
        saved = {name: list(steps) for name, steps in regen_lifecycle.STEPS.items()}
        try:
            for job, command in (("combine", self.COMBINE), ("finish", self.FINISH),
                                 ("finish", self.RETURN)):
                with self.subTest(job=job, command=command):
                    regen_lifecycle.STEPS[job] = [
                        p.replace(command, command + " || true") if isinstance(p, str) else p
                        for p in saved[job]]
                    with self.assertRaisesRegex(ValueError, "checked patch set"):
                        self.check_workflow(source.replace(command, command + " || true", 1),
                                            deliver, tool)
                    regen_lifecycle.STEPS[job] = saved[job]
        finally:
            for name, steps in saved.items():
                regen_lifecycle.STEPS[name] = steps

    def test_a_delivery_pin_edited_to_a_weakened_script_is_still_refused(self):
        source, deliver, tool = self.sources()
        saved = regen_lifecycle.DELIVERY
        try:
            for old, new in ((self.DELIVER, self.DELIVER + " || true"),
                             ("set -euo pipefail", "set -eu"),
                             (self.DELIVER + "\n", "exit 0\n" + self.DELIVER + "\n")):
                with self.subTest(old=old, new=new):
                    planted = deliver.replace(old, new, 1)
                    regen_lifecycle.DELIVERY = planted
                    with self.assertRaisesRegex(ValueError, "fail closed"):
                        self.check_workflow(source, planted, tool)
        finally:
            regen_lifecycle.DELIVERY = saved

    def test_incomplete_binary_numstat_is_refused(self):
        with self.assertRaisesRegex(ValueError, "truncated"):
            patch_names(b"-\t-\tfile", "test.patch")



SCRIPTS = Path(__file__).resolve().parent


@unittest.skipUnless(shutil.which("git") and shutil.which("bash") and
                     shutil.which("sha256sum") and os.name != "nt",
                     "requires Git, Bash and coreutils")
class DeliveryTests(unittest.TestCase):
    """deliver-regenerated.sh run for real, against a local bare remote and a
    `gh` that only records its calls: what reaches the branch, and what a
    refusal leaves there."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        root = Path(self.tmp.name)
        self.root = root
        self.remote = root / "remote.git"
        self.run_cmd(["git", "init", "-q", "--bare", str(self.remote)], root)
        self.repo = root / "checkout"
        self.repo.mkdir()
        self.git("init", "-q", "-b", "feature")
        self.git("config", "user.name", "Local Test")
        self.git("config", "user.email", "test@example.invalid")
        (self.repo / ".github" / "scripts").mkdir(parents=True)
        shutil.copy(SCRIPTS / "patch_set.py", self.repo / ".github" / "scripts")
        for i in range(6):
            (self.repo / ("record%d.txt" % i)).write_text("".join(
                "record line %d of %d\n" % (n, i) for n in range(30)))
        self.git("add", "-A")
        self.git("commit", "-qm", "base")
        self.git("remote", "add", "origin", str(self.remote))
        self.git("push", "-q", "origin", "HEAD:refs/heads/feature")
        self.commit = self.git("rev-parse", "HEAD").decode().strip()
        self.dir = root / "regenerated"
        bin_dir = root / "bin"
        bin_dir.mkdir()
        self.calls = root / "gh-calls"
        (bin_dir / "gh").write_text('#!/usr/bin/env bash\nprintf "%s\\n" "$*" >> "' +
                                    str(self.calls) + '"\n')
        (bin_dir / "gh").chmod(0o755)
        self.env = dict(os.environ, PATH=str(bin_dir) + os.pathsep + os.environ["PATH"],
                        REPO="owner/name", COMMIT=self.commit, BRANCH="feature",
                        RUN_URL="https://example.invalid/run", REGENERATED=str(self.dir),
                        GITHUB_REF="refs/heads/feature", RUNNER_TEMP=str(root),
                        GITHUB_STEP_SUMMARY=str(root / "summary"),
                        GITHUB_OUTPUT=str(root / "output"))
        for key in ("GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE"):
            self.env.pop(key, None)

    def run_cmd(self, args, cwd, env=None, check=True):
        return subprocess.run(args, cwd=cwd, env=env, check=check, stdout=subprocess.PIPE,
                              stderr=subprocess.PIPE, timeout=120)

    def git(self, *args):
        return self.run_cmd(["git", *args], self.repo).stdout

    def regenerated(self):
        """The artifact of `finish`, written as its return step writes it."""
        for i in range(6):
            (self.repo / ("record%d.txt" % i)).write_text("".join(
                "regenerated line %d of %d\n" % (n, i) for n in range(30)))
        (self.repo / "new-record.txt").write_text("added\n")
        self.git("add", "--all")
        manifest = patch_set.write("regenerated", self.dir, self.commit, 300, 3000,
                                   repo=self.repo)
        (self.dir / "regenerated-files.txt").write_bytes(
            self.git("diff", "--cached", "--name-only"))
        (self.dir / "base-commit.txt").write_text(self.commit + "\n")
        (self.dir / "counts.txt").write_text("counts: unchanged\n")
        self.git("reset", "-q", "--hard", "HEAD")
        self.git("clean", "-qfd")
        self.seal()
        self.assertGreater(len(manifest["parts"]), 2)
        return manifest

    def seal(self):
        (self.dir / "SHA256SUMS").unlink(missing_ok=True)
        self.run_cmd(["bash", "-c", "find . -mindepth 1 -maxdepth 1 -type f ! -name SHA256SUMS "
                      "-printf '%P\\0' | LC_ALL=C sort -z | xargs -0 sha256sum > SHA256SUMS"],
                     self.dir)

    def deliver(self, script=None):
        path = SCRIPTS / "deliver-regenerated.sh"
        if script is not None:
            path = self.root / "planted.sh"
            path.write_text(script)
        return self.run_cmd(["bash", str(path)], self.repo, env=self.env, check=False)

    def remote_head(self):
        return self.run_cmd(["git", "--git-dir", str(self.remote), "rev-parse",
                             "refs/heads/feature"], self.root).stdout.decode().strip()

    def gh_calls(self):
        return self.calls.read_text() if self.calls.exists() else ""

    def assert_refused(self, result, pattern):
        self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertRegex((result.stdout + result.stderr).decode("utf-8", "replace"), pattern)
        self.assertEqual(self.remote_head(), self.commit)
        self.assertEqual(self.gh_calls(), "")

    def test_a_whole_set_is_committed_on_the_commit_and_pushed(self):
        manifest = self.regenerated()
        result = self.deliver()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        head = self.remote_head()
        self.assertNotEqual(head, self.commit)
        tree = self.run_cmd(["git", "--git-dir", str(self.remote), "rev-parse",
                             head + "^{tree}"], self.root).stdout.decode().strip()
        self.assertEqual(tree, manifest["tree"])
        self.assertIn("workflow run ci.yml --repo owner/name --ref feature", self.gh_calls())

    def test_an_unlisted_artifact_file_is_refused(self):
        self.regenerated()
        (self.dir / "extra.patch").write_text("diff --git a/x b/x\n")
        self.assert_refused(self.deliver(), "does not list exactly the files")

    def test_an_altered_part_is_refused(self):
        manifest = self.regenerated()
        part = self.dir / manifest["parts"][-1]["file"]
        part.write_bytes(part.read_bytes() + b" ")
        self.assert_refused(self.deliver(), "FAILED|did NOT match")

    def unapplicable(self, manifest):
        """A later part that is still a valid patch of the same paths, with
        its manifest row and SHA256SUMS recomputed to agree."""
        row = manifest["parts"][-1]
        part = self.dir / row["file"]
        data = part.read_bytes()
        self.assertIn(b"\n-record line ", data)
        part.write_bytes(data.replace(b"\n-record line ", b"\n-RECORD line ", 1))
        row["bytes"] = part.stat().st_size
        row["sha256"] = patch_set.sha256(part)
        (self.dir / "regenerated.patchset.json").write_text(json.dumps(manifest))
        self.seal()

    def test_a_later_part_that_does_not_apply_is_refused_and_nothing_is_pushed(self):
        self.unapplicable(self.regenerated())
        result = self.deliver()
        self.assert_refused(result, "(?s)HOLD: git apply.*patch does not apply")
        self.assertEqual(self.git("status", "--porcelain", "--untracked-files=all"), b"")

    def test_a_masked_apply_would_dispatch_the_gate_on_the_unregenerated_commit(self):
        """Why the delivery pin refuses `|| true`: the refusal is real, but a
        masked one reaches the next statement, which finds no change and
        dispatches the gate on the commit as if it were regenerated."""
        self.unapplicable(self.regenerated())
        script = (SCRIPTS / "deliver-regenerated.sh").read_text()
        command = WriterCollisionTests.DELIVER
        result = self.deliver(script.replace(command, command + " || true", 1))
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("workflow run ci.yml", self.gh_calls())
        with self.assertRaisesRegex(ValueError, "fail closed"):
            WriterCollisionTests.check_delivery(script.replace(command, command + " || true", 1))

    def test_a_skipped_apply_would_dispatch_the_gate_and_is_refused(self):
        """Review 6077939760, plant 3: the apply line kept but never run. Run
        for real on the caller's branch, the delivery finds no change and
        dispatches the gate on the unregenerated commit; the delivery pin
        refuses the script, so it cannot be the one `deliver` runs."""
        manifest = self.regenerated()
        script = (SCRIPTS / "deliver-regenerated.sh").read_text()
        command = WriterCollisionTests.DELIVER
        planted = script.replace(command + "\n", "if false; then\n" + command + "\nfi\n", 1)
        self.assertNotEqual(planted, script)
        result = self.deliver(planted)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual(self.remote_head(), self.commit)
        self.assertIn("workflow run ci.yml --repo owner/name --ref feature", self.gh_calls())
        self.assertNotEqual(manifest["tree"], manifest["start_tree"])
        with self.assertRaisesRegex(ValueError, "not the reviewed script"):
            WriterCollisionTests.check_delivery(planted)
        # The reviewed script, on the same artifact, delivers the regenerated
        # tree instead.
        self.calls.unlink()
        self.assertEqual(self.deliver().returncode, 0)
        self.assertNotEqual(self.remote_head(), self.commit)


if __name__ == "__main__":
    unittest.main()

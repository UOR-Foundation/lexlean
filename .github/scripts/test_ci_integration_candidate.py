"""Tests of the fork integration-candidate check (CI #603)."""
import copy
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parent))
import ci_integration_candidate as candidate  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
SHA = "a" * 40
PR = {"number": 614, "state": "open",
      "head": {"sha": SHA, "repo": {"full_name": "contributor/LexLean"}},
      "base": {"ref": "release/v0.4", "sha": "b" * 40,
               "repo": {"full_name": "UOR-Foundation/lexlean"}}}


def workflows():
    return {p.name: p.read_text(encoding="utf-8")
            for p in sorted((ROOT / ".github" / "workflows").glob("*.yml"))}


class Check(unittest.TestCase):
    def test_a_reviewed_fork_change_outside_the_workflows_is_eligible(self):
        eligible, reasons, steps = candidate.check(PR, ["crates/lexlean/src/lib.rs"], SHA, workflows())
        self.assertTrue(eligible, reasons)
        self.assertIn(f"git push origin {SHA}:refs/heads/integrate/pr-614", steps)
        self.assertIn(f"gh pr merge 614 --merge --match-head-commit {SHA}", steps)

    def test_each_unsafe_fact_refuses(self):
        def pr_with(**changes):
            pr = copy.deepcopy(PR)
            for key, value in changes.items():
                parent, _, leaf = key.rpartition(".")
                target = pr
                for part in filter(None, parent.split(".")):
                    target = target[part]
                target[leaf] = value
            return pr
        cases = {
            "closed": (pr_with(state="closed"), ["a.rs"], SHA, "closed"),
            "same repository": (pr_with(**{"head.repo": {"full_name": "UOR-Foundation/lexlean"}}),
                                ["a.rs"], SHA, "not a fork"),
            "moved head": (pr_with(**{"head.sha": "c" * 40}), ["a.rs"], SHA, "not the reviewed"),
            "short sha": (PR, ["a.rs"], SHA[:12], "no full reviewed"),
            "workflow change": (PR, [".github/workflows/ci.yml"], SHA, "this repository's trust"),
            "new workflow": (PR, [".github/workflows/x.yml"], SHA, "this repository's trust"),
            "action change": (PR, [".github/actions/read-trace-stop/action.yml"], SHA,
                              "this repository's trust"),
            "no files": (PR, [], SHA, "empty or unread"),
        }
        for name, (pr, files, reviewed, expect) in cases.items():
            with self.subTest(name):
                eligible, reasons, steps = candidate.check(pr, files, reviewed, workflows())
                self.assertFalse(eligible)
                self.assertEqual(steps, [])
                self.assertTrue(any(expect in r for r in reasons), reasons)

    def test_a_branch_a_workflow_pushes_on_is_refused(self):
        catch_all = {"evil.yml": "on:\n  push:\n    branches:\n      - 'integrate/**'\n"}
        eligible, reasons, _ = candidate.check(PR, ["a.rs"], SHA, catch_all)
        self.assertFalse(eligible)
        self.assertIn("would trigger evil.yml", reasons[0])
        bare = {"any.yml": "on:\n  push:\n  pull_request:\n"}
        self.assertFalse(candidate.check(PR, ["a.rs"], SHA, bare)[0])


class Patterns(unittest.TestCase):
    def test_this_repository_s_push_triggers(self):
        found = {name: candidate.push_patterns(text) for name, text in workflows().items()}
        self.assertIn("release/**", found["ci.yml"])
        self.assertIn("cut/v[0-9]*", found["cut.yml"])
        self.assertIn("regenerate/**", found["regenerate.yml"])
        for pattern in (p for patterns in found.values() for p in patterns):
            self.assertIsNone(candidate.glob_regex(pattern).match("integrate/pr-614"), pattern)

    def test_globs(self):
        self.assertTrue(candidate.glob_regex("release/**").match("release/v0.5"))
        self.assertTrue(candidate.glob_regex("cut/v[0-9]*").match("cut/v0.5.1"))
        self.assertIsNone(candidate.glob_regex("cut/v[0-9]*").match("cut/x"))
        self.assertIsNone(candidate.glob_regex("main").match("main2"))


if __name__ == "__main__":
    unittest.main()

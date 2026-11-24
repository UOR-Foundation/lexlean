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


class Indirect(unittest.TestCase):
    """Code the gate runs around the tests must be acknowledged by kind."""

    def test_each_kind_needs_its_acknowledgement(self):
        cases = {
            "dependencies": ["Cargo.lock"],
            "toolchain": ["crates/x/.cargo/config.toml"],
            "build-scripts": ["crates/lexlean/build.rs"],
            "executed-scripts": ["xtask/src/main.rs"],
        }
        for kind, files in cases.items():
            with self.subTest(kind):
                eligible, reasons, _ = candidate.check(PR, files, SHA, workflows())
                self.assertFalse(eligible)
                self.assertTrue(any(f"--acknowledge {kind}" in r for r in reasons), reasons)
                self.assertTrue(candidate.check(PR, files, SHA, workflows(), [kind])[0])

    def test_an_acknowledgement_covers_only_its_kind(self):
        eligible, reasons, _ = candidate.check(
            PR, ["Cargo.lock", "crates/lexlean/build.rs"], SHA, workflows(), ["dependencies"])
        self.assertFalse(eligible)
        self.assertEqual(len(reasons), 1)
        self.assertIn("build-scripts", reasons[0])

    def test_a_privileged_base_workflow_refuses_the_path(self):
        base = workflows()
        cases = {
            "target": "on:\n  pull_request_target:\njobs: {}\n",
            "after a run": "on:\n  workflow_run:\n    workflows: [ci]\njobs: {}\n",
            "write on pull requests": ("on:\n  pull_request:\npermissions:\n  contents: write\n"
                                       "jobs: {}\n"),
            "secret on pull requests": ("on:\n  pull_request:\njobs:\n  a:\n    steps:\n"
                                        "      - run: echo ${{ secrets.DEPLOY_KEY }}\n"),
            "unreadable triggers": "jobs: {}\n",
        }
        for name, text in cases.items():
            with self.subTest(name):
                eligible, reasons, _ = candidate.check(PR, ["a.rs"], SHA, {**base, "x.yml": text})
                self.assertFalse(eligible, reasons)
        token = "on:\n  pull_request:\njobs:\n  a:\n    env: {T: '${{ secrets.GITHUB_TOKEN }}'}\n"
        self.assertTrue(candidate.check(PR, ["a.rs"], SHA, {**base, "x.yml": token})[0])

    def test_this_repository_s_workflows_grant_nothing_more(self):
        for name, text in workflows().items():
            with self.subTest(name):
                self.assertEqual(candidate.elevation(name, text), [])


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

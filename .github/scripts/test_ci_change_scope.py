"""Tests of the phase-2 shadow decision model (CI #603): every fact a reuse
needs, removed or falsified alone, makes the obligation run."""
import copy
from pathlib import Path
import subprocess
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parent))
import ci_change_scope as scope  # noqa: E402

REPO = "UOR-Foundation/lexlean"
BASE = {"commit": "b" * 40, "tree": "t" * 40}
RECORDER = {"ci_read_trace.py": "d1", "read-trace-start.sh": "d2"}
ROW = {"part": "rest"}


def bundle():
    trace = {
        "schema": scope.TRACE_SCHEMA, "authority": scope.TRACE_AUTHORITY, "usable": True,
        "state": "stopped", "files": ["crates/a/src/lib.rs", "Cargo.toml"], "dirs": ["crates"],
        "git": [], "counts": {"input": 2},
        "provenance": {"tree": BASE["tree"], "head": BASE["commit"], "dirty": False, "matrix": ROW,
                       "recorder": RECORDER,
                       "github": {"GITHUB_REPOSITORY": REPO, "GITHUB_RUN_ID": "11",
                                  "GITHUB_RUN_ATTEMPT": "1"}},
        "recorder_at_stop": RECORDER,
    }
    return {
        "repository": REPO, "base": dict(BASE), "candidate": {"commit": "c" * 40, "tree": "u" * 40},
        "change": [{"status": "M", "old_mode": "100644", "new_mode": "100644",
                    "path": "docs/notes.md"}],
        "expected_recorder": RECORDER,
        "obligations": [{"id": "just vv (rest)", "host": "linux", "traced": True, "row": ROW}],
        "receipts": [{"obligation": "just vv (rest)", "run_id": 11, "attempt": 1, "job_id": 5,
                      "conclusion": "success", "head_repository": REPO, "event": "pull_request",
                      "head_sha": BASE["commit"], "tree": BASE["tree"], "trace": trace}],
    }


def decision(b):
    return scope.decide(b)["obligations"][0]


def trace(b):
    return b["receipts"][0]["trace"]


def prov(b):
    return trace(b)["provenance"]


CASES = {
    "added file": (lambda b: b["change"][0].update(status="A"), "pure modification"),
    "deleted file": (lambda b: b["change"][0].update(status="D"), "pure modification"),
    "mode change": (lambda b: b["change"][0].update(new_mode="100755"), "mode"),
    "symlink": (lambda b: b["change"][0].update(old_mode="120000", new_mode="120000"), "mode"),
    "submodule": (lambda b: b["change"][0].update(old_mode="160000", new_mode="160000"), "mode"),
    "workflow": (lambda b: b["change"][0].update(path=".github/workflows/ci.yml"), "control"),
    "lock": (lambda b: b["change"][0].update(path="Cargo.lock"), "control"),
    "nested toolchain": (lambda b: b["change"][0].update(path="sub/rust-toolchain.toml"), "control"),
    "empty change": (lambda b: b.update(change=[]), "no change entries"),
    "same tree": (lambda b: b["candidate"].update(tree=BASE["tree"]), "identical trees"),
    "unknown base": (lambda b: b["base"].pop("tree"), "identity unknown"),
    "native host": (lambda b: b["obligations"][0].update(host="macos"), "native"),
    "untraced": (lambda b: b["obligations"][0].update(traced=False), "untraced"),
    "no receipt": (lambda b: b.update(receipts=[]), "0 receipts"),
    "two receipts": (lambda b: b["receipts"].append(copy.deepcopy(b["receipts"][0])), "2 receipts"),
    "failed receipt": (lambda b: b["receipts"][0].update(conclusion="failure"), "conclusion"),
    "fork receipt": (lambda b: b["receipts"][0].update(head_repository="someone/fork"),
                     "not this repository"),
    "foreign event": (lambda b: b["receipts"][0].update(event="pull_request_target"), "event"),
    "other commit": (lambda b: b["receipts"][0].update(head_sha="e" * 40), "base commit"),
    "other tree": (lambda b: b["receipts"][0].update(tree="e" * 40), "base tree"),
    "no trace": (lambda b: b["receipts"][0].update(trace=None), "absence never authorizes"),
    "old schema": (lambda b: trace(b).update(schema="lexlean/ci-read-trace/1"), "schema"),
    "unusable": (lambda b: trace(b).update(usable=False), "not usable"),
    "not stopped": (lambda b: trace(b).update(state="missing"), "not usable"),
    "no authority statement": (lambda b: trace(b).pop("authority"), "authority"),
    "trace of another tree": (lambda b: prov(b).update(tree="e" * 40), "trace tree"),
    "trace of another commit": (lambda b: prov(b).update(head="e" * 40), "trace commit"),
    "dirty checkout": (lambda b: prov(b).update(dirty=True), "clean"),
    "another matrix row": (lambda b: prov(b).update(matrix={"part": "examples"}), "matrix row"),
    "another run": (lambda b: prov(b)["github"].update(GITHUB_RUN_ID="12"), "trace run"),
    "another attempt": (lambda b: prov(b)["github"].update(GITHUB_RUN_ATTEMPT="2"), "trace attempt"),
    "another repository": (lambda b: prov(b)["github"].update(GITHUB_REPOSITORY="x/y"),
                           "trace repository"),
    "recorder changed": (lambda b: trace(b).update(recorder_at_stop={"x": "y"}), "during the job"),
    "unexpected recorder": (lambda b: b.update(expected_recorder={"x": "y"}), "unexpected recorder"),
    "saw nothing": (lambda b: trace(b).update(files=[], counts={"input": 0}), "no workspace input"),
    "read the changed file": (lambda b: trace(b)["files"].append("docs/notes.md"),
                              "read docs/notes.md"),
    "listed its directory": (lambda b: trace(b)["dirs"].append("docs"), "listed docs"),
    "read .git": (lambda b: trace(b).update(git=[".git/HEAD"]), ".git"),
}


class Decide(unittest.TestCase):
    def test_the_valid_bundle_would_reuse_and_says_it_is_shadow(self):
        result = scope.decide(bundle())
        self.assertEqual(result["obligations"][0]["decision"], scope.REUSE, result)
        self.assertIn("shadow", result["mode"])

    def test_each_missing_or_false_fact_runs_the_obligation(self):
        for name, (mutate, expect) in CASES.items():
            with self.subTest(name):
                b = bundle()
                mutate(b)
                item = decision(b)
                self.assertEqual(item["decision"], scope.RUN, name)
                self.assertTrue(any(expect in reason for reason in item["reasons"]),
                                (name, item["reasons"]))

    def test_a_top_level_file_and_a_listed_root(self):
        b = bundle()
        b["change"][0]["path"] = "README.md"
        trace(b)["dirs"].append(".")
        self.assertIn("listed .", " ".join(decision(b)["reasons"]))


class Score(unittest.TestCase):
    def test_a_would_reuse_that_failed_is_unsound(self):
        result = scope.decide(bundle())
        good = scope.score(result, {"just vv (rest)": ("success", 600)})
        self.assertTrue(good["sound"])
        self.assertEqual(good["would_save_runner_minutes"], 10.0)
        bad = scope.score(result, {"just vv (rest)": ("failure", 600)})
        self.assertEqual((bad["sound"], bad["unsound"]), (False, ["just vv (rest)"]))
        self.assertFalse(scope.score(result, {})["sound"], "an unscored reuse is not sound")


class Diff(unittest.TestCase):
    def test_raw_diff_entries(self):
        text = (":100644 100644 " + "a" * 40 + " " + "b" * 40 + " M\tdocs/x.md\n"
                ":000000 100644 " + "0" * 40 + " " + "c" * 40 + " A\tnew.rs\n")
        self.assertEqual([(e["status"], e["path"]) for e in scope.parse_raw_diff(text)],
                         [("M", "docs/x.md"), ("A", "new.rs")])

    def test_against_this_repository(self):
        root = Path(__file__).resolve().parents[2]

        def run(command, **kwargs):
            return subprocess.run(["git", "-C", str(root), *command[1:]], **kwargs)
        try:
            entries = scope.change_of("HEAD~1", "HEAD", run=run)
        except (subprocess.CalledProcessError, FileNotFoundError):
            self.skipTest("no history in this checkout")
        self.assertTrue(entries)
        self.assertTrue(all(set(e) == {"status", "old_mode", "new_mode", "path"} for e in entries))


if __name__ == "__main__":
    unittest.main()

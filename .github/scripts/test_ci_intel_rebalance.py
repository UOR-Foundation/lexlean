"""Fail-closed adversarial tests for the audit-only Intel lane model."""
import copy
import json
import tempfile
import unittest
from pathlib import Path
import ci_intel_rebalance as sim


def source(path, value):
    text = json.dumps(value, separators=(",", ":")) + "\n"
    return {"path": path, "raw_text": text, "git_blob_sha": sim.blob_sha(text)}


def fixture():
    parts = {"compiler": [f"CF-{i:02d}" for i in range(1, 100)],
             "verification": [f"VR-{i:02d}" for i in range(1, 100)],
             "bootstrap-moves": [f"BM-{i:02d}" for i in range(1, 32)]}
    lanes = {"producer": ["honesty"], "lanes": {"lower": ["compiler"],
             "print": ["verification"], "validate": ["bootstrap-moves"], "moves": ["rest"]}}
    bdd = {"parts": parts, "targets": {"honesty": ["honesty"]}, "remainder": "rest"}
    cases = {k: [sim.case_name(c) for p in ps if p in parts for c in parts[p]]
             for k, ps in lanes["lanes"].items()}
    cases["moves"] = [f"conformance_zz_{i:03d}" for i in range(278)]
    ids = {"lower": 137457635516, "print": 137457635561,
           "validate": 137457635553, "moves": 137457635539}
    return {"schema": "lexlean-intel-historical-lane-v1", "run_id": 45100057759,
            "head_sha": "b693bb4259d311fd71da90ac38701214a74be27f",
            "sources": {"bdd": source(".github/bdd-parts.json", bdd),
                        "lanes": source(".github/intel-host-lanes.json", lanes)},
            "lanes": {k: {"job_id": ids[k], "started_at": "2026-11-23T00:00:00Z",
                          "completed_at": "2026-11-23T01:00:00Z",
                          "elapsed_seconds": 3656 if k == "moves" else 1972,
                          "execution_step_seconds": 3584 if k == "moves" else 1856,
                          "cases_passed": v, "cases_over_60s": v[:1]}
                      for k, v in cases.items()}}


class Audit(unittest.TestCase):
    def setUp(self):
        self.data = fixture()

    def fail_on(self, mutation):
        candidate = copy.deepcopy(self.data)
        mutation(candidate)
        with self.assertRaises((sim.Hold, ValueError, KeyError, TypeError)):
            sim.analyze(candidate)

    def test_positive_fixture_and_invariant(self):
        out = sim.analyze(self.data)
        self.assertEqual(out["observed_remainder_case_count"], 278)
        self.assertEqual(out["observed_case_counts"]["moves"], 278)
        self.assertEqual(out["scenarios"][1]["additional_runner_seconds_vs_existing"], 72)

    def test_missing_case(self):
        self.fail_on(lambda d: d["lanes"]["moves"]["cases_passed"].pop())

    def test_duplicate_case(self):
        self.fail_on(lambda d: d["lanes"]["moves"]["cases_passed"].append(
            d["lanes"]["moves"]["cases_passed"][0]))

    def test_foreign_case(self):
        self.fail_on(lambda d: d["lanes"]["moves"]["cases_passed"].__setitem__(0, "foreign"))

    def test_cross_lane_case(self):
        self.fail_on(lambda d: d["lanes"]["print"]["cases_passed"].append(
            d["lanes"]["lower"]["cases_passed"][0]))

    def test_altered_blob(self):
        self.fail_on(lambda d: d["sources"]["bdd"].__setitem__("git_blob_sha", "a" * 40))

    def test_stale_source_checkout(self):
        with tempfile.TemporaryDirectory() as td:
            base = Path(td)
            for item in self.data["sources"].values():
                p = base / item["path"]
                p.parent.mkdir(parents=True, exist_ok=True)
                p.write_text(item["raw_text"])
            self.assertEqual(sim.validate(self.data, base), self.data["lanes"])
            (base / ".github/bdd-parts.json").write_text("{}")
            with self.assertRaises(sim.Hold):
                sim.validate(self.data, base)

    def test_removal_of_required_part(self):
        def change(d):
            obj = json.loads(d["sources"]["lanes"]["raw_text"])
            obj["lanes"]["print"] = []
            d["sources"]["lanes"] = source(".github/intel-host-lanes.json", obj)
        self.fail_on(change)

    def test_changed_ownership(self):
        def change(d):
            obj = json.loads(d["sources"]["lanes"]["raw_text"])
            obj["lanes"]["print"].append("rest")
            obj["lanes"]["moves"] = []
            d["sources"]["lanes"] = source(".github/intel-host-lanes.json", obj)
        self.fail_on(change)

    def test_unknown_run(self):
        self.fail_on(lambda d: d.__setitem__("run_id", 1))

    def test_forged_job(self):
        self.fail_on(lambda d: d["lanes"]["moves"].__setitem__("job_id", 666))

    def test_invalid_step(self):
        self.fail_on(lambda d: d["lanes"]["moves"].__setitem__("execution_step_seconds", 999999))

    def test_forged_slow_warning(self):
        self.fail_on(lambda d: d["lanes"]["moves"]["cases_over_60s"].append("conformance_not_run"))

    def test_extra_lane(self):
        self.fail_on(lambda d: d["lanes"].__setitem__("unknown", copy.deepcopy(d["lanes"]["moves"])))

    def test_schedule_model_not_free_work(self):
        base = sim.lpt([3656, 1972, 1964, 1395], 3)
        split = sim.lpt([1864, 1864, 1972, 1964, 1395], 3)
        self.assertGreater(split, base)


if __name__ == "__main__":
    unittest.main()

"""No-network cross-source falsifiers for detailed job hotspot attribution."""
import copy
import json
import tempfile
import unittest
from pathlib import Path

import ci_job_hotspots as mod


SHA = "a" * 40


def fixture():
    jobs = {
        "schema": "lexlean/ci-job-detail/1",
        "period": "before", "expected_jobs": 3,
        "jobs": [
            {
                "job_id": 11, "job_name": "atlas (prefix)",
                "run_id": 101, "run_attempt": 1, "workflow": "ci", "pr": 1, "head_sha": SHA,
                "platform": "linux", "conclusion": "success",
                "started_at": "2026-01-01T00:00:00Z",
                "completed_at": "2026-01-01T00:03:00Z",
                "elapsed_minutes": 3.0,
            },
            {
                "job_id": 12, "job_name": "atlas (prefix)",
                "run_id": 101, "run_attempt": 1, "workflow": "ci", "pr": 1, "head_sha": SHA,
                "platform": "linux", "conclusion": "failure",
                "started_at": "2026-01-01T00:01:00Z",
                "completed_at": "2026-01-01T00:08:00Z",
                "elapsed_minutes": 7.0,
            },
            {
                "job_id": 13, "job_name": "repro",
                "run_id": 103, "run_attempt": 1, "workflow": "reproducibility", "pr": 2,
                "head_sha": "b" * 40, "platform": "linux",
                "conclusion": "skipped", "started_at": None,
                "completed_at": None, "elapsed_minutes": None,
            },
        ],
    }
    costs = {
        "schema": "lexlean/ci-pr-job-snapshot/1",
        "run_costs": [
            {
                "id": 101, "pr": 1, "job_count": 2, "returned": 2,
                "complete": True, "job_minutes": 10.0,
                "by_platform_minutes": {"linux": 10.0},
            },
            {
                "id": 102, "pr": 1, "job_count": 0, "returned": 0,
                "complete": True, "job_minutes": 0.0,
                "by_platform_minutes": {},
            },
            {
                "id": 103, "pr": 2, "job_count": 1, "returned": 1,
                "complete": True, "job_minutes": 0.0,
                "by_platform_minutes": {},
            },
        ],
    }
    runs = {
        "schema": "lexlean/ci-pr-baseline/1",
        "rows": [
            {"number": 1, "head_sha": SHA, "runs": [
                {"id": 101, "name": "ci", "head_sha": SHA, "run_attempt": 1},
                {"id": 102, "name": "ci", "head_sha": SHA, "run_attempt": 1},
            ]},
            {"number": 2, "head_sha": "b" * 40, "runs": [
                {"id": 103, "name": "reproducibility",
                 "head_sha": "b" * 40, "run_attempt": 1},
            ]},
        ],
    }
    return jobs, costs, runs


class JobAttributionContracts(unittest.TestCase):
    def setUp(self):
        self.jobs, self.costs, self.runs = fixture()

    def test_hotspot_observations_reconcile_with_zero_job_runs(self):
        result = mod.analyze(self.jobs, self.costs, self.runs)
        self.assertEqual(result["classification"],
                         "OBSERVED_JOB_ELAPSED_ONLY_NOT_BILLING_OR_CUTOVER_EVIDENCE")
        self.assertEqual(result["job_rows"], 3)
        self.assertEqual(result["run_count"], 3)
        self.assertEqual(result["observed_job_minutes"], 10)
        group = result["job_groups"][0]
        self.assertEqual(group["job"], "atlas (prefix)")
        self.assertEqual(group["observed_job_minutes"], 10)
        self.assertEqual(group["median_job_minutes"], 5)
        self.assertEqual(group["failed"], 1)
        self.assertEqual(group["p90_job_minutes"], 6.6)
        self.assertEqual(next(x for x in result["run_envelopes"]
                              if x["run_id"] == 101)["overlap_ratio"], 1.25)
        self.assertIsNone(next(x for x in result["run_envelopes"]
                               if x["run_id"] == 102)["executed_job_envelope_minutes"])
        self.assertIsNone(next(x for x in result["job_groups"] if x["job"] == "repro")
                          ["median_job_minutes"])

    def test_aggregate_only_historical_snapshot_cannot_invent_hotspots(self):
        with self.assertRaisesRegex(ValueError, "real job-level snapshot"):
            mod.analyze(self.costs, self.costs, self.runs)

    def test_missing_detailed_job_is_hold(self):
        self.jobs["jobs"].pop()
        with self.assertRaisesRegex(ValueError, "truncated"):
            mod.analyze(self.jobs, self.costs, self.runs)

    def test_repeated_id_is_hold(self):
        self.jobs["jobs"][1]["job_id"] = 11
        with self.assertRaisesRegex(ValueError, "duplicate/invalid"):
            mod.analyze(self.jobs, self.costs, self.runs)

    def test_forged_duration_is_hold(self):
        self.jobs["jobs"][0]["elapsed_minutes"] = 1.0
        with self.assertRaisesRegex(ValueError, "does not match"):
            mod.analyze(self.jobs, self.costs, self.runs)

    def test_mismatched_platform_is_hold(self):
        self.jobs["jobs"][1]["platform"] = "windows"
        with self.assertRaisesRegex(ValueError, "per-platform"):
            mod.analyze(self.jobs, self.costs, self.runs)

    def test_cross_head_job_provenance_is_hold(self):
        self.jobs["jobs"][0]["head_sha"] = "f" * 40
        with self.assertRaisesRegex(ValueError, "exact-head"):
            mod.analyze(self.jobs, self.costs, self.runs)

    def test_wrong_attempt_provenance_is_hold(self):
        self.jobs["jobs"][0]["run_attempt"] = 3
        with self.assertRaisesRegex(ValueError, "attempt provenance"):
            mod.analyze(self.jobs, self.costs, self.runs)

    def test_cost_run_without_head_provenance_is_hold(self):
        self.runs["rows"][0]["runs"].pop()
        with self.assertRaisesRegex(ValueError, "cost run not matched"):
            mod.analyze(self.jobs, self.costs, self.runs)

    def test_unattributed_run_is_hold(self):
        self.jobs["jobs"][1]["run_id"] = 999
        with self.assertRaisesRegex(ValueError, "unmatched run"):
            mod.analyze(self.jobs, self.costs, self.runs)

    def test_missing_executed_timestamp_is_hold(self):
        self.jobs["jobs"][0]["completed_at"] = None
        with self.assertRaisesRegex(ValueError, "unaccounted"):
            mod.analyze(self.jobs, self.costs, self.runs)

    def test_job_count_tamper_is_hold(self):
        self.costs["run_costs"][0]["job_count"] = 3
        self.costs["run_costs"][0]["returned"] = 3
        with self.assertRaisesRegex(ValueError, "detail count"):
            mod.analyze(self.jobs, self.costs, self.runs)

    def test_partial_cost_run_is_hold(self):
        self.costs["run_costs"][0]["complete"] = False
        with self.assertRaisesRegex(ValueError, "incomplete"):
            mod.analyze(self.jobs, self.costs, self.runs)

    def test_load_rejects_symlinks(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root/"source.json"
            source.write_text(json.dumps(self.jobs))
            link = root/"linked.json"
            link.symlink_to(source)
            with self.assertRaisesRegex(ValueError, "linked"):
                mod.read_snapshot(link)

    def test_source_has_no_api_calls_or_ci_dispatch(self):
        code = Path(mod.__file__).read_text()
        for prohibited in ("urlopen(", "gh workflow run", "subprocess.run(", "http.client"):
            self.assertNotIn(prohibited, code)


if __name__ == "__main__":
    unittest.main()

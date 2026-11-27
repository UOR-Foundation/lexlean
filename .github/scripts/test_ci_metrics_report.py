"""Synthetic and captured-baseline falsifiers for CI Actions economics charts."""
import copy
import importlib.util
import json
import tempfile
import unittest
import xml.etree.ElementTree as ET
from pathlib import Path

PATH = Path(__file__).with_name("ci_metrics_report.py")
spec = importlib.util.spec_from_file_location("ci_metrics_report", PATH)
metrics = importlib.util.module_from_spec(spec)
spec.loader.exec_module(metrics)

ROOT = Path(__file__).parents[1]
RUNS = ROOT / "metrics" / "ci-before-pr50-runs.json"
COSTS = ROOT / "metrics" / "ci-before-pr50-job-costs-corrected.json"
A = "a" * 40
B = "b" * 40


def fixture():
    p1 = {
        "number": 1, "created_at": "2026-01-01T00:00:00Z",
        "merged_at": "2026-01-01T02:00:00Z", "head_sha": A,
        "base_branch": "release/v0.4", "run_list_complete": True,
        "run_total_reported": 2,
        "runs": [
            {"id": 11, "name": "ci", "event": "pull_request", "head_sha": A,
             "conclusion": "success", "created_at": "2026-01-01T00:05:00Z",
             "updated_at": "2026-01-01T01:00:00Z"},
            {"id": 12, "name": "reproducibility", "event": "pull_request",
             "head_sha": A, "conclusion": "success",
             "created_at": "2026-01-01T00:05:00Z",
             "updated_at": "2026-01-01T00:15:00Z"},
        ],
    }
    p2 = {
        "number": 2, "created_at": "2026-01-02T00:00:00Z",
        "merged_at": "2026-01-02T00:30:00Z", "head_sha": B,
        "base_branch": "main", "run_list_complete": True,
        "run_total_reported": 2,
        "runs": [
            {"id": 21, "name": "ci", "event": "pull_request", "head_sha": B,
             "conclusion": "success", "created_at": "2026-01-02T00:05:00Z",
             "updated_at": "2026-01-02T01:00:00Z"},
            {"id": 22, "name": "reproducibility", "event": "pull_request",
             "head_sha": B, "conclusion": "success",
             "created_at": "2026-01-02T00:05:00Z",
             "updated_at": "2026-01-02T00:10:00Z"},
        ],
    }
    runs = {"schema": metrics.RUN_SCHEMA, "rows": [p1, p2]}
    values = [(11, 60, "linux", 1), (12, 10, "linux", 1),
              (21, 120, "macos", 2), (22, 5, "windows", 2)]
    costs = {"schema": metrics.COST_SCHEMA, "run_costs": [
        {"id": i, "pr": n, "complete": True, "job_count": 1,
         "returned": 1, "job_minutes": duration,
         "by_platform_minutes": {os: duration}}
        for i, duration, os, n in values]}
    return runs, costs


class ReportContracts(unittest.TestCase):
    def test_actual_snapshots_have_50_prs_and_no_invented_after(self):
        prs, summary = metrics.summarize(metrics.load(RUNS), metrics.load(COSTS), "before")
        self.assertEqual(len(prs), 50)
        self.assertEqual(summary["workflow_run_count"], 128)
        self.assertEqual(summary["pr_with_premerge_green"], 29)
        self.assertAlmostEqual(summary["observed_runner_job_minutes_total"], 88298.35, delta=.1)
        self.assertIsNone(summary["billing_minutes"])
        self.assertAlmostEqual(summary["estimated_private_equivalent_usd_total"], 1249.19, delta=0.15)
        self.assertTrue(all(x["period"] == "before" for x in prs))

    def test_premerge_qualification_is_not_merge_time(self):
        runs, costs = fixture()
        prs, summary = metrics.summarize(runs, costs, "before")
        self.assertEqual(prs[0]["final_head_time_to_green_minutes"], 55)
        self.assertEqual(prs[0]["merge_after_green_minutes"], 60)
        self.assertIsNone(prs[1]["final_head_time_to_green_minutes"])
        self.assertEqual(summary["pr_with_premerge_green"], 1)
        self.assertEqual(summary["pr_without_proven_premerge_green"], 1)

    def test_total_is_per_run_job_minutes_not_workflow_wall(self):
        runs, costs = fixture()
        prs, summary = metrics.summarize(runs, costs, "before")
        self.assertEqual(summary["observed_runner_job_minutes_total"], 195)
        self.assertEqual(summary["platform_runner_minutes"]["macos"], 120)
        self.assertAlmostEqual(summary["estimated_private_equivalent_usd_total"], 7.91)
        self.assertAlmostEqual(prs[0]["estimated_private_equivalent_usd"], .42)
        self.assertAlmostEqual(prs[1]["estimated_private_equivalent_usd"], 7.49)

    def test_missing_job_cost_refused_not_interpreted_as_zero(self):
        runs, costs = fixture()
        costs["run_costs"].pop()
        with self.assertRaisesRegex(ValueError, "missing or foreign"):
            metrics.summarize(runs, costs, "before")

    def test_incomplete_job_page_refused(self):
        runs, costs = fixture()
        costs["run_costs"][0]["complete"] = False
        with self.assertRaisesRegex(ValueError, "partial"):
            metrics.summarize(runs, costs, "before")

    def test_truncated_run_inventory_refused(self):
        runs, costs = fixture()
        runs["rows"][0]["run_total_reported"] = 4
        with self.assertRaisesRegex(ValueError, "truncated"):
            metrics.summarize(runs, costs, "before")

    def test_wrong_head_or_foreign_pr_refused(self):
        runs, costs = fixture()
        runs["rows"][0]["runs"][0]["head_sha"] = B
        with self.assertRaisesRegex(ValueError, "foreign"):
            metrics.summarize(runs, costs, "before")
        runs, costs = fixture()
        costs["run_costs"][0]["pr"] = 3
        with self.assertRaisesRegex(ValueError, "different PR"):
            metrics.summarize(runs, costs, "before")

    def test_duplicate_cost_or_pr_refused(self):
        runs, costs = fixture()
        costs["run_costs"].append(copy.deepcopy(costs["run_costs"][0]))
        with self.assertRaisesRegex(ValueError, "duplicate run cost"):
            metrics.summarize(runs, costs, "before")
        runs, costs = fixture()
        runs["rows"][1]["number"] = 1
        with self.assertRaisesRegex(ValueError, "duplicate or invalid PR"):
            metrics.summarize(runs, costs, "before")

    def test_inconsistent_platform_cost_refused(self):
        runs, costs = fixture()
        costs["run_costs"][0]["by_platform_minutes"]["linux"] = 7
        with self.assertRaisesRegex(ValueError, "platform sum disagree"):
            metrics.summarize(runs, costs, "before")

    def test_missing_after_generates_na_not_zero_in_svg_and_summary(self):
        runs, costs = fixture()
        rows, summary = metrics.summarize(runs, costs, "before")
        with tempfile.TemporaryDirectory() as temp:
            result = metrics.write_report(rows, summary, [], None, Path(temp))
            self.assertIsNone(result["after"])
            self.assertEqual(result["cutover_effect_status"], "NOT_MEASURED")
            svg = Path(temp, "runner-minutes-per-pr.svg").read_text()
            ET.fromstring(svg)
            self.assertIn("AWAITING VERIFIED POST-CUTOVER PRS", svg)
            self.assertIn("No zero-minute runs or estimated savings", svg)
            ET.fromstring(Path(temp, "final-head-time-to-green.svg").read_text())
            cost_svg = Path(temp, "private-equivalent-cost-per-pr.svg").read_text()
            ET.fromstring(cost_svg)
            self.assertIn("AWAITING VERIFIED POST-CUTOVER PRS", cost_svg)
            self.assertIn("Estimated private-repo compute equivalent", Path(temp, "README.md").read_text())
            self.assertIn("NOT YET OBSERVED", Path(temp, "README.md").read_text())

    def test_after_merges_cannot_pretend_to_predate_cutover(self):
        runs, costs = fixture()
        _, summary = metrics.summarize(runs, costs, "after")
        self.assertEqual(summary["pr_count"], 2)
        self.assertIsNone(summary["billing_minutes"])

    def test_nonfinite_and_negative_job_minutes_refused(self):
        for value in (-4, float("nan"), float("inf")):
            runs, costs = fixture()
            costs["run_costs"][0]["job_minutes"] = value
            with self.subTest(value=value):
                with self.assertRaisesRegex(ValueError, "invalid runner duration"):
                    metrics.summarize(runs, costs, "before")


if __name__ == "__main__":
    unittest.main()

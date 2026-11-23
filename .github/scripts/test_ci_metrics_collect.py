"""No-network, read-only bounded collection tests for future CI cutover cohorts."""
import copy
import importlib.util
import tempfile
import unittest
from datetime import datetime, timezone
from pathlib import Path

SCRIPT = Path(__file__).with_name("ci_metrics_collect.py")
spec = importlib.util.spec_from_file_location("collector", SCRIPT)
collector = importlib.util.module_from_spec(spec)
spec.loader.exec_module(collector)
B = datetime(2026, 11, 23, 3, tzinfo=timezone.utc)
SHA1 = "a" * 40
SHA2 = "b" * 40


def pr(number, merged, head, updated):
    return {
        "number": number, "created_at": "2026-11-20T00:00:00Z",
        "merged_at": merged, "updated_at": updated,
        "head": {"sha": head, "ref": "test"},
        "base": {"ref": "release/v0.4"},
    }


class FakeReader(collector.Reader):
    def __init__(self):
        super().__init__("UOR-Foundation/lexlean", "TEST_TOKEN_NOT_SENT", 20)
        self.calls = []
        self.data = {
            ("pulls", 1): [
                pr(2, "2026-11-23T04:00:00Z", SHA2, "2026-11-23T04:10:00Z"),
                pr(1, "2026-11-23T02:00:00Z", SHA1, "2026-11-23T02:10:00Z"),
            ],
            ("actions/runs", SHA2): [{
                "id": 202, "name": "ci", "event": "pull_request",
                "conclusion": "success", "status": "completed",
                "head_sha": SHA2, "created_at": "2026-11-23T04:01:00Z",
                "run_started_at": "2026-11-23T04:01:00Z",
                "updated_at": "2026-11-23T04:05:00Z", "run_attempt": 1,
            }],
            ("actions/runs", SHA1): [{
                "id": 101, "name": "ci", "event": "pull_request",
                "conclusion": "success", "status": "completed",
                "head_sha": SHA1, "created_at": "2026-11-23T02:01:00Z",
                "run_started_at": "2026-11-23T02:01:00Z",
                "updated_at": "2026-11-23T02:05:00Z", "run_attempt": 1,
            }],
            ("actions/runs/202/jobs", None): [{
                "started_at": "2026-11-23T04:02:00Z",
                "completed_at": "2026-11-23T04:04:00Z",
                "labels": ["ubuntu-24.04"], "conclusion": "success",
            }],
            ("actions/runs/101/jobs", None): [{
                "started_at": "2026-11-23T02:02:00Z",
                "completed_at": "2026-11-23T02:04:00Z",
                "labels": ["macos-15"], "conclusion": "success",
            }],
        }

    def get(self, endpoint, **params):
        self.calls.append((endpoint, params))
        if endpoint == "pulls":
            return self.data.get(("pulls", params.get("page")), [])
        if endpoint == "actions/runs":
            items = self.data.get(("actions/runs", params["head_sha"]), [])
        else:
            items = self.data.get((endpoint, None), [])
        return {"total_count": len(items),
                "workflow_runs" if endpoint == "actions/runs" else "jobs": items}


class CollectorContracts(unittest.TestCase):
    def test_actual_source_has_no_http_mutations_or_background_loop(self):
        text = SCRIPT.read_text()
        self.assertIn('method="GET"', text)
        self.assertNotIn('method="POST"', text)
        self.assertNotIn('method="PATCH"', text)
        self.assertNotIn('time.sleep', text)
        self.assertIn("API call budget exceeded", text)

    def test_cutover_selects_only_after_and_before(self):
        reader = FakeReader()
        after = reader.pr_candidates("after", B, 50)
        before = reader.pr_candidates("before", B, 50)
        self.assertEqual([x["number"] for x in after], [2])
        self.assertEqual([x["number"] for x in before], [1])

    def test_after_capture_exact_head_and_real_duration(self):
        reader = FakeReader()
        run, cost = reader.capture("after", B, 1)
        self.assertEqual([x["number"] for x in run["rows"]], [2])
        self.assertEqual(cost["run_costs"][0]["id"], 202)
        self.assertEqual(cost["run_costs"][0]["job_minutes"], 2)
        self.assertEqual(cost["run_costs"][0]["by_platform_minutes"], {"linux": 2})
        self.assertTrue(all(call[0] != "POST" for call in reader.calls))

    def test_before_capture_uses_macos_label(self):
        reader = FakeReader()
        runs, costs = reader.capture("before", B, 1)
        self.assertEqual(runs["rows"][0]["number"], 1)
        self.assertEqual(costs["run_costs"][0]["by_platform_minutes"], {"macos": 2})

    def test_before_requires_full_cohort(self):
        reader = FakeReader()
        with self.assertRaisesRegex(ValueError, "not enough prior"):
            reader.capture("before", B, 2)

    def test_in_progress_run_refused(self):
        reader = FakeReader()
        reader.data[("actions/runs", SHA2)][0]["status"] = "in_progress"
        with self.assertRaisesRegex(ValueError, "in progress"):
            reader.capture("after", B, 1)

    def test_cross_head_run_refused(self):
        reader = FakeReader()
        reader.data[("actions/runs", SHA2)][0]["head_sha"] = SHA1
        with self.assertRaisesRegex(ValueError, "not bound"):
            reader.capture("after", B, 1)

    def test_invalid_repo_token_and_budget_refused(self):
        for repo, token, n in (
            ("bad//repo", "X", 5), ("owner/repo", "", 5),
            ("owner/repo", "X", 0), ("owner/repo", "X", 99999),
        ):
            with self.subTest(repo=repo, n=n):
                with self.assertRaises(ValueError):
                    collector.Reader(repo, token, n)

    def test_no_postcutover_merges_yet_means_no_fictitious_after(self):
        reader = FakeReader()
        boundary = datetime(2027, 1, 1, tzinfo=timezone.utc)
        with self.assertRaisesRegex(ValueError, "no merged PRs"):
            reader.capture("after", boundary, 1)

    def test_run_collection_count_mismatch_refused(self):
        reader = FakeReader()
        reader.data[("actions/runs", SHA2)] = []
        out, costs = reader.capture("after", B, 1)
        self.assertEqual(out["rows"][0]["runs"], [])
        self.assertEqual(costs["run_costs"], [])


if __name__ == "__main__":
    unittest.main()

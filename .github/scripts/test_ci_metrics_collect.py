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
                "id": 20201, "name": "Atlas shard (example)",
                "started_at": "2026-11-23T04:02:00Z",
                "completed_at": "2026-11-23T04:04:00Z",
                "labels": ["ubuntu-24.04"], "conclusion": "success",
            }],
            ("actions/runs/101/jobs", None): [{
                "id": 10101, "name": "Windows smoke (fixture)",
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

    def test_optional_job_receipts_add_no_network_requests(self):
        default = FakeReader()
        default.capture("after", B, 1)
        opt = FakeReader()
        runs, costs = opt.capture("after", B, 1, include_jobs=True)
        self.assertEqual(opt.calls, default.calls)
        ledger = opt.job_details
        self.assertEqual(ledger["schema"], collector.JOB_SCHEMA)
        self.assertEqual(ledger["expected_jobs"], 1)
        self.assertEqual(ledger["jobs"][0]["job_id"], 20201)
        self.assertEqual(ledger["jobs"][0]["workflow"], "ci")
        self.assertEqual(ledger["jobs"][0]["elapsed_minutes"], 2.0)
        self.assertEqual(ledger["jobs"][0]["head_sha"], SHA2)
        self.assertEqual(runs["rows"][0]["head_sha"], SHA2)
        self.assertEqual(costs["run_costs"][0]["job_minutes"], 2.0)
        self.assertIsNone(default.job_details)

    def test_skipped_job_preserved_without_invented_zero_duration(self):
        reader = FakeReader()
        reader.data[("actions/runs/202/jobs", None)].append({
            "id": 20202, "name": "Skipped source worker",
            "labels": ["ubuntu-24.04"], "conclusion": "skipped",
            "started_at": None, "completed_at": None,
        })
        _, costs = reader.capture("after", B, 1, include_jobs=True)
        self.assertEqual(costs["run_costs"][0]["job_count"], 2)
        self.assertEqual(reader.job_details["expected_jobs"], 2)
        skipped = reader.job_details["jobs"][-1]
        self.assertIsNone(skipped["elapsed_minutes"])
        self.assertEqual(skipped["conclusion"], "skipped")

    def test_duplicate_provider_job_id_is_hold(self):
        reader = FakeReader()
        item = copy.deepcopy(reader.data[("actions/runs/202/jobs", None)][0])
        reader.data[("actions/runs/202/jobs", None)].append(item)
        with self.assertRaisesRegex(ValueError, "provider job id"):
            reader.capture("after", B, 1, include_jobs=True)

    def test_missing_provider_job_identity_is_hold(self):
        reader = FakeReader()
        del reader.data[("actions/runs/202/jobs", None)][0]["name"]
        with self.assertRaisesRegex(ValueError, "provider job name"):
            reader.capture("after", B, 1, include_jobs=True)

    def test_partial_timing_is_hold(self):
        reader = FakeReader()
        reader.data[("actions/runs/202/jobs", None)][0]["completed_at"] = None
        with self.assertRaisesRegex(ValueError, "partial job timestamps"):
            reader.capture("after", B, 1, include_jobs=True)

    def test_bad_runner_labels_are_hold(self):
        reader = FakeReader()
        reader.data[("actions/runs/202/jobs", None)][0]["labels"] = "windows-2022"
        with self.assertRaisesRegex(ValueError, "runner labels"):
            reader.capture("after", B, 1, include_jobs=True)

    def test_skipped_negative_timestamps_are_not_job_cost(self):
        reader = FakeReader()
        reader.data[("actions/runs/202/jobs", None)].append({
            "id": 20213, "name": "unallocated skip", "conclusion": "skipped",
            "started_at": "2026-11-23T04:08:00Z",
            "completed_at": "2026-11-23T04:07:59Z",
            "labels": ["ubuntu-24.04"], "runner_id": None, "steps": []})
        _, costs = reader.capture("after", B, 1, include_jobs=True)
        self.assertEqual(costs["run_costs"][0]["job_minutes"], 2)
        self.assertEqual(costs["run_costs"][0]["not_executed_skipped_jobs"], 1)
        record = reader.job_details["jobs"][-1]
        self.assertIsNone(record["elapsed_minutes"])
        self.assertIsNone(record["started_at"])

    def test_cancelled_without_runner_or_steps_never_spends_runner_time(self):
        reader = FakeReader()
        reader.data[("actions/runs/202/jobs", None)].append({
            "id": 20214, "name": "queued cancellation", "conclusion": "cancelled",
            "started_at": "2026-11-23T04:02:00Z",
            "completed_at": "2026-11-23T04:28:31Z",
            "labels": ["ubuntu-24.04"], "runner_id": None,
            "runner_name": "", "steps": []})
        _, costs = reader.capture("after", B, 1, include_jobs=True)
        self.assertEqual(costs["run_costs"][0]["job_minutes"], 2)
        self.assertEqual(costs["run_costs"][0]["not_executed_cancelled_jobs"], 1)
        self.assertIsNone(reader.job_details["jobs"][-1]["elapsed_minutes"])

    def test_cancelled_with_actual_worker_is_counted(self):
        reader = FakeReader()
        reader.data[("actions/runs/202/jobs", None)].append({
            "id": 20215, "name": "assigned cancellation", "conclusion": "cancelled",
            "started_at": "2026-11-23T04:02:00Z",
            "completed_at": "2026-11-23T04:07:00Z",
            "labels": ["ubuntu-24.04"], "runner_id": 101,
            "runner_name": "runner-101", "steps": [{"name": "cargo"}]})
        _, costs = reader.capture("after", B, 1)
        self.assertEqual(costs["run_costs"][0]["job_minutes"], 7)
        self.assertEqual(costs["run_costs"][0]["not_executed_cancelled_jobs"], 0)

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

    @staticmethod
    def retry_fixture():
        reader = FakeReader()
        source = reader.data[("actions/runs", SHA2)][0]
        source["run_attempt"] = 2
        baseline = copy.deepcopy(reader.data[("actions/runs/202/jobs", None)][0])
        baseline.update(id=20201, run_attempt=1, status="completed",
                        created_at="2026-11-23T04:01:00Z")
        arm_failure = {"id": 20202, "name": "Linux ARM", "run_attempt": 1,
                       "status": "completed", "conclusion": "failure",
                       "labels": ["ubuntu-24.04-arm"],
                       "created_at": "2026-11-23T04:01:00Z",
                       "started_at": "2026-11-23T04:02:00Z",
                       "completed_at": "2026-11-23T04:07:00Z"}
        carried = copy.deepcopy(baseline)
        carried.update(id=20211, run_attempt=2,
                       created_at="2026-11-23T04:09:00Z")
        arm_retry = copy.deepcopy(arm_failure)
        arm_retry.update(id=20212, run_attempt=2, conclusion="success",
                         created_at="2026-11-23T04:09:00Z",
                         started_at="2026-11-23T04:10:00Z",
                         completed_at="2026-11-23T04:16:00Z")
        reader.data[("actions/runs/202/jobs", None)] = [
            baseline, arm_failure, carried, arm_retry]
        return reader

    def test_rerun_carried_success_does_not_inflate_job_minutes(self):
        reader = self.retry_fixture()
        _, costs = reader.capture("after", B, 1, include_jobs=True)
        cost = costs["run_costs"][0]
        self.assertEqual(cost["provider_job_records"], 4)
        self.assertEqual(cost["carried_reissued_records"], 1)
        self.assertEqual(cost["job_count"], 3)
        self.assertEqual(cost["job_minutes"], 13)
        self.assertEqual(cost["run_attempts"], 2)
        self.assertEqual(reader.job_details["expected_jobs"], 3)
        self.assertEqual(sorted(x["execution_attempt"] for x in
                                reader.job_details["jobs"]), [1, 1, 2])
        self.assertFalse(any("/attempts/" in name for name, _ in reader.calls))

    def test_missing_prior_attempt_requires_read_only_zero_job_proof(self):
        reader = self.retry_fixture()
        only_retry = reader.data[("actions/runs/202/jobs", None)][-1]
        reader.data[("actions/runs/202/jobs", None)] = [only_retry]
        _, costs = reader.capture("after", B, 1)
        self.assertEqual(costs["run_costs"][0]["verified_zero_job_attempts"], [1])
        self.assertNotIn(("actions/runs/202/attempts/1/jobs", None), reader.data)
        self.assertTrue(any("/attempts/1/jobs" in name for name, _ in reader.calls))

    def test_missing_prior_nonempty_attempt_fails_closed(self):
        reader = self.retry_fixture()
        only_retry = reader.data[("actions/runs/202/jobs", None)][-1]
        old = reader.data[("actions/runs/202/jobs", None)][0]
        reader.data[("actions/runs/202/jobs", None)] = [only_retry]
        reader.data[("actions/runs/202/attempts/1/jobs", None)] = [old]
        with self.assertRaisesRegex(ValueError, "missing nonempty historical"):
            reader.capture("after", B, 1)

    def test_conflicting_reissued_host_fails_cost_accounting(self):
        reader = self.retry_fixture()
        reader.data[("actions/runs/202/jobs", None)][2]["labels"] = ["windows-2022"]
        with self.assertRaisesRegex(ValueError, "conflicting result or host"):
            reader.capture("after", B, 1)


if __name__ == "__main__":
    unittest.main()

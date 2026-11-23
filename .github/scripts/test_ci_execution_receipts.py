"""Adversarial provider retry-cost attribution: no virtual runner spending."""
import copy
import unittest
from ci_execution_receipts import canonical_executions


def job(name, jid, attempt, start, end, verdict="success",
        created="2026-11-23T01:00:00Z", labels=None):
    return {"id": jid, "name": name, "run_attempt": attempt, "status": "completed",
            "conclusion": verdict, "started_at": start, "completed_at": end,
            "created_at": created, "labels": ["ubuntu-24.04"] if labels is None else labels}


def fixture():
    old_proof = job("proof", 1, 1, "2026-11-23T01:01:00Z",
                    "2026-11-23T01:04:00Z")
    old_arm = job("ARM", 2, 1, "2026-11-23T01:01:00Z",
                  "2026-11-23T01:06:00Z", verdict="failure")
    copy_proof = job("proof", 11, 2, old_proof["started_at"], old_proof["completed_at"],
                     created="2026-11-23T02:00:00Z")
    new_arm = job("ARM", 12, 2, "2026-11-23T02:01:00Z",
                  "2026-11-23T02:09:00Z", created="2026-11-23T02:00:00Z")
    return [old_proof, old_arm, copy_proof, new_arm]


class ExecutionReceiptsTest(unittest.TestCase):
    def test_reissued_success_is_not_new_compute(self):
        chosen, audit = canonical_executions(fixture(), 2)
        self.assertEqual([r["id"] for r in chosen], [2, 1, 12])
        self.assertEqual(audit["unique_execution_records"], 3)
        self.assertEqual(audit["reissued_carried_records"], 1)

    def test_original_failed_attempt_is_still_counted(self):
        chosen, _ = canonical_executions(fixture(), 2)
        durations = [(int(r["completed_at"][14:16]) -
                      int(r["started_at"][14:16])) for r in chosen]
        self.assertEqual(sorted(durations), [3, 5, 8])
        self.assertEqual(sum(durations), 16)

    def test_real_completed_provider_attempts_not_double_counted(self):
        import json
        from pathlib import Path
        path = Path(__file__).resolve().parents[1] / "metrics" / "ci-retry-execution-receipts-20261123.json"
        if not path.is_file():
            self.skipTest("complete checkout required")
        captured = json.loads(path.read_text())
        self.assertEqual(captured["run_conclusion"], "success")
        self.assertEqual(captured["head_sha"], "f57650e6cf1fd04df0858d7bb97b1071a1c83220")
        chosen, facts = canonical_executions(captured["jobs"], captured["run_attempt"])
        self.assertEqual(facts["provider_job_records"], 156)
        self.assertEqual(facts["unique_execution_records"], 79)
        self.assertEqual(facts["reissued_carried_records"], 77)
        actual = sum((__import__("datetime").datetime.fromisoformat(x["completed_at"].replace("Z", "+00:00")) -
                      __import__("datetime").datetime.fromisoformat(x["started_at"].replace("Z", "+00:00"))).total_seconds() / 60
                     for x in chosen)
        self.assertAlmostEqual(actual, 1233.25, places=3)
        self.assertEqual(captured["independent_repro_conclusion"], "success")

    def test_single_attempt_no_new_api_required(self):
        chosen, evidence = canonical_executions(fixture()[:2], 1)
        self.assertEqual(len(chosen), 2)
        self.assertEqual(evidence["reissued_carried_records"], 0)

    def test_missing_attempt_refused_without_proof(self):
        with self.assertRaisesRegex(ValueError, "unverified missing attempt"):
            canonical_executions([fixture()[-1]], 2)

    def test_zero_job_earlier_attempt_requires_proof(self):
        rows = [job("repro", 5, 2, "2026-11-23T02:01:00Z",
                    "2026-11-23T02:02:00Z", created="2026-11-23T02:00:00Z")]
        chosen, facts = canonical_executions(rows, 2, proven_empty_attempts=(1,))
        self.assertEqual(len(chosen), 1)
        self.assertEqual(facts["verified_empty_attempts"], [1])

    def test_unproven_empty_attempt_rejected(self):
        with self.assertRaisesRegex(ValueError, "both empty and populated"):
            canonical_executions(fixture(), 2, proven_empty_attempts=(1,))

    def test_duplicate_name_within_attempt_is_not_silently_collapsed(self):
        rows = fixture()[:2]
        rows[1]["name"] = "proof"
        with self.assertRaisesRegex(ValueError, "duplicate job name"):
            canonical_executions(rows, 1)

    def test_duplicate_provider_id_refused(self):
        rows = fixture()
        rows[2]["id"] = rows[0]["id"]
        with self.assertRaisesRegex(ValueError, "duplicate/invalid"):
            canonical_executions(rows, 2)

    def test_same_duration_changed_outcome_is_not_reuse(self):
        rows = fixture()
        rows[2]["conclusion"] = "failure"
        with self.assertRaisesRegex(ValueError, "conflicting result"):
            canonical_executions(rows, 2)

    def test_same_duration_changed_host_is_not_reuse(self):
        rows = fixture()
        rows[2]["labels"] = ["windows-2022"]
        with self.assertRaisesRegex(ValueError, "conflicting result or host"):
            canonical_executions(rows, 2)

    def test_overlapping_retry_window_refused(self):
        rows = fixture()
        rows[3]["started_at"] = "2026-11-23T01:03:00Z"
        with self.assertRaisesRegex(ValueError, "overlapping execution"):
            canonical_executions(rows, 2)

    def test_reissued_creation_before_old_end_refused(self):
        rows = fixture()
        rows[2]["created_at"] = "2026-11-23T01:02:00Z"
        with self.assertRaisesRegex(ValueError, "predates original"):
            canonical_executions(rows, 2)

    def test_missing_job_duration_fails(self):
        rows = fixture()
        rows[0]["completed_at"] = None
        with self.assertRaisesRegex(ValueError, "partial execution"):
            canonical_executions(rows, 2)

    def test_skipped_job_with_no_runner_window(self):
        before = job("no-run", 7, 1, None, None, verdict="skipped")
        after = job("no-run", 17, 2, None, None, verdict="skipped",
                    created="2026-11-23T02:00:00Z")
        chosen, info = canonical_executions([before, after], 2)
        self.assertEqual(len(chosen), 1)
        self.assertEqual(info["reissued_carried_records"], 1)

    def test_skipped_negative_provider_timestamps_do_not_create_cost(self):
        from ci_execution_receipts import runtime_window
        before = job("skipped-arm", 101, 1, "2026-11-23T01:10:00Z",
                     "2026-11-23T01:09:59Z", verdict="skipped")
        after = job("skipped-arm", 102, 2, "2026-11-23T02:10:00Z",
                    "2026-11-23T01:09:59Z", verdict="skipped",
                    created="2026-11-23T02:00:00Z")
        self.assertIsNone(runtime_window(before))
        chosen, evidence = canonical_executions([before, after], 2)
        self.assertEqual((len(chosen), evidence["reissued_carried_records"]), (1, 1))

    def test_unallocated_cancelled_work_has_no_runner_window(self):
        from ci_execution_receipts import runtime_window
        row = job("unallocated", 103, 1, "2026-11-23T01:01:00Z",
                  "2026-11-23T01:27:31Z", verdict="cancelled")
        row.update(runner_id=None, runner_name="", steps=[])
        self.assertIsNone(runtime_window(row))

    def test_worker_cancelled_after_start_remains_real_execution(self):
        from ci_execution_receipts import runtime_window
        row = job("assigned", 104, 1, "2026-11-23T01:01:00Z",
                  "2026-11-23T01:07:00Z", verdict="cancelled")
        row.update(runner_id=42, runner_name="Hosted", steps=[{"name": "work"}])
        self.assertEqual(runtime_window(row), (row["started_at"], row["completed_at"]))

    def test_assigned_worker_negative_time_is_not_free(self):
        from ci_execution_receipts import runtime_window
        row = job("assigned", 105, 1, "2026-11-23T01:01:00Z",
                  "2026-11-23T01:00:59Z", verdict="cancelled")
        row.update(runner_id=42, steps=[{"name": "work"}])
        with self.assertRaisesRegex(ValueError, "negative execution duration"):
            runtime_window(row)

    def test_empty_single_attempt_is_valid(self):
        chosen, facts = canonical_executions([], 1, (1,))
        self.assertEqual(chosen, [])
        self.assertEqual(facts["provider_job_records"], 0)

    def test_unknown_attempt_identity_refused(self):
        rows = fixture()
        rows[3]["run_attempt"] = 3
        with self.assertRaisesRegex(ValueError, "foreign or missing"):
            canonical_executions(rows, 2)


if __name__ == "__main__":
    unittest.main()

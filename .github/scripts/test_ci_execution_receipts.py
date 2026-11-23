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

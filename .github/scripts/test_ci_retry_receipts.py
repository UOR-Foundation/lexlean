"""Source-bound false-green and duplicate-execution controls."""
import copy
import json
from pathlib import Path
import unittest
import ci_retry_receipts as audit


def fixture():
    def row(name, jid, attempt, start, end, verdict="success", state="completed"):
        return {"name": name, "id": jid, "run_attempt": attempt,
                "created_at": "2026-11-23T00:00:00Z" if attempt == 1 else "2026-11-23T01:00:00Z",
                "started_at": start, "completed_at": end,
                "conclusion": verdict, "status": state}
    a = [row("proof", 1, 1, "2026-11-23T00:01:00Z", "2026-11-23T00:05:00Z"),
         row("repro", 2, 1, "2026-11-23T00:01:00Z", "2026-11-23T00:03:00Z"),
         row(audit.ARM, 3, 1, "2026-11-23T00:01:00Z", "2026-11-23T00:10:00Z", "failure")]
    b = [row("proof", 11, 2, "2026-11-23T00:01:00Z", "2026-11-23T00:05:00Z"),
         row("repro", 12, 2, "2026-11-23T00:01:00Z", "2026-11-23T00:03:00Z"),
         row(audit.ARM, 13, 2, "2026-11-23T01:01:00Z", None, None, "in_progress")]
    return {"schema": audit.SCHEMA, "run_id": 42, "head_sha": "a"*40,
            "source_ref": "refs/heads/release/v0.4",
            "attempt_1": {"conclusion":"failure", "job_count":3, "success_count":2,
                          "failure_count":1, "jobs":a},
            "attempt_2": {"status":"in_progress", "conclusion":None, "job_count":3,
                          "jobs":b}}


class ReceiptTests(unittest.TestCase):
    def setUp(self):
        self.data = fixture()

    def refuses(self, fragment):
        with self.assertRaisesRegex(ValueError, fragment):
            audit.reconcile(self.data)

    def test_pending_not_release(self):
        r = audit.reconcile(self.data)
        self.assertEqual((r["host_recovery"], r["retained_unweighted_minutes"]), ("PENDING", 6))
        self.assertFalse(r["release_gate_authorized"])

    def test_finished_host_success_still_not_release(self):
        self.data["attempt_2"].update(status="completed", conclusion="success")
        self.data["attempt_2"]["jobs"][2].update(status="completed", conclusion="success",
                                                  completed_at="2026-11-23T01:12:00Z")
        r = audit.reconcile(self.data)
        self.assertEqual(r["host_recovery"], "CONFIRMED")
        self.assertFalse(r["release_gate_authorized"])

    def test_missing_job(self):
        self.data["attempt_2"]["jobs"].pop()
        self.refuses("incomplete")

    def test_duplicate_job_name(self):
        self.data["attempt_2"]["jobs"][1]["name"] = "proof"
        self.refuses("duplicate")

    def test_duplicate_job_id(self):
        self.data["attempt_2"]["jobs"][1]["id"] = 11
        self.refuses("duplicate")

    def test_changed_source_head(self):
        self.data["head_sha"] = "f" * 39
        self.refuses("exact run/SHA")

    def test_wrong_attempt(self):
        self.data["attempt_2"]["jobs"][1]["run_attempt"] = 1
        self.refuses("foreign")

    def test_forged_success_execution(self):
        self.data["attempt_2"]["jobs"][0].update(started_at="2026-11-23T01:01:00Z",
                                                  completed_at="2026-11-23T01:04:00Z")
        self.refuses("execution changed")

    def test_false_green_arm_failure(self):
        self.data["attempt_2"].update(status="completed", conclusion="success")
        self.data["attempt_2"]["jobs"][2].update(status="completed", conclusion="failure",
                                                  completed_at="2026-11-23T01:12:00Z")
        self.refuses("false-green")

    def test_false_green_unfinished_arm(self):
        self.data["attempt_2"].update(status="completed", conclusion="success")
        self.refuses("premature")

    def test_future_arm_job_not_started_yet(self):
        self.data["attempt_2"]["jobs"][2]["created_at"] = "2026-11-23T00:00:00Z"
        self.refuses("later retry")

    def test_new_job_injected(self):
        new = copy.deepcopy(self.data["attempt_2"]["jobs"][0])
        new.update(name="unauthorized", id=123)
        self.data["attempt_2"]["jobs"].append(new)
        self.data["attempt_2"]["job_count"] = 4
        self.refuses("required job set")

    def test_invalid_branch(self):
        self.data["source_ref"] = "refs/heads/main"
        self.refuses("source ref")

    def test_current_historical_ledger(self):
        path = Path(__file__).resolve().parents[1] / "metrics" / "ci-native-targeted-retry-20261123.json"
        if not path.is_file():
            self.skipTest("requires complete repository checkout")
        record = json.loads(path.read_text())
        result = audit.reconcile(record)
        self.assertEqual(result["required_jobs"], 78)
        self.assertEqual(result["retained_successes"], 77)
        self.assertEqual(result["retained_unweighted_minutes"], 1129.95)
        self.assertEqual(result["host_recovery"], "PENDING")
        self.assertFalse(result["release_gate_authorized"])


if __name__ == "__main__":
    unittest.main()

"""Adversarial checks for same-run attempt lineage."""
import copy
import unittest
from ci_attempt_lineage import reconcile, Hold

SHA = "a" * 40
HOSTS = {"proof": "linux", "native": "arm"}


def fixture():
    def job(name, jid, attempt, created, start, end, conclusion, status="completed"):
        return dict(name=name, id=jid, run_id=42, head_sha=SHA, run_attempt=attempt,
                    created_at=created, started_at=start, completed_at=end,
                    conclusion=conclusion, status=status, labels=[HOSTS[name]])
    a = [job("proof", 1, 1, "2026-11-24T00:00:00Z", "2026-11-24T00:01:00Z",
             "2026-11-24T00:11:00Z", "success"),
         job("native", 2, 1, "2026-11-24T00:00:00Z", "2026-11-24T00:01:00Z",
             "2026-11-24T00:03:00Z", "failure")]
    b = copy.deepcopy(a)
    b[0].update(id=3, run_attempt=2, created_at="2026-11-24T00:12:00Z")
    b[1].update(id=4, run_attempt=2, created_at="2026-11-24T00:12:00Z",
                started_at="2026-11-24T00:13:00Z", completed_at=None,
                conclusion=None, status="in_progress")
    return [dict(run_id=42, head_sha=SHA, run_attempt=n, total_count=2, jobs=j)
            for n, j in enumerate((a, b), 1)]


class LineageTests(unittest.TestCase):
    def audit(self, attempts=None):
        return reconcile(42, SHA, HOSTS, fixture() if attempts is None else attempts)

    def test_carried_job_uses_origin_not_reissued_id(self):
        r = self.audit()
        self.assertEqual(r["jobs"]["proof"]["origin_job_id"], 1)
        self.assertEqual(r["jobs"]["proof"]["latest_job_id"], 3)
        self.assertEqual(r["retained_successes"], 1)
        self.assertEqual(r["completed_execution_minutes"], 12)
        self.assertFalse(r["all_latest_jobs_success"])
        self.assertFalse(r["release_gate_authorized"])

    def test_success_is_not_release_authority(self):
        a = fixture()
        a[1]["jobs"][1].update(status="completed", conclusion="success",
                              completed_at="2026-11-24T00:18:00Z")
        r = self.audit(a)
        self.assertTrue(r["all_latest_jobs_success"])
        self.assertEqual(r["completed_execution_minutes"], 17)
        self.assertFalse(r["provider_authenticity_verified"])

    def test_refuse_mutated_snapshots(self):
        for change in (
            {"id": 1}, {"head_sha": "b" * 40}, {"run_id": 43},
            {"run_attempt": 1}, {"name": "native"}, {"labels": ["wrong"]},
            {"started_at": "2026-11-24T00:02:00Z"}, {"conclusion": "skipped"},
        ):
            with self.subTest(change=change):
                a = fixture()
                a[1]["jobs"][0].update(change)
                with self.assertRaises(Hold):
                    self.audit(a)

    def test_missing_roster(self):
        a = fixture()
        a[1]["jobs"].pop()
        with self.assertRaises(Hold):
            self.audit(a)

    def test_missing_first_attempt(self):
        with self.assertRaises(Hold):
            self.audit(fixture()[1:])

    def test_failed_fresh_execution_overrides_prior_green(self):
        a = fixture()
        a[1]["jobs"][0].update(started_at="2026-11-24T00:13:00Z",
                              completed_at="2026-11-24T00:14:00Z", conclusion="failure")
        r = self.audit(a)
        self.assertEqual(r["jobs"]["proof"]["origin_job_id"], 3)
        self.assertEqual(r["jobs"]["proof"]["conclusion"], "failure")
        self.assertEqual(r["retained_successes"], 0)


if __name__ == "__main__":
    unittest.main()

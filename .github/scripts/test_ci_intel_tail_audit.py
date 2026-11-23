"""Negative-control cases for historical native Intel queue/tail evidence."""
import copy
import unittest
import ci_intel_tail_audit as audit


def fixture():
    names = ["lower", "print", "validate", "moves"]
    counts = [6, 20, 86, 395]
    warnings = [6, 18, 12, 31]
    return {"schema": "lexlean-intel-ci-case-tail-v1", "run_id": 45100057759,
            "head_sha": "b693bb4259d311fd71da90ac38701214a74be27f",
            "lanes": {name: {"job_id": audit.EXPECTED[name],
                             "created_at": "2026-11-23T00:00:00Z",
                             "started_at": "2026-11-23T00:01:00Z",
                             "completed_at": "2026-11-23T01:00:00Z",
                             "cases": [{"name": f"conformance_{name}_{i:03d}",
                                        "warning_at": "2026-11-23T00:03:00Z" if i < warnings[n] else None,
                                        "finished_at": "2026-11-23T00:06:00Z"}
                                       for i in range(counts[n])]}
                      for n, name in enumerate(names)}}


class TailAuditTest(unittest.TestCase):
    def setUp(self):
        self.data = fixture()

    def reject(self, mutation):
        data = copy.deepcopy(self.data)
        mutation(data)
        with self.assertRaises((audit.Hold, ValueError, KeyError, TypeError)):
            audit.evaluate(data)

    def test_fixture(self):
        result = audit.evaluate(self.data)
        self.assertEqual(result["total_unique_passed"], 507)
        self.assertEqual(result["lanes"]["moves"]["slow_warning_count"], 31)

    def test_fixed_runtime_three_way_counterexample(self):
        data = copy.deepcopy(self.data)
        data["lanes"]["moves"]["cases"][0]["finished_at"] = "2026-11-23T00:29:00Z"
        result = audit.evaluate(data)
        self.assertGreater(result["moves_longest_case_lower_bound_seconds"],
                           result["moves_equal_three_test_step_seconds"])
        self.assertFalse(result["unchanged_case_duration_equal_split_feasible"])

    def test_historical_ledger_in_repository(self):
        from pathlib import Path
        path = Path(__file__).resolve().parent.parent / "metrics" / "ci-intel-case-tail-20261123.json"
        self.assertTrue(path.is_file(), "historical receipt ledger missing")
        import json
        result = audit.evaluate(json.loads(path.read_text()))
        self.assertEqual(result["total_unique_passed"], 507)
        self.assertEqual(result["moves_longest_case_lower_bound_seconds"], 1450.727)
        self.assertFalse(result["unchanged_case_duration_equal_split_feasible"])
        self.assertEqual(result["lanes"]["moves"]["queue_seconds"], 757)

    def test_missing_case(self):
        self.reject(lambda x: x["lanes"]["moves"]["cases"].pop())

    def test_duplicate_case(self):
        self.reject(lambda x: x["lanes"]["moves"]["cases"].append(x["lanes"]["moves"]["cases"][0]))

    def test_wrong_job(self):
        self.reject(lambda x: x["lanes"]["lower"].__setitem__("job_id", 1))

    def test_missing_lane(self):
        self.reject(lambda x: x["lanes"].pop("print"))

    def test_foreign_case(self):
        self.reject(lambda x: x["lanes"]["lower"]["cases"][0].__setitem__("name", "foreign"))

    def test_bad_warning_order(self):
        self.reject(lambda x: x["lanes"]["lower"]["cases"][0].__setitem__("warning_at", "2026-11-23T00:07:00Z"))

    def test_warning_outside_job(self):
        self.reject(lambda x: x["lanes"]["lower"]["cases"][0].__setitem__("warning_at", "2026-11-22T23:00:00Z"))

    def test_finished_outside_job(self):
        self.reject(lambda x: x["lanes"]["print"]["cases"][0].__setitem__("finished_at", "2026-11-23T02:00:00Z"))

    def test_bad_queue_order(self):
        self.reject(lambda x: x["lanes"]["moves"].__setitem__("created_at", "2026-11-23T00:02:00Z"))

    def test_repeated_warning_missing(self):
        self.reject(lambda x: x["lanes"]["lower"]["cases"][0].__setitem__("warning_at", None))

    def test_extra_case_fields(self):
        self.reject(lambda x: x["lanes"]["lower"]["cases"][0].__setitem__("status", "PASSED"))

    def test_bad_source_run(self):
        self.reject(lambda x: x.__setitem__("head_sha", "a" * 40))


if __name__ == "__main__":
    unittest.main()

"""Raw-vs-corrected historical cohort checks, no provider calls or cutover claims."""
import copy
import json
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1] / "metrics"
ORIGINAL = ROOT / "ci-before-pr50-job-costs.json"
CORRECTED = ROOT / "ci-before-pr50-job-costs-corrected.json"
LEDGER = ROOT / "ci-before-pr50-reconciliation-20261123.json"
KNOWN = frozenset({
    44347758697, 43851090792, 43773711832, 43745407916,
    43709771141, 43582153525, 44270426412,
})


def validate(original, revised, receipt):
    assert receipt.get("schema") == "lexlean/ci-before-pr50-reconciliation/1"
    assert receipt.get("classification") == "HISTORICAL_ACCOUNTING_CORRECTION_NOT_CI_SAVINGS"
    assert receipt.get("billed_minutes") is None
    assert receipt.get("after_cohort") is None
    before = {x["id"]: x for x in original["run_costs"]}
    after = {x["id"]: x for x in revised["run_costs"]}
    assert len(before) == len(after) == 128
    assert set(before) == set(after)
    cases = receipt["source_reconciled_corrections"]
    assert len(cases) == 7
    ids = {x["id"] for x in cases}
    assert ids == KNOWN
    for rid, old in before.items():
        new = after[rid]
        if rid not in ids:
            assert old == new, "unexplained original baseline change"
        else:
            proof = next(x for x in cases if x["id"] == rid)
            assert old["pr"] == new["pr"] == proof["pr"]
            assert old["job_minutes"] == proof["old_runner_job_minutes"]
            assert new["job_minutes"] == proof["corrected_runner_job_minutes"]
            assert new["job_count"] == new["returned"] == proof["distinct_job_rows"]
            assert old["job_count"] == proof["raw_job_rows"]
            assert new["provider_job_records"] == proof["raw_job_rows"]
            assert new["carried_reissued_records"] == proof["carried_rows"]
            assert new["job_minutes"] >= 0
            assert abs(sum(new["by_platform_minutes"].values()) -
                       new["job_minutes"]) <= 0.01
    a = round(sum(x["job_minutes"] for x in before.values()), 3)
    b = round(sum(x["job_minutes"] for x in after.values()), 3)
    assert a == receipt["raw_before_total_unweighted_minutes"] == 99321.656
    assert b == receipt["corrected_before_total_unweighted_minutes"] == 88298.354
    assert round(a-b, 3) == receipt["phantom_metadata_minutes_removed"] == 11023.302
    assert receipt["unchanged_run_count"] == 121
    assert receipt["corrected_run_count"] == 7
    assert receipt["checked_multi_attempt_runs"] == 7
    assert receipt["checked_cancelled_runs"] == 7


class HistoryCostTests(unittest.TestCase):
    def setUp(self):
        if not all(p.is_file() for p in (ORIGINAL, CORRECTED, LEDGER)):
            self.skipTest("requires complete repository evidence checkout")
        self.a, self.b, self.receipt = (
            json.loads(path.read_text()) for path in (ORIGINAL, CORRECTED, LEDGER)
        )

    def test_actual_full_corrected_cohort(self):
        validate(self.a, self.b, self.receipt)

    def test_original_immutable(self):
        self.assertEqual(self.a["expected_runs"], 128)
        self.assertEqual(self.a["run_costs"][0]["id"], self.b["run_costs"][0]["id"])

    def test_unjustified_other_run_change_refused(self):
        changed = copy.deepcopy(self.b)
        row = next(x for x in changed["run_costs"] if x["id"] not in KNOWN)
        row["job_minutes"] = 0
        with self.assertRaises(AssertionError):
            validate(self.a, changed, self.receipt)

    def test_undocumented_fix_refused(self):
        changed = copy.deepcopy(self.b)
        changed["run_costs"] = [
            x for x in changed["run_costs"] if x["id"] != 44347758697
        ]
        with self.assertRaises(AssertionError):
            validate(self.a, changed, self.receipt)

    def test_forged_phantom_saving_refused(self):
        receipt = copy.deepcopy(self.receipt)
        receipt["phantom_metadata_minutes_removed"] += 100
        with self.assertRaises(AssertionError):
            validate(self.a, self.b, receipt)

    def test_billed_savings_claim_refused(self):
        receipt = copy.deepcopy(self.receipt)
        receipt["billed_minutes"] = 0
        with self.assertRaises(AssertionError):
            validate(self.a, self.b, receipt)


if __name__ == "__main__":
    unittest.main()

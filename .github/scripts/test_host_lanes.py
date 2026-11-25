"""Behavioral tests of the host lanes' join (SPEC §8.3), run in honesty."""

import copy
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parent))
import host_lanes  # noqa: E402

BDD = {"parts": {"one": ["AA-01"], "two": ["AA-02"]}, "targets": {"honesty": ["bdd"]}, "remainder": "rest"}
PLAN = {"producer": ["honesty"], "lanes": {"a": ["one"], "b": ["two", "rest"]}}
NAMES = ["producer", "a", "b"]
HERE = {"commit": "c" * 40, "tree": "t" * 40, "run": "7", "attempt": "1"}
HARNESS = ["conformance_aa_01", "conformance_aa_02", "conformance_aa_03"]


def receipts():
    base = {"schema": 1, "target": "x86_64-pc-windows-msvc", "passed": True, "conformance": HARNESS,
            "whole": None, **HERE}
    return {
        "producer": {**base, "lane": "producer", "executed": ["meta", "unit"],
                     "whole": sorted(["meta", "unit", *HARNESS])},
        "a": {**base, "lane": "a", "executed": ["conformance_aa_01"]},
        "b": {**base, "lane": "b", "executed": ["conformance_aa_02", "conformance_aa_03"]},
    }


class Join(unittest.TestCase):
    def check(self, value):
        return host_lanes.check_join(value, NAMES, PLAN, BDD, HERE)

    def refuse(self, value):
        with self.assertRaises(host_lanes.Invalid):
            self.check(value)

    def test_complete_lanes_pass(self):
        self.assertEqual(self.check(receipts()), 5)

    def test_missing_lane(self):
        value = receipts()
        del value["b"]
        self.refuse(value)

    def test_test_run_twice(self):
        value = receipts()
        value["a"]["executed"].append("conformance_aa_02")
        self.refuse(value)

    def test_test_lost(self):
        value = receipts()
        value["b"]["executed"] = ["conformance_aa_02"]
        self.refuse(value)

    def test_failed_lane(self):
        value = receipts()
        value["a"]["passed"] = False
        self.refuse(value)

    def test_other_commit_or_run(self):
        for field in ("commit", "tree", "run"):
            value = receipts()
            value["a"][field] = "x"
            self.refuse(value)

    def test_different_harness(self):
        value = receipts()
        value["b"] = copy.deepcopy(value["b"])
        value["b"]["conformance"] = HARNESS[:2]
        self.refuse(value)

    def test_conformance_lane_outside_harness(self):
        value = receipts()
        value["a"]["executed"] = ["meta"]
        value["producer"]["executed"] = ["conformance_aa_01", "unit"]
        self.refuse(value)

    def test_named_test_absent(self):
        value = receipts()
        for receipt in value.values():
            receipt["conformance"] = ["conformance_aa_02", "conformance_aa_03"]
        value["producer"]["whole"] = sorted(["meta", "unit", "conformance_aa_02", "conformance_aa_03"])
        value["a"]["executed"] = []
        self.refuse(value)


    def test_invalid_attempt(self):
        for attempt in ("0", "-1", "2", "²", "01", "", True):
            with self.subTest(attempt=attempt):
                value = receipts()
                value["a"]["attempt"] = attempt
                self.refuse(value)

    def test_invalid_join_attempt(self):
        with self.assertRaisesRegex(host_lanes.Invalid, "invalid join attempt"):
            host_lanes.check_join(receipts(), NAMES, PLAN, BDD, {**HERE, "attempt": "0"})

    def test_positive_retained_attempt(self):
        self.assertEqual(host_lanes.check_join(receipts(), NAMES, PLAN, BDD, {**HERE, "attempt": "2"}), 5)

    def test_wrong_receipt_lane(self):
        value = receipts()
        value["a"]["lane"] = "b"
        self.refuse(value)

    def test_swapped_lanes(self):
        value = receipts()
        value["a"]["executed"], value["b"]["executed"] = value["b"]["executed"], value["a"]["executed"]
        self.refuse(value)

    def test_producer_absorbs_conformance(self):
        value = receipts()
        value["producer"]["executed"] += value["a"]["executed"]
        value["a"]["executed"] = []
        self.refuse(value)

    def test_duplicate_conformance_inventory(self):
        value = receipts()
        for receipt in value.values():
            receipt["conformance"] = [*HARNESS, HARNESS[0]]
        value["producer"]["whole"].append(HARNESS[0])
        value["a"]["executed"].append(HARNESS[0])
        self.refuse(value)

    def test_repeated_workspace_name_allowed(self):
        value = receipts()
        value["producer"]["whole"].append(HARNESS[0])
        value["producer"]["executed"].append(HARNESS[0])
        self.assertEqual(self.check(value), 6)

    def test_missing_workspace_conformance(self):
        value = receipts()
        value["producer"]["whole"].remove(HARNESS[0])
        value["a"]["executed"] = []
        self.refuse(value)

    def test_plan_drift(self):
        plan = {"producer": ["honesty"], "lanes": {"a": ["one", "two"], "b": ["two", "rest"]}}
        with self.assertRaisesRegex(host_lanes.Invalid, "conformance lane plan drift"):
            host_lanes.check_join(receipts(), NAMES, plan, BDD, HERE)

if __name__ == "__main__":
    unittest.main()

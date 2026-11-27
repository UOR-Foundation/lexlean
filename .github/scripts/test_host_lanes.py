"""Behavioral tests of the host lanes' join (SPEC §8.3), run in honesty."""

import copy
import json
from pathlib import Path
import sys
import tempfile
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


class Hosts(unittest.TestCase):
    """Windows, Apple silicon macOS and Linux AArch64 join only their own host's receipts."""

    ROOT = Path(__file__).resolve().parents[2]

    def test_receipt_of_another_host(self):
        for mine, other in (("aarch64-apple-darwin", "x86_64-pc-windows-msvc"),
                            ("x86_64-pc-windows-msvc", "aarch64-apple-darwin"),
                            ("aarch64-unknown-linux-gnu", "aarch64-apple-darwin"),
                            ("aarch64-apple-darwin", "aarch64-unknown-linux-gnu")):
            with tempfile.TemporaryDirectory() as artifacts:
                lane = Path(artifacts) / f"host-{other}-producer"
                lane.mkdir()
                receipt = {**receipts()["producer"], "target": other}
                (lane / "receipt.json").write_text(json.dumps(receipt))
                with self.assertRaisesRegex(host_lanes.Invalid, "another host"):
                    host_lanes.join(self.ROOT, mine, artifacts)


class LinuxArm(unittest.TestCase):
    """Linux AArch64 joins through the same receipts as the other lane hosts:
    a complete set of this run's and commit's receipts passes, and a missing,
    duplicated, foreign or failed receipt is refused (#624)."""

    ROOT = Path(__file__).resolve().parents[2]
    TARGET = "aarch64-unknown-linux-gnu"

    def setUp(self):
        self.environment = {key: host_lanes.os.environ.get(key) for key in ("GITHUB_RUN_ID", "GITHUB_RUN_ATTEMPT")}
        host_lanes.os.environ.update({"GITHUB_RUN_ID": "7", "GITHUB_RUN_ATTEMPT": "1"})

    def tearDown(self):
        for key, value in self.environment.items():
            if value is None:
                host_lanes.os.environ.pop(key, None)
            else:
                host_lanes.os.environ[key] = value

    def complete(self, target=None):
        """One receipt for each lane of the repository's partition."""
        bdd, _, names = host_lanes.lanes(self.ROOT)
        here = host_lanes.identity(self.ROOT)
        named = sorted({host_lanes.factory.test_name(i) for ids in bdd["parts"].values() for i in ids})
        conformance = [*named, "conformance_unnamed"]
        base = {"schema": 1, "target": target or self.TARGET, "passed": True, "conformance": conformance,
                "whole": None, **here}
        lanes = [name for name in names if name != host_lanes.PRODUCER]
        out = {host_lanes.PRODUCER: {**base, "lane": host_lanes.PRODUCER, "executed": ["unit"],
                                     "whole": sorted(["unit", *conformance])}}
        for index, lane in enumerate(lanes):
            out[lane] = {**base, "lane": lane, "executed": conformance[index::len(lanes)]}
        return out

    def join(self, receipts, extra=()):
        with tempfile.TemporaryDirectory() as artifacts:
            for key, receipt in [*receipts.items(), *extra]:
                lane = Path(artifacts) / f"host-{receipt['target']}-{key}"
                lane.mkdir()
                (lane / "receipt.json").write_text(json.dumps(receipt))
            return host_lanes.join(self.ROOT, self.TARGET, artifacts)

    def test_complete_lanes_pass(self):
        receipts = self.complete()
        self.assertEqual(self.join(receipts), 1 + len(receipts[host_lanes.PRODUCER]["conformance"]))

    def test_missing_lane(self):
        receipts = self.complete()
        del receipts["moves"]
        with self.assertRaisesRegex(host_lanes.Invalid, "lanes .* receipts"):
            self.join(receipts)

    def test_duplicated_lane(self):
        receipts = self.complete()
        with self.assertRaisesRegex(host_lanes.Invalid, "two receipts of lane `lower`"):
            self.join(receipts, extra=[("lower-again", receipts["lower"])])

    def test_foreign_receipt(self):
        receipts = self.complete()
        receipts["print"] = {**receipts["print"], "target": "aarch64-apple-darwin"}
        with self.assertRaisesRegex(host_lanes.Invalid, "another host"):
            self.join(receipts)

    def test_failed_lane(self):
        receipts = self.complete()
        receipts["validate"] = {**receipts["validate"], "passed": False}
        with self.assertRaisesRegex(host_lanes.Invalid, "did not pass"):
            self.join(receipts)

    def test_receipts_joined_as_another_host(self):
        with tempfile.TemporaryDirectory() as artifacts:
            receipt = self.complete()[host_lanes.PRODUCER]
            lane = Path(artifacts) / f"host-{self.TARGET}-producer"
            lane.mkdir()
            (lane / "receipt.json").write_text(json.dumps(receipt))
            with self.assertRaisesRegex(host_lanes.Invalid, "another host"):
                host_lanes.join(self.ROOT, "aarch64-apple-darwin", artifacts)


if __name__ == "__main__":
    unittest.main()

"""Behavioral tests of the host lanes' join (SPEC §8.3), run in honesty."""

import copy
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

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


class ArtifactBoundary(unittest.TestCase):
    TARGET = "x86_64-pc-windows-msvc"

    def setUp(self):
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        self.root = Path(temp.name)
        for lane, receipt in receipts().items():
            path = self.path(lane)
            path.parent.mkdir()
            path.write_text(json.dumps(receipt))

    def path(self, lane):
        return self.root / f"host-{self.TARGET}-{lane}" / "receipt.json"

    def check(self):
        with patch.object(host_lanes, "lanes", return_value=(BDD, PLAN, NAMES)), \
                patch.object(host_lanes, "identity", return_value=HERE):
            return host_lanes.join(self.root, self.TARGET, self.root)

    def refuse(self):
        with self.assertRaises(host_lanes.Invalid):
            self.check()

    def test_exact_artifacts_pass(self):
        self.assertEqual(self.check(), 5)

    def test_swapped_artifact_payloads(self):
        a, b = self.path("a"), self.path("b")
        x, y = a.read_text(), b.read_text()
        a.write_text(y)
        b.write_text(x)
        self.refuse()

    def test_duplicate_json_key(self):
        path = self.path("a")
        path.write_text(path.read_text().replace('"lane": "a"', '"lane": "b", "lane": "a"'))
        self.refuse()

    def test_extra_artifact_file(self):
        (self.path("a").parent / "extra").touch()
        self.refuse()

    def test_boolean_schema(self):
        path = self.path("a")
        path.write_text(path.read_text().replace('"schema": 1', '"schema": true'))
        self.refuse()

    def test_renamed_artifact(self):
        self.path("a").parent.rename(self.root / "wrong")
        self.refuse()

    def test_symlink_receipt(self):
        path = self.path("a")
        with tempfile.TemporaryDirectory() as outside:
            external = Path(outside) / "receipt.json"
            external.write_text(path.read_text())
            path.unlink()
            try:
                path.symlink_to(external)
            except OSError as error:
                self.skipTest(f"symlinks unavailable: {error}")
            self.refuse()


class Hosts(unittest.TestCase):
    """Windows, Apple silicon macOS and Linux AArch64 join only their own host's receipts."""

    ROOT = Path(__file__).resolve().parents[2]

    def test_receipt_of_another_host(self):
        for mine, other in (("aarch64-apple-darwin", "x86_64-pc-windows-msvc"),
                            ("x86_64-pc-windows-msvc", "aarch64-apple-darwin"),
                            ("aarch64-unknown-linux-gnu", "aarch64-apple-darwin"),
                            ("aarch64-apple-darwin", "aarch64-unknown-linux-gnu")):
            # Another host's artifact directory is outside this host's exact
            # artifact set (#631) ...
            with tempfile.TemporaryDirectory() as artifacts:
                lane = Path(artifacts) / f"host-{other}-producer"
                lane.mkdir()
                receipt = {**receipts()["producer"], "target": other}
                (lane / "receipt.json").write_text(json.dumps(receipt))
                with self.assertRaisesRegex(host_lanes.Invalid, "unexpected artifact directory"):
                    host_lanes.join(self.ROOT, mine, artifacts)
            # ... and another host's receipt under this host's own artifact
            # names is still refused by the receipt itself.
            with self.subTest(mine=mine, other=other), \
                    tempfile.TemporaryDirectory() as artifacts:
                _, plan, _ = host_lanes.lanes(self.ROOT)
                for name in ["producer", *plan["lanes"]]:
                    lane = Path(artifacts) / f"host-{mine}-{name}"
                    lane.mkdir()
                    receipt = {**receipts()["producer"], "lane": name, "target": other}
                    (lane / "receipt.json").write_text(json.dumps(receipt))
                with self.assertRaisesRegex(host_lanes.Invalid, "another host"):
                    host_lanes.join(self.ROOT, mine, artifacts)


class LinuxArm(unittest.TestCase):
    """Linux AArch64 joins through the same receipts as the other lane hosts
    (#624): a complete set of this run's and commit's receipts, each lane
    executing exactly its assignment of the repository's partition, passes;
    a missing, duplicated, foreign or failed receipt, a test run in the wrong
    lane, a zero attempt and a foreign artifact directory are each refused."""

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

    def complete(self):
        """One receipt for each lane of the repository's partition, each lane
        executing exactly the conformance tests its parts assign it."""
        bdd, plan, _ = host_lanes.lanes(self.ROOT)
        here = host_lanes.identity(self.ROOT)
        named = sorted({host_lanes.factory.test_name(i) for ids in bdd["parts"].values() for i in ids})
        conformance = [*named, "conformance_unnamed"]
        base = {"schema": 1, "target": self.TARGET, "passed": True, "conformance": conformance,
                "whole": None, **here}
        out = {host_lanes.PRODUCER: {**base, "lane": host_lanes.PRODUCER, "executed": ["unit"],
                                     "whole": sorted(["unit", *conformance])}}
        for lane, parts in plan["lanes"].items():
            executed = [host_lanes.factory.test_name(i) for part in parts if part in bdd["parts"]
                        for i in bdd["parts"][part]]
            if bdd["remainder"] in parts:
                executed.append("conformance_unnamed")
            out[lane] = {**base, "lane": lane, "executed": sorted(executed)}
        return out

    def join(self, receipts, extra=()):
        with tempfile.TemporaryDirectory() as artifacts:
            for directory, receipt in [*((f"host-{self.TARGET}-{lane}", r) for lane, r in receipts.items()),
                                       *extra]:
                lane = Path(artifacts) / directory
                lane.mkdir()
                (lane / "receipt.json").write_text(json.dumps(receipt))
            return host_lanes.join(self.ROOT, self.TARGET, artifacts)

    def test_complete_lanes_pass(self):
        receipts = self.complete()
        self.assertEqual(self.join(receipts), len(receipts[host_lanes.PRODUCER]["whole"]))

    def test_missing_receipt(self):
        receipts = self.complete()
        del receipts["moves"]
        with self.assertRaisesRegex(host_lanes.Invalid, r"artifact files differ: missing=\['host-aarch64-unknown-linux-gnu-moves/receipt.json'\]"):
            self.join(receipts)

    def test_duplicated_receipt(self):
        receipts = self.complete()
        with self.assertRaisesRegex(host_lanes.Invalid, "unexpected artifact directory"):
            self.join(receipts, extra=[(f"host-{self.TARGET}-lower-again", receipts["lower"])])
        receipts["print"] = {**receipts["lower"]}
        with self.assertRaisesRegex(host_lanes.Invalid, "receipt lane differs from artifact name"):
            self.join(receipts)

    def test_foreign_receipt(self):
        receipts = self.complete()
        receipts["print"] = {**receipts["print"], "target": "aarch64-apple-darwin"}
        with self.assertRaisesRegex(host_lanes.Invalid, "another host"):
            self.join(receipts)

    def test_foreign_artifact_directory(self):
        receipts = self.complete()
        foreign = {**receipts[host_lanes.PRODUCER], "target": "aarch64-apple-darwin"}
        with self.assertRaisesRegex(host_lanes.Invalid, "unexpected artifact directory"):
            self.join(receipts, extra=[("host-aarch64-apple-darwin-producer", foreign)])

    def test_failed_receipt(self):
        receipts = self.complete()
        receipts["validate"] = {**receipts["validate"], "passed": False}
        with self.assertRaisesRegex(host_lanes.Invalid, "did not pass"):
            self.join(receipts)

    def test_wrong_lane_swap(self):
        receipts = self.complete()
        lower, moves = receipts["lower"]["executed"], receipts["moves"]["executed"]
        receipts["lower"] = {**receipts["lower"], "executed": sorted([*lower[1:], moves[0]])}
        receipts["moves"] = {**receipts["moves"], "executed": sorted([*moves[1:], lower[0]])}
        with self.assertRaisesRegex(host_lanes.Invalid, "differs from its assigned inventory"):
            self.join(receipts)

    def test_zero_attempt(self):
        receipts = self.complete()
        receipts["print"] = {**receipts["print"], "attempt": "0"}
        with self.assertRaisesRegex(host_lanes.Invalid, "another attempt"):
            self.join(receipts)
        host_lanes.os.environ["GITHUB_RUN_ATTEMPT"] = "0"
        with self.assertRaisesRegex(host_lanes.Invalid, "attempt"):
            self.join(self.complete())

    def test_receipts_joined_as_another_host(self):
        with tempfile.TemporaryDirectory() as artifacts:
            receipt = self.complete()[host_lanes.PRODUCER]
            lane = Path(artifacts) / f"host-{self.TARGET}-producer"
            lane.mkdir()
            (lane / "receipt.json").write_text(json.dumps(receipt))
            with self.assertRaisesRegex(host_lanes.Invalid, "unexpected artifact directory|another host"):
                host_lanes.join(self.ROOT, "aarch64-apple-darwin", artifacts)

if __name__ == "__main__":
    unittest.main()

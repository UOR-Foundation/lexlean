"""Adversarial source-bound case and runner ownership tests (NONQUALIFYING)."""
import copy
import json
import tempfile
import unittest
from pathlib import Path

import ci_test_ownership as mod


def fixture():
    ids = ["RP-14", "CF-01", "BS-08"]
    reg = "spec = 'lexlean/ids/1'\n" + "".join(
        f'[[id]]\nid = "{name}"\nlevel = "build"\n' for name in ids)
    cases = "".join(
        '#[test]\nfn conformance_' + ident.lower().replace("-", "_") +
        '() {\n    repo_conformance::cases::run("' + ident + '");\n}\n'
        for ident in ids)
    bdd = {
        "remainder": "rest", "parts": {"bootstrap-moves": ["BS-08"]},
        "targets": {"honesty": ["bdd"]},
    }
    lanes = {"producer": ["honesty"],
             "lanes": {"lower": ["bootstrap-moves"], "print": [],
                       "validate": [], "moves": ["rest"]}}
    # Fixtures may use empty lanes for a reduced partition model; the source
    # audit checks the four mandatory job families and complete union.
    rows = [
        {"part": "honesty", "bdd_part": "honesty",
         "recipes": "fmt-check model spec-links bdd"},
        {"part": "rest", "bdd_part": "rest",
         "recipes": "lint test features bdd inventory golden repro deny"},
        {"part": "examples", "bdd_part": "", "recipes": "examples"},
        {"part": "bdd-bootstrap-moves", "bdd_part": "bootstrap-moves",
         "recipes": "bdd"},
    ]
    return {
        "model/ids.toml": reg,
        "crates/conformance/tests/conformance.rs": cases,
        "crates/conformance/src/cases/mod.rs":
            'mod repository;\n"RP" => repository::run(id),\n',
        "crates/conformance/src/cases/repository.rs": '"RP-14" => {\n',
        "crates/conformance/src/bdd_parts.rs":
            'pub fn test_name(\npub fn harness_filters(\n'
            'pub fn cargo_targets(\nfor name in self.named_tests()\n'
            'filters.push("--skip".to_owned());\n',
        ".github/bdd-parts.json": json.dumps(bdd),
        ".github/intel-host-lanes.json": json.dumps(lanes),
        ".github/vv-rows.json": json.dumps(rows),
    }


def mutated(path, before, after):
    files = fixture()
    assert before in files[path], (path, before)
    files[path] = files[path].replace(before, after, 1)
    return files


class OwnershipContract(unittest.TestCase):
    def test_source_anchored_mapping_no_reuse_authority(self):
        report = mod.ownership(fixture())
        self.assertEqual(report["conformance_case_count"], 3)
        self.assertEqual(report["rp14"]["bdd_part"], "rest")
        self.assertEqual(report["rp14"]["native_lane"], "moves")
        self.assertFalse(report["cross_commit_reuse_authority"])
        self.assertEqual(report["source_to_runtime_dependency_closure"],
                         "INCOMPLETE")
        self.assertEqual(len(report["source_sha256"]), len(mod.SOURCES))

    def test_wrong_case_dispatch_rejected(self):
        f = mutated("crates/conformance/tests/conformance.rs",
                    'run("RP-14")', 'run("CF-01")')
        with self.assertRaises(mod.Hold):
            mod.ownership(f)

    def test_missing_case_wrapper_rejected(self):
        f = fixture()
        f["crates/conformance/tests/conformance.rs"] = f[
            "crates/conformance/tests/conformance.rs"
        ].replace('#[test]\nfn conformance_rp_14()', 'fn conformance_rp_14()')
        with self.assertRaises(mod.Hold):
            mod.ownership(f)

    def test_new_registered_id_missing_wrapper_rejected(self):
        f = fixture()
        f["model/ids.toml"] += '[[id]]\nid = "RP-15"\n'
        with self.assertRaises(mod.Hold):
            mod.ownership(f)

    def test_duplicate_register_refused(self):
        f = fixture()
        f["model/ids.toml"] += '[[id]]\nid = "RP-14"\n'
        with self.assertRaises(mod.Hold):
            mod.ownership(f)

    def test_registered_names_must_be_exact(self):
        f = mutated("crates/conformance/tests/conformance.rs",
                    "fn conformance_bs_08()", "fn conformance_rp_08()")
        with self.assertRaises(mod.Hold):
            mod.ownership(f)

    def test_case_registry_cannot_drop_rp_dispatch(self):
        f = mutated("crates/conformance/src/cases/mod.rs",
                    '"RP" => repository::run(id),', '"RP" => other::run(id),')
        with self.assertRaises(mod.Hold):
            mod.ownership(f)

    def test_rp14_implementation_cannot_disappear(self):
        f = mutated("crates/conformance/src/cases/repository.rs",
                    '"RP-14" => {', '"RP-19" => {')
        with self.assertRaises(mod.Hold):
            mod.ownership(f)

    def test_bdd_remainder_cannot_claim_named_case(self):
        f = fixture()
        bdd = json.loads(f[".github/bdd-parts.json"])
        bdd["parts"]["bootstrap-moves"].append("RP-14")
        f[".github/bdd-parts.json"] = json.dumps(bdd)
        with self.assertRaisesRegex(mod.Hold, "RP-14 ownership changed"):
            mod.ownership(f)

    def test_double_owned_bdd_case_rejected(self):
        f = fixture()
        bdd = json.loads(f[".github/bdd-parts.json"])
        bdd["parts"]["copy"] = ["BS-08"]
        f[".github/bdd-parts.json"] = json.dumps(bdd)
        with self.assertRaisesRegex(mod.Hold, "multiple BDD"):
            mod.ownership(f)

    def test_unknown_bdd_case_rejected(self):
        f = mutated(".github/bdd-parts.json", "BS-08", "BS-09")
        with self.assertRaisesRegex(mod.Hold, "unregistered"):
            mod.ownership(f)

    def test_honesty_owns_entire_meta_target(self):
        f = mutated(".github/bdd-parts.json", '"honesty": ["bdd"]',
                    '"honesty": ["conformance"]')
        with self.assertRaises(mod.Hold):
            mod.ownership(f)

    def test_incomplete_host_lanes_refused(self):
        f = mutated(".github/intel-host-lanes.json",
                    '"moves": ["rest"]', '"moves": []')
        with self.assertRaises(mod.Hold):
            mod.ownership(f)

    def test_cross_lane_duplicate_refused(self):
        f = mutated(".github/intel-host-lanes.json",
                    '"print": []', '"print": ["bootstrap-moves"]')
        with self.assertRaisesRegex(mod.Hold, "duplicated"):
            mod.ownership(f)

    def test_unknown_host_part_refused(self):
        f = mutated(".github/intel-host-lanes.json",
                    '"rest"', '"unknown"')
        with self.assertRaises(mod.Hold):
            mod.ownership(f)

    def test_missing_bdd_vv_row_refused(self):
        f = fixture()
        rows = json.loads(f[".github/vv-rows.json"])
        rows.pop()
        f[".github/vv-rows.json"] = json.dumps(rows)
        with self.assertRaisesRegex(mod.Hold, "verification recipes"):
            mod.ownership(f)

    def test_mislabelled_bdd_recipe_refused(self):
        f = mutated(".github/vv-rows.json",
                    '"recipes": "bdd"', '"recipes": "echo skip"')
        with self.assertRaisesRegex(mod.Hold, "recipe changed"):
            mod.ownership(f)

    def test_disabled_selector_refused(self):
        f = mutated("crates/conformance/src/bdd_parts.rs",
                    'filters.push("--skip".to_owned());', 'nothing')
        with self.assertRaisesRegex(mod.Hold, "selector"):
            mod.ownership(f)

    def test_duplicate_json_properties_refused(self):
        f = fixture()
        f[".github/bdd-parts.json"] = (
            '{"remainder":"rest","remainder":"rest",'
            '"parts":{"bootstrap-moves":["BS-08"]},'
            '"targets":{"honesty":["bdd"]}}')
        with self.assertRaises(mod.Hold):
            mod.ownership(f)

    def test_missing_source_document_refused(self):
        f = fixture()
        del f["model/ids.toml"]
        with self.assertRaisesRegex(mod.Hold, "missing or unrecognized"):
            mod.ownership(f)

    def test_real_repository_boundaries(self):
        root = Path(__file__).resolve().parents[2]
        if not (root / "model/ids.toml").is_file():
            self.skipTest("requires complete checkout")
        report = mod.audit_repo(root)
        self.assertGreater(report["conformance_case_count"], 500)
        self.assertGreater(report["bdd_named_parts"], 30)
        self.assertEqual(report["rp14"]["native_lane"], "moves")
        self.assertEqual(report["qualification"], "NONQUALIFYING_OWNER_MAP_ONLY")


if __name__ == "__main__":
    unittest.main()

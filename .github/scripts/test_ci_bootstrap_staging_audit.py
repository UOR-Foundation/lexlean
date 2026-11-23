"""Adversarial tests for the ACTUAL governed BDD stage scheduler (#603)."""
import importlib.util
import json
import shutil
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).with_name("ci_bootstrap_staging_audit.py")
ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location("staging_audit", SCRIPT)
mod = importlib.util.module_from_spec(spec)
spec.loader.exec_module(mod)


@unittest.skipUnless(shutil.which("jq"), "actual CI planner requires jq")
class BootstrapSchedulingTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        if not all((ROOT / path).is_file() for path in mod.SOURCES):
            raise unittest.SkipTest("requires checkout with governed source inputs")

    def setUp(self):
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        self.root = Path(temp.name)
        for path in mod.SOURCES:
            target = self.root / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes((ROOT / path).read_bytes())

    def edit(self, path, old, new):
        target = self.root / path
        source = target.read_text()
        self.assertEqual(source.count(old), 1,
                         "test input no longer matches the exact source: " + path)
        target.write_text(source.replace(old, new))

    def hold(self, expected):
        with self.assertRaisesRegex(mod.Hold, expected):
            mod.audit(self.root)

    def test_current_real_planner_executes_and_preserves_all_bootstrap_tests(self):
        result = mod.audit(self.root)
        self.assertEqual(result["classification"], "AUDIT_ONLY_NOT_QUALIFICATION")
        self.assertEqual(result["registered_bootstrap_cases"], 31)
        self.assertEqual(result["producer_dependent_bdd_partitions"], 29)
        self.assertEqual(result["producer_independent_bdd_partitions"], 11)
        self.assertFalse(result["provider_proof_receipts_verified"])

    def test_compiler_mixed_partition_must_wait_for_export(self):
        result = mod.audit(self.root)
        self.assertEqual(result["producer_dependent_bdd_partitions"], 29)
        parts = json.loads((self.root / ".github/bdd-parts.json").read_text())
        self.assertIn("BS-01", parts["parts"]["compiler"])

    def test_deleted_bootstrap_test_from_manifest_is_blocked(self):
        path = ".github/bdd-parts.json"
        self.edit(path, '"BS-09"', '"UNASSIGNED-09"')
        self.hold("bootstrap detector missing/extra")

    def test_duplicate_bootstrap_case_in_two_partitions_is_blocked(self):
        path = ".github/bdd-parts.json"
        self.edit(path, '"bootstrap-lower": [\n      "BS-07"\n    ]',
                  '"bootstrap-lower": [\n      "BS-07", "BS-09"\n    ]')
        self.hold("duplicate bootstrap detector")

    def test_new_registered_bs_case_requires_explicit_partition(self):
        path = "crates/conformance/tests/conformance.rs"
        source = (self.root / path).read_text()
        source += '\n#[test]\nfn conformance_bs_35() {\n    repo_conformance::cases::run("BS-35");\n}\n'
        (self.root / path).write_text(source)
        self.hold("bootstrap detector missing/extra")

    def test_noop_or_wrongly_wired_case_cannot_count_as_coverage(self):
        path = "crates/conformance/tests/conformance.rs"
        self.edit(path, 'repo_conformance::cases::run("BS-09");',
                  'repo_conformance::cases::run("BS-08");')
        self.hold("matching conformance IDs")

    def test_changed_planner_that_starts_all_before_export_is_blocked(self):
        self.edit(".github/ci-plan.sh", 'any($ids[]; startswith("BS-"))',
                  'false')
        self.hold("before its producer")

    def test_wrongly_deferred_independent_rows_are_blocked(self):
        self.edit(".github/ci-plan.sh", 'if .bdd_part == "rest" then false',
                  'if .bdd_part == "rest" then true')
        self.hold("before its producer")

    def test_deleted_producer_dependency_fails(self):
        self.edit(".github/workflows/ci.yml",
                  "    needs: [plan, honesty, bootstrap-export]",
                  "    needs: [plan, honesty]")
        self.hold("producer dependency")

    def test_independent_no_export_guard_must_exist(self):
        self.edit(".github/workflows/ci.yml",
                  '      LEXLEAN_CI_INDEPENDENT_LANE: "1"',
                  '      ANOTHER_ENV: "1"')
        self.hold("lost no-bootstrap-export safeguard")

    def test_duplicate_row_is_blocked(self):
        path = ".github/vv-rows.json"
        rows = json.loads((self.root / path).read_text())
        rows.append(rows[-1])
        (self.root / path).write_text(json.dumps(rows))
        self.hold("duplicated normative row")

    def test_unregistered_part_is_blocked(self):
        path = ".github/vv-rows.json"
        rows = json.loads((self.root / path).read_text())
        row = next(x for x in rows if x["part"] == "bdd-bootstrap-runtime")
        row["bdd_part"] = "unknown"
        (self.root / path).write_text(json.dumps(rows))
        self.hold("BDD partitions and actual vv rows differ")

    def test_duplicate_manifest_json_key_refused(self):
        path = ".github/bdd-parts.json"
        self.edit(path, '"remainder": "rest",',
                  '"remainder": "rest", "remainder": "rest",')
        self.hold("malformed UTF-8 or JSON staging input")

    def test_forged_or_missing_planner_code_refuses(self):
        self.edit(".github/ci-plan.sh", 'classified=$(jq -cn --argjson rows',
                  'classified=$(jq -vn --argjson rows')
        self.hold("actual planner's jq expression missing")

    def test_symlink_input_refused(self):
        path = self.root / ".github/bdd-parts.json"
        path.unlink()
        try:
            path.symlink_to(ROOT / ".github/bdd-parts.json")
        except OSError:
            self.skipTest("symbolic links unavailable")
        self.hold("missing or linked")


if __name__ == "__main__":
    unittest.main()

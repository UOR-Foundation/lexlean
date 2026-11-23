"""A4 nonqualifying candidate classifier, with negative and adversarial cases."""
import copy
import unittest

import ci_incremental_reuse as mod

OLD = "a" * 40
HEAD = "b" * 40
FINGERPRINT = "c" * 64
EVIDENCE = "d" * 64


def sample():
    policy = {
        "schema": mod.SCHEMA, "repository": "UOR-Foundation/lexlean",
        "global_inputs": [],
        "units": [
            {"id": "ordinary", "job": "ordinary test", "host": "ubuntu-24.04",
             "definition": FINGERPRINT, "kind": "ordinary",
             "inputs": ["src/lib.rs"], "explicit_nonimpact": ["notes/readme.md"],
             "needs": []},
            {"id": "downstream", "job": "downstream test", "host": "ubuntu-24.04",
             "definition": FINGERPRINT, "kind": "ordinary",
             "inputs": ["src/helper.rs"], "explicit_nonimpact": ["notes/readme.md", "src/lib.rs"],
             "needs": ["ordinary"]},
            {"id": "proof", "job": "proof verification", "host": "ubuntu-24.04",
             "definition": FINGERPRINT, "kind": "proof",
             "inputs": ["proof.lean"], "explicit_nonimpact": ["notes/readme.md"],
             "needs": []},
        ],
    }
    run_id = 1234
    def receipt(unit):
        return {"unit": unit["id"], "job": unit["job"], "id": {"ordinary": 1, "downstream": 2, "proof": 3}[unit["id"]],
                "run_id": run_id, "head": OLD, "status": "completed",
                "conclusion": "success", "host": unit["host"],
                "definition": FINGERPRINT, "evidence_digest": EVIDENCE}
    observed = {
        "schema": mod.RUN_SCHEMA,
        "run": {"repository": policy["repository"], "id": run_id, "head": OLD,
                "status": "completed", "conclusion": "failure"},
        "jobs": [receipt(u) for u in policy["units"]],
    }
    snapshot = {"old": OLD, "head": HEAD,
                "changed": ["notes/readme.md"], "identical_tree": False}
    return policy, observed, snapshot


def result(mutate=None):
    p, r, s = sample()
    if mutate:
        mutate(p, r, s)
    return mod.classify(p, r, s)


class ReuseContract(unittest.TestCase):
    def test_read_only_advisory_only(self):
        got = result()
        self.assertEqual(got["qualification"], "NONQUALIFYING_NO_MERGE_OR_RELEASE_AUTHORITY")
        self.assertEqual(got["provider_authentication"], "NOT_VERIFIED")
        self.assertEqual(got["source_dependency_closure"], "DECLARED_NOT_PROVEN")
        self.assertEqual(got["candidate_count"], 2)
        self.assertEqual(got["rerun_count"], 1)
        self.assertEqual(got["units"]["proof"]["action"], "RERUN")

    def test_source_change_invalidates_unit_and_downstream(self):
        def change(_, __, s):
            s["changed"] = ["src/lib.rs"]
        got = result(change)
        self.assertIn("declared_input_changed", got["units"]["ordinary"]["reasons"])
        self.assertIn("dependency_invalidated", got["units"]["downstream"]["reasons"])

    def test_unknown_path_fails_closed(self):
        def change(_, __, s):
            s["changed"] = ["scripts/unknown.py"]
        got = result(change)
        self.assertEqual(got["candidate_count"], 0)
        self.assertIn("unknown_change_impact", got["units"]["ordinary"]["reasons"])

    def test_global_workflow_or_toolchain_change_refuses_all(self):
        for item in ("Justfile", "rust-toolchain.toml", ".github/workflows/ci.yml",
                     ".github/scripts/ci_incremental_reuse.py", "SPEC.md"):
            with self.subTest(item=item):
                def change(_, __, s):
                    s["changed"] = [item]
                self.assertEqual(result(change)["candidate_count"], 0)

    def test_rp14_shared_harness_cannot_be_marked_nonimpact(self):
        shared = "crates/conformance/src/cases/repository.rs"
        def falsely_exempt(p, _, s):
            s["changed"] = [shared]
            for unit in p["units"]:
                unit["explicit_nonimpact"].append(shared)
        got = result(falsely_exempt)
        self.assertEqual(got["candidate_count"], 0)
        for unit in got["units"].values():
            self.assertIn("global_policy_or_toolchain_change", unit["reasons"])

    def test_failed_or_cancelled_job_is_not_reuse(self):
        def change(_, r, __):
            r["jobs"][0]["conclusion"] = "failure"
        got = result(change)
        self.assertIn("origin_job_not_successful", got["units"]["ordinary"]["reasons"])
        self.assertEqual(got["units"]["downstream"]["action"], "RERUN")

    def test_in_progress_or_cancelled_run_cannot_supply_success(self):
        for status, conclusion in (("in_progress", None), ("completed", "cancelled")):
            with self.subTest(status=status):
                def change(_, r, __):
                    r["run"]["status"] = status
                    r["run"]["conclusion"] = conclusion
                self.assertEqual(result(change)["candidate_count"], 0)

    def test_missing_receipt_refuses_and_propagates(self):
        def change(_, r, __):
            r["jobs"] = [j for j in r["jobs"] if j["unit"] != "ordinary"]
        got = result(change)
        self.assertIn("missing_receipt", got["units"]["ordinary"]["reasons"])
        self.assertIn("dependency_invalidated", got["units"]["downstream"]["reasons"])

    def test_foreign_head_and_run_are_refused(self):
        for key in ("head", "id", "repository"):
            with self.subTest(key=key):
                def change(_, r, __):
                    r["run"][key] = "another" if key == "repository" else (555 if key == "id" else "f" * 40)
                with self.assertRaises(mod.Hold):
                    result(change)

    def test_forged_host_or_definition_reexecutes(self):
        for key, value in (("host", "windows-2022"),
                           ("definition", "e" * 64), ("job", "foreign test")):
            with self.subTest(key=key):
                def change(_, r, __):
                    r["jobs"][0][key] = value
                self.assertIn("receipt_definition_or_host_mismatch",
                              result(change)["units"]["ordinary"]["reasons"])

    def test_duplicate_job_id_or_unit_rejected(self):
        for key in ("id", "unit"):
            with self.subTest(key=key):
                def change(_, r, __):
                    r["jobs"][1][key] = r["jobs"][0][key]
                with self.assertRaises(mod.Hold):
                    result(change)

    def test_truncated_or_undeclared_dependency_refused(self):
        def change(p, _, __):
            p["units"][1]["needs"] = ["omitted"]
        with self.assertRaises(mod.Hold):
            result(change)

    def test_dependency_cycle_refused(self):
        def change(p, _, __):
            p["units"][0]["needs"] = ["downstream"]
        with self.assertRaisesRegex(mod.Hold, "cycle"):
            result(change)

    def test_nonimpact_cannot_mask_same_path_declared_input(self):
        def change(p, _, __):
            p["units"][0]["explicit_nonimpact"].append("src/lib.rs")
        with self.assertRaisesRegex(mod.Hold, "contradictory"):
            result(change)

    def test_policy_global_addition_invalidates(self):
        def change(p, _, __):
            p["global_inputs"] = ["notes/readme.md"]
        self.assertEqual(result(change)["candidate_count"], 0)

    def test_missing_evidence_digest_and_bad_status_refuse(self):
        for key, value in (("evidence_digest", ""),
                           ("status", "invisible"),
                           ("head", HEAD)):
            with self.subTest(key=key):
                def change(_, r, __):
                    r["jobs"][0][key] = value
                with self.assertRaises(mod.Hold):
                    result(change)

    def test_identical_tree_requires_empty_diff(self):
        def change(_, __, s):
            s["identical_tree"] = True
        with self.assertRaisesRegex(mod.Hold, "tree/diff"):
            result(change)

    def test_same_commit_without_diff_still_has_no_authority(self):
        def change(p, r, s):
            s["head"] = s["old"]
            s["changed"] = []
            s["identical_tree"] = True
        got = result(change)
        self.assertEqual(got["candidate_count"], 2)
        self.assertIn("NONQUALIFYING", got["qualification"])

    def test_rename_includes_both_endpoints(self):
        data = b"R100\0notes/old.md\0notes/new.md\0"
        self.assertEqual(mod.changed_paths(data), ["notes/new.md", "notes/old.md"])

    def test_unsafe_and_partial_git_diff_rejected(self):
        for bad in (b"M\0../bad\0", b"M\0bad", b"X\0src/lib.rs\0",
                    b"R100\0old.md\0", b"M\0x\\y\0"):
            with self.subTest(data=bad):
                with self.assertRaises(mod.Hold):
                    mod.changed_paths(bad)

    def test_duplicate_json_and_nonfinite_refused(self):
        import tempfile
        from pathlib import Path
        for raw in ('{"a":1,"a":2}', '{"x":NaN}'):
            with tempfile.TemporaryDirectory() as tmp:
                filename = Path(tmp) / "test.json"
                filename.write_text(raw)
                with self.assertRaises(mod.Hold):
                    mod.load(filename)


if __name__ == "__main__":
    unittest.main()

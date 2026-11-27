import unittest
from pathlib import Path

from ci_gate_graph import (AuditHold, ATLAS_JOBS, atlas_graph, job_dependencies,
                           strict_json, supported_hosts, build)


def atlas_fixture():
    shards = {}
    jobs = {"plan": "", "honesty": ""}
    for job, group in ATLAS_JOBS.items():
        for name in group:
            if name == "join":
                needs = [n for group2 in ATLAS_JOBS.values() for n in group2 if n != "join"]
            elif name == "prefix":
                needs = []
            else:
                needs = ["prefix"]
            shards[name] = {"needs": needs, "elaborate": [name], "replay": []}
        if len(group) > 1:
            matrix = "    strategy: { matrix: { shard: [" + ", ".join(group) + "] } }\n"
            selector = "    LEXLEAN_EXAMPLES_PART: shard:uor-atlas:" + "$" + "{{ matrix.shard }}\n"
        elif group == ("join",):
            matrix = ""
            selector = "    LEXLEAN_EXAMPLES_PART: join:uor-atlas\n"
        else:
            matrix = ""
            selector = "    LEXLEAN_EXAMPLES_PART: shard:uor-atlas:" + group[0] + "\n"
        deps = ["plan", "honesty"] if job == "atlas-prefix" else ["plan", "atlas-prefix"]
        if job == "atlas-join":
            deps = ["plan"] + [j for j in ATLAS_JOBS if j != "atlas-join"]
        jobs[job] = ("    if: " + "$" + "{{ needs.plan.outputs.full == 'true' }}\n" +
                     "    needs: [" + ", ".join(deps) + "]\n" + matrix + selector)
    return {"project": "uor-atlas", "shards": shards}, jobs


def host_fixture():
    parts = {"compiler": ["BS-01"], "preservation": ["SP-02"]}
    rows = [{"part": "honesty"}, {"part": "examples"}, {"part": "rest"},
            {"part": "bdd-compiler"}, {"part": "bdd-preservation"}]
    lanes = {"producer": ["honesty"],
             "lanes": {"lower": ["rest", "compiler"], "print": ["preservation"]}}
    jobs = {
        "plan": "",
        "host-matrix": "    runs-on: ubuntu-24.04-arm\n    target: aarch64-unknown-linux-gnu\n",
        "intel-part": "    name: Intel conformance\n    runs-on: macos-15-intel\n",
        "intel-join": "    needs: [plan, intel-part]\n    name: build and non-Lean tests (x86_64-apple-darwin)\n",
        "windows-part": "    name: Windows tests\n    runs-on: windows-2022\n",
        "windows-join": "    needs: [plan, windows-part]\n    name: build and non-Lean tests (x86_64-pc-windows-msvc)\n",
        "macos-part": "    name: macOS arm64 tests\n    runs-on: macos-15\n",
        "macos-join": "    needs: [plan, macos-part]\n    name: build and non-Lean tests (aarch64-apple-darwin)\n",
    }
    return rows, {"parts": parts}, lanes, jobs


class GraphControls(unittest.TestCase):
    def test_healthy_atlas_still_unqualified(self):
        atlas, jobs = atlas_fixture()
        out = atlas_graph(atlas, jobs)
        self.assertEqual(out["shards"], 14)
        self.assertFalse(out["provider_receipt_parity_verified"])

    def test_atlas_cycle_fails(self):
        atlas, jobs = atlas_fixture()
        atlas["shards"]["prefix"]["needs"] = ["cert4"]
        with self.assertRaisesRegex(AuditHold, "cycle"):
            atlas_graph(atlas, jobs)

    def test_unowned_atlas_shard_fails(self):
        atlas, jobs = atlas_fixture()
        atlas["shards"]["untracked"] = {"needs": [], "elaborate": [], "replay": []}
        with self.assertRaisesRegex(AuditHold, "mapping"):
            atlas_graph(atlas, jobs)

    def test_missing_atlas_matrix_member_fails(self):
        atlas, jobs = atlas_fixture()
        jobs["atlas-l1"] = jobs["atlas-l1"].replace("census-a, census-b", "census-a")
        with self.assertRaisesRegex(AuditHold, "matrix"):
            atlas_graph(atlas, jobs)

    def test_atlas_missing_required_job_link_fails(self):
        atlas, jobs = atlas_fixture()
        jobs["atlas-cert4"] = jobs["atlas-cert4"].replace("atlas-prefix", "honesty")
        with self.assertRaisesRegex(AuditHold, "not reachable"):
            atlas_graph(atlas, jobs)

    def test_atlas_join_lost_shard_fails(self):
        atlas, jobs = atlas_fixture()
        atlas["shards"]["join"]["needs"].remove("pairs-d")
        with self.assertRaisesRegex(AuditHold, "await every"):
            atlas_graph(atlas, jobs)

    def test_atlas_full_gate_guard_removed_fails(self):
        atlas, jobs = atlas_fixture()
        jobs["atlas-closure"] = jobs["atlas-closure"].replace("needs.plan.outputs.full == 'true'", "true")
        with self.assertRaisesRegex(AuditHold, "full-gate"):
            atlas_graph(atlas, jobs)

    def test_atlas_replay_duplication_fails(self):
        atlas, jobs = atlas_fixture()
        atlas["shards"]["cert4"]["replay"] = ["M", "M"]
        with self.assertRaisesRegex(AuditHold, "duplicate"):
            atlas_graph(atlas, jobs)

    def test_duplicate_elaboration_across_shards_fails(self):
        atlas, jobs = atlas_fixture()
        atlas["shards"]["cert4"]["elaborate"] = ["prefix"]
        with self.assertRaisesRegex(AuditHold, "multiple shards"):
            atlas_graph(atlas, jobs)

    def test_workflow_job_cycle_fails(self):
        with self.assertRaisesRegex(AuditHold, "cycle"):
            job_dependencies({"a": "    needs: [b]\n", "b": "    needs: [a]\n"})

    def test_duplicate_json_key_fails(self):
        with self.assertRaisesRegex(AuditHold, "duplicate JSON"):
            strict_json('{"a":1,"a":2}')

    def test_healthy_host_still_unqualified(self):
        args = host_fixture()
        out = supported_hosts(*args, 'LANES = ".github/intel-host-lanes.json"',
                              "import intel_host_tests as factory")
        self.assertEqual(out["registered_bdd_case_assignments"], 2)
        self.assertFalse(out["native_statuses_policy_verified"])

    def test_duplicate_case_identifier_fails(self):
        rows, bdd, lanes, jobs = host_fixture()
        bdd["parts"]["preservation"] = ["BS-01"]
        with self.assertRaisesRegex(AuditHold, "duplicate"):
            supported_hosts(rows, bdd, lanes, jobs, 'LANES = ".github/intel-host-lanes.json"',
                            "import intel_host_tests as factory")

    def test_missing_host_lane_assignment_fails(self):
        rows, bdd, lanes, jobs = host_fixture()
        lanes["lanes"]["print"] = []
        with self.assertRaisesRegex(AuditHold, "host lane"):
            supported_hosts(rows, bdd, lanes, jobs, 'LANES = ".github/intel-host-lanes.json"',
                            "import intel_host_tests as factory")

    def test_missing_native_status_fails(self):
        rows, bdd, lanes, jobs = host_fixture()
        jobs["windows-join"] = jobs["windows-join"].replace("x86_64-pc-windows-msvc", "wrong")
        with self.assertRaisesRegex(AuditHold, "status"):
            supported_hosts(rows, bdd, lanes, jobs, 'LANES = ".github/intel-host-lanes.json"',
                            "import intel_host_tests as factory")

    def test_source_binding_removed_fails(self):
        rows, bdd, lanes, jobs = host_fixture()
        with self.assertRaisesRegex(AuditHold, "plan source"):
            supported_hosts(rows, bdd, lanes, jobs, 'LANES = "other"',
                            "import intel_host_tests as factory")

    def test_real_checkout(self):
        root = Path(__file__).resolve().parents[2]
        if not (root / "Justfile").exists():
            self.skipTest("test requires GitHub checkout")
        out = build(root)
        self.assertEqual(out["classification"], "AUDIT_ONLY_NO_CUTOVER_AUTHORITY")
        self.assertFalse(out["complete_source_to_normative_obligations"])


if __name__ == "__main__":
    unittest.main()

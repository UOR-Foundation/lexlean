import tempfile
import unittest
from pathlib import Path

import ci_generated_lineage as mod


def fixture():
    just = "".join(name + ":\n    " + cmd + "\n"
                   for name, cmd in mod.WRITE_RECIPES.items())
    regen = (
        'jobs:\n'
        '  writer:\n'
        '    writer: [production-export, records, oracles]\n'
        '    run: |\n'
        '      production-export) recipes="production-export-write"\n'
        '      records) recipes="verify-write"\n'
        '      oracles) recipes="golden-write fixtures-write calculus-write model-write"\n'
        '      just --timestamp something\n'
        '  combine:\n'
        '    needs: [writer]\n'
        '    pattern: regenerate-part-*\n'
        '    name: regenerate-combined\n'
        '  finish:\n'
        '    needs: [combine]\n'
        '    run: cargo xtask counts --write\n'
        '    run: cargo xtask validate-model --write\n'
        '    LEXLEAN_COUNTS=fast\n'
        '    name: regenerated-${{ inputs.ref || github.sha }}\n'
        '  deliver:\n'
        '    needs: [finish]\n'
        '    ref: ${{ inputs.ref || github.sha }}\n'
        '    name: regenerated-${{ inputs.ref || github.sha }}\n'
        '    COMMIT: ${{ inputs.ref || github.sha }}\n'
        '    BRANCH: ${{ inputs.branch }}\n'
        '    run: .github/scripts/deliver-regenerated.sh\n'
    )
    cut = (
        'jobs:\n'
        '  version:\n    run: version\n'
        '  regenerate:\n'
        '    needs: version\n'
        '    uses: ./.github/workflows/regenerate.yml\n'
        '  gate:\n    run: .github/scripts/await-gate.sh\n'
        '  gate-more:\n    run: .github/scripts/await-gate.sh\n'
        '  land:\n'
        '    needs: [version, regenerate, gate, gate-more]\n'
        '    if: needs.gate.outputs.state == \'passed\' or needs.gate-more.outputs.state == \'passed\'\n'
        '    HEAD_SHA: exact\n'
        '  tag:\n'
        '    needs: land\n'
        '    uses: ./.github/workflows/tag.yml\n'
    )
    forward = (
        'jobs:\n'
        '  merge:\n    run: merge\n'
        '  regenerate:\n'
        '    needs: merge\n'
        '    if: needs.merge.outputs.clean == \'true\'\n'
        '    uses: ./.github/workflows/regenerate.yml\n'
    )
    return {
        'Justfile': just,
        '.github/workflows/ci.yml':
            'jobs:\n  plan:\n    run: plan\n  honesty:\n'
            '    needs: [plan]\n    run: cargo xtask check-freshness\n',
        '.github/workflows/regenerate.yml': regen,
        '.github/workflows/cut.yml': cut,
        '.github/workflows/forward.yml': forward,
        '.github/scripts/deliver-regenerated.sh':
            'git push origin "HEAD:refs/heads/$branch"\n'
            'gh workflow run ci.yml --repo "$repo" --ref "$branch"\n',
        '.github/scripts/await-gate.sh':
            '.github/release-gate.sh\nstate failed\n',
    }


class GeneratedLineageNegativeControls(unittest.TestCase):
    def setUp(self):
        self.sources = fixture()

    def mutate(self, path, old, new):
        original = self.sources[path]
        self.assertIn(old, original)
        self.sources[path] = original.replace(old, new)

    def test_healthy_contract_declares_unproven_closure(self):
        report = mod.assert_lineage(self.sources)
        self.assertFalse(report['source_to_generated_input_closure_complete'])
        self.assertFalse(report['actual_generated_output_inventory_verified'])
        self.assertFalse(report['dispatch_proof_receipts_verified'])
        self.assertEqual(len(report['generated_writer_recipes']), 6)

    def test_writer_recipe_change_holds(self):
        self.mutate('Justfile', 'cargo xtask check-golden --write', 'echo skip-generated')
        with self.assertRaisesRegex(mod.AuditHold, 'writer recipe'):
            mod.assert_lineage(self.sources)

    def test_missing_writer_partition_holds(self):
        self.mutate('.github/workflows/regenerate.yml',
                    'writer: [production-export, records, oracles]',
                    'writer: [records, oracles]')
        with self.assertRaisesRegex(mod.AuditHold, 'writer selection'):
            mod.assert_lineage(self.sources)

    def test_missing_combined_writer_evidence_holds(self):
        self.mutate('.github/workflows/regenerate.yml',
                    'pattern: regenerate-part-*', 'pattern: regenerate-part-records')
        with self.assertRaisesRegex(mod.AuditHold, 'combine'):
            mod.assert_lineage(self.sources)

    def test_removed_counts_freshness_generation_holds(self):
        self.mutate('.github/workflows/regenerate.yml',
                    'cargo xtask counts --write', 'echo omit-counts')
        with self.assertRaisesRegex(mod.AuditHold, 'final oracle'):
            mod.assert_lineage(self.sources)

    def test_wrong_regenerated_commit_is_not_delivered(self):
        self.mutate('.github/workflows/regenerate.yml',
                    'COMMIT: ${{ inputs.ref || github.sha }}',
                    'COMMIT: ${{ github.ref_name }}')
        with self.assertRaisesRegex(mod.AuditHold, 'exact source identity'):
            mod.assert_lineage(self.sources)

    def test_wrong_delivery_branch_refuses(self):
        self.mutate('.github/workflows/regenerate.yml',
                    'BRANCH: ${{ inputs.branch }}', 'BRANCH: main')
        with self.assertRaisesRegex(mod.AuditHold, 'delivery'):
            mod.assert_lineage(self.sources)

    def test_missing_exact_artifact_refuses(self):
        self.mutate('.github/workflows/regenerate.yml',
                    'name: regenerated-${{ inputs.ref || github.sha }}',
                    'name: regenerated-floating')
        with self.assertRaisesRegex(mod.AuditHold, 'artifact'):
            mod.assert_lineage(self.sources)

    def test_delivery_no_ci_dispatch_holds(self):
        self.mutate('.github/scripts/deliver-regenerated.sh',
                    'gh workflow run ci.yml', 'echo did-not-dispatch')
        with self.assertRaisesRegex(mod.AuditHold, 'dispatched'):
            mod.assert_lineage(self.sources)

    def test_unqualified_cut_landing_holds(self):
        self.mutate('.github/workflows/cut.yml',
                    "needs.gate.outputs.state == 'passed'", "true")
        with self.assertRaisesRegex(mod.AuditHold, 'gate-pass'):
            mod.assert_lineage(self.sources)

    def test_waiter_cannot_drop_exact_gate_validation(self):
        self.mutate('.github/scripts/await-gate.sh',
                    '.github/release-gate.sh', 'echo apparently-green')
        with self.assertRaisesRegex(mod.AuditHold, 'whole-tree'):
            mod.assert_lineage(self.sources)

    def test_forward_without_regeneration_holds(self):
        self.mutate('.github/workflows/forward.yml',
                    'uses: ./.github/workflows/regenerate.yml',
                    'run: echo skip')
        with self.assertRaisesRegex(mod.AuditHold, 'forwarding'):
            mod.assert_lineage(self.sources)

    def test_ci_without_generated_freshness_holds(self):
        self.mutate('.github/workflows/ci.yml',
                    'cargo xtask check-freshness', 'echo trust-stale-oracles')
        with self.assertRaisesRegex(mod.AuditHold, 'freshness'):
            mod.assert_lineage(self.sources)

    def test_real_repository_source_bindings(self):
        root = Path(__file__).resolve().parents[2]
        if not (root / 'Justfile').is_file():
            self.skipTest('requires full checked-out source')
        report = mod.build(root)
        self.assertEqual(report['classification'],
                         'AUDIT_ONLY_NO_QUALIFICATION_AUTHORITY')
        self.assertFalse(report['source_to_generated_input_closure_complete'])


if __name__ == '__main__':
    unittest.main()

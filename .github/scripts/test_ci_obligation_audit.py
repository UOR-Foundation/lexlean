import json
import tempfile
import unittest
from pathlib import Path

import ci_obligation_audit as mod


def fixture(root):
    just = (
        'vv: fmt-check bdd examples repro\n'
        'fmt-check:\n    cargo fmt --all -- --check\n'
        'bdd:\n    cargo xtask bdd\n'
        'examples:\n    cargo xtask verify-examples\n'
        'repro:\n    cargo xtask check-reproducibility\n'
    )
    rows = [
        {'part': 'honesty', 'recipes': 'fmt-check bdd', 'bdd_part': 'honesty'},
        {'part': 'examples', 'recipes': 'examples', 'bdd_part': ''},
        {'part': 'rest', 'recipes': 'bdd repro', 'bdd_part': 'rest'},
        {'part': 'bdd-compiler', 'recipes': 'bdd', 'bdd_part': 'compiler'},
    ]
    ci = 'jobs:\n'
    for name in sorted(mod.REQUIRED_JOBS):
        ci += '  ' + name + ':\n    runs-on: ubuntu-24.04\n'
        if name == 'vv-fast':
            ci += ('    needs: [plan, honesty]\n    strategy:\n      matrix:\n'
                   '        include: ' + '$' + '{{ fromJSON(needs.plan.outputs.fanout_fast) }}\n')
        if name == 'vv':
            ci += ('    needs: [plan, honesty, bootstrap-export]\n    strategy:\n'
                   '      matrix:\n        include: ' + '$' +
                   '{{ fromJSON(needs.plan.outputs.fanout_export) }}\n')
    repro = ("jobs:\n  repro:\n    if: " + "$" +
             "{{ github.event_name != 'pull_request' || !github.event.pull_request.draft }}\n" +
             "    steps:\n      - run: just repro\n")
    sources = {
        'Justfile': just,
        '.github/vv-rows.json': json.dumps(rows),
        '.github/bdd-parts.json': json.dumps({'remainder': 'rest', 'parts': {'compiler': ['X-01']}}),
        '.github/workflows/ci.yml': ci,
        '.github/workflows/reproducibility.yml': repro,
    }
    for path, contents in sources.items():
        out = root / path
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(contents)
    return sources


class AuditTests(unittest.TestCase):
    def setUp(self):
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        self.root = Path(temp.name)
        fixture(self.root)

    def modify(self, path, old, new):
        file = self.root / path
        original = file.read_text()
        self.assertIn(old, original)
        file.write_text(original.replace(old, new))

    def test_healthy_audit_still_nonqualifying(self):
        result = mod.audit(self.root)
        self.assertEqual(result['classification'], 'AUDIT_ONLY_NOT_QUALIFICATION')
        self.assertFalse(result['complete_source_to_normative_obligations'])
        self.assertEqual(result['vv_rows'], 4)
        self.assertEqual(result['bdd_partitions'], 1)
        self.assertFalse(result['cross_workflow_overlap'][0]['safe_to_eliminate'])
        self.assertEqual(len(result['sources_sha256']), 5)

    def test_duplicate_recipe_runs_hold(self):
        self.modify('.github/vv-rows.json', '"bdd repro"', '"bdd repro repro"')
        with self.assertRaisesRegex(mod.AuditHold, 'non-BDD recipe'):
            mod.audit(self.root)

    def test_missing_partition_holds(self):
        self.modify('.github/vv-rows.json', '"part": "bdd-compiler"', '"part": "bdd-new"')
        with self.assertRaises(mod.AuditHold):
            mod.audit(self.root)

    def test_independent_repro_status_holds(self):
        self.modify('.github/workflows/reproducibility.yml', '- run: just repro', '- run: echo skip')
        with self.assertRaisesRegex(mod.AuditHold, 'repro invocation changed'):
            mod.audit(self.root)

    def test_nested_terminal_command_mutation_holds(self):
        self.modify('Justfile', 'cargo xtask check-reproducibility', 'echo pretend-green')
        with self.assertRaisesRegex(mod.AuditHold, 'repro terminal command'):
            mod.audit(self.root)

    def test_bootstrap_dependency_mutation_holds(self):
        self.modify('.github/workflows/ci.yml',
                    'needs: [plan, honesty, bootstrap-export]', 'needs: [plan, honesty]')
        with self.assertRaisesRegex(mod.AuditHold, 'producer dependency'):
            mod.audit(self.root)

    def test_missing_supported_host_qualifier_holds(self):
        self.modify('.github/workflows/ci.yml', '  windows-join:\n', '  windows-no-join:\n')
        with self.assertRaisesRegex(mod.AuditHold, 'job removed'):
            mod.audit(self.root)

    def test_missing_recipe_holds(self):
        self.modify('Justfile', 'vv: fmt-check bdd examples repro', 'vv: fmt-check bdd examples')
        with self.assertRaisesRegex(mod.AuditHold, 'vv recipe dependencies'):
            mod.audit(self.root)

    def test_real_checkout_obligation_inventory(self):
        repo = Path(__file__).resolve().parents[2]
        if not (repo / 'Justfile').is_file():
            self.skipTest('requires a checked-out source tree')
        result = mod.audit(repo)
        self.assertEqual(result['classification'], 'AUDIT_ONLY_NOT_QUALIFICATION')
        self.assertFalse(result['complete_source_to_normative_obligations'])


if __name__ == '__main__':
    unittest.main()

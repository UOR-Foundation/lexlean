import importlib.util
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).with_name('ci_impact_preview.py')
spec = importlib.util.spec_from_file_location('impact', SCRIPT)
impact = importlib.util.module_from_spec(spec)
spec.loader.exec_module(impact)
BASE = 'a' * 40
HEAD = 'b' * 40


def diff(*records):
    return b''.join((field.encode() + b'\0') for record in records for field in record)


class PreviewContract(unittest.TestCase):
    def test_rust_changes_run_real_format_without_claiming_proof(self):
        plan = impact.preview(BASE, HEAD, diff(('M', 'crates/lexlean/src/lib.rs')))
        self.assertEqual(plan['selected_feedback'], ['diff-whitespace', 'rust-format'])
        self.assertTrue(plan['requires_normative_full_gate_for_merge'])
        self.assertEqual(plan['qualification'], 'NONQUALIFYING')

    def test_renames_use_both_sides(self):
        plan = impact.preview(BASE, HEAD, diff(('R100', 'crates/old/src/lib.rs', '.github/scripts/renamed.py')))
        self.assertIn('rust-format', plan['selected_feedback'])
        self.assertIn('ci-python-contracts', plan['selected_feedback'])
        self.assertEqual(len(plan['changed']), 2)

    def test_python_scripts_select_real_python_contracts(self):
        plan = impact.preview(BASE, HEAD, diff(('A', '.github/scripts/test_intel_host_tests.py')))
        self.assertEqual(plan['selected_feedback'], ['diff-whitespace', 'ci-python-contracts'])

    def test_model_and_readme_never_claim_passive(self):
        for path in ('README.md', 'src/Main.lex.tex', 'Cargo.lock', 'SPEC.md', '.github/workflows/ci.yml'):
            with self.subTest(path=path):
                plan = impact.preview(BASE, HEAD, diff(('M', path)))
                self.assertEqual(plan['selected_feedback'], ['diff-whitespace'])
                self.assertEqual(plan['not_classified_for_fast_feedback'], [path])
                self.assertTrue(plan['requires_normative_full_gate_for_merge'])

    def test_rust_deletion_still_format_checks(self):
        plan = impact.preview(BASE, HEAD, diff(('D', 'crates/lexlean/src/old.rs')))
        self.assertIn('rust-format', plan['selected_feedback'])

    def test_bad_diff_status_and_truncation_refused(self):
        for payload in (b'U\0x\0', b'R100\0only\0', b'M\0x', b'M\0\0'):
            with self.subTest(payload=payload):
                with self.assertRaises(ValueError):
                    impact.preview(BASE, HEAD, payload)

    def test_invalid_paths_refused(self):
        for path in ('../bad.rs', '/tmp/fake.py', 'docs//file.md', './spec', 'folder/../other.rs'):
            with self.subTest(path=path):
                with self.assertRaises(ValueError):
                    impact.preview(BASE, HEAD, diff(('M', path)))

    def test_bad_or_missing_revision_refused(self):
        with self.assertRaises(ValueError):
            impact.preview('main', HEAD, b'')
        with self.assertRaises(ValueError):
            impact.preview(BASE, 'bad', b'')

    def test_empty_diff_runs_no_runner_heavy_verifier(self):
        plan = impact.preview(BASE, HEAD, b'')
        self.assertEqual(plan['selected_feedback'], [])
        self.assertTrue(plan['requires_normative_full_gate_for_merge'])

    def test_cli_writes_boolean_outputs_and_nonqualifying_summary(self):
        with tempfile.TemporaryDirectory() as root:
            d = Path(root)
            (d / 'changed.z').write_bytes(diff(('M', 'crates/a/src/lib.rs')))
            cmd = [sys.executable, str(SCRIPT), '--base', BASE, '--head', HEAD,
                   '--diff-nul', str(d / 'changed.z'), '--github-output', str(d / 'out'),
                   '--summary', str(d / 'summary')]
            ret = subprocess.run(cmd, capture_output=True, text=True)
            self.assertEqual(ret.returncode, 0, ret.stderr)
            self.assertIn('rust_format=true', (d / 'out').read_text())
            self.assertIn('python_contracts=false', (d / 'out').read_text())
            self.assertIn('NONQUALIFYING', (d / 'summary').read_text())
            self.assertEqual(json.loads(ret.stdout)['head'], HEAD)

    def test_workflow_has_no_automatic_trigger_or_privileged_authority(self):
        text = (Path(__file__).parents[1] / 'workflows' / 'development-feedback.yml').read_text()
        header = text.split('\njobs:\n', 1)[0]
        self.assertIn('\n  workflow_dispatch:\n', header)
        self.assertNotIn('\n  pull_request:', header)
        self.assertNotIn('\n  push:', header)
        self.assertNotIn('\n  schedule:', header)
        self.assertIn('\npermissions:\n  contents: read\n', header)
        self.assertIn('persist-credentials: false', text)
        self.assertNotIn('pull_request_target:', text)
        self.assertIn('if: ${{ steps.plan.outputs.rust_format == ', text)
        self.assertIn('if: ${{ steps.plan.outputs.python_contracts == ', text)
        self.assertIn('NONQUALIFYING', text)
        self.assertIn('ci_impact_preview.py', text)

    def test_non_utf8_and_merge_conflict_do_not_get_cheap_clearance(self):
        with self.assertRaises(ValueError):
            impact.preview(BASE, HEAD, b'M\0bad-\xff.rs\0')
        with self.assertRaises(ValueError):
            impact.preview(BASE, HEAD, b'U\0both-sides.rs\0')

    def test_real_git_rename_diff_selects_both_impacted_languages(self):
        with tempfile.TemporaryDirectory() as dirname:
            root = Path(dirname)
            def git(*arguments):
                return subprocess.check_output(
                    ['git', '-C', str(root), *arguments])
            git('init', '-q')
            git('config', 'user.email', 'fixture@example.invalid')
            git('config', 'user.name', 'Fixture')
            old = root / 'crates/example/src/lib.rs'
            old.parent.mkdir(parents=True)
            old.write_text('# generated fixture\n')
            git('add', '.')
            git('commit', '-qm', 'baseline')
            base = git('rev-parse', 'HEAD').decode().strip()
            (root / '.github/scripts').mkdir(parents=True)
            git('mv', 'crates/example/src/lib.rs', '.github/scripts/renamed.py')
            git('commit', '-qm', 'rename')
            head = git('rev-parse', 'HEAD').decode().strip()
            actual_diff = git('diff', '--name-status', '-z', '-M',
                              '--no-ext-diff', base, head)
            report = impact.preview(base, head, actual_diff)
            self.assertEqual(report['selected_feedback'], [
                'diff-whitespace', 'rust-format', 'ci-python-contracts'])
            self.assertEqual(report['qualification'], 'NONQUALIFYING')

    def test_cargo_graph_requires_explicit_source_and_remains_nonqualifying(self):
        from test_ci_cargo_scope import fixture
        with tempfile.TemporaryDirectory() as dirname:
            root = Path(dirname)
            fixture(root)
            result = impact.preview(BASE, HEAD, diff(('M', 'crates/lexlean/src/lib.rs')),
                                    repo_root=root)
            self.assertIn('cargo-check', result['selected_feedback'])
            self.assertEqual(result['cargo_check_packages'], [
                'lexlean', 'repo-conformance', 'xtask'])
            self.assertEqual(result['qualification'], 'NONQUALIFYING')

    def test_cargo_graph_failure_holds_instead_of_cheap_clearance(self):
        with tempfile.TemporaryDirectory() as dirname:
            with self.assertRaises(ValueError):
                impact.preview(BASE, HEAD, diff(('M', 'crates/model/src/lib.rs')),
                               repo_root=Path(dirname))

    def test_no_network_or_shell_execution_in_selector(self):
        source = SCRIPT.read_text()
        self.assertNotIn('shell=True', source)
        self.assertNotIn('requests.', source)
        self.assertNotIn('subprocess.', source)


if __name__ == '__main__':
    unittest.main()

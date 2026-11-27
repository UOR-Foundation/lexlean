"""Guard the nonqualifying Intel evidence checks and their report artifact."""
import unittest
from pathlib import Path

SOURCE = Path(__file__).resolve().parent.parent / 'workflows' / 'development-feedback.yml'
MARKER = '      - name: Validate historical Intel CI evidence'
GUARD = "        if: ${{ steps.plan.outputs.python_contracts == 'true' }}"
REQUIRED = (
    'set -euo pipefail',
    'ci_intel_rebalance.py',
    '--evidence .github/metrics/ci-intel-historical-lanes-20261123.json',
    '> ci-evidence/ci-before-after/intel-lane-scenarios.json',
    'ci_intel_tail_audit.py',
    '--ledger .github/metrics/ci-intel-case-tail-20261123.json',
    '> ci-evidence/ci-before-after/intel-case-tail.json',
)

def check(text):
    assert text.count(MARKER) == 1
    start = text.index(MARKER)
    upload = text.index('      - uses: actions/upload-artifact@', start)
    block = text[start:upload]
    assert GUARD in block
    assert '        shell: bash' in block
    assert 'continue-on-error:' not in block
    assert all(part in block for part in REQUIRED)
    assert '      - name: Generate the observed before-after CI report' in text[:start]
    assert '          name: ci-before-after-evidence' in text[upload:]

class EvidenceWiringTest(unittest.TestCase):
    def setUp(self):
        self.source = SOURCE.read_text()

    def test_actual_source(self):
        check(self.source)

    def test_missing_guard_refuses(self):
        with self.assertRaises(AssertionError):
            check(self.source.replace(MARKER + '\n' + GUARD, MARKER, 1))

    def test_removed_verifier_refuses(self):
        with self.assertRaises(AssertionError):
            check(self.source.replace('ci_intel_tail_audit.py', 'missing.py'))

    def test_failure_suppression_refuses(self):
        with self.assertRaises(AssertionError):
            check(self.source.replace(MARKER, MARKER + '\n        continue-on-error: true'))

    def test_artifact_path_mutation_refuses(self):
        with self.assertRaises(AssertionError):
            check(self.source.replace('intel-lane-scenarios.json', 'lost.json'))

    def test_duplicate_step_refuses(self):
        with self.assertRaises(AssertionError):
            check(self.source.replace(MARKER, MARKER + '\n' + MARKER))

if __name__ == '__main__':
    unittest.main()

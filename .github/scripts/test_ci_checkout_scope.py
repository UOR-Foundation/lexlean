"""Read-only structural and adversarial checks for paired checkout trial."""
from pathlib import Path
import unittest

WORKFLOW = Path(__file__).resolve().parent.parent / 'workflows' / 'development-feedback.yml'
ALT = "github.event_name == 'pull_request' && startsWith(github.event.pull_request.body, '<!-- ci-preview-head-only -->')"
BASE = "github.event_name != 'pull_request' || !startsWith(github.event.pull_request.body, '<!-- ci-preview-head-only -->')"
CHECKOUT = '      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1'
RESTORE = '      - name: Restore complete exact-head ancestry for head-only checkout'
PLAN = '      - name: Pin trusted-base comparison and decide real feedback commands'

def prove(text):
    assert text.count(CHECKOUT) == 2
    assert text.count(RESTORE) == 1
    assert text.count(PLAN) == 1
    assert text.count("<!-- ci-preview-head-only -->") == 4
    assert text.count("<!-- ci-preview-once -->") == 1
    assert text.index('Bounded stale-base preflight before checkout') < text.index(CHECKOUT)
    before, after = text.split(RESTORE, 1)
    assert before.count('if: ${{ ' + BASE + ' }}') == 1
    assert before.count('if: ${{ ' + ALT + ' }}') == 1
    assert after.count('if: ${{ ' + ALT + ' }}') == 1
    assert before.count('fetch-depth: 0') == 1
    assert before.count('fetch-depth: 1') == 1
    assert before.count('filter: blob:none') == 2
    assert before.count('persist-credentials: false') == 2
    restore = after.split(PLAN, 1)[0]
    for required in ('set -euo pipefail', 'timeout 120 git -c protocol.version=2 fetch',
                     '--no-tags --filter=blob:none --unshallow origin',
                     'test "$(git rev-parse --is-shallow-repository)" = false',
                     'git merge-base --is-ancestor "$EXPECTED_BASE" "$EXPECTED_HEAD"'):
        assert required in restore
    assert 'test "$(git rev-parse --verify HEAD)" = "$EXPECTED_HEAD"' in restore
    assert 'continue-on-error:' not in restore
    assert text.count('git merge-base --is-ancestor') == 2
    assert text.count('timeout-minutes: 20') == 1
    assert text.index(RESTORE) < text.index(PLAN)
    assert 'python3 .github/scripts/ci_intel_tail_audit.py' in text

class CheckoutTrial(unittest.TestCase):
    def setUp(self):
        self.text = WORKFLOW.read_text()

    def test_yaml_plain_if_cannot_contain_colon_space_marker(self):
        # A colon followed by space inside an unquoted YAML scalar is invalid,
        # even when enclosed in *expression* string quotes.
        for line in self.text.splitlines():
            if line.strip().startswith('if: ${{'):
                self.assertNotIn(': ', line.split('if: ', 1)[1])

    def test_real_workflow(self): prove(self.text)
    def test_missing_restore(self):
        with self.assertRaises(AssertionError): prove(self.text.replace(RESTORE, '      - name: unbound restore'))
    def test_lost_ancestry_guard(self):
        with self.assertRaises(AssertionError): prove(self.text.replace('git merge-base --is-ancestor "$EXPECTED_BASE" "$EXPECTED_HEAD"', 'test true'))
    def test_lost_unshallow(self):
        with self.assertRaises(AssertionError): prove(self.text.replace('--unshallow origin', 'origin'))
    def test_baseline_mode_weakened(self):
        with self.assertRaises(AssertionError): prove(self.text.replace('fetch-depth: 0', 'fetch-depth: 1'))
    def test_candidate_mode_weakened(self):
        with self.assertRaises(AssertionError): prove(self.text.replace('fetch-depth: 1', 'fetch-depth: 0'))
    def test_candidate_guard_missing(self):
        with self.assertRaises(AssertionError): prove(self.text.replace('if: ${{ ' + ALT + ' }}', 'if: ${{ false }}'))
    def test_baseline_guard_missing(self):
        with self.assertRaises(AssertionError): prove(self.text.replace('if: ${{ ' + BASE + ' }}', 'if: ${{ true }}'))
    def test_restored_repo_must_be_unshallow(self):
        with self.assertRaises(AssertionError): prove(self.text.replace('test "$(git rev-parse --is-shallow-repository)" = false', ':'))
    def test_lost_exact_head_binding(self):
        with self.assertRaises(AssertionError): prove(self.text.replace('test "$(git rev-parse --verify HEAD)" = "$EXPECTED_HEAD"', ':'))

if __name__ == '__main__': unittest.main()

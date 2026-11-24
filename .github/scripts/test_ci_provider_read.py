"""Adversarial controls for bounded, atomic provider reads (CI #603)."""
import os
import subprocess
import tempfile
import unittest
from pathlib import Path


class ProviderReadTests(unittest.TestCase):
    def run_case(self, failures, success):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            stub = root / 'gh'
            stub.write_text('''#!/usr/bin/env bash
set -eu
[ "$1" = api ] && [ "$2" = 'endpoint?per_page=100' ] && [ "$3" = --paginate ]
n=0
[ ! -f "$CALLS" ] || n=$(cat "$CALLS")
n=$((n+1))
printf '%s' "$n" > "$CALLS"
if [ "$n" -le "$FAILURES" ]; then
  printf 'untrusted partial page\\n'
  printf 'HTTP 502\\n' >&2
  exit 1
fi
printf '%s' "$SUCCESS"
''')
            stub.chmod(0o755)
            sleep = root / 'sleep'
            sleep.write_text('#!/bin/sh\nexit 0\n')
            sleep.chmod(0o755)
            helper = Path(__file__).with_name('ci-provider-read.sh').resolve()
            env = dict(os.environ, PATH=f'{root}:{os.environ["PATH"]}',
                       CALLS=str(root / 'calls'), FAILURES=str(failures), SUCCESS=success)
            out = subprocess.run(['bash', '-c',
                                  'source "$1"; ci_provider_read "endpoint?per_page=100" --paginate',
                                  'test', str(helper)], env=env, capture_output=True, text=True,
                                 timeout=10)
            return out, int((root / 'calls').read_text())

    def test_transient_failure_recovers_without_partial_output(self):
        out, calls = self.run_case(1, 'complete evidence')
        self.assertEqual((out.returncode, calls, out.stdout), (0, 2, 'complete evidence\n'))
        self.assertIn('HTTP 502', out.stderr)

    def test_persistent_failure_returns_no_evidence(self):
        out, calls = self.run_case(10, 'never accepted')
        self.assertNotEqual(out.returncode, 0)
        self.assertEqual((calls, out.stdout), (3, ''))

    def test_success_and_empty_result_are_not_retried(self):
        for response in ('valid', ''):
            with self.subTest(response=response):
                out, calls = self.run_case(0, response)
                self.assertEqual((out.returncode, calls, out.stdout), (0, 1, response + '\n'))


if __name__ == '__main__':
    unittest.main()

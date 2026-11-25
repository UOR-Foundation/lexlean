"""Exercise the real planner with real Git and simulated GitHub read failures."""
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest


class PlannerProviderRetry(unittest.TestCase):
    def check_plan(self, fail_runs=0, fail_jobs=0, defect='', changed_tree=False,
                   event='push', draft=''):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            def git(*args):
                return subprocess.check_output(['git', '-C', directory, *args],
                                               stderr=subprocess.DEVNULL).decode().strip()
            git('init', '-q', '-b', 'main')
            git('config', 'user.name', 'Test')
            git('config', 'user.email', 'test@example.invalid')
            (root / 'base').write_text('base')
            git('add', '.')
            git('commit', '-qm', 'base')
            git('checkout', '-qb', 'feature')
            (root / 'feature').write_text('feature')
            git('add', '.')
            git('commit', '-qm', 'feature')
            parent = git('rev-parse', 'HEAD')
            git('checkout', '-q', 'main')
            if changed_tree:
                (root / 'other').write_text('other')
                git('add', '.')
                git('commit', '-qm', 'other')
            git('merge', '-q', '--no-ff', '-m', 'merge', 'feature')
            planner = Path(__file__).resolve().parents[1] / 'ci-plan.sh'
            # The `bdd-bootstrap-*` legs are run by `vv-bootstrap`, after the export.
            parts = ['honesty', 'rest', 'bdd-bootstrap-lower', 'examples', 'bdd-bootstrap-check']
            rows = root / 'rows.json'
            rows.write_text(json.dumps([{'part': part} for part in parts]))
            jobs = ['plan', 'honesty', 'just vv (rest)', 'just vv (examples)',
                    'just vv (bdd-bootstrap-lower)', 'just vv (bdd-bootstrap-check)',
                    'atlas (join)']
            jobs += [f'build and non-Lean tests ({host})' for host in (
                'aarch64-unknown-linux-gnu', 'x86_64-apple-darwin',
                'aarch64-apple-darwin', 'x86_64-pc-windows-msvc')]
            if defect == 'missing':
                jobs.remove('just vv (rest)')
            if defect == 'duplicate':
                jobs.append(jobs[-1])
            # A failed export skips `vv-bootstrap` before its matrix expands:
            # the run lists one `skipped` job that names no leg.
            skipped = defect == 'skipped-bootstrap'
            if skipped:
                jobs.remove('just vv (bdd-bootstrap-check)')
                jobs[jobs.index('just vv (bdd-bootstrap-lower)')] = 'just vv (${{ matrix.part }})'
            def conclusion(name):
                if defect == 'failed' and name == 'honesty':
                    return 'failure'
                return 'skipped' if skipped and '${{' in name else 'success'
            listing = '\n'.join(name + '\t' + conclusion(name) for name in jobs)
            (root / 'jobs').write_text(listing)
            bin_dir = root / 'bin'
            bin_dir.mkdir()
            stub = bin_dir / 'gh'
            stub.write_text('''#!/usr/bin/env bash
set -eu
[ "$1" = api ]
case "$2" in
  */workflows/ci.yml/runs\\?*)
    [[ "$2" == *"head_sha=$PARENT&"* ]] || exit 2
    kind=runs; failures=$FAIL_RUNS; output=42 ;;
  */runs/42/jobs\\?*)
    kind=jobs; failures=$FAIL_JOBS; output=$(cat "$ROOT/jobs") ;;
  *) exit 2 ;;
esac
n=0
[ ! -f "$ROOT/$kind.count" ] || n=$(cat "$ROOT/$kind.count")
n=$((n+1))
printf '%s' "$n" > "$ROOT/$kind.count"
# GitHub answered a retried run's 70-job listing with HTTP 502 at 60 or more
# jobs a page, every time (run 45316289312, #603): model that provider limit.
if [ "$kind" = jobs ]; then
  [[ "$2" =~ per_page=([0-9]+) ]] && [ "${BASH_REMATCH[1]}" -le 50 ] \\
    || { printf 'HTTP 502\\n' >&2; exit 1; }
fi
# Simulate even a valid-looking complete response followed by a transport error.
printf '%s\\n' "$output"
if [ "$n" -le "$failures" ]; then
  printf 'HTTP 502\\n' >&2
  exit 1
fi
''')
            stub.chmod(0o755)
            sleep = bin_dir / 'sleep'
            sleep.write_text('#!/bin/sh\nexit 0\n')
            sleep.chmod(0o755)
            out = root / 'out'
            summary = root / 'summary'
            env = dict(os.environ, PATH=f'{bin_dir}{os.pathsep}{os.environ["PATH"]}', ROOT=directory,
                       PARENT=parent, FAIL_RUNS=str(fail_runs), FAIL_JOBS=str(fail_jobs),
                       EVENT=event, DRAFT=draft, REPO='owner/repo', LEXLEAN_CI_ROWS=str(rows),
                       GITHUB_OUTPUT=str(out), GITHUB_STEP_SUMMARY=str(summary))
            proc = subprocess.run(['bash', str(planner)], cwd=directory, env=env,
                                  capture_output=True, text=True, timeout=15)
            self.assertEqual(proc.returncode, 0, proc.stderr)
            result = dict(line.split('=', 1) for line in out.read_text().splitlines())
            def count(kind):
                p = root / f'{kind}.count'
                return int(p.read_text()) if p.exists() else 0
            return result, summary.read_text(), count('runs'), count('jobs')

    def test_transient_run_listing_failure(self):
        r, summary, runs, jobs = self.check_plan(fail_runs=1)
        self.assertEqual((r['full'], r['rows'], runs, jobs), ('false', '[]', 2, 1))
        self.assertIn('run 42', summary)

    def test_transient_job_listing_failure(self):
        r, _, runs, jobs = self.check_plan(fail_jobs=2)
        self.assertEqual((r['full'], r['rows'], runs, jobs), ('false', '[]', 1, 3))

    def test_persistent_error_refuses_valid_looking_partial_output(self):
        for fr, fj in ((3, 0), (0, 3)):
            with self.subTest(fr=fr, fj=fj):
                r, _, runs, jobs = self.check_plan(fail_runs=fr, fail_jobs=fj)
                self.assertEqual(r['full'], 'true')
                self.assertEqual((runs, jobs), (3, 0) if fr else (1, 3))

    def test_recovered_read_does_not_relax_inventory(self):
        for defect in ('missing', 'duplicate', 'failed'):
            with self.subTest(defect=defect):
                r, _, _, jobs = self.check_plan(fail_jobs=1, defect=defect)
                self.assertEqual((r['full'], jobs), ('true', 2))

    def test_skipped_bootstrap_legs_are_no_verified_run(self):
        r, _, _, jobs = self.check_plan(defect='skipped-bootstrap')
        self.assertEqual((r['full'], jobs), ('true', 1))

    def test_outputs_partition_the_rows(self):
        def parts(value):
            return [row['part'] for row in json.loads(value)]
        r, _, _, _ = self.check_plan(changed_tree=True)
        self.assertEqual((parts(r['honesty']), parts(r['fanout']), parts(r['bootstrap'])),
                         (['honesty'], ['rest', 'examples'],
                          ['bdd-bootstrap-lower', 'bdd-bootstrap-check']))
        # A draft runs honesty then `rest`: no export, so no bootstrap leg.
        r, _, runs, jobs = self.check_plan(event='pull_request', draft='true')
        self.assertEqual((r['full'], parts(r['honesty']), parts(r['fanout']),
                          parts(r['bootstrap']), runs, jobs),
                         ('false', ['honesty'], ['rest'], [], 0, 0))

    def test_changed_merge_tree_does_not_query_provider(self):
        r, _, runs, jobs = self.check_plan(changed_tree=True)
        self.assertEqual((r['full'], runs, jobs), ('true', 0, 0))


if __name__ == '__main__':
    unittest.main()

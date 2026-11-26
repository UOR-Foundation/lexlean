"""Exercise the real planner with real Git and simulated GitHub read failures."""
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest


class PlannerProviderRetry(unittest.TestCase):
    # The `bdd-bootstrap-*` legs are run by `vv-bootstrap`, after the export.
    PARTS = ['honesty', 'rest', 'bdd-bootstrap-lower', 'examples', 'bdd-bootstrap-check']

    def check_plan(self, fail_runs=0, fail_jobs=0, defect='', changed_tree=False,
                   event='push', draft='', nonmerge=False, expected_note=None):
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
            if not nonmerge:
                git('merge', '-q', '--no-ff', '-m', 'merge', 'feature')
            planner = Path(__file__).resolve().parents[1] / 'ci-plan.sh'
            rows = root / 'rows.json'
            rows.write_text(json.dumps([{'part': part} for part in self.PARTS]))
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
            if defect == 'atlas':
                jobs.remove('atlas (join)')
            if defect == 'empty_jobs':
                jobs.clear()
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
    kind=runs; failures=$FAIL_RUNS; output=42
    if [ "$DEFECT" = empty_runs ]; then output=; fi ;;
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
                       DEFECT=defect,
                       EVENT=event, DRAFT=draft, REPO='owner/repo', LEXLEAN_CI_ROWS=str(rows),
                       GITHUB_OUTPUT=str(out), GITHUB_STEP_SUMMARY=str(summary))
            proc = subprocess.run(['bash', str(planner)], cwd=directory, env=env,
                                  capture_output=True, text=True, timeout=15)
            self.assertEqual(proc.returncode, 0, proc.stderr)
            if expected_note is not None:
                self.assertIn(expected_note, proc.stderr)
            self.assertNotIn('CI reuse:', out.read_text())
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


    def test_reuse_diagnostics_preserve_gate_decisions(self):
        cases = [
            ({}, 'complete gate accepted', 'false'),
            ({'fail_runs': 3}, 'successful run lookup failed', 'true'),
            ({'fail_jobs': 3}, 'job lookup failed', 'true'),
            ({'defect': 'empty_runs'}, 'no eligible successful run returned', 'true'),
            ({'defect': 'empty_jobs'}, 'empty or non-success job listing', 'true'),
            ({'defect': 'failed'}, 'empty or non-success job listing', 'true'),
            ({'defect': 'atlas'}, 'missing successful Atlas join', 'true'),
            ({'defect': 'duplicate'}, 'required host contexts absent', 'true'),
            ({'defect': 'missing'}, 'missing successful required row rest', 'true'),
        ]
        for kwargs, note, full in cases:
            with self.subTest(**kwargs):
                r, summary, _, _ = self.check_plan(expected_note=note, **kwargs)
                self.assertEqual(r['full'], full)
                if full == 'false':
                    self.assertEqual(r['rows'], '[]')
                    self.assertIn('run 42', summary)
                else:
                    # A refused reuse runs every row, bootstrap legs included.
                    self.assertEqual([row['part'] for row in json.loads(r['rows'])], self.PARTS)

    def test_ancestry_refusals_are_diagnostic_and_do_not_query_provider(self):
        cases = [({'changed_tree': True}, 'HEAD: second-parent tree differs'),
                 ({'nonmerge': True}, 'HEAD: no second parent available')]
        for kwargs, note in cases:
            with self.subTest(**kwargs):
                r, _, runs, jobs = self.check_plan(expected_note=note, **kwargs)
                self.assertEqual((r['full'], runs, jobs), ('true', 0, 0))

class ProviderRunIndex(unittest.TestCase):
    """Use real jq on provider-shaped records; the status-filter index is stale."""

    def exercise(self, script, status='completed', conclusion='success',
                 repository='owner/repo', event='pull_request', bad_job=False,
                 partial=False, wrong_head=False, missing_head=False,
                 later_page=False):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            def git(*args):
                return subprocess.check_output(['git', '-C', directory, *args],
                                               stderr=subprocess.DEVNULL).decode().strip()
            git('init', '-q', '-b', 'main')
            git('config', 'user.name', 'Test')
            git('config', 'user.email', 'test@example.invalid')
            git('commit', '--allow-empty', '-qm', 'base')
            git('checkout', '-qb', 'feature')
            git('commit', '--allow-empty', '-qm', 'feature')
            parent = git('rev-parse', 'HEAD')
            git('checkout', '-q', 'main')
            git('merge', '-q', '--no-ff', '-m', 'merge', 'feature')
            head = git('rev-parse', 'HEAD')
            stamp = '2026-01-01T00:00:00Z'
            def run(id_, status_, conclusion_, repo_, event_):
                return dict(id=id_, status=status_, conclusion=conclusion_,
                            head_repository={'full_name': repo_}, head_sha=parent,
                            event=event_, created_at=stamp, updated_at=stamp,
                            html_url=f'https://github.com/owner/repo/actions/runs/{id_}')
            records = [run(43, 'completed', 'skipped', 'owner/repo', 'pull_request'),
                       run(44, 'in_progress', None, 'owner/repo', 'pull_request'),
                       run(45, 'completed', 'success', 'foreign/repo', 'pull_request'),
                       run(42, status, conclusion, repository, event)]
            if wrong_head:
                records[-1]['head_sha'] = 'f' * 40
            if missing_head:
                del records[-1]['head_sha']
            (root / 'runs.json').write_text(json.dumps({'workflow_runs': records[:-1]}))
            (root / 'page2.json').write_text(json.dumps({'workflow_runs': records[-1:]}))
            names = ['plan', 'honesty', 'just vv (rest)', 'atlas (join)']
            names += [f'build and non-Lean tests ({host})' for host in (
                'aarch64-unknown-linux-gnu', 'x86_64-apple-darwin',
                'aarch64-apple-darwin', 'x86_64-pc-windows-msvc')]
            jobs = [dict(name=name, conclusion='failure' if bad_job and name == 'honesty'
                         else 'success', started_at=stamp, completed_at=stamp) for name in names]
            (root / 'jobs.json').write_text(json.dumps({'jobs': jobs}))
            rows = root / 'rows.json'
            rows.write_text(json.dumps([{'part': part} for part in ('honesty', 'rest')]))
            bin_dir = root / 'bin'
            bin_dir.mkdir()
            stub = bin_dir / 'gh'
            stub.write_text('''#!/usr/bin/env bash
set -eu
if [ "$1" = run ]; then exit 1; fi
[ "$1" = api ]
case "$2" in
  */workflows/ci.yml/runs\\?*)
    if [[ "$2" == *status=success* ]]; then
      printf '%s\\n' '{"workflow_runs":[]}' | jq -r "${@: -1}"
    elif [[ "$2" == *"head_sha=$PARENT&"* ]]; then
      jq -r "${@: -1}" "$ROOT/runs.json"
      if [ "$LATER_PAGE" = 0 ] || [[ " $* " == *" --paginate "* ]]; then
        jq -r "${@: -1}" "$ROOT/page2.json"
      fi
      if [ "$PARTIAL" = 1 ]; then exit 1; fi
    else
      printf '%s\\n' '{"workflow_runs":[]}' | jq -r "${@: -1}"
    fi ;;
  */runs/*/jobs\\?*)
    printf '%s\\n' "$2" >> "$ROOT/queried"
    jq -r "${@: -1}" "$ROOT/jobs.json" ;;
  *) exit 2 ;;
esac
''')
            stub.chmod(0o755)
            sleep = bin_dir / 'sleep'
            sleep.write_text('#!/bin/sh\nexit 0\n')
            sleep.chmod(0o755)
            out = root / 'out'
            summary = root / 'summary'
            env = dict(os.environ, PATH=f'{bin_dir}{os.pathsep}{os.environ["PATH"]}', ROOT=directory,
                       PARENT=parent, PARTIAL=str(int(partial)), LATER_PAGE=str(int(later_page)),
                       EVENT='push', REPO='owner/repo',
                       SHA=head, OUT=str(out), LEXLEAN_CI_ROWS=str(rows),
                       GITHUB_OUTPUT=str(out), GITHUB_STEP_SUMMARY=str(summary))
            proc = subprocess.run(['bash', str(Path(__file__).resolve().parents[1] / script)],
                                  cwd=directory, env=env, capture_output=True, text=True, timeout=15)
            queried = (root / 'queried').read_text().splitlines() if (root / 'queried').exists() else []
            accepted = (proc.returncode == 0 and out.exists() and
                        ('full=false' in out.read_text() if script == 'ci-plan.sh'
                         else 'gate run id 42,' in out.read_text()))
            if script == 'ci-plan.sh':
                self.assertEqual(proc.returncode, 0, proc.stderr)
            elif not accepted:
                self.assertNotEqual(proc.returncode, 0, proc.stdout)
                self.assertFalse(out.exists())
            return accepted, queried

    def test_stale_success_index_recovers_both_gates(self):
        for script in ('ci-plan.sh', 'release-gate.sh'):
            with self.subTest(script=script):
                accepted, queried = self.exercise(script)
                self.assertTrue(accepted)
                self.assertEqual(len(queried), 1)
                self.assertIn('/runs/42/jobs?', queried[0])

    def test_unqualified_run_metadata_never_reaches_job_check(self):
        for script in ('ci-plan.sh', 'release-gate.sh'):
            for kwargs in ({'status': 'in_progress'}, {'conclusion': 'failure'},
                           {'conclusion': 'skipped'}, {'conclusion': None},
                           {'repository': 'foreign/repo'}, {'partial': True},
                           {'wrong_head': True}, {'missing_head': True}):
                with self.subTest(script=script, **kwargs):
                    accepted, queried = self.exercise(script, **kwargs)
                    self.assertFalse(accepted)
                    self.assertEqual(queried, [])

    def test_later_page_is_read_and_partial_page_is_not_evidence(self):
        for script in ('ci-plan.sh', 'release-gate.sh'):
            with self.subTest(script=script):
                accepted, queried = self.exercise(script, later_page=True)
                self.assertTrue(accepted)
                self.assertEqual(len(queried), 1)
                self.assertEqual(self.exercise(script, later_page=True, partial=True),
                                 (False, []))

    def test_planner_still_refuses_push_run(self):
        self.assertEqual(self.exercise('ci-plan.sh', event='push'), (False, []))

    def test_release_still_accepts_qualified_push_run(self):
        accepted, queried = self.exercise('release-gate.sh', event='push')
        self.assertTrue(accepted)
        self.assertEqual(len(queried), 1)

    def test_successful_run_header_does_not_override_failed_job(self):
        for script in ('ci-plan.sh', 'release-gate.sh'):
            with self.subTest(script=script):
                accepted, queried = self.exercise(script, bad_job=True)
                self.assertFalse(accepted)
                self.assertEqual(len(queried), 1)


if __name__ == '__main__':
    unittest.main()

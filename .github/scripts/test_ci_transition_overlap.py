import copy
import unittest
from ci_transition_overlap import compare


def snapshot(rid, spans, *, terminal=True):
    head = f'{rid:040x}'
    rows = []
    for i, (start, end) in enumerate(spans, 1):
        rows.append(dict(id=rid*100+i, name=f'job-{i}', run_id=rid, head_sha=head,
                         run_attempt=1, status='completed', conclusion='success', runner_id=i,
                         started_at=f'2026-11-24T00:00:{start:02}Z',
                         completed_at=f'2026-11-24T00:00:{end:02}Z'))
    return dict(run=dict(id=rid, head_sha=head, run_attempt=1,
                         created_at='2026-11-24T00:00:00Z',
                         status='completed' if terminal else 'in_progress'),
                observed_at='2026-11-24T00:01:00Z', total_count=len(rows), jobs=rows)


class TransitionOverlapTests(unittest.TestCase):
    def test_parallel_intervals_not_double_counted_as_wall_time(self):
        out = compare(snapshot(1, [(0, 20), (10, 30)]), snapshot(2, [(15, 25)]))
        self.assertEqual(out['combined_completed_job_seconds'], 50)
        self.assertEqual(out['concurrent_run_wall_seconds'], 10)
        self.assertEqual([r['completed_job_seconds_during_other_run'] for r in out['runs']], [15, 10])
        self.assertEqual(out['peak_observed_concurrent_jobs'], 3)
        self.assertIsNone(out['causal_contention_delay_seconds'])
        self.assertIsNone(out['billed_minutes'])

    def test_touching_intervals_not_overlap(self):
        out = compare(snapshot(1, [(0, 10)]), snapshot(2, [(10, 20)]))
        self.assertEqual(out['concurrent_run_wall_seconds'], 0)
        self.assertEqual(out['peak_observed_concurrent_jobs'], 1)

    def test_active_and_skipped_not_counted(self):
        a, b = snapshot(1, [(0, 20), (1, 30), (2, 40)], terminal=False), snapshot(2, [(15, 25)])
        a['jobs'][1].update(status='in_progress', conclusion=None, completed_at=None)
        a['jobs'][2].update(conclusion='skipped', runner_id=None, completed_at='2026-11-24T00:00:00Z')
        out = compare(a, b)
        self.assertFalse(out['terminal_both'])
        self.assertEqual(out['runs'][0]['completed_job_seconds'], 20)
        self.assertEqual(out['runs'][0]['unfinished_jobs'], 1)
        self.assertEqual(out['runs'][0]['nonexecuted_jobs'], 1)

    def test_invalid_evidence_refused(self):
        a, b = snapshot(1, [(0, 20)]), snapshot(2, [(15, 25)])
        mutations = [lambda x: x.update(total_count=2),
                     lambda x: x['run'].update(run_attempt=2),
                     lambda x: x['jobs'][0].update(head_sha='f'*40),
                     lambda x: x['jobs'][0].update(status='queued'),
                     lambda x: x['jobs'][0].update(runner_id=None),
                     lambda x: x['jobs'][0].update(completed_at='2026-11-24T00:02:00Z'),
                     lambda x: x['jobs'][0].update(started_at='2026-11-24T00:00:59Z'),
                     lambda x: x['jobs'][0].update(run_attempt=2)]
        for mutate in mutations:
            broken = copy.deepcopy(a)
            mutate(broken)
            with self.assertRaises(ValueError):
                compare(broken, b)
        with self.assertRaises(ValueError):
            compare(a, a)

    def test_zero_duration_jobs_do_not_inflate_peak(self):
        out = compare(snapshot(1, [(0, 0), (0, 10)]), snapshot(2, [(0, 10)]))
        self.assertEqual(out['peak_observed_concurrent_jobs'], 2)
        self.assertEqual(out['combined_completed_job_seconds'], 20)


if __name__ == '__main__':
    unittest.main()

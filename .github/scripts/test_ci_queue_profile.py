import copy
import unittest
from ci_queue_profile import analyze


def fixture():
    rows = []
    for i, (created, start, end) in enumerate(((1, 11, 21), (2, 22, 32)), 1):
        rows.append(dict(id=i, name=f'job-{i}', run_id=10, head_sha='a'*40, run_attempt=1,
                         status='completed', conclusion='success', runner_id=i, labels=['macos-15'],
                         created_at=f'2026-11-24T00:00:{created:02}Z', started_at=f'2026-11-24T00:00:{start:02}Z',
                         completed_at=f'2026-11-24T00:00:{end:02}Z'))
    return dict(run=dict(id=10, head_sha='a'*40, run_attempt=1, status='completed', created_at='2026-11-24T00:00:00Z'),
                observed_at='2026-11-24T00:01:00Z', total_count=2, jobs=rows)


class QueueProfileTests(unittest.TestCase):
    def test_wait_and_work_separated(self):
        result = analyze(fixture())
        row = result['runner_classes'][0]
        self.assertEqual((row['median_created_to_start_seconds'], row['max_created_to_start_seconds'], row['completed_execution_seconds']), (15, 20, 20))
        self.assertIsNone(result['causal_contention_seconds'])
        self.assertIsNone(result['critical_path_seconds'])

    def test_queued_and_skipped_metadata_ignored(self):
        data = fixture()
        data['run']['status'] = 'in_progress'
        data['jobs'][0].update(status='queued', conclusion=None, runner_id=None)
        data['jobs'][1].update(conclusion='skipped', runner_id=None)
        out = analyze(data)
        self.assertEqual(out['runner_classes'][0]['started_samples'], 0)
        self.assertEqual(out['runner_classes'][0]['completed_execution_seconds'], 0)
        self.assertEqual(out['runner_classes'][0]['unfinished_jobs'], 1)

    def test_in_progress_wait_measured_no_runtime_estimate(self):
        data = fixture()
        data['run']['status'] = 'in_progress'
        data['jobs'][0].update(status='in_progress', conclusion=None, completed_at=None)
        out = analyze(data)
        self.assertEqual(out['jobs'][0]['created_to_start_seconds'], 10)
        self.assertIsNone(out['jobs'][0]['completed_execution_seconds'])
        self.assertEqual(out['runner_classes'][0]['completed_execution_seconds'], 10)

    def test_invalid_evidence_refused(self):
        mutations = (lambda x: x.update(total_count=3), lambda x: x['run'].update(run_attempt=2),
                     lambda x: x['jobs'][0].update(head_sha='b'*40), lambda x: x['jobs'][0].update(id=2),
                     lambda x: x['jobs'][0].update(status='queued'), lambda x: x['jobs'][0].update(runner_id=None),
                     lambda x: x['jobs'][0].update(conclusion=None), lambda x: x['jobs'][0].update(runner_id=0),
                     lambda x: x['jobs'][0].update(created_at='2026-11-24T00:00:59Z'),
                     lambda x: x['jobs'][0].update(completed_at='2026-11-24T00:02:00Z'))
        for mutate in mutations:
            data = copy.deepcopy(fixture())
            mutate(data)
            with self.assertRaises(ValueError):
                analyze(data)

    def test_runner_labels_and_cancelled_execution(self):
        data = fixture()
        data['jobs'][1].update(labels=['ubuntu-24.04'], conclusion='cancelled')
        out = analyze(data)
        self.assertEqual(len(out['runner_classes']), 2)
        self.assertEqual(out['jobs'][1]['completed_execution_seconds'], 10)


if __name__ == '__main__':
    unittest.main()

import copy
import unittest
from ci_supersession_preflight import ACTIVE, candidate, classify, inspect

REPO = 'UOR-Foundation/lexlean'
R = {'id': 42, 'full_name': REPO}


def pr():
    return {'number': 620, 'state': 'open', 'draft': True,
            'base': {'repo': R, 'ref': 'release/v0.5', 'sha': 'a'*40},
            'head': {'repo': R, 'ref': 'ci/620', 'sha': 'b'*40}}


def run(rid=1, **kw):
    row = {'id': rid, 'repository': R, 'head_repository': R,
           'head_sha': 'c'*40, 'status': 'in_progress', 'event': 'pull_request',
           'head_branch': 'ci/620', 'path': '.github/workflows/ci.yml',
           'pull_requests': [pr()]}
    row.update(kw)
    return row


class Reader:
    def __init__(self, rows):
        self.rows = rows
        self.calls = 0
        self.changed = False
        self.short = False

    def get(self, endpoint, **params):
        self.calls += 1
        if endpoint.startswith('pulls/'):
            out = copy.deepcopy(pr())
            if self.changed and self.calls > 1:
                out['head']['sha'] = 'f'*40
            return out
        rows = [r for r in self.rows if r['status'] == params['status']]
        offset = (params['page']-1)*params['per_page']
        page = rows[offset:offset+params['per_page']]
        return {'total_count': len(rows), 'workflow_runs': page[:-1] if self.short and page else page}


class PreflightTests(unittest.TestCase):
    def test_same_pr_older_head_only_review(self):
        out = classify(candidate(pr(), REPO), [run()], REPO)
        self.assertEqual(out['runs'][0]['decision'], 'REVIEW_SUPERSEDED_PR_HEAD')
        self.assertFalse(out['runs'][0]['cancellation_authorized'])
        self.assertFalse(out['dispatch_authorized'])

    def test_exact_head_never_duplicate(self):
        out = classify(candidate(pr(), REPO), [run(head_sha='b'*40)], REPO)
        self.assertEqual(out['runs'][0]['decision'], 'KEEP_CURRENT_HEAD_NO_DUPLICATE_DISPATCH')

    def test_same_line_push_is_not_automatic_supersession(self):
        out = classify(candidate(pr(), REPO), [run(event='push', head_branch='release/v0.5', pull_requests=[])], REPO)
        self.assertEqual(out['runs'][0]['decision'], 'REVIEW_SAME_LINE_PUSH_NOT_PROVEN_REDUNDANT')
        self.assertFalse(out['runs'][0]['cancellation_authorized'])

    def test_other_and_multi_pr_runs_preserved(self):
        other = pr(); other['number'] = 621
        for associations in ([other], [pr(), other], []):
            with self.subTest(associations=associations):
                out = classify(candidate(pr(), REPO), [run(pull_requests=associations)], REPO)
                self.assertTrue(out['runs'][0]['decision'].startswith('KEEP_'))

    def test_repro_and_release_are_out_of_scope(self):
        out = classify(candidate(pr(), REPO), [run(1, path='.github/workflows/reproducibility.yml'),
                       run(2, path='.github/workflows/release.yml')], REPO)
        self.assertEqual(out['runs'], [])

    def test_foreign_head_and_mismatched_branch_preserved(self):
        for row in (run(head_repository={'full_name': 'other/fork'}), run(head_branch='other')):
            self.assertTrue(classify(candidate(pr(), REPO), [row], REPO)['runs'][0]['decision'].startswith('KEEP_'))

    def test_pagination_and_all_active_states(self):
        rows = [run(i+1, status=ACTIVE[i % len(ACTIVE)]) for i in range(180)]
        reader = Reader(rows)
        out = inspect(reader, REPO, 620)
        self.assertEqual(len(out['runs']), 180)
        self.assertEqual(reader.calls, 12)

    def test_changed_candidate_or_truncated_page_refused(self):
        for flag in ('changed', 'short'):
            reader = Reader([run()]); setattr(reader, flag, True)
            with self.assertRaises(ValueError):
                inspect(reader, REPO, 620)

    def test_bad_identity_refused(self):
        for rows in ([run(), run()], [run(status='completed')], [run(repository={'full_name':'other/repo'})]):
            with self.assertRaises(ValueError):
                classify(candidate(pr(), REPO), rows, REPO)


if __name__ == '__main__':
    unittest.main()

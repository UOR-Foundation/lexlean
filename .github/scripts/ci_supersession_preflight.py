#!/usr/bin/env python3
"""Read-only pre-Ready capacity review. No recommendation authorizes cancellation."""
import argparse
import json
import os
import re

ACTIVE = ('queued', 'in_progress', 'waiting', 'pending', 'requested')
PAGE = 35
MAX_RUNS = 350
SHA = re.compile(r'[0-9a-f]{40}\Z')


def require(ok, message):
    if not ok:
        raise ValueError(message)


def candidate(pr, repo):
    require(pr['state'] == 'open' and pr['base']['repo']['full_name'] == repo
            and pr['head']['repo']['full_name'] == repo,
            'candidate is not an open same-repository PR')
    require(type(pr['number']) is int and pr['number'] > 0
            and SHA.fullmatch(pr['head']['sha']) and SHA.fullmatch(pr['base']['sha']),
            'invalid candidate identity')
    line = pr['base']['ref']
    require(line == 'main' or line.startswith('release/'), 'candidate is not targeting an integration line')
    require(type(pr['draft']) is bool, 'unknown candidate readiness')
    return {'pr': pr['number'], 'head': pr['head']['sha'], 'base': pr['base']['sha'],
            'line': line, 'draft': pr['draft'], 'head_branch': pr['head']['ref']}


def classify(c, runs, repo):
    """Classify conflicts, never assert evidence coverage or cancellation authority."""
    seen, decisions = set(), []
    for run in runs:
        rid = run['id']
        require(type(rid) is int and rid > 0 and rid not in seen,
                'duplicate or invalid run identity')
        seen.add(rid)
        require(run['repository']['full_name'] == repo and run['status'] in ACTIVE
                and SHA.fullmatch(run['head_sha']), 'foreign, terminal or malformed active run')
        if run['path'] != '.github/workflows/ci.yml':
            continue  # independent reproducibility/publication are outside this control
        assoc = run.get('pull_requests')
        require(isinstance(assoc, list), 'missing PR associations')
        same_repo = (run.get('head_repository') or {}).get('full_name') == repo
        direct = (len(assoc) == 1 and assoc[0].get('number') == c['pr']
                  and assoc[0].get('base', {}).get('ref') == c['line']
                  and assoc[0].get('base', {}).get('repo', {}).get('id') == run['repository']['id']
                  and assoc[0].get('head', {}).get('repo', {}).get('id') == run['repository']['id'])
        if same_repo and run['head_sha'] == c['head']:
            decision = 'KEEP_CURRENT_HEAD_NO_DUPLICATE_DISPATCH'
        elif same_repo and run['event'] == 'pull_request' and direct and run['head_branch'] == c['head_branch']:
            decision = 'REVIEW_SUPERSEDED_PR_HEAD'
        elif same_repo and run['event'] == 'push' and run['head_branch'] == c['line']:
            decision = 'REVIEW_SAME_LINE_PUSH_NOT_PROVEN_REDUNDANT'
        elif any(p.get('base', {}).get('ref') == c['line'] for p in assoc):
            decision = 'KEEP_OTHER_PR_SHARED_LINE_CAPACITY'
        else:
            decision = 'KEEP_UNKNOWN_OR_INDEPENDENT_CI'
        decisions.append({'id': rid, 'head': run['head_sha'], 'status': run['status'],
                          'event': run['event'], 'branch': run['head_branch'],
                          'decision': decision, 'cancellation_authorized': False})
    return {'schema': 'lexlean/ci-supersession-preflight/1', 'candidate': c,
            'runs': sorted(decisions, key=lambda row: row['id']),
            'authority': 'ADVISORY_ONLY', 'dispatch_authorized': False,
            'required_review': 'Verify exact tree, obligations, independent consumers and authority before cancellation.'}


def inspect(reader, repo, pr_number):
    require(type(pr_number) is int and pr_number > 0, 'invalid PR number')
    endpoint = f'pulls/{pr_number}'
    c = candidate(reader.get(endpoint), repo)
    require(c['pr'] == pr_number, 'foreign candidate PR')
    runs = []
    # Query all active states, including queued work. A transition during these
    # non-atomic reads can create duplicates: refuse, never silently deduplicate.
    for status in ACTIVE:
        count = None
        seen = 0
        for page in range(1, MAX_RUNS // PAGE + 2):
            response = reader.get('actions/runs', status=status, per_page=PAGE, page=page)
            total, batch = response['total_count'], response['workflow_runs']
            require(type(total) is int and 0 <= total <= MAX_RUNS and isinstance(batch, list),
                    'active inventory too large or malformed')
            if count is None:
                count = total
            require(total == count and len(batch) == min(PAGE, count - seen),
                    'active roster changed or page incomplete')
            require(all(row['status'] == status for row in batch), 'run changed status during read')
            runs.extend(batch)
            seen += len(batch)
            if seen == count:
                break
        require(seen == count, 'incomplete active inventory')
    require(candidate(reader.get(endpoint), repo) == c, 'candidate changed during preflight')
    result = classify(c, runs, repo)
    result['limitations'] = 'Non-atomic provider observation; repeat immediately before authorized action. No automatic cancellation.'
    return result


def main():
    from ci_metrics_collect import Reader
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', default='UOR-Foundation/lexlean')
    parser.add_argument('--pr', type=int, required=True)
    args = parser.parse_args()
    reader = Reader(args.repo, os.environ.get('GITHUB_TOKEN', ''), 60)
    print(json.dumps(inspect(reader, args.repo, args.pr), indent=2))


if __name__ == '__main__':
    main()

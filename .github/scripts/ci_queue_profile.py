#!/usr/bin/env python3
"""Observed scheduling intervals, not a causal queue or critical-path model."""
import argparse
import json
from collections import defaultdict
from datetime import datetime
from pathlib import Path
import re
import statistics


def require(ok, reason):
    if not ok:
        raise ValueError(reason)


def stamp(value):
    require(isinstance(value, str) and re.fullmatch(r'\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z', value), 'invalid timestamp')
    return datetime.fromisoformat(value.replace('Z', '+00:00'))


def analyze(snapshot):
    run, jobs = snapshot['run'], snapshot['jobs']
    rid, head = run['id'], run['head_sha']
    require(type(rid) is int and rid > 0 and isinstance(head, str) and re.fullmatch(r'[0-9a-f]{40}', head)
            and run['run_attempt'] == 1, 'invalid or multi-attempt run identity')
    require(type(snapshot['total_count']) is int and isinstance(jobs, list)
            and 0 <= len(jobs) == snapshot['total_count'] <= 1000, 'incomplete roster')
    origin, observed = stamp(run['created_at']), stamp(snapshot['observed_at'])
    require(origin <= observed, 'invalid observation boundary')
    terminal = run['status'] == 'completed'
    groups, rows, seen = defaultdict(list), [], set()
    for job in jobs:
        jid = job['id']
        require(type(jid) is int and jid > 0 and jid not in seen and job['run_id'] == rid
                and job['head_sha'] == head and job['run_attempt'] == 1, 'duplicate or foreign job')
        seen.add(jid)
        status, conclusion = job['status'], job.get('conclusion')
        require(status in ('completed', 'queued', 'in_progress', 'pending', 'waiting', 'requested')
                and (not terminal or status == 'completed'), 'invalid job status')
        require((status == 'completed' and conclusion in ('success', 'failure', 'cancelled', 'timed_out', 'skipped', 'neutral', 'action_required'))
                or (status != 'completed' and conclusion is None), 'invalid job conclusion')
        labels = job.get('labels') or []
        require(isinstance(labels, list) and all(isinstance(x, str) and x for x in labels), 'invalid labels')
        runner_class = tuple(sorted(set(labels)))
        allocated = (type(job.get('runner_id')) is int and job['runner_id'] > 0) or bool(job.get('runner_name')) or bool(job.get('steps'))
        entry = {'id': jid, 'name': job['name'], 'runner_labels': list(runner_class), 'status': status,
                 'conclusion': conclusion, 'created_to_start_seconds': None,
                 'run_created_to_job_created_seconds': None, 'completed_execution_seconds': None}
        # GitHub can attach synthetic started_at to queued or skipped jobs.
        if status in ('completed', 'in_progress') and conclusion != 'skipped' and allocated:
            created, start = stamp(job['created_at']), stamp(job['started_at'])
            require(origin <= created <= start <= observed, 'invalid allocated scheduling interval')
            entry['created_to_start_seconds'] = int((start-created).total_seconds())
            entry['run_created_to_job_created_seconds'] = int((created-origin).total_seconds())
            if status == 'completed':
                end = stamp(job['completed_at'])
                require(start <= end <= observed, 'invalid completed execution interval')
                entry['completed_execution_seconds'] = int((end-start).total_seconds())
        elif status == 'completed':
            require(conclusion in ('skipped', 'cancelled'), 'completed job lacks allocation evidence')
        groups[runner_class].append(entry)
        rows.append(entry)
    summary = []
    for labels, entries in sorted(groups.items()):
        waits = [r['created_to_start_seconds'] for r in entries if r['created_to_start_seconds'] is not None]
        durations = [r['completed_execution_seconds'] for r in entries if r['completed_execution_seconds'] is not None]
        summary.append({'runner_labels': list(labels), 'observed_jobs': len(entries),
                        'started_samples': len(waits), 'median_created_to_start_seconds': statistics.median(waits) if waits else None,
                        'max_created_to_start_seconds': max(waits, default=None),
                        'completed_execution_seconds': sum(durations),
                        'unfinished_jobs': sum(r['status'] != 'completed' for r in entries)})
    return {'schema': 'lexlean/ci-queue-profile/1', 'run_id': rid, 'head_sha': head, 'terminal': terminal,
            'runner_classes': summary, 'jobs': rows, 'billed_minutes': None,
            'causal_contention_seconds': None, 'critical_path_seconds': None,
            'interpretation': 'Created-to-start is observed scheduling elapsed time, not isolated runner queue delay. Job creation can follow dependencies. Active/unknown work is not estimated.'}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('snapshot', type=Path)
    args = parser.parse_args()
    path = args.snapshot
    require(path.is_file() and not path.is_symlink() and path.stat().st_size <= 8*1024*1024, 'missing, linked or oversized snapshot')
    print(json.dumps(analyze(json.loads(path.read_text(encoding='utf-8'))), indent=2))


if __name__ == '__main__':
    main()

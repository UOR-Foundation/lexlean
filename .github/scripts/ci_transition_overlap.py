#!/usr/bin/env python3
"""Read-only measurement of two explicit, first-attempt Actions runs.

Supplemental to the final-HEAD cohort report, not a qualification verdict or
billing report. Active snapshots are lower bounds; never estimate active cost.
"""
import argparse
import json
import re
from datetime import datetime
from pathlib import Path


def require(ok, reason):
    if not ok:
        raise ValueError(reason)


def utc(value):
    require(isinstance(value, str) and re.fullmatch(
        r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z", value), "invalid UTC timestamp")
    return datetime.fromisoformat(value.replace("Z", "+00:00"))


def intervals(snapshot):
    run, jobs = snapshot["run"], snapshot["jobs"]
    rid, head = run["id"], run["head_sha"]
    require(type(rid) is int and rid > 0 and isinstance(head, str)
            and re.fullmatch(r"[0-9a-f]{40}", head), "invalid run identity")
    require(run["run_attempt"] == 1, "multi-attempt runs require lineage accounting")
    require(type(snapshot["total_count"]) is int and isinstance(jobs, list)
            and len(jobs) == snapshot["total_count"] <= 1000, "incomplete job snapshot")
    terminal = run["status"] == "completed"
    observed = utc(snapshot["observed_at"])
    origin = utc(run["created_at"])
    require(origin <= observed, "observation precedes run")
    seen_ids, seen_names, windows = set(), set(), []
    nonexecuted = active = 0
    for j in jobs:
        jid, name = j["id"], j["name"]
        require(type(jid) is int and jid > 0 and jid not in seen_ids
                and isinstance(name, str) and name and name not in seen_names,
                "duplicate or invalid job identity")
        seen_ids.add(jid)
        seen_names.add(name)
        require(j["run_id"] == rid and j["head_sha"] == head and j["run_attempt"] == 1,
                "foreign job or attempt")
        if j["status"] != "completed":
            require(not terminal and j["status"] in ("queued", "in_progress", "waiting", "pending", "requested"),
                    "unfinished job in terminal run or invalid status")
            active += 1
            continue
        conclusion = j["conclusion"]
        require(conclusion in ("success", "failure", "cancelled", "timed_out", "skipped", "neutral", "action_required"),
                "invalid completed conclusion")
        allocated = j.get("runner_id") is not None or bool(j.get("runner_name")) or bool(j.get("steps"))
        if conclusion == "skipped" or (conclusion == "cancelled" and not allocated):
            nonexecuted += 1
            continue
        require(allocated, "completed job lacks runner allocation evidence")
        start, end = utc(j["started_at"]), utc(j["completed_at"])
        require(origin <= start <= end <= observed, "invalid execution window")
        windows.append((start, end))
    return rid, head, terminal, active, nonexecuted, windows


def compare(left, right):
    a, b = intervals(left), intervals(right)
    require(a[0] != b[0], "same run cannot be compared with itself")
    changes = {}
    totals = []
    for index, run in enumerate((a, b)):
        totals.append(sum(int((end-start).total_seconds()) for start, end in run[5]))
        for start, end in run[5]:
            if start == end:
                continue
            changes.setdefault(start, [0, 0])[index] += 1
            changes.setdefault(end, [0, 0])[index] -= 1
    active = [0, 0]
    overlap_seconds = 0
    overlap_job_seconds = [0, 0]
    peak = 0
    previous = None
    for time, change in sorted(changes.items()):
        if previous is not None and all(active):
            seconds = int((time-previous).total_seconds())
            overlap_seconds += seconds
            for i in range(2):
                overlap_job_seconds[i] += seconds * active[i]
        active = [active[i]+change[i] for i in range(2)]
        peak = max(peak, sum(active))
        previous = time
    require(active == [0, 0], "unbalanced execution windows")
    return {
        "schema": "lexlean/ci-transition-overlap/1",
        "scope": "observed completed first-attempt job intervals only",
        "terminal_both": a[2] and b[2],
        "runs": [{"id": r[0], "head_sha": r[1], "terminal": r[2],
                  "unfinished_jobs": r[3], "nonexecuted_jobs": r[4],
                  "completed_job_seconds": totals[i],
                  "completed_job_seconds_during_other_run": overlap_job_seconds[i]}
                 for i, r in enumerate((a, b))],
        "combined_completed_job_seconds": sum(totals),
        "concurrent_run_wall_seconds": overlap_seconds,
        "peak_observed_concurrent_jobs": peak,
        "billed_minutes": None,
        "causal_contention_delay_seconds": None,
        "warning": "Incomplete snapshots are lower bounds. Overlap is not proof of queue contention or delay."
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("left", type=Path)
    parser.add_argument("right", type=Path)
    args = parser.parse_args()
    def load(path):
        require(path.is_file() and not path.is_symlink() and path.stat().st_size <= 8*1024*1024,
                "missing, linked or oversized snapshot")
        return json.loads(path.read_text(encoding="utf-8"))
    print(json.dumps(compare(load(args.left), load(args.right)), indent=2))


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""Count real GitHub Actions executions, not reissued retry metadata records."""
from datetime import datetime


def require(ok, message):
    if not ok:
        raise ValueError(message)


def utc(value):
    require(isinstance(value, str) and value.endswith("Z"), "missing UTC timestamp")
    try:
        return datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError as exc:
        raise ValueError("invalid UTC timestamp") from exc


def runtime_window(row):
    """Return a real runner execution interval, not provider scheduling metadata."""
    conclusion = row.get("conclusion")
    start, end = row.get("started_at"), row.get("completed_at")
    # Skipped jobs never received an execution opportunity. GitHub may
    # synthesize contradictory start/end timestamps for them.
    allocated = (row.get("runner_id") is not None or
                 bool(row.get("runner_name")) or bool(row.get("steps")))
    if conclusion == "skipped" or (conclusion == "cancelled" and not allocated):
        for value in (start, end):
            if value is not None:
                utc(value)  # metadata validity, not runtime arithmetic
        return None
    require(bool(start) == bool(end), "partial job timestamps (partial execution)")
    if start is None:
        require(conclusion == "cancelled", "executed job without timing")
        # Allocated cancelled job with no timing is unknown, not free.
        raise ValueError("assigned cancelled job lacks execution timestamps")
    require((utc(end) - utc(start)).total_seconds() >= 0,
            "negative execution duration")
    return start, end


def canonical_executions(records, latest_attempt, proven_empty_attempts=()):
    """Return unique executed job records and explain all duplicated provider rows.

    GitHub's filter=all can return reissued job IDs on each rerun for jobs whose
    original start/end times were unchanged. Those are NOT new runner work.
    Rerun work with a genuinely new execution interval is counted separately.
    Missing attempt numbers require separate provider proof of zero jobs.
    """
    require(type(latest_attempt) is int and 1 <= latest_attempt <= 100,
            "invalid current run attempt")
    require(isinstance(records, list) and len(records) <= 10000,
            "oversized or malformed provider job set")
    empty = set(proven_empty_attempts)
    require(all(type(i) is int and 1 <= i <= latest_attempt for i in empty),
            "untrusted empty-attempt claim")
    ids, indexed = set(), {}
    for row in records:
        require(isinstance(row, dict), "invalid provider job record")
        jid, name = row.get("id"), row.get("name")
        require(type(jid) is int and jid > 0 and jid not in ids,
                "duplicate/invalid provider job id")
        require(isinstance(name, str) and 0 < len(name) <= 300,
                "missing provider job name")
        ids.add(jid)
        attempt = row.get("run_attempt", 1 if latest_attempt == 1 else None)
        require(type(attempt) is int and 1 <= attempt <= latest_attempt,
                "foreign or missing job attempt")
        by_name = indexed.setdefault(attempt, {})
        require(name not in by_name, "duplicate job name within one attempt")
        by_name[name] = row
        require(row.get("status", "completed") == "completed",
                "completed run has unfinished job")
        conclusion = row.get("conclusion")
        require(conclusion in ("success", "failure", "cancelled", "timed_out",
                               "skipped", "neutral", "action_required"),
                "invalid completed job conclusion")
        runtime_window(row)
        labels = row.get("labels") or []
        require(isinstance(labels, list) and
                all(isinstance(v, str) and len(v) <= 128 for v in labels),
                "untrusted runner labels")
    require(not (set(indexed) & empty), "attempt cannot be both empty and populated")
    require(set(indexed) | empty == set(range(1, latest_attempt + 1)),
            "unverified missing attempt in provider receipt")
    executions, by_name, carried = [], {}, 0
    for attempt in sorted(indexed):
        for name, row in sorted(indexed[attempt].items()):
            prior = by_name.setdefault(name, [])
            # No-run skipped/cancelled provider rows can have different bogus
            # metadata timestamps on every attempt without any runner work.
            window = runtime_window(row)
            identity = ("NON_EXECUTED", row["conclusion"]) if window is None else window
            old = next((
                other for other in prior
                if ((("NON_EXECUTED", other["conclusion"])
                     if runtime_window(other) is None else runtime_window(other)) == identity)
            ), None)
            if old is not None:
                require(old.get("conclusion") == row.get("conclusion") and
                        (old.get("labels") or []) == (row.get("labels") or []),
                        "same execution has conflicting result or host")
                created = row.get("created_at")
                end = old.get("completed_at")
                if created is not None and end is not None and window is not None:
                    require(utc(created) >= utc(end),
                            "reissued job predates original execution")
                carried += 1
                continue
            if window is not None:
                for other in prior:
                    previous = runtime_window(other)
                    if previous is not None:
                        require(utc(window[0]) > utc(previous[1]),
                                "overlapping execution across attempts")
            prior.append(row)
            executions.append(row)
    return executions, {
        "provider_job_records": len(records),
        "unique_execution_records": len(executions),
        "reissued_carried_records": carried,
        "verified_empty_attempts": sorted(empty),
    }

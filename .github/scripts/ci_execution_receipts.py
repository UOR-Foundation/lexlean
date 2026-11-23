#!/usr/bin/env python3
"""Count real GitHub Actions executions, not reissued retry metadata records."""
from datetime import datetime
import math


def require(ok, message):
    if not ok:
        raise ValueError(message)


def utc(value):
    require(isinstance(value, str) and value.endswith("Z"), "missing UTC timestamp")
    try:
        return datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError as exc:
        raise ValueError("invalid UTC timestamp") from exc


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
        start, finish = row.get("started_at"), row.get("completed_at")
        require(bool(start) == bool(finish), "partial job timestamps (partial execution)")
        if start:
            require((utc(finish) - utc(start)).total_seconds() >= 0,
                    "negative execution duration")
        else:
            require(conclusion in ("skipped", "cancelled"),
                    "executed job without timing")
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
            old = next((other for other in prior
                        if (other.get("started_at"), other.get("completed_at")) ==
                           (row.get("started_at"), row.get("completed_at"))), None)
            if old is not None:
                require(old.get("conclusion") == row.get("conclusion") and
                        (old.get("labels") or []) == (row.get("labels") or []),
                        "same execution has conflicting result or host")
                created = row.get("created_at")
                end = old.get("completed_at")
                if created is not None and end is not None:
                    require(utc(created) >= utc(end),
                            "reissued job predates original execution")
                carried += 1
                continue
            if row.get("started_at"):
                for other in prior:
                    if other.get("completed_at"):
                        require(utc(row["started_at"]) > utc(other["completed_at"]),
                                "overlapping execution across attempts")
            prior.append(row)
            executions.append(row)
    return executions, {
        "provider_job_records": len(records),
        "unique_execution_records": len(executions),
        "reissued_carried_records": carried,
        "verified_empty_attempts": sorted(empty),
    }

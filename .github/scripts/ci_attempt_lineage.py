"""Read-only same-run execution lineage; never CI or release authority."""
from datetime import datetime
import re


class Hold(ValueError):
    """Incomplete or inconsistent attempt evidence."""


def require(ok, message):
    if not ok:
        raise Hold(message)


def utc(value):
    require(isinstance(value, str) and re.fullmatch(
        r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z", value), "invalid UTC time")
    return datetime.fromisoformat(value.replace("Z", "+00:00"))


def reconcile(run_id, head_sha, expected_hosts, attempts):
    """Map latest records to original executions, using complete attempt snapshots.

    Expected hosts must be obtained independently from the governed workflow.
    This function validates snapshot consistency, not provider authenticity.
    It never creates a status, imports evidence, or authorizes test reuse.
    """
    require(type(run_id) is int and run_id > 0 and isinstance(head_sha, str)
            and re.fullmatch(r"[0-9a-f]{40}", head_sha), "invalid run identity")
    require(isinstance(expected_hosts, dict) and 0 < len(expected_hosts) <= 1000
            and all(isinstance(n, str) and n and isinstance(h, str) and h
                    for n, h in expected_hosts.items()), "invalid expected roster")
    require(isinstance(attempts, list) and 0 < len(attempts) <= 100,
            "incomplete attempt history")
    latest, origin, seen_ids, execution_seconds = {}, {}, set(), 0
    for number, snapshot in enumerate(attempts, 1):
        require(snapshot["run_id"] == run_id and snapshot["head_sha"] == head_sha
                and snapshot["run_attempt"] == number, "foreign/missing attempt")
        rows = snapshot["jobs"]
        require(isinstance(rows, list) and len(rows) == snapshot["total_count"]
                == len(expected_hosts), "incomplete roster")
        current = {}
        for row in rows:
            name, jid = row["name"], row["id"]
            require(name in expected_hosts and name not in current and type(jid) is int
                    and jid > 0 and jid not in seen_ids and row["run_id"] == run_id
                    and row["head_sha"] == head_sha and row["run_attempt"] == number,
                    "duplicate or foreign job")
            seen_ids.add(jid)
            current[name] = row
            labels = row["labels"]
            require(isinstance(labels, list) and all(isinstance(x, str) for x in labels)
                    and len(set(labels)) == len(labels), "invalid labels")
            state, result = row["status"], row["conclusion"]
            start, end = row["started_at"], row["completed_at"]
            created = utc(row["created_at"])
            require(state in ("completed", "in_progress", "queued", "waiting"),
                    "unknown job state")
            if state == "completed":
                require(result in ("success", "failure", "cancelled", "timed_out"),
                        "nonqualifying or unknown completed status")
                require(utc(end) >= utc(start), "invalid execution interval")
            else:
                require(number == len(attempts) and result is None and end is None,
                        "nonterminal historical job or premature verdict")
                require((state == "in_progress") == (start is not None), "invalid start")
            if result == "success" or state == "in_progress":
                require(expected_hosts[name] in labels, "wrong host label")
            previous = latest.get(name)
            carried = (previous is not None and previous["status"] == state == "completed"
                       and previous["conclusion"] == result == "success"
                       and previous["started_at"] == start and previous["completed_at"] == end
                       and previous["labels"] == labels)
            if not carried:
                if previous is not None:
                    require(created > utc(previous["completed_at"]), "overlapping or stale execution")
                if start is not None:
                    require(utc(start) >= created, "execution predates job creation")
                origin[name] = {"attempt": number, "job_id": jid}
                if state == "completed":
                    execution_seconds += (utc(end) - utc(start)).total_seconds()
        require(set(current) == set(expected_hosts), "missing expected job")
        if number < len(attempts):
            require(all(r["status"] == "completed" for r in rows), "historical attempt active")
        latest = current
    jobs = {name: {"origin_attempt": origin[name]["attempt"],
                   "origin_job_id": origin[name]["job_id"], "latest_job_id": row["id"],
                   "conclusion": row["conclusion"], "status": row["status"]}
            for name, row in sorted(latest.items())}
    return {"run_id": run_id, "head_sha": head_sha, "jobs": jobs,
            "retained_successes": sum(r["conclusion"] == "success" and
                                      origin[n]["attempt"] < len(attempts)
                                      for n, r in latest.items()),
            "completed_execution_minutes": round(execution_seconds / 60, 3),
            "all_latest_jobs_success": all(r["conclusion"] == "success" for r in latest.values()),
            "qualification": "NONQUALIFYING_SNAPSHOT_LINEAGE",
            "provider_authenticity_verified": False, "release_gate_authorized": False}

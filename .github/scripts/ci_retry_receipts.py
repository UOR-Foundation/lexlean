#!/usr/bin/env python3
"""Reconcile frozen GitHub partial-rerun receipts; no CI/release authority."""
import argparse
from datetime import datetime
import json
from pathlib import Path
import re
import sys

SCHEMA = "lexlean/targeted-host-recovery/1"
ARM = "build and non-Lean tests (aarch64-unknown-linux-gnu)"


def check(ok, message):
    if not ok:
        raise ValueError(message)


def timestamp(raw):
    check(isinstance(raw, str) and raw.endswith("Z"), "timestamp must be UTC")
    try:
        return datetime.fromisoformat(raw.replace("Z", "+00:00"))
    except ValueError as err:
        raise ValueError("malformed UTC timestamp") from err


def index(attempt, number):
    check(isinstance(attempt, dict), "attempt object missing")
    entries = attempt.get("jobs")
    check(isinstance(entries, list) and 1 <= len(entries) <= 256 and
          len(entries) == attempt.get("job_count"), "incomplete job inventory")
    by_name, ids = {}, set()
    for row in entries:
        check(isinstance(row, dict), "invalid job record")
        name, jid = row.get("name"), row.get("id")
        check(isinstance(name, str) and name and len(name) < 300 and
              name not in by_name, "duplicate or missing job name")
        check(type(jid) is int and jid > 0 and jid not in ids and
              row.get("run_attempt") == number, "foreign or duplicate job ID")
        ids.add(jid)
        state, outcome = row.get("status"), row.get("conclusion")
        check(state in ("completed", "in_progress", "queued", "waiting"),
              "invalid job status")
        created = timestamp(row.get("created_at"))
        start = timestamp(row["started_at"]) if row.get("started_at") else None
        if start is None:
            check(state in ("queued", "waiting"), "missing execution start")
        else:
            # A reissued metadata record can postdate a carried successful
            # execution. Exact old timestamps must still match in reconcile().
            check(start >= created or
                  (number == 2 and state == "completed" and outcome == "success"),
                  "execution predates creation without carried-success proof")
        if state == "completed":
            check(outcome in ("success", "failure", "cancelled", "timed_out"),
                  "missing job conclusion")
            end = timestamp(row.get("completed_at"))
            check(start is not None and end >= start, "invalid execution interval")
        else:
            check(outcome is None and row.get("completed_at") is None,
                  "premature job verdict")
        by_name[name] = row
    return by_name


def reconcile(data):
    check(isinstance(data, dict) and data.get("schema") == SCHEMA, "wrong schema")
    check(type(data.get("run_id")) is int and data["run_id"] > 0 and
          isinstance(data.get("head_sha"), str) and
          re.fullmatch("[0-9a-f]{40}", data["head_sha"]), "invalid exact run/SHA")
    check(data.get("source_ref") == "refs/heads/release/v0.4", "foreign source ref")
    a, b = data.get("attempt_1"), data.get("attempt_2")
    before, after = index(a, 1), index(b, 2)
    check(set(before) == set(after), "required job set changed")
    success = {name: j for name, j in before.items() if j["conclusion"] == "success"}
    failed = {name: j for name, j in before.items() if j["conclusion"] == "failure"}
    check(set(failed) == {ARM} and len(success) == len(before) - 1 and
          a.get("conclusion") == "failure" and
          a.get("success_count") == len(success) and a.get("failure_count") == 1,
          "original single-host failure not proven")
    preserved_seconds = 0
    for name, earlier in success.items():
        later = after[name]
        check(later["status"] == "completed" and later["conclusion"] == "success" and
              earlier["started_at"] == later["started_at"] and
              earlier["completed_at"] == later["completed_at"] and
              earlier["id"] != later["id"], "prior successful execution changed: " + name)
        preserved_seconds += (timestamp(earlier["completed_at"]) -
                              timestamp(earlier["started_at"])).total_seconds()
    old, new = failed[ARM], after[ARM]
    check(old["id"] != new["id"] and
          timestamp(new["created_at"]) > timestamp(old["completed_at"]),
          "missing later retry execution")
    state, conclusion = b.get("status"), b.get("conclusion")
    check(state in ("in_progress", "queued", "completed"), "invalid overall state")
    if state == "completed":
        check(conclusion in ("success", "failure", "cancelled", "timed_out") and
              new["status"] == "completed", "premature whole-run conclusion")
        check((conclusion == "success") == (new["conclusion"] == "success"),
              "false-green run/host mismatch")
    else:
        check(conclusion is None and new["status"] != "completed",
              "premature whole-run verdict")
    return {
        "schema": "lexlean/ci-retry-verification/1",
        "classification": "READ_ONLY_NOT_RELEASE_AUTHORITY",
        "run_id": data["run_id"], "head_sha": data["head_sha"],
        "required_jobs": len(before), "retained_successes": len(success),
        "new_execution_windows": 1,
        "retained_unweighted_minutes": round(preserved_seconds / 60, 3),
        "host_recovery": ("CONFIRMED" if conclusion == "success"
                          else "FAILED" if state == "completed" else "PENDING"),
        "release_gate_authorized": False, "billed_savings_verified": False,
    }


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--ledger", type=Path, required=True)
    args = ap.parse_args(argv)
    try:
        path = args.ledger
        check(path.is_file() and not path.is_symlink() and
              path.stat().st_size <= 1024 * 1024, "missing, linked or oversized ledger")
        print(json.dumps(reconcile(json.loads(path.read_text())), sort_keys=True, indent=2))
    except (OSError, ValueError, TypeError, KeyError) as err:
        print("HOLD: " + str(err), file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

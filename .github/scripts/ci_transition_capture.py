#!/usr/bin/env python3
"""Capture immutable first-attempt transition evidence, not acceptance authority."""
import argparse
import json
import os
from pathlib import Path
import tempfile
from datetime import datetime, timezone


PAGE_SIZE = 35
MAX_JOBS = 1000


def require(ok, reason):
    if not ok:
        raise ValueError(reason)


def capture(reader, run_id, *, allow_active=False):
    """Reject changing run headers/rosters; active captures are only lower bounds."""
    require(type(run_id) is int and run_id > 0, "invalid run id")
    endpoint = f"actions/runs/{run_id}"
    header = reader.get(endpoint)
    fields = ("id", "head_sha", "run_attempt", "status", "conclusion", "created_at",
              "updated_at", "run_started_at", "path", "event", "html_url")
    def identity(value):
        return tuple(value[k] for k in fields[:7])
    require(header["id"] == run_id and header["run_attempt"] == 1,
            "foreign run or multi-attempt evidence; use attempt lineage")
    require(allow_active or header["status"] == "completed", "run is not terminal")
    rows, count, seen = [], None, set()
    for page in range(1, MAX_JOBS // PAGE_SIZE + 2):
        response = reader.get(endpoint + "/attempts/1/jobs", per_page=PAGE_SIZE, page=page)
        total, batch = response["total_count"], response["jobs"]
        require(type(total) is int and 0 <= total <= MAX_JOBS and isinstance(batch, list),
                "invalid provider inventory")
        if count is None:
            count = total
        require(total == count and len(batch) == min(PAGE_SIZE, count-len(rows)),
                "provider roster changed or page incomplete")
        for row in batch:
            require(row["run_id"] == run_id and row["head_sha"] == header["head_sha"]
                    and row["run_attempt"] == 1 and row["id"] not in seen,
                    "foreign or repeated job record")
            seen.add(row["id"])
            rows.append(row)
        if len(rows) == count:
            break
    require(len(rows) == count, "provider roster incomplete")
    final = reader.get(endpoint)
    require(identity(header) == identity(final), "run changed during capture; retry read")
    if header["status"] == "completed":
        require(all(row["status"] == "completed" for row in rows),
                "terminal run has unfinished jobs")
    observed = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    return {"schema": "lexlean/ci-transition-run-snapshot/1",
            "source": "GitHub REST explicit attempt 1; stable run header and page count, not an atomic provider snapshot",
            "run": {k: header.get(k) for k in fields}, "observed_at": observed,
            "total_count": count, "jobs": rows}


def write_new(path, value):
    """Publish complete data atomically without replacing prior observations."""
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(mode="w", encoding="utf-8", dir=path.parent,
                                     delete=False) as temporary:
        temp = Path(temporary.name)
        try:
            json.dump(value, temporary, indent=2)
            temporary.write("\n")
            temporary.flush()
            os.fsync(temporary.fileno())
            os.link(temp, path)
        finally:
            temp.unlink(missing_ok=True)


def main():
    from ci_metrics_collect import Reader
    from ci_transition_overlap import compare
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("left", type=int)
    parser.add_argument("right", type=int)
    parser.add_argument("--repo", default="UOR-Foundation/lexlean")
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--allow-active", action="store_true")
    args = parser.parse_args()
    require(args.left != args.right, "cannot compare the same run")
    reader = Reader(args.repo, os.environ.get("GITHUB_TOKEN", ""), 70)
    left = capture(reader, args.left, allow_active=args.allow_active)
    right = capture(reader, args.right, allow_active=args.allow_active)
    result = compare(left, right)
    write_new(args.output, {"schema": "lexlean/ci-transition-capture/1",
                            "left": left, "right": right, "report": result})
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""Offline CI job hotspot evidence. No networking, mutation, or savings claims.

The historical v1 cohort only stored per-run totals. It MUST NOT be used to
invent individual job timings or critical paths. Feed this report explicit
--include-job-breakdown output from the existing bounded GET-only collector.
"""
import argparse
import csv
import json
import math
import statistics
from collections import defaultdict
from datetime import datetime
from pathlib import Path

from ci_metrics_collect import JOB_SCHEMA
from ci_metrics_report import COST_SCHEMA, pct

OS = ("linux", "windows", "macos", "unknown")


def require(ok, reason):
    if not ok:
        raise ValueError(reason)


def read_snapshot(path):
    file = Path(path)
    require(file.is_file() and not file.is_symlink() and
            file.stat().st_size <= 12 * 1024 * 1024, "missing, linked or oversized evidence")
    return json.loads(file.read_text(encoding="utf-8"))


def timestamp(s):
    require(isinstance(s, str) and s.endswith("Z"), "job timestamp missing UTC")
    try:
        return datetime.fromisoformat(s.replace("Z", "+00:00"))
    except ValueError as exc:
        raise ValueError("job timestamp malformed") from exc


def analyze(details, costs):
    require(isinstance(details, dict) and details.get("schema") == JOB_SCHEMA,
            "real job-level snapshot required, aggregate-only historical file insufficient")
    require(isinstance(costs, dict) and costs.get("schema") == COST_SCHEMA,
            "run cost snapshot schema missing")
    jobs = details.get("jobs")
    source_runs = costs.get("run_costs")
    require(isinstance(jobs, list) and jobs and
            details.get("expected_jobs") == len(jobs), "missing/truncated per-job inventory")
    require(isinstance(source_runs, list) and source_runs,
            "missing run totals")
    by_run = {}
    for run in source_runs:
        run_id = run.get("id")
        require(type(run_id) is int and run_id > 0 and run_id not in by_run,
                "duplicate/invalid source run id")
        require(run.get("complete") is True and run.get("returned") == run.get("job_count"),
                "incomplete source run jobs")
        by_run[run_id] = run
    buckets = defaultdict(list)
    within = defaultdict(list)
    seen_ids = set()
    for job in jobs:
        require(isinstance(job, dict), "malformed per-job row")
        jid, rid = job.get("job_id"), job.get("run_id")
        require(type(jid) is int and jid > 0 and jid not in seen_ids,
                "duplicate/invalid job ID")
        seen_ids.add(jid)
        require(type(rid) is int and rid in by_run, "unmatched run for job")
        run = by_run[rid]
        require(job.get("pr") == run.get("pr"), "job attributed to wrong PR")
        name, workflow, platform = job.get("job_name"), job.get("workflow"), job.get("platform")
        require(isinstance(name, str) and 0 < len(name) <= 300 and
                isinstance(workflow, str) and workflow and platform in OS,
                "missing job identity, workflow or platform")
        start, end, minutes = job.get("started_at"), job.get("completed_at"), job.get("elapsed_minutes")
        if start is None or end is None:
            require(start is None and end is None and minutes is None and
                    job.get("conclusion") in ("skipped", "cancelled", None),
                    "unaccounted job with missing timing")
        else:
            begin, finish = timestamp(start), timestamp(end)
            elapsed = (finish - begin).total_seconds() / 60
            require(elapsed >= 0 and math.isfinite(elapsed) and
                    type(minutes) in (int, float) and math.isfinite(minutes) and
                    minutes >= 0 and abs(minutes - elapsed) <= 0.0011,
                    "job duration does not match provider timestamps")
        buckets[(workflow, name, platform)].append(job)
        within[rid].append(job)
    require(set(within) == {rid for rid, row in by_run.items() if row["job_count"] > 0},
            "unmatched executed and zero-job source runs")
    per_run = []
    for rid, run in by_run.items():
        rows = within.get(rid, [])
        require(len(rows) == run["job_count"], "job detail count does not match source")
        observed = sum(j["elapsed_minutes"] or 0 for j in rows)
        require(abs(observed - run["job_minutes"]) <= 0.001 * (len(rows) + 1),
                "job minutes do not reconcile with observed run total")
        for platform in OS:
            duration = sum(j["elapsed_minutes"] or 0 for j in rows if j["platform"] == platform)
            reported = run["by_platform_minutes"].get(platform, 0)
            require(abs(duration - reported) <= 0.001 * (len(rows) + 1),
                    "per-platform job minutes do not reconcile")
        executed = [j for j in rows if j["started_at"] is not None]
        starts = [timestamp(j["started_at"]) for j in executed]
        ends = [timestamp(j["completed_at"]) for j in executed]
        envelope = (max(ends) - min(starts)).total_seconds()/60 if executed else None
        require(envelope is None or envelope >= 0, "invalid job envelope")
        per_run.append({
            "run_id": rid, "pr": run["pr"], "executed_jobs": len(executed),
            "all_jobs": len(rows), "job_minutes": round(observed, 3),
            "executed_job_envelope_minutes": round(envelope, 3) if envelope is not None else None,
            "overlap_ratio": round(observed/envelope, 3) if envelope and envelope > 0 else None,
        })
    hottest = []
    for (workflow, name, platform), rows in buckets.items():
        values = [j["elapsed_minutes"] for j in rows if j["elapsed_minutes"] is not None]
        hottest.append({
            "workflow": workflow, "job": name, "platform": platform,
            "observations": len(rows), "executed": len(values),
            "skipped": sum(j["conclusion"] == "skipped" for j in rows),
            "failed": sum(j["conclusion"] in ("failure", "timed_out") for j in rows),
            "observed_job_minutes": round(sum(values), 3),
            "median_job_minutes": round(statistics.median(values), 3) if values else None,
            "p90_job_minutes": pct(values, .9),
        })
    hottest.sort(key=lambda x: (-x["observed_job_minutes"],
                                x["workflow"], x["job"], x["platform"]))
    return {
        "schema": "lexlean/ci-job-hotspots/1",
        "classification": "OBSERVED_JOB_ELAPSED_ONLY_NOT_BILLING_OR_CUTOVER_EVIDENCE",
        "pr_sample": len({j["pr"] for j in jobs}),
        "run_count": len(by_run),
        "job_rows": len(jobs),
        "observed_job_minutes": round(sum(r["job_minutes"] for r in per_run), 3),
        "job_groups": hottest,
        "run_envelopes": sorted(per_run, key=lambda x: (x["pr"], x["run_id"])),
        "unavailable_from_this_snapshot": [
            "provider billed minutes", "entire PR revision lifetime",
            "queue time or protected-branch merge gate latency",
            "causal savings from staging, caching or skipped detectors",
        ],
    }


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--jobs", type=Path, required=True)
    ap.add_argument("--costs", type=Path, required=True)
    ap.add_argument("--output-dir", type=Path, required=True)
    args = ap.parse_args(argv)
    result = analyze(read_snapshot(args.jobs), read_snapshot(args.costs))
    args.output_dir.mkdir(parents=True, exist_ok=True)
    (args.output_dir / "job-hotspots.json").write_text(
        json.dumps(result, indent=2, sort_keys=True)+"\n", encoding="utf-8")
    with (args.output_dir / "job-hotspots.csv").open("w", newline="", encoding="utf-8") as fh:
        fields = ("workflow", "job", "platform", "observations", "executed",
                  "skipped", "failed", "observed_job_minutes",
                  "median_job_minutes", "p90_job_minutes")
        writer = csv.DictWriter(fh, fieldnames=fields)
        writer.writeheader()
        writer.writerows(result["job_groups"])
    print(json.dumps({
        "job_rows": result["job_rows"], "run_count": result["run_count"],
        "observed_job_minutes": result["observed_job_minutes"],
        "status": result["classification"]}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

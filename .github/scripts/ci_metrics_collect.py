#!/usr/bin/env python3
"""On-demand READ-ONLY GitHub Actions evidence collector for CI #603.

Captures 50 merged PR final-HEAD workflow runs and all attempted job timings.
Never creates workflows, PRs, schedules or checks, and never infers billing.
Requires explicit read-only GITHUB_TOKEN. Output is inert source snapshot JSON.
"""
import argparse
import json
import math
import os
import re
import sys
import urllib.error
import urllib.parse
import urllib.request
from datetime import datetime, timezone
from pathlib import Path

from ci_execution_receipts import canonical_executions, runtime_window

REPO_PATTERN = re.compile(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+\Z")
JOB_SCHEMA = "lexlean/ci-job-detail/1"


def require(ok, message):
    if not ok:
        raise ValueError(message)


def clock(s):
    return datetime.fromisoformat(s.replace("Z", "+00:00")).astimezone(timezone.utc)


class Reader:
    def __init__(self, repo, token, limit):
        require(REPO_PATTERN.fullmatch(repo), "invalid repo identity")
        require(bool(token), "GITHUB_TOKEN with read-only Actions/Pull requests access required")
        require(type(limit) is int and 1 <= limit <= 1200, "API request limit out of bounds")
        self.repo = repo
        self.token = token
        self.limit = limit
        self.requests = 0
        # Populated only on explicit request, using already-fetched job data.
        self.job_details = None

    def get(self, endpoint, **params):
        require(self.requests < self.limit, "API call budget exceeded, fail closed")
        path = urllib.parse.quote(endpoint, safe="/")
        url = "https://api.github.com/repos/" + self.repo + "/" + path
        if params:
            url += "?" + urllib.parse.urlencode(params)
        req = urllib.request.Request(url, headers={
            "Accept": "application/vnd.github+json",
            "Authorization": "Bearer " + self.token,
            "X-GitHub-Api-Version": "2022-11-28",
            "User-Agent": "lexlean-ci-metrics-read-only",
        }, method="GET")
        self.requests += 1
        try:
            with urllib.request.urlopen(req, timeout=45) as response:
                require(response.status == 200, "unexpected GitHub status")
                raw = response.read(8 * 1024 * 1024 + 1)
        except urllib.error.HTTPError as exc:
            raise ValueError("GitHub API read failed with HTTP " + str(exc.code)) from None
        except (urllib.error.URLError, TimeoutError) as exc:
            raise ValueError("GitHub API read unavailable") from exc
        require(len(raw) <= 8 * 1024 * 1024, "oversized GitHub response")
        return json.loads(raw)

    def paged(self, endpoint, array_key, **filters):
        entries = []
        page = 1
        total = None
        while True:
            response = self.get(endpoint, per_page=50, page=page, **filters)
            if array_key is None:
                require(isinstance(response, list), "expected list from GitHub")
                values = response
            else:
                require(isinstance(response, dict), "expected GitHub collection")
                values = response.get(array_key)
                require(isinstance(values, list), "missing GitHub collection")
                if total is None:
                    total = response.get("total_count")
                    require(type(total) is int, "missing declared collection count")
            entries.extend(values)
            require(len(entries) <= 10000, "too many paginated records")
            if len(values) < 50:
                break
            page += 1
        if total is not None:
            require(len(entries) == total, "GitHub collection count mismatch")
        return entries

    def pr_candidates(self, mode, boundary, limit):
        """Find true oldest-first postcutover, latest-first precutover merged PRs.

        GitHub's PR listing sorts by updated_at, not merged_at. Exhaust pages
        until their updated_at lower bound proves there is no unseen eligible
        merge newer than the selected before cutoff, or after the cutover.
        """
        all_pr = []
        page = 1
        while True:
            group = self.get("pulls", state="closed", sort="updated",
                             direction="desc", per_page=100, page=page)
            require(isinstance(group, list), "invalid PR list")
            all_pr += group
            eligible = [p for p in all_pr if p.get("merged_at") and
                        ((clock(p["merged_at"]) >= boundary) if mode == "after"
                         else (clock(p["merged_at"]) < boundary))]
            ordered = sorted(eligible, key=lambda x: (x["merged_at"], x["number"]),
                             reverse=(mode == "before"))
            if not group:
                return ordered[:limit]
            oldest_updated = min(clock(p["updated_at"]) for p in group)
            if mode == "after":
                if oldest_updated < boundary:
                    return sorted(eligible, key=lambda x: (x["merged_at"], x["number"]))[:limit]
            elif len(ordered) >= limit and oldest_updated < clock(ordered[limit - 1]["merged_at"]):
                return ordered[:limit]
            page += 1
            require(page <= 100, "merged PR search incomplete under page budget")

    def capture(self, mode, boundary, limit, include_jobs=False):
        require(type(include_jobs) is bool, "include_jobs must be bool")
        self.job_details = None
        prs = self.pr_candidates(mode, boundary, limit)
        if mode == "before":
            require(len(prs) == limit, "not enough prior merged PRs for baseline")
        # An empty after window is legitimate but no improvement can be
        # reported. Retain an absent after snapshot until the first PR lands.
        require(prs, "no merged PRs in requested window; no after data")
        rows = []
        all_costs = []
        job_rows, seen_job_ids = [], set()
        for pr in prs:
            head = pr["head"]["sha"]
            require(re.fullmatch(r"[0-9a-f]{40}", head), "missing exact HEAD")
            runs = self.paged("actions/runs", "workflow_runs", head_sha=head)
            selected = []
            for run in runs:
                require(run["head_sha"] == head, "run not bound to PR HEAD")
                require(run["status"] == "completed", "run still in progress: HOLD")
                selected.append({k:run.get(k) for k in (
                    "id", "name", "event", "conclusion", "status", "created_at",
                    "run_started_at", "updated_at", "run_attempt", "head_sha")})
                # GET filter=all includes every attempt. GitHub may reissue
                # successful rows with NEW job IDs without executing them again.
                # Include genuine failed and retried execution windows once each.
                jobs_path = "actions/runs/" + str(run["id"]) + "/jobs"
                provider_rows = self.paged(jobs_path, "jobs", filter="all")
                attempt = run.get("run_attempt")
                require(type(attempt) is int and attempt >= 1,
                        "workflow run lacks exact attempt count")
                present = {row.get("run_attempt", 1 if attempt == 1 else None)
                           for row in provider_rows}
                require(None not in present, "provider job missing rerun attempt")
                zero_attempts = []
                for missing in sorted(set(range(1, attempt + 1)) - present):
                    # An early workflow attempt may have ZERO allocated jobs.
                    # Prove that using its attempt-specific endpoint, not a guess.
                    proof = self.paged("actions/runs/" + str(run["id"]) +
                                       "/attempts/" + str(missing) + "/jobs", "jobs")
                    require(not proof, "missing nonempty historical job attempt")
                    zero_attempts.append(missing)
                jobs, execution_receipts = canonical_executions(
                    provider_rows, attempt, proven_empty_attempts=zero_attempts)
                # Detail reuses the same paginated GETs; only missing-attempt
                # zero-job provenance may require an additional read.
                if include_jobs:
                    for item in jobs:
                        jid, name = item.get("id"), item.get("name")
                        require(type(jid) is int and jid > 0 and jid not in seen_job_ids,
                                "missing or repeated provider job id")
                        require(isinstance(name, str) and 0 < len(name) <= 300,
                                "missing or invalid provider job name")
                        seen_job_ids.add(jid)
                        actual = runtime_window(item)
                        started, completed = actual if actual is not None else (None, None)
                        duration = ((clock(completed) - clock(started)).total_seconds()/60
                                    if actual is not None else None)
                        require(duration is None or
                                (math.isfinite(duration) and duration >= 0),
                                "invalid detailed job duration")
                        labels = item.get("labels") or []
                        require(isinstance(labels, list) and
                                all(isinstance(x, str) and len(x) <= 128 for x in labels),
                                "invalid job runner labels")
                        label_text = ",".join(labels).lower()
                        platform = ("windows" if "windows" in label_text else
                                    "macos" if "macos" in label_text else
                                    "linux" if "ubuntu" in label_text or "linux" in label_text
                                    else "unknown")
                        job_rows.append({
                            "job_id": jid, "job_name": name,
                            "run_id": run["id"], "run_attempt": run.get("run_attempt"),
                            "execution_attempt": item.get("run_attempt", 1),
                            "workflow": run["name"], "pr": pr["number"], "head_sha": head,
                            "platform": platform, "labels": labels,
                            "conclusion": item.get("conclusion"), "started_at": started,
                            "completed_at": completed,
                            "nonexecution_provider_started_at": item.get("started_at") if actual is None else None,
                            "nonexecution_provider_completed_at": item.get("completed_at") if actual is None else None,
                            "elapsed_minutes": round(duration, 3) if duration is not None else None,
                        })
                buckets, elapsed, first_start, last_completed = {}, 0.0, None, None
                for job in jobs:
                    actual = runtime_window(job)
                    if actual is None:
                        continue
                    start, finish = actual
                    minutes = (clock(finish) - clock(start)).total_seconds()/60
                    require(math.isfinite(minutes) and minutes >= 0,
                            "invalid job duration")
                    labels = ",".join(job.get("labels") or []).lower()
                    platform = ("windows" if "windows" in labels else
                                "macos" if "macos" in labels else
                                "linux" if "ubuntu" in labels or "linux" in labels
                                else "unknown")
                    buckets[platform] = buckets.get(platform, 0.0)+minutes
                    elapsed += minutes
                    first_start = min(first_start, start) if first_start else start
                    last_completed = max(last_completed, finish) if last_completed else finish
                all_costs.append({
                    "id": run["id"], "pr": pr["number"],
                    # job_count means actual distinct execution windows, not
                    # provider metadata rows echoed again on a retry.
                    "job_count": len(jobs), "returned": len(jobs), "complete": True,
                    "provider_job_records": execution_receipts["provider_job_records"],
                    "carried_reissued_records": execution_receipts["reissued_carried_records"],
                    "verified_zero_job_attempts": execution_receipts["verified_empty_attempts"],
                    "run_attempts": attempt,
                    "not_executed_skipped_jobs": sum(j["conclusion"] == "skipped" for j in jobs),
                    "not_executed_cancelled_jobs": sum(j["conclusion"] == "cancelled" and runtime_window(j) is None for j in jobs),
                    "job_minutes": round(elapsed, 3),
                    "by_platform_minutes": {k: round(v, 3) for k,v in buckets.items()},
                    "first_job_started": first_start, "last_job_completed": last_completed,
                })
            rows.append({
                "number": pr["number"], "merged_at": pr["merged_at"],
                "created_at": pr["created_at"], "base_branch": pr["base"]["ref"],
                "head_sha": head, "head_branch": pr["head"]["ref"],
                "run_total_reported": len(selected), "run_list_complete": True,
                "runs": selected,
            })
        base = {
            "schema": "lexlean/ci-pr-baseline/1", "window": mode,
            "scope": "exact-final-HEAD-attributed-runs-only",
            "source": "GitHub REST, on-demand read-only collector",
            "cutover_commit": None,
            "cutover_utc": boundary.isoformat(), "pr_count": len(rows), "rows": rows,
        }
        costs = {
            "schema": "lexlean/ci-pr-job-snapshot/1",
            "linked_run_inventory": mode + "-runs.json",
            "expected_runs": len(all_costs), "run_costs": all_costs,
        }
        if include_jobs:
            require(len(job_rows) == sum(x["job_count"] for x in all_costs),
                    "incomplete provider job ledger")
            self.job_details = {
                "schema": JOB_SCHEMA,
                "scope": "exact-final-HEAD GitHub executed-job elapsed time; NOT billed time",
                "period": mode, "expected_jobs": len(job_rows),
                "linked_run_inventory": mode + "-runs.json",
                "linked_cost_inventory": mode + "-costs.json",
                "jobs": job_rows,
            }
        return base, costs


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", default="UOR-Foundation/lexlean")
    parser.add_argument("--period", choices=("before", "after"), required=True)
    parser.add_argument("--cutover-utc", required=True)
    parser.add_argument("--limit", type=int, default=50)
    parser.add_argument("--max-api-calls", type=int, default=800)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--include-job-breakdown", action="store_true",
                        help="Capture per-job timing from existing API pages, no additional reads")
    args = parser.parse_args(argv)
    require(1 <= args.limit <= 50, "PR cohort must be within 1..50")
    boundary = clock(args.cutover_utc)
    reader = Reader(args.repo, os.environ.get("GITHUB_TOKEN", ""), args.max_api_calls)
    runs, costs = reader.capture(args.period, boundary, args.limit,
                                 include_jobs=args.include_job_breakdown)
    args.output_dir.mkdir(parents=True, exist_ok=True)
    # Do not write partial snapshots if a GitHub query or bound failed.
    (args.output_dir/(args.period+"-runs.json")).write_text(
        json.dumps(runs, indent=2, sort_keys=True)+"\n", encoding="utf-8")
    (args.output_dir/(args.period+"-costs.json")).write_text(
        json.dumps(costs, indent=2, sort_keys=True)+"\n", encoding="utf-8")
    if args.include_job_breakdown:
        (args.output_dir/(args.period+"-jobs.json")).write_text(
            json.dumps(reader.job_details, indent=2, sort_keys=True)+"\n", encoding="utf-8")
    print(json.dumps({
        "period": args.period, "pr_count": len(runs["rows"]),
        "workflow_runs": len(costs["run_costs"]), "read_api_calls": reader.requests,
        "detailed_jobs": len(reader.job_details["jobs"]) if reader.job_details else None,
        "actual_billed_minutes": None,
        "provider_cost": "NOT_MEASURED", "scope": runs["scope"]}, sort_keys=True))
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, OSError, KeyError, TypeError) as exc:
        print("HOLD: " + str(exc), file=sys.stderr)
        sys.exit(2)

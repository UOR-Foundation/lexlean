#!/usr/bin/env python3
"""DOD-6 provider collector for UOR-Foundation/lexlean issue #603.

Read-only: every provider call is a `gh api` GET. Nothing is posted,
dispatched, re-run, cancelled or labelled.

Collects:
  * the preserved 50-PR BEFORE cohort (run IDs taken unchanged from
    .github/metrics/ci-before-pr50-runs.json on advisory PR #606);
  * every workflow run created in the observation window (default from
    2026-11-22T00:00:00Z, which precedes the #601 v0.4 draft guard, #602,
    #619 and #620), for draft-avoidance, post-merge reuse and AFTER cohorts;
  * PR metadata and timelines (ready_for_review) for merged PRs.

Every raw provider response is cached under raw/ so analyze.py can be
re-run offline (`--offline`) against the exact bytes collected.

Lineage rule (retried runs): the provider relabels carried-forward jobs
with the new run_attempt and new job IDs but keeps the original
started_at/completed_at. Each executed (name, started_at, completed_at)
is counted once, keeping the lowest attempt. Jobs that never received a
runner (skipped, or cancelled before allocation: runner_name null and no
executed steps) carry synthetic timestamps and are not execution.

Jobs are read at per_page=50: per_page>=60 returns HTTP 502 for retried
runs (#620).
"""
import argparse
import json
import os
import re
import subprocess
import sys
import time
from datetime import datetime, timezone

REPO = "UOR-Foundation/lexlean"
HERE = os.path.dirname(os.path.abspath(__file__))
RAW = os.path.join(HERE, "raw")
BEFORE_RUNS_FILE = os.path.join(HERE, "advisory", "ci-before-pr50-runs.json")
JOB_PAGE = 50
OFFLINE = False
REFRESH_ACTIVE = True
CALLS = {"n": 0}


def clock(s):
    return datetime.fromisoformat(s.replace("Z", "+00:00")) if s else None


def gh_get(endpoint, tries=4):
    """One GET through the gh CLI, bounded retry on transient 5xx."""
    if OFFLINE:
        raise RuntimeError("offline mode: uncached endpoint " + endpoint)
    last = None
    for attempt in range(tries):
        CALLS["n"] += 1
        p = subprocess.run(["gh", "api", "-X", "GET", endpoint],
                           capture_output=True, text=True, encoding="utf-8")
        if p.returncode == 0:
            return json.loads(p.stdout)
        last = p.stderr.strip()
        if "HTTP 404" in last:
            return None
        time.sleep(2 * (attempt + 1))
    raise RuntimeError("GET failed after %d tries: %s: %s" % (tries, endpoint, last))


def cache_path(kind, key):
    d = os.path.join(RAW, kind)
    os.makedirs(d, exist_ok=True)
    return os.path.join(d, re.sub(r"[^A-Za-z0-9_.-]", "_", str(key)) + ".json")


def cached(kind, key, fetch, terminal=lambda v: True):
    path = cache_path(kind, key)
    if os.path.exists(path):
        with open(path, encoding="utf-8") as f:
            v = json.load(f)
        if OFFLINE or terminal(v) or not REFRESH_ACTIVE:
            return v
    v = fetch()
    with open(path, "w", encoding="utf-8") as f:
        json.dump(v, f, indent=1, sort_keys=True)
    return v


# --------------------------------------------------------------- runs/jobs

def get_run(run_id):
    return cached("runs", run_id,
                  lambda: gh_get("repos/%s/actions/runs/%s" % (REPO, run_id)),
                  terminal=lambda v: v is None or v.get("status") == "completed")


def get_jobs(run_id, run_status):
    """All attempts' job rows, read in pages of 50, fail closed on gaps."""
    def fetch():
        rows, pages, page, total = [], [], 1, None
        while True:
            r = gh_get("repos/%s/actions/runs/%s/jobs?filter=all&per_page=%d&page=%d"
                       % (REPO, run_id, JOB_PAGE, page))
            if r is None:
                raise RuntimeError("jobs 404 for run %s" % run_id)
            if total is None:
                total = r["total_count"]
            elif r["total_count"] != total:
                raise RuntimeError("total_count changed between pages for %s" % run_id)
            pages.append({"page": page, "returned": len(r["jobs"])})
            rows += r["jobs"]
            if len(rows) >= total or not r["jobs"]:
                break
            page += 1
        ids = [j["id"] for j in rows]
        if len(ids) != len(set(ids)):
            raise RuntimeError("duplicate job id in pages for %s" % run_id)
        if len(rows) != total:
            raise RuntimeError("short job inventory for %s: %d of %d" % (run_id, len(rows), total))
        return {"run_id": run_id, "total_count": total, "pages": pages,
                "page_size": JOB_PAGE, "run_status_at_read": run_status,
                "read_at": datetime.now(timezone.utc).isoformat(), "jobs": rows}
    return cached("jobs", run_id, fetch,
                  terminal=lambda v: v.get("run_status_at_read") == "completed")


def list_runs(since):
    """Every run created at/after `since` (all workflows, all events)."""
    key = "since_" + since.replace(":", "")
    def fetch():
        out, page = [], 1
        while True:
            r = gh_get("repos/%s/actions/runs?created=%%3E%%3D%s&per_page=100&page=%d"
                       % (REPO, since, page))
            out += r["workflow_runs"]
            if len(out) >= r["total_count"] or not r["workflow_runs"]:
                break
            page += 1
            if page > 10:
                raise RuntimeError("run list beyond 1000-result search cap; narrow window")
        return {"since": since, "total_count": r["total_count"],
                "read_at": datetime.now(timezone.utc).isoformat(), "runs": out}
    return cached("runlist", key, fetch, terminal=lambda v: False)


def get_pr(number):
    return cached("pulls", number,
                  lambda: gh_get("repos/%s/pulls/%s" % (REPO, number)),
                  terminal=lambda v: v is None or v.get("state") == "closed")


def get_timeline(number, pr_state):
    def fetch():
        out, page = [], 1
        while True:
            # issue events (not the full timeline): carries ready_for_review /
            # convert_to_draft without comment and commit bodies
            r = gh_get("repos/%s/issues/%s/events?per_page=100&page=%d" % (REPO, number, page))
            out += r
            if len(r) < 100:
                break
            page += 1
        # keep only the fields needed: event kind and time
        return {"number": number, "state_at_read": pr_state,
                "events": [{"event": e.get("event"),
                            "created_at": e.get("created_at") or
                            (e.get("committer") or {}).get("date"),
                            "sha": e.get("sha")} for e in out]}
    return cached("timeline", number, fetch,
                  terminal=lambda v: v.get("state_at_read") == "closed")


def merged_since(since):
    def fetch():
        out, page = [], 1
        while page <= 5:
            r = gh_get("repos/%s/pulls?state=closed&sort=updated&direction=desc&per_page=100&page=%d"
                       % (REPO, page))
            out += [{"number": p["number"], "merged_at": p.get("merged_at"),
                     "updated_at": p["updated_at"]} for p in r]
            if len(r) < 100 or min(p["updated_at"] for p in r) < since:
                break
            page += 1
        return out
    rows = cached("pullslist", "closed_" + since.replace(":", ""), fetch, terminal=lambda v: False)
    return [p for p in rows if p["merged_at"] and p["merged_at"] >= since]


def get_commit_pulls(sha):
    return cached("commitpulls", sha,
                  lambda: gh_get("repos/%s/commits/%s/pulls" % (REPO, sha)))


# ----------------------------------------------------------- classification

def classify_host(job):
    """Runner OS/arch from labels; job-name fallback for skipped placeholders."""
    labels = ",".join(job.get("labels") or []).lower()
    name = (job.get("name") or "").lower()
    if labels:
        if "windows" in labels:
            return "windows-arm64" if "arm" in labels else "windows-x64"
        if "macos" in labels:
            if "intel" in labels or re.search(r"macos-1[0-3]\b", labels) or "large" in labels:
                return "macos-x64"
            return "macos-arm64"
        if "ubuntu" in labels or "linux" in labels:
            return "linux-arm64" if "arm" in labels else "linux-x64"
        return "unknown"
    if name.startswith("windows"):
        return "windows-x64"
    if name.startswith("intel"):
        return "macos-x64"
    if "macos arm64" in name or "apple-darwin" in name:
        return "macos-arm64"
    if "aarch64-unknown-linux" in name or "linux arm" in name:
        return "linux-arm64"
    return "linux-x64"


def allocated(job):
    """A job executed iff the provider assigned it a runner (or ran steps)."""
    if job.get("conclusion") == "skipped":
        return False
    if not (job.get("started_at") and job.get("completed_at")):
        return False
    if clock(job["completed_at"]) < clock(job["started_at"]):
        return False
    if job.get("runner_name") or job.get("runner_id"):
        return True
    steps = job.get("steps") or []
    return any(s.get("status") == "completed" and s.get("conclusion") not in (None, "skipped")
               for s in steps)


def reconcile(jobs_doc):
    """Collapse retried-run lineage: each executed window counted once."""
    rows = sorted(jobs_doc["jobs"], key=lambda j: (j.get("run_attempt") or 1, j["id"]))
    seen, executions, placeholders = {}, [], []
    attempts = {}
    for j in rows:
        a = j.get("run_attempt") or 1
        attempts.setdefault(a, {"rows": 0, "executed_new": 0, "carried": 0, "unallocated": 0})
        attempts[a]["rows"] += 1
        if not allocated(j):
            attempts[a]["unallocated"] += 1
            placeholders.append({"name": j["name"], "attempt": a,
                                 "conclusion": j.get("conclusion"), "host": classify_host(j)})
            continue
        key = (j["name"], j["started_at"], j["completed_at"])
        if key in seen:
            attempts[a]["carried"] += 1
            seen[key]["latest_attempt"] = a
            seen[key]["latest_conclusion"] = j.get("conclusion")
            continue
        attempts[a]["executed_new"] += 1
        e = {"job_id": j["id"], "name": j["name"], "attempt": a, "latest_attempt": a,
             "conclusion": j.get("conclusion"), "latest_conclusion": j.get("conclusion"),
             "created_at": j.get("created_at"), "started_at": j["started_at"],
             "completed_at": j["completed_at"], "host": classify_host(j),
             "labels": j.get("labels") or [], "runner_name": j.get("runner_name"),
             "minutes": round((clock(j["completed_at"]) - clock(j["started_at"])).total_seconds() / 60, 4)}
        seen[key] = e
        executions.append(e)
    max_attempt = max(attempts) if attempts else 1
    for e in executions:
        # retained: executed once and still present (carried) in the final attempt
        e["in_final_attempt"] = e["latest_attempt"] == max_attempt
    return {"executions": executions, "placeholders": placeholders,
            "attempts": {str(k): v for k, v in sorted(attempts.items())},
            "provider_rows": len(rows)}


# ------------------------------------------------------------ gate revision

GIT = ["git", "-C", r"C:\dev\UOR\lexlean"]
FEATURES = {
    # first commits carrying each mechanism (see report.md, Phase attribution)
    "v04_draft_guard_601": "cb8eca0021",   # draft jobs skip before allocation on release/v0.4
    "split_602": "3588a567c8",             # independent vs bootstrap-dependent split (issue's "Phase A" PR)
    "b1_cutover_619": "bd80fa3739",        # #619 head: v0.5 draft suppression / cutover
    "b1_reader_620": "603336bf27",         # #620 head: pages of 50, bounded atomic reads, targeted retry
    "b2_combined_609": "96f35dce8c",       # #609/#607 combined composition (B2 pilot)
}
_anc = {}


def is_anc(c, sha):
    if not sha:
        return False
    k = (c, sha)
    if k not in _anc:
        _anc[k] = subprocess.run(GIT + ["merge-base", "--is-ancestor", c, sha],
                                 capture_output=True).returncode == 0
    return _anc[k]


_tip = {}


def base_tip(branch, at):
    k = (branch, at)
    if k not in _tip:
        p = subprocess.run(GIT + ["rev-list", "-1", "--first-parent", "--before=" + at,
                                  "origin/" + branch], capture_output=True, text=True)
        _tip[k] = p.stdout.strip() or None
    return _tip[k]


def gate_features(run, base):
    shas = [run["head_sha"]]
    if run["event"] in ("pull_request", "pull_request_target") and base:
        t = base_tip(base, run["created_at"])
        if t:
            shas.append(t)
    return {f: any(is_anc(c, s) for s in shas) for f, c in FEATURES.items()}


def phase_of(feat):
    if feat["b2_combined_609"]:
        return "B2"
    if feat["b1_cutover_619"]:
        return "B1"
    return "A"


# --------------------------------------------------------------------- main

def summarize_run(run, base, pr_number, jobs_doc):
    rec = reconcile(jobs_doc)
    feat = gate_features(run, base)
    return {
        "id": run["id"], "workflow": run["name"], "event": run["event"],
        "status": run["status"], "conclusion": run["conclusion"],
        "head_sha": run["head_sha"], "head_branch": run["head_branch"],
        "base_branch": base, "pr": pr_number, "run_attempt": run.get("run_attempt"),
        "created_at": run["created_at"], "run_started_at": run.get("run_started_at"),
        "updated_at": run["updated_at"], "gate_features": feat, "phase": phase_of(feat),
        "provider_rows": rec["provider_rows"], "attempts": rec["attempts"],
        "executions": rec["executions"], "placeholders": rec["placeholders"],
    }


def pr_for_run(run):
    prs = run.get("pull_requests") or []
    if prs:
        return prs[0]["number"], prs[0]["base"]["ref"]
    if run["event"] in ("pull_request", "pull_request_target"):
        for p in get_commit_pulls(run["head_sha"]) or []:
            return p["number"], p["base"]["ref"]
    return None, None


def main(argv=None):
    global OFFLINE
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--since", default="2026-11-22T00:00:00Z")
    ap.add_argument("--offline", action="store_true", help="replay raw/ cache only")
    ap.add_argument("--out", default=os.path.join(HERE, "runs_normalized.json"))
    a = ap.parse_args(argv)
    OFFLINE = a.offline

    before = json.load(open(BEFORE_RUNS_FILE, encoding="utf-8"))
    before_rows = before["rows"]
    before_ids = {r["id"]: (row["number"], row["base_branch"])
                  for row in before_rows for r in row["runs"]}

    listing = list_runs(a.since)
    window = {r["id"]: r for r in listing["runs"]}

    out_runs = {}
    ids = sorted(set(window) | set(before_ids))

    def process(rid):
        run = window.get(rid) or get_run(rid)
        if run is None:
            print("missing run", rid, file=sys.stderr)
            return None
        if run["status"] != "completed":
            run = get_run(rid)
        if rid in before_ids:
            pr_number, base = before_ids[rid]
        else:
            pr_number, base = pr_for_run(run)
        if base is None and run["event"] in ("push", "workflow_dispatch", "schedule"):
            base = run["head_branch"]
        jobs = get_jobs(rid, run["status"])
        s = summarize_run(run, base, pr_number, jobs)
        s["in_before_cohort"] = rid in before_ids
        s["in_window"] = rid in window
        return s

    # Independent GETs and git ancestry checks; bounded parallelism (8).
    from concurrent.futures import ThreadPoolExecutor
    with ThreadPoolExecutor(max_workers=8) as pool:
        for i, s in enumerate(pool.map(process, ids)):
            if s is not None:
                out_runs[s["id"]] = s
            if i % 50 == 0:
                print("..%d/%d runs (api calls %d)" % (i, len(ids), CALLS["n"]), file=sys.stderr, flush=True)

    # PR metadata for every PR in BEFORE cohort and every PR merged in window
    prs = {}
    pr_numbers = {row["number"] for row in before_rows}
    pr_numbers |= {s["pr"] for s in out_runs.values() if s["pr"]}
    # Fork-origin PRs carry no run.pull_requests association, so also take
    # every PR merged inside the window from the closed-PR listing.
    pr_numbers |= {p["number"] for p in merged_since(a.since)}
    before_numbers = {row["number"] for row in before_rows}
    cutover = "2026-11-24T15:03:55Z"

    def pr_and_events(n):
        p = get_pr(n)
        if p is None:
            return n, None, None
        need = p.get("merged_at") and (n in before_numbers or p["merged_at"] >= cutover)
        return n, p, (get_timeline(n, p["state"]) if need else None)

    from concurrent.futures import ThreadPoolExecutor as _TPE
    with _TPE(max_workers=8) as pool:
        fetched = list(pool.map(pr_and_events, sorted(pr_numbers)))
    for n, p, tl in fetched:
        if p is None:
            continue
        ready = [e["created_at"] for e in (tl or {}).get("events", [])
                 if e["event"] == "ready_for_review" and e["created_at"]]
        prs[n] = {"number": n, "state": p["state"], "draft": p.get("draft"),
                  "base": p["base"]["ref"], "head_sha": p["head"]["sha"],
                  "head_ref": p["head"]["ref"], "created_at": p["created_at"],
                  "merged_at": p.get("merged_at"),
                  "merge_commit_sha": p.get("merge_commit_sha"),
                  "ready_for_review_events": ready}

    doc = {"schema": "lexlean/dod6-normalized/1", "repository": REPO,
           "collected_at": datetime.now(timezone.utc).isoformat(),
           "window_since": a.since, "window_run_total": listing["total_count"],
           "run_list_read_at": listing["read_at"],
           "before_cohort_source": {
               "file": ".github/metrics/ci-before-pr50-runs.json",
               "branch": "ci/phase-b-selective-development-feedback-20261122 (PR #606)",
               "window": before["window"], "scope": before["scope"],
               "pr_count": before["pr_count"], "run_count": len(before_ids)},
           "job_page_size": JOB_PAGE, "features": FEATURES,
           "api_calls_this_invocation": CALLS["n"],
           "runs": [out_runs[k] for k in sorted(out_runs)], "prs": prs}
    with open(a.out, "w", encoding="utf-8") as f:
        json.dump(doc, f, indent=1)
    print("wrote %s: %d runs, %d PRs, %d API calls" % (a.out, len(out_runs), len(prs), CALLS["n"]),
          file=sys.stderr)


if __name__ == "__main__":
    main()

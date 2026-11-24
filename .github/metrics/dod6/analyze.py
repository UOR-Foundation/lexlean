#!/usr/bin/env python3
"""DOD-6 analysis: runs_normalized.json -> dataset.json + report.md.

Offline and deterministic given runs_normalized.json. Every reported value
carries one evidence label:
  MEASURED        computed from provider job timestamps / PR events
  MODELED         measured minutes x published list price (not billing)
  COUNTERFACTUAL  minutes a run would have used had it executed a gate
  CONTROLLED      same source/host/toolchain comparison (none available yet)
"""
import json
import math
import os
import statistics
import sys
from collections import Counter, defaultdict
from datetime import datetime

HERE = os.path.dirname(os.path.abspath(__file__))
CUTOVER = "2026-11-24T15:03:55Z"      # #619 merged as 44616baa (B1 cutover on release/v0.5)
FIX_620 = "2026-11-24T20:33:54Z"      # #620 merged as 471237b3
RATES_SOURCE = {
    "url": "https://docs.github.com/en/billing/reference/actions-runner-pricing",
    "fetched": "2026-11-24",
    "page_date": "page carries no date/version",
    "rows": {
        "Linux 2-core (x64) actions_linux": 0.006,
        "Linux 2-core (arm64) actions_linux_arm": 0.005,
        "Windows 2-core (x64) actions_windows": 0.010,
        "Windows 2-core (arm64) actions_windows_arm": 0.010,
        "macOS 3-core or 4-core (M1 or Intel) actions_macos": 0.062,
    },
    "rounding": "page: minutes and partial minutes each job uses are rounded up to the nearest whole minute",
}
RATE = {"linux-x64": 0.006, "linux-arm64": 0.005, "windows-x64": 0.010,
        "windows-arm64": 0.010, "macos-arm64": 0.062, "macos-x64": 0.062, "unknown": 0.006}
HOSTS = ["linux-x64", "linux-arm64", "windows-x64", "macos-arm64", "macos-x64"]
FULL_GATE_MIN_JOBS = 40
KNOWN_RETRIES = [45316289312, 45437716141]
KNOWN_REUSE = 45482125626


def clock(s):
    return datetime.fromisoformat(s.replace("Z", "+00:00"))


def mins(a, b):
    return (clock(b) - clock(a)).total_seconds() / 60


def r3(x):
    return None if x is None else round(x, 3)


def pct(vals, p):
    v = sorted(vals)
    if not v:
        return None
    k = (len(v) - 1) * p
    f, c = math.floor(k), math.ceil(k)
    return v[f] if f == c else v[f] + (v[c] - v[f]) * (k - f)


def dist(vals):
    vals = [x for x in vals if x is not None]
    if not vals:
        return {"n": 0}
    return {"n": len(vals), "median": r3(statistics.median(vals)), "p90": r3(pct(vals, .9)),
            "p95": r3(pct(vals, .95)) if len(vals) >= 20 else None, "max": r3(max(vals)),
            "sum": r3(sum(vals))}


# ------------------------------------------------------------- run metrics

def run_metrics(r):
    ex = r["executions"]
    by = defaultdict(float)
    cost, cost_ceil = 0.0, 0.0
    for e in ex:
        by[e["host"]] += e["minutes"]
        cost += e["minutes"] * RATE[e["host"]]
        cost_ceil += math.ceil(e["minutes"] - 1e-9) * RATE[e["host"]] if e["minutes"] > 0 else 0
    fails = [e for e in ex if e["conclusion"] == "failure"]
    m = {
        "executed_jobs": len(ex), "placeholders": len(r["placeholders"]),
        "minutes": r3(sum(e["minutes"] for e in ex)),
        "by_host": {h: r3(v) for h, v in sorted(by.items())},
        "modeled_usd": round(cost, 4), "modeled_usd_per_job_ceil": round(cost_ceil, 4),
        "first_failure_min": r3(min(mins(r["created_at"], e["completed_at"]) for e in fails)) if fails else None,
        "terminal_min": r3(max(mins(r["created_at"], e["completed_at"]) for e in ex)) if ex else None,
        "queue_s": [(e["host"], (clock(e["started_at"]) - clock(e["created_at"])).total_seconds())
                    for e in ex if e.get("created_at") and e["attempt"] == 1],
        "full_gate": r["workflow"] == "ci" and len(ex) >= FULL_GATE_MIN_JOBS,
        "zero_alloc": len(ex) == 0 and len(r["placeholders"]) > 0 and
                      all(p["conclusion"] == "skipped" for p in r["placeholders"]),
        "plan_only": [e["name"] for e in ex] == ["plan"] and len(r["placeholders"]) > 0,
    }
    return m


def concurrency(runs):
    """Max simultaneously executing jobs, per host and overall (sweep)."""
    ev = defaultdict(list)
    for r in runs:
        for e in r["executions"]:
            for h in (e["host"], "all"):
                ev[h].append((clock(e["started_at"]), 1))
                ev[h].append((clock(e["completed_at"]), -1))
    out = {}
    for h, xs in ev.items():
        xs.sort(key=lambda t: (t[0], t[1]))
        cur = peak = 0
        for _, d in xs:
            cur += d
            peak = max(peak, cur)
        out[h] = peak
    return dict(sorted(out.items()))


def aggregate(runs, label):
    ms = [r["_m"] for r in runs]
    by = defaultdict(float)
    for m in ms:
        for h, v in m["by_host"].items():
            by[h] += v
    q = defaultdict(list)
    for m in ms:
        for h, s in m["queue_s"]:
            q[h].append(s)
            q["all"].append(s)
    term = [r["status"] == "completed" for r in runs]
    ci_full = [r for r in runs if r["_m"]["full_gate"]]
    return {
        "label": label, "runs": len(runs), "nonterminal_runs": term.count(False),
        "executed_jobs": sum(m["executed_jobs"] for m in ms),
        "unallocated_placeholders": sum(m["placeholders"] for m in ms),
        "minutes_MEASURED": r3(sum(m["minutes"] for m in ms)),
        "minutes_by_host_MEASURED": {h: r3(v) for h, v in sorted(by.items())},
        "modeled_usd_MODELED": round(sum(m["modeled_usd"] for m in ms), 2),
        "modeled_usd_per_job_ceil_MODELED": round(sum(m["modeled_usd_per_job_ceil"] for m in ms), 2),
        "full_gate_ci_runs": len(ci_full),
        "full_gate_minutes_MEASURED": dist([r["_m"]["minutes"] for r in ci_full]),
        "first_failure_min_MEASURED": dist([r["_m"]["first_failure_min"] for r in ci_full]),
        "terminal_min_MEASURED": dist([r["_m"]["terminal_min"] for r in ci_full]),
        "queue_delay_s_MEASURED": {h: dist(v) for h, v in sorted(q.items())},
        "max_concurrent_jobs_MEASURED": concurrency(runs),
    }


# -------------------------------------------------------------- retry/reuse

def retry_lineage(r):
    final = max(int(k) for k in r["attempts"]) if r["attempts"] else 1
    ex = r["executions"]
    retained = [e for e in ex if e["attempt"] < final and e["in_final_attempt"]]
    lost = [e for e in ex if e["attempt"] < final and not e["in_final_attempt"]]
    repeated = [e for e in ex if e["attempt"] == final and final > 1]
    s = lambda xs: r3(sum(e["minutes"] for e in xs))
    return {"run": r["id"], "head_sha": r["head_sha"], "final_attempt": final,
            "provider_rows": r["provider_rows"], "attempts": r["attempts"],
            "distinct_executions": len(ex),
            "retained_jobs": len(retained), "retained_min_MEASURED": s(retained),
            "lost_jobs": [(e["name"], e["conclusion"], e["minutes"]) for e in lost],
            "lost_min_MEASURED": s(lost),
            "repeated_jobs": [(e["name"], e["conclusion"], e["minutes"]) for e in repeated],
            "repeated_min_MEASURED": s(repeated),
            "total_executed_min_MEASURED": s(ex),
            "final_logical_inventory_min_MEASURED": s([e for e in ex if e["in_final_attempt"]]),
            "full_rerun_counterfactual_min_COUNTERFACTUAL": s([e for e in ex if e["in_final_attempt"]]),
            "avoided_by_targeted_retry_min_COUNTERFACTUAL": s(retained)}


def main():
    doc = json.load(open(os.path.join(HERE, "runs_normalized.json"), encoding="utf-8"))
    runs = doc["runs"]
    prs = {int(k): v for k, v in doc["prs"].items()}
    for r in runs:
        r["_m"] = run_metrics(r)
        r["line"] = r["base_branch"] or r["head_branch"]
    byid = {r["id"]: r for r in runs}

    # ---------------- BEFORE cohort (preserved definition)
    before = [r for r in runs if r["in_before_cohort"]]
    before_agg = aggregate(before, "BEFORE 50-PR cohort (preserved)")
    per_pr = defaultdict(float)
    for r in before:
        per_pr[r["pr"]] += r["_m"]["minutes"]
    before_agg["per_pr_minutes_MEASURED"] = dist(list(per_pr.values()))
    before_agg["reference_corrected_total"] = 88298.354
    before_agg["delta_vs_reference_min"] = r3(before_agg["minutes_MEASURED"] - 88298.354)
    before_agg["gate_feature_counts"] = dict(Counter(
        "+".join(k for k, v in r["gate_features"].items() if v) or "none" for r in before))

    def pr_latency(numbers, cohort_runs):
        rows = []
        for n in sorted(numbers):
            p = prs.get(n)
            if not p or not p.get("merged_at"):
                continue
            mine = [r for r in cohort_runs if r["head_sha"] == p["head_sha"]]
            first_run = min((r["created_at"] for r in mine), default=None)
            ready = [t for t in p["ready_for_review_events"] if t <= p["merged_at"]]
            start = max([t for t in [first_run, max(ready) if ready else None] if t], default=None)
            ci = [r for r in mine if r["workflow"] == "ci" and r["event"] == "pull_request"
                  and r["_m"]["executed_jobs"] > 0]
            rp = [r for r in mine if r["workflow"] == "reproducibility" and r["event"] == "pull_request"
                  and r["_m"]["executed_jobs"] > 0]
            green = None
            okci = [r for r in ci if r["conclusion"] == "success" and r["updated_at"] <= p["merged_at"]]
            okrp = [r for r in rp if r["conclusion"] == "success" and r["updated_at"] <= p["merged_at"]]
            if okci and okrp and first_run:
                green = mins(first_run, max(min(x["updated_at"] for x in okci),
                                            min(x["updated_at"] for x in okrp)))
            ff = [r["_m"]["first_failure_min"] for r in ci if r["_m"]["first_failure_min"] is not None]
            rows.append({"pr": n, "base": p["base"], "merged_at": p["merged_at"],
                         "ready_or_last_push_at": start,
                         "ready_or_push_to_merge_min": r3(mins(start, p["merged_at"])) if start else None,
                         "final_head_time_to_green_min": r3(green),
                         "first_failure_min": r3(min(ff)) if ff else None,
                         "final_head_minutes": r3(sum(r["_m"]["minutes"] for r in mine))})
        return rows

    before_prs = pr_latency({r["pr"] for r in before}, before)
    before_agg["pr_ready_or_push_to_merge_min_MEASURED"] = dist([x["ready_or_push_to_merge_min"] for x in before_prs])
    before_agg["pr_time_to_green_min_MEASURED"] = dist([x["final_head_time_to_green_min"] for x in before_prs])
    before_agg["pr_first_failure_min_MEASURED"] = dist([x["first_failure_min"] for x in before_prs])

    # ---------------- window by phase (gate revision)
    window = [r for r in runs if r["in_window"]]
    phases = {}
    for ph in ("A", "B1", "B2"):
        sel = [r for r in window if r["phase"] == ph]
        phases[ph] = aggregate(sel, "window runs, gate revision phase " + ph)
        phases[ph]["by_line_minutes_MEASURED"] = {
            k: r3(sum(r["_m"]["minutes"] for r in sel if r["line"] == k))
            for k in sorted({r["line"] or "?" for r in sel})}
    phases["C"] = {"label": "Rust-specific optimization", "runs": 0, "note": "none implemented yet (DOD-7 not started)"}

    # ---------------- AFTER cohort
    after = [r for r in window if r["created_at"] >= CUTOVER]
    after_lines = defaultdict(list)
    for r in after:
        key = r["line"] if r["line"] in ("release/v0.5", "release/v0.4") else "other"
        after_lines[(key, r["phase"])].append(r)
    after_agg = {"%s|%s" % k: aggregate(v, "AFTER runs %s phase %s" % k) for k, v in sorted(after_lines.items())}
    after_agg_all = aggregate(after, "AFTER all runs since cutover")
    after_pr_numbers = sorted(n for n, p in prs.items()
                              if p.get("merged_at") and p["merged_at"] >= CUTOVER)
    after_prs = pr_latency(set(after_pr_numbers), window)

    # ---------------- draft / zero-allocation avoidance
    full_med = {}
    for ph in ("A", "B1", "B2"):
        for line in ("release/v0.4", "release/v0.5"):
            xs = [r["_m"]["minutes"] for r in window + before
                  if r["_m"]["full_gate"] and r["line"] == line and r["phase"] == ph and r["status"] == "completed"]
            if xs:
                full_med[(ph, line)] = statistics.median(xs)
    full_any = statistics.median([r["_m"]["minutes"] for r in window + before
                                  if r["_m"]["full_gate"] and r["status"] == "completed"])
    repro_any = statistics.median([r["_m"]["minutes"] for r in window + before
                                   if r["workflow"] == "reproducibility" and r["_m"]["executed_jobs"] > 0
                                   and r["status"] == "completed"])
    zero = []
    for r in window:
        if r["event"] != "pull_request" or r["workflow"] not in ("ci", "reproducibility"):
            continue
        if not r["_m"]["zero_alloc"]:
            continue
        p = prs.get(r["pr"]) or {}
        if r["workflow"] == "ci":
            cf = full_med.get((r["phase"], r["line"]), full_any)
        else:
            cf = repro_any
        zero.append({"run": r["id"], "workflow": r["workflow"], "pr": r["pr"], "line": r["line"],
                     "phase": r["phase"], "created_at": r["created_at"],
                     "placeholders": r["_m"]["placeholders"],
                     "pr_draft_now": p.get("draft"), "pr_state_now": p.get("state"),
                     "counterfactual_full_gate_min_COUNTERFACTUAL": r3(cf)})
    za = defaultdict(lambda: {"runs": 0, "ci_runs": 0, "repro_runs": 0, "placeholders": 0,
                              "counterfactual_min_COUNTERFACTUAL": 0.0, "pr_currently_draft": 0})
    for z in zero:
        k = "%s|%s" % (z["line"], z["phase"])
        za[k]["runs"] += 1
        za[k]["ci_runs" if z["workflow"] == "ci" else "repro_runs"] += 1
        za[k]["placeholders"] += z["placeholders"]
        za[k]["counterfactual_min_COUNTERFACTUAL"] += z["counterfactual_full_gate_min_COUNTERFACTUAL"]
        za[k]["pr_currently_draft"] += 1 if z["pr_draft_now"] else 0
    # partial-draft counterfactual: drafts before the guard ran honesty + `rest`
    partial = [r["_m"]["minutes"] for r in window + before if r["workflow"] == "ci" and
               r["event"] == "pull_request" and 1 < r["_m"]["executed_jobs"] <= 5 and r["status"] == "completed"]
    partial_med = statistics.median(partial) if partial else 0.0
    for v in za.values():
        v["counterfactual_min_COUNTERFACTUAL"] = r3(v["counterfactual_min_COUNTERFACTUAL"])
        v["lower_counterfactual_min_COUNTERFACTUAL"] = r3(v["ci_runs"] * partial_med + v["repro_runs"] * repro_any)

    # ---------------- retries
    retries = [retry_lineage(byid[i]) for i in KNOWN_RETRIES if i in byid]
    other_retries = [retry_lineage(r) for r in window
                     if (r["run_attempt"] or 1) > 1 and r["id"] not in KNOWN_RETRIES and r["_m"]["executed_jobs"]]

    # ---------------- post-merge reuse
    reuse = []
    for r in window:
        if r["workflow"] == "ci" and r["event"] == "push" and r["_m"]["plan_only"]:
            cf = full_med.get((r["phase"], r["line"]), full_any)
            reuse.append({"run": r["id"], "line": r["line"], "phase": r["phase"],
                          "head_sha": r["head_sha"], "created_at": r["created_at"],
                          "conclusion": r["conclusion"], "plan_min_MEASURED": r["_m"]["minutes"],
                          "placeholders": r["_m"]["placeholders"],
                          "counterfactual_full_gate_min_COUNTERFACTUAL": r3(cf)})
    pushes_full = [{"run": r["id"], "line": r["line"], "phase": r["phase"], "head_sha": r["head_sha"],
                    "conclusion": r["conclusion"], "minutes_MEASURED": r["_m"]["minutes"],
                    "created_at": r["created_at"]}
                   for r in window if r["workflow"] == "ci" and r["event"] == "push" and r["_m"]["full_gate"]]
    ref620 = byid.get(45437716141)
    specific = None
    if KNOWN_REUSE in byid and ref620:
        rl = retry_lineage(ref620)
        specific = {"run": KNOWN_REUSE, "plan_min_MEASURED": byid[KNOWN_REUSE]["_m"]["minutes"],
                    "reused_verified_run": 45437716141,
                    "counterfactual_rerun_of_verified_inventory_min_COUNTERFACTUAL": rl["final_logical_inventory_min_MEASURED"],
                    "observed_prior_merge_push_45421145394_min_MEASURED":
                        byid[45421145394]["_m"]["minutes"] if 45421145394 in byid else None}

    # ---------------- assurance parity
    def names(r):
        return sorted({e["name"] for e in r["executions"]} | {p["name"] for p in r["placeholders"]})
    def latest_full(ph, line):
        xs = [r for r in window + before if r["_m"]["full_gate"] and r["phase"] == ph and r["line"] == line]
        xs = [r for r in xs if r["status"] == "completed"] or xs
        return max(xs, key=lambda r: r["created_at"]) if xs else None
    parity = {}
    refs = {k: latest_full(*k) for k in [("A", "release/v0.5"), ("A", "release/v0.4"),
                                         ("B1", "release/v0.5"), ("B2", "release/v0.4")]}
    for k, r in refs.items():
        if r:
            parity["%s|%s" % k] = {"run": r["id"], "head_sha": r["head_sha"], "status": r["status"],
                                   "conclusion": r["conclusion"], "job_names": names(r),
                                   "executed": r["_m"]["executed_jobs"], "placeholders": r["_m"]["placeholders"]}
    def diff(a, b):
        if a in parity and b in parity:
            A, B = set(parity[a]["job_names"]), set(parity[b]["job_names"])
            return {"from": a, "to": b, "missing_in_to": sorted(A - B), "new_in_to": sorted(B - A)}
    parity_diffs = [d for d in [diff("A|release/v0.5", "B1|release/v0.5"),
                                diff("A|release/v0.4", "B2|release/v0.4"),
                                diff("A|release/v0.4", "A|release/v0.5")] if d]

    # Union roster over every full-gate ci run per (phase, line): a name that
    # any A run on a line carried must still appear in that line's B runs.
    union = defaultdict(set)
    ucount = Counter()
    for r in window + before:
        if r["_m"]["full_gate"]:
            union[(r["phase"], r["line"])] |= set(names(r))
            ucount[(r["phase"], r["line"])] += 1
    parity_union = {"%s|%s" % k: {"full_gate_runs": ucount[k], "distinct_names": len(v)} for k, v in union.items()}
    for ph, line in [("B1", "release/v0.5"), ("B2", "release/v0.4")]:
        if (ph, line) in union:
            A, B = union[("A", line)], union[(ph, line)]
            parity_union["%s|%s_vs_A" % (ph, line)] = {"missing_in_B": sorted(A - B), "new_in_B": sorted(B - A)}
        else:
            parity_union["%s|%s_vs_A" % (ph, line)] = "NO TERMINAL FULL GATE YET"

    dataset = {
        "schema": "lexlean/dod6-dataset/1", "collected_at": doc["collected_at"],
        "repository": doc["repository"], "repository_visibility": "public (standard hosted runners not billed per-minute; all cost below is MODELED private-equivalent)",
        "cutover_b1": {"pr": 619, "merge": "44616baa", "at": CUTOVER},
        "fix_620": {"pr": 620, "merge": "471237b3", "at": FIX_620},
        "rates_MODELED": RATES_SOURCE, "rate_map": RATE,
        "before_cohort_definition": doc["before_cohort_source"],
        "before": before_agg, "before_prs": before_prs,
        "phases_window": phases, "window_since": doc["window_since"],
        "window_run_total_reported": doc["window_run_total"],
        "after_all": after_agg_all, "after_by_line_phase": after_agg,
        "after_merged_prs": after_prs, "after_pr_count": len(after_pr_numbers),
        "zero_allocation_pr_runs": zero, "zero_allocation_summary": dict(za),
        "counterfactual_basis": {"full_gate_median_min_by_phase_line": {"%s|%s" % k: r3(v) for k, v in full_med.items()},
                                 "full_gate_median_min_all": r3(full_any),
                                 "repro_median_min_all": r3(repro_any),
                                 "pre_guard_partial_draft_ci_min": dist(partial)},
        "targeted_retries": retries, "other_multi_attempt_runs": other_retries,
        "post_merge_plan_only_runs": reuse, "post_merge_full_gate_push_runs": pushes_full,
        "post_merge_specific": specific,
        "assurance_parity": parity, "assurance_parity_diffs": parity_diffs,
        "assurance_parity_union": parity_union,
        "runs": [{k: v for k, v in r.items() if k not in ("executions", "placeholders", "_m")} |
                 {"metrics": {k: v for k, v in r["_m"].items() if k != "queue_s"}} for r in runs],
    }
    with open(os.path.join(HERE, "dataset.json"), "w", encoding="utf-8") as f:
        json.dump(dataset, f, indent=1)
    print("dataset.json written", file=sys.stderr)


if __name__ == "__main__":
    main()

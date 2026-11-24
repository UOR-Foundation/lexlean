#!/usr/bin/env python3
"""dataset.json -> report.md (tables only; narrative lives in NOTES below)."""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
HOSTS = ["linux-x64", "linux-arm64", "windows-x64", "macos-arm64", "macos-x64"]


def f(x, nd=1):
    if x is None:
        return "n/a"
    if isinstance(x, float):
        return ("{:,.%df}" % nd).format(x)
    return "{:,}".format(x) if isinstance(x, int) else str(x)


def d(dd, key="median"):
    return f(dd.get(key)) if dd and dd.get("n") else "n/a"


def agg_rows(items):
    out = ["| Group | Runs (nonterminal) | Executed jobs | Job-min MEASURED | " +
           " | ".join(HOSTS) + " | Modeled USD (exact / per-job ceil) MODELED | Max concurrent (all) |",
           "|---" * (6 + len(HOSTS)) + "|"]
    for name, a in items:
        if not a.get("runs"):
            out.append("| %s | 0 | - | - |%s - | - |" % (name, " - |" * len(HOSTS)))
            continue
        bh = a["minutes_by_host_MEASURED"]
        out.append("| %s | %d (%d) | %s | %s | %s | %s / %s | %s |" % (
            name, a["runs"], a["nonterminal_runs"], f(a["executed_jobs"]), f(a["minutes_MEASURED"]),
            " | ".join(f(bh.get(h)) if bh.get(h) else "0" for h in HOSTS),
            f(a["modeled_usd_MODELED"], 2), f(a["modeled_usd_per_job_ceil_MODELED"], 2),
            a["max_concurrent_jobs_MEASURED"].get("all", 0)))
    return "\n".join(out)


def lat_rows(items):
    out = ["| Group | Full-gate ci runs | Job-min/full gate (median, max) | First failure min (n, median, max) | Terminal min (median, p90, max) | Queue s all (median, p95, max) | Queue s macos-x64 (median, max) | Queue s macos-arm64 (median, max) |",
           "|---|---|---|---|---|---|---|---|"]
    for name, a in items:
        if not a.get("runs"):
            continue
        q = a["queue_delay_s_MEASURED"]
        ff = a["first_failure_min_MEASURED"]
        out.append("| %s | %d | %s, %s | %s, %s, %s | %s, %s, %s | %s, %s, %s | %s, %s | %s, %s |" % (
            name, a["full_gate_ci_runs"], d(a["full_gate_minutes_MEASURED"]), d(a["full_gate_minutes_MEASURED"], "max"),
            ff.get("n", 0), d(ff), d(ff, "max"),
            d(a["terminal_min_MEASURED"]), d(a["terminal_min_MEASURED"], "p90"), d(a["terminal_min_MEASURED"], "max"),
            d(q.get("all")), d(q.get("all"), "p95"), d(q.get("all"), "max"),
            d(q.get("macos-x64")), d(q.get("macos-x64"), "max"),
            d(q.get("macos-arm64")), d(q.get("macos-arm64"), "max")))
    return "\n".join(out)


def conc_rows(items):
    keys = HOSTS + ["all"]
    out = ["| Group | " + " | ".join(keys) + " |", "|---" * (len(keys) + 1) + "|"]
    for name, a in items:
        if a.get("runs"):
            c = a["max_concurrent_jobs_MEASURED"]
            out.append("| %s | %s |" % (name, " | ".join(str(c.get(k, 0)) for k in keys)))
    return "\n".join(out)


def main():
    D = json.load(open(os.path.join(HERE, "dataset.json"), encoding="utf-8"))
    notes = open(os.path.join(HERE, "report_template.txt"), encoding="utf-8").read()
    B = D["before"]
    P = D["phases_window"]
    groups = [("BEFORE 50-PR cohort", B)] + [("Window phase " + k, P[k]) for k in ("A", "B1", "B2")] + \
             [("AFTER all (since cutover)", D["after_all"])] + \
             [("AFTER " + k.replace("|", " / "), v) for k, v in D["after_by_line_phase"].items()]
    t = {}
    t["AGG"] = agg_rows(groups)
    t["LAT"] = lat_rows(groups)
    t["CONC"] = conc_rows(groups)
    t["BEFORE_PR"] = "| Metric | n | median | p90 | max |\n|---|---|---|---|---|\n" + "\n".join(
        "| %s | %s | %s | %s | %s |" % (k, B[key].get("n"), d(B[key]), d(B[key], "p90"), d(B[key], "max"))
        for k, key in [("Job-min per PR (final head) MEASURED", "per_pr_minutes_MEASURED"),
                       ("Ready-or-last-push to merge, min MEASURED", "pr_ready_or_push_to_merge_min_MEASURED"),
                       ("Final-head time to green (PR ci+repro), min MEASURED", "pr_time_to_green_min_MEASURED"),
                       ("First failing job completion after ci created, min MEASURED", "pr_first_failure_min_MEASURED")])
    t["AFTER_PR"] = "| PR | Base | Merged | Ready/push -> merge min | Time to green min | First failure min | Final-head job-min |\n|---|---|---|---|---|---|---|\n" + \
        "\n".join("| #%d | %s | %s | %s | %s | %s | %s |" % (
            x["pr"], x["base"], x["merged_at"], f(x["ready_or_push_to_merge_min"]),
            f(x["final_head_time_to_green_min"]), f(x["first_failure_min"]), f(x["final_head_minutes"]))
            for x in D["after_merged_prs"])
    t["ZERO"] = "| Line / phase | Zero-allocation PR runs | ci | repro | Placeholders skipped | PR still draft now | COUNTERFACTUAL lower (honesty+rest draft) min | COUNTERFACTUAL upper (full gate) min |\n|---|---|---|---|---|---|---|---|\n" + \
        "\n".join("| %s | %d | %d | %d | %d | %d | %s | %s |" % (k.replace("|", " / "), v["runs"], v["ci_runs"], v["repro_runs"], v["placeholders"],
                                                         v["pr_currently_draft"], f(v["lower_counterfactual_min_COUNTERFACTUAL"]),
                                                         f(v["counterfactual_min_COUNTERFACTUAL"]))
                  for k, v in sorted(D["zero_allocation_summary"].items()))
    rt = []
    for r in D["targeted_retries"]:
        rt.append("| %d | `%s` | %d | %d | %s | %s (%s) | %s (%s) | %s | %s |" % (
            r["run"], r["head_sha"][:8], r["final_attempt"], r["retained_jobs"], f(r["retained_min_MEASURED"], 3),
            f(r["lost_min_MEASURED"], 3), "; ".join("%s %s" % (n, c) for n, c, _ in r["lost_jobs"]),
            f(r["repeated_min_MEASURED"], 3), "; ".join("%s %s" % (n, c) for n, c, _ in r["repeated_jobs"]),
            f(r["total_executed_min_MEASURED"], 3), f(r["avoided_by_targeted_retry_min_COUNTERFACTUAL"], 3)))
    t["RETRY"] = "| Run | Head | Attempts | Retained jobs | Retained min MEASURED | Lost min MEASURED (jobs) | Repeated min MEASURED (jobs) | Total executed min MEASURED | Full-rerun minutes avoided COUNTERFACTUAL |\n|---|---|---|---|---|---|---|---|---|\n" + "\n".join(rt)
    t["OTHER_RETRY"] = "\n".join("- run %d (attempts %d): retained %d jobs / %s min, lost %s, repeated %s, total %s MEASURED" % (
        r["run"], r["final_attempt"], r["retained_jobs"], f(r["retained_min_MEASURED"], 3), f(r["lost_min_MEASURED"], 3),
        f(r["repeated_min_MEASURED"], 3), f(r["total_executed_min_MEASURED"], 3)) for r in D["other_multi_attempt_runs"]) or "- none"
    t["REUSE"] = "| Run | Line | Phase | Head | Created | Conclusion | plan min MEASURED | Skipped placeholders | COUNTERFACTUAL full-gate min |\n|---|---|---|---|---|---|---|---|---|\n" + \
        "\n".join("| %d | %s | %s | `%s` | %s | %s | %s | %d | %s |" % (
            r["run"], r["line"], r["phase"], r["head_sha"][:8], r["created_at"], r["conclusion"],
            f(r["plan_min_MEASURED"], 3), r["placeholders"], f(r["counterfactual_full_gate_min_COUNTERFACTUAL"]))
            for r in D["post_merge_plan_only_runs"])
    t["PUSHFULL"] = "\n".join("- run %d %s phase %s `%s` %s: %s job-min MEASURED" % (
        r["run"], r["line"], r["phase"], r["head_sha"][:8], r["conclusion"], f(r["minutes_MEASURED"], 3))
        for r in D["post_merge_full_gate_push_runs"]) or "- none"
    par = []
    for k, v in D["assurance_parity"].items():
        par.append("- **%s**: run %d `%s` %s/%s, %d executed + %d placeholders, %d distinct job names" % (
            k, v["run"], v["head_sha"][:8], v["status"], v["conclusion"], v["executed"], v["placeholders"], len(v["job_names"])))
    for x in D["assurance_parity_diffs"]:
        par.append("- %s -> %s: missing in target: %s; new in target: %s" % (
            x["from"], x["to"], ", ".join("`%s`" % n for n in x["missing_in_to"]) or "none",
            ", ".join("`%s`" % n for n in x["new_in_to"]) or "none"))
    par.append("")
    par.append("Union roster over every full-gate `ci` run, per phase and line (the A|v0.4 -> A|v0.5 difference above is a release-line content difference, not a phase loss):")
    for k, v in sorted(D["assurance_parity_union"].items()):
        if isinstance(v, dict) and "full_gate_runs" in v:
            par.append("- %s: %d full-gate runs, %d distinct job names" % (k, v["full_gate_runs"], v["distinct_names"]))
        elif isinstance(v, dict):
            par.append("- **%s**: missing in B: %s; new in B: %s" % (
                k, ", ".join("`%s`" % n for n in v["missing_in_B"]) or "none",
                ", ".join("`%s`" % n for n in v["new_in_B"]) or "none"))
        else:
            par.append("- **%s**: %s" % (k, v))
    t["PARITY"] = "\n".join(par)
    cb = D["counterfactual_basis"]
    t["CFBASIS"] = "full-gate median by phase/line: %s; all full gates median %s min; reproducibility median %s min; pre-guard partial draft ci (honesty + rest): %s" % (
        ", ".join("%s=%s" % (k, f(v)) for k, v in cb["full_gate_median_min_by_phase_line"].items()),
        f(cb["full_gate_median_min_all"]), f(cb["repro_median_min_all"]),
        "n=%s median %s" % (cb["pre_guard_partial_draft_ci_min"].get("n"), d(cb["pre_guard_partial_draft_ci_min"])))
    sp = D.get("post_merge_specific") or {}
    t["SPECIFIC"] = "run %s: plan %s min MEASURED; reused verified run %s whose final logical inventory is %s min (COUNTERFACTUAL had it been re-executed); the prior merge push 45421145394 (same mechanism failing on a retried run) executed %s min MEASURED" % (
        sp.get("run"), f(sp.get("plan_min_MEASURED"), 3), sp.get("reused_verified_run"),
        f(sp.get("counterfactual_rerun_of_verified_inventory_min_COUNTERFACTUAL"), 3),
        f(sp.get("observed_prior_merge_push_45421145394_min_MEASURED"), 3))
    t["BEFORE_TOTAL"] = f(B["minutes_MEASURED"], 3)
    t["BEFORE_DELTA"] = f(B["delta_vs_reference_min"], 3)
    t["BEFORE_FEATURES"] = ", ".join("%s: %d" % kv for kv in sorted(B["gate_feature_counts"].items()))
    t["AFTER_PR_COUNT"] = str(D["after_pr_count"])
    t["COLLECTED"] = D["collected_at"]
    t["WINDOW_TOTAL"] = str(D["window_run_total_reported"])
    t["GAPS"] = open(os.path.join(HERE, "gaps.txt"), encoding="utf-8").read()
    out = notes
    for k, v in t.items():
        out = out.replace("{{" + k + "}}", v)
    open(os.path.join(HERE, "report.md"), "w", encoding="utf-8").write(out)


if __name__ == "__main__":
    main()

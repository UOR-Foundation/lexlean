#!/usr/bin/env python3
"""Offline, auditable LexLean Actions before/after report (CI #603).

Inputs are read-only GitHub REST snapshots, not user-supplied estimates.
It does not invoke GitHub, mutate actions, determine branch protection,
or claim provider-billed costs. Python 3.9+ standard library.
"""
import argparse
import csv
import json
import math
import statistics
from datetime import datetime, timezone
from pathlib import Path
from xml.sax.saxutils import escape

RUN_SCHEMA = "lexlean/ci-pr-baseline/1"
COST_SCHEMA = "lexlean/ci-pr-job-snapshot/1"
OS = ("linux", "windows", "macos", "unknown")
# 2026 GitHub standard private-repository hosted runner rates. Historical
# Linux x64 and Linux ARM job-minutes are pooled; the x64 rate is a proxy.
# Actual GitHub billing rounds EACH job to a whole minute; not in this
# existing aggregate capture. This is explicitly an illustrative estimate.
PRIVATE_RATE_PROXY = {"linux": 0.006, "windows": 0.010, "macos": 0.062}
RATE_REFERENCE = "https://docs.github.com/en/billing/reference/actions-runner-pricing"


def require(condition, reason):
    if not condition:
        raise ValueError(reason)


def instant(value):
    require(isinstance(value, str), "missing timestamp")
    return datetime.fromisoformat(value.replace("Z", "+00:00")).astimezone(timezone.utc)


def load(path):
    file = Path(path)
    require(file.is_file() and not file.is_symlink() and file.stat().st_size <= 4 * 1024 * 1024,
            "missing, linked or oversized snapshot")
    return json.loads(file.read_text(encoding="utf-8"))


def pct(values, portion):
    if not values:
        return None
    values = sorted(values)
    idx = (len(values) - 1) * portion
    low = math.floor(idx)
    return round(values[low] + (values[min(low + 1, len(values) - 1)] - values[low]) *
                 (idx - low), 3)


def summarize(runs_data, costs_data, label):
    require(runs_data.get("schema") == RUN_SCHEMA, "incorrect run snapshot schema")
    require(costs_data.get("schema") == COST_SCHEMA, "incorrect cost snapshot schema")
    prs = runs_data.get("rows")
    costs = costs_data.get("run_costs")
    require(isinstance(prs, list) and 1 <= len(prs) <= 50, "expected 1-50 merged PRs")
    require(isinstance(costs, list), "run cost snapshot missing")
    by_id = {}
    for cost in costs:
        rid = cost.get("id")
        require(type(rid) is int and rid not in by_id, "missing/duplicate run cost ID")
        require(cost.get("complete") is True, "partial/unavailable run job inventory: " + str(rid))
        duration = cost.get("job_minutes")
        require(type(duration) in (float, int) and math.isfinite(duration) and duration >= 0,
                "invalid runner duration: " + str(rid))
        require(type(cost.get("job_count")) is int and cost["job_count"] >= 0,
                "invalid job count: " + str(rid))
        require(cost.get("job_count") == cost.get("returned"),
                "job list incomplete: " + str(rid))
        platforms = cost.get("by_platform_minutes")
        require(isinstance(platforms, dict) and set(platforms).issubset(OS),
                "invalid platforms: " + str(rid))
        for item in platforms.values():
            require(type(item) in (int, float) and math.isfinite(item) and item >= 0,
                    "invalid platform minutes")
        require(abs(sum(platforms.values()) - duration) <= 0.01,
                "job minutes and platform sum disagree: " + str(rid))
        by_id[rid] = cost
    seen_pr = set()
    seen_run = set()
    result = []
    for pr in prs:
        number = pr.get("number")
        require(type(number) is int and number > 0 and number not in seen_pr,
                "duplicate or invalid PR")
        seen_pr.add(number)
        merged = instant(pr.get("merged_at"))
        created = instant(pr.get("created_at"))
        require(created <= merged, "PR merged before creation")
        sha = pr.get("head_sha")
        require(isinstance(sha, str) and len(sha) == 40 and
                all(x in "0123456789abcdef" for x in sha), "invalid exact PR HEAD")
        run_list = pr.get("runs")
        require(isinstance(run_list, list) and pr.get("run_list_complete") is True,
                "unavailable PR head run inventory")
        require(pr.get("run_total_reported") == len(run_list),
                "truncated exact-head run inventory")
        minutes = 0.0
        by_platform = dict.fromkeys(OS, 0.0)
        qualification = []
        for r in run_list:
            rid = r.get("id")
            require(type(rid) is int and rid not in seen_run, "run appears more than once")
            seen_run.add(rid)
            require(rid in by_id and r.get("head_sha") == sha,
                    "missing or foreign exact-head run cost: " + str(rid))
            rcost = by_id[rid]
            require(rcost.get("pr") == number, "job cost attributed to different PR")
            minutes += rcost["job_minutes"]
            for os_name, value in rcost["by_platform_minutes"].items():
                by_platform[os_name] += value
            if r.get("name") in ("ci", "reproducibility") and r.get("event") == "pull_request":
                qualification.append(r)
        before_merge = [r for r in qualification
                        if r.get("conclusion") == "success" and instant(r["updated_at"]) <= merged]
        # This is a CORRELATION in public provider data, not proof of branch
        # protection or that a check actually held the merge.
        matches = {}
        for name in ("ci", "reproducibility"):
            matching = [r for r in before_merge if r.get("name") == name]
            if matching:
                matches[name] = min(matching, key=lambda r: instant(r["updated_at"]))
        green = max((instant(r["updated_at"]) for r in matches.values()),
                    default=None) if len(matches) == 2 else None
        first = [instant(r["created_at"]) for r in qualification
                 if r.get("conclusion") != "skipped"]
        first = min(first) if first else None
        wait = round((green - first).total_seconds() / 60, 3) if green and first else None
        require(wait is None or wait >= 0, "green before selected run creation")
        result.append({
            "period": label, "pr": number, "base": pr.get("base_branch"),
            "head_sha": sha, "merged_at": merged.isoformat(),
            "workflow_runs": len(run_list),
            "observed_job_minutes": round(minutes, 3),
            "estimated_private_equivalent_usd": round(sum(
                by_platform.get(os_name, 0.0) * rate
                for os_name, rate in PRIVATE_RATE_PROXY.items()), 3),
            "by_platform_minutes": {k: round(v, 3) for k, v in by_platform.items()},
            "verified_premerge_green": green is not None,
            "final_head_time_to_green_minutes": wait,
            "merge_after_green_minutes": round((merged-green).total_seconds()/60, 3)
            if green else None,
        })
    require(seen_run == set(by_id), "cost rows are not a bijection of source runs")
    result.sort(key=lambda x: (x["merged_at"], x["pr"]))
    usage = [p["observed_job_minutes"] for p in result]
    shadow_usd = [p["estimated_private_equivalent_usd"] for p in result]
    waits = [p["final_head_time_to_green_minutes"] for p in result
             if p["final_head_time_to_green_minutes"] is not None]
    return result, {
        "pr_count": len(result), "workflow_run_count": len(seen_run),
        "pr_with_premerge_green": len(waits),
        "pr_without_proven_premerge_green": len(result)-len(waits),
        "observed_runner_job_minutes_total": round(sum(usage), 2),
        "observed_runner_job_minutes_median": pct(usage, .5),
        "observed_runner_job_minutes_p90": pct(usage, .9),
        "estimated_private_equivalent_usd_total": round(sum(shadow_usd), 2),
        "estimated_private_equivalent_usd_median": pct(shadow_usd, .5),
        "estimated_private_equivalent_usd_p90": pct(shadow_usd, .9),
        "private_equivalent_model": "ILLUSTRATIVE_LINUX_X64_PROXY_UNROUNDED_BEFORE_ALLOWANCE_STORAGE_EXCLUDED",
        "private_equivalent_rate_reference": RATE_REFERENCE,
        "final_head_time_to_green_minutes_median": pct(waits, .5),
        "final_head_time_to_green_minutes_p90": pct(waits, .9),
        "platform_runner_minutes": {
            name: round(sum(p["by_platform_minutes"][name] for p in result), 2)
            for name in OS
        },
        "billing_minutes": None,
        "billing_data_status": "NOT_ESTABLISHED_BY_JOB_TIMINGS",
        "scope": "FINAL_HEAD_ONLY; earlier PR revisions, postmerge pushes and organization spend not counted",
    }


def svg_report(before, after, field, output, title, units):
    """Two side-by-side panels, both on the same scale; no after data -> N/A."""
    values = [float(p[field]) for p in before + after if p[field] is not None]
    ymax = max(values or [1.0])
    ymax = math.ceil(ymax / 5) * 5 if ymax < 100 else math.ceil(ymax / 100) * 100
    width, height, top, bottom = 1650, 600, 108, 490
    panel_width = 685
    panels = [("BEFORE", before, 105), ("AFTER", after, 860)]
    lines = [
        '<?xml version="1.0" encoding="UTF-8"?>',
        '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1650 600" role="img">',
        '<rect width="1650" height="600" fill="#fff"/>',
        '<style>text{font-family:Arial,Helvetica,sans-serif;fill:#252525}'
        '.head{font-weight:700;font-size:23px}.label{font-weight:700;font-size:18px}'
        '.small{font-size:12px;fill:#666}.unit{font-size:14px;fill:#555}</style>',
        '<text x="65" y="43" class="head">' + escape(title) + '</text>',
        '<text x="65" y="69" class="unit">' + escape(units) +
        ' • GitHub job timestamps • Private equivalent is estimated, not actual billed spend</text>',
    ]
    for label, rows, start in panels:
        lines.append(f'<text x="{start}" y="101" class="label">{label}: {len(rows)} PRs</text>')
        lines.append(f'<line x1="{start}" y1="{bottom}" x2="{start+panel_width}" '
                     f'y2="{bottom}" stroke="#666" stroke-width="1"/>')
        for i in range(5):
            y = bottom - (bottom-top) * i/4
            lines.append(f'<line x1="{start}" y1="{y:.1f}" x2="{start+panel_width}" '
                         f'y2="{y:.1f}" stroke="#e0e0e0"/>')
            if label == "BEFORE":
                lines.append(f'<text x="{start-12}" y="{y+4:.1f}" text-anchor="end" '
                             f'class="small">{ymax*i/4:.0f}</text>')
        if not rows:
            lines.append(f'<rect x="{start+8}" y="{top+10}" width="{panel_width-16}" '
                         f'height="{bottom-top-12}" fill="#f5f5f5" stroke="#aaa" '
                         'stroke-dasharray="7 5"/>')
            lines.append(f'<text x="{start+panel_width/2}" y="280" text-anchor="middle" '
                         'class="label">AWAITING VERIFIED POST-CUTOVER PRS</text>')
            lines.append(f'<text x="{start+panel_width/2}" y="306" text-anchor="middle" '
                         'class="small">No zero-minute runs or estimated savings have been inserted.</text>')
            continue
        step = panel_width/50
        for i, row in enumerate(rows):
            value = row[field]
            if value is None:
                if field == "final_head_time_to_green_minutes":
                    x = start+(i+.5)*step
                    lines.append(f'<path d="M{x-3:.1f},{bottom-7} l6,6 m-6,0 l6,-6" '
                                 'stroke="#888" stroke-width="1.3"/>')
                continue
            h = (bottom-top)*(float(value)/max(ymax,1))
            x = start + i*step + 1.5
            lines.append(f'<rect x="{x:.1f}" y="{bottom-h:.1f}" '
                         f'width="{max(2,step-3):.1f}" height="{h:.1f}" '
                         f'fill="{"#4c6273" if label=="BEFORE" else "#6c8578"}"/>')
            if i % 10 == 0:
                lines.append(f'<text x="{x:.1f}" y="{bottom+20}" class="small">'
                             f'#{row["pr"]}</text>')
        visible = [x[field] for x in rows if x[field] is not None]
        lines.append(f'<text x="{start}" y="555" class="unit">Measured: {len(visible)}/{len(rows)}'
                     f' • Median: {pct(visible,.5) if visible else "N/A"}'
                     f' • p90: {pct(visible,.9) if visible else "N/A"}</text>')
    lines.append('</svg>')
    output.write_text("\n".join(lines)+"\n", encoding="utf-8")


def write_report(before, before_info, after, after_info, out):
    out.mkdir(parents=True, exist_ok=True)
    for rows, label in ((before, "before"), (after, "after")):
        pass
    fields = ("period", "pr", "base", "head_sha", "merged_at", "workflow_runs",
              "observed_job_minutes", "estimated_private_equivalent_usd",
              "linux_job_minutes", "windows_job_minutes",
              "macos_job_minutes", "unknown_job_minutes", "verified_premerge_green",
              "final_head_time_to_green_minutes", "merge_after_green_minutes")
    with (out/"ci-before-after-prs.csv").open("w", encoding="utf-8", newline="") as fh:
        writer = csv.DictWriter(fh, fieldnames=fields)
        writer.writeheader()
        for row in before+after:
            data = {k:v for k,v in row.items() if k in fields}
            for os_name, minutes in row["by_platform_minutes"].items():
                data[os_name+"_job_minutes"] = minutes
            writer.writerow(data)
    svg_report(before, after, "observed_job_minutes", out/"runner-minutes-per-pr.svg",
               "LexLean Actions: observed job consumption per merged PR", "Unweighted runner-job minutes")
    svg_report(before, after, "estimated_private_equivalent_usd",
               out/"private-equivalent-cost-per-pr.svg",
               "LexLean Actions: estimated private-repository compute cost per PR",
               "USD, 2026 standard runner rate proxy; not actual billing")
    svg_report(before, after, "final_head_time_to_green_minutes",
               out/"final-head-time-to-green.svg",
               "LexLean CI: final-HEAD time to pre-merge green",
               "Minutes from first non-skipped final-HEAD CI to both successful required workflows")
    result = {
        "schema": "lexlean/ci-before-after-report/1", "before": before_info,
        "after": after_info, "cutover_effect": None,
        "cutover_effect_status": "NOT_MEASURED" if not after else
        "OBSERVATIONAL_ONLY; controlled cohort matching required",
        "notes": [
            "Not GitHub-billed Actions minutes: timing API may return zero for public projects.",
            "Observed job-minutes are unweighted; private equivalence applies 2026 OS rates but is not provider billing.",
            "Linux ARM/x64 combined and per-job minute rounding unavailable: private equivalent is illustrative, not an invoice.",
            "Private compute equivalent excludes included monthly usage and storage.",
            "Latest exact PR HEAD only; earlier revisions and postmerge workflow runs are omitted.",
            "Time-to-green is a correlation from workflow timestamps, not proven required-check blocking.",
            "Missing premerge green is N/A, not zero minutes.",
            "Historical workflow designs, base branches, and change classes may differ from cutover PRs.",
        ],
    }
    (out/"ci-before-after-summary.json").write_text(
        json.dumps(result, indent=2, sort_keys=True)+"\n", encoding="utf-8")
    lines = [
        "# LexLean CI before/after: observed, not projected",
        "",
        "## Baseline and after",
        "",
        "| Measure | Before | After |",
        "| --- | ---: | ---: |",
    ]
    for key, display in [
        ("pr_count", "Merged PRs"),
        ("workflow_run_count", "Final-HEAD workflow runs"),
        ("observed_runner_job_minutes_total", "Observed total runner-job minutes"),
        ("observed_runner_job_minutes_median", "Median runner-job minutes/PR"),
        ("observed_runner_job_minutes_p90", "p90 runner-job minutes/PR"),
        ("estimated_private_equivalent_usd_total", "Estimated private-repo compute equivalent total (USD)"),
        ("estimated_private_equivalent_usd_median", "Median estimated private-repo cost per PR (USD)"),
        ("pr_with_premerge_green", "PRs with both CI and reproducibility green before merge"),
        ("final_head_time_to_green_minutes_median", "Median time-to-green (only qualifying samples)"),
        ("final_head_time_to_green_minutes_p90", "p90 time-to-green (only qualifying samples)"),
    ]:
        a = before_info[key]
        b = after_info[key] if after_info else "NOT YET OBSERVED"
        lines.append(f"| {display} | {a} | {b} |")
    lines += [
        "", "![Per-PR runner workload](runner-minutes-per-pr.svg)", "",
        "![Private-repository compute equivalent](private-equivalent-cost-per-pr.svg)", "",
        "![Final HEAD time to green](final-head-time-to-green.svg)", "",
        "## Measurement boundaries", "",
        "- All values are from GitHub job/run timestamps at exact final PR HEAD.",
        "- **Not provider billed minutes**: billing data is unavailable in these snapshots;",
        "  zero billable minutes is not zero runner consumption.",
        "- Estimated private compute equivalent applies Linux $0.006, Windows $0.010, macOS",
        "  $0.062 per elapsed runner-job minute, before allowances, with Linux ARM pooled with x64.",
        "  Actual private charges would round each job to a whole minute; this historical capture",
        "  cannot reconstruct that rounding. No private invoice or measured savings are claimed.",
        "- Pricing source: " + RATE_REFERENCE,
        "- The baseline is the most recent 50 merged PRs in the captured pre-cutover window.",
        "- The post-cutover cohort is **unpopulated** until real merged PRs exist.",
        "- Completion before merge is not proof branch protection actually required the checks.",
        "- PRs with missing premerge green are excluded from wait percentiles and counted explicitly.",
        "- The current source-backed snapshot misses earlier PR revisions and postmerge pushes,",
        "  so the runner-minute total is an attributable observed scope, not entire organization spend.",
        "- Compare matched PR change classes, base/release lines, workflow versions and runner types",
        "  before declaring causal savings; an early after sample is provisional.",
        "- Full production qualification remains under issue #603; a green preview is nonqualifying.",
    ]
    (out/"README.md").write_text("\n".join(lines)+"\n", encoding="utf-8")
    return result


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--before-runs", type=Path, required=True)
    parser.add_argument("--before-costs", type=Path, required=True)
    parser.add_argument("--after-runs", type=Path)
    parser.add_argument("--after-costs", type=Path)
    parser.add_argument("--cutover-utc", help="Required exact ISO time when after data is provided")
    parser.add_argument("--output-dir", type=Path, required=True)
    args = parser.parse_args(argv)
    require((args.after_runs is None) == (args.after_costs is None),
            "after run and job snapshots must appear together")
    before, b_summary = summarize(load(args.before_runs), load(args.before_costs), "before")
    after, a_summary = [], None
    if args.after_runs:
        require(args.cutover_utc is not None, "cutover instant required for after data")
        boundary = instant(args.cutover_utc)
        after, a_summary = summarize(load(args.after_runs), load(args.after_costs), "after")
        require(all(instant(x["merged_at"]) < boundary for x in before),
                "before sample crosses cutover boundary")
        require(all(instant(x["merged_at"]) >= boundary for x in after),
                "after sample predates cutover boundary")
    result = write_report(before, b_summary, after, a_summary, args.output_dir)
    print(json.dumps({"before": b_summary, "after": a_summary,
                      "effect_status": result["cutover_effect_status"]}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

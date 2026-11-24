#!/usr/bin/env python3
"""DOD-7a: derive baseline.json from raw_parsed.json (parse_logs.py) plus a
second pass over the logs for cross-job timing (bootstrap export -> BS legs).

Usage: python -I derive.py <dod7a_dir>
"""
import json
import os
import sys
from datetime import datetime, timezone

here = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, here)
import parse_logs  # noqa: E402

base = sys.argv[1]
raw = json.load(open(os.path.join(base, "raw_parsed.json"), encoding="utf-8"))
RUNS = {"45437716141": {"pr": 620, "head": "603336bf"}, "45316289312": {"pr": 619, "head": "bd80fa37"}}
SPECS = {  # GitHub docs, standard hosted runners, public repository (repo visibility: public)
    "ubuntu-24.04": {"vcpu": 4, "ram_gb": 16, "arch": "x64"},
    "ubuntu-24.04-arm": {"vcpu": 4, "ram_gb": 16, "arch": "arm64"},
    "windows-2022": {"vcpu": 4, "ram_gb": 16, "arch": "x64"},
    "macos-15-intel": {"vcpu": 4, "ram_gb": 14, "arch": "x64 (Intel)"},
    "macos-15": {"vcpu": 3, "ram_gb": 7, "arch": "arm64 (M1)"},
}


def iso(s):
    return datetime.strptime(s, "%Y-%m-%dT%H:%M:%SZ").replace(tzinfo=timezone.utc).timestamp()


out = {"source": "GitHub Actions job logs + jobs API, UOR-Foundation/lexlean", "runs": {},
       "host_specs_public_repo": SPECS,
       "host_specs_citation": "https://docs.github.com/en/actions/reference/runners/github-hosted-runners (fetched 2026-11-24)",
       "labels": "MEASURED = runner timestamps / cargo / libtest / ci-summary lines; ESTIMATED = inferred (see parse_logs.py docstring)"}

for run, jobs in raw["runs"].items():
    r = {"meta": RUNS[run], "jobs": []}
    byname = {j["name"]: j for j in jobs}
    starts = [iso(j["api_started"]) for j in jobs]
    ends = [iso(j["api_completed"]) for j in jobs]
    r["run_wall_s"] = {"value": max(ends) - min(starts), "label": "MEASURED (jobs API first start -> last completion)"}
    r["job_minutes_sum"] = {"value": round(sum(j["api_duration_s"] for j in jobs) / 60, 1), "label": "MEASURED"}
    # compile across the run
    r["cargo_finished_sum_s"] = {"value": round(sum(j["cargo_compile_reported_s"] for j in jobs), 1),
                                 "label": "MEASURED (sum of cargo 'Finished ... in' over all jobs)"}
    r["setup_sum_s"] = {"value": round(sum(v for j in jobs for k, v in j["step_split_s"].items() if k.startswith("setup")), 1),
                        "label": "MEASURED (checkout+toolchain+elan+swap+artifact download steps)"}
    r["libtest_sum_s"] = {"value": round(sum(j["test_exec_s_sum_libtest"] for j in jobs), 1),
                          "label": "MEASURED (sum of libtest 'finished in' over all binaries/jobs)"}
    # bootstrap export -> BS legs
    exp = byname.get("bootstrap export")
    exp_end = iso(exp["api_completed"]) if exp else None
    legs = []
    for j in jobs:
        if j["name"].startswith("just vv (bdd-bootstrap-"):
            log = parse_logs.parse_log(os.path.join(base, "logs", run, j["job_id"] + ".log"))
            conf = [b for b in log["binaries"] if "conformance" in b["name"] and b["n_tests"] > 0]
            if not conf:
                continue
            b = conf[0]
            legs.append({"job_id": j["job_id"], "leg": j["name"], "cases": list(j["ci_summary_cases_s"].items()),
                         "test_start_utc": datetime.fromtimestamp(b["t_running"], timezone.utc).isoformat(),
                         "test_end_utc": datetime.fromtimestamp(b["t_result"], timezone.utc).isoformat(),
                         "case_wall_s": b["libtest_finished_s"],
                         "test_end_minus_export_job_end_s": round(b["t_result"] - exp_end, 1),
                         "test_start_minus_export_job_end_s": round(b["t_running"] - exp_end, 1),
                         "cargo_finished_s": j["cargo_compile_reported_s"], "job_s": j["api_duration_s"]})
    r["bootstrap_export_job"] = {"job_id": exp["job_id"], "job_s": exp["api_duration_s"], "completed": exp["api_completed"]} if exp else None
    r["bdd_bootstrap_legs"] = legs
    for j in jobs:
        j2 = dict(j)
        j2["host_spec"] = SPECS.get(j["labels"].split(",")[0])
        r["jobs"].append(j2)
    out["runs"][run] = r

json.dump(out, open(os.path.join(base, "baseline.json"), "w", encoding="utf-8"), indent=1)
for run, r in out["runs"].items():
    print(run, {k: r[k]["value"] for k in ("run_wall_s", "job_minutes_sum", "cargo_finished_sum_s", "setup_sum_s", "libtest_sum_s")})
    print(" export", r["bootstrap_export_job"])
    for l in r["bdd_bootstrap_legs"]:
        print("  ", l["job_id"], l["leg"][9:], l["cases"], "wall", l["case_wall_s"], "start-exp", l["test_start_minus_export_job_end_s"], "end-exp", l["test_end_minus_export_job_end_s"])

#!/usr/bin/env python3
"""DOD-7a baseline: parse GitHub Actions job logs (ANSI-stripped) of lexlean CI runs.

Usage: python -I parse_logs.py <out.json> <run_id>:<jobs.tsv>:<logdir> [...]

Every number this script emits is derived from runner timestamps (the ISO
prefix GitHub writes on every log line) or from lines cargo/libtest/ci-summary
print. Labels:
  MEASURED  - read directly (timestamp deltas between boundary lines, cargo's
              own `Finished ... in`, libtest's `finished in`, ci-summary rows).
  ESTIMATED - inferred (per-test durations from completion order; see below).

Per-test estimation. libtest prints a test's line when it completes and
"has been running for over 60 seconds" 60 s after it starts. It starts tests in
name order as slots free (concurrency = --test-threads or
available_parallelism). We (a) recover each test's libtest start by replaying
that slot model with the observed completion order, choosing the thread count
that best matches the 60 s warnings, and (b) because every `BS` case takes the
process-wide HEAVY mutex (crates/conformance/src/cases/bootstrap.rs), a BS
case's execution starts no earlier than the previous BS completion in the same
process; its execution estimate is ok_ts - max(libtest_start, prev_BS_ok).
"""
import json
import re
import sys
from datetime import datetime, timezone

TS = re.compile(r"^\ufeff?(\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d\.\d+)Z ?(.*)$")
RUNNING = re.compile(r"^\s*Running (unittests )?(\S+)(?: \((\S+)\))?")
DOCTEST = re.compile(r"^\s*Doc-tests (\S+)")
NTESTS = re.compile(r"^running (\d+) tests?$")
RESULT = re.compile(r"^test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out; finished in ([\d.]+)s")
TESTLINE = re.compile(r"^test (\S+) \.\.\. (ok|ignored|FAILED)")
WARN = re.compile(r"^test (\S+) has been running for over 60 seconds")
FINISHED = re.compile(r"^\s*Finished `(\w+)` profile \[([^\]]+)\] target\(s\) in (?:(\d+)m )?([\d.]+)s")
COMPILING = re.compile(r"^\s*Compiling (\S+) v")
JUSTTS = re.compile(r"^\[(\d+\.\d+)\] (.+)$")
MEM = re.compile(r"^Mem:\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)")
SUMROW = re.compile(r"^\| ([A-Z]{2,3}-\d+) \| ([\d.]+) \| (true|false) \|$")
RECIPEROW = re.compile(r"^\| `([^`]+)` \| (\d+) \| ([\d.]+) \| (\d*) \| (\d*) \| (\d*) \|$")


def ts(s):
    return datetime.strptime(s[:26], "%Y-%m-%dT%H:%M:%S.%f").replace(tzinfo=timezone.utc).timestamp()


def classify(step):
    s = step.lower()
    # the gate's own command wins over anything its script also mentions
    if any(k in s for k in ("cargo xtask", "just --timestamp", "intel_host_tests.py", "host_lanes.py",
                            "cargo test", "target/debug/xtask", "ci-parts/atlas-xtask/xtask verify")):
        return "post:ci-summary" if "ci-summary" in s.split(chr(10))[0] else "work"
    if "checkout" in s:
        return "setup:checkout"
    if "elan-init" in s or "toolchain install" in s and "elan" in s:
        return "setup:lean-toolchain"
    if "fallocate" in s or "mkswap" in s:
        return "setup:swap"
    if "rustup" in s or "install-action" in s:
        return "setup:rust-toolchain"
    if "download-artifact" in s:
        return "setup:artifact-download"
    if "upload-artifact" in s:
        return "post:artifact-upload"
    if "ci-summary" in s:
        return "post:ci-summary"
    if "rustc -vv" in s or "host this leg" in s or "bail()" in s:
        return "setup:host-check"
    return "work"


def slot_model(order, completions, t0, n):
    """libtest start time of every test under concurrency n."""
    start = {}
    queue = list(order)
    for name in queue[:n]:
        start[name] = t0
    nxt = n
    for _, (t, _name) in enumerate(completions):
        if nxt < len(queue):
            start[queue[nxt]] = t
            nxt += 1
    return start


def analyse_binary(b):
    tests = b["tests"]
    comp = sorted(((t["end"], name) for name, t in tests.items() if "end" in t))
    if not comp:
        return
    order = sorted(tests)  # libtest runs in name order
    warned = {n: t["warn"] - 60.0 for n, t in tests.items() if "warn" in t}
    best = None
    for n in range(1, 9):
        st = slot_model(order, comp, b["t_running"], n)
        if warned:
            err = sum(abs(st.get(k, 1e18) - v) for k, v in warned.items()) / len(warned)
        else:
            err = 0.0
        if best is None or err < best[1] - 1e-6:
            best = (n, err, st)
    n, err, st = best
    b["inferred_test_threads"] = n if warned else None
    b["slot_model_mean_abs_err_s"] = round(err, 2) if warned else None
    prev_bs = None
    for end, name in comp:
        t = tests[name]
        s = st.get(name, b["t_running"])
        t["libtest_start_est"] = s
        t["libtest_wall_est_s"] = round(end - s, 2)
        if name.startswith("conformance_bs_"):
            exec_start = s if prev_bs is None else max(s, prev_bs)
            t["bs_exec_est_s"] = round(end - exec_start, 2)
            prev_bs = end
    bs = [(tests[n]["end"], n) for n in tests if n.startswith("conformance_bs_") and "end" in tests[n]]
    if bs:
        first_start = min(tests[n]["libtest_start_est"] for _, n in bs)
        b["bs_chain"] = {"cases": len(bs), "first_start": first_start, "last_end": max(bs)[0],
                         "wall_s": round(max(bs)[0] - first_start, 1),
                         "sum_exec_est_s": round(sum(tests[n]["bs_exec_est_s"] for _, n in bs), 1),
                         "start_offset_s": round(first_start - b["t_running"], 1),
                         "end_offset_s": round(max(bs)[0] - b["t_running"], 1)}
    nonbs = [tests[n]["libtest_wall_est_s"] for n in tests if not n.startswith("conformance_bs_") and "end" in tests[n]]
    b["non_bs"] = {"tests": len(nonbs), "sum_wall_est_s": round(sum(nonbs), 1), "max_wall_est_s": round(max(nonbs), 1) if nonbs else 0,
                   "over_60s": sum(1 for x in nonbs if x > 60), "sum_over_60s_est_s": round(sum(x for x in nonbs if x > 60), 1)}
    # timestamps all identical => output was buffered and arrived at process end
    ends = [e for e, _ in comp]
    b["per_test_timestamps_usable"] = (max(ends) - min(ends)) > 1.0 or len(ends) < 3


def parse_log(path):
    lines = []
    with open(path, encoding="utf-8", errors="replace") as f:
        for raw in f:
            m = TS.match(raw.rstrip("\n"))
            if m:
                lines.append((ts(m.group(1)), m.group(2)))
    job = {"header": {}, "steps": [], "cargo_finished": [], "binaries": [], "recipes": [],
           "mem": {"total_mib": None, "min_available_mib": None}, "ci_summary_cases": {},
           "ci_summary_recipes": [], "compiling_lines": 0, "first_compiling": None,
           "last_compiling": None}
    if not lines:
        return job
    job["t_first"], job["t_last"] = lines[0][0], lines[-1][0]
    group = None
    cur_step = None
    pending_name = None
    cur_bin = None
    for t, text in lines:
        if text.startswith("##[group]"):
            group = text[len("##[group]"):]
            if group.startswith("Run "):
                if cur_step:
                    cur_step["end"] = t
                cur_step = {"name": group[4:][:160], "start": t, "kind": None, "body": []}
                job["steps"].append(cur_step)
            continue
        if text.startswith("##[endgroup]"):
            if group and group.startswith("Run ") and cur_step and cur_step["kind"] is None:
                cur_step["kind"] = classify(cur_step["name"] + chr(10) + chr(10).join(cur_step["body"][:12]))
            group = None
            continue
        if group and group.startswith("Run ") and cur_step is not None and cur_step["kind"] is None:
            cur_step["body"].append(text)
        if group in ("Runner Image", "Operating System", "Runner Image Provisioner"):
            if ":" in text:
                k, v = text.split(":", 1)
                job["header"].setdefault(group + "/" + k.strip(), v.strip())
            else:
                job["header"].setdefault(group + "/line", text.strip())
        if text.startswith("Post job cleanup") or text.startswith("Cleaning up orphan processes"):
            if cur_step and "end" not in cur_step:
                cur_step["end"] = t
        m = COMPILING.match(text)
        if m:
            job["compiling_lines"] += 1
            job["first_compiling"] = job["first_compiling"] or t
            job["last_compiling"] = t
        m = FINISHED.match(text)
        if m:
            secs = float(m.group(4)) + 60 * int(m.group(3) or 0)
            job["cargo_finished"].append({"t": t, "profile": m.group(1), "seconds": secs,
                                          "step": cur_step["name"] if cur_step else None})
        m = JUSTTS.match(text)
        if m and not text.startswith("[INFO"):
            job["recipes"].append({"epoch": float(m.group(1)), "t": t, "text": m.group(2)[:120]})
        m = MEM.match(text)
        if m:
            tot, avail = int(m.group(1)), int(m.group(6))
            job["mem"]["total_mib"] = tot
            mn = job["mem"]["min_available_mib"]
            job["mem"]["min_available_mib"] = avail if mn is None else min(mn, avail)
        m = SUMROW.match(text)
        if m:
            job["ci_summary_cases"][m.group(1)] = float(m.group(2))
        m = RECIPEROW.match(text)
        if m:
            job["ci_summary_recipes"].append({"recipe": m.group(1), "seconds": int(m.group(2)),
                                              "low_free_mib": m.group(4), "max_swap_mib": m.group(5),
                                              "largest_proc_mib": m.group(6)})
        m = RUNNING.match(text)
        if m and "target/debug/xtask" not in text and "`" not in text:
            pending_name = m.group(2)
            pending_t = t
            continue
        m = DOCTEST.match(text)
        if m:
            pending_name = "Doc-tests " + m.group(1)
            pending_t = t
            continue
        m = NTESTS.match(text)
        if m:
            cur_bin = {"name": pending_name or "(unnamed)", "t_announced": pending_t if pending_name else t,
                       "t_running": t, "n_tests": int(m.group(1)), "tests": {},
                       "step": cur_step["name"] if cur_step else None}
            pending_name = None
            continue
        if cur_bin is not None:
            m = TESTLINE.match(text)
            if m:
                cur_bin["tests"].setdefault(m.group(1), {})["end"] = t
                cur_bin["tests"][m.group(1)]["status"] = m.group(2)
                continue
            m = WARN.match(text)
            if m:
                cur_bin["tests"].setdefault(m.group(1), {})["warn"] = t
                continue
            m = RESULT.match(text)
            if m:
                cur_bin.update({"t_result": t, "status": m.group(1), "passed": int(m.group(2)),
                                "failed": int(m.group(3)), "ignored": int(m.group(4)),
                                "filtered": int(m.group(6)), "libtest_finished_s": float(m.group(7)),
                                "wall_s": round(t - cur_bin["t_announced"], 2)})
                analyse_binary(cur_bin)
                job["binaries"].append(cur_bin)
                cur_bin = None
    if cur_step and "end" not in cur_step:
        cur_step["end"] = lines[-1][0]
    for s in job["steps"]:
        s["seconds"] = round(s["end"] - s["start"], 1)
        if s["kind"] is None:
            s["kind"] = classify(s["name"] + chr(10) + chr(10).join(s["body"][:12]))
        s["compile_s"] = round(sum(c["seconds"] for c in job["cargo_finished"] if s["start"] <= c["t"] <= s["end"]), 1)
        s["test_s"] = round(sum(b.get("libtest_finished_s") or 0 for b in job["binaries"] if s["start"] <= b["t_running"] <= s["end"]), 1)
    return job


def summarize(job, meta):
    out = {"job_id": meta["id"], "name": meta["name"], "run_id": meta["run"], "conclusion": meta["conclusion"],
           "labels": meta["labels"], "runner_name": meta["runner"],
           "api_started": meta["started"], "api_completed": meta["completed"],
           "image": job["header"].get("Runner Image/Image"),
           "image_version": job["header"].get("Runner Image/Version"),
           "provisioner_cloud": job["header"].get("Runner Image Provisioner/Cloud"),
           "mem_total_mib_from_free": job["mem"]["total_mib"],
           "mem_min_available_mib_from_free": job["mem"]["min_available_mib"],
           "log_wall_s": round(job.get("t_last", 0) - job.get("t_first", 0), 1)}
    split = {}
    for s in job["steps"]:
        split[s["kind"]] = round(split.get(s["kind"], 0) + s["seconds"], 1)
    out["step_split_s"] = split
    out["steps"] = [{"name": s["name"], "kind": s["kind"], "seconds": s["seconds"], "cargo_finished_s": s["compile_s"],
                     "libtest_s": s["test_s"], "other_s": round(s["seconds"] - s["compile_s"] - s["test_s"], 1)} for s in job["steps"]]
    w = [s for s in job["steps"] if s["kind"] == "work"]
    out["work_split_s"] = {"cargo_finished": round(sum(s["compile_s"] for s in w), 1),
                           "libtest": round(sum(s["test_s"] for s in w), 1),
                           "other": round(sum(s["seconds"] - s["compile_s"] - s["test_s"] for s in w), 1)}
    out["cargo_finished"] = [{"profile": c["profile"], "seconds": c["seconds"]} for c in job["cargo_finished"]]
    out["cargo_compile_reported_s"] = round(sum(c["seconds"] for c in job["cargo_finished"]), 1)
    out["compiling_crates_lines"] = job["compiling_lines"]
    if job["first_compiling"]:
        out["compile_window_s"] = round(job["last_compiling"] - job["first_compiling"], 1)
    bins = []
    for b in job["binaries"]:
        rec = {k: b.get(k) for k in ("name", "n_tests", "passed", "failed", "ignored", "filtered",
                                      "libtest_finished_s", "wall_s", "inferred_test_threads",
                                      "slot_model_mean_abs_err_s", "per_test_timestamps_usable")}
        if "bs_chain" in b:
            rec["bs_chain"] = {k: v for k, v in b["bs_chain"].items() if k not in ("first_start", "last_end")}
        if b.get("per_test_timestamps_usable") and "non_bs" in b:
            rec["non_bs"] = b["non_bs"]
        top = sorted(((t.get("bs_exec_est_s", t.get("libtest_wall_est_s", 0)), n, t) for n, t in b["tests"].items()
                      if "end" in t), reverse=True)[:25]
        if b.get("per_test_timestamps_usable"):
            rec["slowest_tests_est"] = [{"test": n, "est_s": s,
                                         "libtest_wall_est_s": t.get("libtest_wall_est_s"),
                                         "bs_exec_est_s": t.get("bs_exec_est_s"),
                                         "warned_60s": "warn" in t} for s, n, t in top]
            # sum of per-test libtest walls / binary wall = average busy libtest threads
            tot = sum(t.get("libtest_wall_est_s", 0) for t in b["tests"].values())
            rec["avg_concurrent_tests_est"] = round(tot / b["libtest_finished_s"], 2) if b["libtest_finished_s"] else None
        bins.append(rec)
    out["test_binaries"] = bins
    out["test_exec_s_sum_libtest"] = round(sum(b.get("libtest_finished_s") or 0 for b in job["binaries"]), 1)
    out["ci_summary_cases_s"] = job["ci_summary_cases"]
    out["ci_summary_recipes"] = job["ci_summary_recipes"]
    rec = job["recipes"]
    out["just_recipes"] = [{"recipe": r["text"], "start_epoch": r["epoch"]} for r in rec]
    return out


def main():
    out_path = sys.argv[1]
    result = {"generated_by": "parse_logs.py", "runs": {}}
    for spec in sys.argv[2:]:
        run, tsv, logdir = spec.split("|")
        jobs = []
        with open(tsv, encoding="utf-8") as f:
            for row in f:
                p = row.rstrip("\n").split("\t")
                meta = {"id": p[0], "name": p[1], "conclusion": p[2], "started": p[3], "completed": p[4],
                        "labels": p[5], "runner": p[6] if len(p) > 6 else "", "run": run}
                job = parse_log(f"{logdir}/{p[0]}.log")
                s = summarize(job, meta)
                a, b = (datetime.strptime(meta[k], "%Y-%m-%dT%H:%M:%SZ").timestamp() for k in ("started", "completed"))
                s["api_duration_s"] = b - a
                jobs.append(s)
        jobs.sort(key=lambda j: -j["api_duration_s"])
        result["runs"][run] = jobs
    with open(out_path, "w", encoding="utf-8") as f:
        json.dump(result, f, indent=1)


if __name__ == "__main__":
    main()

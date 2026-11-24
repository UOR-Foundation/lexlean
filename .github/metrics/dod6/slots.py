"""DOD-7a: libtest slot occupancy (ESTIMATED from the slot model in parse_logs.py).
For each heavy conformance binary: slot-seconds occupied by a BS test that is
blocked on the HEAVY mutex (ESTIMATED), by the running BS test, and by non-BS tests;
and the tail after the last BS completion. Usage: python -I slots.py <dod7a_dir> <run>:<job> ..."""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import parse_logs
base = sys.argv[1]
res = {}
for spec in sys.argv[2:]:
    run, job = spec.split(":")
    log = parse_logs.parse_log(os.path.join(base, "logs", run, job + ".log"))
    for b in log["binaries"]:
        if "conformance" not in b["name"] and b["name"] != "(unnamed)":
            continue
        if b["n_tests"] < 2 or not b.get("per_test_timestamps_usable"):
            continue
        t0, t1 = b["t_running"], b["t_result"]
        ev = []
        bs_exec = []
        for n, t in b["tests"].items():
            if "end" not in t: continue
            s = t["libtest_start_est"]
            if n.startswith("conformance_bs_"):
                es = t["end"] - t["bs_exec_est_s"]
                ev.append((s, es, "bs_blocked")); ev.append((es, t["end"], "bs_running"))
            else:
                ev.append((s, t["end"], "non_bs"))
        acc = {"bs_blocked": 0.0, "bs_running": 0.0, "non_bs": 0.0}
        for s, e, k in ev: acc[k] += max(0.0, e - s)
        thr = b["inferred_test_threads"]
        total = thr * (t1 - t0)
        last_bs = max((t["end"] for n, t in b["tests"].items() if n.startswith("conformance_bs_") and "end" in t), default=None)
        r = {"binary_wall_s": round(t1 - t0, 1), "threads": thr, "slot_seconds": round(total),
             "bs_blocked_slot_s": round(acc["bs_blocked"]), "bs_running_slot_s": round(acc["bs_running"]),
             "non_bs_slot_s": round(acc["non_bs"]),
             "unused_slot_s": round(total - sum(acc.values())),
             "bs_blocked_pct_of_slots": round(100 * acc["bs_blocked"] / total, 1),
             "tail_after_last_bs_s": round(t1 - last_bs, 1) if last_bs else None}
        if last_bs:
            tail = sorted(((t["end"] - max(t["libtest_start_est"], last_bs), n) for n, t in b["tests"].items()
                           if "end" in t and t["end"] > last_bs), reverse=True)[:5]
            r["longest_tests_running_after_last_bs"] = [(n, round(x, 1)) for x, n in tail]
        res[f"{run}:{job}:{b['name']}"] = r
        print(run, job, b["name"], json.dumps(r))
json.dump(res, open(os.path.join(base, "slots.json"), "w"), indent=1)

#!/usr/bin/env python3
"""Audit-only historical Intel lane scenarios. Never grants CI status authority.

Measured job runtime is NOT per-test CPU time. Ideal splits are optimistic
bounds, never predicted runner cost or end-to-end CI latency.
"""
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import re
import sys


class Hold(ValueError):
    """Insufficient or contradictory source/evidence."""


def check(condition, message):
    if not condition:
        raise Hold(message)


def blob_sha(content):
    data = content.encode("utf-8")
    return hashlib.sha1(b"blob " + str(len(data)).encode() + b"\0" + data).hexdigest()


def case_name(identifier):
    return "conformance_" + identifier.lower().replace("-", "_")


def validate(evidence, checkout=None):
    check(evidence.get("schema") == "lexlean-intel-historical-lane-v1", "unknown evidence schema")
    check(evidence.get("run_id") == 45100057759 and evidence.get("head_sha") ==
          "b693bb4259d311fd71da90ac38701214a74be27f", "unexpected historical run")
    sources = evidence.get("sources", {})
    check(set(sources) == {"bdd", "lanes"}, "unknown/missing sources")
    for label, path in (("bdd", ".github/bdd-parts.json"), ("lanes", ".github/intel-host-lanes.json")):
        src = sources[label]
        check(set(src) == {"path", "git_blob_sha", "raw_text"} and src["path"] == path,
              f"unbound {label} source")
        check(blob_sha(src["raw_text"]) == src["git_blob_sha"], f"{label} blob hash mismatch")
        if checkout is not None:
            check((Path(checkout) / path).read_bytes() == src["raw_text"].encode("utf-8"),
                  f"{label} checkout drift: historical model cannot apply to new source")
    bdd = json.loads(sources["bdd"]["raw_text"])
    lanes = json.loads(sources["lanes"]["raw_text"])
    check(set(bdd) == {"parts", "targets", "remainder"} and
          set(lanes) == {"producer", "lanes"}, "unsupported manifest structure")
    check(lanes["producer"] == ["honesty"] and set(lanes["lanes"]) ==
          {"lower", "print", "validate", "moves"}, "unexpected lane contract")
    check(bdd["remainder"] == "rest", "unexpected remainder owner")
    flattened = [part for parts in lanes["lanes"].values() for part in parts]
    check(Counter(flattened) == Counter([*bdd["parts"], bdd["remainder"]]),
          "BDD parts missing, duplicated, foreign, or owned by producer")
    check(lanes["lanes"]["moves"].count("rest") == 1, "remainder owner drift")
    named = [case_name(case) for ids in bdd["parts"].values() for case in ids]
    check(len(named) == len(set(named)) == 229, "registered BDD inventory mismatch")
    jobs = evidence.get("lanes", {})
    check(set(jobs) == set(lanes["lanes"]), "missing/foreign native lane receipt")
    combined = []
    expected_ids = {"lower": 137457635516, "print": 137457635561,
                    "validate": 137457635553, "moves": 137457635539}
    for lane, parts in lanes["lanes"].items():
        job = jobs[lane]
        check(set(job) == {"job_id", "started_at", "completed_at", "elapsed_seconds",
                          "execution_step_seconds", "cases_passed", "cases_over_60s"},
              f"incomplete {lane} native job evidence")
        ids = job["cases_passed"]
        check(isinstance(ids, list) and ids and all(isinstance(x, str) and
              re.fullmatch(r"conformance_[a-z0-9_]+", x) for x in ids),
              f"invalid {lane} case receipts")
        check(len(ids) == len(set(ids)), f"duplicate case in {lane}")
        check(job["job_id"] == expected_ids[lane], f"wrong {lane} job provenance")
        check(isinstance(job["elapsed_seconds"], (int, float)) and
              isinstance(job["execution_step_seconds"], (int, float)) and
              0 < job["execution_step_seconds"] <= job["elapsed_seconds"] < 14400,
              f"invalid {lane} time")
        check(len(job["cases_over_60s"]) == len(set(job["cases_over_60s"])) and
              set(job["cases_over_60s"]) <= set(ids), f"invalid {lane} slow warnings")
        wanted = {case_name(n) for p in parts if p in bdd["parts"] for n in bdd["parts"][p]}
        actual_named = set(ids) & set(named)
        check(actual_named == wanted, f"incorrect named detector ownership in {lane}")
        if lane != "moves":
            check(set(ids) == wanted, f"remainder escaped to {lane}")
        combined += ids
    check(len(combined) == len(set(combined)) == 507 and set(named) <= set(combined),
          "507-case native result missing, duplicated or foreign")
    check(len(set(combined) - set(named)) == 278 and
          set(combined) - set(named) <= set(jobs["moves"]["cases_passed"]),
          "remainder receipt incomplete")
    return jobs


def lpt(jobs, slots):
    """Non-preemptive greedy placement into hypothetical fixed worker slots."""
    check(isinstance(slots, int) and 1 <= slots <= 8, "invalid hypothetical host slots")
    buckets = [0.0] * slots
    for duration in sorted(jobs, reverse=True):
        check(isinstance(duration, (float, int)) and 0 < duration < 14400,
              "invalid hypothetical job duration")
        smallest = min(range(slots), key=lambda i: (buckets[i], i))
        buckets[smallest] += duration
    return round(max(buckets), 2)


def analyze(evidence, checkout=None):
    jobs = validate(evidence, checkout)
    measured = {name: float(job["elapsed_seconds"]) for name, job in jobs.items()}
    moves = jobs["moves"]
    overhead = float(moves["elapsed_seconds"] - moves["execution_step_seconds"])
    check(0 <= overhead < 240, "unreasonable extra-lane overhead")
    scenarios = []
    for splits in (1, 2, 3, 4):
        durations = [measured[k] for k in ("lower", "print", "validate")]
        durations += [float(moves["execution_step_seconds"]) / splits + overhead] * splits
        scenarios.append({
            "ideal_moves_sublanes": splits,
            "hypothetical_extra_jobs": splits - 1,
            "job_sum_seconds": round(sum(durations), 2),
            "additional_runner_seconds_vs_existing": round(sum(durations) - sum(measured.values()), 2),
            "optimistic_envelopes_seconds_by_slots": {str(n): lpt(durations, n)
                                                       for n in (1, 2, 3, 4, 5)},
        })
    return {
        "classification": "SIMULATED_UPPER_BOUND_ONLY_NOT_AN_APPROVED_CI_CHANGE",
        "run_id": evidence["run_id"], "historical_head_sha": evidence["head_sha"],
        "source_blobs": {k: v["git_blob_sha"] for k, v in evidence["sources"].items()},
        "observed_case_counts": {k: len(v["cases_passed"]) for k, v in jobs.items()},
        "observed_slow_warning_counts": {k: len(v["cases_over_60s"]) for k, v in jobs.items()},
        "observed_remainder_case_count": 278,
        "observed_peak_simultaneous_intel_conformance_jobs": 3,
        "provider_concurrency_limit": "UNKNOWN: observed peak does not establish runner capacity",
        "observed_intel_moves_test_step_seconds": moves["execution_step_seconds"],
        "observed_intel_moves_overhead_seconds": round(overhead, 2),
        "limits": [
            "Even split of measured moves test-step wall is NOT measured per-case CPU demand",
            "LPT assumes fixed worker slots, no queue, setup overlap, interference, failures or cache drift",
            "Source manifest is shared with Windows/macOS ARM; proposals are not wired to CI",
            "No billable-minute, required-check, merge-ready or assurance-parity benefit established",
        ],
        "scenarios": scenarios,
    }


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--evidence", type=Path, required=True)
    p.add_argument("--checkout", type=Path, help="Reject when live source differs from the historical run")
    args = p.parse_args()
    try:
        data = json.loads(args.evidence.read_text())
        print(json.dumps(analyze(data, args.checkout), indent=2, sort_keys=True))
    except (Hold, OSError, ValueError, KeyError, TypeError) as error:
        print("HOLD: " + str(error), file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())

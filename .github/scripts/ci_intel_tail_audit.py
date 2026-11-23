#!/usr/bin/env python3
"""Read-only Intel native CI queue/case-tail audit; no merge-gate authority."""
import argparse
from datetime import datetime
import json
from pathlib import Path
import sys

EXPECTED = {"lower": 137457635516, "print": 137457635561,
            "validate": 137457635553, "moves": 137457635539}


class Hold(ValueError):
    pass


def require(test, message):
    if not test:
        raise Hold(message)


def timestamp(value):
    require(isinstance(value, str) and value.endswith("Z"), "bad timestamp")
    return datetime.fromisoformat(value.replace("Z", "+00:00")).timestamp()


def evaluate(ledger):
    require(ledger.get("schema") == "lexlean-intel-ci-case-tail-v1", "unknown schema")
    require(ledger.get("run_id") == 45100057759 and ledger.get("head_sha") ==
            "b693bb4259d311fd71da90ac38701214a74be27f", "wrong run/revision")
    lanes = ledger.get("lanes", {})
    require(set(lanes) == set(EXPECTED), "missing or extra host lane")
    cases = []
    results = {}
    for lane, job_id in EXPECTED.items():
        job = lanes[lane]
        require(job.get("job_id") == job_id, "wrong native job ID")
        require(isinstance(job.get("cases"), list), "missing case evidence")
        created, started, ended = [timestamp(job[k]) for k in
                                   ("created_at", "started_at", "completed_at")]
        require(created <= started < ended, "invalid queue/job time ordering")
        require(0 <= started-created < 86400, "invalid queue delay")
        case_names = []
        slow = []
        for entry in job["cases"]:
            require(set(entry) == {"name", "finished_at", "warning_at"}, "unexpected case fields")
            name = entry["name"]
            require(isinstance(name, str) and name.startswith("conformance_"), "foreign test")
            finished = timestamp(entry["finished_at"])
            require(started <= finished <= ended, "case finished outside job")
            case_names.append(name)
            if entry["warning_at"] is not None:
                warning = timestamp(entry["warning_at"])
                require(started <= warning < finished, "invalid warning order")
                floor = 60 + finished - warning
                require(floor <= ended-started+1, "test lower bound exceeds job")
                slow.append({"name": name, "minimum_seconds": round(floor, 3)})
        require(len(case_names) == len(set(case_names)), "duplicate case within lane")
        cases.extend(case_names)
        results[lane] = {"case_count": len(case_names), "slow_warning_count": len(slow),
                         "queue_seconds": round(started-created, 3),
                         "job_seconds": round(ended-started, 3),
                         "top_slow_cases": sorted(slow, key=lambda x: -x["minimum_seconds"])[:10],
                         "sum_slow_lower_bounds_seconds_nonadditive": round(sum(x["minimum_seconds"] for x in slow), 3)}
    require(len(cases) == 507 and len(set(cases)) == 507, "507-case inventory not complete/unique")
    require([results[x]["case_count"] for x in EXPECTED] == [6, 20, 86, 395],
            "unexpected lane counts")
    require([results[x]["slow_warning_count"] for x in EXPECTED] == [6, 18, 12, 31],
            "unexpected warning inventory")
    worst = results["moves"]["top_slow_cases"][0]
    ideal = 3584/3
    return {"classification": "HISTORICAL_LOWER_BOUNDS_NOT_PER_CASE_EXACT_DURATIONS",
            "run_id": ledger["run_id"], "head_sha": ledger["head_sha"],
            "total_unique_passed": len(cases), "lanes": results,
            "moves_longest_case_lower_bound_seconds": worst["minimum_seconds"],
            "moves_equal_three_test_step_seconds": round(ideal, 3),
            "unchanged_case_duration_equal_split_feasible": worst["minimum_seconds"] <= ideal,
            "interpretation": "An individual case can run faster or slower if host concurrency/contention changes; lower bounds cannot be summed as CPU time."}


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--ledger", type=Path, required=True)
    a = p.parse_args()
    try:
        print(json.dumps(evaluate(json.loads(a.ledger.read_text())), indent=2, sort_keys=True))
    except (Hold, ValueError, OSError, KeyError, TypeError) as error:
        print("HOLD: " + str(error), file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())

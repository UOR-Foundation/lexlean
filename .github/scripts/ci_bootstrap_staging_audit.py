#!/usr/bin/env python3
"""Execute the committed BDD job partition, without invoking the CI gate.

Regression proof for CI #603: bootstrap-consuming tests must not allocate
runners before the single run-bound bootstrap export is ready. This audit
executes the same jq expression from ci-plan.sh against actual source rows.
It is READ-ONLY, AUDIT-ONLY, and not merge/release qualification.
"""
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

SOURCES = (
    ".github/ci-plan.sh",
    ".github/workflows/ci.yml",
    ".github/vv-rows.json",
    ".github/bdd-parts.json",
    "crates/conformance/tests/conformance.rs",
)
JQ_BEGIN = 'classified=$(jq -cn --argjson rows "$2" --slurpfile manifest "$bdd_parts_file" \''
JQ_END = "') || return 1"
BS_TEST = re.compile(
    r'fn conformance_bs_(\d{2})\(\)\s*\{\s*'
    r'repo_conformance::cases::run\("BS-(\d{2})"\);\s*\}'
)


class Hold(ValueError):
    pass


def require(ok, message):
    if not ok:
        raise Hold(message)


def strict_json(blob):
    def pairs(items):
        result = {}
        for key, val in items:
            require(key not in result, "duplicate JSON key: " + str(key))
            result[key] = val
        return result
    return json.loads(blob, object_pairs_hook=pairs)


def extract_program(source):
    require(source.count(JQ_BEGIN) == 1 and source.count(JQ_END) >= 1,
            "actual planner's jq expression missing or ambiguous")
    after = source.split(JQ_BEGIN, 1)[1]
    program, separator, _ = after.partition(JQ_END)
    require(separator and program.strip(), "actual planner jq block not terminated")
    return program


def job_block(workflow, name):
    jobs = workflow.split("\njobs:\n", 1)
    require(len(jobs) == 2, "normative workflow jobs section unavailable")
    match = re.search(r"^  " + re.escape(name) + r":\s*$", jobs[1], re.M)
    require(match, "missing normative " + name + " job")
    remaining = jobs[1][match.end():]
    next_job = re.search(r"^  [a-z][a-z0-9-]+:\s*$", remaining, re.M)
    return remaining[:next_job.start()] if next_job else remaining


def audit(root):
    root = Path(root)
    blobs = {}
    for name in SOURCES:
        file = root / name
        require(file.is_file() and not file.is_symlink(),
                "missing or linked audit input: " + name)
        blobs[name] = file.read_bytes()
    try:
        source = {name: value.decode("utf-8") for name, value in blobs.items()}
        rows = strict_json(source[".github/vv-rows.json"])
        parts_file = strict_json(source[".github/bdd-parts.json"])
    except (UnicodeError, ValueError) as error:
        raise Hold("malformed UTF-8 or JSON staging input") from error
    require(isinstance(rows, list) and all(isinstance(x, dict) for x in rows),
            "invalid normative row list")
    require(isinstance(parts_file, dict) and parts_file.get("remainder") == "rest",
            "unreviewed BDD remainder")
    parts = parts_file.get("parts")
    require(isinstance(parts, dict) and parts and all(
        isinstance(k, str) and isinstance(v, list) and v and
        all(isinstance(i, str) for i in v) for k, v in parts.items()),
        "invalid BDD part inventory")
    names = [x.get("part") for x in rows]
    require(all(isinstance(x, str) for x in names) and
            len(names) == len(set(names)), "missing/duplicated normative row")
    bdd_rows = [x for x in rows if x["part"].startswith("bdd-")]
    require(set(x.get("bdd_part") for x in bdd_rows) == set(parts) and
            all(x["part"] == "bdd-" + x["bdd_part"] for x in bdd_rows),
            "BDD partitions and actual vv rows differ")
    require(set(names) - {x["part"] for x in bdd_rows} ==
            {"honesty", "examples", "rest"},
            "unreviewed other normative rows")
    require(next(x for x in rows if x["part"] == "rest").get("bdd_part") == "rest",
            "unreviewed BDD remainder row")
    bs_ids = [case for cases in parts.values() for case in cases if case.startswith("BS-")]
    require(len(bs_ids) == len(set(bs_ids)), "duplicate bootstrap detector in BDD parts")
    registry = source["crates/conformance/tests/conformance.rs"]
    declared = re.findall(r"fn conformance_bs_(\d{2})\(\)", registry)
    wired = BS_TEST.findall(registry)
    require(declared and len(declared) == len(wired) and
            all(a == b for a, b in wired) and len(declared) == len(set(declared)),
            "bootstrap tests must execute their matching conformance IDs")
    expected_bs = {"BS-" + n for n in declared}
    require(expected_bs == set(bs_ids),
            "bootstrap detector missing/extra in BDD staging manifest")

    job_fast = job_block(source[".github/workflows/ci.yml"], "vv-fast")
    job_export = job_block(source[".github/workflows/ci.yml"], "vv")
    require(re.search(r"^    needs: \[plan, honesty\]$", job_fast, re.M) and
            "fromJSON(needs.plan.outputs.fanout_fast)" in job_fast,
            "independent lane dependency/selector changed")
    require(re.search(r"^    needs: \[plan, honesty, bootstrap-export\]$", job_export, re.M) and
            "fromJSON(needs.plan.outputs.fanout_export)" in job_export,
            "bootstrap consumer missing producer dependency/selector")
    require("LEXLEAN_CI_INDEPENDENT_LANE:" in job_fast,
            "independent lane lost no-bootstrap-export safeguard")
    program = extract_program(source[".github/ci-plan.sh"])
    try:
        executed = subprocess.run(
            ["jq", "-cn", "--argjson", "rows", json.dumps(rows, separators=(",", ":")),
             "--slurpfile", "manifest", str(root / ".github/bdd-parts.json"), program],
            capture_output=True, text=True, timeout=8, check=False,
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        raise Hold("planner jq execution unavailable") from error
    require(executed.returncode == 0, "actual planner jq refused case classification")
    try:
        assigned = strict_json(executed.stdout)
    except (ValueError, json.JSONDecodeError) as error:
        raise Hold("actual planner returned malformed classification") from error
    require(isinstance(assigned, dict) and set(assigned) == {"fast", "export"} and
            all(isinstance(assigned[key], list) for key in assigned),
            "actual planner gave invalid lane structure")

    work = [x for x in rows if x["part"] != "honesty"]
    expected_export = [x for x in work if x["part"].startswith("bdd-") and
                       any(i.startswith("BS-") for i in parts[x["bdd_part"]])]
    expected_fast = [x for x in work if x not in expected_export]
    require(assigned["fast"] == expected_fast and
            assigned["export"] == expected_export,
            "actual CI planner schedules bootstrap consumer before its producer")
    return {
        "schema": "lexlean/bootstrap-staging-audit/1",
        "classification": "AUDIT_ONLY_NOT_QUALIFICATION",
        "source_sha256": {name: hashlib.sha256(content).hexdigest()
                          for name, content in sorted(blobs.items())},
        "registered_bootstrap_cases": len(declared),
        "producer_dependent_bdd_partitions": len(expected_export),
        "producer_independent_bdd_partitions": sum(
            row["part"].startswith("bdd-") for row in expected_fast),
        "independent_rows_including_examples_and_rest": len(expected_fast),
        "checked_planner": "actual jq extracted from ci-plan.sh",
        "provider_proof_receipts_verified": False,
    }


def main():
    root = Path(sys.argv[1]) if len(sys.argv) == 2 else Path(".")
    try:
        print(json.dumps(audit(root), indent=2, sort_keys=True))
        return 0
    except (Hold, OSError, ValueError) as error:
        print("HOLD: " + str(error), file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())

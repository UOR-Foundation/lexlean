#!/usr/bin/env python3
"""Reviewable source-bound acceptance inventory; NEVER a selective merge gate."""
import argparse
import hashlib
import json
import re
import sys
from collections import Counter
from pathlib import Path

REQUIRED_JOBS = frozenset({
    "plan", "honesty", "bootstrap-export", "vv-fast", "vv",
    "atlas-prefix", "atlas-cert4", "atlas-l1", "atlas-census-tail",
    "atlas-cert4-replay", "atlas-closure", "atlas-l3", "atlas-join",
    "host-matrix", "intel-construction", "intel-part", "intel-join",
    "windows-part", "windows-join", "macos-part", "macos-join",
})
SOURCES = ("Justfile", ".github/vv-rows.json", ".github/bdd-parts.json",
           ".github/workflows/ci.yml", ".github/workflows/reproducibility.yml")


class AuditHold(ValueError):
    """A source change requires re-evaluation rather than inferred safety."""


def require(ok, message):
    if not ok:
        raise AuditHold(message)


def recipes_in(text):
    names = {}
    current = None
    for line in text.splitlines():
        m = re.fullmatch(r"([a-z][a-z0-9_-]*):(?:\s+(.*))?", line)
        if m:
            name = m.group(1)
            require(name not in names, "duplicate Justfile recipe: " + name)
            names[name] = {"dependencies": (m.group(2) or "").split(), "commands": []}
            current = name
        elif current and (line.startswith(" ") or line.startswith("\t")) and line.strip():
            names[current]["commands"].append(line.strip())
    return names


def job_blocks(text):
    match = re.search(r"^jobs:\s*$", text, re.M)
    require(match, "ci.yml has no jobs section")
    body = text[match.end():]
    matches = list(re.finditer(r"^  ([a-z][a-z0-9-]+):\s*$", body, re.M))
    require(matches, "ci.yml job inventory empty")
    result = {}
    for i, m in enumerate(matches):
        name = m.group(1)
        require(name not in result, "duplicate ci.yml job: " + name)
        result[name] = body[m.end():matches[i + 1].start() if i + 1 < len(matches) else len(body)]
    return result


def audit(root):
    root = Path(root)
    try:
        data = {path: (root / path).read_bytes() for path in SOURCES}
        text = {path: value.decode("utf-8") for path, value in data.items()}
        rows = json.loads(text[".github/vv-rows.json"])
        parts = json.loads(text[".github/bdd-parts.json"])
    except (OSError, UnicodeError, ValueError) as error:
        raise AuditHold("missing, unreadable or malformed source inventory") from error
    require(isinstance(rows, list) and rows, "vv row ledger missing")
    require(isinstance(parts, dict) and isinstance(parts.get("parts"), dict), "BDD manifest invalid")
    require(parts.get("remainder") == "rest", "BDD remainder requires review")
    recipes = recipes_in(text["Justfile"])
    require("vv" in recipes and "repro" in recipes, "Justfile lost normative recipes")
    vv = recipes["vv"]["dependencies"]
    require(len(vv) == len(set(vv)) and "bdd" in vv and "repro" in vv,
            "vv recipe dependencies changed or duplicated")
    require(all(x in recipes for x in vv), "vv dependency has no recipe")
    require(recipes["repro"]["commands"] == ["cargo xtask check-reproducibility"],
            "repro terminal command changed: re-evaluate detector")
    counts = Counter()
    row_names = set()
    bdd_row_parts = set()
    for row in rows:
        require(isinstance(row, dict), "vv row must be an object")
        name, command, bdd = row.get("part"), row.get("recipes"), row.get("bdd_part")
        require(isinstance(name, str) and name and name not in row_names,
                "duplicate or invalid vv row part")
        require(isinstance(command, str) and isinstance(bdd, str), "invalid vv row recipe")
        row_names.add(name)
        used = command.split()
        require(used and all(x in vv for x in used), "unknown recipe in vv row " + name)
        counts.update(used)
        if name in {"honesty", "rest"}:
            require(bdd == name and "bdd" in used, "BDD root row changed: " + name)
        elif name.startswith("bdd-"):
            part = name[4:]
            require(bdd == part and used == ["bdd"], "BDD partition row changed: " + name)
            require(part in parts["parts"], "unregistered BDD partition: " + part)
            bdd_row_parts.add(part)
        else:
            require(bdd == "" and "bdd" not in used,
                    "non-BDD row has a BDD partition: " + name)
    require({"honesty", "rest", "examples"} <= row_names, "key vv rows missing")
    require(bdd_row_parts == set(parts["parts"]), "BDD partition rows and manifest differ")
    require(all(counts[r] == 1 for r in vv if r != "bdd"),
            "non-BDD recipe missing or duplicated in vv rows")
    require(counts["bdd"] == len(parts["parts"]) + 2,
            "BDD partitions must include honesty and remainder")
    rest = next(row for row in rows if row["part"] == "rest")
    require("repro" in rest["recipes"].split(), "repro moved out of rest without governance")
    jobs = job_blocks(text[".github/workflows/ci.yml"])
    require(REQUIRED_JOBS <= set(jobs), "normative ci.yml job removed or renamed")
    for job, expected_needs, fanout in (
        ("vv-fast", "needs: [plan, honesty]", "fanout_fast"),
        ("vv", "needs: [plan, honesty, bootstrap-export]", "fanout_export"),
    ):
        require(expected_needs in jobs[job] and
                "fromJSON(needs.plan.outputs." + fanout + ")" in jobs[job],
                "vv work or producer dependency changed: " + job)
    standalone = text[".github/workflows/reproducibility.yml"]
    require(len(re.findall(r"^\s*- run: just repro\s*$", standalone, re.M)) == 1,
            "standalone repro invocation changed: review status/independence")
    expected_guard = "if: " + "$" + "{{ github.event_name != 'pull_request' || !github.event.pull_request.draft }}"
    require(expected_guard in standalone, "standalone reproducibility draft behavior changed")
    return {
        "schema": "lexlean/ci-obligation-audit/1",
        "classification": "AUDIT_ONLY_NOT_QUALIFICATION",
        "complete_source_to_normative_obligations": False,
        "sources_sha256": {k: hashlib.sha256(v).hexdigest() for k, v in sorted(data.items())},
        "vv_recipes": vv, "vv_rows": len(rows), "bdd_partitions": len(parts["parts"]),
        "normative_jobs": sorted(jobs),
        "cross_workflow_overlap": [{
            "terminal": "cargo xtask check-reproducibility",
            "via_ci": "vv/rest -> repro",
            "via_standalone": "reproducibility.yml -> just repro",
            "safe_to_eliminate": False, "required_status_mapping_verified": False,
        }],
        "blocked_on": "Complete source-to-detector/proof/host/release coverage and governed required statuses",
    }


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--root", default=".")
    args = p.parse_args()
    try:
        report = audit(args.root)
    except AuditHold as error:
        print("HOLD: " + str(error), file=sys.stderr)
        return 2
    print(json.dumps(report, sort_keys=True, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

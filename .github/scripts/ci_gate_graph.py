#!/usr/bin/env python3
"""Read-only partial graph of LexLean proof/host obligations, not merge authority."""
import argparse
import hashlib
import json
import re
import sys
from collections import Counter
from pathlib import Path

from ci_obligation_audit import AuditHold, audit, job_blocks, require

ATLAS_JOBS = {
    "atlas-prefix": ("prefix",),
    "atlas-cert4": ("cert4",),
    "atlas-l1": ("census-a", "census-b"),
    "atlas-census-tail": ("census-c", "census-d"),
    "atlas-cert4-replay": ("cert4-replay",),
    "atlas-closure": ("closure",),
    "atlas-l3": ("orbit", "pairs-a", "pairs-b", "pairs-c", "pairs-d"),
    "atlas-join": ("join",),
}
SOURCES = (
    ".github/verify-shards.json", ".github/intel-host-lanes.json",
    ".github/workflows/ci.yml", ".github/vv-rows.json", ".github/bdd-parts.json",
    ".github/scripts/intel_host_tests.py", ".github/scripts/host_lanes.py",
)


def strict_json(raw):
    def unique(pairs):
        result = {}
        for key, value in pairs:
            require(key not in result, "duplicate JSON property: " + key)
            result[key] = value
        return result
    try:
        return json.loads(raw, object_pairs_hook=unique)
    except AuditHold:
        raise
    except (ValueError, TypeError) as error:
        raise AuditHold("malformed or ambiguous JSON") from error


def job_dependencies(jobs):
    result = {}
    for name, body in jobs.items():
        match = re.search(r"^    needs: \[([^\]\n]+)\]\s*$", body, re.M)
        # Only jobs with explicit bracket-form needs take part in this graph.
        if match:
            deps = tuple(x.strip() for x in match.group(1).split(","))
            require(len(set(deps)) == len(deps) and all(x in jobs for x in deps),
                    "unknown or duplicate workflow dependency: " + name)
            result[name] = deps
        else:
            result[name] = ()
    def reachable(name, stack=()):
        require(name not in stack, "cycle in workflow job dependencies: " + name)
        reached = set()
        for dep in result[name]:
            reached.add(dep)
            reached.update(reachable(dep, stack + (name,)))
        return reached
    return {name: reachable(name) for name in result}


def atlas_graph(document, jobs):
    require(isinstance(document, dict) and document.get("project") == "uor-atlas" and
            isinstance(document.get("shards"), dict), "Atlas shard manifest invalid")
    shards = document["shards"]
    seen = Counter(name for group in ATLAS_JOBS.values() for name in group)
    require(set(shards) == set(seen) and all(count == 1 for count in seen.values()),
            "Atlas shards and workflow job mapping disagree")
    names = set(shards)
    for name, shard in shards.items():
        require(isinstance(shard, dict) and all(isinstance(shard.get(k), list)
                for k in ("needs", "elaborate", "replay")),
                "Atlas shard malformed: " + name)
        for key in ("needs", "elaborate", "replay"):
            entries = shard[key]
            require(all(isinstance(x, str) and x for x in entries) and
                    len(entries) == len(set(entries)),
                    "duplicate or invalid Atlas " + key + ": " + name)
        require(set(shard["needs"]) <= names and name not in shard["needs"],
                "unknown Atlas dependency: " + name)
    require(set(shards["join"]["needs"]) == names - {"join"},
            "Atlas join must await every other shard")
    def ancestor(name, stack=()):
        require(name not in stack, "Atlas proof dependency cycle: " + name)
        found = set(shards[name]["needs"])
        for need in shards[name]["needs"]:
            found.update(ancestor(need, stack + (name,)))
        return found
    for name in names:
        ancestor(name)
    closure = job_dependencies(jobs)
    owner = {shard: job for job, group in ATLAS_JOBS.items() for shard in group}
    for job, group in ATLAS_JOBS.items():
        require(job in jobs, "Atlas job missing: " + job)
        body = jobs[job]
        require("needs.plan.outputs.full == 'true'" in body,
                "Atlas job no longer full-gate guarded: " + job)
        if len(group) > 1:
            m = re.search(r"matrix:\s*\{\s*shard:\s*\[([^\]]+)\]", body)
            require(m and tuple(x.strip() for x in m.group(1).split(",")) == group,
                    "Atlas matrix does not match manifest mapping: " + job)
            require("LEXLEAN_EXAMPLES_PART: shard:uor-atlas:" in body,
                    "Atlas shard selection missing: " + job)
        elif group == ("join",):
            require("LEXLEAN_EXAMPLES_PART: join:uor-atlas" in body,
                    "Atlas join selection missing")
        else:
            require("LEXLEAN_EXAMPLES_PART: shard:uor-atlas:" + group[0] in body,
                    "Atlas selector missing: " + job)
        for shard in group:
            for need in shards[shard]["needs"]:
                require(owner[need] in closure[job],
                        "Atlas upstream shard not reachable: " + shard + " <- " + need)
    return {
        "shards": len(shards),
        "modules_elaborated": sum(len(v["elaborate"]) for v in shards.values()),
        "replay_assignments": sum(len(v["replay"]) for v in shards.values()),
        "shard_job_owner": {k: owner[k] for k in sorted(owner)},
        "join_needs": sorted(shards["join"]["needs"]),
        "provider_receipt_parity_verified": False,
    }


def supported_hosts(rows, bdd, lane_file, jobs, intel_source, host_source):
    require(isinstance(rows, list) and isinstance(bdd, dict) and
            isinstance(bdd.get("parts"), dict), "invalid BDD or row structure")
    part_names = set(bdd["parts"])
    require(part_names, "no BDD partitions")
    cases = [identifier for ids in bdd["parts"].values() for identifier in ids]
    require(all(isinstance(x, str) and re.fullmatch(r"[A-Z]{2}-\d{2}", x)
                for x in cases) and len(cases) == len(set(cases)),
            "duplicate or malformed BDD case identifier")
    expected_rows = {x["part"] for x in rows if x["part"] not in ("honesty", "examples")}
    require(expected_rows == {"rest"} | {"bdd-" + x for x in part_names},
            "BDD manifest does not match active workflow rows")
    require(isinstance(lane_file, dict) and lane_file.get("producer") == ["honesty"] and
            isinstance(lane_file.get("lanes"), dict), "host lane shape changed")
    actual = []
    for lane, members in lane_file["lanes"].items():
        require(isinstance(lane, str) and lane and isinstance(members, list) and members,
                "malformed host lane")
        actual.extend(members)
    # Host lanes name BDD parts without the workflow-only "bdd-" prefix.
    normalized_rows = {name[4:] if name.startswith("bdd-") else name
                       for name in expected_rows}
    require(len(actual) == len(set(actual)) and set(actual) == normalized_rows,
            "host lane partition duplicates/omits normative verification rows")
    require('LANES = ".github/intel-host-lanes.json"' in intel_source and
            "import intel_host_tests as factory" in host_source,
            "native host lane implementation lost plan source")
    for name, label, runner in (
        ("host-matrix", "aarch64-unknown-linux-gnu", "ubuntu-24.04-arm"),
        ("intel-part", "Intel conformance", "macos-15-intel"),
        ("windows-part", "Windows tests", "windows-2022"),
        ("macos-part", "macOS arm64 tests", "macos-15"),
    ):
        require(name in jobs and label in jobs[name] and runner in jobs[name],
                "native execution lane removed or rehosted: " + name)
    for name, prior, status in (
        ("intel-join", "intel-part", "x86_64-apple-darwin"),
        ("windows-join", "windows-part", "x86_64-pc-windows-msvc"),
        ("macos-join", "macos-part", "aarch64-apple-darwin"),
    ):
        require(name in jobs and prior in job_dependencies(jobs)[name] and
                "build and non-Lean tests (" + status + ")" in jobs[name],
                "native join or required host status disappeared: " + name)
    return {
        "registered_bdd_case_assignments": len(cases),
        "bdd_partitions": len(part_names),
        "named_host_lanes": sorted(lane_file["lanes"]),
        "host_partitioned_rows": len(expected_rows),
        "native_statuses": ["aarch64-unknown-linux-gnu", "x86_64-apple-darwin",
                            "x86_64-pc-windows-msvc", "aarch64-apple-darwin"],
        "native_statuses_policy_verified": False,
        "runtime_test_inventory_verified": False,
    }


def build(root):
    root = Path(root)
    prior = audit(root)
    try:
        data = {path: (root / path).read_bytes() for path in SOURCES}
        strings = {path: value.decode("utf-8") for path, value in data.items()}
        atlas = strict_json(strings[".github/verify-shards.json"])
        lanes = strict_json(strings[".github/intel-host-lanes.json"])
        rows = strict_json(strings[".github/vv-rows.json"])
        bdd = strict_json(strings[".github/bdd-parts.json"])
    except (OSError, UnicodeError) as error:
        raise AuditHold("unavailable graph source") from error
    jobs = job_blocks(strings[".github/workflows/ci.yml"])
    a = atlas_graph(atlas, jobs)
    h = supported_hosts(rows, bdd, lanes, jobs,
                        strings[".github/scripts/intel_host_tests.py"],
                        strings[".github/scripts/host_lanes.py"])
    return {
        "schema": "lexlean/ci-obligation-graph/1",
        "classification": "AUDIT_ONLY_NO_CUTOVER_AUTHORITY",
        "complete_source_to_normative_obligations": False,
        "input_sha256": {p: hashlib.sha256(v).hexdigest() for p, v in sorted(data.items())},
        "recipe_audit": {"vv_rows": prior["vv_rows"], "vv_recipes": prior["vv_recipes"],
                         "duplicate_repro_execution": prior["cross_workflow_overlap"]},
        "atlas": a,
        "supported_hosts": h,
        "unproven": ["SPEC §9.4 and RP-13/14 oracle parity", "source to generated-input dependency closure",
                     "live branch required-status rules", "release and publication transitive guards",
                     "provider artifact/host identity receipts", "every live harness test listed and executed"],
    }


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", default=".")
    args = parser.parse_args(argv)
    try:
        print(json.dumps(build(args.root), indent=2, sort_keys=True))
    except AuditHold as exc:
        print("HOLD: " + str(exc), file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

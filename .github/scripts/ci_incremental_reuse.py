#!/usr/bin/env python3
"""A4: conservative, NONQUALIFYING cross-commit CI evidence reuse classifier.

This file does not authenticate GitHub records, authorize skipped status checks,
join receipts, or change SPEC 9.4. All decisions are advisory; run the normative
full gate until source closure and current-head aggregation are independently
qualified. No network, runner dispatch, or repository mutation occurs.
"""
import argparse
import json
import re
import subprocess
from pathlib import Path

SHA = re.compile(r"(?:[0-9a-f]{40}|[0-9a-f]{64})\Z")
IDENT = re.compile(r"[A-Za-z0-9_.:-]{1,120}\Z")
STATUS = re.compile(r"(?:[AMD]|[RC][0-9]{1,3})\Z")
SCHEMA = "lexlean/ci-incremental-reuse/1"
RUN_SCHEMA = "lexlean/ci-provider-observations/1"
# A change in policy enforcement, the job graph or toolchain invalidates every
# experimental reuse claim. More global inputs may be added by policy, never
# fewer than these mandatory controls.
CONTROLS = frozenset((
    ".github/workflows/ci.yml", ".github/workflows/reproducibility.yml",
    ".github/ci-plan.sh", ".github/scripts/ci_incremental_reuse.py",
    "rust-toolchain.toml", "lean-toolchain", "Cargo.lock",
    "Justfile", ".github/vv-rows.json", ".github/bdd-parts.json",
    ".github/verify-shards.json", "SPEC.md", "spec/src/Chapter09.lex.tex",
    # RP-14 is compiled into every conformance lane's common Rust binary.
    # A file-level policy is not allowed to declare this shared test source,
    # its lane planner, or the receipt validators as harmless changed paths.
    "crates/conformance/tests/conformance.rs",
    "crates/conformance/src/cases/repository.rs",
    "crates/conformance/src/cases/mod.rs",
    "crates/conformance/src/lib.rs",
    "xtask/src/ci.rs", ".github/intel-host-lanes.json",
    ".github/scripts/host_lanes.py", ".github/scripts/intel_host_tests.py",
    ".github/scripts/ci_execution_receipts.py",
    ".github/scripts/ci_generated_lineage.py",
))


class Hold(ValueError):
    """Untrustworthy or ambiguous input: no reuse classification."""


def require(condition, reason):
    if not condition:
        raise Hold(reason)


def sha(value):
    return isinstance(value, str) and SHA.fullmatch(value) is not None


def path(value):
    return (isinstance(value, str) and 0 < len(value) < 4096
            and not value.startswith("/") and "\\" not in value
            and "\x00" not in value
            and all(p not in ("", ".", "..") for p in value.split("/")))


def exact(value, keys, label):
    require(isinstance(value, dict) and set(value) == set(keys),
            "ambiguous fields in " + label)


def sequence(value, predicate, label):
    require(isinstance(value, list) and all(predicate(v) for v in value)
            and len(set(value)) == len(value), "invalid/duplicate " + label)
    return set(value)


def load(pathname):
    def unique(pairs):
        result = {}
        for key, value in pairs:
            require(key not in result, "duplicate JSON property")
            result[key] = value
        return result

    def bad_constant(value):
        raise Hold("nonfinite JSON constant")

    try:
        return json.loads(Path(pathname).read_text(encoding="utf-8"),
                          object_pairs_hook=unique, parse_constant=bad_constant)
    except (ValueError, UnicodeError, OSError) as error:
        raise Hold("unreadable or malformed JSON") from error


def changed_paths(data):
    """Parse git diff --name-status -z -M, including both rename/copy sides."""
    require(isinstance(data, bytes) and len(data) <= 1048576,
            "invalid or oversized diff")
    if not data:
        return []
    require(data.endswith(b"\0"), "unterminated git diff")
    parts = data[:-1].split(b"\0")
    result = []
    index = 0
    while index < len(parts):
        try:
            kind = parts[index].decode("ascii")
            count = 2 if kind.startswith(("R", "C")) else 1
            require(STATUS.fullmatch(kind) is not None, "unknown git status")
            require(index + count < len(parts), "truncated git diff")
            names = [parts[index + i].decode("utf-8") for i in range(1, count + 1)]
        except UnicodeError as error:
            raise Hold("unrepresentable git path") from error
        require(all(path(v) for v in names), "unsafe git path")
        result.extend(names)
        index += count + 1
    require(len(result) <= 10000, "too many changed paths")
    return sorted(set(result))


def git_snapshot(repo_root, old, head):
    require(sha(old) and sha(head), "exact commit IDs required")
    root = Path(repo_root).resolve()

    def run(*args):
        try:
            return subprocess.run(["git", "-C", str(root), *args],
                                  capture_output=True, timeout=30, check=True)
        except (OSError, subprocess.SubprocessError) as error:
            raise Hold("Git ancestry, object or diff unverified") from error

    require(run("rev-parse", "--verify", "HEAD").stdout.strip().decode() == head,
            "checked-out HEAD mismatch")
    for commit in (old, head):
        require(run("rev-parse", "--verify", commit + "^{commit}")
                .stdout.strip().decode() == commit, "missing exact commit")
    run("merge-base", "--is-ancestor", old, head)
    old_tree = run("rev-parse", old + "^{tree}").stdout.strip()
    head_tree = run("rev-parse", head + "^{tree}").stdout.strip()
    diff = run("diff", "--name-status", "-z", "-M", "--no-ext-diff",
               old, head).stdout
    changed = changed_paths(diff)
    require((old_tree == head_tree) == (not changed),
            "tree/diff disagreement")
    return {"old": old, "head": head, "changed": changed,
            "identical_tree": old_tree == head_tree}


def classify(policy, observations, snapshot):
    """Return candidates for review, NEVER a current-HEAD or release verdict."""
    exact(policy, ("schema", "repository", "global_inputs", "units"), "policy")
    require(policy["schema"] == SCHEMA and
            isinstance(policy["repository"], str) and policy["repository"],
            "unknown policy schema or repository")
    extras = sequence(policy["global_inputs"], path, "global controls")
    units = policy["units"]
    require(isinstance(units, list) and 0 < len(units) <= 500,
            "unknown/incomplete unit universe")
    indexed = {}
    jobs = set()
    for unit in units:
        exact(unit, ("id", "job", "host", "definition", "kind",
                     "inputs", "explicit_nonimpact", "needs"), "unit")
        ident = unit["id"]
        require(isinstance(ident, str) and IDENT.fullmatch(ident) is not None
                and ident not in indexed, "invalid/duplicate unit ID")
        require(isinstance(unit["job"], str) and bool(unit["job"]) and
                unit["job"] not in jobs, "duplicate/invalid job name")
        jobs.add(unit["job"])
        require(isinstance(unit["host"], str) and bool(unit["host"]) and
                sha(unit["definition"]), "invalid host or definition")
        require(unit["kind"] in ("ordinary", "native", "proof", "repro", "release"),
                "unknown assurance kind")
        inputs = sequence(unit["inputs"], path, "unit inputs")
        nonimpact = sequence(unit["explicit_nonimpact"], path, "nonimpact paths")
        require(not (inputs & nonimpact), "contradictory input/nonimpact paths")
        sequence(unit["needs"], lambda x: isinstance(x, str) and
                 IDENT.fullmatch(x) is not None, "dependencies")
        indexed[ident] = unit

    # Validate graph before any classifications. Cycles, missing dependencies
    # and partial policies cannot silently result in a positive recommendation.
    for ident, unit in indexed.items():
        require(all(dep in indexed and dep != ident for dep in unit["needs"]),
                "unknown/self dependency")
    seen, stack = set(), set()
    def visit(ident):
        require(ident not in stack, "dependency cycle")
        if ident in seen:
            return
        stack.add(ident)
        for dep in indexed[ident]["needs"]:
            visit(dep)
        stack.remove(ident)
        seen.add(ident)
    for ident in indexed:
        visit(ident)

    exact(snapshot, ("old", "head", "changed", "identical_tree"), "snapshot")
    require(sha(snapshot["old"]) and sha(snapshot["head"]) and
            isinstance(snapshot["identical_tree"], bool),
            "unverified commit identity")
    changed = sequence(snapshot["changed"], path, "changed paths")
    require(snapshot["old"] != snapshot["head"] or not changed,
            "identical commit has changes")
    require(snapshot["identical_tree"] == (not changed),
            "tree/diff status mismatch")

    exact(observations, ("schema", "run", "jobs"), "provider observations")
    exact(observations["run"],
          ("repository", "id", "head", "status", "conclusion"),
          "provider run")
    run = observations["run"]
    require(observations["schema"] == RUN_SCHEMA and
            run["repository"] == policy["repository"] and
            run["head"] == snapshot["old"] and
            isinstance(run["id"], int) and not isinstance(run["id"], bool)
            and run["id"] > 0, "foreign/stale provider run")
    require(isinstance(observations["jobs"], list) and
            len(observations["jobs"]) <= 500, "invalid provider job inventory")
    receipts = {}
    job_ids = set()
    for entry in observations["jobs"]:
        exact(entry, ("unit", "job", "id", "run_id", "head", "status",
                      "conclusion", "host", "definition", "evidence_digest"),
              "job receipt")
        ident = entry["unit"]
        require(isinstance(ident, str) and ident in indexed and
                ident not in receipts, "duplicate/unknown receipt unit")
        job_id = entry["id"]
        require(isinstance(job_id, int) and not isinstance(job_id, bool) and
                job_id > 0 and job_id not in job_ids,
                "duplicate/invalid provider job ID")
        job_ids.add(job_id)
        require(entry["run_id"] == run["id"] and
                entry["head"] == snapshot["old"], "cross-run receipt")
        require(entry["status"] in ("completed", "in_progress", "queued") and
                entry["conclusion"] in ("success", "failure", "cancelled",
                                        "skipped", None),
                "invalid job state")
        require(isinstance(entry["job"], str) and
                isinstance(entry["host"], str) and
                sha(entry["definition"]) and sha(entry["evidence_digest"]),
                "incomplete evidence identity")
        receipts[ident] = entry

    global_changed = bool(changed & (CONTROLS | extras))
    decision = {}
    def decide(ident):
        if ident in decision:
            return decision[ident]
        unit = indexed[ident]
        reasons = []
        if run["status"] != "completed" or run["conclusion"] not in ("success", "failure"):
            reasons.append("origin_run_not_terminal")
        if unit["kind"] != "ordinary":
            reasons.append("high_assurance_not_supported")
        if global_changed:
            reasons.append("global_policy_or_toolchain_change")
        relevant = changed & set(unit["inputs"])
        unknown = changed - set(unit["inputs"]) - set(unit["explicit_nonimpact"])
        if relevant:
            reasons.append("declared_input_changed")
        if unknown:
            reasons.append("unknown_change_impact")
        entry = receipts.get(ident)
        if entry is None:
            reasons.append("missing_receipt")
        else:
            if (entry["job"], entry["host"], entry["definition"]) != (
                    unit["job"], unit["host"], unit["definition"]):
                reasons.append("receipt_definition_or_host_mismatch")
            if entry["status"] != "completed" or entry["conclusion"] != "success":
                reasons.append("origin_job_not_successful")
        for dependency in unit["needs"]:
            if decide(dependency)["action"] != "CANDIDATE_REUSE":
                reasons.append("dependency_invalidated")
        decision[ident] = {"action": "RERUN" if reasons else "CANDIDATE_REUSE",
                           "reasons": sorted(set(reasons))}
        return decision[ident]

    for ident in indexed:
        decide(ident)
    return {
        "schema": "lexlean/ci-incremental-advisory/1",
        "qualification": "NONQUALIFYING_NO_MERGE_OR_RELEASE_AUTHORITY",
        "provider_authentication": "NOT_VERIFIED",
        "source_dependency_closure": "DECLARED_NOT_PROVEN",
        "normative_gate_requirement": "FULL_CURRENT_HEAD_GATE_UNCHANGED",
        "old": snapshot["old"], "head": snapshot["head"],
        "changed_paths": sorted(changed),
        "units": {ident: decision[ident] for ident in sorted(decision)},
        "candidate_count": sum(v["action"] == "CANDIDATE_REUSE"
                               for v in decision.values()),
        "rerun_count": sum(v["action"] == "RERUN" for v in decision.values()),
    }


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, required=True)
    parser.add_argument("--old", required=True)
    parser.add_argument("--head", required=True)
    parser.add_argument("--policy", type=Path, required=True)
    parser.add_argument("--observations", type=Path, required=True)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args(argv)
    try:
        snapshot = git_snapshot(args.repo_root, args.old, args.head)
        policy = load(args.policy)
        # The policy must be committed unchanged on both revisions. A local,
        # altered or newly introduced policy does not supply closed evidence.
        rel = args.policy.resolve().relative_to(args.repo_root.resolve()).as_posix()
        require(path(rel), "policy outside trusted checkout")
        def show(revision):
            return subprocess.run(
                ["git", "-C", str(args.repo_root), "show", revision + ":" + rel],
                check=True, capture_output=True, timeout=30).stdout
        require(show(args.old) == show(args.head) ==
                args.policy.read_bytes(), "policy unavailable or changed")
        report = classify(policy, load(args.observations), snapshot)
    except (Hold, OSError, ValueError, subprocess.SubprocessError) as error:
        parser.exit(2, "HOLD: " + str(error) + "\n")
    encoded = json.dumps(report, sort_keys=True, indent=2) + "\n"
    if args.output:
        args.output.write_text(encoded, encoding="utf-8")
    else:
        print(encoded, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

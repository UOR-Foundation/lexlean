#!/usr/bin/env python3
"""Which gate obligations a change could leave untouched (CI #603, phase 2; shadow).

Phase 2 decides nothing that runs. Given a verified base, a candidate, the
change between them, and the base's receipts and read traces (phase 1), it
says for each obligation either `run` or `would-reuse`, with every reason. A
shadow run is then scored against the candidate's real full gate: a
`would-reuse` obligation that failed there is an unsound decision, and one is
enough to stop the model from ever being enforced.

The model is conservative by construction. An obligation is `would-reuse`
only when every rule below holds, and each missing or doubtful fact is a
reason to run, never a gap to fill:

1. The change is a non-empty set of pure content modifications: no add,
   delete, rename, copy, mode, symlink or submodule change, because a trace
   cannot see what a job would have done with a file that was absent, and a
   directory listing changes with membership.
2. No changed path is a control path: what decides which obligations exist,
   how they run, or which toolchain runs them.
3. The obligation runs on Linux and is traced. Native hosts always run: a
   Linux trace is no evidence about another host.
4. Exactly one receipt: a successful job of a same-repository run on the base
   commit, whose tree is the base tree.
5. Exactly that job's trace, decoded and `usable`, bound to the same
   repository, run, attempt, commit, tree and matrix row, recorded from a
   clean checkout by the expected recorder, unchanged from start to stop.
6. The job read no changed file, listed no changed file's directory, and read
   nothing under `.git` (which depends on the commit every change alters).

A trace's absence, or a trace that saw nothing, never authorizes anything.
Python 3.9+, standard library only.
"""

import argparse
import json
from pathlib import PurePosixPath
import subprocess
import sys

SCHEMA = "lexlean/ci-change-scope/1"
TRACE_SCHEMA = "lexlean/ci-read-trace/2"
TRACE_AUTHORITY = "advisory: observed opens of one run; never sufficient to skip an obligation"
MODE = "shadow: decides nothing; enforcement is phase 3 and needs the owner's approval"
RUN, REUSE = "run", "would-reuse"

# What decides which obligations exist and how they run (the workflows, the
# plan, the release gate, the recorder, the legs), or which toolchain runs
# them. A change to any of these runs everything.
CONTROL_PREFIXES = (".github/", ".cargo/")
CONTROL_FILES = frozenset((
    "Cargo.lock", "rust-toolchain", "rust-toolchain.toml", "lean-toolchain",
    "lake-manifest.json", "justfile", "deny.toml",
))
PURE_MODES = frozenset(("100644", "100755"))
TRUSTED_EVENTS = frozenset(("pull_request", "workflow_dispatch", "push"))


def control(path):
    return path in CONTROL_FILES or path.startswith(CONTROL_PREFIXES) or (
        PurePosixPath(path).name in CONTROL_FILES)


def parse_raw_diff(text):
    """The entries of `git diff --raw --no-abbrev` text output, one per line."""
    entries = []
    for line in text.splitlines():
        if not line.startswith(":"):
            continue
        meta, _, path = line.partition("\t")
        old_mode, new_mode, _old, _new, status = meta[1:].split(" ")
        entries.append({"status": status, "old_mode": old_mode, "new_mode": new_mode,
                        "path": path})
    return entries


def change_of(base, candidate, run=subprocess.run):
    """The change from BASE to CANDIDATE, as `git diff --raw` reports it."""
    done = run(["git", "diff", "--raw", "--no-renames", "--no-abbrev", base, candidate],
               capture_output=True, text=True, check=True)
    return parse_raw_diff(done.stdout)


def global_reasons(bundle):
    """Reasons that run every obligation, whatever the traces say."""
    reasons = []
    change = bundle.get("change")
    base, candidate = bundle.get("base") or {}, bundle.get("candidate") or {}
    if not base.get("tree") or not candidate.get("tree") or not base.get("commit"):
        reasons.append("base or candidate identity unknown")
    elif base["tree"] == candidate["tree"]:
        reasons.append("identical trees: the existing tree-verified rule applies, not this one")
    if not isinstance(change, list) or not change:
        reasons.append("no change entries")
        return reasons
    for entry in change:
        path = entry.get("path", "")
        if entry.get("status") != "M":
            reasons.append(f"{path}: status {entry.get('status')} is not a pure modification")
        elif entry.get("old_mode") != entry.get("new_mode") or entry.get("new_mode") not in PURE_MODES:
            reasons.append(f"{path}: mode {entry.get('old_mode')} -> {entry.get('new_mode')}")
        if control(path):
            reasons.append(f"{path}: control path")
    return reasons


def receipt_reasons(bundle, obligation, receipts):
    if len(receipts) != 1:
        return [f"{len(receipts)} receipts for this obligation"], None
    receipt = receipts[0]
    reasons = []
    if receipt.get("conclusion") != "success":
        reasons.append(f"receipt conclusion {receipt.get('conclusion')}")
    if receipt.get("head_repository") != bundle.get("repository"):
        reasons.append(f"receipt from {receipt.get('head_repository')}, not this repository")
    if receipt.get("event") not in TRUSTED_EVENTS:
        reasons.append(f"receipt event {receipt.get('event')}")
    if receipt.get("head_sha") != bundle["base"].get("commit"):
        reasons.append("receipt is not of the base commit")
    if receipt.get("tree") != bundle["base"].get("tree"):
        reasons.append("receipt tree is not the base tree")
    return reasons, receipt


def trace_reasons(bundle, obligation, receipt):
    trace = receipt.get("trace")
    if not isinstance(trace, dict):
        return ["no trace (absence never authorizes a reuse)"]
    reasons = []
    prov = trace.get("provenance") or {}
    github = prov.get("github") or {}
    checks = (
        (trace.get("schema") == TRACE_SCHEMA, "trace schema"),
        (trace.get("authority") == TRACE_AUTHORITY, "trace authority statement"),
        (trace.get("usable") is True and trace.get("state") == "stopped", "trace not usable"),
        (prov.get("tree") == bundle["base"].get("tree"), "trace tree is not the base tree"),
        (prov.get("head") == receipt.get("head_sha"), "trace commit is not the receipt's"),
        (prov.get("dirty") is False, "trace checkout not clean"),
        (prov.get("matrix") == obligation.get("row"), "trace matrix row is not the obligation's"),
        (github.get("GITHUB_REPOSITORY") == bundle.get("repository"), "trace repository"),
        (str(github.get("GITHUB_RUN_ID")) == str(receipt.get("run_id")), "trace run"),
        (str(github.get("GITHUB_RUN_ATTEMPT")) == str(receipt.get("attempt")), "trace attempt"),
        (prov.get("recorder") == trace.get("recorder_at_stop"), "recorder changed during the job"),
        (prov.get("recorder") == bundle.get("expected_recorder"), "unexpected recorder"),
        # A trace that saw no workspace input saw nothing: it is no evidence
        # that the job reads nothing.
        (((trace.get("counts") or {}).get("input") or 0) > 0 and bool(trace.get("files")),
         "trace observed no workspace input"),
    )
    reasons.extend(f"{what}" for ok, what in checks if not ok)
    return reasons


def dependency_reasons(bundle, trace):
    files, dirs = set(trace.get("files") or ()), set(trace.get("dirs") or ())
    reasons = []
    if trace.get("git"):
        reasons.append("the job read .git, which every change alters")
    for entry in bundle["change"]:
        path = entry["path"]
        parent = str(PurePosixPath(path).parent)
        if path in files:
            reasons.append(f"the job read {path}")
        if parent in dirs:
            reasons.append(f"the job listed {parent}, which holds {path}")
    return reasons


def decide(bundle):
    """The shadow decision for every obligation of BUNDLE."""
    shared = global_reasons(bundle)
    out = []
    for obligation in bundle.get("obligations") or ():
        oid = obligation.get("id")
        reasons = list(shared)
        if obligation.get("host") != "linux" or not obligation.get("traced"):
            reasons.append("native or untraced obligation: always runs")
        receipts = [r for r in bundle.get("receipts") or () if r.get("obligation") == oid]
        more, receipt = receipt_reasons(bundle, obligation, receipts)
        reasons.extend(more)
        if receipt is not None:
            trace = receipt.get("trace")
            more = trace_reasons(bundle, obligation, receipt)
            reasons.extend(more)
            if isinstance(trace, dict) and not more and isinstance(bundle.get("change"), list):
                reasons.extend(dependency_reasons(bundle, trace))
        out.append({"id": oid, "decision": RUN if reasons else REUSE, "reasons": reasons,
                    "receipt": None if receipt is None else {
                        k: receipt.get(k) for k in ("run_id", "attempt", "job_id")}})
    return {"schema": SCHEMA, "mode": MODE, "global": shared, "obligations": out}


def score(decision, actual):
    """Score a shadow DECISION against ACTUAL {obligation id: (conclusion, seconds)}
    from the candidate's own full gate. Unsound when any would-reuse failed."""
    unsound, missing, saved = [], [], 0.0
    for item in decision["obligations"]:
        if item["decision"] != REUSE:
            continue
        if item["id"] not in actual:
            missing.append(item["id"])
            continue
        conclusion, seconds = actual[item["id"]]
        if conclusion != "success":
            unsound.append(item["id"])
        else:
            saved += seconds
    return {"sound": not unsound and not missing, "unsound": unsound, "unscored": missing,
            "would_reuse": sum(i["decision"] == REUSE for i in decision["obligations"]),
            "obligations": len(decision["obligations"]),
            "would_save_runner_minutes": round(saved / 60, 3)}


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    one = sub.add_parser("decide")
    one.add_argument("bundle")
    two = sub.add_parser("change")
    two.add_argument("base")
    two.add_argument("candidate")
    three = sub.add_parser("score")
    three.add_argument("decision")
    three.add_argument("actual")
    arguments = parser.parse_args(argv)
    try:
        if arguments.command == "decide":
            with open(arguments.bundle, encoding="utf-8") as handle:
                result = decide(json.load(handle))
        elif arguments.command == "change":
            result = change_of(arguments.base, arguments.candidate)
        else:
            with open(arguments.decision, encoding="utf-8") as handle:
                decision = json.load(handle)
            with open(arguments.actual, encoding="utf-8") as handle:
                actual = {k: tuple(v) for k, v in json.load(handle).items()}
            result = score(decision, actual)
    except (OSError, ValueError, KeyError, subprocess.CalledProcessError) as error:
        print(f"ci-change-scope: {error}", file=sys.stderr)
        return 1
    print(json.dumps(result, indent=1, sort_keys=True))
    return 0


if __name__ == "__main__":
    sys.exit(main())

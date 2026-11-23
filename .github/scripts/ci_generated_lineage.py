#!/usr/bin/env python3
"""Audit generated-input write/delivery chain without authorizing merge CI.

This source-level contract intentionally reports incomplete transitive closure.
It may discover broken provenance paths, but can never waive a proof obligation.
"""
import argparse
import hashlib
import json
import sys
from pathlib import Path

from ci_obligation_audit import AuditHold, job_blocks, recipes_in, require

SOURCES = (
    "Justfile", ".github/workflows/ci.yml",
    ".github/workflows/regenerate.yml", ".github/workflows/cut.yml",
    ".github/workflows/forward.yml",
    ".github/scripts/deliver-regenerated.sh",
    ".github/scripts/await-gate.sh",
)
WRITE_RECIPES = {
    "production-export-write": "cargo xtask check-production-export --write",
    "verify-write": "cargo xtask verify-examples --write",
    "golden-write": "cargo xtask check-golden --write",
    "fixtures-write": "cargo xtask check-fixtures --write",
    "calculus-write": "cargo xtask check-calculus --write",
    "model-write": "cargo xtask validate-model --write",
}


def assert_lineage(source):
    just = recipes_in(source["Justfile"])
    for recipe, command in WRITE_RECIPES.items():
        require(recipe in just and just[recipe]["commands"] == [command],
                "generated writer recipe changed or missing: " + recipe)
    regen = job_blocks(source[".github/workflows/regenerate.yml"])
    require({"writer", "combine", "finish", "deliver"} <= set(regen),
            "regeneration job missing")
    writer = regen["writer"]
    for token in (
        "writer: [production-export, records, oracles]",
        'production-export) recipes="production-export-write"',
        'recipes="verify-write"',
        'oracles) recipes="golden-write fixtures-write calculus-write model-write"',
        "just --timestamp",
    ):
        require(token in writer, "regeneration writer selection changed: " + token)
    require("needs: [writer]" in regen["combine"] and
            "pattern: regenerate-part-*" in regen["combine"] and
            "name: regenerate-combined" in regen["combine"],
            "regeneration combine no longer consumes every writer")
    require("needs: [combine]" in regen["finish"] and
            "cargo xtask counts --write" in regen["finish"] and
            "cargo xtask validate-model --write" in regen["finish"] and
            "LEXLEAN_COUNTS=fast" in regen["finish"],
            "regeneration final oracle/count stage changed")
    require("needs: [finish]" in regen["deliver"] and
            "run: .github/scripts/deliver-regenerated.sh" in regen["deliver"] and
            "head:" in regen["deliver"] and "COMMIT:" in regen["deliver"],
            "regeneration delivery is not bound to final head")
    delivery = source[".github/scripts/deliver-regenerated.sh"]
    require('gh workflow run ci.yml --repo "$repo" --ref "$branch"' in delivery and
            'git push origin "HEAD:refs/heads/$branch"' in delivery,
            "generated commit not dispatched to exact branch CI")
    cut = job_blocks(source[".github/workflows/cut.yml"])
    require({"version", "regenerate", "gate", "gate-more", "land", "tag"} <= set(cut),
            "release-cut job missing")
    require("needs: version" in cut["regenerate"] and
            "uses: ./.github/workflows/regenerate.yml" in cut["regenerate"],
            "cut must invoke regeneration before gate")
    for job in ("gate", "gate-more"):
        require("run: .github/scripts/await-gate.sh" in cut[job],
                "cut wait stage lost evidence validator: " + job)
    require("needs: [version, regenerate, gate, gate-more]" in cut["land"] and
            "needs.gate.outputs.state == 'passed'" in cut["land"] and
            "needs.gate-more.outputs.state == 'passed'" in cut["land"] and
            "HEAD_SHA:" in cut["land"],
            "release cut could land without gate-pass receipts")
    require("needs: land" in cut["tag"] and
            "uses: ./.github/workflows/tag.yml" in cut["tag"],
            "tag must depend on the verified cut landing")
    waiter = source[".github/scripts/await-gate.sh"]
    require(".github/release-gate.sh" in waiter and
            "state" in waiter and "failed" in waiter,
            "cut gate waiter lost whole-tree validation")
    forward = job_blocks(source[".github/workflows/forward.yml"])
    require("merge" in forward and "regenerate" in forward and
            "needs: merge" in forward["regenerate"] and
            "needs.merge.outputs.clean == 'true'" in forward["regenerate"] and
            "uses: ./.github/workflows/regenerate.yml" in forward["regenerate"],
            "release-line forwarding lost re-generation")
    ci = job_blocks(source[".github/workflows/ci.yml"])
    require("honesty" in ci and "needs: [plan]" in ci["honesty"] and
            "run: cargo xtask check-freshness" in ci["honesty"],
            "generated oracle freshness no longer precedes qualification")
    return {
        "generated_writer_recipes": sorted(WRITE_RECIPES),
        "regeneration_stages": ["writer", "combine", "finish", "deliver"],
        "cut_stages": ["version", "regenerate", "gate", "gate-more", "land", "tag"],
        "forward_regeneration_checked": True,
        "ci_freshness_obligation_checked": True,
        "actual_generated_output_inventory_verified": False,
        "source_to_generated_input_closure_complete": False,
        "dispatch_proof_receipts_verified": False,
    }


def build(root):
    root = Path(root)
    try:
        data = {p: (root / p).read_bytes() for p in SOURCES}
        source = {p: raw.decode("utf-8") for p, raw in data.items()}
    except (OSError, UnicodeError) as error:
        raise AuditHold("missing or undecodable generated-input source") from error
    result = assert_lineage(source)
    return {
        "schema": "lexlean/generated-input-lineage/1",
        "classification": "AUDIT_ONLY_NO_QUALIFICATION_AUTHORITY",
        "sources_sha256": {p: hashlib.sha256(raw).hexdigest()
                           for p, raw in sorted(data.items())},
        **result,
        "blocked_on": [
            "enumerated output files and exact generator dependency edges",
            "runtime proof of freshness and changed-input closure",
            "release/branch protected status authority",
            "same-run host/Atlas and publication receipts",
        ],
    }


def main(argv=None):
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--root", default=".")
    args = p.parse_args(argv)
    try:
        print(json.dumps(build(args.root), indent=2, sort_keys=True))
    except AuditHold as error:
        print("HOLD: " + str(error), file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Nonqualifying LexLean developer-feedback selector, never merge authority.

Only the explicit preview recipe is selective. SPEC §9.4 continues to require
full exact-head qualification for ready PRs into main/release branches.
"""
import argparse
import hashlib
import json
import re
from pathlib import Path

from ci_cargo_scope import affected as cargo_affected

_MAX_BYTES = 1024 * 1024
_SHA = re.compile(r"[0-9a-f]{40}(?:[0-9a-f]{24})?\Z")
_STATUS = re.compile(r"(?:[AMDT]|[RC][0-9]{1,3})\Z")


def _path(token):
    try:
        path = token.decode("utf-8", errors="strict")
    except UnicodeDecodeError as exc:
        raise ValueError("non-UTF-8 tracked path: cannot classify") from exc
    if (not path or path.startswith("/") or "\\" in path or
            any(part in ("", ".", "..") for part in path.split("/"))):
        raise ValueError("unsafe or ambiguous tracked path")
    return path


def changed_paths(data):
    """Parse exact 'git diff --name-status -z -M' including both rename sides."""
    if not data:
        return []
    if len(data) > _MAX_BYTES or data[-1:] != b"\0":
        raise ValueError("invalid or oversized NUL-delimited diff")
    fields = data[:-1].split(b"\0")
    result = []
    at = 0
    while at < len(fields):
        try:
            status = fields[at].decode("ascii", errors="strict")
        except UnicodeDecodeError as exc:
            raise ValueError("unknown git status") from exc
        if not _STATUS.fullmatch(status):
            raise ValueError("unsupported git diff status: " + status[:16])
        count = 2 if status.startswith(("R", "C")) else 1
        if at + count >= len(fields):
            raise ValueError("truncated git diff record")
        result.extend(_path(x) for x in fields[at + 1:at + 1 + count])
        at += count + 1
    if len(result) > 10000:
        raise ValueError("too many tracked changes")
    return sorted(set(result))


def preview(base, head, diff, repo_root=None):
    """Select actual feedback actions, not protected verification obligations."""
    if not _SHA.fullmatch(base) or not _SHA.fullmatch(head):
        raise ValueError("expected exact Git commit identities")
    paths = changed_paths(diff)
    rust = any(p.endswith(".rs") for p in paths)
    python = any(p.startswith(".github/scripts/") and p.endswith(".py") for p in paths)
    # Anything that can change generated semantics, trust, release, model inputs,
    # or is not proven passive still requires the unchanged normative full gate.
    # Since ALL paths require the full gate before a merge, this flag is never false.
    risk = []
    for p in paths:
        if p.endswith(".rs") or (p.startswith(".github/scripts/") and p.endswith(".py")):
            continue
        risk.append(p)
    selected = (["diff-whitespace"] if paths else [])
    if rust:
        selected.append("rust-format")
    if python:
        selected.append("ci-python-contracts")
    cargo = cargo_affected(repo_root, paths) if repo_root is not None else {
        "cargo_package_roots": [], "cargo_check_packages": [],
        "cargo_unresolved_rust_paths": [], "cargo_manifest_sha256": None,
        "cargo_feedback_only": True,
    }
    if cargo["cargo_unresolved_rust_paths"]:
        risk = sorted(set(risk) | set(cargo["cargo_unresolved_rust_paths"]))
    if cargo["cargo_check_packages"]:
        selected.append("cargo-check")
    return {
        "schema": "lexlean/dev-feedback/1",
        "base": base,
        "head": head,
        "diff_sha256": hashlib.sha256(diff).hexdigest(),
        "changed": paths,
        "selected_feedback": selected,
        **cargo,
        "not_classified_for_fast_feedback": risk,
        "requires_normative_full_gate_for_merge": True,
        "qualification": "NONQUALIFYING",
        "unselected_normative_obligations": "NOT_VERIFIED_BY_PREVIEW",
    }


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base", required=True)
    parser.add_argument("--head", required=True)
    parser.add_argument("--diff-nul", type=Path, required=True)
    parser.add_argument("--repo-root", type=Path)
    parser.add_argument("--github-output", type=Path)
    parser.add_argument("--summary", type=Path)
    args = parser.parse_args(argv)
    try:
        diff = args.diff_nul.read_bytes()
        plan = preview(args.base, args.head, diff, repo_root=args.repo_root)
    except (OSError, ValueError) as exc:
        parser.exit(2, f"HOLD: {exc}\n")
    if args.github_output:
        with args.github_output.open("a", encoding="utf-8") as out:
            for name, enabled in (
                ("whitespace", "diff-whitespace" in plan["selected_feedback"]),
                ("rust_format", "rust-format" in plan["selected_feedback"]),
                ("python_contracts", "ci-python-contracts" in plan["selected_feedback"]),
                ("cargo_check", "cargo-check" in plan["selected_feedback"]),
                *(("cargo_" + name.replace("-", "_"),
                   name in plan["cargo_check_packages"])
                  for name in ("repo-model", "lexlean", "repo-conformance", "xtask")),
            ):
                out.write(f"{name}={'true' if enabled else 'false'}\n")
    if args.summary:
        # Never print tracked path text into Markdown without escaping it.
        with args.summary.open("a", encoding="utf-8") as out:
            out.write("### NONQUALIFYING development feedback\n\n")
            out.write(f"Selected real commands: {', '.join(plan['selected_feedback']) or 'none'}\n\n")
            out.write(f"Changed paths: {len(plan['changed'])}. Unclassified for fast feedback: {len(plan['not_classified_for_fast_feedback'])}.\n\n")
            out.write("**This result does not satisfy SPEC §9.4 or authorize merge/release.**\n")
            if plan["cargo_check_packages"]:
                out.write("\nCargo check packages: " + ", ".join(plan["cargo_check_packages"]) + "\n")
    print(json.dumps(plan, sort_keys=True, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Whether a fork pull request may be qualified as a same-repository
integration candidate (CI #603), and the exact steps if so. Decides and does
nothing on its own: a maintainer runs the steps, and pushing them is the
trust decision.

Why: the plan and the release gate reuse a run only when its head repository
is this repository (`head_repository.full_name == repo`), so a fork's gate is
admission evidence but never a reusable receipt, and landing it runs the whole
gate again after the merge. Pushing the *reviewed fork commit itself* to a
branch of this repository and opening a draft pull request from it gives one
same-repository run of that exact commit, which the existing predicates accept
without any change to them; merging the fork pull request with
`--match-head-commit` then lands a merge whose second parent is that commit.

What this must not weaken, and how it holds:

- Origin checks: unchanged. The receipt is a same-repository run because a
  maintainer put the commit in this repository; no fork run is trusted.
- Execution security: on a branch of this repository, the workflows that run
  are the branch's own, with this repository's trust. So a candidate whose
  change touches `.github/workflows/` or `.github/actions/` is refused: those
  are the only files that can ask for permissions or secrets. ci.yml and
  reproducibility.yml declare read-only tokens and use no secrets, and the
  branch name matches no other workflow's push trigger (checked here).
- Artifact custody: the run's artifacts are produced by this repository's
  workflows from the reviewed commit; nothing produced by a fork run is
  imported.
- Exactness: the reviewed SHA must equal the fork's head when checked, when
  pushed, and when merged (`--match-head-commit`). If the base moved, the
  merge's tree differs from the candidate's and the gate runs in full after
  the merge, as it should.

Python 3.9+, standard library only.
"""

import argparse
import json
import re
import subprocess
import sys

BRANCH = "integrate/pr-{number}"
PROTECTED = (".github/workflows/", ".github/actions/")


def glob_regex(pattern):
    """A GitHub branch filter pattern as a regular expression (`*`, `**`, `?`, `[...]`)."""
    out, index = [], 0
    while index < len(pattern):
        char = pattern[index]
        if pattern.startswith("**", index):
            out.append(".*")
            index += 2
            continue
        if char == "*":
            out.append("[^/]*")
        elif char == "?":
            out.append("[^/]")
        elif char == "[":
            end = pattern.index("]", index)
            out.append(pattern[index:end + 1])
            index = end + 1
            continue
        else:
            out.append(re.escape(char))
        index += 1
    return re.compile("".join(out) + r"\Z")


def flow_list(text, start):
    """The items of the YAML flow list opening at TEXT[START] == '[', quotes respected."""
    items, current, quote, index = [], "", None, start + 1
    while index < len(text):
        char = text[index]
        if quote:
            if char == quote:
                quote = None
            else:
                current += char
        elif char in "'\"":
            quote = char
        elif char == "[" and current.strip():
            # A bracket inside a bare item, as in `cut/v[0-9]*`.
            close = text.index("]", index)
            current += text[index:close + 1]
            index = close
        elif char in ",]":
            if current.strip():
                items.append(current.strip())
            current = ""
            if char == "]":
                return items
        else:
            current += char
        index += 1
    raise ValueError("unterminated flow list")


def push_patterns(workflow_text):
    """Every branch pattern a workflow's `on: push` admits; `**` when it admits
    every branch, or when the trigger cannot be read (fail closed)."""
    lines = workflow_text.splitlines()
    for number, line in enumerate(lines):
        if re.match(r"on:\s*(push\s*$|\[[^\]]*\bpush\b)", line):
            return ["**"]
        match = re.match(r"  push:(.*)$", line)
        if not match:
            continue
        section = [match.group(1)]
        for following in lines[number + 1:]:
            if following.strip() and not following.startswith("    "):
                break
            section.append(following)
        text = "\n".join(section)
        if "branches-ignore" in text or "branches" not in text:
            return [] if "tags" in text and "branches" not in text else ["**"]
        after = text[text.index("branches") + len("branches"):].lstrip(" :")
        if after.startswith("["):
            return flow_list(after, 0)
        patterns = [item.strip().strip("'\"") for item in re.findall(r"^\s+-\s+(.+)$", after, re.M)]
        return patterns or ["**"]
    return []


def check(pr, files, reviewed_sha, workflows):
    """(eligible, reasons, steps) for fork pull request PR.

    PR is the provider's pull request object, FILES the paths its change
    touches, REVIEWED_SHA the commit a maintainer reviewed, WORKFLOWS
    {name: text} of the base's workflow files."""
    reasons = []
    number = pr.get("number")
    head = pr.get("head") or {}
    base = pr.get("base") or {}
    head_repo = (head.get("repo") or {}).get("full_name")
    base_repo = (base.get("repo") or {}).get("full_name")
    sha = head.get("sha", "")
    branch = BRANCH.format(number=number)
    if pr.get("state") != "open":
        reasons.append(f"the pull request is {pr.get('state')}")
    if not head_repo or head_repo == base_repo:
        reasons.append("not a fork pull request: its own run is already a same-repository receipt")
    if not re.fullmatch(r"[0-9a-f]{40}", reviewed_sha or ""):
        reasons.append("no full reviewed commit SHA")
    elif reviewed_sha != sha:
        reasons.append(f"the fork's head {sha[:12]} is not the reviewed commit {reviewed_sha[:12]}")
    if not files:
        reasons.append("the change's file list is empty or unread")
    for path in files:
        if path.startswith(PROTECTED):
            reasons.append(f"{path}: would run with this repository's trust on a same-repository branch")
    if not workflows:
        reasons.append("the base's workflows are unread")
    for name, text in sorted(workflows.items()):
        try:
            patterns = push_patterns(text)
        except ValueError as error:
            reasons.append(f"cannot read the push trigger of {name}: {error}")
            continue
        for pattern in patterns:
            if glob_regex(pattern).match(branch):
                reasons.append(f"{branch} would trigger {name} (push: {pattern})")
    steps = [
        f"git fetch origin pull/{number}/head",
        f"test \"$(git rev-parse FETCH_HEAD)\" = {reviewed_sha}",
        f"git push origin {reviewed_sha}:refs/heads/{branch}",
        f"gh pr create --draft --base {base.get('ref')} --head {branch} "
        f"--title 'Integration candidate for #{number} at {reviewed_sha[:12]}'",
        "mark it ready once for the one qualifying gate",
        f"gh pr merge {number} --merge --match-head-commit {reviewed_sha}",
        f"close the candidate and delete {branch}",
    ]
    return not reasons, reasons, steps if not reasons else []


def gh(*arguments):
    return subprocess.run(["gh", "api", *arguments], capture_output=True, text=True,
                          encoding="utf-8", check=True).stdout


def gh_json(*arguments):
    return json.loads(gh(*arguments))


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", required=True)
    parser.add_argument("--pr", type=int, required=True)
    parser.add_argument("--reviewed-sha", required=True)
    arguments = parser.parse_args(argv)
    try:
        pr = gh_json(f"repos/{arguments.repo}/pulls/{arguments.pr}")
        files = gh("--paginate", f"repos/{arguments.repo}/pulls/{arguments.pr}/files?per_page=50",
                   "--jq", ".[].filename").splitlines()
        base = pr["base"]["sha"]
        listing = gh_json(f"repos/{arguments.repo}/contents/.github/workflows?ref={base}")
        workflows = {}
        for entry in listing:
            if entry["name"].endswith((".yml", ".yaml")):
                workflows[entry["name"]] = gh(
                    "-H", "Accept: application/vnd.github.raw",
                    f"repos/{arguments.repo}/contents/{entry['path']}?ref={base}")
    except (OSError, ValueError, KeyError, subprocess.CalledProcessError) as error:
        print(f"integration-candidate: cannot read the pull request: {error}", file=sys.stderr)
        return 1
    eligible, reasons, steps = check(pr, files, arguments.reviewed_sha, workflows)
    print(json.dumps({"eligible": eligible, "reasons": reasons, "steps": steps}, indent=1))
    return 0 if eligible else 2


if __name__ == "__main__":
    sys.exit(main())

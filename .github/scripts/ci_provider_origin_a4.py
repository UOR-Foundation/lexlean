#!/usr/bin/env python3
"""A4-3 read-only GitHub-origin metadata verifier. NEVER merge/release authority.

Trust boundary: only the live HTTPS GitHub REST path with a read-only token
can establish provider-observed identities. A caller-supplied JSON snapshot
is untrusted data. REST artifact metadata alone never proves which job uploaded
an archive or the artifact's bytes, test coverage or source dependency closure.
"""
import argparse
import json
import os
import re
import ssl
import urllib.error
import urllib.request
from pathlib import Path

RUN_SCHEMA = "lexlean/a4-provider-origin-policy/1"
OUT_SCHEMA = "lexlean/a4-provider-origin-observation/1"
REPO_RE = re.compile(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+\Z")
HEX_RE = re.compile(r"[0-9a-f]{40}\Z")
DIGEST_RE = re.compile(r"sha256:[0-9a-f]{64}\Z")
PATH_RE = re.compile(r"\.github/workflows/[A-Za-z0-9_.-]+\.ya?ml\Z")
MAX_JOBS = 1000
MAX_ARTIFACTS = 1000


class Hold(ValueError):
    """Unverified, ambiguous or inconsistent provider metadata."""


def require(condition, reason):
    if not condition:
        raise Hold(reason)


def number(value):
    return type(value) is int and value > 0


def expect(value, fields, label):
    require(isinstance(value, dict) and set(fields) <= set(value),
            "missing provider " + label + " fields")


def pinned_policy(policy):
    require(isinstance(policy, dict) and set(policy) ==
            {"schema", "repository", "repository_id", "run_id",
             "run_attempt", "head_sha", "workflow_id", "workflow_path",
             "event", "job_hosts", "artifact_names"}, "invalid origin policy")
    require(policy["schema"] == RUN_SCHEMA and
            isinstance(policy["repository"], str) and
            REPO_RE.fullmatch(policy["repository"]) is not None and
            number(policy["repository_id"]) and number(policy["run_id"]) and
            number(policy["run_attempt"]) and number(policy["workflow_id"]) and
            isinstance(policy["head_sha"], str) and
            HEX_RE.fullmatch(policy["head_sha"]) is not None and
            isinstance(policy["workflow_path"], str) and
            PATH_RE.fullmatch(policy["workflow_path"]) is not None and
            policy["event"] in ("pull_request", "workflow_dispatch", "push"),
            "untrusted origin identity")
    hosts = policy["job_hosts"]
    artifacts = policy["artifact_names"]
    require(isinstance(hosts, dict) and hosts and len(hosts) <= MAX_JOBS and
            all(isinstance(k, str) and k and len(k) < 300 and
                isinstance(v, str) and v and len(v) < 100
                for k, v in hosts.items()), "untrusted expected job names or hosts")
    require(isinstance(artifacts, list) and len(artifacts) <= MAX_ARTIFACTS and
            len(set(artifacts)) == len(artifacts) and
            all(isinstance(a, str) and a and len(a) < 300
                for a in artifacts), "untrusted artifact name inventory")
    return policy


def verified_metadata(policy, run, jobs, artifacts):
    """Validate closed provider objects. Callers cannot claim authenticated REST.

    This function is also used with synthetic adversarial fixtures, so its
    output explicitly says that transport authenticity has NOT been proven.
    """
    pinned_policy(policy)
    expect(run, ("id", "run_attempt", "head_sha", "head_repository",
                 "repository", "workflow_id", "path", "event",
                 "status", "conclusion"), "run")
    expect(run["repository"], ("id", "full_name"), "repository")
    expect(run["head_repository"], ("id", "full_name"), "head repository")
    require(run["id"] == policy["run_id"] and
            run["run_attempt"] == policy["run_attempt"] and
            run["head_sha"] == policy["head_sha"] and
            run["workflow_id"] == policy["workflow_id"] and
            run["path"] == policy["workflow_path"] and
            run["event"] == policy["event"] and
            run["repository"]["id"] == policy["repository_id"] and
            run["repository"]["full_name"] == policy["repository"] and
            run["head_repository"]["id"] == policy["repository_id"] and
            run["head_repository"]["full_name"] == policy["repository"],
            "foreign run, source, repository or workflow")
    require(run["status"] == "completed" and
            run["conclusion"] in ("success", "failure"),
            "origin run is not terminal success/failure")
    require(isinstance(jobs, list) and 0 < len(jobs) <= MAX_JOBS and
            isinstance(artifacts, list) and len(artifacts) <= MAX_ARTIFACTS,
            "incomplete or oversized job/artifact inventory")
    observed = {}
    unique_ids = set()
    for job in jobs:
        expect(job, ("id", "run_id", "run_attempt", "head_sha", "name",
                     "status", "conclusion", "labels", "runner_id"),
               "job")
        jid, name = job["id"], job["name"]
        require(number(jid) and jid not in unique_ids and
                isinstance(name, str) and name in policy["job_hosts"] and
                name not in observed and
                job["run_id"] == policy["run_id"] and
                job["run_attempt"] == policy["run_attempt"] and
                job["head_sha"] == policy["head_sha"], "duplicate/foreign job")
        unique_ids.add(jid)
        require(job["status"] == "completed" and
                job["conclusion"] in
                ("success", "failure", "cancelled", "skipped", "timed_out",
                 "action_required", "neutral"),
                "unsettled or invalid job status")
        labels = job["labels"]
        require(isinstance(labels, list) and all(isinstance(v, str) for v in labels)
                and len(labels) == len(set(labels)), "malformed runner labels")
        outcome = job["conclusion"]
        if outcome == "success":
            require(number(job["runner_id"]) and
                    policy["job_hosts"][name] in labels,
                    "successful job has unproven runner identity")
        # Failed or skipped jobs never become positive reused evidence.
        observed[name] = {
            "job_id": jid, "result": outcome,
            "host_label_expected": policy["job_hosts"][name],
            "host_label_observed": policy["job_hosts"][name] in labels,
            "runner_allocated": number(job["runner_id"]),
            "eligible_for_candidate_consideration": outcome == "success",
        }
    require(set(observed) == set(policy["job_hosts"]),
            "incomplete expected job inventory")

    names, ids, verified_artifacts = set(), set(), []
    for item in artifacts:
        expect(item, ("id", "name", "expired", "digest", "size_in_bytes",
                      "workflow_run"), "artifact")
        expect(item["workflow_run"],
               ("id", "repository_id", "head_repository_id", "head_sha"),
               "artifact workflow")
        ident, name = item["id"], item["name"]
        require(number(ident) and ident not in ids and
                isinstance(name, str) and name in policy["artifact_names"] and
                name not in names and
                item["expired"] is False and
                isinstance(item["digest"], str) and
                DIGEST_RE.fullmatch(item["digest"]) is not None and
                type(item["size_in_bytes"]) is int and item["size_in_bytes"] > 0 and
                item["workflow_run"]["id"] == policy["run_id"] and
                item["workflow_run"]["repository_id"] == policy["repository_id"] and
                item["workflow_run"]["head_repository_id"] == policy["repository_id"] and
                item["workflow_run"]["head_sha"] == policy["head_sha"],
                "invalid/foreign/expired/duplicate artifact identity")
        names.add(name)
        ids.add(ident)
        verified_artifacts.append({
            "artifact_id": ident, "name": name, "sha256_digest_claim": item["digest"],
            "artifact_archive_bytes_checked": False,
            "uploading_job_identity_proven": False,
        })
    require(names == set(policy["artifact_names"]),
            "missing expected artifact")
    return {
        "schema": OUT_SCHEMA,
        "qualification": "NONQUALIFYING_ORIGIN_METADATA_ONLY",
        "origin_transport": "UNVERIFIED_SNAPSHOT",
        "run_id": policy["run_id"], "run_attempt": policy["run_attempt"],
        "head_sha": policy["head_sha"], "repository": policy["repository"],
        "workflow_path": policy["workflow_path"],
        "origin_conclusion": run["conclusion"],
        "jobs": {key: observed[key] for key in sorted(observed)},
        "artifacts": sorted(verified_artifacts, key=lambda a: a["name"]),
        "job_inventory_complete": True, "artifact_inventory_complete": True,
        "source_dependency_closure": "NOT_PROVEN",
        "artifact_producer_binding": "NOT_PROVEN_BY_REST",
        "artifact_archive_byte_digest": "NOT_VERIFIED",
        "host_runtime_binary_attestation": "NOT_PROVEN",
        "release_or_current_head_authority": False,
    }


class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        raise Hold("API redirect refused")


def live_json(url, token):
    """Read only fixed GitHub REST API endpoints, never a user URL."""
    require(isinstance(url, str) and
            re.fullmatch(r"https://api\.github\.com/repos/"
                         r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+/actions/"
                         r"(?:runs/[0-9]+(?:/(?:jobs|artifacts)"
                         r"\?per_page=100&page=[0-9]+)?|runs/[0-9]+)\Z", url)
            is not None, "unexpected GitHub API endpoint")
    require(isinstance(token, str) and 16 <= len(token) <= 1000,
            "read-only GitHub credential absent")
    request = urllib.request.Request(
        url, headers={
            "Authorization": "Bearer " + token,
            "Accept": "application/vnd.github+json",
            "X-GitHub-Api-Version": "2022-11-28",
            "User-Agent": "lexlean-a4-readonly-observer",
        }, method="GET")
    opener = urllib.request.build_opener(
        urllib.request.HTTPSHandler(context=ssl.create_default_context()),
        NoRedirect())
    try:
        with opener.open(request, timeout=15) as response:
            require(response.status == 200, "unexpected provider response status")
            raw = response.read(5 * 1024 * 1024 + 1)
    except (urllib.error.HTTPError, urllib.error.URLError, TimeoutError,
            OSError) as error:
        raise Hold("GitHub origin API could not be verified") from error
    require(len(raw) <= 5 * 1024 * 1024, "oversized provider response")
    try:
        return json.loads(raw.decode("utf-8"),
                          object_pairs_hook=_no_duplicate,
                          parse_constant=_nonfinite)
    except (ValueError, UnicodeError) as error:
        raise Hold("malformed provider response") from error


def _no_duplicate(pairs):
    found = {}
    for key, value in pairs:
        require(key not in found, "duplicate provider field")
        found[key] = value
    return found


def _nonfinite(_):
    raise Hold("invalid provider number")


def fetch_live(policy, token, reader=live_json):
    """Fetch origin metadata directly, with complete pages, before classifying."""
    pinned_policy(policy)
    base = ("https://api.github.com/repos/" + policy["repository"]
            + "/actions/runs/" + str(policy["run_id"]))
    run = reader(base, token)
    def pages(kind, maximum):
        records, page = [], 1
        while True:
            data = reader(base + "/" + kind +
                          "?per_page=100&page=" + str(page), token)
            expect(data, ("total_count", kind), kind + " listing")
            total = data["total_count"]
            batch = data[kind]
            require(type(total) is int and 0 <= total <= maximum and
                    isinstance(batch, list) and len(batch) <= 100 and
                    len(records) + len(batch) <= total,
                    "invalid/oversized provider pagination")
            records.extend(batch)
            if len(records) == total:
                return records
            require(batch and page < (maximum + 99) // 100,
                    "truncated provider pagination")
            page += 1
    observations = verified_metadata(
        policy, run, pages("jobs", MAX_JOBS),
        pages("artifacts", MAX_ARTIFACTS))
    # Only this trusted read-only API path claims live observation. STILL no
    # authority to reuse old-HEAD test evidence on current HEAD.
    observations["origin_transport"] = "GITHUB_REST_LIVE_READ_ONLY"
    return observations


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--policy", type=Path, required=True)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args(argv)
    try:
        policy = json.loads(args.policy.read_text(encoding="utf-8"),
                            object_pairs_hook=_no_duplicate,
                            parse_constant=_nonfinite)
        report = fetch_live(policy, os.environ.get("GITHUB_TOKEN", ""))
        # This tool never creates a required check or imports old artifacts.
        encoded = json.dumps(report, indent=2, sort_keys=True) + "\n"
        if args.output:
            args.output.write_text(encoded, encoding="utf-8")
        else:
            print(encoded, end="")
        return 0
    except (Hold, ValueError, OSError) as error:
        parser.exit(2, "HOLD: " + str(error) + "\n")


if __name__ == "__main__":
    raise SystemExit(main())

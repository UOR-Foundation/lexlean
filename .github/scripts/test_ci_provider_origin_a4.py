"""Adversarial GitHub REST origin metadata validation, no network required."""
import copy
import unittest
from unittest import mock

import ci_provider_origin_a4 as a4

SHA = "a" * 40
DIGEST = "sha256:" + "b" * 64


def fixture():
    policy = {
        "schema": a4.RUN_SCHEMA, "repository": "UOR-Foundation/lexlean",
        "repository_id": 1335579506, "run_id": 45289630677, "run_attempt": 1,
        "head_sha": SHA, "workflow_id": 427297183,
        "workflow_path": ".github/workflows/development-feedback.yml",
        "event": "pull_request",
        "job_hosts": {"python": "ubuntu-24.04", "failed": "windows-2022"},
        "artifact_names": ["ci-before-after-evidence"],
    }
    run = {
        "id": policy["run_id"], "run_attempt": 1, "head_sha": SHA,
        "head_repository": {"id": policy["repository_id"], "full_name": policy["repository"]},
        "repository": {"id": policy["repository_id"], "full_name": policy["repository"]},
        "workflow_id": policy["workflow_id"], "path": policy["workflow_path"],
        "event": policy["event"], "status": "completed", "conclusion": "failure",
    }
    def job(name, jid, labels, outcome):
        return {"id": jid, "run_id": policy["run_id"], "run_attempt": 1,
                "head_sha": SHA, "name": name, "status": "completed",
                "conclusion": outcome, "labels": labels,
                "runner_id": 100 if outcome != "skipped" else None}
    jobs = [job("python", 51, ["ubuntu-24.04"], "success"),
            job("failed", 52, ["windows-2022"], "failure")]
    artifacts = [{
        "id": 14873706677, "name": "ci-before-after-evidence", "expired": False,
        "digest": DIGEST, "size_in_bytes": 12989,
        "workflow_run": {
            "id": policy["run_id"], "repository_id": policy["repository_id"],
            "head_repository_id": policy["repository_id"], "head_sha": SHA,
        },
    }]
    return policy, run, jobs, artifacts


def case(mutator=None):
    p, r, j, a = fixture()
    if mutator:
        mutator(p, r, j, a)
    return a4.verified_metadata(p, r, j, a)


class OriginMetadataContract(unittest.TestCase):
    def test_mixed_failed_origin_may_have_successful_jobs_but_no_authority(self):
        report = case()
        self.assertTrue(report["jobs"]["python"]["eligible_for_candidate_consideration"])
        self.assertFalse(report["jobs"]["failed"]["eligible_for_candidate_consideration"])
        self.assertFalse(report["release_or_current_head_authority"])
        self.assertEqual(report["origin_transport"], "UNVERIFIED_SNAPSHOT")
        self.assertFalse(report["artifacts"][0]["uploading_job_identity_proven"])

    def test_successful_live_metadata_has_no_attestation(self):
        p, r, j, a = fixture()
        r["conclusion"] = "success"
        j[1]["conclusion"] = "success"
        got = a4.verified_metadata(p, r, j, a)
        self.assertFalse(got["release_or_current_head_authority"])
        self.assertEqual(got["artifact_archive_byte_digest"], "NOT_VERIFIED")

    def test_forged_repo_id_or_head_refused(self):
        for target, field, value in [
            ("run", "head_sha", "d" * 40),
            ("run", "workflow_id", 1),
            ("run", "path", ".github/workflows/ci.yml"),
            ("run", "event", "push"),
            ("repository", "id", 13),
            ("head_repository", "full_name", "attacker/fork"),
        ]:
            with self.subTest(target=target,field=field):
                def change(_, run, __, ___):
                    (run if target == "run" else run[target])[field] = value
                with self.assertRaises(a4.Hold):
                    case(change)

    def test_bad_origin_attempt_and_state_hold(self):
        for field, value in [("run_attempt", 2), ("status", "in_progress"),
                             ("conclusion", "cancelled")]:
            with self.subTest(field=field):
                def change(_, run, __, ___):
                    run[field] = value
                with self.assertRaises(a4.Hold):
                    case(change)

    def test_duplicate_foreign_or_missing_job_hold(self):
        for kind in ("duplicate_job", "foreign_job", "missing_job",
                     "invalid_attempt", "missing_runner", "wrong_host"):
            with self.subTest(kind=kind):
                def change(_, __, jobs, ___):
                    if kind == "duplicate_job":
                        jobs.append(copy.deepcopy(jobs[0]))
                    if kind == "foreign_job":
                        jobs[0]["head_sha"] = "e" * 40
                    if kind == "missing_job":
                        jobs.pop()
                    if kind == "invalid_attempt":
                        jobs[0]["run_attempt"] = 2
                    if kind == "missing_runner":
                        jobs[0]["runner_id"] = None
                    if kind == "wrong_host":
                        jobs[0]["labels"] = ["macos-15"]
                with self.assertRaises(a4.Hold):
                    case(change)

    def test_unsettled_job_cannot_supply_success(self):
        def change(_, __, jobs, ___):
            jobs[0]["status"] = "in_progress"
        with self.assertRaises(a4.Hold):
            case(change)

    def test_skipped_job_never_positive(self):
        def change(_, __, jobs, ___):
            jobs[1]["conclusion"] = "skipped"
            jobs[1]["runner_id"] = None
        self.assertFalse(case(change)["jobs"]["failed"]["eligible_for_candidate_consideration"])

    def test_duplicate_or_wrong_origin_artifact_refused(self):
        for kind in ("duplicate", "expired", "digest", "head",
                     "repository", "size", "wrong_name", "absent"):
            with self.subTest(kind=kind):
                def change(_, __, ___, arts):
                    if kind == "duplicate": arts.append(copy.deepcopy(arts[0]))
                    if kind == "expired": arts[0]["expired"] = True
                    if kind == "digest": arts[0]["digest"] = "not-a-hash"
                    if kind == "head": arts[0]["workflow_run"]["head_sha"] = "f" * 40
                    if kind == "repository": arts[0]["workflow_run"]["repository_id"] = 0
                    if kind == "size": arts[0]["size_in_bytes"] = 0
                    if kind == "wrong_name": arts[0]["name"] = "foreign"
                    if kind == "absent": arts.clear()
                with self.assertRaises(a4.Hold):
                    case(change)

    def test_expected_job_roster_must_be_complete(self):
        def change(p, _, __, ___):
            p["job_hosts"]["third"] = "ubuntu-24.04"
        with self.assertRaisesRegex(a4.Hold, "incomplete"):
            case(change)

    def test_policy_refuses_unknown_identity(self):
        for field, value in (("run_id", 0), ("run_attempt", False),
                             ("head_sha", "main"),
                             ("repository", "evil/repo/extra"),
                             ("workflow_path", "../../evil.yml")):
            with self.subTest(field=field):
                def change(p, _, __, ___):
                    p[field] = value
                with self.assertRaises(a4.Hold):
                    case(change)

    def test_live_reader_uses_pinned_endpoint_and_closes_pages(self):
        p, r, jobs, arts = fixture()
        seen = []
        def get(url, token):
            self.assertEqual(token, "testtoken" * 3)
            seen.append(url)
            if url.endswith("/jobs?per_page=100&page=1"):
                return {"total_count": 2, "jobs": jobs}
            if url.endswith("/artifacts?per_page=100&page=1"):
                return {"total_count": 1, "artifacts": arts}
            return r
        got = a4.fetch_live(p, "testtoken" * 3, reader=get)
        self.assertEqual(got["origin_transport"], "INJECTED_READER_UNVERIFIED")
        self.assertFalse(got["provider_pages_checked_live"])
        self.assertEqual(got["expected_inventory_trust"],
                         "CALLER_DECLARED_NOT_GOVERNED")
        self.assertEqual(len(seen), 3)
        self.assertTrue(all(url.startswith("https://api.github.com/repos/UOR-Foundation/lexlean/actions/runs/")
                            for url in seen))
        self.assertFalse(got["release_or_current_head_authority"])

    def test_missing_or_truncated_provider_page_refuses(self):
        p, run, jobs, arts = fixture()
        def get(url, _):
            if url.endswith("/jobs?per_page=100&page=1"):
                return {"total_count": 3, "jobs": jobs}
            if url.endswith("/jobs?per_page=100&page=2"):
                return {"total_count": 3, "jobs": []}
            if url.endswith("/artifacts?per_page=100&page=1"):
                return {"total_count": 1, "artifacts": arts}
            return run
        with self.assertRaisesRegex(a4.Hold, "pagination"):
            a4.fetch_live(p, "token", reader=get)

    def test_wrong_protocol_or_redirect_sends_no_credential(self):
        for url in ("https://evil.com/repos/attacker/repo/actions/runs/4",
                    "http://api.github.com/repos/UOR-Foundation/lexlean/actions/runs/1",
                    "https://api.github.com/repos/UOR-Foundation/lexlean/actions/runs/1/../../../bad"):
            with self.subTest(url=url):
                with self.assertRaises(a4.Hold):
                    a4.live_json(url, "token" * 10)
        with self.assertRaisesRegex(a4.Hold, "redirect"):
            a4.NoRedirect().redirect_request(None, None, 302, "Found", {}, "https://evil.com")

    def test_unhashable_artifact_policy_is_hold_not_exception(self):
        p, run, jobs, artifacts = fixture()
        p["artifact_names"] = [{"name": "bad"}]
        with self.assertRaisesRegex(a4.Hold, "artifact name"):
            a4.verified_metadata(p, run, jobs, artifacts)

    def test_injected_reader_cannot_claim_live_provider(self):
        p, run, jobs, artifacts = fixture()
        def injected(url, token):
            if "/jobs?" in url:
                return {"total_count": 2, "jobs": jobs}
            if "/artifacts?" in url:
                return {"total_count": 1, "artifacts": artifacts}
            return run
        result = a4.fetch_live(p, "synthetic", reader=injected)
        self.assertEqual(result["origin_transport"],
                         "INJECTED_READER_UNVERIFIED")
        self.assertFalse(result["provider_pages_checked_live"])
        self.assertFalse(result["release_or_current_head_authority"])

    def test_no_auth_token_or_invalid_json_is_rejected(self):
        with self.assertRaisesRegex(a4.Hold, "credential"):
            a4.live_json("https://api.github.com/repos/UOR-Foundation/lexlean/actions/runs/1", "")
        for contents in ('{"x":1,"x":2}', '{"x":NaN}'):
            with self.assertRaises(a4.Hold):
                __import__("json").loads(contents, object_pairs_hook=a4._no_duplicate,
                                         parse_constant=a4._nonfinite)

    def test_rerun_inconsistent_attempt_is_refused(self):
        def change(p, _, __, ___):
            p["run_attempt"] = 2
        with self.assertRaises(a4.Hold):
            case(change)

    def test_boolean_ids_are_not_valid_identity(self):
        def change(_, __, jobs, ___):
            jobs[0]["id"] = True
        with self.assertRaises(a4.Hold):
            case(change)

    def retry_fixture(self):
        p, r, old, arts = fixture()
        for job in old:
            job.update(created_at="2026-11-24T00:00:00Z",
                       started_at="2026-11-24T00:01:00Z",
                       completed_at="2026-11-24T00:10:00Z")
        p["run_attempt"] = 2
        first = copy.deepcopy(r)
        r.update(run_attempt=2, conclusion="success")
        new = copy.deepcopy(old)
        new[0].update(id=61, run_attempt=2, created_at="2026-11-24T00:11:00Z")
        new[1].update(id=62, run_attempt=2, created_at="2026-11-24T00:11:00Z",
                      started_at="2026-11-24T00:12:00Z",
                      completed_at="2026-11-24T00:22:00Z", conclusion="success")
        return p, r, first, old, new, arts

    def retry_read(self, mutate=None):
        p, r, first, old, new, arts = self.retry_fixture()
        latest = copy.deepcopy(new)
        if mutate:
            mutate(r, first, old, new, latest)
        def reader(url, token):
            if "/artifacts?" in url:
                return {"total_count": len(arts), "artifacts": arts}
            if "/jobs?" in url:
                rows = old if "/attempts/1/" in url else new if "/attempts/2/" in url else latest
                return {"total_count": len(rows), "jobs": rows}
            if url.endswith("/attempts/1"):
                return first
            return r
        return a4.fetch_live(p, "synthetic", reader=reader)

    def test_retry_read_preserves_original_job_id(self):
        report = self.retry_read()
        self.assertEqual(report["jobs"]["python"]["original_execution_job_id"], 51)
        self.assertEqual(report["jobs"]["failed"]["original_execution_job_id"], 62)
        self.assertEqual(report["attempt_lineage"]["retained_successes"], 1)
        self.assertEqual(report["attempt_lineage"]["completed_execution_minutes"], 28)
        self.assertFalse(report["release_or_current_head_authority"])
        self.assertFalse(report["attempt_lineage"]["provider_authenticity_verified"])

    def test_retry_read_refuses_ambiguous_history(self):
        cases = (
            lambda r, first, old, new, latest: first.update(head_sha="b"*40),
            lambda r, first, old, new, latest: old.pop(),
            lambda r, first, old, new, latest: new[0].update(id=51),
            lambda r, first, old, new, latest: latest[0].update(id=99),
            lambda r, first, old, new, latest: (
                new[0].update(runner_id=999), latest[0].update(runner_id=999)),
            lambda r, first, old, new, latest: r.update(conclusion="failure"),
        )
        for mutate in cases:
            with self.subTest(mutate=mutate):
                with self.assertRaises(a4.Hold):
                    self.retry_read(mutate)


if __name__ == "__main__":
    unittest.main()

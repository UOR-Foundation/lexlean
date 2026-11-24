# A4-3 provider origin metadata preflight, NOT reusable proof (#603)

This is an intentionally **nonqualifying** read-only tool. It verifies
provider-observed GitHub run/job/artifact metadata via a pinned repository,
exact run ID, attempt, HEAD SHA, workflow ID/path, host label inventory, and
expected artifact names. No writes, workflow dispatches, status creation,
artifact imports, or network access in tests.

The CLI reads a committed/audited policy and calls GitHub's official
REST endpoints using GITHUB_TOKEN (read-only Actions permission).
It refuses redirects, unknown hosts, missing expected jobs, unclosed
pagination, foreign forks/repositories, stale attempts, nonterminal runs,
unexpected workflows and expired/missing/invalid artifact digests. A failed
origin run may contain successful individual jobs; only completed-success
jobs can be *candidates* for later scrutiny. Every other result is negative.

**Critical limitations:** A GitHub REST artifact.digest is a provider claim
about the archive, not a locally checked ZIP hash. The artifacts endpoint
does not itself bind a particular upload job to that artifact. Run and job
metadata do not prove native runtime binary identity, test/fixture/source
closure, or supply a verified complete cross-HEAD release proof. The tool
returns release_or_current_head_authority=false for every outcome.

Example read-only invocation using a pinned review-approved JSON policy:

    GITHUB_TOKEN=... python3 .github/scripts/ci_provider_origin_a4.py \
      --policy .github/ci-a4-origin-policy.json

Do not commit credentials or invent artifact identity hashes. The policy
must name repository, repository_id, run_id, run_attempt, head_sha,
workflow_id, workflow_path, event, exact job_hosts map and artifact_names.
Schema: lexlean/a4-provider-origin-policy/1. No policy is shipped in this
phase because an origin run must be chosen and approved separately.

**A4-3 remains OPEN** until authenticated producing-job custody and artifact
bytes/attestation are verified, with adversarial acceptance in a separate
approved current-HEAD workflow. The existing SPEC §9.4 exact-head gate,
native/Lean/Atlas receipts and release-gate.sh remain authoritative.

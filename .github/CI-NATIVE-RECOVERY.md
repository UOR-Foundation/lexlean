# CI host interruption: exact-head, single-job recovery

**Scope:** Read-only recovery runbook for [CI issue #603](https://github.com/UOR-Foundation/lexlean/issues/603), not a change to normative CI, SPEC, required checks, release rights or host verifiers.

## Incident and admissible recovery

On [full release CI #45170496660](https://github.com/UOR-Foundation/lexlean/actions/runs/45170496660), commit `f57650e6cf1fd04df0858d7bb97b1071a1c83220`, **77 of 78 jobs completed successfully**. The Linux ARM job, `build and non-Lean tests (aarch64-unknown-linux-gnu)`, ran on the required architecture but received a runner shutdown signal mid-`cargo test` (after 2,763 seconds). Its failure is not a test-assertion verdict; the run **must remain unqualified** unless the exact host obligation actually passes.

One manual, provider-native **rerun of the failed job ID `137680959434`** was requested after verifying the original run had a single failure and remained at the exact live release SHA. GitHub advanced the same run to attempt 2; the only newly executing host job was `137750931489`. This is an allowed operational experiment, **not** a general auto-retry policy.

## Critical attempt/receipt semantics

GitHub presents **78 job records for both attempts** and assigns **new job IDs to all 78 records** in attempt 2. The 77 successful job receipts have **exactly identical original started/completed UTC times** and conclusions; they were not newly executed, despite their new IDs and `run_attempt: 2` values. The Linux ARM record alone has a later start. Therefore, compare job names, status, exact timestamps and source SHA, not job ID deltas or the mere count of returned attempt-2 rows.

The original 77 successes represent **1129.950 historical unweighted runner-job minutes** that a fresh full-matrix run would need to execute again, if it had the same workload. This is *avoided duplicate work*, not a GitHub invoice discount or proof that an alternative full run would take exactly that long.

## Manual procedure, one attempt at a time

1. Confirm the original run is completed, the release branch still points to its exact SHA, and exactly one host job failed due to an interruption rather than a product verdict. Review the exact step log and provenance before requesting anything.
2. Inspect the current workflow dependency graph. For this observed commit no job has `needs: [host-matrix]`, so the isolated ARM retry does not require downstream work. Do not generalize this to an altered workflow without reviewing it again.
3. Rerun **only the failed job**, not all jobs or the full workflow; do not enable automatic retries. Record the new attempt and the new native job ID.
4. Page through `attempts/1/jobs` and `attempts/2/jobs` completely. Refuse success on a missing/duplicate required status, changed original success timestamp, additional newly executing job, wrong SHA, wrong host or pending/failed ARM test.
5. Even after a green attempt, verify all 78 required jobs, proof/Atlas join, supported-host status list, exact SHA and independent reproducibility context. The existing `.github/release-gate.sh` reads the latest jobs listing; do **not** weaken its all-success and four-host checks.
6. A re-run does **not** retroactively restart a completed `tag.yml` workflow. No tag, publish or carry-to-main action is authorized by this runbook; those have their own explicit existing gates.

## Observed receipt and disposition

The complete paginated job snapshot is stored in [`.github/metrics/ci-native-targeted-retry-20261123.json`](metrics/ci-native-targeted-retry-20261123.json). At snapshot, the run is **IN_PROGRESS** with ARM in_progress; no exact-run success is claimed. The old 77 successful job execution times were preserved. The final result must be checked separately before any release status claim.

Do **not** replace an interrupted host with `continue-on-error`, a zero-test pass, a different architecture, a skipped matrix row, an unapproved required-check exemption or a historical green job from another commit.

## Read-only retry-verification implementation and qualification (2026-11-23)

Run `python3 .github/scripts/ci_retry_receipts.py --ledger
.github/metrics/ci-native-targeted-retry-20261123.json` on the frozen
attempt snapshot. This utility checks run and exact SHA, 78 unique
required job names, attempt identity, prior successful execution timestamps,
single new ARM execution and false-green verdicts. It never calls GitHub,
retries jobs or authorizes release. The captured attempt is still marked
PENDING, regardless of a later live result until refreshed from GitHub.

Hosted [nonqualifying run #45195912523](https://github.com/UOR-Foundation/lexlean/actions/runs/45195912523)
passed 225 Python tests, including all 14 new adversarial cases and the
real in-repository attempt snapshot integration. Temporary opt-in marker
removed. Do not treat developer-feedback green as a replacement for an
uninterrupted native host conformance verdict or release-gate.sh.

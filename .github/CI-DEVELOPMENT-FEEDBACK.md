# CI development feedback: Phase B pilot (nonqualifying)

**Owner issue:** [#603](https://github.com/UOR-Foundation/lexlean/issues/603).
**Baseline:** full qualified PR #602 used **1,051.85 unweighted completed runner-job minutes** across main CI and standalone reproducibility. This is an observed starting point, not a predicted saving from this pilot.

This is an **optional, explicitly invoked** feedback workflow, independent of the normative `ci.yml` and `reproducibility.yml`. It **never** replaces SPEC.md §9.4, required checks, exact-HEAD native/proof/release evidence, or RP-13/14. Do not add its status to required merge checks. It has **no push or synchronize trigger**. Its only PR trigger is a deliberately opted-in **draft PR body edit**; routine draft/ready PR pushes allocate no runner for this pilot.

## What runs

The workflow `.github/workflows/development-feedback.yml` checks out the exact manually dispatched HEAD without write credentials, fetches the authorized `main` or `release/vX.Y` baseline, requires that baseline to be an ancestor, and computes the actual NUL-delimited, rename-aware `git diff` against that base. An unavailable/stale/foreign base or malformed diff **fails**, not a guessed partial plan.

The stdlib-only `.github/scripts/ci_impact_preview.py` chooses real actions within a **single** bounded Ubuntu worker:

- Changed tracked files: `git diff --check` for whitespace mistakes.
- Any changed, renamed or deleted Rust source (`.rs`): pinned-toolchain `cargo fmt --all -- --check`.
- Any changed CI Python script/test under `.github/scripts/`: actual `unittest discover` for the existing CI Python contract tests.
- Any changed Rust source within one of the four known Cargo packages: read the **current workspace + all four crate manifests**, validate the declared Cargo members and internal path dependencies, derive the reverse **transitive downstream package closure**, and run exactly one `cargo check --all-targets --all-features` call with only those `-p` packages. For example, `xtask/src/*` changes select only `xtask`; compiler changes select `lexlean`, `repo-conformance` and `xtask`. This compiles selected targets but **does not execute the test or proof suites**.
- A new crate, malformed/missing manifest, unknown inherited or local path dependency, or changed workspace membership/exclusion **fails the preview** instead of blessing a partial dependency graph. Unknown `.rs` files outside the four known package trees are reported as unresolved and never treated as safely skipped.
- Other paths (including model or generated inputs, README, SPEC, policies, lockfiles, toolchains, unknowns): **not proven unaffected** and not classified as harmless. The preview still performs available cheap checks but explicitly leaves the full normative obligations unverified.

This **is not yet a transitive dependency-to-verifier graph**. Its small, conservative classification demonstrates selected execution, actual commands and explicit unknowns without changing the protected full workflow. Once the project-specific impact closure and threat/detector map have passed negative controls, Phase B can compare deeper graph selection against stage placement and conventional work-reuse alternatives.

## Use and verification

When the workflow file is available on the repository's **default branch**, manually dispatch it via **Actions → CI development feedback (NONQUALIFYING) → Run workflow**, selecting the source branch and specifying `base_ref`. GitHub Actions requires a `workflow_dispatch` workflow to exist on default before manual runs are available; **this draft PR does not make that route available**.

To perform an explicitly requested, bounded **hosted evaluation while this PR remains draft**, edit its PR description and put the exact token `<!-- ci-preview-once -->` on **the first line**, then save the edit. The `pull_request: edited` event runs **one 20-minute-max diagnostic job** only when the PR is draft, the head is in this repository, and that exact marker begins the body. The existing normative `ci.yml` and `reproducibility.yml` do not subscribe to `edited`, so no full matrix is started. **Remove the token after the observation**; ordinary synchronize/push events never trigger the preview, and an unmarked body edit allocates no runner. A PR event checks out the exact PR head and refuses a base revision that has moved since the event.

The preview requires **Python 3.11+** for standard-library TOML parsing (`tomllib`), provided on Ubuntu 24.04.

To validate without any hosted runner:

```sh
python3 -m unittest discover -s .github/scripts -p 'test_ci_impact_preview.py' -v
```

The local tests plant Rust deletion, cross-language rename, CI Python, model/README/lockfile changes, type-changed/nonregular files, bad statuses, truncated NUL diffs, invalid paths, stale/malformed revision IDs, false green workflow triggers, Cargo downstream dependency closure, workspace member/alias/path drift, and missing required boundary. An isolated green test suite is not live provider execution or economic outcome proof.

## Next gated phase

Before replacing any `ci.yml` ready-PR or release path: inventory every terminal command/required context, build the trusted source→transitive obligation map, reconcile RP-13/14 and SPEC against actual source, compare two or more alternative graphs and seeded detection failures, agree on required-check migration, and prove actual like-for-like runner-minute and critical-path savings. **Keep the full gate unchanged until those controls are met.**


## Before/after Actions economics report

A reproducible, offline Python-stdlib report generator is committed as
`.github/scripts/ci_metrics_report.py`; source evidence is two immutable
historical capture files under `.github/metrics/`. They cover the last **50
merged PRs in the captured pre-Phase-B-cutover window**, November 14–23, 2026:
128 workflow runs attributed to the exact **final PR head SHA**. Job inventories
include **all attempts** and were retrieved in paginated batches, including
older 100+ job workflows.

For an opt-in developer-feedback run that changes CI Python, the runner creates
`ci-before-after-evidence` as a short-lived artifact containing:
`runner-minutes-per-pr.svg`, `final-head-time-to-green.svg`,
`ci-before-after-prs.csv`, `ci-before-after-summary.json` and `README.md`.
Both graph panels have fixed 50-PR horizontal space. **After remains visibly
N/A**, not a fabricated zero, until the actual production cutover has qualified
PRs. Independently regenerate without GitHub Actions:

```sh
python3 .github/scripts/ci_metrics_report.py \
  --before-runs .github/metrics/ci-before-pr50-runs.json \
  --before-costs .github/metrics/ci-before-pr50-job-costs.json \
  --output-dir ci-evidence/ci-before-after
python3 -m unittest discover -s .github/scripts -p 'test_ci_metrics_report.py' -v
```

When the normative production CI migration is merged and proven, snapshot the
**first up to 50 comparable merged PRs after that cutover**, preserving exact
source/run/job receipts. Supply `--after-runs`, `--after-costs`, and the
actual `--cutover-utc`; mixed windows or pre-cutover after samples are refused.
Do not pretend the optional feedback workflow itself is the cutover.

**Metric contract:** The graph's usage minutes are the sum of each completed
GitHub job's `completed_at - started_at` times, by runner platform. They are
**unweighted, not GitHub's billed minutes or dollars**. A public repository's
GitHub run timing endpoint may report 0 *billable* minutes even when thousands
of runner-job minutes were consumed. No billable/minute budget improvement is
asserted without independent billing evidence. The time-to-green graph measures
the interval from the first non-skipped, final-HEAD `ci` or `reproducibility`
PR run to the point **both** had successful conclusions *before merge*; cases
without this evidence are N/A, not 0. This is **not** a claim that branch
protection actually prevented merging, nor the total time from PR creation.
It is useful evidence about CI latency with that precise scope.

A PR can have older heads and postmerge workflows that the captured last-HEAD
query does not count. These are explicitly excluded; the 50-PR total is an
**observed attributed scope, not total organization Actions spend**.
Compare PR base branch, change type, CI workflow revision, host mix, and
qualification class before interpreting a percent difference as causal ROI.
Do not publish an after/before improvement from unrelated cohorts.

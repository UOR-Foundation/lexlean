# LexLean Actions economics: before/after evidence ledger

**Owner issue:** [#603](https://github.com/UOR-Foundation/lexlean/issues/603).
**Implementation:** [draft PR #606](https://github.com/UOR-Foundation/lexlean/pull/606).
**Current hosted report:** [successful run #45104134139](https://github.com/UOR-Foundation/lexlean/actions/runs/45104134139), exact source HEAD `3547cb158e82191c131f8d02566e8cb32ae18214`, [six-file evidence artifact](https://github.com/UOR-Foundation/lexlean/actions/runs/45104134139/artifacts/14786995941) (expires 2026-12-07). **All 88 Python tests and graph generation passed**, using the pinned historical snapshots. The artifact contains the new estimated private-cost chart, observed runner-minutes chart, premerge time-to-green chart, CSV, report summary and Markdown README. This is hosted verification of the report, not a measured postcutover change in mandatory CI.

## Evidence ledger

| Metric | Before (50 merged PRs) | After production cutover |
| --- | ---: | --- |
| Captured merged PR cohort | 50 | **NOT YET AVAILABLE** |
| Final-HEAD matched workflow runs | 128 | N/A |
| Unweighted runner-job minutes, all matched workflows | **99,321.66** | N/A |
| Median runner-job minutes per merged PR | **1,873.40** | N/A |
| p90 runner-job minutes per merged PR | **3,225.22** | N/A |
| Linux job-minutes | **77,326.62** | N/A |
| Windows job-minutes | **8,055.18** | N/A |
| macOS job-minutes | **13,939.85** | N/A |
| **Estimated private-repository standard-runner compute equivalent** | **$1,408.78** | **NOT YET AVAILABLE** |
| Actual public-repo standard-hosted compute charge | **$0** | N/A |
| PRs with both PR-triggered CI and repro green before merge | **29 / 50** | N/A |
| Median final-HEAD CI time-to-green (the 29 qualifying PRs) | **186.23 min** | N/A |
| p90 final-HEAD CI time-to-green (the 29 qualifying PRs) | **450.96 min** | N/A |
| Provider-billed minutes / actual monetary budget | **NOT ESTABLISHED** | **NOT ESTABLISHED** |
| Measured causal reduction after replacement | **NOT MEASURED** | **NOT MEASURED** |

**Historical window:** most recent 50 merged PRs when captured, from November
14 through November 23, 2026 UTC. The two source snapshots contain exact PR
HEADs, run IDs, run conclusions, timestamps, all-attempt paginated job counts,
platform labels and elapsed durations. All 128 run job inventories are
complete. A handful of older runs had more than 100 jobs and required pagination.

**Shadow-cost methodology:** The **$1,408.78** is a rate-weighted estimate, not a bill or cost savings: 77,326.62 pooled Linux elapsed job-minutes × $0.006/min + 8,055.18 Windows minutes × $0.010/min + 13,939.85 macOS minutes × $0.062/min (published 2026 standard private-repository rates). Historical Linux ARM minutes are pooled with x64, so the x64 rate is a proxy; per-job whole-minute billing rounding cannot be reconstructed from the aggregate source, and storage, larger runners and monthly included compute allowances are excluded. Depending on platform mix and rounding, an actual private-repo charge would differ. [GitHub Actions runner pricing](https://docs.github.com/en/billing/reference/actions-runner-pricing) and [billing rules](https://docs.github.com/en/billing/concepts/product-billing/github-actions) are the authoritative rate sources. **Public standard runners are free, but the workload still consumed substantial compute.**

**Scopes and caveats:** This is **not** organization-wide GitHub Actions spend,
or a complete PR-lifetime total. The baseline only attributes workflow runs
matching each PR's *final HEAD* and does not include earlier revision attempts
or postmerge push/cut workflows; those omission risks bias downward. The 50 PRs
span different base branches and CI workflow designs. A one-run GitHub timing
API response recorded **0 billable milliseconds** despite substantial measured
runner activity. Therefore no weighted billing, dollars, budget utilization or
provider cost savings can be inferred from the job timestamp sample.

**Waiting metric:** only PR-triggered, exact-final-HEAD `ci` and
`reproducibility` success completed **before the PR merge** count as observed
green. A successful `workflow_dispatch` run is not automatically substituted
for the required PR context. The start is the earliest non-skipped
PR-triggered verification run on that final head. The terminal green time is
when the last of the two required workflow conclusions is recorded; duration
is the difference. The other **21/50 PRs** are not treated as instantaneous;
their premerge qualification result is **N/A**. Historical branch protection
could not be read and these timestamps do not prove the CI checks blocked
merging. We also do not mistake PR creation-to-merge time for CI waiting.

## Graphs and reproducibility

The updated report generates three **separate, same-scale before/after SVGs**:
`runner-minutes-per-pr.svg`, `private-equivalent-cost-per-pr.svg`, and `final-head-time-to-green.svg`, plus
`ci-before-after-prs.csv`, machine-readable summary JSON and a README.
The after panels are intentionally blank, not a zero or guessed forecast.

Regenerate offline from the source snapshots using:

```sh
python3 .github/scripts/ci_metrics_report.py \
  --before-runs .github/metrics/ci-before-pr50-runs.json \
  --before-costs .github/metrics/ci-before-pr50-job-costs.json \
  --output-dir ci-evidence/ci-before-after
```

The **read-only, on-demand** `.github/scripts/ci_metrics_collect.py` can
refreeze the **last 50 PRs before** the actual future cutover and capture the
**first 50 PRs after** it. Use exactly the same cutover UTC boundary for both;
the current frozen before window is only an early baseline until a production
cutover is actually authorized and merged. It requires `GITHUB_TOKEN` for
read-only API access, performs only GET requests, respects a maximum API-call
budget, and writes complete source snapshots or fails. It never schedules or
launches an Actions workflow. Feed both complete cohorts into
`ci_metrics_report.py` with `--after-runs`, `--after-costs` and
`--cutover-utc`.

**Comparison gate:** Do not assert an after/before percentage as causal ROI
until sufficient representative PRs have landed, source-change and release
cohorts have been matched or stratified, protection/unique-failure parity has
been demonstrated, and both job-minutes and time-to-green are materially
better on comparable actual runs.

## Engineering objective still open

The optional preview's **one job** passed hosted tests and genuine
change-conditional Python/whitespace execution; targeted Rust package
selection is implemented and tested synthetically but not yet exercised by a
hosted Rust-source change. None of this replaces `SPEC.md §9.4`, RP-13/14,
Atlas, supported-host, release or reproducibility obligations.

**#603 is OPEN** until the actual ready-PR production CI is repaired and
qualified, and the real postcutover results are measurable. The purpose of this
ledger is to prevent the team from confusing a green pilot with that outcome.

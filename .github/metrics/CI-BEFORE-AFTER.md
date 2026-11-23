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
selection passed a nonqualifying hosted `xtask` source change in [run #45105140427](https://github.com/UOR-Foundation/lexlean/actions/runs/45105140427),
while downstream transitive proof and merge qualification remain unverified. None of this replaces `SPEC.md §9.4`, RP-13/14,
Atlas, supported-host, release or reproducibility obligations.

**#603 is OPEN** until the actual ready-PR production CI is repaired and
qualified, and the real postcutover results are measurable. The purpose of this
ledger is to prevent the team from confusing a green pilot with that outcome.

## Actual hosted Rust selector exercise and duplicate-work economics (2026-11-23 UTC)

- **Hosted proof (NONQUALIFYING):** [development-feedback run #45105140427](https://github.com/UOR-Foundation/lexlean/actions/runs/45105140427) completed SUCCESS on exact, temporary Rust-change HEAD `a3bfbf8be2356744c34a3301002c13bd0eb742e4`. The scoped diff added one comment to `xtask/src/main.rs`. The trusted-base planner read the real Cargo manifests and selected **only `xtask`**; Rust formatting passed, `cargo check --all-targets --all-features -p xtask` genuinely executed and passed (Cargo reported 1m27s), all 88 Python contract tests passed, and the existing normative `ci` and `reproducibility` runs were SKIPPED. The report artifact was regenerated successfully. This demonstrates provider execution of one Rust source class; it does **not** qualify downstream transitive classes or replace SPEC §9.4.
- **Transient probe cleaned:** commit [`e2633d4`](https://github.com/UOR-Foundation/lexlean/commit/e2633d4d7d0ac4402d304e08bb80b03b434b6d6f) removed that comment. The `xtask/src/main.rs` blob returned to original SHA `4ef56fb32c2b34e72871ea1e295d37d3a03492d7`. The one-time opt-in marker was removed before the revert. The draft PR's later HEAD is **not** claimed as exact-head-qualified by this historical test.
- **Feedback critical path:** hosted job log spans about 3m16s; checkout consumed approximately 90s, and selected Cargo check consumed about 87s. These are log-derived durations, **not GitHub billing data**. Optimize checkout only with independently tested fetch/ancestor/malformed-diff safeguards; do not presume an improvement from a shallow or partial clone.
- **Observed nested execution overlap:** `Justfile` makes `repro` part of `vv`; `.github/vv-rows.json` schedules `repro` inside `rest`; `.github/workflows/reproducibility.yml` independently executes `just repro` (itself `cargo xtask check-reproducibility`). This is a confirmed same-function double execution opportunity, **not proof that the independent status/proof boundary is redundant**. Preserve the extra status until SPEC, branch rule, and unique-detector audit resolves it.
- **Magnitude (frozen historical sample):** join `ci-before-pr50-runs.json` to `ci-before-pr50-job-costs.json` on run ID. Of **99,321.656** observed job-minutes, `ci` contributes **97,835.487**, standalone `reproducibility` **275.668**, and regenerate/release/tag collectively **1,210.501**. Even zeroing *all* standalone repro runner minutes would be an illustrative upper bound of **0.278%** of this sample, not a permissible cut or earned saving. The much larger `ci` matrix and its actual test internals remain the primary optimization priority.

**Engineering gate still HOLD:** source-bound complete normative detector and proof-obligation closure; real downstream-change hosted validation; alternative strategy/failure simulations; required-status governance; exact-head production cutover; matched post-cutover savings. Nothing here changes the acceptance workflow or establishes AFTER ROI.

## Per-job evidence gap and next cost study (2026-11-23 UTC)

Frozen history: 128 complete final-HEAD workflow runs across 50 merged PRs contain
99,321.656 observed unweighted runner-job minutes. Main `ci` accounts for
97,835.487 of those minutes, across 68 runs and 4,296 reported jobs;
standalone `reproducibility` accounts for 275.668 minutes. However, the
snapshot preserves **per-run** platform durations, not individual job names
and timing. It cannot identify which BDD, Atlas, checkout, or native host
jobs dominate. Do not impute a job ranking from aggregated run totals.

Seven of the 50 PRs each exceeded 3,000 observed job-minutes; together
they account for 27,891.885 minutes (28.08% of the sample). This is
concentration, not a proven explanation of waste or merge delay.

New optional `ci_metrics_collect.py --include-job-breakdown` writes
`<period>-jobs.json` from **existing** paginated GET responses, without
any additional network calls or workflow executions. It retains exact
PR HEAD, run and attempt, unique job ID/name, status, runner labels, and
observed timestamps. The original snapshot schemas remain compatible.

The offline `ci_job_hotspots.py` validates the job ledger against **both**
the run inventory and run cost snapshot before producing a group ranking,
observed execution envelope, and source-attributed CSV/JSON. Skipped jobs
have N/A elapsed time rather than fabricated zero. It refuses missing,
duplicated, partial, cross-HEAD or cross-attempt evidence. Historical
aggregate-only source is explicitly insufficient.

On an explicitly authorized future read-only capture, use:

```sh
python3 .github/scripts/ci_metrics_collect.py \
  --period before --cutover-utc "$CUTOVER_UTC" \
  --limit 50 --max-api-calls 800 --include-job-breakdown \
  --output-dir ci-evidence/jobs
python3 .github/scripts/ci_job_hotspots.py \
  --jobs ci-evidence/jobs/before-jobs.json \
  --runs ci-evidence/jobs/before-runs.json \
  --costs ci-evidence/jobs/before-costs.json \
  --output-dir ci-evidence/hotspots
```

Requires read-only `GITHUB_TOKEN`; there is no scheduled collection,
postcutover outcome or provider-billed-dollar measurement. Do **not**
spend API or Actions budgets merely to generate an attractive forecast.

**Decision gate:** First establish job-level hotspots. Compare preserving
the mandatory full gate with (a) required-check-preserving early stages
and (b) same-run build reuse validated against clean-build/proof receipts.
Early staging can delay clean PRs; reuse may weaken independent assurance.
Source-selective *merge* qualification stays HOLD until all generated,
native, proof and release edges and required-status governance are proven.
Simulated or predicted production ROI remains **UNKNOWN**, not zero.

## Controlled preview gate A/B: full-history lazy blobs (2026-11-23 UTC)

Two 20-minute-capped, NONQUALIFYING preview runs were performed on
[disposable draft PR #611](https://github.com/UOR-Foundation/lexlean/pull/611),
based on the same then-live `release/v0.4` SHA
`a1baff747319f026882aec7e84382c8619949d20`.
The same full preview workflow and six other source inputs were copied
byte-for-byte from pilot #606, with only checkout filtering changed
between the two commits. **Both** runs passed current-base authenticated
preflight, full checkout, exact tree and ancestor validation, whitespace,
49/49 Python tests, frozen-baseline graph generation and artifact upload.

| Candidate | Run and immutable commit | Observed runner log span | Report artifact |
| --- | --- | ---: | --- |
| Full checkout (`fetch-depth: 0`) | [#45116345568](https://github.com/UOR-Foundation/lexlean/actions/runs/45116345568), `357c946a6ab596ad2aebb38b91bfd08fde9accff` | **81.005 s** | `14791189802` |
| Full ancestry, lazy blobs (`fetch-depth: 0`, `filter: blob:none`) | [#45116629117](https://github.com/UOR-Foundation/lexlean/actions/runs/45116629117), `8e4a9d8d06b758ed7d03f566099eab5691970697` | **34.766 s** | `14792021512` |

**Observed change:** 46.239 fewer runner-seconds (**57.08%** of the
unweighted control duration). The GitHub checkout log's repository-fetch
phase spanned ~68.97 seconds in the first run, versus ~1.66 seconds to
checkout in the partial-clone run. Materializing the filtered working
tree still took ~20.46 seconds. The Python suite ran 49 tests in 3.878
and 2.400 seconds respectively. A two-run comparison on distinct HEADs
does not prove variance, causality beyond the only intentionally changed
checkout field, or any improvement in protected merge acceptance.

**Private-repository economics:** runner-seconds are NOT billed minutes.
With the separate report's illustrative Linux $0.006/min rate and
whole-job per-minute rounding, these two *hypothetical* private-job
durations correspond to 2 vs 1 billable minutes, or $0.012 vs $0.006
before plan allowances. Neither number is the public repository's actual
bill or evidence of a 57% monetary saving. No provider invoice accessed.

**Adoption:** The nonqualifying draft-only pilot #606 now includes the
validated `filter: blob:none` with full `fetch-depth: 0`; the source-bound
test asserts both. Its strict 5-second authenticated early ref preflight
remains. The earlier actual moved-base failure run #45115407349 refused
before checkout in 1.234 seconds, compared with a previous 80.713-second
checkout-first refusal #45113854088. A separate positive preflight-only
run #45115686535 passed in 0.547 seconds. The full valid-base A/B above
closes the additional 'positive path after checkout' evidence gap.

**Scope and release safety:** Neither experiment changed `ci.yml`,
`reproducibility.yml`, proof payloads, expected outputs, host coverage,
required check names, tag rules or release evidence. Draft #606 remains
OPEN and UNMERGED; disposable draft #611 was CLOSED UNMERGED. The
production cutover, AFTER cohort and realized ROI are **NOT MEASURED**.

### Filtered-checkout Rust-source coverage (2026-11-23 UTC)

Temporary [draft PR #612](https://github.com/UOR-Foundation/lexlean/pull/612)
started from the successful filtered preview experiment and planted
a single comment in `xtask/src/main.rs` to exercise actual Rust-source
impact selection. Hosted [run #45117080560](https://github.com/UOR-Foundation/lexlean/actions/runs/45117080560)
on exact HEAD `ea759db5e578d68b52702458b88e5a1b3b61a688` **PASSED**:
authenticated live-base preflight, full-history lazy-blob checkout, exact
HEAD/ancestor comparison, whitespace, 49/49 Python tests, before/after
report upload, pinned Rust installation, `cargo fmt --all -- --check`,
and the real `cargo check --all-targets --all-features -p xtask`.

Observed whole-job log span: **114.717 seconds**; Cargo reported
**1m08s** for its selected compilation. This validates a second workload
class under filtered checkout, but is **not a controlled Rust-source A/B**
against the older nonfiltered run on a different base/test cohort. The
synthetic Rust comment was reverted byte-for-byte (source blob restored
to `4ef56fb32c2b34e72871ea1e295d37d3a03492d7`), the one-shot
marker was removed, and fixture PR #612 was CLOSED UNMERGED. No merge
acceptance or production cost-saving evidence is claimed.

## Matched 28-partition bootstrap CI historical run pair (2026-11-23 UTC)

Unlike the two-run **draft-feedback** partial-clone A/B above, this is
**actual complete normative CI** evidence retrieved read-only from GitHub:

- **BEFORE:** [full run #44599982425](https://github.com/UOR-Foundation/lexlean/actions/runs/44599982425) on 2026-11-19, checkout prefix `6c6912f`, workflow Git blob `daa958aac0d72ef62b80fb1cd4ffd8a3a145327c`. All bootstrap BDD consumers began beside the `bootstrap-export` producer and could poll for its artifact while occupying runners.
- **AFTER:** [full run #45100057759](https://github.com/UOR-Foundation/lexlean/actions/runs/45100057759) on 2026-11-23, exact HEAD `b693bb4259d311fd71da90ac38701214a74be27f`, workflow blob `f4b6ab5d8a229665d3d43ba19e6e2db1e34e5f63`. [Merged PR #602](https://github.com/UOR-Foundation/lexlean/pull/602) had already split the manifest-selected jobs into `vv-fast` and producer-dependent `vv`. **This scheduler change is ALREADY MERGED**, not a new #606 change.

Downloaded **56 archived acceptance-evidence ZIPs** and compared all **28 shared `bdd-bootstrap-*` partitions**, matching each conformance case ID across runs. Each archive's `summary.md` gives BDD recipe seconds, `cases-telemetry.jsonl` gives case wall milliseconds and PASS, and `take-bootstrap-export.txt` gives request/receipt clock times. All **28/28 case IDs matched, and every case passed in both runs**. The durable per-partition metrics, artifact IDs, exact source/workflow IDs and caveats are in [ci-bdd-bootstrap-28-matched-20261123.json](ci-bdd-bootstrap-28-matched-20261123.json). GitHub original artifact ZIPs expire under their retention policy; the extracted counts must stay durable.

| Observed sum across 28 matching partitions | Before | After | Change |
| --- | ---: | ---: | ---: |
| BDD recipe execution (runner-occupied, excludes job setup) | **1,094.82 min** | **179.50 min** | **−915.32 min (−83.60%)** |
| Individual conformance-case wall time | 1,058.50 min | 96.81 min | −961.69 min (−90.85%) |
| Waiting for run-bound export after request | 909.47 min | 2.40 min | −907.07 min (−99.74%) |
| Matched case failure count | 0 | 0 | no detector omitted in this paired set |

**Specific detector:** BS-09's case wall time went from **2,305.935s**
to **126.963s**, while its export wait decreased from **2,161s** to **8s**;
the BDD recipe fell from **2,380s** to **299s**. Before/after archived
`acceptance-gate-evidence-bdd-bootstrap-runtime` artifacts:
`14571111769` and `14787015783`.

**Causal restraint:** These are two **different code commits** and
not an identical-head controlled A/B. Revisions, host conditions,
case implementations and other work may contribute. Crucially,
**BS-15** waited only **3s** for the export in the original run yet
its recipe also sped up: the full 83.6% recipe delta **cannot**
be credited solely to scheduling. The measured artifact-wait
reduction is the most directly schedule-connected evidence.
These are recipe and test elapsed times, **not complete billable
runner-job minutes or protected time-to-merge**. For an illustrative
unrounded private Linux rate of $0.006/min, multiplying the 915.32
recipe-minute difference yields ~$5.49 of **modelled** compute
for this subset. It excludes runner setup, per-job minute rounding,
allowances and unrelated jobs; this is not a provider bill or earned savings.

**Regression check (new, draft-only):**
`ci_bootstrap_staging_audit.py` executes the real committed jq planner
against the row and BDD manifests, Rust BS test registrations and
actual `vv-fast`/`vv` dependency source. It found **29 producer-dependent
and 11 independent BDD partitions**, and all **31 BS tests** assigned
uniquely. Its 15 adversarial mutations, including forced early scheduling
and removal of the producer dependency, all passed in [hosted
nonqualifying run #45119389992](https://github.com/UOR-Foundation/lexlean/actions/runs/45119389992);
the whole selected Python suite was **64/64 PASS**, artifact uploaded.
This is source/planner regression evidence, not execution of the
40+ minute normative BDD matrix. PR #613 was only its temporary draft fixture.

**Decision:** Retain and protect the already-merged two-lane scheduler.
Do not introduce a second rearrangement or record the 915.32 minute
historical delta as a new #606 ROI. Continue testing remaining actual
post-staging native-host, Atlas and BDD costs with source-specific
falsifiers and no unnecessary full Actions runs. The 50-PR production
AFTER cohort remains **UNKNOWN**, not fabricated from this run pair.

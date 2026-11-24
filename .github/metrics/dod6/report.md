# DOD-6 measurement baseline (#603), A / B1 / B2 / C

Collected 2026-11-24T22:25:15.505464+00:00 from the GitHub REST API, using `gh api` GET calls only.

To reproduce:
- **Live:** `python -I collect.py`. It caches every raw response under `raw/`.
- **Offline replay of the cached bytes:** `python -I collect.py --offline && python -I analyze.py && python -I report.py`.

**Evidence labels:**
- **MEASURED:** computed from provider job `started_at`/`completed_at`, or from PR events.
- **MODELED:** MEASURED minutes multiplied by published list price. This is not this repository's billing.
- **COUNTERFACTUAL:** minutes a run would have used had it executed a gate.
- **CONTROLLED:** same source, host and toolchain. No CONTROLLED comparison exists in this baseline.

Nothing here is a causal saving. Every comparison between A and B crosses source revisions,
PR mix, release lines, provider queue state and workload, and §10 lists these differences.

## 1. Method

**Job accounting, with retried-run lineage reconciled.**
- **Reads:** jobs are read with `jobs?filter=all&per_page=50`. A page size of 60 or more returns HTTP 502 on retried runs.
- **Fail closed:** a read is refused if `total_count` changes between pages, if the inventory is short, or if a job ID repeats.
- **Lineage:** on a retried run, the provider gives carried-forward jobs the new `run_attempt` and new job IDs, but keeps their original `started_at`/`completed_at`. Each executed `(name, started_at, completed_at)` is therefore counted once, at its lowest attempt.
- **Executed:** a job counts as executed only if the provider allocated it a runner (`runner_name`/`runner_id` set, or steps executed) and `completed_at >= started_at`.
- **Placeholders:** skipped jobs, and cancelled jobs that never received a runner, carry synthetic timestamps. They are placeholders and count as zero minutes.

**Host classification** comes from the runner labels:

| Label | Host |
| --- | --- |
| `ubuntu-24.04` | linux-x64 |
| `ubuntu-24.04-arm` | linux-arm64 |
| `windows-2022` | windows-x64 |
| `macos-15` | macos-arm64 |
| `macos-15-intel` or `macos-13` | macos-x64 |

A placeholder has no labels, so it is classified by its job name:
- "Windows tests (...)" is windows-x64.
- "Intel ..." is macos-x64.
- "macOS arm64 ..." and `*-apple-darwin` are macos-arm64.
- `aarch64-unknown-linux-gnu` is linux-arm64.

**Phase attribution is by gate revision, not by calendar date.** The collector tests each
run's `head_sha` for ancestry of a set of marker commits. For a `pull_request` run it also
tests the base-branch tip at the time the run was created. The marker commits are:
- `cb8eca0021`: the #601 draft guard on release/v0.4, landed 2026-11-22 08:46Z.
- `3588a567c8`: the #602 independent/bootstrap split (the issue's "Phase A" PR), merged 2026-11-23 00:08Z.
- `bd80fa3739`: the #619 head, the B1 cutover on release/v0.5, merged as `44616baa` at 2026-11-24 15:03:55Z.
- `603336bf27`: the #620 head (pages of 50, bounded atomic reads, retry custody), merged as `471237b3` at 20:33:54Z.
- `96f35dce8c`: the #609/#607 combined composition, the B2 pilot.

The phases are assigned as follows:
- **B2:** the gate revision contains the combined composition.
- **B1:** the gate revision contains the #619 head.
- **A:** everything else.
- **C:** no runs. DOD-7 has not started.

**Confounder you must carry: release/v0.4 already has a B1-type mechanism.**
- #601 draft suppression landed on release/v0.4 on 2026-11-22, which is inside the BEFORE window and before #619.
- release/v0.4 has neither the #619 cutover nor the #620 reader, so its runs are labelled A.
- Its zero-allocation draft runs are reported separately by line in §6, so the #619 cutover is not credited with them.

**Modeled rates (MODELED, list price).**
- **Source:** <https://docs.github.com/en/billing/reference/actions-runner-pricing>, fetched 2026-11-24. The page itself carries no date or version.

| Standard hosted runner | SKU | Rate per minute |
| --- | --- | ---: |
| Linux 2-core x64 | `actions_linux` | $0.006 |
| Linux 2-core arm64 | `actions_linux_arm` | $0.005 |
| Windows 2-core x64 | `actions_windows` | $0.010 |
| Windows 2-core arm64 | `actions_windows_arm` | $0.010 |
| macOS 3-core or 4-core (M1 or Intel) | `actions_macos` | $0.062 |

- **Rounding:** the page says each job's minutes are rounded up to the next whole minute. Cost is therefore shown two ways: exact minutes times rate, and per-job ceiling minutes times rate.
- **Repository visibility:** `UOR-Foundation/lexlean` is **public**. Standard hosted runners are not metered per minute for a public repository, so this repository's actual billing is unknown, and probably zero. These dollars are a private-repository-equivalent model only.
- **Not comparable to the earlier figure:** the earlier #606 economics comment priced linux-arm64 at the x64 rate ($0.006), and this model uses $0.005. Its $1,408.78 is therefore not directly comparable with the dollars here.

## 2. BEFORE cohort (preserved unchanged)

The definition is taken verbatim from `.github/metrics/ci-before-pr50-runs.json` on the
advisory branch `ci/phase-b-selective-development-feedback-20261122` (PR #606):

- **Window:** `last-50-merged-prs-before-phase-b-production-cutover-preview`.
- **Scope:** `exact-final-HEAD-run-attribution; earlier PR revisions and postmerge push omitted`.
- **PRs:** the 50 most recently merged PRs by `merged_at`, merged from 2026-11-14T15:58:52Z to 2026-11-23T00:56:43Z.
- **Bases:**

  | Base | PRs |
  | --- | ---: |
  | release/v0.4 | 25 |
  | main | 16 |
  | release/v0.5 | 7 |
  | stacked feature branches | 2 |

- **Runs:** every workflow run, of any workflow and any event, whose `head_sha` equals the PR's final head. There are 128.

The 50 PRs are:

#405, 431, 441, 493, 503, 507, 518, 519, 526, 529,
535, 536, 539, 540, 543, 545, 550, 551, 553, 554,
556, 557, 560, 561, 563, 564, 566, 567, 570, 572,
573, 574, 575, 576, 577, 578, 579, 580, 581, 584,
587, 590, 591, 595, 596, 597, 599, 601, 602, 604.

This collector reuses those exact 128 run IDs. It refetches their complete job inventories
and applies the lineage rule from §1.

**Recomputed total: 87,770.686 job-min (MEASURED).** That is -527.668 min away
from the published corrected baseline of 88,298.354. The difference comes from the
executed-only rule (allocation required) and from rounding.

Gate features in the cohort's runs: none: 118, v04_draft_guard_601: 4, v04_draft_guard_601+split_602: 6.

| Metric | n | median | p90 | max |
|---|---|---|---|---|
| Job-min per PR (final head) MEASURED | 50 | 1,776.3 | 2,615.0 | 4,816.2 |
| Ready-or-last-push to merge, min MEASURED | 50 | 213.8 | 611.6 | 810.4 |
| Final-head time to green (PR ci+repro), min MEASURED | 29 | 186.2 | 451.1 | 722.9 |
| First failing job completion after ci created, min MEASURED | 15 | 35.6 | 127.2 | 485.9 |

## 3. Runner-job minutes and modeled cost, per cohort and phase

The window holds every run created since 2026-11-22T00:00Z. The provider reported
629 runs. The BEFORE cohort lies partly inside the window, so its runs also
appear in the window phases.

| Group | Runs (nonterminal) | Executed jobs | Job-min MEASURED | linux-x64 | linux-arm64 | windows-x64 | macos-arm64 | macos-x64 | Modeled USD (exact / per-job ceil) MODELED | Max concurrent (all) |
|---|---|---|---|---|---|---|---|---|---|---|
| BEFORE 50-PR cohort | 128 (0) | 3,631 | 87,770.7 | 64,274.9 | 4,452.0 | 7,182.7 | 5,890.8 | 5,970.3 | 1,215.12 / 1,233.54 | 56 |
| Window phase A | 593 (1) | 1,673 | 32,464.5 | 21,230.2 | 2,005.7 | 3,530.7 | 3,045.2 | 2,652.7 | 525.98 / 535.19 | 56 |
| Window phase B1 | 26 (0) | 224 | 7,829.4 | 5,122.9 | 504.2 | 755.7 | 461.7 | 984.9 | 130.50 / 131.68 | 53 |
| Window phase B2 | 10 (1) | 27 | 442.9 | 369.1 | 0 | 60.5 | 13.4 | 0 | 3.65 / 3.76 | 17 |
| AFTER all (since cutover) | 84 (2) | 276 | 7,795.0 | 4,867.7 | 479.5 | 952.0 | 575.6 | 920.1 | 133.86 / 135.43 | 53 |
| AFTER other / A | 2 (0) | 4 | 77.7 | 77.7 | 0 | 0 | 0 | 0 | 0.47 / 0.47 | 2 |
| AFTER release/v0.4 / A | 50 (1) | 93 | 2,067.4 | 972.1 | 187.4 | 390.5 | 282.9 | 234.4 | 42.75 / 43.36 | 46 |
| AFTER release/v0.4 / B2 | 10 (1) | 27 | 442.9 | 369.1 | 0 | 60.5 | 13.4 | 0 | 3.65 / 3.76 | 17 |
| AFTER release/v0.5 / B1 | 22 (0) | 152 | 5,207.0 | 3,448.9 | 292.1 | 501.1 | 279.3 | 685.7 | 86.99 / 87.84 | 53 |

## 4. Feedback latency and queue delay (full-gate `ci` runs, at least 40 executed jobs)

- **First failure:** the first failed job's `completed_at` minus the run's `created_at`. It is a lower bound on time to the first actionable failure: the log is readable at that point, but nobody has classified it yet.
- **Terminal:** the last executed job's `completed_at` minus the run's `created_at`.
- **Queue:** a job's `started_at` minus its own `created_at`, for attempt-1 executions only.

| Group | Full-gate ci runs | Job-min/full gate (median, max) | First failure min (n, median, max) | Terminal min (median, p90, max) | Queue s all (median, p95, max) | Queue s macos-x64 (median, max) | Queue s macos-arm64 (median, max) |
|---|---|---|---|---|---|---|---|
| BEFORE 50-PR cohort | 50 | 1,746.2, 2,869.4 | 16, 50.8, 485.9 | 191.0, 436.6, 722.9 | 418.0, 5,311.3, 29,834.0 | 2,814.5, 22,154.0 | 2,477.0, 29,834.0 |
| Window phase A | 19 | 1,503.5, 2,651.2 | 6, 46.8, 116.3 | 123.8, 200.3, 202.6 | 3.0, 1,347.5, 4,172.0 | 470.0, 4,172.0 | 130.0, 4,079.0 |
| Window phase B1 | 3 | 2,615.6, 2,643.3 | 3, 57.9, 105.7 | 236.1, 357.5, 387.8 | 2.0, 250.8, 714.0 | 4.0, 591.0 | 2.0, 714.0 |
| Window phase B2 | 0 | n/a, n/a | 0, n/a, n/a | n/a, n/a, n/a | 2.0, 3.0, 304.0 | n/a, n/a | 304.0, 304.0 |
| AFTER all (since cutover) | 3 | 2,520.9, 2,643.3 | 2, 81.8, 105.7 | 192.2, 227.3, 236.1 | 2.0, 385.6, 4,172.0 | 254.5, 4,172.0 | 509.0, 4,079.0 |
| AFTER other / A | 0 | n/a, n/a | 0, n/a, n/a | n/a, n/a, n/a | 2.0, n/a, 3.0 | n/a, n/a | n/a, n/a |
| AFTER release/v0.4 / A | 1 | 2,057.9, 2,057.9 | 0, n/a, n/a | 192.2, 192.2, 192.2 | 3.0, 1,442.2, 4,172.0 | 1,815.0, 4,172.0 | 2,035.0, 4,079.0 |
| AFTER release/v0.4 / B2 | 0 | n/a, n/a | 0, n/a, n/a | n/a, n/a, n/a | 2.0, 3.0, 304.0 | n/a, n/a | 304.0, 304.0 |
| AFTER release/v0.5 / B1 | 2 | 2,582.1, 2,643.3 | 2, 81.8, 105.7 | 170.9, 223.0, 236.1 | 2.0, 297.0, 714.0 | 206.5, 591.0 | 358.0, 714.0 |

### Maximum simultaneous executing jobs per host (sweep over every run in the group)

| Group | linux-x64 | linux-arm64 | windows-x64 | macos-arm64 | macos-x64 | all |
|---|---|---|---|---|---|---|
| BEFORE 50-PR cohort | 48 | 3 | 10 | 5 | 5 | 56 |
| Window phase A | 47 | 4 | 11 | 5 | 5 | 56 |
| Window phase B1 | 42 | 2 | 5 | 1 | 4 | 53 |
| Window phase B2 | 15 | 0 | 2 | 1 | 0 | 17 |
| AFTER all (since cutover) | 42 | 2 | 5 | 4 | 4 | 53 |
| AFTER other / A | 2 | 0 | 0 | 0 | 0 | 2 |
| AFTER release/v0.4 / A | 38 | 1 | 5 | 4 | 3 | 46 |
| AFTER release/v0.4 / B2 | 15 | 0 | 2 | 1 | 0 | 17 |
| AFTER release/v0.5 / B1 | 42 | 2 | 5 | 1 | 4 | 53 |

## 5. AFTER cohort (naturally occurring, still accumulating)

**Definition.**
- **Runs:** every run created at or after the B1 cutover (2026-11-24T15:03:55Z, when #619 merged as `44616baa`), grouped by line and by gate-revision phase.
- **PR-level AFTER (defined the same way as BEFORE):** PRs merged at or after the cutover, attributed to their final-head runs.

**PRs merged so far: 3 of the 50 that the BEFORE definition needs.** The
cohort is still accumulating, and no workload was manufactured for it.

| PR | Base | Merged | Ready/push -> merge min | Time to green min | First failure min | Final-head job-min |
|---|---|---|---|---|---|---|
| #614 | release/v0.4 | 2026-11-24T18:37:06Z | 483.9 | 202.3 | n/a | 1,826.0 |
| #619 | release/v0.5 | 2026-11-24T15:03:55Z | 724.6 | 388.6 | 19.9 | 2,622.3 |
| #620 | release/v0.5 | 2026-11-24T20:33:54Z | 244.5 | 237.1 | 57.9 | 2,652.1 |

## 6. Draft execution avoidance (zero-allocation `pull_request` runs in the window)

A zero-allocation run is a `ci` or `reproducibility` run whose every job is a skipped
placeholder, with no runner allocated and no step executed.

**COUNTERFACTUAL charge per run:**
- **`ci`:** the median completed full-gate `ci` for the same phase and line. Where there is none, the median of all full gates.
- **`reproducibility`:** the median reproducibility run.

Counterfactual basis: full-gate median by phase/line: A|release/v0.4=1,330.4, A|release/v0.5=2,094.6, B1|release/v0.5=2,615.6; all full gates median 1,736.5 min; reproducibility median 5.8 min; pre-guard partial draft ci (honesty + rest): n=20 median 8.1.

**Read it as an upper bound:**
- Before the guards, a draft ran only honesty plus `rest`, not the full matrix.
- A run can also be zero-allocation because a ready PR is stacked on a feature base. The "PR still draft now" column separates those cases from genuine drafts.

| Line / phase | Zero-allocation PR runs | ci | repro | Placeholders skipped | PR still draft now | COUNTERFACTUAL lower (honesty+rest draft) min | COUNTERFACTUAL upper (full gate) min |
|---|---|---|---|---|---|---|---|
| ci/experiment-bdd-staging-falsifiers-20261123 / A | 2 | 1 | 1 | 22 | 0 | 13.9 | 1,742.3 |
| ci/experiment-bounded-ref-positive-20261123 / A | 4 | 2 | 2 | 44 | 0 | 27.9 | 3,484.6 |
| ci/experiment-complete-valid-preview-20261123 / A | 4 | 2 | 2 | 44 | 0 | 27.9 | 3,484.6 |
| ci/experiment-filtered-rust-source-20261123 / A | 4 | 2 | 2 | 44 | 0 | 27.9 | 3,484.6 |
| release/v0.4 / A | 372 | 186 | 186 | 4074 | 304 | 2,593.1 | 248,533.2 |
| release/v0.4 / B2 | 8 | 4 | 4 | 88 | 0 | 55.8 | 6,969.1 |
| release/v0.5 / A | 22 | 11 | 11 | 209 | 0 | 153.4 | 23,103.8 |
| release/v0.5 / B1 | 14 | 7 | 7 | 133 | 10 | 97.6 | 18,349.6 |

## 7. Targeted retries: retained versus repeated minutes (MEASURED)

- **Retained:** executed in an earlier attempt and carried into the final attempt with identical timestamps.
- **Lost:** an earlier-attempt execution that is absent from the final attempt.
- **Repeated:** newly executed in the final attempt.
- **Avoided (COUNTERFACTUAL):** the retained minutes, which a full re-run would have executed again.

| Run | Head | Attempts | Retained jobs | Retained min MEASURED | Lost min MEASURED (jobs) | Repeated min MEASURED (jobs) | Total executed min MEASURED | Full-rerun minutes avoided COUNTERFACTUAL |
|---|---|---|---|---|---|---|---|---|
| 45316289312 | `bd80fa37` | 2 | 69 | 2,403.383 | 14.500 (build and non-Lean tests (aarch64-unknown-linux-gnu) failure) | 197.683 (build and non-Lean tests (aarch64-unknown-linux-gnu) success) | 2,615.566 | 2,403.383 |
| 45437716141 | `603336bf` | 2 | 69 | 2,559.233 | 52.033 (just vv (bdd-verification) failure) | 32.067 (just vv (bdd-verification) success) | 2,643.333 | 2,559.233 |

Other multi-attempt runs in the window:

- run 45148610286 (attempts 2): retained 70 jobs / 986.633 min, lost 53.033, repeated 11.883, total 1,051.550 MEASURED
- run 45170496660 (attempts 2): retained 77 jobs / 1,129.950 min, lost 46.050, repeated 57.250, total 1,233.250 MEASURED

## 8. Avoided duplicate post-merge executions

These are `push` `ci` runs whose only executed job is `plan`, because the tree equals a
verified head.

| Run | Line | Phase | Head | Created | Conclusion | plan min MEASURED | Skipped placeholders | COUNTERFACTUAL full-gate min |
|---|---|---|---|---|---|---|---|---|
| 45055728865 | main | A | `f30b6b84` | 2026-11-22T18:33:08Z | success | 0.900 | 19 | 1,736.5 |
| 45110610574 | release/v0.4 | A | `a1baff74` | 2026-11-23T04:14:16Z | success | 0.633 | 20 | 1,330.4 |
| 45168629811 | main | A | `9b9fc2a0` | 2026-11-23T11:49:30Z | success | 0.733 | 19 | 1,736.5 |
| 45482125626 | release/v0.5 | B1 | `471237b3` | 2026-11-24T20:33:57Z | success | 1.367 | 17 | 2,615.6 |

Specific case: run 45482125626: plan 1.367 min MEASURED; reused verified run 45437716141 whose final logical inventory is 2,591.300 min (COUNTERFACTUAL had it been re-executed); the prior merge push 45421145394 (same mechanism failing on a retried run) executed 2,520.933 min MEASURED.

Full-gate `push` `ci` runs in the window, where reuse did not apply:

- run 45000219302 main phase A `4a9d79e6` success: 2,400.683 job-min MEASURED
- run 45091715846 release/v0.4 phase A `24ce6718` success: 1,178.817 job-min MEASURED
- run 45170496660 release/v0.4 phase A `f57650e6` success: 1,233.250 job-min MEASURED
- run 45421145394 release/v0.5 phase B1 `44616baa` cancelled: 2,520.933 job-min MEASURED
- run 45461480630 release/v0.4 phase A `d553cbf4` None: 2,057.884 job-min MEASURED

## 9. Assurance parity: normative job names per phase

Each phase is represented by its latest full-gate `ci` run on the given line. The job names
are the union of executed and placeholder names in that run.

- **A|release/v0.5**: run 45256672496 `53f359a9` completed/failure, 70 executed + 0 placeholders, 70 distinct job names
- **A|release/v0.4**: run 45375860724 `db6f84e7` completed/success, 86 executed + 3 placeholders, 89 distinct job names
- **B1|release/v0.5**: run 45437716141 `603336bf` completed/success, 71 executed + 0 placeholders, 70 distinct job names
- A|release/v0.5 -> B1|release/v0.5: missing in target: none; new in target: none
- A|release/v0.4 -> A|release/v0.5: missing in target: `Intel conformance (${{ matrix.lane }})`, `just vv (bdd-bootstrap-core-json)`, `just vv (bdd-bootstrap-core-print)`, `just vv (bdd-bootstrap-core-read)`, `just vv (bdd-bootstrap-grammar-math)`, `just vv (bdd-bootstrap-grammar-proof)`, `just vv (bdd-bootstrap-lexicon-entry)`, `just vv (bdd-bootstrap-lexicon-schema)`, `just vv (bdd-bootstrap-lse-scopes)`, `just vv (bdd-bootstrap-lse-types)`, `just vv (bdd-bootstrap-package-ref)`, `just vv (bdd-bootstrap-proposition-terms)`, `just vv (bdd-bootstrap-semantic-print)`, `just vv (bdd-bootstrap-semantic-read)`, `just vv (bdd-bootstrap-toml-read)`, `just vv (bdd-language-models)`, `just vv (bdd-stdlib)`, `just vv (bdd-wasi)`, `macOS arm64 tests (lower)`, `macOS arm64 tests (moves)`, `macOS arm64 tests (print)`, `macOS arm64 tests (producer)`, `macOS arm64 tests (validate)`; new in target: `Intel conformance (lower)`, `Intel conformance (moves)`, `Intel conformance (print)`, `Intel conformance (validate)`

Union roster over every full-gate `ci` run, per phase and line (the A|v0.4 -> A|v0.5 difference above is a release-line content difference, not a phase loss):
- A|batch/bs35-52: 1 full-gate runs, 89 distinct job names
- A|cut/v0.4.5: 1 full-gate runs, 78 distinct job names
- A|main: 22 full-gate runs, 79 distinct job names
- A|release/v0.4: 36 full-gate runs, 93 distinct job names
- A|release/v0.5: 9 full-gate runs, 70 distinct job names
- B1|release/v0.5: 3 full-gate runs, 70 distinct job names
- **B1|release/v0.5_vs_A**: missing in B: none; new in B: none
- **B2|release/v0.4_vs_A**: NO TERMINAL FULL GATE YET

## 10. Gaps, confounders and what this baseline does not show

**Billing.**
- No billed usage was obtained. The repository is public, so standard hosted minutes are not metered per minute. Organisation and enterprise billing endpoints were not read (they need admin scope), and per-run `/timing` was not sampled in this pass; an earlier #606 sample reported 0 billable ms.
- Every dollar figure here is MODELED list price for a private-repository equivalent.

**The BEFORE recomputation differs from the published 88,298.354 by -527.6 min.**
- The whole difference sits in three cancelled `ci` runs: 43592465105 (#431), 43624887562 (#535) and 44269465823 (#567).
- Each has jobs with `runner_id: 0`, an empty `runner_name` and no steps, which means a queued request that never received a runner.
- One example: `Intel conformance (print)` in 43592465105 waited on `macos-15-intel` from 15:04:58Z to 20:02:39Z, about 298 min.
- The published corrected baseline counts these waits as runner time. This collector does not, because they are queue wait rather than execution.
- The PR list and the run IDs are unchanged. Only the executed-only accounting rule differs. Choose one rule deliberately before freezing BEFORE.

**Nonterminal at collection.** These runs' minutes are lower bounds:
- B2 combined #609 CI 45490646681 was in progress, with 26 jobs executed and 17 placeholders.
- #614's post-merge push CI 45461480630 on release/v0.4 was in progress, with 90 jobs executed.
- B2 therefore has no terminal full gate. B2 latency, cost and parity are not yet measurable.

**Phase marker approximations.**
- Phase is decided by ancestry of marker commits. Earlier draft revisions of #619 do not contain its final head `bd80fa37`, so they are labelled A even though their merge-ref workflow already carried draft suppression. That is the `release/v0.5|A` zero-allocation row.
- A `pull_request` run's base tip is approximated by the first-parent commit before the run was created, using committer date.

**B1 full gates are not cheaper than A full gates.**
- Median full-gate `ci` minutes (MEASURED):

  | Gate | Median min | n |
  | --- | ---: | ---: |
  | BEFORE cohort | 1,746 | 50 |
  | Window A, release/v0.5 | 2,095 | |
  | Window A, release/v0.4 | 1,330 | |
  | B1 | 2,616 | 3 |

- The workload, the job roster and the source revisions all differ. The B1 mechanisms save minutes by not executing: draft suppression, plan-only post-merge reuse and targeted retry. They do not make a single full gate cheaper. No CONTROLLED (same source, same host) comparison exists.

**Feedback latency.**
- "First failure" is the first failing job's completion, which is when a log becomes readable. It is not the time a failure was classified as actionable.
- Ready-or-push-to-merge includes human review and waiting time. It is not CI time.
- Terminal minutes count from run creation and include queue wait.

**Queue delay.**
- Queue delay is attempt-1 job `started_at` minus job `created_at` for allocated jobs only. Never-allocated waits are excluded, and some of those reached about 5 h on macOS Intel and ARM in the BEFORE window.
- The provider does not expose per-runner pool capacity. Concurrency is therefore observed peak simultaneous executions, not capacity.

**Draft counterfactual bounds.**
- The upper bound charges each zero-allocation `ci` run a full-gate median.
- The lower bound charges the median of 20 observed honesty-plus-`rest`-style partial PR runs: 8.1 min, with a p90 of 70.8 min. Before the guards, drafts ran honesty plus `rest`, not the full matrix.
- The true avoided amount lies between the two bounds and is not established.
- Some zero-allocation runs belong to ready PRs into experiment or feature branches, which is stacked-PR skipping rather than draft suppression. The "PR still draft now" column separates them, as of read time only.

**Mechanisms not separately measured.** Intel deferral, the atomic and bounded provider reads (correctness, not minutes), and B2 batching throughput.

**Phase C.** No Rust-specific optimisation exists. DOD-7 has not started.

**AFTER is far from 50.**
- Three PRs have merged since the cutover: #619 (the cutover PR itself), #620 and #614. #614 is on release/v0.4, whose gate revision is A-line.
- AFTER runs are naturally occurring. Nothing was dispatched for this measurement.

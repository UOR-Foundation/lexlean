# DOD-7a baseline: Rust test execution in hosted CI (issue #603)

Evidence: two complete, all-success whole gates on release/v0.5 code.

| Run | PR | Head | Jobs | Job-minutes (sum) | Run wall (first start to last end) |
| --- | --- | --- | ---: | ---: | ---: |
| 45437716141 | #620 | 603336bf | 70 | 2591.3 MEASURED | 14151 s (3.93 h) MEASURED |
| 45316289312 | #619 | bd80fa37 | 70 | 2601.1 MEASURED | 23265 s (6.46 h) MEASURED, retried run, includes re-queue time |

Sources: jobs API (`filter=latest`, `per_page=30`) and every job log (ANSI stripped). Every number below is
labeled MEASURED (runner timestamps, cargo `Finished ... in`, libtest `finished in`, `ci-summary` rows) or
ESTIMATED (inferred, method stated). Job ids are given so each figure can be re-derived (`fetch.sh`,
`parse_logs.py`, `derive.py`, `slots.py` in this directory). Unless stated, figures are from #620 (run
45437716141); the #619 figure follows after a slash.

Read-only: no comment, push, rerun, dispatch or cancel was made. Uploaded `ci-evidence` artifacts were not
downloaded (logs only); they hold finer telemetry (per-recipe `resources.tsv`, `take-bootstrap-export.txt`)
and are the next data source if wanted.

## 1. Exact commands of the heavy jobs (`.github/workflows/ci.yml`, `Justfile` on origin/release/v0.5)

| Job | Runner label | Command that runs the tests | Threads | Doctests |
| --- | --- | --- | --- | --- |
| `build and non-Lean tests (aarch64-unknown-linux-gnu)` / `(aarch64-apple-darwin)` (`host-matrix`) | `ubuntu-24.04-arm` / `macos-15` | `cargo test --locked --workspace --all-features --no-fail-fast` (one step, test profile = dev: `debug = "line-tables-only"`, `lexlean` opt-level 1, `sha2` opt-level 3; no `--release`) | libtest default; no `--test-threads`/`RUST_TEST_THREADS` anywhere in ci.yml, Justfile, xtask, scripts | yes, in the same invocation; 0 doctests exist (lexlean, repo_conformance, repo_model all `running 0 tests`), overhead about 5 s MEASURED (138732592227: 19:49:32.75 to 19:49:37.59) |
| `Intel test construction and local targets` | `macos-15-intel` | `intel_host_tests.py build`: `cargo test --locked --workspace --all-features --no-run --no-fail-fast --message-format=json`, then runs every non-conformance binary and `cargo test ... --doc` | default | yes (here) |
| `Intel conformance (<lane>)` x4 | `macos-15-intel` | `intel_host_tests.py execute`: the prebuilt `conformance` binary, lane filter from `.github/intel-host-lanes.json`; the script refuses `RUST_TEST_*`, `RUST_TEST_THREADS`, `CARGO_BUILD_*`, `CARGO_PROFILE_*`, `RUSTFLAGS` in the environment | default | no |
| `Windows tests (<lane>)` x5 | `windows-2022` | `host_lanes.py execute`: `cargo test --locked --all-features --no-fail-fast -p repo-conformance --test conformance -- --exact <lane cases>`; `producer` lane runs every other target and doctests | default | producer only |
| `just vv (<part>)` x ~45 | `ubuntu-24.04` | `just --timestamp <recipes>`; `bdd` = `cargo xtask bdd` = `cargo test -p repo-conformance --all-features -- <part filters>` (`LEXLEAN_BDD_PART`); `test` = `cargo test --workspace --exclude repo-conformance --all-features` | default | `bdd` runs `Doc-tests repo_conformance` (0 tests) |

Nested subprocesses inside tests (from source): conformance cases build generated Rust packages with
`cargo build --offline --workspace --quiet` in a fresh tempdir with its own `CARGO_TARGET_DIR`,
`RUST_MIN_STACK=64 MiB`, `RUSTFLAGS` removed (`crates/conformance/src/rust_build.rs::cargo_in`), then run
the built harness binaries; Lean-backed cases run `lean`/`leanchecker` on the normative host. Every `BS`
case copies the bootstrap and stdlib trees into a new tempdir and checks the project again
(`support::P::bootstrap()`, `check_ok()`, `checked_project()`); nothing caches that across cases.

## 2. Host capacity

| Label | Image / version (log header, MEASURED) | vCPU / RAM (GitHub docs, public repo) | libtest threads inferred from log (ESTIMATED, validated) | Memory from log |
| --- | --- | --- | --- | --- |
| ubuntu-24.04 | ubuntu-24.04 / 20261116.133.1, 20261116.376.1 | 4 / 16 GB | 4 | `free -m`: total 15983 MiB, swap 7167 MiB (3 GiB image + 4 GiB `fallocate` step) MEASURED |
| ubuntu-24.04-arm | ubuntu-24.04-arm / 20261116.188.1 | 4 / 16 GB | 4 | not printed |
| macos-15 (arm64, M1) | macos-15-arm64 / 20261118.0550.1 | 3 / 7 GB | 3 | not printed |
| macos-15-intel | macos-15 / 20260824.0482.1 | 4 / 14 GB | 4 | not printed |
| windows-2022 | windows-2022 / 20261116.370.1 | 4 / 16 GB | not inferable (output arrives in one batch, see below) | not printed |

Citation: https://docs.github.com/en/actions/reference/runners/github-hosted-runners (standard runners,
public repositories; `gh api repos/UOR-Foundation/lexlean --jq .visibility` = `public`). No `nproc`,
`sysctl hw.ncpu` or cgroup/affinity output exists in any log; CPU utilization is not sampled anywhere
(`resources.tsv` samples memory, swap and the largest RSS only).

Thread inference: libtest starts tests in name order as slots free and prints
`has been running for over 60 seconds` 60 s after a start. Replaying that slot model on the observed
completion order and choosing the thread count that best fits the warnings gives 4 / 3 / 4 with a mean
absolute error of 0.00 to 0.11 s over all warned tests (138732662619, 138732592227, 138732592426,
138732593648, 138732592652). Cross-check: the model's estimate for `conformance_ka_01` in `just vv (rest)`
is 1344.55 s; `ci-summary` MEASURED 1344.5 s (138732592652). The default therefore equals the documented
vCPU count on every host; nothing overrides it.

Windows lanes: `host_lanes.py` relays the child's stdout, but every `test ... ok` line of a lane carries
the same timestamp (e.g. 138732592073: all at 17:42:06.73 after `running 20 tests`), so only per-binary
totals are measurable on Windows.

## 3. Highest-cost jobs: setup / compile / test split

`setup` = checkout + rustup/install-action + elan/Lean + swap + artifact download; `compile` = sum of
cargo `Finished` durations; `test` = sum of libtest `finished in`; all MEASURED.

| Job (job id #620 / #619) | Job min | Setup s | Compile s | Test s | Test share |
| --- | ---: | ---: | ---: | ---: | ---: |
| host-matrix aarch64-unknown-linux-gnu (138732662619 / 138255634209) | 195.5 / 197.7 | 60 / 56 | 201 / 210 | 11460 / 11584 | 97.7% |
| host-matrix aarch64-apple-darwin (138732592227 / 138255636018) | 182.6 / 182.4 | 73 / 73 | 453 / 430 | 10412 / 10423 | 95.0% |
| Intel conformance (lower) (138732592426 / 138255637175) | 129.5 / 83.8 | 115 / 93 | 0 (prebuilt) | 7615 / 4906 | 98.0% |
| Intel conformance (print) (138732593648 / 138255636099) | 108.4 / 85.9 | 103 / 91 | 0 | 6374 / 5037 | 98.0% |
| just vv (examples) (138732592545 / 138255636069) | 100.8 / 91.1 | 90 / 82 | 126 / 125 | 0 (Lean `verify-examples`: certificates 4716 s) | 0% |
| Windows tests (lower) (138732685518 / 138255635413) | 76.4 / 75.4 | 72 / 68 | 282 / 278 | 4219 / 4166 | 92.0% |
| Windows tests (print) (138732592073 / 138255636379) | 66.9 / 80.1 | 75 / 70 | 258 / 287 | 3670 / 4439 | 91.4% |
| just vv (rest) (138732592652 / 138255635092) | 66.6 / 64.6 | 93 / 89 | 290 / 285 | 3277 / 3185 | 82.0% |
| Intel conformance (moves) (138732593529 / 138255635990) | 61.0 / 71.5 | 93 / 115 | 0 | 3539 / 4143 | 96.7% |
| Intel test construction (138732592209 / 138255636282) | 24.4 / 18.2 | 37 / 85 | 815 / 526 | 535 / 413 | 36.5% |
| just vv (bdd-bootstrap-*) x28, each (e.g. gnaf 138732592454) | 35.2 to 49.3 / 39.6 to 48.2 | 70 to 90 | 136 to 199 | 1242 to 2719 / 2112 to 2652 | 86 to 92% |
| bootstrap export (138732592334 / 138255636009) | 34.2 / 38.6 | 82 / 78 | 107 / 123 | 0 (Lean: certificates 1513 s of 1738 s) | 0% |

Whole run #620 by class (MEASURED sums):

| Class | Jobs | Job-min | Setup min | Compile min | libtest min |
| --- | ---: | ---: | ---: | ---: | ---: |
| vv bdd-bootstrap-* | 28 | 986.6 | 39.1 | 72.9 | 867.4 |
| ARM host-matrix | 2 | 378.1 | 2.2 | 10.9 | 364.5 |
| Intel conformance lanes | 4 | 351.9 | 7.0 | 0.0 | 342.7 |
| Windows lanes | 5 | 258.6 | 6.5 | 27.4 | 223.5 |
| vv bdd other parts | 9 | 218.2 | 12.1 | 23.7 | 180.1 |
| vv examples + rest | 2 | 167.3 | 3.1 | 6.9 | 54.6 |
| Atlas shards (Lean) | 14 | 164.1 | see note | 2.1 | 0 |

(Atlas and export work steps are Lean verification, not Rust tests.) Compile is 1.7 to 4.2% of the
longest Rust jobs; the whole run's cargo `Finished` sum is 9554 s / 8791 s (about 6% of job-minutes).
Compile is not the lever on the heavy jobs; it is repeated per job (about 45 Linux legs each spend 136 to
199 s building `xtask` plus `repo-conformance`; five Windows lanes each 245 to 291 s).

## 4. Per test binary, heaviest jobs

| Job | Binary | Tests | libtest s (MEASURED) | Notes |
| --- | --- | ---: | ---: | --- |
| macOS ARM 138732592227 | tests/conformance.rs | 524 | 10059.1 (96.6% of test time) | dominant |
| | tests/property.rs | 11 | 314.5 | one test: `formatter_is_idempotent_on_every_example` 314.2 s ESTIMATED; other 10 under 1 s |
| | lexlean lib unit | 236 | 16.8 | |
| | everything else (14 binaries + 3 doctest harnesses) | 128 | < 25 total | |
| Linux ARM 138732662619 | tests/conformance.rs | 524 | 11202.5 (97.8%) | dominant |
| | tests/property.rs | 11 | 227.4 | same single test 227.3 s ESTIMATED |
| just vv (rest) 138732592652 | tests/conformance.rs (321 of 524) | 321 | 3105.3 | KA-01 1344.5, GN-09 935.2, SL-01 667.7 s (ci-summary MEASURED) |
| | tests/property.rs | 11 | 150.7 | same single test 150.6 s |

### 4.1 The serialized `BS` chain (the main mechanism)

Every `BS` case takes one process-wide `HEAVY` mutex before running
(`crates/conformance/src/cases/bootstrap.rs::run`, comment: each case peaks at 5 to 10 GiB, so they run one
at a time). A libtest thread that starts a `BS` case while another holds the lock sits blocked but still
occupies its slot. So inside one process the 31 `BS` cases are a serial chain, and blocked `BS` tests take
slots that non-`BS` tests could use. Execution estimate per BS case = completion minus max(libtest start,
previous BS completion) (ESTIMATED; it is exact if the mutex hands over at once).

| Binary (job) | Threads | Binary wall s | BS chain wall s (31 cases) | Slot-s blocked on HEAVY | Tail after last BS s | Non-BS slot-s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| macOS ARM 138732592227 / 138255636018 | 3 | 10059 / 10071 | 8356 / 8480 (83 / 84%) | 15603 = 51.7% / 52.6% | 1400 / 1280 | 6216 / 5839 |
| Linux ARM 138732662619 / 138255634209 | 4 | 11203 / 11330 | 10143 / 10266 (91%) | 29104 = 64.9% / 64.8% | 832 / 842 | 5560 / 5678 |
| Intel lower 138732592426 (6 BS) | 4 | 7615 | 7615 (100%) | 73.1% | 0 | 0 |
| Intel print 138732593648 (20 BS) | 4 | 6374 | 6374 (100%) | 74.1% | 0 | 0 |
| Intel validate 138732593621 (4 BS + 81) | 4 | 3029 | 3029 | 24.6% (34.8% slots unused) | 0 | 1886 |

All slot figures ESTIMATED (slot model plus HEAVY ordering). The tail after the last BS is spent on
non-BS tests that could not start while slots held blocked BS tests (macOS: `sl_05` 358 s, `rp_19` 301 s,
`ka_08` 214 s started after the chain).

Slowest individual tests (ESTIMATED execution, #620 / #619):

| Test | macOS ARM | Linux ARM | Intel (lane) | Windows | Linux x86 normative |
| --- | ---: | ---: | ---: | --- | --- |
| BS-07 (`rust_lower`) | 3060 / 2972 | 4125 / 4187 | 7110 / 4503 (lower) | lower lane total 4219 / 4166 MEASURED (6 BS) | leg 2719 / 2652 MEASURED, of which 896 / 562 s after the export arrived |
| BS-05 | 1141 / 1394 | 1446 / 1424 | validate lane | | export-bound, see 4.2 |
| BS-06 | 1125 / 1422 | 1513 / 1418 | 2396 / 1879 (print) | | export-bound |
| AU-08 (non-BS) | 1223 | 1339 | 1466 tail (moves) | | 1364.3 MEASURED (bdd-verification) |
| BS-08 | 690 / 534 | 678 / 773 | | | export-bound |
| formatter_is_idempotent_on_every_example (property.rs) | 314 | 227 | 476 (construction) | producer lane binary 299.5 MEASURED | 150.6 |

On non-normative hosts a BS case skips the evaluator and seeded corpora (`support::normative_host()`), so
its time there is the project check plus the nested `cargo build` of rendered rust-std harness packages
(literal-heavy, chunked at `HARNESS_NODES = 60000`) and their runs. The per-phase split inside a case is
printed with `eprintln!` but captured by libtest on success, so it is not in the logs.

### 4.2 `just vv (bdd-bootstrap-*)` legs wait on the Lean export, not on Rust

Each of the 28 legs runs exactly one BS case (`.github/bdd-parts.json`). Each case's test completes
37 to 90 s (#620) / 37 to 87 s (#619) after the `bootstrap export` job completes (17:09:29Z / 03:43:18Z),
for every leg except `bdd-bootstrap-lower` (BS-07: +896 s / +562 s). MEASURED (`derive.py`,
`bdd_bootstrap_legs`). The tests start 1177 to 1834 s (#620) / 2041 to 2103 s (#619) before the export is
ready, and the export job is 1513 s Lean `certificates` of 1738 s (ci-summary of 138732592334). The legs'
1242 to 2160 s of "test time" therefore cannot be shortened by Rust test-execution tuning: at most the
last 40 to 90 s per leg (plus BS-07's 9 to 15 min) is post-export work. How much case work happens before
the case first asks for the export (`wanted`) is not in the logs (`ci-evidence/take-bootstrap-export.txt`
holds it). This is about 867 libtest-minutes of #620's 2591 job-minutes, a generic scheduling and Lean
matter (DOD-6 / B), not DOD-7 (C).

## 5. Native platform differences (same suite, same commit)

| Workload | Linux x86 (4 vCPU, normative) | Linux ARM (4) | macOS ARM (3, 7 GB) | Intel macOS (4) | Windows (4) |
| --- | ---: | ---: | ---: | ---: | ---: |
| Cold `test` profile build, workspace | about 198 s (rest leg `test`) | 201 / 210 | 453 / 430 | 815 / 526 | 245 to 291 per lane |
| property.rs single long test | 150.6 | 227.3 | 314.2 | 475.8 / 362.1 | 299.5 (binary) / 194.6 |
| lexlean lib unit tests (236) | in `test` recipe | n/a | 16.8 | 27.0 / 20.7 | 14.5 / 9.6 |
| Full conformance (524) in one process | not run as one | 11203 / 11330 | 10059 / 10071 | split into 4 lanes | split into 4 lanes |
| Run-to-run variance of the conformance job | | 1.1% | 0.1% | lower 55%, print 27% | print 21%, moves 21% (3058 vs 2535 s) |

All MEASURED except the property test (ESTIMATED, single long test in its binary). macOS ARM finishes the
full binary sooner than Linux ARM despite 3 vs 4 threads because the BS chain is shorter there
(8356 vs 10143 s); Linux ARM has more blocked slots (4 threads, same serial chain). Intel macOS has the
highest variance and the slowest BS-07 (an older image, 20260824).

## 6. Data gaps (not measured; do not infer)

- CPU utilization and effective parallelism inside a running BS case (helper threads `available_parallelism() - 1`
  for the evaluator on the normative host; nested `cargo build -j` default elsewhere). No CPU sampling exists.
- Peak RSS outside Linux x86 (`ci-summary` largest process 2854 to 13821 MiB there; verification leg peaks
  at 13821 MiB with 6258 MiB swap).
- Per-test times on Windows (batched output).
- Pre-export work of BS cases on the normative host.
- Warm-cache build times: every job is a clean checkout without a build cache by design (ci.yml header,
  SPEC.md §9.4), so only cold builds are observed.

## 7. Ranked controlled experiments for DOD-7b

Ranked by measured mechanism and size; each is one workload, one host, controlled A/B on the same commit,
at least 3 repetitions, attributed as C (Rust-specific) only after the B freeze. None is nextest.

1. **Stop blocked BS tests from holding libtest slots (host-matrix macOS ARM and Linux ARM, Intel lanes
   with mixed cases).** Mechanism: 51.7% (macOS) and 64.9% (Linux ARM) of slot-seconds are BS tests blocked
   on `HEAVY`; non-BS work queues behind them and leaves a tail of 1400 s / 832 s after the chain. Variant:
   in the same job and the same build, run the conformance binary twice, concurrently or sequentially:
   `conformance_bs_` cases with `--test-threads=1`, and the rest with `--skip conformance_bs_` at the default
   thread count. Expected bound: binary wall falls toward max(BS chain, non-BS work), at most about 1400 s
   (14%) on macOS and 830 s (7%) on Linux ARM; zero if CPU is already saturated, which item 5 measures.
   Parity risks: the union of the two runs must equal `-- --list` exactly with no duplicates (RP-13; reuse
   `host_lanes.py` listing and disjointness checks); `--exact` and filter semantics; peak memory when a BS
   case and non-BS cases overlap on the 7 GB macOS host (HEAVY's reason; measure RSS); `--no-fail-fast`
   and exit codes of two invocations; `intel_host_tests.py` refuses `RUST_TEST_*` env (use arguments, not
   env); the §8.3 host-status receipt must still name one complete execution.

2. **Order the BS chain longest-first, or start it first.** Mechanism: name-order scheduling starts
   `bs_07` (3060 to 7110 s, the single longest test on every host) only when its slot frees; on Intel
   `lower` it is 93% of the lane (7110 of 7615 s). Critical path cannot go below the chain, but a lane mix
   can be rebalanced by moving non-BS cases into BS-only lanes (Intel `lower`/`print` have 0 non-BS tests
   and 73 to 74% blocked slots, so 3 of 4 slots hold nothing runnable). Expected: Intel `lower`/`print`
   lanes absorb other lanes' non-BS cases at no wall-time cost. Parity risks: lane partition is shared
   with Windows and checked by honesty (`intel-host-lanes.json` completeness), receipts per lane, and this
   overlaps DOD-6 lane design: attribute to B if it changes job topology, to C only if it changes
   scheduling inside a process.

3. **Measure, then reduce, per-case recomputation in BS cases.** Mechanism: each of the 31 BS cases copies
   the bootstrap and stdlib trees and checks the project twice (`check_ok()` then `checked_project()`), and
   builds its rendered packages in a fresh `CARGO_TARGET_DIR`; nothing is shared across the serial chain.
   First step costs nothing in assurance: run one host's conformance binary with `--nocapture` (or
   `--show-output`) to expose the cases' existing `eprintln!` phase timings ("rustc built both packages
   (...)", "N vectors (...)"). Only if the check or nested build dominates, try a process-local
   `OnceLock` of the checked bootstrap project. Parity risks: a shared fixture must stay read-only (cases
   mutate their copy?), a planted defect in one case must not leak into another, the HEAVY memory budget,
   and `stop_before_runs`/counts modes; never share nested build output across cases, since each case's
   evidence is its own compiled package.

4. **Nested rustc builds of generated harnesses (non-normative hosts).** Mechanism: on hosts where the
   evaluator is skipped, BS time is the project check plus `cargo build --offline` of literal-heavy
   generated crates and their runs; BS-07 is 2972 to 7110 s there, and the code notes rustc taking most of
   an hour on large literals. Experiment only after item 3 shows the build share: compare the current
   profile with an explicit nested `CARGO_PROFILE_DEV_DEBUG=0`/codegen-units setting on one case. Parity
   risks: the evidence claim is "compiled by the pinned rustc"; overflow checks and debug assertions must
   stay identical (do not switch to `--release`); `cargo_in` deliberately removes `RUSTFLAGS` and fixes
   `RUST_MIN_STACK` for cross-host identical evidence; generated package bytes and outcomes must be
   unchanged.

5. **Instrument CPU before any thread change.** Mechanism under test: whether cores idle during the BS
   chain (one runnable case, blocked siblings) or are saturated by nested cargo/rustc. Add a CPU sampler
   beside the existing memory sampler (`/proc/stat` on Linux, `top -l`/`vm_stat` on macOS) for one
   host-matrix run. This decides whether items 1 and 2 can pay at all, and whether a `--test-threads`
   change above vCPU count (oversubscription) or below it is worth a run. Parity risks: none on evidence if
   the sampler is operational only (as `resources.tsv` is); keep it out of any artifact identity.

Not recommended from this evidence: tuning the `bdd-bootstrap-*` legs (section 4.2: bound by the Lean
export), compile-time work on the heavy jobs (1.7 to 4.2% of them), and doctest changes (0 doctests,
about 5 s per invocation).

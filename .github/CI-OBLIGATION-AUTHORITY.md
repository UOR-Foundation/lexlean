# CI obligation and authority inventory (issue #603, DOD-1)

Status: **source-bound inventory, not a proof or a merge authorization**. This document records the *actual* acceptance paths and detector owners; it does not claim that a skipped check passes or that GitHub currently enforces the specification. It is anchored to `release/v0.5` baseline tree `94b8595153c3acfa8f171928ede0230c726a2cec` plus the reviewed source in draft PR #619 (original inventory review head `75eda100d8461130baf2063dee96097d483c548f`). Reinspect the files below after changes. The alternative Phase A workflow on `release/v0.4` must not be confused with the `release/v0.5` job graph.

## Authority, triggers, and public entrypoints

| Workflow | Trigger and owner | Effect and boundary |
| --- | --- | --- |
| `.github/workflows/ci.yml` | Ready PR opened/updated or `ready_for_review` to `main` / `release/**`; qualifying push; explicitly dispatched check | `plan` gates all 18 CI job definitions. Draft `pull_request` events skip plan and **allocate no runner**; manual dispatch on draft remains nonqualifying developer feedback. Full gate executes the `Justfile` `vv` recipes, Atlas proof shards, and native host checks. `.github/ci-plan.sh` decides full vs stacked/feedback vs same-tree merge. |
| `.github/workflows/reproducibility.yml` | Same push, PR, `ready_for_review`, manual events as CI | Independent job `repro`, command `just repro`, resolves to `cargo xtask check-reproducibility`; draft PR events allocate no runner. **Its separate status is not owned by the recipe inside `vv`.** |
| `.github/workflows/tag.yml` | Push to `release/**` | `tag` validates the successful acceptance workflow through `.github/release-gate.sh` before tagging the cut and dispatching `release.yml`. This is not an alternate proof gate. Draft tag-gate hardening in PR #607 is **not qualified/merged**. |
| `.github/workflows/release.yml` | Version-tag push / manually dispatched | `identity -> [crate,gate,binaries] -> image/manifest -> publish`. Verifies a past successful exact-tree acceptance receipt, packages binaries/container/SBOM/checksums, and runs `cargo xtask release-check`. Publication is conditional on tag, gate, and credentials; none is a substitute for acceptance. |
| `.github/workflows/crate.yml` | Reusable workflow call from release and manual prerelease dispatch | `readiness` checks byte-identical `cargo package` outputs, dry-run and release-artifact hash; `prerelease` upload conditional on explicit dispatch and publishing authority. |
| `.github/workflows/cut.yml` | Push to `cut/v*` | `version` runs `cargo xtask bump-version` and creates cut PR; calls `regenerate.yml` to regenerate reviewed artifacts on the cut. |
| `.github/workflows/regenerate.yml` | Push to `regenerate/**`, dispatch, or workflow call | Writer-only path: `identity -> writer -> combine -> finish -> deliver`; `just relock-identity`, `verify-write`, `golden-write`, `fixtures-write` and other named writers. This path **writes** generated expectations; it is never a successful `vv` acceptance receipt. `.github/scripts/deliver-regenerated.sh` checks HEAD, applies a bounded patch and explicitly dispatches CI. |

The exact event and permission definitions are those in the seven workflow files; the table is not permission to add or drop event types. `SPEC.md` §§8.3, 9.2, 9.4, 17.17, 23.1, 28.2-28.6, 29.5 and 30.3-30.4 govern the related host, acceptance, proof, reproducibility, regeneration and release obligations.

## Normative CI graph and terminal execution ownership

All 19 `ci.yml` jobs are inside the `plan` dependency graph. `RP-14` in `crates/conformance/src/cases/repository.rs` requires planner custody and refuses unconditional `always()` bypass. Grouping below is explanatory only; individual matrix/part jobs retain their native receipts.

| CI jobs (exact source identifiers) | Dependencies / actual terminal responsibility | Unique detector or evidence owner |
| --- | --- | --- |
| `plan` | Event-aware `.github/ci-plan.sh`, from committed `.github/vv-rows.json` and `.github/bdd-parts.json`. Skips automatic Draft PRs. | Full/feedback classification, exact-tree reuse policy, row selectors, no runner allocation on automatic drafts |
| `honesty` | `plan`; `just` selected honesty recipes, `cargo xtask check-freshness`, host-lane Python tests, CI summary | Early format/model/spec-links and BDD honesty; must follow successful plan |
| `bootstrap-export` | `plan, honesty`; `cargo xtask export-bootstrap`, same-run artifact | Single bootstrap export; importers must use same-run artifact and may not infer prior-run proof |
| `vv` | `plan, honesty`; every planned row but `honesty` and the `bdd-bootstrap-*` rows (`fanout`); per-row `just` recipes; `cargo xtask ci-summary`; `.github/scripts/take-bootstrap-export.sh` when needed | Committed `vv` recipe partition, each recipe exactly once, BDD partitions incl `rest`, `examples` plus generated proof/fixture equality, no skipped required part |
| `vv-bootstrap` | `plan, honesty, bootstrap-export`; the `bdd-bootstrap-*` rows (`bootstrap`), the steps of `vv` line for line under the same `just vv (<part>)` context | Starts only after a successful export, so its legs do not hold runners waiting for it (#603); a failed or skipped export skips every leg, which no whole-gate predicate accepts; RP-14 requires the two matrices to partition the planned rows |
| `atlas-prefix`, `atlas-cert4`, `atlas-l1`, `atlas-census-tail`, `atlas-cert4-replay`, `atlas-closure`, `atlas-l3`, `atlas-join` | `plan` + explicit predecessor DAG. `xtask verify-examples` on selected Atlas shards and same-run proof artifacts; `atlas-join` enforces complete expected verification records | Lean/kernel checks, shard coverage and dependency custody, normalized oracle comparison and all-rows join |
| `host-matrix` | `plan, honesty`; `cargo test --locked --workspace --all-features --no-fail-fast` on Linux AArch64 and macOS AArch64, with actual rustc host-triple check | Native two ARM hosts (not emulated on Linux x86) |
| `intel-construction`, `intel-part`, `intel-join` | `plan`; `.github/scripts/intel_host_tests.py build / execute / join`; producer builds once, parts execute same-run construction, join validates every lane and the final host status | Native macOS x86_64 all-features workspace test coverage, construction SHA/cargo/toolchain provenance, complete lane set |
| `windows-part`, `windows-join` | `plan`; `.github/scripts/host_lanes.py execute / join`, parts `producer,lower,print,validate,moves`; join verifies the complete inventory and exit statuses | Native Windows x86_64 workspace test coverage and all lane/host receipts |

The normative Linux x86_64 recipes run on `ubuntu-24.04` as committed; the other four SPEC §8.3 hosts are: Linux AArch64, macOS AArch64, macOS x86_64 and Windows x86_64. Their successful job names and joins are evidence, not permission for a source-only Green. The complete recipe owner is `just vv`, expanded as:

| Recipe in `Justfile` | Terminal detector command |
| --- | --- |
| `fmt-check` | `cargo fmt --all -- --check` |
| `model` | `cargo xtask validate-model` |
| `spec-links` | `cargo xtask validate-spec-links` |
| `lint` | `cargo clippy --workspace --all-targets --all-features -- -D warnings` |
| `test` | `cargo test --workspace --exclude repo-conformance --all-features` |
| `features` | `cargo check --workspace --all-features --all-targets` |
| `bdd` | `cargo xtask bdd`, partitions from `.github/bdd-parts.json` and `crates/conformance/tests/conformance.rs` |
| `inventory` | `cargo xtask check-test-inventory` (RP-13 partition/unique detector ownership) |
| `examples` | `cargo xtask verify-examples` (strict normalized `expected/verify` comparison) |
| `golden` | `cargo xtask check-golden` |
| `repro` | `cargo xtask check-reproducibility` |
| `deny` | `cargo deny --all-features check` |

`Justfile` additionally defines **writer-only** `model-write, golden-write, fixtures-write, production-export-write, calculus-write, verify-write, relock-identity`. Every writer is a distinct authorized regeneration operation, **not** a substitute acceptance detector. The negative fixture suite is owned by `conformance_ex_07` and `just fixtures`; proof preservation and helpdesk tests are conformance detectors, not replaceable by metadata or a green workflow label.

## Nested script/command chains inspected

| Source | Nested execution, trust boundary and its parent |
| --- | --- |
| `.github/ci-plan.sh` | Calls `jq` to partition actual `vv-rows.json`; invokes GitHub API via `gh api` to check exact-tree previous successful runs, all returned job conclusions, and second-parent tree equality. Called from CI `plan`; never grants proof from a skipped or different tree. |
| `.github/scripts/take-bootstrap-export.sh` | `gh api` looks up current run `bootstrap-export` artifact and producer status. Runs in `vv` consumer; no artifact from older runs accepted. |
| `.github/scripts/intel_host_tests.py` | Nested `cargo metadata`, `cargo test --locked --workspace --all-features --no-run --no-fail-fast --message-format=json`, `cargo test --locked --workspace --all-features --doc --no-fail-fast`, executes enumerated test binaries on Intel, rechecks native rustc/cargo/system pin and SHA. All lanes joined; one detached successful build is not a successful host. |
| `.github/scripts/host_lanes.py` | Nested `cargo metadata`, `cargo test --locked --workspace --all-features -- --list`, native `rustc -vV`, then per-lane test subprocesses and inventory join. Called by Windows parts/join. |
| `.github/scripts/ci_read_trace.py`, `read-trace-{start,stop}.sh`, `.github/actions/read-trace-start`, `.github/actions/read-trace-stop` | Advisory (#603 phase 1). After checkout in the 11 Linux gate jobs (`honesty`, `bootstrap-export`, `vv`, the Atlas jobs), `sudo` bind-mounts the workspace onto itself and records every fanotify open/exec/directory-open on it until the job's last step, which prints a digest-checked report into the job log (no artifact). The report is bound to checkout `HEAD`/tree, run/attempt, matrix row/index and recorder digests. | **Not an obligation, not evidence of a pass, and never sufficient to skip one.** Decides nothing; never fails a job (no `continue-on-error`; each action is one step without `-e` that ignores its script's status and exits 0; no upload step). Bounded: 1 GiB address space, OOM-first, bounded kernel queue, 100,000-path cap, bounded loop turn; unmounts on every exit. Blind to `stat`-only probes, failed opens, symlinks leaving the workspace, pre-mark descriptors, environment, network, clock, and other control flows; overflow, cap or error makes it `usable: false`. Native hosts untraced. |
| `.github/scripts/relock-identity.sh` | `cargo build`, `cargo xtask semantics-records --write`, locked identity updates, guarded git writes; used only by regeneration. |
| `.github/scripts/deliver-regenerated.sh` | `gh api` exact PR-head discovery, `git apply --index`, commit/push, `gh workflow run ci.yml`; guarded by same-head/custody checks. |
| `.github/release-gate.sh` | GitHub CI-run and paginated job inspection, required `just vv (part)` statuses from `vv-rows.json`, selected host/Atlas join checks, same-tree lineage and required run-bound acceptance artifact retrieval. Called from tag/release workflow. PR #607 proposes hardening successful provider-receipt handling and remains separately unqualified. |

For nested `cargo xtask` commands, the implementation is the `xtask` crate, with executable conformance cases in `crates/conformance`. Do not infer that a nested command's name means an independently verified status: compare actual checkout SHA, exact `--locked` options, package/part and output artifacts.

## Cross-workflow overlap and status-ownership decisions

**Confirmed duplicated terminal command:** CI's `vv/rest -> repro -> cargo xtask check-reproducibility` and independent `reproducibility.yml -> just repro -> cargo xtask check-reproducibility` run the same named verifier on the same revision. **Do not delete either invocation yet.** They have distinct status ownership and possibly distinct environment and failure/custody evidence. The independent workflow remains authoritative until a source-bound detector/receipt parity test plus a governed effective required-status change proves safe removal. `vv` excluding the conformance suite from `test` and assigning it to `bdd` is a **deliberate existing intra-gate deduplication**, checked by RP-13; it is not a second savings claim. BDD bootstrap partitions share a single same-run export, and Atlas shards divide native proof work with an explicit join. Never deduplicate a failure detector merely because two command lines look similar.

| Status owner | Required acceptance implication |
| --- | --- |
| CI `just vv (<part>)` jobs and `honesty` | All committed rows complete and succeed at one accepted source revision; no omitted BDD part; only same-run bootstrap evidence |
| CI `atlas (join)` plus proof-producing predecessor jobs | All normalized Atlas records/Lean checks present, verified and joined |
| CI `build and non-Lean tests (<target>)` for the supported host set | Every SPEC §8.3 host succeeds; Windows and Intel joins validate per-lane success, not just successful construction |
| Independent `repro` | Separate reproducibility workflow on the same ready head completes successfully |
| `tag`, `release`, `crate`, `cut`, `regenerate` | Publish/regenerate only with their own explicit guards and receipts; do not treat these as a substitute CI status |

**GitHub enforcement fact, not an assumption:** on 2026-11-23, GET `/repos/UOR-Foundation/lexlean/branches/release/v0.5` returned `protected=false`, `protection.enabled=false` and **no enforced required status contexts** on base `94b8595153c3acfa8f171928ede0230c726a2cec`. Active visible rulesets `governance-6.2-t1` and `main` are default-branch oriented; branch-protection admin endpoint independently returns 403 for this GitHub App. Therefore automated **required-check enforcement on release/v0.5 is absent in the readable branch metadata**; the exact authoritative platform configuration is bounded by app permissions. **Merge must be manually refused** until *all* SPEC-required statuses, receipts, provenance and independent repro satisfy the full ready gate. A `mergeable=true` API result proves no gate. The repository owner must verify/establish an appropriate required-check rule separately if automatic enforcement is desired. Do not bypass any normative rule to land #619.

## Completion versus production acceptance

DOD-1 is an inventory: source/workflow/triggers, terminal and nested command graph, unique detectors, proof/host/release paths, cross-workflow overlap, and the **observed limitation of enforcement authority** are recorded above. It does not require a passing new full CI run. DOD-2 is architecture comparison and scenarios; DOD-3 is actual source implementation; DOD-4 is executable exact-head all-host/proof/repro qualification; DOD-5 is governed merge; DOD-6 is comparable outcome measurement. Those later obligations may remain open without invalidating the completed inventory.

To keep this inventory live, diff the seven workflow files, `Justfile`, `.github/ci-plan.sh`, `.github/release-gate.sh`, both JSON partition manifests, nested host/export/regeneration scripts, `SPEC.md`, and the Rust conformance registry whenever the gate changes. If source and inventory disagree, treat DOD-1 as reopened until reconciled. See issue [#603](https://github.com/UOR-Foundation/lexlean/issues/603) for the canonical live checklist.
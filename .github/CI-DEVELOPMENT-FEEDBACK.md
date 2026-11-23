# CI development feedback: Phase B pilot (nonqualifying)

**Owner issue:** [#603](https://github.com/UOR-Foundation/lexlean/issues/603).
**Baseline:** full qualified PR #602 used **1,051.85 unweighted completed runner-job minutes** across main CI and standalone reproducibility. This is an observed starting point, not a predicted saving from this pilot.

This is an **optional, explicitly invoked** feedback workflow, independent of the normative `ci.yml` and `reproducibility.yml`. It **never** replaces SPEC.md §9.4, required checks, exact-HEAD native/proof/release evidence, or RP-13/14. Do not add its status to required merge checks. It has no `push` or `pull_request` triggers, so routine draft and ready PRs do not allocate workers for this pilot.

## What runs

The workflow `.github/workflows/development-feedback.yml` checks out the exact manually dispatched HEAD without write credentials, fetches the authorized `main` or `release/vX.Y` baseline, requires that baseline to be an ancestor, and computes the actual NUL-delimited, rename-aware `git diff` against that base. An unavailable/stale/foreign base or malformed diff **fails**, not a guessed partial plan.

The stdlib-only `.github/scripts/ci_impact_preview.py` chooses real actions within a **single** bounded Ubuntu worker:

- Changed tracked files: `git diff --check` for whitespace mistakes.
- Any changed, renamed or deleted Rust source (`.rs`): pinned-toolchain `cargo fmt --all -- --check`.
- Any changed CI Python script/test under `.github/scripts/`: actual `unittest discover` for the existing CI Python contract tests.
- Other paths (including model or generated inputs, README, SPEC, policies, lockfiles, toolchains, unknowns): **not proven unaffected** and not classified as harmless. The preview still performs available cheap checks but explicitly leaves the full normative obligations unverified.

This **is not yet a transitive dependency-to-verifier graph**. Its small, conservative classification demonstrates selected execution, actual commands and explicit unknowns without changing the protected full workflow. Once the project-specific impact closure and threat/detector map have passed negative controls, Phase B can compare deeper graph selection against stage placement and conventional work-reuse alternatives.

## Use and verification

The workflow is manually dispatched using **Actions → CI development feedback (NONQUALIFYING) → Run workflow**, selecting the source branch and specifying `base_ref`. GitHub Actions generally requires a `workflow_dispatch` workflow to exist on the default branch before manual runs are available; this PR alone does not establish that it is dispatchable.

To validate without any hosted runner:

```sh
python3 -m unittest discover -s .github/scripts -p 'test_ci_impact_preview.py' -v
```

The local tests plant Rust deletion, cross-language rename, CI Python, model/README/lockfile changes, bad statuses, truncated NUL diffs, invalid paths, stale/malformed revision IDs, false green workflow triggers and missing required boundary. An isolated green test suite is not live provider execution or economic outcome proof.

## Next gated phase

Before replacing any `ci.yml` ready-PR or release path: inventory every terminal command/required context, build the trusted source→transitive obligation map, reconcile RP-13/14 and SPEC against actual source, compare two or more alternative graphs and seeded detection failures, agree on required-check migration, and prove actual like-for-like runner-minute and critical-path savings. **Keep the full gate unchanged until those controls are met.**

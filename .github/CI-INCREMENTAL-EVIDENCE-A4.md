# A4: incremental verification evidence, prototype boundary (#603)

**Status: implemented read-only advisory classifier on Draft PR #606. NOT a
production CI gate, merge check, release receipt, or authorization to skip any
normative verification.** The full SPEC 9.4 gate still applies to PR #619.

Problem demonstrated by CI #45256672496: successful BDD/native/Atlas jobs were
spent again across changing PR heads. RP-14's shared source validator failed
across multiple hosts; the Windows join correctly failed closed. This is
observed duplicated *execution*, not proof that earlier results can survive
an altered shared test binary.

## Components

- ci_incremental_reuse.py: read-only evaluator of an exact checked-out Git
  ancestry and NUL-safe rename-aware source diff, a per-unit declared impact
  policy, and prior GitHub provider-observation data. No Actions dispatch,
  GitHub status creation, release action, network access, or mutation.
- test_ci_incremental_reuse.py: positive *candidate* evaluation, full negative
  controls for invalidation, stale or foreign receipts, dependency cycles,
  duplicate job IDs, incomplete source/evidence, changed toolchain/workflows,
  unknown paths, and a cross-platform failure.
- Existing ci_impact_preview.py, ci_gate_graph.py, ci_generated_lineage.py
  and ci_execution_receipts.py remain the independent source and metrics
  inputs. No policy is inferred to be complete merely because the graph
  script was able to parse it.

The advisory CLI now refuses to classify a changed HEAD until the real
checked-out conformance-to-host ownership auditor succeeds. Its output attaches
exact SHA-256 values of the eight inspected ownership source files. This does
NOT certify transitive dependencies or authenticate a prior GitHub job. Source
audit failures are HOLD and cannot result in positive advisory suggestions.

Every unit must explicitly classify EACH changed path as an input or as
explicit nonimpact. Unknown paths mean RERUN. A changed control file or policy
forces RERUN. LexLean's 524-case shared conformance source, native lane
planners and test/receipt custody logic are mandatory invalidation controls:
a hand-written "nonimpact" claim cannot override them. This conservative
file-level policy deliberately yields no cross-HEAD reuse for the RP-14 fix.
Missing/failed/cancelled receipts and downstream dependents
force RERUN. Native, Lean/proof, repro, and release kinds are **always RERUN**
in this prototype. The algorithm emits CANDIDATE_REUSE only for declared
ordinary units, never a validated carry-forward receipt.

CLI, when a versioned policy exists unchanged at both Git revisions:

    python3 .github/scripts/ci_incremental_reuse.py \
      --repo-root . --old <40-character-commit-sha> \
      --head <40-character-commit-sha> \
      --policy path/to/versioned-policy.json \
      --observations path/to/provider-observations.json

The policy schema is lexlean/ci-incremental-reuse/1 with repository,
global_inputs and units. Units have id, job, host, definition (fingerprint),
kind, inputs, explicit_nonimpact and needs. Provider observations use schema
lexlean/ci-provider-observations/1 with run repository/id/head/status/
conclusion and jobs each declaring unit/job/id/run_id/head/status/conclusion/
host/definition/evidence_digest. SHA/digest strings are exact, but **a digest
value supplied in JSON does not authenticate its publisher**.

## Authority still missing before real changed-HEAD reuse

1. Source-bound, adversarially audited per-test command/fixture/toolchain/
   generated-output/dependency closure, including Rust shared harness changes.
   Hand-authored explicit_nonimpact is NOT sufficient to establish closure.
2. Provider-authenticated successful origin job and immutable artifact
   identity, using a constrained read-only repository-scope identity.
3. Current-HEAD execution-and-reuse manifest with complete normative
   detector inventory, verified dependencies and freshness, fail-closed join,
   published *current-HEAD* required context, and source-authorized release
   lineage. GitHub skipped jobs and green historical checks do not substitute.
4. Coordinated changes to spec/src/Chapter09.lex.tex, generated SPEC.md,
   RP-13/14, the actual workflows, and release-gate.sh, plus positive/negative
   hosted acceptance. The current same-run rule MUST remain enforced until
   this work is complete.
5. Matched provider job-minute and critical-path comparison against A3,
   without claiming that 61 already-green jobs are automatically reusable.

**Do not promote this code into normative ci.yml or release-gate.sh.**

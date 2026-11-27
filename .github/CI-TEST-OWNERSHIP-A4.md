# A4-2: source-bound conformance and native-lane ownership (#603)

This is a **nonqualifying read-only audit**, not a license to reuse old
results or skip tests. See ci_incremental_reuse.py for advisory decisions.

The analysis reads the exact checkout's generated model ID register,
conformance test wrappers, case dispatch, harness selector, BDD partition
manifest, native host lane partition manifest, and normative VV recipe rows.

It proves only a **declared, source-bound ownership mapping** and computes
SHA-256 for every inspected source. It refuses drift from missing or duplicated
IDs, wrong dispatch, bad target ownership, altered RP-14 lane, missing VV rows,
duplicate/foreign native lanes, changed harness selection and malformed data.

Run without GitHub API calls or runner dispatch:

    python3 .github/scripts/ci_test_ownership.py --repo-root .

In Draft pilot #606 the register currently has 521 tests and 54 named BDD
partitions; the current production #619 source has a *different* inventory
(524 tests and 37 named partitions). Never carry a static count or part
mapping from one tree into another. RP-14 is owned by BDD rest, mapped to
host moves. Linux VV/rest also owns this case.

The conformance tests share one compiled Rust crate and test binary.
Source ownership is not the complete source-to-compiled-code/fixtures/oracles/
toolchain/proof dependency closure. A change in repository.rs may invalidate
other case results even if the source mapping says only RP-14 is named there.
This tool explicitly marks source-to-runtime closure INCOMPLETE and cross-commit
reuse authority false. The A4 classifier globally invalidates on shared
conformance source, so a changed RP-14 file does not reuse old green jobs.

Remaining work: prove transitive module/test-level dependencies and actual
native/Lean/oracle imports, attach authenticated origin job+artifact receipts,
then build a current-head owner-only join and revise SPEC/RP-14/release gates
with independent negative controls. Full existing CI remains authoritative.

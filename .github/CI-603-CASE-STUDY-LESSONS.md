# CI #603: case-study decision and failure ledger

This is a working evidence ledger, not a final claim of causality or measured ROI. Retain immutable raw snapshots; append new observations rather than rewriting historical ones. A = original architecture, B = generic CI changes, C = additional Rust tuning only after B is frozen. Distinguish unweighted execution time from billed usage, and time to green from PR-to-merge latency.

| Observation / decision | Evidence | Lesson / follow-up |
| --- | --- | --- |
| Automatic Draft events previously consumed runner time despite having no merge authority | #603 historical baseline, #619 architecture decision | Suppress automatic nonqualifying work; retain explicit focused feedback and full Ready acceptance. |
| Source-aware impact classification is not itself a proof of completeness | #606 A4 source ownership, input closure and negative controls | Unknown dependencies invalidate; do not use declared nonimpact or historical green alone as authority. |
| Same-source selective failure recovery retained 69 completed executions | CI #45316289312 attempts 1 and 2; metrics/ci-v05-targeted-retry-provider-snapshot-20261124.json | Preserve original execution identity, not reissued job IDs; 2,403.383 unweighted job-minutes retained, not billed savings. |
| One ARM64 retry took 197.683 minutes | Job #138255634209, terminal success | Long-tail workload and infrastructure failures magnify the value of selective retries. |
| Exact-tree merge preserved qualification | #619 merge 44616baa, tree 2f618ea9, ordered parents 94b85951 and bd80fa37; 70/70 CI plus independent repro | Verify actual resulting tree and source lineage, not merely mergeability. |
| GitHub required-status enforcement was deliberately absent | #603 owner-directed minimum effective governance decision | Do not restore restrictive settings wholesale. Evaluate history preservation and traceability, plus a future trusted aggregate verdict, by assurance benefit versus friction. Manual qualification is not a waiver of evidence. |
| Same-tree planner unexpectedly ran full CI after an HTTP 502 | CI #45421145394, plan job #138540685551, error at 2026-11-24T15:05:12.0329232Z; metrics/ci-v05-post-merge-provider-fallback-20261124.json | Provider availability is distinct from evidence invalidity. Bounded atomic read retry (#620) before conservative fallback; reject partial results. A fallback is safe but expensive. Record actual wasted/duplicated work and do not claim merge reuse succeeded. |
| Post-merge independent reproducibility passed; ordinary merge did not publish a release | #45421145205 and #45421145299 | Separate independent assurance and publication authority from redundant same-tree verification. |

At snapshot: 67 observed jobs, 48 completed, 19 active; 1465.450 completed unweighted job-minutes (incomplete, includes planner). No final duplicated-work total or billed-dollar claim. No Rust-specific changes in B. Continue recording per-change validation, failure classification, actual cost, critical path, authority decisions, and negative results. Publish charts and comparative case study after measured phase dispositions.

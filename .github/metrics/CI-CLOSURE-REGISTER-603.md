# #603 closure register — bounded execution

Owner direction: finish the existing DOD-6 through DOD-10 without dramatically increasing scope; preserve case-study validity and reopen ordinary contributor flow at operational acceptance. This is a working closure register, not a waiver of normative checks or a change to the definition of done.

| DOD | Existing evidence | Remaining closure decision |
| --- | --- | --- |
| 6 | Corrected 50-PR BEFORE, qualified initial cutover, pre-B1 cohort and B2 pilot | Qualify #630/#633 once, measure allocation and elapsed/feedback with queue confounders, verify landing and reuse. Freeze B before C; record owner-approved outcome if no measured win. |
| 7 | Hosted thread-count experiment rejects two-thread cap for tested workload; existing rendering fix f100b2cd locally improves 321.3 to 14.2 seconds with byte-identical output | Complete #634/#624 recovery, measure rendering improvement on ordinary qualification, record co-changes. Reassess critical path; do not add ARM partitioning or nextest conversion without material demonstrated need. |
| 8 | Historical arrival/cost replay, compatibility review and one real #607/#609 combined PR qualification | Disposition #628/#631 custody work; choose qualified adoption or retain conservative recovery. Recommend bounded manual compatible composites under existing authority, not automatic batching infrastructure; owner decision required. |
| 9 | Existing source register, comparative architecture review, DAG and finite-fleet models | Reconcile material external claims against frozen internal B/C evidence. Adopt only material proven improvements; explicitly disposition other candidates. No requirement to exhaust literature or implement every suggestion. |
| 10 | Frozen receipts, replay tooling, provisional operational handoff, retained benchmark archive | Finish evidence reconciliation, final configuration and report, limitations, transferable CI Guard lessons, technical and owner review. Natural 50-PR AFTER cohort or explicitly owner-approved comparable design; no fabricated samples or publication without approval. |

## Execution boundaries

- #630 b5bfe17c is reviewed and draft. Its previously approved single qualification is the immediate B1 dependency; Claude owns Ready transition. No duplicate trigger.
- #634 b71bc518 requires two targeted independent-review corrections, then #624 4660137a must integrate the reviewed patch-set implementation and identity stage conversion on the actual base. Never reuse the stale artifact on a new base.
- #628/#631 stay unqualified until their real POSIX/native and retained-attempt evidence is complete. Do not activate changed-head or cross-attempt acceptance prematurely.
- Freeze A → B1/B2 → C before externally prompted D refinements. Collecting outside evidence is not authorization to introduce new changes into an internal phase.
- New work belongs in the closure path only if necessary to satisfy an existing criterion, fix a demonstrated correctness issue, or remove a measured material blocker. Otherwise defer to a separately tracked follow-on after #603. No speculative new workflow lanes, provider migration, tool replacement or additional benchmark matrix just to enrich the report.
- Normal draft/local development is available now; final operational handoff is separate from longitudinal observation. Keep all release/proof/native/repro safeguards. No criterion is marked complete merely to reduce backlog.

## Independent replay

At commit 0707667a50ceb9a3fcb9d7d40947e77cae46a3dc, run `node .github/metrics/dod10/replay-frozen-receipts.mjs` from the advisory checkout. Five inputs pinned by Git blob identity; pure replay exercised in JS against 50 PRs, 128 historical run rows, 12 terminal first-attempt run receipts, and ten independent negative mutations (all refused). Node CLI wrapper not independently executed. Summary replay is not provider authentication or full historical attempt reconstruction.

Corrected BEFORE: 88,298.354 unweighted runner-minutes; median 1,818.701 (0.001 rounding difference from earlier display), p90 2,615.012. Premerge green: 29/50, median 186.233 minutes, p90 450.963. No AFTER or causal savings claim. Failed/cancelled and zero-allocation skipped receipts remain included where applicable.

Formal status remains 5/10; the purpose of this register is to complete the remaining criteria, not expand them.

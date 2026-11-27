# DOD-8 decision packet: bounded manual composites; no automatic queue

2026-11-26 UTC. Recommendation for owner disposition, not an adopted workflow or waiver. The existing complete normative gate, independent reproducibility, release authority and exact-tree custody remain mandatory. This document resolves the choice of next architecture to recommend, not the outstanding qualification of #628/#631.

## 1. Evidence already available

The historical 50-PR screening found 22 selected costly release-line gate/repro pairs totaling 33,998.731 unweighted runner-minutes. This is a selected positive-cost/success subset, not the cost of all 50 PRs, and the arrival proxy is the first selected successful CI creation rather than actual Ready events. All hypothetical members were assumed compatible in the initial sensitivity; later review classified only 3/22 as review candidates, 2 unknown and 17 other categories (including identity, release, control repair, product repair/control and security). Only 14/22 have complete listed path inventories. The later admission review found **zero approved hypothetical multi-member batches** because required eligibility and exact composite evidence were absent. This is a conservative evidence outcome, not proof that no pair could ever be compatible.

| Hypothetical size / max age | Gate invocations vs 22 | Median / p95 staging minutes | Modeled runner-min without recovery | With one extra gate per multi-batch |
| --- | ---: | ---: | ---: | ---: |
| 2 / 6h | 15 | 161.85 / 360 | 24883.482 | 36145.749 |
| 3 / 6h | 14 | 360 / 360 | 23470.699 | 33320.183 |
| 4 / 24h | 9 | 603.15 / 1440 | 15357.232 | 26598.932 |

For the two-member six-hour scenario, even a single extra gate per multi-member batch raises modeled cost about 6.32% above the selected independent cost. The larger/older scenario trades lower modeled compute for potentially a day of staging; these are not measured throughput or acceptable contributor delay. The complete nine scenarios and their assumptions remain in the original sensitivity ledger. No configuration is justified as an automatic default.

Actual compatible composite #607/#609: exact-head complete PR gate 45490646681 (86 success, 3 planned skips) and independent repro 45490646770 passed; 1,866.2 allocated runner-min, 200.517 min CI elapsed. Four draft checks zero allocation. Exact qualified tree e736f044 landed as fa2280e2. Immediate four postmerge runs add 372.367 allocated min, including a cancelled CI (Intel incomplete) and failed forward; scoped total 2,238.567 min. These are real observations but exclude earlier preparation/regeneration/later recovery and do not identify the unobserved separate-gate counterfactual. Do not claim a measured batch savings, full lifecycle improvement, or frictionless release.

## 2. Recommended disposition

- Retain existing direct route as default, with urgent work using the same complete normative gate without waiting for another change.
- Allow manually reviewed compatible two-member composites under existing authority, only where both changes are already available, nonurgent, on the same release line, and explicit membership and dependency review are complete. Two is a proposed starting bound, not a new required policy. No mandatory waiting window; absent an immediately compatible peer, proceed directly. Do not enqueue an identity, trust, release, security, control-repair or unknown change by default.
- Do not adopt native merge queue, automated batch admission, speculative composite qualification or a new staging workflow for #603. Reconsider only on measured arrivals, conflict rate, time-to-green, full lifecycle cost and sustained queue pressure after B/C are stable.
- Retain current authorized same-source provider-native recovery only when its actual lineage, obligation coverage and downstream dependencies can be reconciled. Do not widen cross-attempt artifact acceptance or changed-HEAD reuse on the strength of synthetic tests. #628/#631 remain separate candidate corrections; no authorization to bypass review or dismiss demonstrated verifier defects.

## 3. Manual composite acceptance boundary

Before Ready: freeze member head identities, target/base SHA, composition order, exact final source/tree, dependency/conflict assessment, urgency and owner approval. Check each member's unique obligations remain present, run existing local honesty and negative controls, and verify no duplicate active qualification. An unknown member, changed base/tree, missing receipt, unauthorized fast route or incompatible identity change is HOLD or direct full-gate requalification, never implicit admission.

Before merge: exact current candidate complete normative CI, independent reproducibility, native/proof/receipt integrity and release controls; verify resulting tree and ordered ancestry preserve accepted composite. Skipped draft status is not approval. Recheck applicable governance and current base/head before merge; inspect postmerge planner, repro, forwarding and tag behavior afterward.

Failure: identify affected obligations; do not merge individual members on a failed composite or retain successes across a changed tree without proven authority. For infrastructure interruption use bounded authorized same-source recovery, with complete provider receipt reconciliation. For deterministic failure, isolate member, correct or split the batch, and qualify the exact resulting candidate. Keep direct urgent escape and auditable rollback; no force push or published cut merely to clear a queue. Count all actual failed/cancelled/recovery allocation; report staging, review, queue and CI separately.

## 4. Remaining DOD-8 disposition

8a historical classification/sensitivity is available; 8b admission and failure-model evidence is advisory; 8c existing real pilot is observed with limitations above. 8d needs explicit owner approval of bounded manual composites/no automated queue, and final disposition of #628/#631 with their independent exact-source and real provider/native requirements. No new hosted pilot is proposed solely to strengthen this memo. No primary or subcriterion checkbox is modified by this packet.

Source anchors (Git blobs):
- ci-batching-sensitivity-dod8-20261124.json d26efe4ccf2c92ac697c867639d17333603f4022
- ci-batching-admission-review-dod8-20261124.json a8024d04c537dfe665879b9a11af01080c0aa6dd
- ci-b2-composite-607-609-20261126.json b5bf3aa36e40296ea6a1dd44ca4a17d33e620919
- ci-b2-composite-607-609-postmerge-20261126.json a0058c1f417a5d98a9e857ec03aab71c94628cfa
- #603 comments 7142609342 (DOD-8 design), 7180974631 (observed composite), 7181152109 (bounded closure).

No normative source change, production dispatch, causal savings, final adoption or merge authority follows from this document.

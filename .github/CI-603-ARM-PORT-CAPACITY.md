# #603: ARM lane port boundary and capacity sensitivity

## Corrected history

The earlier macOS ARM partition was not removed from release/v0.5. GitHub compare of historical v0.4 cut head b693bb4259d311fd71da90ac38701214a74be27f and current #620 head 603336bf2752ecbfa4d688ccb5d52b3585bfa64b reports diverged, 299 ahead / 241 behind, common ancestor 6a02d2c7d519f1a4d169ebb3496fbad51bd259cb. Commit 8154aa0bf0aa529763c298a26c97a24389543bb3 implemented macOS lanes on the maintenance line and is an ancestor of current release/v0.4 c0cbb715 (196 commits ahead, zero behind). It is not present in the v0.5 workflow, which explicitly retains two monolithic ARM hosts. The historical and current host_lanes.py Git blobs are identical (bbe3c9b8a145100456f02753209e9c2a2423d3ef). This is a forward-port candidate, not a rollback of a removed optimization.

## Exact port boundary

Use 8154aa0b as a reference, not a wholesale cherry-pick: it contains maintenance-line SPEC statements that defer Intel on certain PRs, unlike v0.5. Preserve every v0.5 supported-host obligation. In the next coordinated candidate:
1. Remove only macOS ARM from host-matrix, retain Linux ARM unchanged initially, add five native macos-part lanes and a Linux macos-join with the existing exact aggregate name. Keep pinned actions, current retry.sh Rust installation, full-gate/honesty dependency and current timeout policy. Resolve cancelled-run join policy explicitly; do not accidentally import a different Windows guard.
2. Port the §8.3 and §9.4 changes into the authoritative v0.5 spec modules and regenerate projections through the governed process, not by copying v0.4 SPEC.md. Preserve all four exact supported-host contexts, including Intel on every full gate.
3. Generalize RP-14 windows_host_workflow into a host-parameterized validator and update intel_host_workflow context inventory, standard-runner exemptions, join timeout treatment, honesty_dependencies roots and planted root checks. Preserve #620 provider tests and #621 tracing changes when integrated; their different versions of RP-14 must not be overwritten.
4. Port cross-host receipt rejection tests from the historical commit. Adversarially verify wrong host/commit/tree/run/attempt, missing/duplicate lanes, lane failure or cancellation, stale artifact and missing/extra/duplicate tests; assess receipt checks for test identity collisions, ignored tests and attempt provenance. Keep the exact same-workload inventory, including all doctests, meta gate and non-conformance targets. No new reuse authority.
5. Validate offline first. Only after #620 and the active Claude track are coordinated, qualify the resulting current candidate with an authorized hosted run, same exact source, all acceptance obligations and independent reproducibility. Measure actual per-lane setup, execution, queue, join, overall gate critical path, aggregate runner time and retry behavior. Linux ARM has no equivalent measured historical five-lane result in this investigation; evaluate separately.

## Conditional capacity model

Tested locally: python -m unittest -v, three test methods passed (0.009s). Model enumerates all static assignments for at most six independent lanes and 1..N continuously available identical runners. All lanes start eligible together, join follows last native completion immediately on another runner. No competition, queue delay or artifact wait beyond measured lane work; it is an ideal conditional bound, not a hosted forecast. Additional overhead is an explicit sensitivity per native lane.

Historical #45100057759 native job execution seconds: producer 694, moves 2207, validate 838, print 1690, lower 1986; join 30. Historical actual creation-to-join completion 3580s. Model results:

| Concurrent native runners assumed | No additional overhead | +5 min per lane | +10 min per lane |
| --- | ---: | ---: | ---: |
| 1 | 7445s | 8945s | 10445s |
| 2 | 3769s | 4527s | 5348s |
| 3 | 2710s | 3310s | 3910s |
| 4 | 2237s | 2537s | 2837s |
| 5 | 2237s | 2537s | 2837s |

Total executed job-seconds (regardless of capacity): 7445 / 8945 / 10445 respectively. Historical monolithic Nov 19 execution: 7480s; current v0.5 #619 original macOS ARM: 10942s. Different revisions and inventories; none is a controlled causal savings estimate. One available runner provides negligible advantage at historical baseline duration, and added setup can make partitioning worse. Four available runners already reach the longest native lane in this model; five offers no additional benefit. The earlier observed peak of three simultaneous native jobs is not a capacity guarantee.

## Decision

Recommend macOS ARM forward-port as a separately reviewable candidate once the shared workflow is available; do not change production during #620 qualification or conflict with #621. Do not claim Linux ARM parity, cost reduction, or a specific whole-gate time from this model. No hosted workflows requested, no changes to active implementation branches. Model and port contract are nonqualifying evidence.

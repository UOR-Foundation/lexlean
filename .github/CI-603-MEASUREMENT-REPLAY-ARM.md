# Independent measurement qualification and historical ARM scheduling evidence

## Queue snapshot exact replay

On the independent Linux container, reconstructed the full 67-job JSON snapshot from its committed provider records without changing bytes. Git blob hash verified: `c2161f09870a6147d4a9c9b49e76587d856c7920`. Executed the exact committed queue analyzer (blob `fd47003a5a5f7e6d98afb91caa66f4791a7cf5ce`) against that snapshot and re-ran all five fixture test methods (PASS, 0.002 s). All five runner-class summaries match the earlier independent connector calculation, including counts, median/max elapsed scheduling wait and completed execution seconds. The full local replay output Git blob hash is `654657d197bfd5787f8ba62f2f810616842e1825` (local output, not committed as a separate blob). This qualifies the queue analyzer's replay of that captured dataset, not live authenticated CLI retrieval or final cost. Active jobs remain unestimated.

## Historical macOS ARM host-lane evidence

The older metrics ledger lacked the aggregate join result. A new full provider inventory for run 45100057759 (78/78 unique jobs, terminal success) confirms all five native lanes AND Linux aggregate context `build and non-Lean tests (aarch64-apple-darwin)` passed. Original provider job timestamps, including join, are in `.github/metrics/ci-macos-arm-joined-history-20261124.json`.

| Observation | Native + join job-minutes | Job creation to host completion |
| --- | ---: | ---: |
| Nov 19 monolithic, run 44599982425 | 124.667 | 145m14s |
| Nov 23 five native lanes + Linux join, run 45100057759 | 124.083 | 59m40s |
| Nov 24 monolithic, original attempt of run 45316289312 | 182.367 | 182m24s |

The earlier partitioned workflow had peak three observed simultaneous native jobs and a 26m02s wait for its lower lane. Its native execution cost was 123.583m plus 0.5m for the aggregate join. The native host context elapsed span was 85m34s shorter than the Nov 19 monolithic observation, but that difference includes scheduling conditions and source/workload changes (502 vs 507 reported conformance cases). It is not a causal benefit estimate. The current ARM64 monolithic observation uses the original attempt-1 job, not the later carried reissued record's creation timestamp.

The historical workflow at `b693bb42` uses `host_lanes.py execute --target aarch64-apple-darwin` and a fail-closed Linux aggregate join; inspect why this was removed before restoring it. Reuse the existing lane design rather than inventing another one. Require exact current source/host/test inventory, artifact custody and detector parity; measure critical path and total runner minutes under a bounded candidate qualification. Do not infer that a two- or three-runner pool is guaranteed or that partitioning alone reduces billed cost. No Rust-specific runtime tuning, new CI execution or production branch changes in this work.

## Coordination

Claude's #621 is stacked on #620 and addresses source read tracing for change-scoped qualification. This independent measurement track does not modify those branches. Retain the current active #620 qualification and coordinate a single subsequent candidate rather than opening competing full-gate experiments.

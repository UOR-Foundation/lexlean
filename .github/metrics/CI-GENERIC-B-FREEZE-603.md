# Generic B phase: observed comparison freeze

2026-11-26 UTC. Advisory analysis, not authority. One matched full 70-job successful PR gate on each side; source, workload, host availability and queue differ. Not a controlled experiment or the natural 50-PR AFTER cohort. Billed usage and dollars unavailable.

| Measurement (seconds) | #621 pre-B1 | #630 B1 | Difference |
| --- | ---: | ---: | ---: |
| Full CI allocation | 111785 | 112375 | 590 |
| CI elapsed | 19041 | 11526 | -7515 |
| All-job queue sum | 111696 | 11943 | -99753 |
| Bootstrap allocation | 22635 | 22358 | -277 |
| Bootstrap queue sum | 65942 | 11341 | -54601 |
| Bootstrap pre-export overlap | 3384 | 0 | -3384 |

Both first attempts passed all 70 jobs and independent reproducibility. Wall time decreased 7,515 s (125.25 min); allocated CI increased 590 s (9.83 min). Bootstrap allocation decreased 277 s (4.62 min); zero pre-export allocation in B1. Queue sums are not elapsed or allocated compute. Pre-export overlap is not proof of idle time. #621 Intel validate alone queued 9,750 s. No causal attribution. Both runs succeeded; first actionable failure is not observed and cannot be inferred from honesty success.

B1 exact qualified tree 0fd20b0aa6fe55fffb8b445472161c87c4c88738 landed as 11d665cf4a27415dc4d34202d61984dfb841d353. Production reuse 45780833433 succeeded (66 s planner, 18 unallocated skips). Tag 45780833387 succeeded (59 s, no cut). Independent postmerge repro 45780833444 tracked separately. No completed longitudinal outcome or DOD-6 closure claimed.

B2 #607/#609: actual 111972 s PR CI + repro, 22342 s immediate postmerge allocation including cancelled CI and failed forward; 134314 s scoped total. Not full lifecycle or avoided independent-gate costs. Recommend existing manually approved compatible composite route, not an automatic queue.

## Attribution boundary

B observations frozen before C adoption, not a freeze of outstanding longitudinal evidence. Rust rendering f100b2cd local byte-identical result is a separate intervention; #634/#624 recovery and co-changes must be recorded for later measurements. Do not count a regenerated tree as B-only. No external D refinement added. Final owner DOD6/8 outcome decisions and AFTER remain open.

Pinned source blobs:
- ci-b1-630-terminal-20261126.json: 84307c7fa081415825f8be8a337d01bf04b2330b
- ci-dod6-terminal-621-623-20261126.json: 6c689141e4cc523baffa1c51a6c7dd4e51d8bfd4
- ci-b1-preintervention-621-complete-cohort.json: e34400b317024e6c1cf64dfadd75e68d580cf19b
- ci-b2-composite-607-609-20261126.json: b5bf3aa36e40296ea6a1dd44ca4a17d33e620919
- ci-b2-composite-607-609-postmerge-20261126.json: a0058c1f417a5d98a9e857ec03aab71c94628cfa

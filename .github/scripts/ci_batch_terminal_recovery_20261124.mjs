// DOD-8c: frozen observed batch economics and retry custody preflight. Advisory only.
// Exact run attempt 1 receipts captured from paginated GitHub API. Prior premerge roster independently verified in #603 comment 7149037363. Does not authenticate arbitrary external receipts or artifacts; not merge/release authority.
import {readFileSync} from 'node:fs';
const snapshot={
  "pre": {
    "id": 45490646681,
    "head": "a5e846de239113ca591f949eb75764d40096f7e9",
    "attempt": 1,
    "status": "completed",
    "conclusion": "success",
    "created": "2026-11-24T21:23:42Z",
    "updated": "2026-11-25T00:44:13Z",
    "total": 89,
    "jobs": [
      {
        "id": 138787742928,
        "name": "plan",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182577,
        "steps": 6,
        "start": "2026-11-24T21:23:45Z",
        "end": "2026-11-24T21:24:19Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138788102640,
        "name": "honesty",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182579,
        "steps": 11,
        "start": "2026-11-24T21:24:22Z",
        "end": "2026-11-24T21:28:20Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138790487035,
        "name": "build and non-Lean tests (aarch64-unknown-linux-gnu)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182588,
        "steps": 7,
        "start": "2026-11-24T21:28:25Z",
        "end": "2026-11-25T00:44:13Z",
        "labels": [
          "ubuntu-24.04-arm"
        ]
      },
      {
        "id": 138790487068,
        "name": "bootstrap export",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182591,
        "steps": 12,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T22:22:53Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138790487080,
        "name": "atlas (prefix)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182592,
        "steps": 13,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T21:59:23Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138790487239,
        "name": "Windows tests (print)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182585,
        "steps": 8,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T22:14:09Z",
        "labels": [
          "windows-2022"
        ]
      },
      {
        "id": 138790487300,
        "name": "Windows tests (producer)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182584,
        "steps": 8,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T21:43:04Z",
        "labels": [
          "windows-2022"
        ]
      },
      {
        "id": 138790487312,
        "name": "Windows tests (validate)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182586,
        "steps": 8,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T22:25:58Z",
        "labels": [
          "windows-2022"
        ]
      },
      {
        "id": 138790487348,
        "name": "Windows tests (moves)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182587,
        "steps": 8,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T23:19:04Z",
        "labels": [
          "windows-2022"
        ]
      },
      {
        "id": 138790487349,
        "name": "Windows tests (lower)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182595,
        "steps": 8,
        "start": "2026-11-24T21:28:22Z",
        "end": "2026-11-24T23:48:35Z",
        "labels": [
          "windows-2022"
        ]
      },
      {
        "id": 138790487517,
        "name": "macOS arm64 tests (lower)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182590,
        "steps": 8,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T23:35:31Z",
        "labels": [
          "macos-15"
        ]
      },
      {
        "id": 138790487536,
        "name": "macOS arm64 tests (moves)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182589,
        "steps": 8,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T22:37:30Z",
        "labels": [
          "macos-15"
        ]
      },
      {
        "id": 138790487566,
        "name": "macOS arm64 tests (producer)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182612,
        "steps": 8,
        "start": "2026-11-24T21:33:25Z",
        "end": "2026-11-24T21:46:48Z",
        "labels": [
          "macos-15"
        ]
      },
      {
        "id": 138790487571,
        "name": "macOS arm64 tests (validate)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182596,
        "steps": 8,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T22:34:51Z",
        "labels": [
          "macos-15"
        ]
      },
      {
        "id": 138790487644,
        "name": "macOS arm64 tests (print)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182665,
        "steps": 8,
        "start": "2026-11-24T21:46:50Z",
        "end": "2026-11-24T22:30:12Z",
        "labels": [
          "macos-15"
        ]
      },
      {
        "id": 138790487645,
        "name": "just vv (bdd-preservation-prims)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182593,
        "steps": 13,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T21:33:13Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138790487700,
        "name": "just vv (bdd-language-models)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182600,
        "steps": 13,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T21:41:04Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138790487707,
        "name": "just vv (rest)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182599,
        "steps": 13,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T22:11:30Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138790487738,
        "name": "just vv (bdd-production)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182601,
        "steps": 13,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T22:05:16Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138790487744,
        "name": "just vv (bdd-preservation-runtime)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182603,
        "steps": 13,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T21:36:59Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138790487751,
        "name": "just vv (bdd-verification)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182594,
        "steps": 13,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T21:55:13Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138790487759,
        "name": "just vv (bdd-stdlib)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182606,
        "steps": 13,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T21:43:47Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138790487761,
        "name": "just vv (bdd-preservation-support)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182604,
        "steps": 13,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T21:40:27Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138790487789,
        "name": "just vv (bdd-preservation-std)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182602,
        "steps": 13,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T21:35:21Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138790487813,
        "name": "just vv (examples)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182597,
        "steps": 13,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T22:07:05Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138790487858,
        "name": "just vv (bdd-preservation-lists)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182598,
        "steps": 13,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T21:44:24Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138790487993,
        "name": "just vv (bdd-preservation)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182605,
        "steps": 13,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T22:04:51Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138790488208,
        "name": "just vv (bdd-wasi)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182607,
        "steps": 13,
        "start": "2026-11-24T21:28:23Z",
        "end": "2026-11-24T21:40:36Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138790489366,
        "name": "Intel test construction and local targets",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "skipped",
        "runner": null,
        "steps": 0,
        "start": "2026-11-24T21:28:21Z",
        "end": "2026-11-24T21:28:20Z",
        "labels": [
          "macos-15-intel"
        ]
      },
      {
        "id": 138790491712,
        "name": "Intel conformance (${{ matrix.lane }})",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "skipped",
        "runner": null,
        "steps": 0,
        "start": "2026-11-24T21:28:21Z",
        "end": "2026-11-24T21:28:21Z",
        "labels": [
          "macos-15-intel"
        ]
      },
      {
        "id": 138790493742,
        "name": "build and non-Lean tests (x86_64-apple-darwin)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "skipped",
        "runner": null,
        "steps": 0,
        "start": "2026-11-24T21:28:21Z",
        "end": "2026-11-24T21:28:21Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138808378528,
        "name": "atlas (census-d)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182677,
        "steps": 12,
        "start": "2026-11-24T21:59:26Z",
        "end": "2026-11-24T22:07:59Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138808378535,
        "name": "atlas (census-b)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182675,
        "steps": 12,
        "start": "2026-11-24T21:59:26Z",
        "end": "2026-11-24T22:08:12Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138808378555,
        "name": "atlas (cert4)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182678,
        "steps": 12,
        "start": "2026-11-24T21:59:26Z",
        "end": "2026-11-24T22:02:41Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138808378588,
        "name": "atlas (census-c)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182679,
        "steps": 12,
        "start": "2026-11-24T21:59:26Z",
        "end": "2026-11-24T22:16:46Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138808378622,
        "name": "atlas (census-a)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182676,
        "steps": 12,
        "start": "2026-11-24T21:59:26Z",
        "end": "2026-11-24T22:09:35Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138810269177,
        "name": "atlas (cert4-replay)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182688,
        "steps": 12,
        "start": "2026-11-24T22:02:44Z",
        "end": "2026-11-24T22:06:12Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138814274379,
        "name": "atlas (closure)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182720,
        "steps": 12,
        "start": "2026-11-24T22:09:39Z",
        "end": "2026-11-24T22:15:18Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138817538105,
        "name": "atlas (pairs-b)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182725,
        "steps": 12,
        "start": "2026-11-24T22:15:21Z",
        "end": "2026-11-24T22:27:41Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138817538117,
        "name": "atlas (pairs-d)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182724,
        "steps": 12,
        "start": "2026-11-24T22:15:21Z",
        "end": "2026-11-24T22:27:10Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138817538244,
        "name": "atlas (pairs-a)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182726,
        "steps": 12,
        "start": "2026-11-24T22:15:21Z",
        "end": "2026-11-24T22:28:34Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138817538413,
        "name": "atlas (orbit)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182727,
        "steps": 12,
        "start": "2026-11-24T22:15:21Z",
        "end": "2026-11-24T22:24:22Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138817538706,
        "name": "atlas (pairs-c)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182728,
        "steps": 12,
        "start": "2026-11-24T22:15:21Z",
        "end": "2026-11-24T22:27:11Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821786871,
        "name": "just vv (bdd-bootstrap-gnaf)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182737,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:29:46Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821786872,
        "name": "just vv (bdd-bootstrap-lexicon-lre)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182738,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:29:13Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821786925,
        "name": "just vv (bdd-bootstrap-structural-grammar)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182744,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:39:17Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821786942,
        "name": "just vv (bdd-bootstrap-project-text)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182739,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:28:36Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821786976,
        "name": "just vv (bdd-bootstrap-check)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182736,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:31:38Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821786984,
        "name": "just vv (bdd-bootstrap-tooling-latex)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182745,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:31:39Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821786989,
        "name": "just vv (bdd-bootstrap-lower)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182747,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:46:46Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787000,
        "name": "just vv (bdd-bootstrap-surface-generic)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182742,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:30:12Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787034,
        "name": "just vv (bdd-bootstrap-lexicon-lse)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182758,
        "steps": 13,
        "start": "2026-11-24T22:22:57Z",
        "end": "2026-11-24T22:30:03Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787036,
        "name": "just vv (bdd-bootstrap-assemble)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182740,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:30:06Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787040,
        "name": "just vv (bdd-bootstrap-moves)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182750,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:32:51Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787059,
        "name": "just vv (bdd-bootstrap-surface-lex)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182755,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:32:03Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787068,
        "name": "just vv (bdd-bootstrap-bodies)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182741,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:30:36Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787086,
        "name": "just vv (bdd-bootstrap-validate)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182752,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:37:02Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787094,
        "name": "just vv (bdd-bootstrap-runtime)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182748,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:31:24Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787115,
        "name": "just vv (bdd-bootstrap-print)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182760,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:38:38Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787128,
        "name": "just vv (bdd-bootstrap-succeed)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182764,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:30:05Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787259,
        "name": "just vv (bdd-bootstrap-surface-tooling)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182751,
        "steps": 13,
        "start": "2026-11-24T22:22:57Z",
        "end": "2026-11-24T22:29:50Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787288,
        "name": "just vv (bdd-bootstrap-grammar-proof)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182743,
        "steps": 13,
        "start": "2026-11-24T22:22:55Z",
        "end": "2026-11-24T22:29:23Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787292,
        "name": "just vv (bdd-bootstrap-toml-read)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182774,
        "steps": 13,
        "start": "2026-11-24T22:26:24Z",
        "end": "2026-11-24T22:32:23Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787339,
        "name": "just vv (bdd-bootstrap-terms)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182756,
        "steps": 13,
        "start": "2026-11-24T22:22:57Z",
        "end": "2026-11-24T22:29:31Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787368,
        "name": "just vv (bdd-bootstrap-core-read)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182746,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:35:45Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787369,
        "name": "just vv (bdd-bootstrap-package)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182765,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:34:04Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787380,
        "name": "just vv (bdd-bootstrap-proposition-terms)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182762,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:34:04Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787381,
        "name": "just vv (bdd-bootstrap-loops)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182780,
        "steps": 13,
        "start": "2026-11-24T22:27:43Z",
        "end": "2026-11-24T22:33:56Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787384,
        "name": "just vv (bdd-bootstrap-lock-text)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182777,
        "steps": 13,
        "start": "2026-11-24T22:27:28Z",
        "end": "2026-11-24T22:34:53Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787427,
        "name": "just vv (bdd-bootstrap-package-ref)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182759,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:28:59Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787434,
        "name": "just vv (bdd-bootstrap-lse-types)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182771,
        "steps": 13,
        "start": "2026-11-24T22:24:24Z",
        "end": "2026-11-24T22:30:17Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787435,
        "name": "just vv (bdd-bootstrap-manifest-text)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182761,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:29:36Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787444,
        "name": "just vv (bdd-bootstrap-core-json)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182754,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:45:21Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787448,
        "name": "just vv (bdd-bootstrap-lexicon-schema)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182770,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:29:21Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787465,
        "name": "just vv (bdd-compiler)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182779,
        "steps": 13,
        "start": "2026-11-24T22:27:30Z",
        "end": "2026-11-24T22:47:09Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787469,
        "name": "just vv (bdd-bootstrap-surface-module)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182763,
        "steps": 13,
        "start": "2026-11-24T22:22:57Z",
        "end": "2026-11-24T22:29:49Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787477,
        "name": "just vv (bdd-bootstrap-cli-output)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182749,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:28:46Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787493,
        "name": "just vv (bdd-bootstrap-semantic-read)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182766,
        "steps": 13,
        "start": "2026-11-24T22:22:57Z",
        "end": "2026-11-24T22:56:05Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787500,
        "name": "just vv (bdd-bootstrap-lse-scopes)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182778,
        "steps": 13,
        "start": "2026-11-24T22:27:30Z",
        "end": "2026-11-24T22:34:32Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787514,
        "name": "just vv (bdd-bootstrap-core-print)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182768,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:47:41Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787516,
        "name": "just vv (bdd-bootstrap-grammar-math)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182767,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:30:28Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787526,
        "name": "just vv (bdd-bootstrap-diagnostics)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182753,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:30:08Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787537,
        "name": "just vv (bdd-bootstrap-content-id)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182757,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:28:24Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787557,
        "name": "just vv (bdd-bootstrap-surface-doc)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182781,
        "steps": 13,
        "start": "2026-11-24T22:28:26Z",
        "end": "2026-11-24T22:37:30Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787580,
        "name": "just vv (bdd-bootstrap-semantic-print)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182772,
        "steps": 13,
        "start": "2026-11-24T22:24:38Z",
        "end": "2026-11-24T23:38:39Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138821787653,
        "name": "just vv (bdd-bootstrap-lexicon-entry)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182769,
        "steps": 13,
        "start": "2026-11-24T22:22:56Z",
        "end": "2026-11-24T22:27:26Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138824907516,
        "name": "atlas (join)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182783,
        "steps": 12,
        "start": "2026-11-24T22:28:43Z",
        "end": "2026-11-24T22:30:38Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138859276997,
        "name": "build and non-Lean tests (aarch64-apple-darwin)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182918,
        "steps": 7,
        "start": "2026-11-24T23:35:34Z",
        "end": "2026-11-24T23:36:05Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138865361086,
        "name": "build and non-Lean tests (x86_64-pc-windows-msvc)",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182920,
        "steps": 7,
        "start": "2026-11-24T23:48:37Z",
        "end": "2026-11-24T23:49:17Z",
        "labels": [
          "ubuntu-24.04"
        ]
      }
    ]
  },
  "post": {
    "id": 45520783138,
    "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
    "attempt": 1,
    "status": "completed",
    "conclusion": "cancelled",
    "created": "2026-11-25T00:47:11Z",
    "updated": "2026-11-25T03:02:11Z",
    "total": 24,
    "jobs": [
      {
        "id": 138891776627,
        "name": "plan",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000183050,
        "steps": 6,
        "start": "2026-11-25T00:47:14Z",
        "end": "2026-11-25T00:47:57Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138892105855,
        "name": "honesty",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000183055,
        "steps": 11,
        "start": "2026-11-25T00:47:59Z",
        "end": "2026-11-25T00:51:21Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138893594271,
        "name": "Intel test construction and local targets",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000183076,
        "steps": 7,
        "start": "2026-11-25T00:51:24Z",
        "end": "2026-11-25T01:07:37Z",
        "labels": [
          "macos-15-intel"
        ]
      },
      {
        "id": 138893596104,
        "name": "build and non-Lean tests (${{ matrix.target }})",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "skipped",
        "runner": null,
        "steps": 0,
        "start": "2026-11-25T00:51:21Z",
        "end": "2026-11-25T00:51:21Z",
        "labels": []
      },
      {
        "id": 138893596989,
        "name": "atlas (cert4)",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "skipped",
        "runner": null,
        "steps": 0,
        "start": "2026-11-25T00:51:21Z",
        "end": "2026-11-25T00:51:21Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138893597084,
        "name": "Windows tests (${{ matrix.lane }})",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "skipped",
        "runner": null,
        "steps": 0,
        "start": "2026-11-25T00:51:21Z",
        "end": "2026-11-25T00:51:21Z",
        "labels": [
          "windows-2022"
        ]
      },
      {
        "id": 138893597176,
        "name": "bootstrap export",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "skipped",
        "runner": null,
        "steps": 0,
        "start": "2026-11-25T00:51:21Z",
        "end": "2026-11-25T00:51:21Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138893597202,
        "name": "macOS arm64 tests (${{ matrix.lane }})",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "skipped",
        "runner": null,
        "steps": 0,
        "start": "2026-11-25T00:51:21Z",
        "end": "2026-11-25T00:51:21Z",
        "labels": [
          "macos-15"
        ]
      },
      {
        "id": 138893597274,
        "name": "build and non-Lean tests (x86_64-pc-windows-msvc)",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "skipped",
        "runner": null,
        "steps": 0,
        "start": "2026-11-25T00:51:21Z",
        "end": "2026-11-25T00:51:21Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138893597316,
        "name": "atlas (prefix)",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "skipped",
        "runner": null,
        "steps": 0,
        "start": "2026-11-25T00:51:21Z",
        "end": "2026-11-25T00:51:21Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138893598085,
        "name": "atlas (${{ matrix.shard }})",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "skipped",
        "runner": null,
        "steps": 0,
        "start": "2026-11-25T00:51:21Z",
        "end": "2026-11-25T00:51:21Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138893598131,
        "name": "atlas (${{ matrix.shard }})",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "skipped",
        "runner": null,
        "steps": 0,
        "start": "2026-11-25T00:51:21Z",
        "end": "2026-11-25T00:51:21Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138893598401,
        "name": "atlas (closure)",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "skipped",
        "runner": null,
        "steps": 0,
        "start": "2026-11-25T00:51:21Z",
        "end": "2026-11-25T00:51:21Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138893598808,
        "name": "just vv (${{ matrix.part }})",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "skipped",
        "runner": null,
        "steps": 0,
        "start": "2026-11-25T00:51:22Z",
        "end": "2026-11-25T00:51:21Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138893598871,
        "name": "build and non-Lean tests (aarch64-apple-darwin)",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "skipped",
        "runner": null,
        "steps": 0,
        "start": "2026-11-25T00:51:22Z",
        "end": "2026-11-25T00:51:21Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138893599169,
        "name": "atlas (cert4-replay)",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "skipped",
        "runner": null,
        "steps": 0,
        "start": "2026-11-25T00:51:22Z",
        "end": "2026-11-25T00:51:21Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138893599287,
        "name": "atlas (${{ matrix.shard }})",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "skipped",
        "runner": null,
        "steps": 0,
        "start": "2026-11-25T00:51:22Z",
        "end": "2026-11-25T00:51:21Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138893600440,
        "name": "atlas (join)",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "skipped",
        "runner": null,
        "steps": 0,
        "start": "2026-11-25T00:51:22Z",
        "end": "2026-11-25T00:51:21Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138893600446,
        "name": "just vv (${{ matrix.part }})",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "skipped",
        "runner": null,
        "steps": 0,
        "start": "2026-11-25T00:51:22Z",
        "end": "2026-11-25T00:51:21Z",
        "labels": [
          "ubuntu-24.04"
        ]
      },
      {
        "id": 138900594197,
        "name": "Intel conformance (moves)",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000183137,
        "steps": 8,
        "start": "2026-11-25T01:07:43Z",
        "end": "2026-11-25T02:39:57Z",
        "labels": [
          "macos-15-intel"
        ]
      },
      {
        "id": 138900594220,
        "name": "Intel conformance (validate)",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000183136,
        "steps": 8,
        "start": "2026-11-25T01:07:41Z",
        "end": "2026-11-25T02:37:00Z",
        "labels": [
          "macos-15-intel"
        ]
      },
      {
        "id": 138900594300,
        "name": "Intel conformance (lower)",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "cancelled",
        "runner": 1000183138,
        "steps": 8,
        "start": "2026-11-25T01:07:40Z",
        "end": "2026-11-25T03:01:27Z",
        "labels": [
          "macos-15-intel"
        ]
      },
      {
        "id": 138900594352,
        "name": "Intel conformance (print)",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000183146,
        "steps": 8,
        "start": "2026-11-25T01:20:58Z",
        "end": "2026-11-25T02:07:35Z",
        "labels": [
          "macos-15-intel"
        ]
      },
      {
        "id": 138946044173,
        "name": "build and non-Lean tests (x86_64-apple-darwin)",
        "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
        "attempt": 1,
        "status": "completed",
        "conclusion": "failure",
        "runner": 1000183237,
        "steps": 8,
        "start": "2026-11-25T03:01:29Z",
        "end": "2026-11-25T03:02:10Z",
        "labels": [
          "ubuntu-24.04"
        ]
      }
    ]
  },
  "repro": {
    "id": 45490646770,
    "head": "a5e846de239113ca591f949eb75764d40096f7e9",
    "attempt": 1,
    "status": "completed",
    "conclusion": "success",
    "created": "2026-11-24T21:23:42Z",
    "updated": "2026-11-24T21:29:20Z",
    "total": 1,
    "jobs": [
      {
        "id": 138787742569,
        "name": "repro",
        "head": "a5e846de239113ca591f949eb75764d40096f7e9",
        "attempt": 1,
        "status": "completed",
        "conclusion": "success",
        "runner": 1000182578,
        "steps": 7,
        "start": "2026-11-24T21:23:45Z",
        "end": "2026-11-24T21:29:19Z",
        "labels": [
          "ubuntu-24.04"
        ]
      }
    ]
  },
  "custody": {
    "blob": "dfe1e64eaf2d004082212793047426ac16234db1",
    "strictAttempt": true,
    "artifactNames": [
      "intel-construction",
      "intel-lower",
      "acceptance-gate-evidence-intel"
    ],
    "constructionExpires": "2026-12-09T01:07:26Z"
  }
};
function evaluate(s){
 const fail=m=>{throw Error(m)}, req=(v,m)=>{if(!v)fail(m)};
 const rates={"ubuntu-24.04":.006,"ubuntu-24.04-arm":.005,"windows-2022":.01,"macos-15":.062,"macos-15-intel":.062};
 const expectedIds={pre:[45490646681,"a5e846de239113ca591f949eb75764d40096f7e9",89,"success"],post:[45520783138,"fa2280e226649e7d8fccacbad981f12faf88b602",24,"cancelled"],repro:[45490646770,"a5e846de239113ca591f949eb75764d40096f7e9",1,"success"]};
 const out={};for(const [key,[id,head,n,conclusion]] of Object.entries(expectedIds)){
  const r=s[key];req(r&&r.id===id&&r.head===head&&r.attempt===1&&r.status==="completed"&&r.conclusion===conclusion,"foreign/nonterminal run "+key);
  req(r.total===n&&r.jobs.length===n&&new Set(r.jobs.map(j=>j.id)).size===n,"incomplete/duplicate inventory "+key);
  const counts={},byHost={};let seconds=0,cost=0,roundedCost=0,retainedSeconds=0;
  for(const j of r.jobs){req(j.head===head&&j.attempt===1&&j.status==="completed","foreign/nonterminal job");counts[j.conclusion]=(counts[j.conclusion]||0)+1;
   if(j.conclusion==="skipped"){req(!j.runner&&!j.steps,"allocated skip");continue}
   req(j.runner>0&&j.steps>0,"missing allocation");let d=(Date.parse(j.end)-Date.parse(j.start))/1000;req(Number.isFinite(d)&&d>=0,"invalid interval");const hosts=j.labels.filter(l=>Object.hasOwn(rates,l));req(hosts.length===1,"unclassified runner "+j.name);const h=hosts[0];seconds+=d;byHost[h]=(byHost[h]||0)+d;cost+=d/60*rates[h];roundedCost+=Math.ceil(d/60)*rates[h];if(j.conclusion==="success")retainedSeconds+=d;
  }
  const expect=key==="pre"?{success:86,skipped:3}:key==="post"?{success:6,skipped:16,cancelled:1,failure:1}:{success:1};req(Object.keys(counts).length===Object.keys(expect).length&&Object.entries(expect).every(([k,v])=>counts[k]===v),"unexpected conclusion inventory");
  out[key]={counts,seconds,byHost,modeledCost:cost,roundedModeledCost:roundedCost,retainedSeconds,elapsedSeconds:(Date.parse(r.updated)-Date.parse(r.created))/1000};
 }
 const post=s.post.jobs;const bad=post.filter(j=>["cancelled","failure"].includes(j.conclusion));req(bad.length===2&&bad.some(j=>j.id===138900594300&&j.name==="Intel conformance (lower)"&&j.conclusion==="cancelled")&&bad.some(j=>j.id===138946044173&&j.name==="build and non-Lean tests (x86_64-apple-darwin)"&&j.conclusion==="failure"),"unexpected recovery roots");
 req(s.custody.blob==="dfe1e64eaf2d004082212793047426ac16234db1"&&s.custody.strictAttempt===true,"missing source custody");
 const closure=post.filter(j=>j.name==="Intel test construction and local targets"||/^Intel conformance \((moves|validate|lower|print)\)$/.test(j.name)||j.name==="build and non-Lean tests (x86_64-apple-darwin)");req(closure.length===6&&new Set(closure.map(j=>j.name)).size===6,"incomplete construction closure");
 out.recovery={disposition:"HOLD_LOWER_ONLY_STRICT_ATTEMPT",reason:"construction binds GITHUB_RUN_ATTEMPT; attempt-2 lane cannot consume attempt-1 construction; rerunning construction requires all dependent lanes and join, subject to artifact collision and provider dependency semantics review",candidateClosure:closure.map(j=>j.name),priorClosureSeconds:closure.reduce((a,j)=>a+(Date.parse(j.end)-Date.parse(j.start))/1000,0),retainedOutsideClosureSeconds:out.post.retainedSeconds-closure.filter(j=>j.conclusion==="success").reduce((a,j)=>a+(Date.parse(j.end)-Date.parse(j.start))/1000,0),pilotComplete:false};
 return out;
}
const mutants=[s=>s.pre.jobs.pop(),
s=>s.post.jobs[0].head="bad",
s=>s.post.jobs[1].id=s.post.jobs[0].id,
s=>s.post.attempt=2,
s=>s.post.jobs.find(j=>j.conclusion==="cancelled").conclusion="success",
s=>s.custody.strictAttempt=false,
s=>s.repro.jobs[0].labels=["unknown"],
s=>s.pre.jobs.find(j=>j.conclusion==="skipped").runner=1,
s=>s.post.jobs.find(j=>j.conclusion==="failure").name="wrong join",
s=>s.post.jobs.find(j=>j.conclusion==="success").end="invalid"];
for(const mutate of mutants){const s=structuredClone(snapshot);mutate(s);let rejected=false;try{evaluate(s)}catch{rejected=true}if(!rejected)throw Error('mutant survived')}
console.log(JSON.stringify(evaluate(process.argv[2]?JSON.parse(readFileSync(process.argv[2],'utf8')):snapshot),null,2));

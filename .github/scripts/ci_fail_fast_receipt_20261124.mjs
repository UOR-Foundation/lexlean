// DOD-6 observed failed-run comparison; advisory only, no qualification authority.
// Replay: node .github/scripts/ci_fail_fast_receipt_20261124.mjs
// Provider job IDs are unique; unexpanded skipped matrix placeholders may legitimately share names.
// Different revisions and defects; not causal, not billed usage. Independent repro counted separately. Failed-job completion is not first actionable log time.
const receipts=[
  {
    "id": 45256672496,
    "head": "53f359a95a8f23091090771bf42138044372d76f",
    "attempt": 1,
    "status": "completed",
    "conclusion": "failure",
    "created": "2026-11-23T20:10:57Z",
    "updated": "2026-11-23T23:30:43Z",
    "total": 70,
    "pages": [
      {
        "page": 1,
        "total": 70,
        "jobs": [
          {
            "id": 137980194340,
            "name": "plan",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:11:01Z",
            "completed": "2026-11-23T20:15:24Z",
            "runner": "GitHub Actions 1000179316",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 5
          },
          {
            "id": 137982952372,
            "name": "honesty",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:15:27Z",
            "completed": "2026-11-23T20:19:36Z",
            "runner": "GitHub Actions 1000179321",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985538989,
            "name": "atlas (prefix)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T20:47:01Z",
            "runner": "GitHub Actions 1000179331",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985539016,
            "name": "Intel test construction and local targets",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:40Z",
            "completed": "2026-11-23T20:40:17Z",
            "runner": "GitHub Actions 1000179345",
            "labels": [
              "macos-15-intel"
            ],
            "steps": 7
          },
          {
            "id": 137985539062,
            "name": "bootstrap export",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:39Z",
            "completed": "2026-11-23T20:58:33Z",
            "runner": "GitHub Actions 1000179332",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 11
          },
          {
            "id": 137985539100,
            "name": "build and non-Lean tests (aarch64-apple-darwin)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "failure",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T23:18:11Z",
            "runner": "GitHub Actions 1000179339",
            "labels": [
              "macos-15"
            ],
            "steps": 7
          },
          {
            "id": 137985539114,
            "name": "Windows tests (moves)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "failure",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T20:53:39Z",
            "runner": "GitHub Actions 1000179340",
            "labels": [
              "windows-2022"
            ],
            "steps": 8
          },
          {
            "id": 137985539149,
            "name": "Windows tests (print)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T21:40:55Z",
            "runner": "GitHub Actions 1000179344",
            "labels": [
              "windows-2022"
            ],
            "steps": 8
          },
          {
            "id": 137985539173,
            "name": "Windows tests (lower)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T21:34:22Z",
            "runner": "GitHub Actions 1000179341",
            "labels": [
              "windows-2022"
            ],
            "steps": 8
          },
          {
            "id": 137985539180,
            "name": "Windows tests (validate)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T20:55:34Z",
            "runner": "GitHub Actions 1000179343",
            "labels": [
              "windows-2022"
            ],
            "steps": 8
          },
          {
            "id": 137985539220,
            "name": "Windows tests (producer)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T20:35:21Z",
            "runner": "GitHub Actions 1000179342",
            "labels": [
              "windows-2022"
            ],
            "steps": 8
          },
          {
            "id": 137985539683,
            "name": "build and non-Lean tests (aarch64-unknown-linux-gnu)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "failure",
            "started": "2026-11-23T20:19:41Z",
            "completed": "2026-11-23T23:30:43Z",
            "runner": "GitHub Actions 1000179379",
            "labels": [
              "ubuntu-24.04-arm"
            ],
            "steps": 7
          },
          {
            "id": 137985539956,
            "name": "just vv (bdd-bootstrap-gnaf)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:39Z",
            "completed": "2026-11-23T20:59:41Z",
            "runner": "GitHub Actions 1000179337",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985539981,
            "name": "just vv (bdd-bootstrap-lexicon-lse)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T20:59:39Z",
            "runner": "GitHub Actions 1000179370",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985539989,
            "name": "just vv (bdd-bootstrap-lexicon-lre)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T21:00:14Z",
            "runner": "GitHub Actions 1000179347",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985539998,
            "name": "just vv (bdd-bootstrap-lower)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T21:24:26Z",
            "runner": "GitHub Actions 1000179333",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540030,
            "name": "just vv (bdd-bootstrap-bodies)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T21:00:08Z",
            "runner": "GitHub Actions 1000179336",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540035,
            "name": "just vv (bdd-bootstrap-runtime)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:39Z",
            "completed": "2026-11-23T20:59:48Z",
            "runner": "GitHub Actions 1000179335",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540040,
            "name": "just vv (bdd-bootstrap-check)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:39Z",
            "completed": "2026-11-23T21:10:01Z",
            "runner": "GitHub Actions 1000179352",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540053,
            "name": "just vv (examples)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T21:50:33Z",
            "runner": "GitHub Actions 1000179369",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540059,
            "name": "just vv (bdd-bootstrap-print)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T20:59:53Z",
            "runner": "GitHub Actions 1000179334",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540084,
            "name": "just vv (bdd-bootstrap-loops)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:39Z",
            "completed": "2026-11-23T20:59:43Z",
            "runner": "GitHub Actions 1000179350",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540097,
            "name": "just vv (bdd-bootstrap-package)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T20:59:43Z",
            "runner": "GitHub Actions 1000179351",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540137,
            "name": "just vv (rest)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "failure",
            "started": "2026-11-23T20:19:39Z",
            "completed": "2026-11-23T21:22:05Z",
            "runner": "GitHub Actions 1000179349",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540155,
            "name": "just vv (bdd-bootstrap-tooling-latex)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:39Z",
            "completed": "2026-11-23T20:59:47Z",
            "runner": "GitHub Actions 1000179363",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540161,
            "name": "just vv (bdd-bootstrap-validate)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T20:59:33Z",
            "runner": "GitHub Actions 1000179353",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540271,
            "name": "just vv (bdd-bootstrap-structural-grammar)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T20:59:54Z",
            "runner": "GitHub Actions 1000179346",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540276,
            "name": "just vv (bdd-bootstrap-terms)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:39Z",
            "completed": "2026-11-23T21:00:16Z",
            "runner": "GitHub Actions 1000179364",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540297,
            "name": "just vv (bdd-bootstrap-surface-generic)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T20:59:50Z",
            "runner": "GitHub Actions 1000179365",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540334,
            "name": "just vv (bdd-bootstrap-succeed)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T20:59:59Z",
            "runner": "GitHub Actions 1000179358",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540344,
            "name": "just vv (bdd-bootstrap-project-text)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T20:59:19Z",
            "runner": "GitHub Actions 1000179360",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540356,
            "name": "just vv (bdd-bootstrap-assemble)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T20:59:59Z",
            "runner": "GitHub Actions 1000179348",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540379,
            "name": "just vv (bdd-preservation-runtime)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:39Z",
            "completed": "2026-11-23T20:28:35Z",
            "runner": "GitHub Actions 1000179338",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540383,
            "name": "just vv (bdd-bootstrap-surface-lex)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:39Z",
            "completed": "2026-11-23T20:59:15Z",
            "runner": "GitHub Actions 1000179362",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540409,
            "name": "just vv (bdd-compiler)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:39Z",
            "completed": "2026-11-23T21:05:50Z",
            "runner": "GitHub Actions 1000179354",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540443,
            "name": "just vv (bdd-preservation)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:39Z",
            "completed": "2026-11-23T21:01:33Z",
            "runner": "GitHub Actions 1000179375",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540446,
            "name": "just vv (bdd-preservation-lists)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:39Z",
            "completed": "2026-11-23T20:33:16Z",
            "runner": "GitHub Actions 1000179366",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540449,
            "name": "just vv (bdd-preservation-prims)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T20:25:48Z",
            "runner": "GitHub Actions 1000179359",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540453,
            "name": "just vv (bdd-bootstrap-lock-text)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T20:59:29Z",
            "runner": "GitHub Actions 1000179372",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540455,
            "name": "just vv (bdd-preservation-support)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T20:32:02Z",
            "runner": "GitHub Actions 1000179356",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540476,
            "name": "just vv (bdd-bootstrap-surface-module)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:39Z",
            "completed": "2026-11-23T20:59:41Z",
            "runner": "GitHub Actions 1000179355",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540494,
            "name": "just vv (bdd-verification)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T20:48:54Z",
            "runner": "GitHub Actions 1000179361",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540508,
            "name": "just vv (bdd-bootstrap-surface-tooling)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:39Z",
            "completed": "2026-11-23T20:59:46Z",
            "runner": "GitHub Actions 1000179376",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540535,
            "name": "just vv (bdd-bootstrap-content-id)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:39Z",
            "completed": "2026-11-23T20:59:53Z",
            "runner": "GitHub Actions 1000179367",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540579,
            "name": "just vv (bdd-bootstrap-surface-doc)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:39Z",
            "completed": "2026-11-23T20:59:40Z",
            "runner": "GitHub Actions 1000179368",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540604,
            "name": "just vv (bdd-bootstrap-diagnostics)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:39Z",
            "completed": "2026-11-23T20:59:42Z",
            "runner": "GitHub Actions 1000179371",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540646,
            "name": "just vv (bdd-bootstrap-cli-output)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T20:59:57Z",
            "runner": "GitHub Actions 1000179373",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540650,
            "name": "just vv (bdd-production)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:39Z",
            "completed": "2026-11-23T20:54:54Z",
            "runner": "GitHub Actions 1000179357",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540688,
            "name": "just vv (bdd-bootstrap-moves)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:39Z",
            "completed": "2026-11-23T20:59:49Z",
            "runner": "GitHub Actions 1000179377",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137985540692,
            "name": "just vv (bdd-bootstrap-manifest-text)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:38Z",
            "completed": "2026-11-23T20:59:38Z",
            "runner": "GitHub Actions 1000179378",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          }
        ]
      },
      {
        "page": 2,
        "total": 70,
        "jobs": [
          {
            "id": 137985540796,
            "name": "just vv (bdd-preservation-std)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:19:39Z",
            "completed": "2026-11-23T20:27:41Z",
            "runner": "GitHub Actions 1000179374",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 13
          },
          {
            "id": 137998099981,
            "name": "Intel conformance (moves)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "failure",
            "started": "2026-11-23T20:40:23Z",
            "completed": "2026-11-23T21:57:17Z",
            "runner": "GitHub Actions 1000179402",
            "labels": [
              "macos-15-intel"
            ],
            "steps": 8
          },
          {
            "id": 137998100050,
            "name": "Intel conformance (print)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:40:19Z",
            "completed": "2026-11-23T22:07:11Z",
            "runner": "GitHub Actions 1000179403",
            "labels": [
              "macos-15-intel"
            ],
            "steps": 8
          },
          {
            "id": 137998100053,
            "name": "Intel conformance (validate)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:40:19Z",
            "completed": "2026-11-23T21:27:06Z",
            "runner": "GitHub Actions 1000179401",
            "labels": [
              "macos-15-intel"
            ],
            "steps": 8
          },
          {
            "id": 137998100450,
            "name": "Intel conformance (lower)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:40:20Z",
            "completed": "2026-11-23T22:03:53Z",
            "runner": "GitHub Actions 1000179404",
            "labels": [
              "macos-15-intel"
            ],
            "steps": 8
          },
          {
            "id": 138002125686,
            "name": "atlas (cert4)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:47:03Z",
            "completed": "2026-11-23T20:52:11Z",
            "runner": "GitHub Actions 1000179425",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 12
          },
          {
            "id": 138002125773,
            "name": "atlas (census-d)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:47:04Z",
            "completed": "2026-11-23T20:56:28Z",
            "runner": "GitHub Actions 1000179424",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 12
          },
          {
            "id": 138002125809,
            "name": "atlas (census-c)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:47:03Z",
            "completed": "2026-11-23T21:06:37Z",
            "runner": "GitHub Actions 1000179426",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 12
          },
          {
            "id": 138002125857,
            "name": "atlas (census-a)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:47:04Z",
            "completed": "2026-11-23T20:58:41Z",
            "runner": "GitHub Actions 1000179423",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 12
          },
          {
            "id": 138002125953,
            "name": "atlas (census-b)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:47:03Z",
            "completed": "2026-11-23T20:56:42Z",
            "runner": "GitHub Actions 1000179427",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 12
          },
          {
            "id": 138005198753,
            "name": "atlas (cert4-replay)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:52:14Z",
            "completed": "2026-11-23T20:56:07Z",
            "runner": "GitHub Actions 1000179437",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 12
          },
          {
            "id": 138009033029,
            "name": "atlas (closure)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:58:43Z",
            "completed": "2026-11-23T21:05:55Z",
            "runner": "GitHub Actions 1000179443",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 12
          },
          {
            "id": 138013418783,
            "name": "atlas (pairs-b)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T21:05:57Z",
            "completed": "2026-11-23T21:18:55Z",
            "runner": "GitHub Actions 1000179458",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 12
          },
          {
            "id": 138013418865,
            "name": "atlas (pairs-c)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T21:05:57Z",
            "completed": "2026-11-23T21:20:05Z",
            "runner": "GitHub Actions 1000179460",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 12
          },
          {
            "id": 138013418914,
            "name": "atlas (pairs-a)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T21:05:57Z",
            "completed": "2026-11-23T21:19:07Z",
            "runner": "GitHub Actions 1000179461",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 12
          },
          {
            "id": 138013418971,
            "name": "atlas (orbit)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T21:05:58Z",
            "completed": "2026-11-23T21:16:36Z",
            "runner": "GitHub Actions 1000179459",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 12
          },
          {
            "id": 138013418979,
            "name": "atlas (pairs-d)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T21:05:57Z",
            "completed": "2026-11-23T21:18:25Z",
            "runner": "GitHub Actions 1000179457",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 12
          },
          {
            "id": 138022001973,
            "name": "atlas (join)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T21:20:07Z",
            "completed": "2026-11-23T21:24:28Z",
            "runner": "GitHub Actions 1000179483",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 12
          },
          {
            "id": 138034386725,
            "name": "build and non-Lean tests (x86_64-pc-windows-msvc)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "failure",
            "started": "2026-11-23T21:40:57Z",
            "completed": "2026-11-23T21:42:14Z",
            "runner": "GitHub Actions 1000179506",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 7
          },
          {
            "id": 138049599907,
            "name": "build and non-Lean tests (x86_64-apple-darwin)",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "failure",
            "started": "2026-11-23T22:07:14Z",
            "completed": "2026-11-23T22:08:22Z",
            "runner": "GitHub Actions 1000179526",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 8
          }
        ]
      }
    ]
  },
  {
    "id": 45256672489,
    "head": "53f359a95a8f23091090771bf42138044372d76f",
    "attempt": 1,
    "status": "completed",
    "conclusion": "success",
    "created": "2026-11-23T20:10:57Z",
    "updated": "2026-11-23T20:17:38Z",
    "total": 1,
    "pages": [
      {
        "page": 1,
        "total": 1,
        "jobs": [
          {
            "id": 137980193724,
            "name": "repro",
            "head": "53f359a95a8f23091090771bf42138044372d76f",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-23T20:11:01Z",
            "completed": "2026-11-23T20:17:37Z",
            "runner": "GitHub Actions 1000179315",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 7
          }
        ]
      }
    ]
  },
  {
    "id": 45487283362,
    "head": "51ff76c7e3eddb2b62dc2bdbdaae5bda0c3a9f09",
    "attempt": 1,
    "status": "completed",
    "conclusion": "failure",
    "created": "2026-11-24T21:04:14Z",
    "updated": "2026-11-24T21:10:32Z",
    "total": 18,
    "pages": [
      {
        "page": 1,
        "total": 18,
        "jobs": [
          {
            "id": 138775926774,
            "name": "plan",
            "head": "51ff76c7e3eddb2b62dc2bdbdaae5bda0c3a9f09",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-24T21:04:17Z",
            "completed": "2026-11-24T21:05:28Z",
            "runner": "GitHub Actions 1000182551",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 5
          },
          {
            "id": 138776669640,
            "name": "honesty",
            "head": "51ff76c7e3eddb2b62dc2bdbdaae5bda0c3a9f09",
            "attempt": 1,
            "status": "completed",
            "conclusion": "failure",
            "started": "2026-11-24T21:05:31Z",
            "completed": "2026-11-24T21:09:06Z",
            "runner": "GitHub Actions 1000182553",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 16
          },
          {
            "id": 138778908781,
            "name": "build and non-Lean tests (x86_64-pc-windows-msvc)",
            "head": "51ff76c7e3eddb2b62dc2bdbdaae5bda0c3a9f09",
            "attempt": 1,
            "status": "completed",
            "conclusion": "failure",
            "started": "2026-11-24T21:09:08Z",
            "completed": "2026-11-24T21:10:32Z",
            "runner": "GitHub Actions 1000182558",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 7
          },
          {
            "id": 138778909361,
            "name": "build and non-Lean tests (x86_64-apple-darwin)",
            "head": "51ff76c7e3eddb2b62dc2bdbdaae5bda0c3a9f09",
            "attempt": 1,
            "status": "completed",
            "conclusion": "failure",
            "started": "2026-11-24T21:09:09Z",
            "completed": "2026-11-24T21:10:22Z",
            "runner": "GitHub Actions 1000182557",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 8
          },
          {
            "id": 138778910007,
            "name": "Windows tests (${{ matrix.lane }})",
            "head": "51ff76c7e3eddb2b62dc2bdbdaae5bda0c3a9f09",
            "attempt": 1,
            "status": "completed",
            "conclusion": "skipped",
            "started": "2026-11-24T21:09:06Z",
            "completed": "2026-11-24T21:09:06Z",
            "runner": null,
            "labels": [
              "windows-2022"
            ],
            "steps": 0
          },
          {
            "id": 138778910294,
            "name": "just vv (${{ matrix.part }})",
            "head": "51ff76c7e3eddb2b62dc2bdbdaae5bda0c3a9f09",
            "attempt": 1,
            "status": "completed",
            "conclusion": "skipped",
            "started": "2026-11-24T21:09:06Z",
            "completed": "2026-11-24T21:09:06Z",
            "runner": null,
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 0
          },
          {
            "id": 138778910737,
            "name": "build and non-Lean tests (${{ matrix.target }})",
            "head": "51ff76c7e3eddb2b62dc2bdbdaae5bda0c3a9f09",
            "attempt": 1,
            "status": "completed",
            "conclusion": "skipped",
            "started": "2026-11-24T21:09:06Z",
            "completed": "2026-11-24T21:09:06Z",
            "runner": null,
            "labels": [],
            "steps": 0
          },
          {
            "id": 138778910781,
            "name": "atlas (prefix)",
            "head": "51ff76c7e3eddb2b62dc2bdbdaae5bda0c3a9f09",
            "attempt": 1,
            "status": "completed",
            "conclusion": "skipped",
            "started": "2026-11-24T21:09:06Z",
            "completed": "2026-11-24T21:09:06Z",
            "runner": null,
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 0
          },
          {
            "id": 138778910791,
            "name": "Intel test construction and local targets",
            "head": "51ff76c7e3eddb2b62dc2bdbdaae5bda0c3a9f09",
            "attempt": 1,
            "status": "completed",
            "conclusion": "skipped",
            "started": "2026-11-24T21:09:06Z",
            "completed": "2026-11-24T21:09:06Z",
            "runner": null,
            "labels": [
              "macos-15-intel"
            ],
            "steps": 0
          },
          {
            "id": 138778911128,
            "name": "bootstrap export",
            "head": "51ff76c7e3eddb2b62dc2bdbdaae5bda0c3a9f09",
            "attempt": 1,
            "status": "completed",
            "conclusion": "skipped",
            "started": "2026-11-24T21:09:06Z",
            "completed": "2026-11-24T21:09:06Z",
            "runner": null,
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 0
          },
          {
            "id": 138778911880,
            "name": "atlas (${{ matrix.shard }})",
            "head": "51ff76c7e3eddb2b62dc2bdbdaae5bda0c3a9f09",
            "attempt": 1,
            "status": "completed",
            "conclusion": "skipped",
            "started": "2026-11-24T21:09:06Z",
            "completed": "2026-11-24T21:09:06Z",
            "runner": null,
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 0
          },
          {
            "id": 138778912040,
            "name": "atlas (${{ matrix.shard }})",
            "head": "51ff76c7e3eddb2b62dc2bdbdaae5bda0c3a9f09",
            "attempt": 1,
            "status": "completed",
            "conclusion": "skipped",
            "started": "2026-11-24T21:09:06Z",
            "completed": "2026-11-24T21:09:06Z",
            "runner": null,
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 0
          },
          {
            "id": 138778912570,
            "name": "atlas (closure)",
            "head": "51ff76c7e3eddb2b62dc2bdbdaae5bda0c3a9f09",
            "attempt": 1,
            "status": "completed",
            "conclusion": "skipped",
            "started": "2026-11-24T21:09:06Z",
            "completed": "2026-11-24T21:09:06Z",
            "runner": null,
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 0
          },
          {
            "id": 138778912760,
            "name": "Intel conformance (${{ matrix.lane }})",
            "head": "51ff76c7e3eddb2b62dc2bdbdaae5bda0c3a9f09",
            "attempt": 1,
            "status": "completed",
            "conclusion": "skipped",
            "started": "2026-11-24T21:09:06Z",
            "completed": "2026-11-24T21:09:06Z",
            "runner": null,
            "labels": [
              "macos-15-intel"
            ],
            "steps": 0
          },
          {
            "id": 138778912821,
            "name": "atlas (cert4)",
            "head": "51ff76c7e3eddb2b62dc2bdbdaae5bda0c3a9f09",
            "attempt": 1,
            "status": "completed",
            "conclusion": "skipped",
            "started": "2026-11-24T21:09:06Z",
            "completed": "2026-11-24T21:09:06Z",
            "runner": null,
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 0
          },
          {
            "id": 138778914185,
            "name": "atlas (${{ matrix.shard }})",
            "head": "51ff76c7e3eddb2b62dc2bdbdaae5bda0c3a9f09",
            "attempt": 1,
            "status": "completed",
            "conclusion": "skipped",
            "started": "2026-11-24T21:09:06Z",
            "completed": "2026-11-24T21:09:06Z",
            "runner": null,
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 0
          },
          {
            "id": 138778914257,
            "name": "atlas (cert4-replay)",
            "head": "51ff76c7e3eddb2b62dc2bdbdaae5bda0c3a9f09",
            "attempt": 1,
            "status": "completed",
            "conclusion": "skipped",
            "started": "2026-11-24T21:09:06Z",
            "completed": "2026-11-24T21:09:06Z",
            "runner": null,
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 0
          },
          {
            "id": 138778915118,
            "name": "atlas (join)",
            "head": "51ff76c7e3eddb2b62dc2bdbdaae5bda0c3a9f09",
            "attempt": 1,
            "status": "completed",
            "conclusion": "skipped",
            "started": "2026-11-24T21:09:06Z",
            "completed": "2026-11-24T21:09:06Z",
            "runner": null,
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 0
          }
        ]
      }
    ]
  },
  {
    "id": 45487283519,
    "head": "51ff76c7e3eddb2b62dc2bdbdaae5bda0c3a9f09",
    "attempt": 1,
    "status": "completed",
    "conclusion": "success",
    "created": "2026-11-24T21:04:14Z",
    "updated": "2026-11-24T21:11:07Z",
    "total": 1,
    "pages": [
      {
        "page": 1,
        "total": 1,
        "jobs": [
          {
            "id": 138775927519,
            "name": "repro",
            "head": "51ff76c7e3eddb2b62dc2bdbdaae5bda0c3a9f09",
            "attempt": 1,
            "status": "completed",
            "conclusion": "success",
            "started": "2026-11-24T21:04:17Z",
            "completed": "2026-11-24T21:11:06Z",
            "runner": "GitHub Actions 1000182552",
            "labels": [
              "ubuntu-24.04"
            ],
            "steps": 7
          }
        ]
      }
    ]
  }
];
function calc(r){
 const fail=m=>{throw Error(m)},dt=v=>{let n=Date.parse(v);if(!Number.isFinite(n))fail("timestamp");return n};
 if(r.status!=="completed"||r.attempt!==1||!["success","failure"].includes(r.conclusion)||!Array.isArray(r.pages)||r.pages.length!==Math.ceil(r.total/50))fail("run identity");
 let jobs=[];for(let i=0;i<r.pages.length;i++){let p=r.pages[i];if(p.page!==i+1||p.total!==r.total||p.jobs.length!==Math.min(50,r.total-i*50))fail("page inventory");jobs.push(...p.jobs)}
 if(new Set(jobs.map(x=>x.id)).size!==r.total)fail("duplicate job id");
 const ran=jobs.filter(j=>j.runner);if(new Set(ran.map(j=>j.name)).size!==ran.length)fail("duplicate allocated name");
 let seconds=0,allocated=0,skipped=0,failed=[],byHost={};for(const j of jobs){if(j.head!==r.head||j.attempt!==r.attempt||j.status!=="completed")fail("job custody");if(!j.runner){if(j.conclusion!=="skipped"||j.steps!==0)fail("unallocated non-skip");skipped++;continue}
 if(!Array.isArray(j.labels)||j.labels.length!==1||!["success","failure"].includes(j.conclusion)||j.steps<1)fail("allocated job");
 let start=dt(j.started),end=dt(j.completed),s=(end-start)/1000;if(s<0||start<dt(r.created)||end>dt(r.updated))fail("job interval");seconds+=s;allocated++;let host=j.labels[0];byHost[host]=(byHost[host]||0)+s;if(j.conclusion==="failure")failed.push({name:j.name,secondsAfterCreation:(end-dt(r.created))/1000})}
 if((r.conclusion==="failure")!==(failed.length>0))fail("conclusion");
 return {id:r.id,allocated,skipped,seconds,byHost,firstFailedCompletionSeconds:failed.length?Math.min(...failed.map(x=>x.secondsAfterCreation)):null,wallSeconds:(dt(r.updated)-dt(r.created))/1000,failed};
}
function compare(m){const [a,b,c,d]=m;return {ciOnlySecondsDifference:a.seconds-c.seconds,ciOnlyReductionPct:100*(1-c.seconds/a.seconds),includingIndependentReproSecondsBefore:a.seconds+b.seconds,includingIndependentReproSecondsAfter:c.seconds+d.seconds,firstFailedJobCompletionBeforeSeconds:a.firstFailedCompletionSeconds,firstFailedJobCompletionAfterSeconds:c.firstFailedCompletionSeconds}}
function test(receipts,cases){return cases.map(([name,mut])=>{let r=JSON.parse(JSON.stringify(receipts[2]));mut(r);try{calc(r)}catch(e){return {name,result:"refused",reason:e.message}}throw Error("accepted mutant "+name)})}
const cases=[["missing page",r=>r.pages.pop()],["duplicate job",r=>r.pages[0].jobs[1]=r.pages[0].jobs[0]],["foreign head",r=>r.pages[0].jobs[0].head="0".repeat(40)],["unexpected attempt",r=>r.pages[0].jobs[0].attempt=2],["unallocated failure",r=>{r.pages[0].jobs.find(x=>!x.runner).conclusion="failure"}],["negative duration",r=>{r.pages[0].jobs.find(x=>x.runner).completed="2020-01-01T00:00:00Z"}],["incomplete job",r=>r.pages[0].jobs[0].status="in_progress"],["false run conclusion",r=>r.conclusion="success"],["duplicate allocated name",r=>{let a=r.pages[0].jobs.filter(j=>j.runner);a[1].name=a[0].name}]];
const metrics=receipts.map(calc);console.log(JSON.stringify({metrics,comparison:compare(metrics),tests:test(receipts,cases),authority:false},null,2));

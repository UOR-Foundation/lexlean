// DOD-8c advisory, source-derived expected names for #609's specific v0.4 non-cut PR.
// Not release authority. Only attempt 1 supported; a retry needs separate attempt lineage reconciliation.
// Read a freshly independently captured receipt JSON as argv[2], or replay frozen observation and synthetic controls.
// Inventory extracted from exact workflow blocks and vv-rows.json. Not a generalized YAML interpreter or proof of artifact contents.
import {readFileSync} from "node:fs";
const expected={
  "repo": "UOR-Foundation/lexlean",
  "head": "a5e846de239113ca591f949eb75764d40096f7e9",
  "base": "d553cbf42f4d51673f6e4040da2337dda4627c09",
  "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
  "ci": 45490646681,
  "repro": 45490646770,
  "workflowBlob": "3eeaa65755e2e12499f513590ff014db84404bdc",
  "rowsBlob": "f35167d5551cec6db851140a6c3aa9955b91fb10",
  "success": [
    "Windows tests (lower)",
    "Windows tests (moves)",
    "Windows tests (print)",
    "Windows tests (producer)",
    "Windows tests (validate)",
    "atlas (census-a)",
    "atlas (census-b)",
    "atlas (census-c)",
    "atlas (census-d)",
    "atlas (cert4)",
    "atlas (cert4-replay)",
    "atlas (closure)",
    "atlas (join)",
    "atlas (orbit)",
    "atlas (pairs-a)",
    "atlas (pairs-b)",
    "atlas (pairs-c)",
    "atlas (pairs-d)",
    "atlas (prefix)",
    "bootstrap export",
    "build and non-Lean tests (aarch64-apple-darwin)",
    "build and non-Lean tests (aarch64-unknown-linux-gnu)",
    "build and non-Lean tests (x86_64-pc-windows-msvc)",
    "honesty",
    "just vv (bdd-bootstrap-assemble)",
    "just vv (bdd-bootstrap-bodies)",
    "just vv (bdd-bootstrap-check)",
    "just vv (bdd-bootstrap-cli-output)",
    "just vv (bdd-bootstrap-content-id)",
    "just vv (bdd-bootstrap-core-json)",
    "just vv (bdd-bootstrap-core-print)",
    "just vv (bdd-bootstrap-core-read)",
    "just vv (bdd-bootstrap-diagnostics)",
    "just vv (bdd-bootstrap-gnaf)",
    "just vv (bdd-bootstrap-grammar-math)",
    "just vv (bdd-bootstrap-grammar-proof)",
    "just vv (bdd-bootstrap-lexicon-entry)",
    "just vv (bdd-bootstrap-lexicon-lre)",
    "just vv (bdd-bootstrap-lexicon-lse)",
    "just vv (bdd-bootstrap-lexicon-schema)",
    "just vv (bdd-bootstrap-lock-text)",
    "just vv (bdd-bootstrap-loops)",
    "just vv (bdd-bootstrap-lower)",
    "just vv (bdd-bootstrap-lse-scopes)",
    "just vv (bdd-bootstrap-lse-types)",
    "just vv (bdd-bootstrap-manifest-text)",
    "just vv (bdd-bootstrap-moves)",
    "just vv (bdd-bootstrap-package)",
    "just vv (bdd-bootstrap-package-ref)",
    "just vv (bdd-bootstrap-print)",
    "just vv (bdd-bootstrap-project-text)",
    "just vv (bdd-bootstrap-proposition-terms)",
    "just vv (bdd-bootstrap-runtime)",
    "just vv (bdd-bootstrap-semantic-print)",
    "just vv (bdd-bootstrap-semantic-read)",
    "just vv (bdd-bootstrap-structural-grammar)",
    "just vv (bdd-bootstrap-succeed)",
    "just vv (bdd-bootstrap-surface-doc)",
    "just vv (bdd-bootstrap-surface-generic)",
    "just vv (bdd-bootstrap-surface-lex)",
    "just vv (bdd-bootstrap-surface-module)",
    "just vv (bdd-bootstrap-surface-tooling)",
    "just vv (bdd-bootstrap-terms)",
    "just vv (bdd-bootstrap-toml-read)",
    "just vv (bdd-bootstrap-tooling-latex)",
    "just vv (bdd-bootstrap-validate)",
    "just vv (bdd-compiler)",
    "just vv (bdd-language-models)",
    "just vv (bdd-preservation)",
    "just vv (bdd-preservation-lists)",
    "just vv (bdd-preservation-prims)",
    "just vv (bdd-preservation-runtime)",
    "just vv (bdd-preservation-std)",
    "just vv (bdd-preservation-support)",
    "just vv (bdd-production)",
    "just vv (bdd-stdlib)",
    "just vv (bdd-verification)",
    "just vv (bdd-wasi)",
    "just vv (examples)",
    "just vv (rest)",
    "macOS arm64 tests (lower)",
    "macOS arm64 tests (moves)",
    "macOS arm64 tests (print)",
    "macOS arm64 tests (producer)",
    "macOS arm64 tests (validate)",
    "plan"
  ],
  "skip": [
    "Intel conformance (${{ matrix.lane }})",
    "Intel test construction and local targets",
    "build and non-Lean tests (x86_64-apple-darwin)"
  ]
};
const observed={
  "ci": {
    "id": 45490646681,
    "head": "a5e846de239113ca591f949eb75764d40096f7e9",
    "status": "in_progress",
    "conclusion": null,
    "attempt": 1,
    "total": 37,
    "pages": [
      {
        "page": 1,
        "total": 37,
        "jobs": [
          {
            "id": 138787742928,
            "name": "plan",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "GitHub Actions 1000182577",
            "steps": 6
          },
          {
            "id": 138788102640,
            "name": "honesty",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "GitHub Actions 1000182579",
            "steps": 11
          },
          {
            "id": 138790487035,
            "name": "build and non-Lean tests (aarch64-unknown-linux-gnu)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "in_progress",
            "conclusion": null,
            "runner": "GitHub Actions 1000182588",
            "steps": 6
          },
          {
            "id": 138790487068,
            "name": "bootstrap export",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "in_progress",
            "conclusion": null,
            "runner": "GitHub Actions 1000182591",
            "steps": 11
          },
          {
            "id": 138790487080,
            "name": "atlas (prefix)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "GitHub Actions 1000182592",
            "steps": 13
          },
          {
            "id": 138790487239,
            "name": "Windows tests (print)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "in_progress",
            "conclusion": null,
            "runner": "GitHub Actions 1000182585",
            "steps": 7
          },
          {
            "id": 138790487300,
            "name": "Windows tests (producer)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "GitHub Actions 1000182584",
            "steps": 8
          },
          {
            "id": 138790487312,
            "name": "Windows tests (validate)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "in_progress",
            "conclusion": null,
            "runner": "GitHub Actions 1000182586",
            "steps": 7
          },
          {
            "id": 138790487348,
            "name": "Windows tests (moves)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "in_progress",
            "conclusion": null,
            "runner": "GitHub Actions 1000182587",
            "steps": 7
          },
          {
            "id": 138790487349,
            "name": "Windows tests (lower)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "in_progress",
            "conclusion": null,
            "runner": "GitHub Actions 1000182595",
            "steps": 7
          },
          {
            "id": 138790487517,
            "name": "macOS arm64 tests (lower)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "in_progress",
            "conclusion": null,
            "runner": "GitHub Actions 1000182590",
            "steps": 7
          },
          {
            "id": 138790487536,
            "name": "macOS arm64 tests (moves)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "in_progress",
            "conclusion": null,
            "runner": "GitHub Actions 1000182589",
            "steps": 7
          },
          {
            "id": 138790487566,
            "name": "macOS arm64 tests (producer)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "GitHub Actions 1000182612",
            "steps": 8
          },
          {
            "id": 138790487571,
            "name": "macOS arm64 tests (validate)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "in_progress",
            "conclusion": null,
            "runner": "GitHub Actions 1000182596",
            "steps": 7
          },
          {
            "id": 138790487644,
            "name": "macOS arm64 tests (print)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "in_progress",
            "conclusion": null,
            "runner": "GitHub Actions 1000182665",
            "steps": 7
          },
          {
            "id": 138790487645,
            "name": "just vv (bdd-preservation-prims)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "GitHub Actions 1000182593",
            "steps": 13
          },
          {
            "id": 138790487700,
            "name": "just vv (bdd-language-models)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "GitHub Actions 1000182600",
            "steps": 13
          },
          {
            "id": 138790487707,
            "name": "just vv (rest)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "in_progress",
            "conclusion": null,
            "runner": "GitHub Actions 1000182599",
            "steps": 12
          },
          {
            "id": 138790487738,
            "name": "just vv (bdd-production)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "in_progress",
            "conclusion": null,
            "runner": "GitHub Actions 1000182601",
            "steps": 12
          },
          {
            "id": 138790487744,
            "name": "just vv (bdd-preservation-runtime)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "GitHub Actions 1000182603",
            "steps": 13
          },
          {
            "id": 138790487751,
            "name": "just vv (bdd-verification)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "GitHub Actions 1000182594",
            "steps": 13
          },
          {
            "id": 138790487759,
            "name": "just vv (bdd-stdlib)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "GitHub Actions 1000182606",
            "steps": 13
          },
          {
            "id": 138790487761,
            "name": "just vv (bdd-preservation-support)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "GitHub Actions 1000182604",
            "steps": 13
          },
          {
            "id": 138790487789,
            "name": "just vv (bdd-preservation-std)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "GitHub Actions 1000182602",
            "steps": 13
          },
          {
            "id": 138790487813,
            "name": "just vv (examples)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "in_progress",
            "conclusion": null,
            "runner": "GitHub Actions 1000182597",
            "steps": 12
          },
          {
            "id": 138790487858,
            "name": "just vv (bdd-preservation-lists)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "GitHub Actions 1000182598",
            "steps": 13
          },
          {
            "id": 138790487993,
            "name": "just vv (bdd-preservation)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "GitHub Actions 1000182605",
            "steps": 13
          },
          {
            "id": 138790488208,
            "name": "just vv (bdd-wasi)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "GitHub Actions 1000182607",
            "steps": 13
          },
          {
            "id": 138790489366,
            "name": "Intel test construction and local targets",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "skipped",
            "runner": null,
            "steps": 0
          },
          {
            "id": 138790491712,
            "name": "Intel conformance (${{ matrix.lane }})",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "skipped",
            "runner": null,
            "steps": 0
          },
          {
            "id": 138790493742,
            "name": "build and non-Lean tests (x86_64-apple-darwin)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "skipped",
            "runner": null,
            "steps": 0
          },
          {
            "id": 138808378528,
            "name": "atlas (census-d)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "in_progress",
            "conclusion": null,
            "runner": "GitHub Actions 1000182677",
            "steps": 11
          },
          {
            "id": 138808378535,
            "name": "atlas (census-b)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "in_progress",
            "conclusion": null,
            "runner": "GitHub Actions 1000182675",
            "steps": 11
          },
          {
            "id": 138808378555,
            "name": "atlas (cert4)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "GitHub Actions 1000182678",
            "steps": 12
          },
          {
            "id": 138808378588,
            "name": "atlas (census-c)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "in_progress",
            "conclusion": null,
            "runner": "GitHub Actions 1000182679",
            "steps": 11
          },
          {
            "id": 138808378622,
            "name": "atlas (census-a)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "in_progress",
            "conclusion": null,
            "runner": "GitHub Actions 1000182676",
            "steps": 11
          },
          {
            "id": 138810269177,
            "name": "atlas (cert4-replay)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "in_progress",
            "conclusion": null,
            "runner": "GitHub Actions 1000182688",
            "steps": 11
          }
        ]
      }
    ]
  },
  "repro": {
    "id": 45490646770,
    "head": "a5e846de239113ca591f949eb75764d40096f7e9",
    "status": "completed",
    "conclusion": "success",
    "attempt": 1,
    "total": 1,
    "pages": [
      {
        "page": 1,
        "total": 1,
        "jobs": [
          {
            "id": 138787742569,
            "name": "repro",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "GitHub Actions 1000182578",
            "steps": 7
          }
        ]
      }
    ]
  }
};
const synthetic={
  "ci": {
    "id": 45490646681,
    "head": "a5e846de239113ca591f949eb75764d40096f7e9",
    "status": "completed",
    "conclusion": "success",
    "attempt": 1,
    "total": 89,
    "pages": [
      {
        "page": 1,
        "total": 89,
        "jobs": [
          {
            "id": 1,
            "name": "Windows tests (lower)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 2,
            "name": "Windows tests (moves)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 3,
            "name": "Windows tests (print)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 4,
            "name": "Windows tests (producer)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 5,
            "name": "Windows tests (validate)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 6,
            "name": "atlas (census-a)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 7,
            "name": "atlas (census-b)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 8,
            "name": "atlas (census-c)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 9,
            "name": "atlas (census-d)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 10,
            "name": "atlas (cert4)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 11,
            "name": "atlas (cert4-replay)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 12,
            "name": "atlas (closure)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 13,
            "name": "atlas (join)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 14,
            "name": "atlas (orbit)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 15,
            "name": "atlas (pairs-a)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 16,
            "name": "atlas (pairs-b)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 17,
            "name": "atlas (pairs-c)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 18,
            "name": "atlas (pairs-d)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 19,
            "name": "atlas (prefix)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 20,
            "name": "bootstrap export",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 21,
            "name": "build and non-Lean tests (aarch64-apple-darwin)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 22,
            "name": "build and non-Lean tests (aarch64-unknown-linux-gnu)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 23,
            "name": "build and non-Lean tests (x86_64-pc-windows-msvc)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 24,
            "name": "honesty",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 25,
            "name": "just vv (bdd-bootstrap-assemble)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 26,
            "name": "just vv (bdd-bootstrap-bodies)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 27,
            "name": "just vv (bdd-bootstrap-check)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 28,
            "name": "just vv (bdd-bootstrap-cli-output)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 29,
            "name": "just vv (bdd-bootstrap-content-id)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 30,
            "name": "just vv (bdd-bootstrap-core-json)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 31,
            "name": "just vv (bdd-bootstrap-core-print)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 32,
            "name": "just vv (bdd-bootstrap-core-read)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 33,
            "name": "just vv (bdd-bootstrap-diagnostics)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 34,
            "name": "just vv (bdd-bootstrap-gnaf)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 35,
            "name": "just vv (bdd-bootstrap-grammar-math)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 36,
            "name": "just vv (bdd-bootstrap-grammar-proof)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 37,
            "name": "just vv (bdd-bootstrap-lexicon-entry)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 38,
            "name": "just vv (bdd-bootstrap-lexicon-lre)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 39,
            "name": "just vv (bdd-bootstrap-lexicon-lse)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 40,
            "name": "just vv (bdd-bootstrap-lexicon-schema)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 41,
            "name": "just vv (bdd-bootstrap-lock-text)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 42,
            "name": "just vv (bdd-bootstrap-loops)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 43,
            "name": "just vv (bdd-bootstrap-lower)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 44,
            "name": "just vv (bdd-bootstrap-lse-scopes)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 45,
            "name": "just vv (bdd-bootstrap-lse-types)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 46,
            "name": "just vv (bdd-bootstrap-manifest-text)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 47,
            "name": "just vv (bdd-bootstrap-moves)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 48,
            "name": "just vv (bdd-bootstrap-package)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 49,
            "name": "just vv (bdd-bootstrap-package-ref)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 50,
            "name": "just vv (bdd-bootstrap-print)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          }
        ]
      },
      {
        "page": 2,
        "total": 89,
        "jobs": [
          {
            "id": 51,
            "name": "just vv (bdd-bootstrap-project-text)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 52,
            "name": "just vv (bdd-bootstrap-proposition-terms)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 53,
            "name": "just vv (bdd-bootstrap-runtime)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 54,
            "name": "just vv (bdd-bootstrap-semantic-print)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 55,
            "name": "just vv (bdd-bootstrap-semantic-read)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 56,
            "name": "just vv (bdd-bootstrap-structural-grammar)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 57,
            "name": "just vv (bdd-bootstrap-succeed)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 58,
            "name": "just vv (bdd-bootstrap-surface-doc)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 59,
            "name": "just vv (bdd-bootstrap-surface-generic)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 60,
            "name": "just vv (bdd-bootstrap-surface-lex)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 61,
            "name": "just vv (bdd-bootstrap-surface-module)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 62,
            "name": "just vv (bdd-bootstrap-surface-tooling)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 63,
            "name": "just vv (bdd-bootstrap-terms)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 64,
            "name": "just vv (bdd-bootstrap-toml-read)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 65,
            "name": "just vv (bdd-bootstrap-tooling-latex)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 66,
            "name": "just vv (bdd-bootstrap-validate)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 67,
            "name": "just vv (bdd-compiler)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 68,
            "name": "just vv (bdd-language-models)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 69,
            "name": "just vv (bdd-preservation)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 70,
            "name": "just vv (bdd-preservation-lists)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 71,
            "name": "just vv (bdd-preservation-prims)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 72,
            "name": "just vv (bdd-preservation-runtime)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 73,
            "name": "just vv (bdd-preservation-std)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 74,
            "name": "just vv (bdd-preservation-support)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 75,
            "name": "just vv (bdd-production)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 76,
            "name": "just vv (bdd-stdlib)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 77,
            "name": "just vv (bdd-verification)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 78,
            "name": "just vv (bdd-wasi)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 79,
            "name": "just vv (examples)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 80,
            "name": "just vv (rest)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 81,
            "name": "macOS arm64 tests (lower)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 82,
            "name": "macOS arm64 tests (moves)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 83,
            "name": "macOS arm64 tests (print)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 84,
            "name": "macOS arm64 tests (producer)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 85,
            "name": "macOS arm64 tests (validate)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 86,
            "name": "plan",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "synthetic",
            "steps": 1
          },
          {
            "id": 87,
            "name": "Intel conformance (${{ matrix.lane }})",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "skipped",
            "runner": null,
            "steps": 0
          },
          {
            "id": 88,
            "name": "Intel test construction and local targets",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "skipped",
            "runner": null,
            "steps": 0
          },
          {
            "id": 89,
            "name": "build and non-Lean tests (x86_64-apple-darwin)",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "skipped",
            "runner": null,
            "steps": 0
          }
        ]
      }
    ]
  },
  "repro": {
    "id": 45490646770,
    "head": "a5e846de239113ca591f949eb75764d40096f7e9",
    "status": "completed",
    "conclusion": "success",
    "attempt": 1,
    "total": 1,
    "pages": [
      {
        "page": 1,
        "total": 1,
        "jobs": [
          {
            "id": 138787742569,
            "name": "repro",
            "head": "a5e846de239113ca591f949eb75764d40096f7e9",
            "status": "completed",
            "conclusion": "success",
            "runner": "GitHub Actions 1000182578",
            "steps": 7
          }
        ]
      }
    ]
  }
};
function check(e,o){const hold=reason=>({status:"HOLD",reason,authority:false});const ok=(r,id)=>r&&r.id===id&&r.head===e.head&&r.attempt===1&&Array.isArray(r.pages)&&r.pages.length===Math.ceil(r.total/50);function inventory(r){let j=[];for(let i=0;i<r.pages.length;i++){const p=r.pages[i];if(p.page!==i+1||p.total!==r.total||p.jobs.length!==Math.min(50,r.total-i*50))return null;j.push(...p.jobs)}if(new Set(j.map(x=>x.id)).size!==j.length||new Set(j.map(x=>x.name)).size!==j.length||j.some(x=>x.head!==e.head))return null;return j}
if(!ok(o.ci,e.ci)||!ok(o.repro,e.repro))return hold("run_identity");const j=inventory(o.ci),r=inventory(o.repro);if(!j||!r)return hold("page_or_job_identity");
if(o.ci.status!=="completed"||o.repro.status!=="completed")return hold("nonterminal");
if(o.ci.conclusion!=="success"||o.repro.conclusion!=="success")return hold("workflow_failure");
const names=[...e.success,...e.skip].sort();if(JSON.stringify(j.map(x=>x.name).sort())!==JSON.stringify(names))return hold("expected_inventory");
if(j.some(x=>x.status!=="completed"||(e.skip.includes(x.name)?x.conclusion!=="skipped"||x.runner||x.steps!==0:x.conclusion!=="success"||!x.runner||x.steps<1)))return hold("job_result");
if(r.length!==1||r[0].name!=="repro"||r[0].status!=="completed"||r[0].conclusion!=="success"||!r[0].runner||r[0].steps<1)return hold("repro_inventory");
return {status:"PREMERGE_JOB_RECEIPTS_MATCH",reason:"Intel postmerge, exact tree, artifact custody, authorization and release evidence remain separate",authority:false}}
function tests(e,s,cases){return cases.map(([name,mut,want])=>{let c=JSON.parse(JSON.stringify(s));mut(c);const r=check(e,c);if(r.status!==want||r.authority!==false)throw Error(name);return {name,status:r.status}})}
const cases=[["positive",s=>{},"PREMERGE_JOB_RECEIPTS_MATCH"],["missing page",s=>s.ci.pages.pop(),"HOLD"],["foreign SHA",s=>s.ci.head="0".repeat(40),"HOLD"],["changed job SHA",s=>s.ci.pages[0].jobs[0].head="0".repeat(40),"HOLD"],["duplicate job",s=>s.ci.pages[0].jobs[1]=s.ci.pages[0].jobs[0],"HOLD"],["wrong job",s=>s.ci.pages[0].jobs[0].name="other","HOLD"],["skip required",s=>{let j=s.ci.pages.flatMap(p=>p.jobs).find(j=>j.name==="honesty");j.conclusion="skipped";j.runner=null;j.steps=0},"HOLD"],["unexpected Intel execution",s=>{let j=s.ci.pages.flatMap(p=>p.jobs).find(j=>j.name.startsWith("Intel conformance"));j.conclusion="success";j.runner="x";j.steps=1},"HOLD"],["failed repro",s=>s.repro.conclusion="failure","HOLD"],["nonterminal",s=>s.ci.status="in_progress","HOLD"],["attempt changed",s=>s.ci.attempt=2,"HOLD"]];
console.log(JSON.stringify({expected,observed:check(expected,process.argv[2]?JSON.parse(readFileSync(process.argv[2],"utf8")):observed),tests:tests(expected,synthetic,cases)},null,2));

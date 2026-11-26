# Operational CI handoff — provisional, #603

Status: advisory runbook, not normative workflow, approval or final operational acceptance. Current qualified release/v0.5 base 903797a784df34f0bc0c2910e1f68ad7163b1aa6 (tree 2f17544694c5e3243b7b09f10fbbbf2dcb54e62c); proposed B1 candidate b5bfe17ca44ecca3f6bf95798c7788e6254c790a (#630 includes #633). Recheck source and authority after every integration. No change to SPEC, AGENTS, branch rules or gate requirements.

## Everyday development, effective on the current qualified line

1. Work on a draft PR into the correct release line. Automatic draft opened/synchronize events allocate no runner for CI or independent reproducibility. Draft/skipped is NOT accepted evidence. Stack dependent changes where helpful; stacked-base feedback cannot land a release.
2. Run focused local checks for touched code, including formatting, relevant unit/conformance tests and negative controls; use the actual Justfile recipes and repository rules. Full normative command remains just vv; local subset is feedback only. Optional explicitly dispatched draft CI runs honesty and rest, not full qualification; manual dispatch can consume runners. Do not dispatch merely to obtain a green status.
3. Before Ready, inspect exact head, current target base, source/contract changes, artifact and identity obligations, prior unresolved reviews, and required preflight. If a new commit is pushed while draft, wait for its automatic draft CI and repro runs to finish skipped before marking Ready: #623 exposed a synchronize/ready concurrency race. Do not mark Ready twice or manually dispatch a duplicate gate.
4. One Ready candidate per serialized release-line landing; other development remains in drafts. Ready PR into the line requires full job roster, proof/Atlas joins, native receipts and independent repro as applicable to that line, plus required regeneration where the identity changes. Verify all against exact candidate head and current base. A provider success or mergeable flag alone is insufficient.
5. Recheck head and base immediately before authorized merge. Follow AGENTS and SPEC §9.4 (main receives the release line at its tag; identity-changing PRs land one at a time with regeneration and full gate). Merge with preserved tree and ordered parents; no unqualified force push. Postmerge verify same-tree reuse, any push-only native obligations, independent repro and tag/forward behavior. A later-tree qualification is not retrospective proof for a failed exact earlier tree.

## Recovery decision table

| Observation | Action |
| --- | --- |
| Deterministic failure or changed source | Fix on draft, rerun focused tests, qualify final changed tree. No old-head acceptance. |
| Infrastructure interruption | Confirm terminal run and exact head, inspect dependency closure and actual provider attempt semantics; use bounded provider-native failed-job recovery only if all retained executions/receipts remain eligible under current line. |
| Missing/expired/foreign receipt, skipped required role, unproven retention | Refuse reuse; requalify affected scope only if current policy proves it, otherwise full gate. |
| Apparent green but missing expected jobs/joins | HOLD; inspect planner/release validator, do not tag or merge. |
| Same-tree merge with eligible prior complete gate | Let planner validate the exact tree and receipt; observe actual postmerge jobs. Never cancel necessary push-only native work solely because PR CI passed. |
| New change on stale base | Recompose and qualify actual candidate; do not infer acceptance from stale PR metadata. |

#628/#631 cross-attempt/host inventory changes are not deployed; current conservative recovery remains authoritative. #621 read traces are advisory, never a basis to skip proof/test obligations. #630/#633 must be separately qualified before adopting their revised scheduling/provider lookup. Manual composite admission may be considered for reviewed compatible nonidentity changes under existing policy; #607/#609 demonstrates one exact composite PR admission, not unrestricted batch or native merge queue acceptance. Urgent fixes can take ordinary full qualification.

## Operational exit evidence (not yet all satisfied)

- [x] Qualified initial cutover #619/#620 and #621 advisory trace landing. Existing safeguards preserved; source-bound acceptance remains.
- [ ] B1 #630/#633 exact Ready gate, independent repro, authorized tree-equal merge, postmerge receipt and measured cost/latency/feedback decision. Capture before introducing C. Do not repeat a gate solely for measurement.
- [ ] #624 forward recovery: artifact-specific diagnosis and approved bounded patch lifecycle; source/identity migration qualified; carry existing f100b2cd performance improvement, measure actual impact separately from B1. Reassess Linux ARM only after measurement.
- [ ] #628/#631: independent full-source/native validation and real retained-attempt qualification if adopted, or explicit disposition while retaining current conservative recovery. No early activation.
- [ ] DOD-8 batch decision based on existing pilot, arrival/failure/age tradeoffs, owner acceptance; no automatic permanent queue or policy changes.
- [ ] Final operational configuration: record qualified source/tree, required roster, observed allocation and queue/critical path/feedback, known limitations, rollback, and owner acceptance. If wall-clock improvement is not measured, state it and record decision rather than imply lower runner cost means faster feedback.

Research closure (DOD-6–10) stays separate: immutable A, generic B1/B2, then C, then D external refinements. Natural 50-PR observation does not prevent development; it remains a case-study requirement unless explicitly adjusted. No new speculative experiments or external-influenced changes in active internal phases.

## Evidence references

- #621 CI 45695722918 70/70, independent reproducibility 45695722901, merge 903797a7; postmerge 45738611061 reused the qualified tree.
- #630 review 6067619839 at b5bfe17c; automatic draft CI 45739062667 and repro 45739062705 skipped. No Ready gate at checkpoint.
- #607/#609 composite: PR CI 45490646681 success (86 executed, 3 planned skips), repro 45490646770 success; merged fa2280e2, exact tree e736f044. Postmerge Intel interrupted; no false completion claim.
- #603 comments 7180812672 (scope convergence), 7180974631 (B2 evidence). See immutable receipt files in .github/metrics.
- Source anchors at 903797a7: AGENTS.md 7e177a9f9698883066f04f779fe53004ab63c38c, ci-plan.sh d6549e06f382478e27407fd6294685dbf5695bb1, ci.yml e06e5350a95f13146802ad8613d123134248ad06, Justfile 024b419a17f82f29d75b0569cd1d6b45127c1562, reproducibility.yml f1ca0057f94a71ba04f2080e598fdaad7aafa95c.

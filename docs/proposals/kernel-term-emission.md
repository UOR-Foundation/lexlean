# Proofs as kernel terms: verification without the elaborator

Draft for owner acceptance, written 2026-11-19, against `main` at `29554e9`. Normative text is `SPEC.md`; this document explains and asks. It is a proposal only: it adds this one file and changes no specification text, register row, workflow, compiler code, or generated document. Every number below is evidence from a spike, at no honesty level; nothing here is a claim.

## 1. Summary

Finding a proof needs Lean's elaborator and library. Checking one needs only the kernel. Today `verify` does both, every time, and so does every tab that wants to prove a LexLean module.

The proposal is that `lexlean build` writes each proof as a kernel term, so that checking needs Lean's kernel and nothing else:

- **Closed proof forms** (`rfl`, `exact`, `rw`, `constructor`, and their kin) become terms written directly from the linked IR.
- **Search forms** (`linear_arithmetic`, later `decide` and the simplifier sets) become a small certificate plus `Eq.refl true`. A checker, proved sound once, is evaluated by the kernel on that certificate. The search that found it needs no trust.
- **The kernel stays Lean's own**, the C++ kernel Foundry already ships as WebAssembly (`foundry/v2/in-tab-kernel`).

**What it saves.** A tab needs the 421 KB kernel and a pruned library, not the 66 MB elaborator module and 98 MB of `Init` that the in-tab elaborator plan carries. On the spike, proofs were found in under a millisecond and judged in milliseconds, and with every input addressed by κ-label an unchanged 20,000-theorem module cost 13 ms and an edit 168 ms.

**What it changes.** A proof judged this way was not elaborated by Lean in that run. R9 says verified status requires Lean elaboration, so this is the owner's decision (D3).

| # | Decision | Recommendation |
| --- | --- | --- |
| D1 | `build` writes kernel terms for the proof forms it can | Yes, form by form |
| D2 | Which kernel judges them | Lean's own kernel, native in `verify` and as WebAssembly in the tab. No new kernel |
| D3 | What R9's "verified" requires | Unchanged. Add a separate, weaker status for "kernel-judged from emitted terms"; never report it as verified |
| D4 | How search forms are proved | By reflection: a small checker in a source package, and bridging lemmas around it, all verified by the full pipeline |
| D5 | The oracle for emitted terms | Per declaration: the emitted statement equals the one Lean elaborated, and the kernel's verdict agrees with full `verify` |
| D6 | The runtime's importability | `verify` keeps, or publishes by address, what an importer of the runtime needs |
| D7 | The in-tab elaborator plan of 2026-11-18 | Keep it as the fallback prover and as the oracle of D5; emission takes forms off its load one at a time |
| D8 | Reuse of kernel verdicts by κ-label | Yes, from the prover's own store only, keyed by the environment; declarations are shared, verdicts are not |
| D9 | Proofs no tab can judge quickly (Atlas) | Decide who may attest a verdict by address, or accept that such modules are not self-proved in the tab |

## 2. What exists

- `check` runs on `wasm32-wasip1`. `verify` shells out to `lean`, `leanchecker` and the axiom audit under the pinned toolchain.
- Foundry runs Lean 4.32.1's kernel in the tab: 2.2 MB, 421 KB gzipped, with whole-file and resident-library checking and an axiom report. Its report measures `FoldLaws.Kernels` at 1.69 s for the kernel against 12.53 s for elaboration, both in WebAssembly.
- Foundry's own assessment (`ELABORATOR.md`) names the two levers this proposal pulls: a runtime that does not drag in all of Lean, and proofs as terms rather than tactics.
- The Atlas core form already carries declarations and proofs as kernel-term data, rebuilt with `addDecl`.
- Three open pull requests touch the same ground and are assumed, not replaced: #571 and #581 compute every identity as a BLAKE3 κ-label; #578 adds `lexlean lean-modules`, which prints the Lean a build elaborates, runtime included; #580 has a release publish the browser prover's assets by κ-label.

## 3. Evidence

All runs used the kernel WebAssembly pinned in Foundry's manifest, under Node 22 on one Windows desktop. No browser was used. The kernel is 4.32.1; the exports are from 4.32.2.

**Pruned certificates.** Each example was elaborated by Lean, exported with `lean4export` restricted to the declarations it reaches, and judged whole. The unpruned export of `Init` is 345 MB.

| Example | Certificate, gzipped | Declarations | Kernel |
| --- | --- | --- | --- |
| nat-add-zero | 4.6 KB | 26 | 3 ms |
| list-induction | 21 KB | 95 | 18 ms |
| propositional-logic | 30 KB | 118 | 14 ms |
| peano-arithmetic | 76 KB | 274 | 117 ms |

**Direct emission.** A 270-line emitter wrote terms for all 8 theorems of peano's `Addition` module (`rfl`, `exact`, `constructor`, `rw`) with no Lean running. Emit took 7 to 10 ms including parsing a 72 KB library. The kernel judged the 8 proofs in 1.2 to 1.9 ms. All 8 statements were identical to Lean's. A false statement was refused.

**Reflection, synthetic.** A checker for linear integer inequalities, proved sound in about 200 lines of Lean, on 8 hand-written theorems: search and emit 0.4 to 0.5 ms, proof 2 to 3 KB, kernel 3 to 8 ms for the six small ones. A chain of 40 inequalities took 55 to 69 ms to search, 27 KB, and 90 to 138 ms in the kernel; `omega`'s own term for it is about 216 KB and no faster to check. Forged certificates were refused on both a false and a true statement.

**Reflection, real.** Against the Lean that `lexlean build` generates for `examples/fold-laws`, over the genuine runtime and the real `requantize`:

| Theorem | Search and emit | Proof | Kernel | Statement |
| --- | --- | --- | --- | --- |
| `requantize_low` | 0.58 ms | 2.1 KB | 5.7 ms | identical to Lean's |
| `requantize_high` | 0.49 ms | 2.1 KB | 5.2 ms | identical to Lean's |

That first checker covered 2 of the 16 theorems of `Kernels`. The false bound `1 <= requantize x` found no certificate, and a forged one was refused.

**Coverage, widened.** The checker's core was then kept small and everything else moved into lemmas proved once and applied by the emitter around it:

- The core reads integer inequalities with conditionals, and refutes over the integers: a certificate is a list of cuts (a combination divided by a common factor, its constant rounded down) and a final combination that is a negative constant. Its soundness theorem rests on `propext` and `Quot.sound`.
- About fifty bridging lemmas turn each statement form into those inequalities or back: Boolean `lessThan` equal to true or false, equalities, truncating division and remainder by a constant (with the sign of the dividend taken both ways only when the search needs it), natural numbers, `UInt8` and `Int8`.
- A closed equality is proved by `Eq.refl`; the kernel evaluates both sides.

On the whole `fold-laws` example, with statements taken from Lean's elaboration (see below):

| Module | Theorems | Emitted and accepted | Axioms identical to Lean's own proof |
| --- | --- | --- | --- |
| `Kernels` | 16 | 16 | 16 |
| `Folds` | 9 | 1 | 1 |
| `Main` | 6 | 1 | 1 |

Every arithmetic theorem of the example is covered by the emitter. The 13 it does not emit are laws about folds over lists, which need induction; a fusion schema proved once would bring them in, and is not built. Until then they are covered as described under *Full coverage*. Search and emit took 0.1 to 7 ms a theorem, the proofs are 0.1 to 26 KB, and the kernel took 1 to 180 ms each (the two monotonicity theorems are the large ones, at four sign cases times nine conditional cases).

Adding a form is adding lemmas, not touching the core: natural-number equality and remainder took four lemmas and about 25 lines of emitter, and brought in `parity_step`. It also exposed a weakness in the search (it forgot that variables are integers when it projected out an equality), fixed by substituting equalities exactly. The search is still not a complete decision procedure for integer arithmetic.

**Full coverage.** What the emitter cannot find, Lean has already found: `verify` elaborates every theorem when a module is published. Those proof terms are exported as a pack that continues the library's file (the library is checked to be a byte prefix of the export that includes the proofs), and the pack is found by its κ-label. In the tab a pack is data, not code: it is outside the pin, its bytes are checked against its label, and the kernel judges every declaration in it. Each theorem is then bound to its expected statement by one more declaration, `LexBind.T : LexStmt.T := T`, which the kernel accepts only if the theorem's type is that statement.

| fold-laws, 31 theorems | Result |
| --- | --- |
| Found by the emitter | 18, 63 KB of terms, kernel 0.27 s |
| By Lean's own proof, from the pack | 13, pack 1.0 MB (164 KB gzipped), kernel 0.16 s |
| Judged proved by the kernel | 31 of 31, each with Lean's exact axioms |
| Same with Lean's own proof for all 31 | pack 2.3 MB (388 KB gzipped), kernel 0.65 s |
| In the tab, first visit, whole example | 2.2 s Chromium, 3.6 s Firefox, 2.9 s WebKit |
| In the tab, returning visit | 11 to 41 ms, no kernel started |

Native Lean 4.32.2 accepted the same packs, and accepted the library with all 31 of Lean's proofs judged from nothing (2,739 declarations). A theorem offered as the proof of another's statement was refused, and so was a pack with one number altered.

So coverage is full wherever `verify` has run, and it is Lean's own proof that is judged. What the emitter adds is that a proof it can find needs no published pack and no Lean at all, which is what an edit made in the tab needs. For a new theorem of a form the emitter does not cover, something still has to run Lean once.

**Fold laws, by fusion.** Induction was then done once, in the library: if a relation between accumulators holds at the start and every step keeps it, it holds between the folds (`fold_rel1` to `fold_rel3`, and two variants that carry the list's length). These lemmas use no axioms. The emitter abstracts the folds over one list out of the goal to get the relation, and is left with two obligations that have no fold in them, which it compiles as arithmetic. Two derived facts cover the rest: a fold over `xs ++ ys` is the fold over `ys` started from the fold over `xs`, and a fold whose steps only add, started from `z`, is `z` plus the fold started from zero (itself proved by fusion). Equalities of pairs are split into their parts.

| fold-laws, 31 theorems | Found by the emitter | Accepted by the kernel | Axioms identical to Lean's |
| --- | --- | --- | --- |
| Before fusion | 18 | 18 | 18 |
| With fusion | 31 | 31 | 29 |

The two that differ are instructive. For `requantized_total` the emitted proof uses fewer axioms than Lean's (no `Classical.choice`). For `statistics_one_pass` Lean's proof uses none and the emitted one rests on `propext` and `Quot.sound`, through the arithmetic core. Under an exact axiom policy neither is the same as Lean's result, so the rule adopted is: a proof found in place counts only if it rests on exactly the axioms declared for the theorem; otherwise Lean's own proof is judged instead. With that rule the whole example is covered as 29 found in place and 2 by Lean's own proof (a 245 KB pack, 41 KB gzipped), in 2.4 s on a first visit in Chromium, 3.1 s in Firefox and 2.6 s in WebKit, and 13 to 41 ms on a returning one. Native Lean 4.32.2 accepts all of it, including the library with all 31 emitted proofs judged from nothing.

**Statements.** In this widened run the emitter no longer writes statements: it takes each one as Lean elaborated it and finds only the proof. Writing statements from the IR is direct work with no search, and the earlier runs did it for the integer fragment, but it is not shown here for the Boolean, natural and byte forms. D5 stands either way.

**Pinned to the official kernel and library.**

- The library the tab holds is exported from the official Lean 4.32.2 toolchain's own compiled files, from the runtime `lexlean verify` compiled under that toolchain, and from the reflective library compiled by the same.
- The WebAssembly kernel was first Foundry's build of Lean 4.32.1. It was then rebuilt from a clean checkout of Lean 4.32.2 (`f3b06c70`) with Foundry's scripts and Emscripten 6.0.13: 2,235,146 bytes, 421 KB gzipped, `sha256:49cdfae0…`. Nothing under `src/kernel` is changed; the build's one patch touches two files of `src/runtime/uv`. The build took about two minutes on this desktop and carries the commit hash in its bytes. Every suite passes on it.
- Every emitted proof was also judged natively by Lean 4.32.2 itself, through `Environment.replay`: it accepted the same 18 declarations, accepted all 2,433 declarations of the library and the proofs judged from nothing in 1.3 s, and refused a proof with one number altered.
- Not checked: the toolchain's files against the release archive's hashes; whether the kernel build is byte-reproducible. The local `lexlean` carried the spike's switch that keeps verify's scratch files.

## 4. Scale

The first checker did not scale: dense coefficient lists, a list walk per variable, and naive elimination. It was rewritten with sparse coefficients, a tree for the variables, direct Boolean tests in place of `decide`, and a sparse search. The figures in section 3 predate the rewrite.

**One proof growing.** A chain of n inequalities, one theorem:

| n | Search and emit, before | after | Kernel, before | after |
| --- | --- | --- | --- | --- |
| 40 | 68 ms | 3 ms | 130 ms | 49 ms |
| 160 | 4,255 ms | 19 ms | 2,230 ms | 253 ms |
| 1,280 | out of memory | 299 ms | not reached | 3,455 ms |

Search is now linear. The kernel grows somewhat faster than linearly. Above roughly 600 nested hypotheses the engine's default call stack overflows inside the kernel; the 1,280 row needed a raised stack, which a browser does not offer. A theorem that deep is not realistic, but deeply nested data would hit the same limit.

**Many proofs.** 20,000 small theorems over one resident library: emit 0.25 ms each, kernel about 8 ms each, 164 s in all on one kernel, heap flat at 64 to 92 MB. On 8 kernel workers: 34 s, 592 theorems a second. The cost of a small arithmetic proof is the kernel's own reducer, about 1 ms per hypothesis combined, and was not reduced further.

**Atlas.** The `Group` module, exported pruned: 21.8 MB (3.6 MB gzipped), 4,981 declarations. The WebAssembly kernel accepted it in 379 s using 2.0 GB. Native `leanchecker` on the same module took 396 s and 3.4 GB, measured while another job was running. So the cost is the proofs, not WebAssembly, and no kernel judges this module at low latency. It cannot be judged on a phone at all. For proofs of this class the only fast path is to judge once and find the verdict by address (section 5), and a tab that did not do the judging has to trust whoever did.

## 5. Nothing judged twice: κ-addressed reuse

Every input is given a UOR κ-label (BLAKE3, `@uor-foundation/uor-addr` 0.4.0), and the prover looks before it works:

| Address of | Covers | A hit means |
| --- | --- | --- |
| environment | kernel, loader, library, emitter | nothing; it is part of every other address |
| module | environment, module text | every theorem in it is proved |
| chunk | environment, the module's names, 256 consecutive theorems | all 256 are proved |
| theorem | environment, its text without its name, the addresses of what it cites | proved; same content under any name is one entry |
| declaration | environment, the emitted type and term, without names | what a peer fetches and judges for itself |

A κ-label costs 37 µs for a theorem-sized key and 9 to 14 ms for the 2.2 MB kernel or the 5.2 MB library.

Measured on the 20,000-theorem module, 8 workers, first under Node with `uor-addr` labels and a file-backed memo:

| Case | Time | Judged |
| --- | --- | --- |
| First time | 33.8 s | 20,000 |
| Same module again | 13 ms | 0 |
| One theorem edited | 168 ms | 1 |
| Twenty theorems edited | 187 ms | 20 |
| One theorem added | 853 ms | 1 |
| 20,000 new theorems, 200 distinct | 1.3 s | 204 |
| New session, memo read from disk (3.3 MB) | 10 ms, no kernel started | 0 |

One false theorem among 600 failed alone; the other 599 were kept.

**The reuse rule.** A verdict is reused only from the prover's own memo, under the exact environment address. Verdicts are not accepted from elsewhere. What is shared is the declaration, found by its address, because the receiving kernel judges it again. This is the rule `InTabKernel.reuseAdmits` states in Foundry.

**In a browser tab, on Hologram's store.** The prover was then run in a Chromium 156 tab on the same desktop, served as static files with no isolation headers (`crossOriginIsolated` false, no `SharedArrayBuffer`). κ-labels are minted by Hologram's `hologram-store` (commit `12bc8cb`, built to WebAssembly with its `js-api` feature, 187 KB). The judged terms and the memo are kept in its OPFS store by address; the memo is a chain of immutable segments, and the only thing not found by content is one small file naming the newest segment. Kernels run in Web Workers.

| Case, 20,000 theorems, 8 workers | In the tab |
| --- | --- |
| Open: fetch, address the kernel and library, read the memo | 0.09 to 0.16 s |
| First time | 23.1 s, of which 1.3 s starting workers; 921 theorems a second after that |
| Same module again | 8 ms |
| One theorem edited | 81 ms |
| Twenty theorems edited | 103 ms |
| One theorem added | 189 ms |
| 20,000 new theorems, 200 distinct | 367 ms |
| Returning session after a reload | 11 ms, no kernel worker started |
| Storage used | 29.5 MB: 23.5 MB of judged terms, the rest memo |

With 20 workers on the 24-thread machine the first run took 20.1 s: the kernels contend, and little is gained past 8. Each worker's heap stayed at 64 MB.

A memo segment altered in place in OPFS was refused by the store on the next load ("OPFS content failed σ-axis verification"), and the prover judged the module again.

**One label for everything.** The tab was then rebuilt around a pin: one JSON document that names the Lean version and commit and gives the κ-label of every file that runs or is judged (kernel, loader, library, emitter, prover, worker, store). The pin's own label is the environment, so every memo address changes if any byte does.

- **Boot.** The page's inline script is the only code that runs unverified, and it only verifies: the hashing code against SHA-256 with the browser's own `crypto.subtle`, then every other file against its BLAKE3 label. Modules and workers are then built from those checked bytes, never from a URL. An altered file is refused before anything runs; so is a pin other than the one asked for.
- **The library is judged in the tab.** The first kernel a session starts judges the whole library (1.2 to 1.9 s), and that verdict is remembered under the library's and the kernel's labels. No published binding is trusted.
- **Any storage.** Because objects are found by the label of their own bytes, the store needs no trust: Hologram's OPFS store, else IndexedDB with the label re-derived on read, else memory. All three were run.
- **Only the kernels the work needs.** One kernel for a small module, up to the limit for a large one.

Measured in the same Chromium tab on the 4.32.2 kernel:

| What | Result |
| --- | --- |
| Download, gzipped | 1.75 MB: library 1.22 MB, kernel 0.43 MB, everything else 0.10 MB |
| Fetch and verify all ten files (9.4 MB) | 0.10 to 0.16 s |
| The real `Kernels` module, 16 theorems, first visit | 2.0 to 3.0 s, of which 1.2 to 1.6 s is judging the library, once |
| The same, any later visit | 0.3 ms, ready 0.11 s after the page starts, no kernel started |
| 20,000 generated theorems, first time, 8 kernels | 32 s |
| The same after one edit | 0.19 s |

**Other engines.** The same pin, unchanged, was then run headless in Playwright's Firefox 159 and WebKit 27.2 builds on the same desktop:

| | Firefox 159 | WebKit 27.2 |
| --- | --- | --- |
| Store chosen | Hologram on OPFS | IndexedDB (OPFS writes failed there, so the fallback took over) |
| Real `Kernels`, first visit | 2.2 s | 1.9 s |
| Real `Kernels`, returning visit | under 1 ms, ready 0.15 s after the page starts, no kernel | under 1 ms, ready 0.35 s, no kernel |
| 2,000 generated theorems, first time, 6 kernels | 5.8 s | 6.0 s |
| The same after one edit | 0.11 s | 0.13 s |
| One false theorem among 600 | fails alone | fails alone |

No code was changed for either engine. Playwright's WebKit on Windows is the engine Safari uses, not Safari itself, and it ran on desktop hardware.

**Not done.** No Safari on macOS or iOS, and no phone, has run it. Peers: declarations are stored by label but nothing was sent between tabs, so `KappaSync` is untested.

**Open in the design.** Adding a theorem changes the module's list of names, which invalidates every chunk address. A finer rule would bind a chunk only to the names it mentions.

## 6. Why it is sound

- **The kernel judges every term.** An emitter bug produces a refusal or a failure to emit. It cannot produce an accepted false theorem.
- **The statement is bound separately.** The kernel proves the statement it is given, so the risk is emitting the wrong statement. D5 closes it: the emitted statement must equal the one Lean elaborated from the same IR, for every declaration, on every release-line run.
- **Reflection adds one trusted fact,** the soundness theorem of the checker. It is an ordinary theorem, verified by the unchanged pipeline, with its axioms audited. In the spike it rests on `propext` and `Quot.sound`.
- **R8 holds if the emitter is a third backend of the same linked IR.** The spike parsed generated Lean text instead; that shortcut must not ship.

## 7. What it does not do

- It does not make a kernel-judged proof "verified". R9 stands; the new status is weaker and is named differently (D3).
- It does not replace the kernel, and proposes no kernel written in LexLean. That can follow later, admitted by the same corpus gate the patched in-tab engine uses.
- It does not cover the simplifier. `simp only` is the most frequent line in the generated Lean (626 of roughly 3,000 tactic lines by a rough count; `rfl` 587, `exact` 475, `intro` 324, `split` 290, `decide` 269, `omega` 201). How many of those sit inside the fixed `linear_arithmetic` script, where one certificate would replace them all, was not measured.

## 8. Order of work

1. **Certificates from `verify`.** Each verified module also publishes its pruned kernel export by address. Lean still finds every proof. No trust changes, and Foundry's tab can use it at once.
2. **Direct forms in `build`.** The closed forms, from the IR, each gated by D5.
3. **Reflective forms.** `linear_arithmetic` first: division by a literal, Boolean comparisons, fixed-width and natural numbers. The checker lives in a source package.
4. **The emitter in language 1.2,** as a bootstrap module, extracted through the certified Rust path to WebAssembly.

## 9. Open risks

- **Atlas.** Measured in section 4: minutes and gigabytes for one module, in any kernel. D9.
- **Checker speed.** Now sparse (section 4). The kernel still spends about 1 ms per hypothesis combined, and elimination can still blow up on dense problems; a simplex search would bound it.
- **Depth.** Terms nested more than a few hundred deep overflow the engine's call stack in the WebAssembly kernel.
- **Library size.** The soundness proof used `omega` and `simp`, which pull their lemmas into the library. A hand proof would shrink it.
- **Inference.** The spike infers implicit arguments by syntactic matching. A form that needs unfolding to infer one would fail to emit.
- **Integers.** The search reasons over rationals and misses facts that need rounding.

## 10. Found in passing

- `verify` cannot run on Windows: it looks for `lean` without `.exe` (`LLV7001`).
- `verify`'s scratch cleanup removes the runtime's `.olean.server` and `.olean.private` files, so no Lean file outside the run can import the runtime. The spike kept them with a local, unmerged switch. This is D6, and the request Foundry records as upstream request 2. #578 and #580 appear to close it by publishing the runtime's Lean and its compiled files; the spike did not use them.

## 11. Reproducing

The spike's emitter, checker, prover, browser page, runners and exports are held outside this repository. On acceptance of D1 they would be attached to the first pull request as evidence, not as shipped code.

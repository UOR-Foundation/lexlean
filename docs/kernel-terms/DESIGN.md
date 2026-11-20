# Kernel-term production, phase 0

A theorem whose proof is one of six semantic proof forms is now turned into a Lean kernel proof term by Rust code
inside LexLean. No elaborator, tactic, or Lean process is involved. Lean's own kernel then judges that term,
natively or compiled to WebAssembly in a browser tab. SPEC.md §18.10 is the contract, and `LN-13` is the claim and
its test. This note records:

- the term each form lowers to today and the term the producer writes;
- the library constants each form needs;
- how a produced proof is bound to the statement as written;
- the measured census, coverage, latency, and sizes;
- what is not done.

## Why this shape

Porting Lean's elaborator to WebAssembly would need about 360 MB of `.olean` data and threads, and more memory than
a phone tab has (foundry `v2/in-tab-kernel/ELABORATOR.md`). LexLean does not need it. Its proofs come from a closed
set of semantic forms, and each form fixes the shape of its proof term. So each form gets a producer that writes
that term directly. Lean 4.32.1's C++ kernel already runs in a tab as 2.2 MB of single-threaded WebAssembly
(foundry `v2/in-tab-kernel/`), and it checks whatever the producers write. That is why the producers are untrusted:
a wrong term is refused, never believed.

## Pipeline

```text
linked IR ─▶ lower (statement) ─┐
          └▶ prove (proof term) ─┴▶ lean4export 3.1.0 NDJSON ─▶ Lean's kernel ─▶ accepted + axioms, or refused
```

- `crates/lexlean/src/backend/kernel/lower.rs` writes the statement as a kernel term, from the linked IR.
- `prove.rs` writes the proof term, one fixed shape per form (the table below).
- `export.rs` writes the declarations. Over a library export it continues that export's numbering, so the output is
  a suffix the tab judges with the library resident. Without a library it is self-contained, for
  `crates/conformance/lean/KernelCheck.lean`, which judges it with nothing but the pinned Lean.
- `mod.rs` holds the public entry points: `produce`, `modules_from_snapshot`, `lean_sources`, `export_roots`.
  Every theorem gets a row: produced, or refused with the construct named.

## Binding: no statement drift

For each produced theorem `T`, two declarations are written:

```text
LexLeanKernel.T       : S := <produced proof>      S = the statement lowered from the IR
LexLeanKernel.bound.T : S := T                     T = the theorem Lean elaborated from the generated source
```

The kernel accepts `bound.T` only if `S` is definitionally the type Lean gave `T`. That type is the statement the
document wrote, as the Lean backend renders it (§18.4). So if a producer lowered a statement wrongly, or a source's
statement were altered, the binding is refused. The planted fault `altered-statement` shows this: the proof of the
altered statement still checks, and the binding does not. This is the same check foundry's `bind.mjs` makes, done
in the kernel instead of by hand.

The binding is exact up to definitional equality, and no further. One planted fault shows the edge. In forge `app`,
`naturalEventAdmitted` was given the statement of `flagEventAdmitted`, and its binding was accepted. Both statements
are closed Boolean computations that reduce to `true = true`, so the kernel cannot tell them apart, and either
proof proves both. A statement with a free variable, or one that does not
reduce to the same normal form, is told apart: every other altered-statement plant was refused.

Without Lean's `T` (a tab that never ran Lean), the produced theorem's type is the lowered statement itself. The
statement is then fixed by the lowering, a fixed function of the IR. That lowering is the one component trusted
without a kernel check (foundry's assumption A2). Wherever Lean's theorem is available, the binding discharges it
per theorem.

## Per form

The "today" column is what the Lean backend's tactic elaborates to (§18.7). The producer writes the right-hand
column directly. τ is the type of an equation's sides; `params` are the inductive type's parameters.

| Form | Today (tactic → kernel term) | Producer | Library constants |
|---|---|---|---|
| `reflexivity` | `rfl` → `@Eq.refl τ l` (`Iff.refl p` for `↔`) | the same term; a `let` goal is reduced by substituting its value | `Eq.refl`, `Iff.refl` |
| `decide` | `decide` → `@of_decide_eq_true p inst (@Eq.refl Bool true)`, `inst` found by instance search | the same term, `inst` from a fixed table read off the pinned toolchain (below); the kernel evaluates `decide p` by reduction, never by `Lean.reduceBool` or `Lean.reduceNat`, which the WebAssembly kernel refuses | `of_decide_eq_true`, `Eq.refl`, `Bool.true`, the instances |
| `cases x` | `cases x with …` → `T.casesOn` (for `Nat`, Lean's registered eliminator, definitionally `Nat.casesOn`) | `@T.casesOn params (fun x => G) x m₁ … mₙ`; the motive abstracts `x` from the goal `G`, and each minor premise binds the constructor's fields | `T.casesOn` and the constructors |
| `induction x generalizing g…` | `induction` → `T.rec` (for `Nat`, Lean's registered eliminator, definitionally `Nat.rec`) | `@T.rec params (fun x => ∀ g…, G) m₁ … mₙ x g…`; each minor premise binds the fields, then one hypothesis per recursive field (the motive at that field), then the generalized locals again | `T.rec` and the constructors |
| `apply thm args` | `exact thm args` → `thm args` | `LexLeanKernel.thm types args`: only a prior theorem that was itself produced; any other is refused by name | none beyond the arguments' |
| `law` | `exact @law types args premises` | the same term; each premise must be a produced theorem | the registered law (`LexLeanLaws.*`, part of the runtime library) |

In a case branch, the goal is the statement with the scrutinee replaced, wherever it occurs, by the constructor
applied to fresh field variables. This is what Lean's `cases` does to every hypothesis.

### The statement vocabulary

Each IR construct maps to one term, with the implicit arguments and instances Lean's elaborator fills made explicit.
The instance names were read off Lean 4.32.2 with `#synth` and `#check` under `pp.explicit`.

| IR | Term |
|---|---|
| numeral `n : Nat` | `@OfNat.ofNat Nat n (instOfNatNat n)` |
| integer `n : Int` (negative: `Neg.neg` at `Int.instNegInt`) | `@OfNat.ofNat Int n (@instOfNat n)` |
| integer `n : IntK`/`UIntK` | `@OfNat.ofNat IntK n (@IntK.instOfNat n)`, negative at `IntK.instNeg` |
| `a + b` | `@HAdd.hAdd τ τ τ (@instHAdd τ add) a b`, `add` ∈ `instAddNat`, `Int.instAdd`, `instAddIntK` |
| `a <= b`, `a < b` | `@LE.le τ le a b`, `@LT.lt τ lt a b`, `le` ∈ `instLENat`, `Int.instLEInt`, `instLEIntK` |
| `=` | `@Eq τ a b` |
| `Nat.beq`, `Nat.ble`, `Nat.blt`, `&&`, `\|\|`, `!` | the constant applied (`Bool.and`, `Bool.or`, `Bool.not`) |
| `/\`, `<->`, `->`, `forall` | `And`, `Iff`, a non-dependent `Π`, a `Π` |
| Boolean `if c` | `@ite α (c = true) (instDecidableEqBool c true) t e` |
| `match` | `@T.casesOn params (fun _ => α) s m…` |
| `let` | `let` |
| record, constructor, projection | `@S.mk args fields`, `@C targs args`, `@S.field targs v` |
| list, option, result, pair, map, set, graph literals | `List.cons`/`List.nil`, `Option.*`, `Except.*`, `Prod.mk`, and lists of pairs for maps, sets, and graphs (§17.12) |
| runtime primitive | `@LexLeanRuntime.f α inst args` or `@LexLeanCollections.f κ… inst args`; the type arguments come from the operands, and the instance from the table (`instSubNat`, `LexLeanRuntime.instQuotientNat`, `LexLeanCollections.instKeyNat`, …) |

`decide` instances: `instDecidableEqNat`, `instDecidableEqBool`, `instDecidableEqString`, `Int.instDecidableEq`,
`instDecidableEqIntK`, `instDecidableEqList`, `instDecidableEqProd`, `Option.instDecidableEq`, `Nat.decLe`/`decLt`
(and `Int.`, `IntK.`), `instDecidableAnd`, `instDecidableIff`, `instDecidableForall`. A propositional definition is
unfolded to its body's instance, which Lean's reducibility makes an instance of the application.

Anything else is refused by name. That covers fixed-width multiplication and division, widening conversions, checked
operations, instance values, contract and reasoning types, and document types without `DecidableEq`.

## Library export and verdicts

The library is a lean4export 3.1.0 export from Lean 4.32.2 (`docs/kernel-terms/run.sh`). Its roots are:

- the producers' vocabulary (`lower::VOCABULARY`, `prove::VOCABULARY`);
- the runtime vocabulary of each runtime module the project imports;
- every project declaration, with its `casesOn` and projections.

foundry's tab admits a library by its address after one whole check, then judges a suffix against the resident
library (`check_suffix`). `wasm.mjs` does exactly that.

A theorem is established when three things hold:

1. the producer wrote it;
2. the kernel accepted both declarations;
3. the axioms the kernel environment shows for the produced proof equal the theorem's declared axioms exactly
   (§22.6).

`summarize.mjs` reports each of the three. In foundry's vocabulary, the tab's verdict on a produced theorem is
"kernel-checked" by Lean's own kernel, and the binding is how it becomes "the statement the spec wrote".

## Modelled and proved in LexLean

The producers are also stated as LexLean source: `bootstrap/src/KernelTerms.lex.tex` (SPEC.md
§17.23). It works over a kernel-term datatype of its own, `Term`. A constant is a number, a binder
is a position, and universe levels and binder names are left to the stage-0 producer.

The module proves the following; the pinned Lean verifies each theorem with exactly its declared
axioms.

| Claim | Theorems | Standing |
|---|---|---|
| **Binding** (`BS-36`): `declare` gives a declaration exactly the statement it was handed as its type; a refusal declares nothing | `declare_binds_statement` | built and proved here, no axiom |
| **Totality** (`BS-37`): each producer makes a term for exactly the inputs its check passes; `decideTerm` for every input | `reflexivity_total`, `cases_total`, `induction_total`, `apply_total`, `law_total`, `decide_total` | built and proved here, `propext` (none for decide) |
| **Refusal** (`BS-38`): each producer refuses exactly the inputs its check fails, with the check's typed `Refusal`; every refusal kind is reached | `*_refuses_exactly`, `missing_case_refused`, `repeated_case_refused`, `binder_count_refused`, `unknown_case_refused`, `not_an_equation_refused`, `unproduced_theorem_refused`, `unregistered_law_refused` | built and proved here |
| **Realization** (`BS-35`): the three checks the producers decide by, rendered to certified `rust-std` packages, agree with the stage-0 producer's decisions | the production roots `checkFrom`, `memberCheck`, `lawCheck` | built and validated by differential (evidence, not proof) |
| A produced term proves its statement | none | **imported from Lean**: the kernel's judgement of every declaration, never claimed by LexLean |
| Coverage, latency, sizes | none | **measured** (`evidence.json`), never asserted |

The stage-0 producer (`crates/lexlean/src/backend/kernel/select.rs`) decides every case analysis,
induction, citation, and law premise through the same three checks. A refusal the Rust producer
reports is therefore one the model proves it refuses with the same `Refusal`.

What is not proved, and why:

- **Statement lowering.** Lowering the IR to the statement term stays in Rust. Modelling it means
  stating the whole semantic IR (some 40 term forms, 20 types, and the instance table) as LexLean
  data. Binding is proved of the term the producer is handed. That this term is the statement as
  written is checked per theorem by the kernel (the `bound` declaration), not proved for all.
- **An independent characterization of well-formedness.** `checkFrom` *is* the definition of a
  well-formed case analysis. No theorem restates it in other words, such as "every constructor
  has exactly one branch". That needs list induction whose branches close by arithmetic over
  counts, which the phase-0 and `simplify` forms do not reach without auxiliary lemmas. It was not
  attempted, so this claim is absent rather than weakened.
- **The term builders are not production roots.** Building a `casesOn` application (motive, field
  lambdas) is the stage-0 producer's. Only the decisions are realized from the model and held to
  it. The built terms are judged by the kernel on every theorem.
- **The shipped crate still runs the stage-0 Rust.** SPEC.md §17.23 keeps the Rust a component
  replaces as the stage-0 oracle until the self-hosting fixed point, and adds a component only
  with a differential against it. The certified packages are therefore published and held to the
  Rust, not linked in place of it.

## Census

`cargo xtask proof-census` links every project and counts proof forms from the IR. `docs/kernel-terms/census.json`
records the result. It covers the repository's 29 linkable projects and two sibling projects, forge `app` and
`prism`, whose Lean pin is set to 4.32.2 in a copy, since linking does not read it.

That makes 1472 written semantic theorems, in 1786 proof nodes counting nested branches:

| Form | Whole proofs | All nodes |
|---|---:|---:|
| `reflexivity` | 610 | 654 |
| `term_cases` | 285 | 288 |
| `linear_arithmetic` | 261 | 270 |
| `decide` | 175 | 276 |
| `induction` | 54 | 54 |
| `simplify` | 27 | 151 |
| `apply` | 24 | 28 |
| `cases` | 24 | 53 |
| `region_check` | 4 | 4 |
| `law` | 3 | 3 |
| `constructor` | 2 | 2 |
| `boolean_reflection`, `congruence`, `nested_induction` | 1 each | 1 each |

The six phase-0 forms account for:

- 890 of 1472 whole proofs (60.5%);
- 1068 of 1786 nodes (59.8%);
- 832 theorems whose every node is a phase-0 form (56.5%).

Prose and core modules hold 16,462 more theorems with no semantic proof form, most of them UOR Atlas and the
preservation library.

The counts are much lower than the rough figures that motivated this work, such as 3514 `linear_arithmetic`. The
census counts each project once from its linked IR. It does not count the many local copies of LexLean and foundry
checkouts beside this one.

## Measured

See `evidence.json`, written by `summarize.mjs` from `run.sh` over each project.

The run covered 21 projects: 19 in the repository, forge `app`, and `prism`. That is every linkable semantic project
but the package library, which has 1 theorem.

| | |
|---|---|
| theorems with a semantic proof | 1471 |
| whole proof in a phase-0 form | 889 |
| produced | 745: 83.8% of the phase-0 theorems, 50.6% of all |
| accepted by Lean's kernel, natively (4.32.1, whole file) and as WebAssembly (each theorem's part over the resident library), with each binding | 745 of 745 |
| rests on exactly the declared axioms | 745 of 745 |
| per-theorem check in the WebAssembly kernel, library resident | median 6.8 ms, p90 385 ms, worst 10.4 s |
| production of a whole project, including parsing its library export | 21 ms (language-1.2) to 5.0 s (compiler, an 81 MB export) |
| kernel, gzip | 420 KB (2.24 MB raw) |
| library export, gzip | 0.16 to 0.18 MB for a project that imports no runtime module; 2.8 to 4.5 MB with the runtime; 12.1 MB for the compiler |
| produced suffix, gzip | 0.25 to 19.4 KB per project |
| loading the library once in the tab | 84 ms to 5.2 s; the compiler's library holds an 877 MB heap |

The produced share by form:

| Form | Produced |
|---|---|
| `reflexivity` | 557 of 609 |
| `decide` | 164 of 175 |
| `cases` | 19 of 24 |
| `apply` | 2 of 24 |
| `law` | 2 of 3 |
| `induction` | 1 of 54 |

The slow tail is closed evaluation inside the kernel, not proof size.

- The worst theorem, 10.4 s, is the compiler's `ReasoningFixtures.reasoningPlan5Outcome`, which runs a reasoner to its
  answer.
- Five more reasoning fixtures take 1.4 to 1.5 s, and the models project's network evaluations 0.6 to 0.9 s.
- Outside the compiler and models, every theorem checks in under 155 ms.

Why the other 144 phase-0 theorems were refused:

| Count | Reason |
|---:|---|
| 58 | a branch is closed by a form outside phase 0, `simplify` above all |
| 23 | `apply` or a law premise cites a theorem that is not produced |
| 28 | a type or term outside the vocabulary: contract and reasoning types, byte strings and literals, instance values, widening |
| 12 | a call to a declaration that is not a definition |
| 10 | case analysis on a type outside the shapes: mutual groups, value parameters |
| 7 | decidability of a construct without an instance |
| 6 | other instance or primitive gaps |

Planted faults were run through the browser path: planted in the IR, produced over the library export, judged by the
WebAssembly kernel with the library resident.

- The false decide is refused by the kernel in all 10 projects that have one.
- The altered statement is refused by the kernel at the binding in 7 of 8 projects; forge `app` is the
  definitional-equality case above.
- The omitted case is refused by the producer in all 3 projects that have one.
- No produced corpus theorem has two branches with different proofs, so the swapped branch is planted only in
  `LN-13`'s fixture. The kernel refuses it there, through `KernelCheck.lean`.

Native Lean (the 4.32.1 checker over the whole file, library included) accepted every project, 744 to 9787
declarations. The WebAssembly kernel accepted every whole file as well. The compiler's whole check took 356 s in the
tab kernel.

## Phase 1: `term_cases`

Lean proves `term_cases` with `cases h : t <;> simp only [defs, h]`, which rewrites `t` after unfolding `defs`. The
producer here does not rewrite. It enumerates instead: every local of a finite type (`Bool`, or a document type whose
constructors carry no field) is split with `casesOn`, and each case is closed by `Eq.refl` or `decide`. Once those
locals are constructors, the kernel computes the split term and the statement itself (`LN-14`; the LexLean model and
its proofs are `BS-39`).

On the compiler project, judged by Lean's kernel through `KernelCheck.lean`, enumeration is accepted for 84 of the 285
`term_cases` theorems: all of `RuntimeKeysBool` and `RuntimeSupport`, and parts of `RustStd` and `RustRuntime`. The
other 201 split on a comparison of numbers (the numeric `RuntimeKeys*` modules), which no finite split fixes. They need
the split term abstracted from the unfolded statement and rewritten with `Eq.mpr`/`congrArg`. A first attempt (branch
`claude/kernel-terms-s2-wip`) reached 65 acceptances with 188 definitional mismatches, so a faithful unfolding engine
is the next step, shared with `simplify`. `linear_arithmetic` (261 theorems) needs that engine and a certificate
producer and is not started.

## Not done

- **Definitions and types still come from Lean.** The library export holds the project's definitions and inductive
  types as Lean elaborated them. The kernel checks them, and nothing beyond that pins them to the IR.
  foundry's `emit.mjs` closes this by stating each definition's equations and each type's constructors. Porting
  that into the producer is the step that makes a project with no Lean run anywhere fully tab-native.
- **Phase-0 `induction` and `cases` mostly cannot close their branches.** In the corpus, 53 of 54 `induction`
  proofs close branches with `simplify` (phase 1), so they are refused. The forms themselves are produced and
  kernel-accepted, including `generalizing`.
- **`apply` and `law` need their cited theorems produced.** A citation of a `linear_arithmetic` or `simplify`
  theorem is refused. Phases 1 and 2 lift this.
- **Vocabulary gaps.** These are refused by name: fixed-width multiplication, division and widening; checked
  operations; contract-violation and reasoning-failure types; instance values; byte literals; and document types
  without `DecidableEq` under `decide`.
- **Axiom exactness.** A produced proof can rest on fewer axioms than the policy Lean's tactic proof needed.
  `summarize.mjs` reports such theorems rather than calling them verified.
- **The compiler project's library is too large for a phone.** It is 12 MB gzip and holds 877 MB of heap, because
  every runtime target module is in its closure. Trimming the library to what the produced theorems reach (phase 3)
  is the remedy; a project that imports no runtime module needs 0.16 MB.
- **A part check rechecks nothing it shares with another part.** Each theorem's part restates the terms it needs
  over the library. A tab that judges many edits keeps the parts it has accepted resident; that is not built.
- **The WebAssembly kernel is 4.32.1 and the library is exported from 4.32.2.** Every check here judges the whole
  file, library included, so the pairing is checked rather than assumed. Upgrading the tab kernel to the pinned
  4.32.2 (or 4.33.1, which fixes leanprover/lean4#14576) is separate work.
- **Not measured on a phone or in a browser engine.** The latencies are Node's V8 on the desktop CPU. foundry's
  phone-size emulation ran 1.5 to 1.7 times slower.
- **`import Lean` trimming (phase 3) is not started.**

# Fold-law provenance

`LexLeanRuntime.Laws` (SPEC.md §17.12, *Fold laws*) is the fifth native module
of the runtime project `language/runtime-1.2/library/`: the sums, bridges, and
fold laws that `linear_arithmetic` rewrites with and a `law` proof applies
(#117). Like the four modules migrated before it
(`tests/runtime-library/MIGRATION.md`), it is LexLean source, schema
`lexlean/core-module/2` in the core notation (§17.21.8). This record is the
evidence that the source states what was proved, and that the Lean the
compiler generates from it is the same environment.

## The probe

Run on `main` at `32afe5da` with the pinned Lean 4.32.1, before this change.

- **An integer kernel.** `clampStep x = clamp(((x + 20) · 9365133 + 2³¹) quot 2³²,
  0, 255)` and its monotonicity, written with `less_than`, proved by
  `linear_arithmetic{definitions: [clampStep]}`: `omega could not prove the
  goal: No usable constraints found`. The script did not unfold `less_than`,
  `quotient`, or `negate`, and nothing split the clamp's `if`s.
- **Two folds.** `doubledTotal xs = total xs + total xs`, where `total` folds
  `acc + v` and `doubledTotal` folds `(acc + v) + v`: `omega` reads the two
  folds as unrelated atoms. The missing lemma is a normal form for additive
  folds: a fold whose step adds what each element contributes is its
  initial value plus the sum of the contributions, and the sum is linear in
  the contribution. Once both folds are in that form, the equality is linear
  over one atom, the sum of the identity.

## What the module states

120 declarations: the definitions `sumNat` and `sumInt` with what Lean
generates for them, the theorems the registry names
(`language/runtime-1.2/laws.toml`: 55 rewrites and 3 applied laws),
`ite_eq_iff`, which the second attempt of `linear_arithmetic` adds (PF-26,
below), the general
`sumNat_perm` and `sumInt_perm` that the reverse laws are proved from, and the
auxiliary rows Lean realized while proving them: equation lemmas, matchers,
splitters, and private proof terms. That includes equation lemmas of
`LexLeanCollections.lessThan`, `listFold`, and
`LexLeanRuntime.quotient`/`remainder`, which Lean realizes in the module that
asks for them. Every row pins its exact axioms. No row depends on
`Classical.choice`: the bridges are proved by case analysis on `compare` and
by rewriting `isZero`, never by `by_cases` or `simp_all`, so a theorem
proved by `linear_arithmetic` inherits at most `propext` and `Quot.sound` from
the laws it uses.

## How

The one-time oracle is in the checkpoint commit that adds this record
(`b58f1952`, `tests/runtime-library/laws-oracle/`); the commit after it
removes it. Its
exporter, `flags.lean`, `canon.py`, and `compare.py` are those of the runtime
migration (`MIGRATION.md`), with the module list narrowed to `Laws`.
`run.sh <repository> <work>`:

1. `lexlean build` of the runtime project generates the Lean of `Portable` and
   `Collections`. `Laws.lean` is compiled against them with the pinned Lean,
   silently.
2. `flags.lean` and `export.lean` export the `Laws` environment, and `canon.py`
   writes its data form. `lexlean fmt` prints that in the notation, and the
   result equals the committed `language/runtime-1.2/library/src/Laws.lex.tex`
   byte for byte.
3. The Lean `lexlean build` generates from the committed source is compiled,
   exported, and converted by the same programs, and `compare.py` compares the
   two declaration by declaration, as trees of kernel terms.

## Equivalence

Result: the same 119 declarations in the same order. 118 rows are equal in
every byte of their terms. One row, `lessThan.match_1.splitter._sparseCasesOn_2`
(a sparse `casesOn`), is equal up to the names of bound variables, which the
elaborator's construction makes hygienically and no kernel judgment sees. This
is the same difference the migration found for the sparse `casesOn` rows of
`Collections`. The only other difference is the private `Lean` import the
decoder adds to a regenerated module, which SPEC.md §17.10 allows.

## Verification

- Every verification of a project that imports `Laws` (every language-1.2
  module with a `linear_arithmetic` or `law` proof) compiles it, replays it
  with `leanchecker`, and audits each declaration against its row's exact
  axioms. `conformance_rt_03` verifies the runtime project itself, `Laws`
  included.
- `conformance_pf_22` holds the module native with its imports, the
  registry's names declared in it, and every row within `propext` and
  `Quot.sound`. It refuses a registry that names a missing declaration or a
  law twice, and a laws row that only bounds its axioms. It also verifies the
  fold equalities of `examples/fold-laws` for all inputs and has Lean refuse
  three false ones.
- The planted defects are recorded in `VERIFICATION.md` ("the fold laws can
  fail").

## Identity

Languages 1.0 and 1.1 are byte-identical. Every file this change adds or
changes lies in the language-1.2 partition of the compiler-semantics ID
(`language/runtime-1.2/`, `language/semantics-1.2.toml`, the semantic-module
and snapshot schemas of version 2). Every language-1.2 lock (the
compiler-semantics ID substituted), golden, fixture expectation, calculus
package, production export, and verified record was regenerated by the
repository's writers.

## The equation of a conditional (PF-26)

`ite_eq_iff` states `((if p then a else b) = c) ↔ (p ∧ a = c) ∨ (¬p ∧ b = c)`
for a decidable `p`, proved by `by_cases` on the instance and `simp only`; its
row pins `[propext]`.

**Why.** The fixed script splits every `if` with `repeat' split`. On a
first-match chain of more than about ten conditions (a state machine's
transition table, a policy's rules) the simplifier inside `split` exhausts its
step bound and `omega` fails. Read through `ite_eq_iff`, with `and_false,
false_and, or_false, false_or, not_false_eq_true, not_true_eq_false` dropping
each branch whose condition is decided, the chain becomes cases `omega`
decides directly.

**Why a second attempt.** Added to the rewrite set, the law would also rewrite
a goal of the form `if … = c`, such as `(a + x) % 2` read through
`remainder_nat`, into a disjunction, and `omega` refutes the negation of a
disjunction with `Classical.choice`: `FoldLaws.Main.parity_step` would observe
`[Classical.choice, Quot.sound, propext]` against its pinned `[Quot.sound,
propext]`. So the law is not registered as a rewrite. `linear_arithmetic`
lowers to `first | (fixed script; done) | (fixed script with the law and the
simplifications)`, and a proof the fixed script closes keeps its proof term
and its axioms.

**Export.** The oracle of #117 (`tests/runtime-library/laws-oracle/`) was
restored with the law appended to `Laws.lean`, run once to write the module,
and removed in the commit after. The check run compiled the committed
source's generated Lean, exported it, and compared: 120 declarations, 120
equal in every byte of their terms, and the decoder's private `Lean` import
the only other difference (SPEC.md §17.10).

## Natural-number arithmetic (PF-28)

**Why.** `linear_arithmetic` unfolds `LexLeanRuntime.multiply` to `*`, and
`omega` reads `a * b` of two variables as an opaque atom: it cannot relate
`m * (2 * p)` to `2 * (m * p)`. Hologram AI's stages compute sizes as
`mantissa * powTwo scale` and its kernels shift by variable amounts, so the
equality of a kernel and its stage for every input needs the ring, order,
and division laws. No nonlinear decision procedure is added: every step is a
law and `omega`.

**What.** 23 theorems in `LexLeanLaws`, over `Nat` and the runtime's
`multiply`, `quotient`, `remainder`, and `subtract` exactly as a document
writes them, so each matches without unfolding (SPEC.md §17.12, *Fold laws*,
rule 7), and two helpers, `quotient_nat_pos` and `remainder_nat_pos`, which
read a quotient and remainder by a positive divisor as `/` and `%` by `rfl`
on the divisor's successor case. Each quotient and remainder law takes a
positive divisor, so it holds for every zero case; a zero case on a law's
right side is one of its left side's, so that `simp` instantiates it. The
cancellation `nat_le_of_mul_le_mul_left` names a factor its conclusion does
not contain, which `simp` cannot instantiate, so `nat_mul_le_mul_left_iff`
states it as a rewrite of the product order. Rows pin `[]` or `[propext]`:
`propext` comes from core's proofs of `Nat.mul_assoc`, `Nat.mul_sub` and the
division lemmas. Core's `Nat.mod_mul_right_div_self` depends on
`Classical.choice`, so `nat_mod_mul_left_div_self` is proved from
`Nat.mod_add_div`, `Nat.add_mul_div_left`, `Nat.add_mul_mod_self_left`, and
`Nat.mod_eq_of_lt` instead.

**Export.** The oracle of #117 was restored outside the repository and run
with the pinned Lean 4.32.2. The authored module was the Lean `lexlean build`
generates from the committed source (the decoder that adds its 120
declarations) with the new theorems appended, so every committed row is
re-added from its own data. It was exported, its header imports set to the
committed `Init`, `Portable` and `Collections` (the decoder's private imports
are the generated Lean's, SPEC.md §17.10), and printed in the notation by
`lexlean::sdk::to_notation` and `lexlean fmt`. The check run compiled the new
source's generated Lean, exported it, and compared: 145 declarations, all
equal in every byte of their terms. A second comparison against the export of
the committed source found its 120 declarations present, in their order, and
equal in every field and term; the 25 added rows are the 23 laws and the two
helpers, and Lean realized no auxiliary row for them.


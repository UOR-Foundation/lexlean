# Application building blocks

`std-apps` is a source package of proved blocks that an application cites by
name instead of proving list and state facts again. SPEC.md §17.22
(*Application building blocks*) is the contract; `SL-15`, `SL-16` and `EX-10`
are its register rows. This page says how to use it.

## Why it exists

An agent-written cart had to invent a sum, a count and a remove-first, then
prove that removal conserves the sum. Seven of its theorems failed. The
failures came from the proof steps themselves, not from the agent:

- `linear_arithmetic` unfolds definitions but cannot cite a theorem.
- `simplify` cannot use the premise of the theorem it proves.

A list lemma needs both, so an application cannot prove one in a single step.
The package proves each such lemma once, with helper lemmas an application
never sees. Each lemma is stated in a shape that one application step can
use.

## What is in it

The blocks follow the state shapes in `survey.json`, measured over forge,
verdant-engine, the cart draft and this repository's examples.

| Module | Use it for |
| --- | --- |
| `Lists` | `List Nat` items: counts, appends, remove-first |
| `Lines` | priced lines `(item, amount)`: a total that a removal conserves |
| `Tables` | a map keyed by `Nat`, where a later write shadows an earlier one |
| `Ledger` | balances: deposits, and transfers that conserve what they move |
| `Counter` | caps, quotas, retries, and shortfall or excess against a target |
| `Machine` | transitions guarded by a fault count, whose runs keep that count |
| `Access` | rank and owner rules, and a guard that leaves a denied state unchanged |

## How an application cites a block

1. Import the module (`\importmodule{LexLeanStdApps.Lines}`) and declare the
   package in `lexlean.toml` with the version in `std-apps/pins.json`.
2. Write each transition as a thin wrapper over a block. Pass any guard as a
   `Bool` argument; don't wrap the block in your own `if`:

   ```text
   afterRemove state item = record State {lines := (LexLeanStdApps.Lines::removeIf (canRemove state item) item (project state lines))}
   ```

3. State the theorem as an instance of the lemma, and prove it with one
   `apply`:

   ```text
   statement: (((canRemove state item) = false) -> ((totalOf (afterRemove state item)) = (totalOf state))),
   proof: apply{arguments: [(canRemove state item), item, (project state lines)], theorem: LexLeanStdApps.Lines::total_removeIf_false}
   ```

   `apply` is one `exact`, so it holds whenever unfolding your own
   definitions turns your statement into the lemma's.
4. Prove guard and reply facts over your own definitions with
   `linear_arithmetic`. List the guard (`canRemove`) among the definitions, so
   that its comparison becomes arithmetic `omega` can use.

`examples/cart` is the worked case. It has 12 theorems, each one step. The
draft it replaces had 4 recursive definitions, 2 helper lemmas and 2
inductions. The cart reaches 17 declarations of `Lines` (155 lines), and the
application no longer writes any of them. `cart.json` records these counts.

## Stability: pins and versions

Every declaration of the package is pinned in `std-apps/pins.json`:

- a definition by its alpha identity, which renaming its binders leaves
  unchanged;
- every other declaration by the SHA-256 of its canonical linked form without
  its proof.

A changed definition, statement, parameter or axiom policy fails `SL-16` and
names the declaration. A changed proof does not.

To change a pinned declaration on purpose:

1. Raise `version` in the pins file and in every importer.
2. Record the change in `CHANGELOG.md`.
3. Commit the pins that `SL-16` prints.

The pins file sits outside the package's source roots. Announcing a change
does not move any importer's lock, which pins the bytes (SPEC.md §10.5).

## How prism-code's proof store reuses these proofs

Each lemma is verified once under a fixed Lean kernel. Its identity does not
depend on which application cites it. That is what lets a proof store share
it across applications. In prism-code (prismpm#715), a proof record is
`prism/kernel-proof/2`. Its key is `proofKey({kernel, library, statement,
policy})`. For a block lemma, each part of the key comes from LexLean:

| Key part | Where it comes from |
| --- | --- |
| `kernel` | the pinned toolchain, `leanprover/lean4:v4.32.2` |
| `library` | the package's `tree_sha256` in the importer's `lexlean.lock` (`[[source_package]]`, SPEC.md §10.5): the exact bytes of the blocks |
| `statement` | the lemma's `statement_id` in `std-apps/pins.json`, or a definition's `alpha_id` |
| `policy` | the lemma's exact axiom policy, part of the same pinned identity |

Two applications that cite `LexLeanStdApps.Lines::total_removeIf` from the
same package bytes therefore look up the same record. Under prism-code's rule,
a stored proof stands in for producing a term, never for judging it. On reuse
the kernel still re-checks the term, at about 20 to 70 ms per lemma once the
kernel is open, against the minutes it takes to produce it.

Shared Lean modules are the other half (prismpm#694, the content-addressed
store). Every importer generates the package's modules under one prefix,
`LexLeanStdApps`, so the same Lean modules can be shared across apps instead
of being regenerated per app.

What LexLean guarantees ends at the identities. That the store keys on
exactly these four parts is prism-code's design, and prism-code checks it.

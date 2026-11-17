# cart

A shopping cart that an agent drafted with lists of its own, rewritten on the
`std-apps` building blocks. Priced lines are added and removed under guards,
and each request gets a reply. Every theorem is one step: an `apply` of a
`Lines` lemma, a `linear_arithmetic` over the cart's own guards and replies,
or reflexivity. The cart defines no recursion of its own.

- **Language:** 1.2. **Source modules:** 1 under `src/`, plus the package's
  `LexLeanStdApps.Lines`.
- **Register rows whose gates run on it** (`model/ids.toml`, level `build`):
  - `EX-10`;
  - `SL-15` and `SL-16`, whose plants break the lemmas it cites;
  - `EX-08` (every example is discovered and must pass the full example gate).
- **What is claimed.** `cargo xtask verify-examples` runs the project through
  every stage and compares the results with `expected/` byte for byte
  (SPEC.md §28.6). The stages are formatting, locking, checking, building, and
  verifying under the pinned `leanprover/lean4:v4.32.2`: elaboration,
  `leanchecker` replay, and an exact axiom audit.
- **What it no longer writes.** `docs/app-blocks/cart.json` compares the cart
  with the draft it replaces.
  - The draft had 9 theorems in 214 lines (236 in canonical form); Lean
    rejected 4 of them with 7 errors. It defined 4 recursive functions, 2 helper lemmas,
    and 2 inductions of its own.
  - The cart has 12 one-step theorems in 210 canonical lines, and defines none
    of those.
  - The 7 lemmas it cites reach 17 declarations (181 lines) of
    `LexLeanStdApps.Lines`, proved once in the package.
- **What is not claimed.** The draft's price table and answer codes are kept as
  they were. A line records its price when it is added, so the conservation
  theorem speaks of the amount the removed line carries. Nothing is claimed
  about prices that change after a line is added.
- **Run.** From this directory, with the `lexlean` binary and the pinned
  toolchain installed, run `lexlean check` and `lexlean verify`.

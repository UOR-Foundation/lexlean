# reflect-library

The lemma library behind the in-tab kernel path (UOR-Foundation/lexlean#584): reflective integer arithmetic
and fold fusion, as one native core module (SPEC §17.10).

A proof found in a browser tab is a short term that applies one of these lemmas to a certificate. Lean's kernel
judges that term; these lemmas are why it is sound. They are therefore part of what is trusted, and belong here,
verified by LexLean itself, not beside the prover as handwritten Lean.

## What is in it

`src/Reflect.lex.tex`: 510 kernel declarations in authoring notation, namespace `LexReflect`.

- A checker for refutations of integer inequalities with conditionals, and `soundT`, its soundness theorem.
- Bridging lemmas from the runtime's operations (comparisons, truncating division and remainder, naturals,
  bytes, pairs) to that checker's language.
- Fold fusion: `fold_rel1`, `fold_rel2`, `fold_rel3`, their length-indexed forms, and `fold_append`.

Every declaration is on `propext` and `Quot.sound` at most. Nothing uses `Classical.choice`.

## Checked

With Lean 4.32.2 on Linux:

```
lexlean lock && lexlean fmt --check && lexlean check && lexlean verify
```

`fmt --check` reports the module canonical, and `verify` reports `verified 1 module`. The prover's resident
library, exported from this verified build, proves the `fold-laws` example as before: 31 of 31, 29 found in
place on the declared axioms, 2 by Lean's own proof.

## Not done

- **Not registered.** No register row, scenario, conformance case or `expected/` oracle, and the package is not
  a workspace member. R3 asks for those first; this is the artifact, offered ahead of them.
- **Not authored in LexLean.** The module was produced from a Lean development with the exporter of
  `tests/preservation-library/oracle` and `sdk::to_notation`. It is kernel declarations as data: it can be
  verified here, but not comfortably edited here. Changing a lemma means changing the Lean development and
  converting again, and that development is not in this repository.
- **One decoder limit met.** A structure declared in the same module as recursive functions over it failed in
  `verify` ("was not compiled; `compileDecls` must run on inductive types first"). The library avoids the
  pattern with a pair; the limit itself is not fixed.

# rs2lex

A translator from the compiler's own Rust (`crates/lexlean/src`) to language-1.2 source, for the bootstrap of
SPEC.md §17.23 (#269). It reads a crate, translates every function it can into one `.lex.tex` module, and reports
for each function that does not translate the construct that stopped it.

The translation is a tool, not the bootstrap. A module it makes becomes bootstrap source the way
`bootstrap/src/AxiomPolicy.lex.tex` (`BS-74`) did. The module is cut out with `extract.py`, committed, and from
then on held to the Rust it came from by its own conformance case, like any hand-written member. Nothing here
runs in the gates. What this directory claims is what its two checks below show, run by hand.

## Use

Each script takes the compiler binary from `LEXLEAN`, which defaults to `target/release/lexlean`. Python 3.10 or
later is needed. The scripts run from the repository root.

```sh
python tooling/rs2lex/translate.py                      # crates/lexlean/src -> target/rs2lex/
bash   tooling/rs2lex/check.sh                          # lexlean lock and check on target/rs2lex/Crate.lex.tex
bash   tooling/rs2lex/probe.sh                          # the probe: translate, build, run under Lean, compare
python tooling/rs2lex/extract.py target/rs2lex/Crate.lex.tex OUT.lex.tex MODULE old=new[:effect,...] ...
```

`translate.py` writes `Crate.lex.tex`, `report.json` (one row per function: its file, its bytes, and `ok` or the
reason), and `summary.txt` (the totals, and each root cause with everything it blocks through its callers).

`check.sh` makes a workspace (`workspace.sh`): the project `mech` holds the translation and the two shim modules
it imports (`shim/`), beside a copy of `stdlib/`. It then runs `lexlean lock` and `lexlean check` with the
repository's compiler.

`probe.sh` translates `probe/lib.rs`, a small crate written to exercise one rule of the translation per
function. It builds the result and the runtime (`language/runtime-1.2/library`) with the repository's compiler,
runs it as a Lean program, and compares every line with what Rust's semantics give (`probe/cases.py` writes the
inputs and the expected lines). A rule is trusted only once a probe function exercises it. The probe has caught
five silent miscompiles, each fixed or now refused:

- a recursive call dropped from a specialized function;
- a format string's implicit capture broken by an alias substitution;
- a shadowed variable's value leaving its scope as the outer variable's (`unshadow.py` now gives every binding
  its own name);
- a named closure's writes to what it captures, lost at each call;
- a write through a reference bound by a pattern, lost (refused).

## What it translates

The translation is a continuation-passing compiler from a Rust AST to the semantic notation (`rs2lex_core.py`,
with the rules for the standard library and the crate's own types in `rs2lex.py`):

| Rust | language 1.2 |
| --- | --- |
| a loop, `?`, early `return` | a fold or a bounded iteration whose state carries the exit |
| recursion | a definition on fuel (`u64::MAX`), in one mutual group per strongly connected component |
| `&mut` parameters | returned beside the result and rebound at each call |
| `Cell`, `RefCell` | the value, the parameter threaded as `&mut` (`interior.py`); a local alias of a place in a parameter is that place |
| `HashMap`, `BTreeMap` with `Bytes`, `Nat`, or `Nat`-tuple keys | `LexLeanStd.OrdMap` (`SL-15`), the key written as order-preserving bytes |
| other maps | an ascending list of pairs |
| a closure passed as `FnMut` | the callee copied once per closure, the closure's captures its parameters |
| a closure with effects (it writes what it captures, or returns early) in a terminal adapter (`any`, `all`, `find`, `find_map`, `fold`, `for_each`, `try_for_each`), in `filter`/`filter_map` before `collect`, or in an `Option`/`Result` combinator | the loop or the `match` it stands for, the closure bound to a name and called |
| a closure bound to a name | inlined at each call; one that writes a captured variable reads its current value and writes back |
| shadowed names | renamed apart before translation (`unshadow.py`) |
| `char` | its UTF-8 bytes; `chars`, `char_indices`, `is_whitespace` (`AxiomOutput::whiteLength`), the `is_ascii_*` tests |
| slice patterns | list constructor patterns, or a length guard where a rest precedes other elements |
| derived `Ord` and `PartialEq` | lexicographic comparison and equality, only where the type derives them |

It refuses rather than guess. Examples:

- a subtraction on an integer whose type was only defaulted: Rust would make it an `i32`, and here it would be a
  `Nat` that stops at zero;
- iteration over a hash map, whose order would matter;
- a write through a reference bound by a pattern or a `let` (`if let Some(x) = y.as_mut()`, `for x in
  v.iter_mut()`): the name would be a copy, and the write would not reach the value it refers to;
- a function value bound by `let`, which the language refuses;
- a struct ordered by an `Ord` it does not derive;
- a closure that escapes into a data structure;
- a trait object other than `dyn std::error::Error`, which is the message it displays.

## Where it stands

On `release/v0.4` with `SL-15`, `translate.py` reports 1,473 of 3,616 functions, 15.8 % of the bytes. The
repository's compiler (0.4.5) accepts the whole module: `check.sh` passes. Every probe function translates, and
all of the probe's 1,800 cases match Rust's. Over the same rules, a separate harness held the translated functions
that take and return plain data to the real crate, called through its own tests: 92 functions, identical on
12,577 cases. That harness modifies a copy of the crate and is not part of this directory.

`summary.txt` ranks what stops the rest. Most of it is code that calls something which does not translate. The
largest causes of their own are:

- `serde_json`'s `json!` and `Value`, which need a mapping onto the standard library's `Json` (which has no `null`
  and no floating-point numbers);
- the paths and file system of the effects boundary (#271, #426);
- trait objects other than errors.

## Files

| File | |
| --- | --- |
| `rs2lex_core.py` | the parser, the type store, and the translation of expressions, patterns, and functions |
| `rs2lex.py` | the rules for the standard library, the crate's types, and the module's declaration order |
| `interior.py` | `Cell`/`RefCell` as threaded state |
| `unshadow.py` | every local binding on its own name; the check for writes through bound references |
| `crate_load.py` | reading a crate: items, `use` resolution, type aliases scoped to their module, derives |
| `translate.py` | the whole crate to one module, with the report |
| `extract.py` | one module cut out of a translation, with its axioms stated |
| `workspace.sh`, `check.sh`, `probe.sh` | the workspace, the check, and the probe |
| `shim/` | `RustStd` and `AxiomOutput`, the modules the translation imports beside the standard library |
| `probe/` | the probe crate and its cases |

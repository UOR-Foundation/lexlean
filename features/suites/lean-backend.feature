Feature: lean-backend

  Prose-free deterministic Lean generation (§18).

  @LN-01 @build
  Scenario: Each generated Lean file has the exact module, import, option, namespace, declaration, and end structure.
    Given the nat-add-zero example project rendered without Lean or publication
    When the generated `Main` Lean text is scanned in order
    Then `module` opens the file as its first line
    And `import Init`, `set_option autoImplicit false`, `namespace LexLeanExample.Main`, `theorem add_zero`, and `end LexLeanExample.Main` occur in that order

  @LN-02 @build
  Scenario: Imports are explicit, deduplicated, sorted, and every external global is fully qualified.
    Given the nat-add-zero example project rendered to Lean
    When the `import` lines of the generated `Main` module are collected
    Then the import list equals its sorted and deduplicated form
    And the external global appears fully qualified as `Nat.add`
    And no `open` statement occurs anywhere in the file

  @LN-03 @build
  Scenario: Generated Lean contains no comments, documentation, prose-bearing strings, or copied source prose.
    Given the generated Lean for the nat-add-zero example and for the test.defs definitions fixture
    When each Lean text is searched for comment and string markers
    Then neither text contains `--`, `/-`, or a double quote
    And neither text contains the copied source phrase `natural number`

  @LN-04 @build
  Scenario: Generated Lean contains no sorry, admit, axiom, opaque, unsafe, native_decide, or placeholder declaration.
    Given the generated Lean for the nat-add-zero example and for the test.defs definitions fixture
    When each Lean text is searched for the six forbidden declaration spellings
    Then no sorry-, admit-, axiom-, opaque-, unsafe-, or native-decide-form token appears in either file

  @LN-05 @build
  Scenario: Every linked term and proof variant has one defined Lean lowering and missing lowering is a hard error.
    Given ten one- and two-theorem modules exercising assume, witness, left alternative, have, rewrite, simplify, constructor, induction, calculate, and apply proofs
    When each module is written to `src/Main.lex.tex`, checked, and rendered to Lean
    Then every module checks successfully
    And the concatenated Lean corpus contains `intro`, `exact`, `apply`, `rfl`, `refine`, `left`, `have`, `rw [`, `simp only [`, `constructor`, `induction`, and `calc`

  @LN-06 @build
  Scenario: Leading universal binders become deterministic declaration parameters with complete source mapping.
    Given the nat-add-zero example project rendered to Lean
    When the first module's Lean text is inspected for the peeled leading universal
    Then the declaration carries the parameter `(llv0 : Nat)`
    And no residual universal quantifier symbol remains in the text
    And the byte offset of `llv0` lies inside one Lean coverage row

  @LN-07 @build
  Scenario: All document definitions emit def and never alternate declaration forms.
    Given the test.defs definitions fixture with a type, a term, and a predicate definition
    When the fixture is rendered to Lean
    Then the `Main` module contains `def count`, `def double`, and `def good`
    And it contains none of `abbrev`, `instance `, `structure `, or `inductive `

  @LN-08 @build
  Scenario: Proof lowering uses only the fixed pinned Lean forms enumerated by the specification.
    Given the nat-add-zero example project rendered to Lean
    When the tactic block after `:= by` and before `end` is split into whitespace tokens
    Then every token is `rfl` or begins with `llv` or `llh`

  @LN-09 @build
  Scenario: Lean formatting is byte-deterministic with fixed indentation, LF, and final LF.
    Given the nat-add-zero example project
    When the `Main` module is rendered to Lean twice in one process
    Then both renderings are byte-identical
    And the text ends with a single final LF and contains no CR or tab
    And every line's leading space count is a multiple of two

  @LN-10 @build
  Scenario: Every non-whitespace generated Lean token has a source, glossary, IR, or synthetic-core mapping.
    Given the nat-add-zero example project rendered to Lean with its coverage rows
    When every non-whitespace byte of the `Main` Lean text is checked against the Lean coverage rows
    Then each such byte lies inside exactly one coverage row

  @LN-11 @build
  Scenario: The generated-source audit tokenizes and rejects prose-bearing or forbidden Lean tokens before verification.
    Given the shared verified run of the nat-add-zero example
    When the published build root is inspected after verification
    Then an `audit` directory exists inside the build root
    And the committed `tests/golden/axiom-parser/rejected.txt` corpus is nonempty

  @LN-12 @build
  Scenario: Generated file paths and module names exactly mirror the configured module prefix and source module.
    Given the nat-add-zero example project with `module_prefix = "LexLeanExample"`
    When the project is rendered, then relocked and rendered again with `module_prefix = "Other"`
    Then the first build's module is `LexLeanExample.Main` at `modules/LexLeanExample/Main.lean` with `namespace LexLeanExample.Main`
    And the second build's module path is `modules/Other/Main.lean` with `namespace Other.Main`

  @LN-13 @build
  Scenario: Kernel-term production writes, with no elaborator and no tactic, a lean4export 3.1.0 theorem for every semantic theorem whose proof tree is reflexivity, decide, cases, induction, apply of a produced theorem, or law, typed by the statement lowered from the linked IR and bound to the theorem Lean elaborates; Lean's kernel accepts every produced theorem and binding of a module that uses each form, and refuses a produced theorem whose statement was altered, whose case takes another constructor's branch, or whose decide obligation is false, and production refuses a proof that omits a case.
    Given a language-1.2 semantic module whose theorems are proved by reflexivity, decide, cases, induction, apply, and law
    When kernel terms are produced from its linked IR with no library and judged by `crates/conformance/lean/KernelCheck.lean` over its compiled modules
    Then every produced theorem and its binding are accepted with exactly the declared axioms
    And a planted altered statement, swapped case branch, and false decide are each refused by the kernel, and an omitted case is refused by production

  @LN-14 @build
  Scenario: Kernel-term production writes a `term_cases` proof by enumeration: every local of a finite type (`Bool`, or a document type whose constructors carry no field) is split with `casesOn` in scope order, and each case is closed by `Eq.refl` for an equation and `decide` otherwise, so the split term's value is computed by the kernel and never rewritten; Lean's kernel accepts the produced theorem and its binding for a module whose `term_cases` split term is fixed by its finite locals, and refuses a produced theorem whose statement is false in one case.
    Given a language-1.2 semantic module whose `term_cases` theorem splits on a call over a Boolean and a natural-number parameter, and a copy whose statement is false when the Boolean is `true`
    When kernel terms are produced from its linked IR with no library and judged by `crates/conformance/lean/KernelCheck.lean` over its compiled modules
    Then the `term_cases` theorem and its binding are accepted with exactly the declared axioms
    And the produced theorem of the false copy is refused by the kernel

  @LN-15 @build
  Scenario: Kernel declarations write, from the linked IR with no elaborator, a project's own declarations as lean4export 3.1.0 lines: each structure and inductive type without value parameters, a mutual or nested group as one inductive declaration, whose recursors the kernel generates; each type's casesOn, defined from its group's recursor over one universe parameter; each structure field's projection; each definition, its structural recursion (mutual or nested) written through its family's recursor with a tuple motive and its well-founded recursion through fuel of its measure plus one; and each theorem's statement as the proposition LexStmt.T, with the member statements of a nested induction, every term lowered as kernel-term production lowers a statement; production then runs over them unbound, the lines are deterministic, a declaration of a kind not written (a type with value parameters, a class, or an instance) is refused with its reason together with every declaration that names it, and Lean's kernel, holding only its own library and the runtime, accepts every declaration and every produced theorem of a module with an inductive type, a structure, and structural recursion, and of a module with mutual and nested types, mutual and nested recursion, and well-founded recursion.
    Given the committed higher-order example, which declares an inductive type, a structure, and structurally recursive definitions, the committed recursion example, which declares mutual and nested types and mutual, nested, and well-founded recursion, and the committed semantic-1.1 example, which declares classes and instances
    When their declarations are written from the linked IR with no library, twice each
    Then higher-order and recursion refuse nothing, every theorem has its statement, and both runs write the same lines
    And semantic-1.1 refuses its classes and instances with a reason, and recursion with a value parameter planted on its nested type refuses that type and each declaration that names it
    And where the pinned toolchain is installed, Lean's kernel holding only its own library and the runtime accepts every declaration and produced theorem of higher-order and of recursion

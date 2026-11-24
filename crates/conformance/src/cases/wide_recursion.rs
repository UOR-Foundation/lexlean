//! A definition with more parameters than Lean's structural search considers
//! recurses in its generated Lean on the argument its source declares
//! (SPEC.md §17.12): `SP-34`.
//!
//! Lean's search tries no candidate argument once more than ten are possible
//! and then compiles the definition by well-founded recursion, which a
//! `reflexivity` proof through it cannot unfold. `wideOf` (executable) and
//! `wideModelOf` (not executable) of the fixture have eleven parameters and a
//! theorem proved by `reflexivity` each; `narrowOf` has ten, states no
//! argument, and generates the bytes it generated before the rule.

use crate::support::{self, P};

/// The fixture's generated clause for an eleven-parameter definition.
const STATED: &str = "termination_by structural _ _ _ _ _ _ _ _ _ _ fuel => fuel";

/// `SP-34`.
///
/// # Panics
///
/// When the fixture's Lean or its verification disagrees with the row.
pub fn run(id: &str) {
    assert_eq!(id, "SP-34");
    let project = P::copy_tests_project("fixtures/wide-recursion");
    project.relock();
    let lean = support::lean_text(&support::rendered(&project), "Main");
    for stated in ["wideOf", "wideModelOf"] {
        let definition = super::recursive_argument::definition(&lean, stated);
        assert!(
            definition.contains(STATED),
            "`{stated}` has eleven parameters and states its argument:\n{definition}"
        );
    }
    let narrow = super::recursive_argument::definition(&lean, "narrowOf");
    assert!(
        !narrow.contains("termination_by"),
        "`narrowOf` has ten parameters and states no argument:\n{narrow}"
    );
    // The kernel reduces each definition by `reflexivity`, which a definition
    // compiled by well-founded recursion does not allow.
    support::verify_ok_backed(id, &project);
}

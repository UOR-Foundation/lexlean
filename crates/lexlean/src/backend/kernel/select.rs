//! The producers' decisions, by the checks `bootstrap/src/KernelTerms.lex.tex`
//! states as LexLean source and proves its producers refuse exactly
//! (`BS-35` to `BS-38`): which branch is the minor premise of each
//! constructor, and whether a cited theorem or law may be cited. This is
//! the stage-0 realization (SPEC.md §17.23). `BS-35` holds it to the
//! `rust-std` packages the production pipeline renders from those roots.
//!
//! Each function is the root of the same name read over Rust data:
//! `check_from` is `checkFrom`, `member_check` is `memberCheck`, and
//! `law_check` is `lawCheck`.

/// Why a producer writes no term: the model's `Refusal`, in its order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal<N> {
    NotAnEquation,
    MissingCase(usize),
    RepeatedCase(usize),
    BinderCount(usize),
    UnknownCase(usize),
    NotProduced(N),
    UnregisteredLaw(N),
}

/// A constructor of the scrutinee's type: its fields, and how many of
/// them are of the type itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CtorShape {
    pub fields: usize,
    pub recursive: usize,
}

/// A written branch: the constructor it names, by position (a position
/// past the last constructor names none), and how many names it binds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BranchHead {
    pub ctor: usize,
    pub binders: usize,
}

fn wanted(shape: CtorShape, induct: bool) -> usize {
    if induct {
        shape.fields + shape.recursive
    } else {
        shape.fields
    }
}

/// `checkFrom ctors index branches induct`: from the constructor at
/// `index` on, each one has exactly one branch binding its fields (and,
/// under induction, one hypothesis per recursive field), and then no
/// branch names a constructor past the last.
///
/// # Errors
///
/// The first constructor with no branch, more than one, or the wrong
/// number of names, or else the first branch naming no constructor.
pub fn check_from<N>(
    ctors: &[CtorShape],
    index: usize,
    branches: &[BranchHead],
    induct: bool,
) -> Result<(), Refusal<N>> {
    let mut at = index;
    for shape in ctors {
        let mut of = branches.iter().filter(|branch| branch.ctor == at);
        let first = of.next();
        match (first, of.next()) {
            (None, _) => return Err(Refusal::MissingCase(at)),
            (Some(_), Some(_)) => return Err(Refusal::RepeatedCase(at)),
            (Some(branch), None) if branch.binders != wanted(*shape, induct) => {
                return Err(Refusal::BinderCount(at))
            }
            (Some(_), None) => {}
        }
        at += 1;
    }
    match branches.iter().find(|branch| branch.ctor >= at) {
        Some(stray) => Err(Refusal::UnknownCase(stray.ctor)),
        None => Ok(()),
    }
}

/// The branch whose proof is the minor premise of the constructor at
/// `index`: the first naming it, as the model's `proofOf`.
#[must_use]
pub fn minor_of(index: usize, branches: &[BranchHead]) -> Option<usize> {
    branches.iter().position(|branch| branch.ctor == index)
}

/// `memberCheck names name`: `name` is one of `names`.
///
/// # Errors
///
/// `NotProduced(name)` when it is not.
pub fn member_check<N: PartialEq + Clone>(names: &[N], name: &N) -> Result<(), Refusal<N>> {
    if names.contains(name) {
        Ok(())
    } else {
        Err(Refusal::NotProduced(name.clone()))
    }
}

/// `lawCheck law registered premises produced`: the law is registered and
/// every premise was produced.
///
/// # Errors
///
/// `UnregisteredLaw(law)`, or `NotProduced` of the first premise that was
/// not produced.
pub fn law_check<N: PartialEq + Clone>(
    law: &N,
    registered: &[N],
    premises: &[N],
    produced: &[N],
) -> Result<(), Refusal<N>> {
    if member_check(registered, law).is_err() {
        return Err(Refusal::UnregisteredLaw(law.clone()));
    }
    for premise in premises {
        member_check(produced, premise)?;
    }
    Ok(())
}

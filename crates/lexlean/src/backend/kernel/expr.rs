//! Lean kernel expressions in the locally nameless style: a term under
//! construction names its binders' variables by `FVar`, and a binder is
//! closed by abstracting its variable into de Bruijn index `0`. A finished
//! term holds no `FVar`; the writer refuses one that does.

use std::rc::Rc;

/// A universe level. LexLean data lives in `Type` and its proofs in `Prop`,
/// so no produced term needs `max` or `imax`; a level variable is needed only
/// by a declared type's `casesOn`, which both a value match (`Type`) and a
/// proof by cases (`Prop`) eliminate through.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Level {
    Zero,
    Succ(Rc<Level>),
    Param(String),
}

impl Level {
    /// The level `n`.
    #[must_use]
    pub fn of(n: u32) -> Self {
        (0..n).fold(Self::Zero, |level, _| Self::Succ(Rc::new(level)))
    }
}

/// A kernel expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    BVar(u32),
    FVar(u32),
    Sort(Level),
    Const(String, Vec<Level>),
    App(Rc<Expr>, Rc<Expr>),
    Lam(String, Rc<Expr>, Rc<Expr>),
    Pi(String, Rc<Expr>, Rc<Expr>),
    Let(String, Rc<Expr>, Rc<Expr>, Rc<Expr>),
    Nat(String),
    Str(String),
}

/// A constant with no universe arguments.
#[must_use]
pub fn c(name: &str) -> Expr {
    Expr::Const(name.to_owned(), Vec::new())
}

/// A constant at the given universe levels.
#[must_use]
pub fn cu(name: &str, levels: &[u32]) -> Expr {
    Expr::Const(
        name.to_owned(),
        levels.iter().map(|n| Level::of(*n)).collect(),
    )
}

/// `f a₁ … aₙ`.
#[must_use]
pub fn app(function: Expr, arguments: impl IntoIterator<Item = Expr>) -> Expr {
    arguments
        .into_iter()
        .fold(function, |f, a| Expr::App(Rc::new(f), Rc::new(a)))
}

/// `Sort n`.
#[must_use]
pub fn sort(n: u32) -> Expr {
    Expr::Sort(Level::of(n))
}

impl Expr {
    /// Replaces free variable `id` by de Bruijn index `depth`.
    #[must_use]
    pub fn abstract_fvar(&self, id: u32, depth: u32) -> Self {
        match self {
            Self::FVar(other) if *other == id => Self::BVar(depth),
            Self::App(f, a) => Self::App(
                Rc::new(f.abstract_fvar(id, depth)),
                Rc::new(a.abstract_fvar(id, depth)),
            ),
            Self::Lam(n, t, b) => Self::Lam(
                n.clone(),
                Rc::new(t.abstract_fvar(id, depth)),
                Rc::new(b.abstract_fvar(id, depth + 1)),
            ),
            Self::Pi(n, t, b) => Self::Pi(
                n.clone(),
                Rc::new(t.abstract_fvar(id, depth)),
                Rc::new(b.abstract_fvar(id, depth + 1)),
            ),
            Self::Let(n, t, v, b) => Self::Let(
                n.clone(),
                Rc::new(t.abstract_fvar(id, depth)),
                Rc::new(v.abstract_fvar(id, depth)),
                Rc::new(b.abstract_fvar(id, depth + 1)),
            ),
            other => other.clone(),
        }
    }

    /// Replaces free variable `id` by `by`, which holds no loose index.
    #[must_use]
    pub fn replace_fvar(&self, id: u32, by: &Self) -> Self {
        match self {
            Self::FVar(other) if *other == id => by.clone(),
            Self::App(f, a) => Self::App(
                Rc::new(f.replace_fvar(id, by)),
                Rc::new(a.replace_fvar(id, by)),
            ),
            Self::Lam(n, t, b) => Self::Lam(
                n.clone(),
                Rc::new(t.replace_fvar(id, by)),
                Rc::new(b.replace_fvar(id, by)),
            ),
            Self::Pi(n, t, b) => Self::Pi(
                n.clone(),
                Rc::new(t.replace_fvar(id, by)),
                Rc::new(b.replace_fvar(id, by)),
            ),
            Self::Let(n, t, v, b) => Self::Let(
                n.clone(),
                Rc::new(t.replace_fvar(id, by)),
                Rc::new(v.replace_fvar(id, by)),
                Rc::new(b.replace_fvar(id, by)),
            ),
            other => other.clone(),
        }
    }

    /// Every constant the expression names, into `out`.
    pub fn constants(&self, out: &mut std::collections::BTreeSet<String>) {
        match self {
            Self::Const(name, _) => {
                out.insert(name.clone());
            }
            Self::App(f, a) => {
                f.constants(out);
                a.constants(out);
            }
            Self::Lam(_, t, b) | Self::Pi(_, t, b) => {
                t.constants(out);
                b.constants(out);
            }
            Self::Let(_, t, v, b) => {
                t.constants(out);
                v.constants(out);
                b.constants(out);
            }
            _ => {}
        }
    }
}

/// `fun (name : ty) => body`, closing free variable `id`.
#[must_use]
pub fn lam(name: &str, id: u32, ty: Expr, body: &Expr) -> Expr {
    Expr::Lam(
        name.to_owned(),
        Rc::new(ty),
        Rc::new(body.abstract_fvar(id, 0)),
    )
}

/// `(name : ty) → body`, closing free variable `id`.
#[must_use]
pub fn pi(name: &str, id: u32, ty: Expr, body: &Expr) -> Expr {
    Expr::Pi(
        name.to_owned(),
        Rc::new(ty),
        Rc::new(body.abstract_fvar(id, 0)),
    )
}

/// `ty → body`, a binder the body does not use.
#[must_use]
pub fn arrow(ty: Expr, body: Expr) -> Expr {
    Expr::Pi("a".to_owned(), Rc::new(ty), Rc::new(body))
}

/// `let name : ty := value; body`, closing free variable `id`.
#[must_use]
pub fn let_in(name: &str, id: u32, ty: Expr, value: Expr, body: &Expr) -> Expr {
    Expr::Let(
        name.to_owned(),
        Rc::new(ty),
        Rc::new(value),
        Rc::new(body.abstract_fvar(id, 0)),
    )
}

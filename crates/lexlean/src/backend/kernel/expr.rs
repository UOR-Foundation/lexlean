//! Lean kernel expressions in the locally nameless style: a term under
//! construction names its binders' variables by `FVar`, and a binder is
//! closed by abstracting its variable into de Bruijn index `0`. A finished
//! term holds no `FVar`; the writer refuses one that does.

use std::rc::Rc;

/// A universe level. LexLean data lives in `Type` and its proofs in `Prop`,
/// so a term a producer states needs only `Zero` and `Succ`; the others
/// occur in the library's own terms, which evaluation copies.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Level {
    Zero,
    Succ(Rc<Level>),
    Max(Rc<Level>, Rc<Level>),
    IMax(Rc<Level>, Rc<Level>),
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
    /// Field `index` of a value of structure `type_name`.
    Proj(String, u32, Rc<Expr>),
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
        self.abstract_shared(id, depth, &mut std::collections::HashMap::new())
    }

    /// `abstract_fvar` over a term whose subterms are shared, each shared
    /// subterm visited once per depth rather than once per path to it.
    fn abstract_shared(
        &self,
        id: u32,
        depth: u32,
        memo: &mut std::collections::HashMap<(usize, u32), Self>,
    ) -> Self {
        let key = (std::ptr::from_ref(self) as usize, depth);
        if let Some(done) = memo.get(&key) {
            return done.clone();
        }
        let out = match self {
            Self::FVar(other) if *other == id => Self::BVar(depth),
            Self::App(f, a) => Self::App(
                Rc::new(f.abstract_shared(id, depth, memo)),
                Rc::new(a.abstract_shared(id, depth, memo)),
            ),
            Self::Lam(n, t, b) => Self::Lam(
                n.clone(),
                Rc::new(t.abstract_shared(id, depth, memo)),
                Rc::new(b.abstract_shared(id, depth + 1, memo)),
            ),
            Self::Pi(n, t, b) => Self::Pi(
                n.clone(),
                Rc::new(t.abstract_shared(id, depth, memo)),
                Rc::new(b.abstract_shared(id, depth + 1, memo)),
            ),
            Self::Let(n, t, v, b) => Self::Let(
                n.clone(),
                Rc::new(t.abstract_shared(id, depth, memo)),
                Rc::new(v.abstract_shared(id, depth, memo)),
                Rc::new(b.abstract_shared(id, depth + 1, memo)),
            ),
            Self::Proj(t, i, e) => Self::Proj(t.clone(), *i, Rc::new(e.abstract_shared(id, depth, memo))),
            other => other.clone(),
        };
        memo.insert(key, out.clone());
        out
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
            Self::Proj(t, i, e) => Self::Proj(t.clone(), *i, Rc::new(e.replace_fvar(id, by))),
            other => other.clone(),
        }
    }

    /// Every constant the expression names, into `out`.
    pub fn constants(&self, out: &mut std::collections::BTreeSet<String>) {
        self.constants_shared(out, &mut std::collections::HashSet::new());
    }

    fn constants_shared(
        &self,
        out: &mut std::collections::BTreeSet<String>,
        seen: &mut std::collections::HashSet<usize>,
    ) {
        if !seen.insert(std::ptr::from_ref(self) as usize) {
            return;
        }
        match self {
            Self::Const(name, _) => {
                out.insert(name.clone());
            }
            Self::App(f, a) => {
                f.constants_shared(out, seen);
                a.constants_shared(out, seen);
            }
            Self::Lam(_, t, b) | Self::Pi(_, t, b) => {
                t.constants_shared(out, seen);
                b.constants_shared(out, seen);
            }
            Self::Let(_, t, v, b) => {
                t.constants_shared(out, seen);
                v.constants_shared(out, seen);
                b.constants_shared(out, seen);
            }
            Self::Proj(t, _, e) => {
                out.insert(t.clone());
                e.constants_shared(out, seen);
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

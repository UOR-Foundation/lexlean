//! Evaluation over the library's own Lean terms, and the `term_cases`
//! producer built on it (SPEC.md §18.10).
//!
//! Lean proves `term_cases` with `cases h : t <;> simp only [defs, h]`.
//! The producer here reaches the same proof by evaluation: it reduces both
//! sides of the goal with the definitions exactly as the library states
//! them, and where a side is stuck on a `casesOn` whose major premise is
//! not a constructor, it splits that major premise,
//!
//! ```text
//! @T.rec ps (fun o => m = o → Q o) (fun fields h => …)… m (Eq.refl m) : Q m
//! ```
//!
//! where `Q m` is the partly reduced goal with `m` abstracted. A later
//! occurrence of a term the kernel equates with a split one is rewritten,
//! `@Eq.mpr G (Q c) (congrArg Q h) …`, and a goal whose sides reduce to the
//! same term is closed by `Eq.refl`.
//!
//! Nothing here is trusted. The reducer only finds the terms to split and
//! the motives to state; every motive is a reduct of the goal by delta,
//! beta, zeta, iota, and projection, and the kernel checks each of them,
//! and every leaf, by its own definitional equality. A reducer that went
//! wrong yields a term the kernel refuses.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use serde_json::Value;

use super::expr::{app, c, Expr, Level};

type R<T> = Result<T, String>;

/// The most reduction steps one theorem's production may take.
const STEPS: usize = 2_000_000;
/// The most splits and rewrites one theorem's proof may hold.
const SPLITS: usize = 512;
/// The most nested reductions and comparisons: a refusal past it, where the
/// thread's stack would otherwise end the whole production.
const DEPTH: usize = 1024;

/// One level of nesting, given back when dropped.
struct Nested<'c>(&'c Cell<usize>);

impl Drop for Nested<'_> {
    fn drop(&mut self) {
        self.0.set(self.0.get() - 1);
    }
}

enum Node {
    BVar(u32),
    Sort(u64),
    Const(u64, Vec<u64>),
    App(u64, u64),
    Lam(u64, u64, u64),
    Pi(u64, u64, u64),
    Let(u64, u64, u64, u64),
    Nat(String),
    Str(String),
    Proj(u64, u32, u64),
    MData(u64),
}

enum LevelNode {
    Succ(u64),
    Max(u64, u64),
    IMax(u64, u64),
    Param(u64),
}

/// What evaluation needs of a declaration.
enum Decl {
    /// A definition the kernel unfolds.
    Def { params: Vec<String>, value: u64 },
    /// A theorem, axiom, or opaque constant: never unfolded.
    Opaque,
    Ctor {
        induct: String,
        params: u32,
        ty: u64,
    },
    Induct {
        level_params: Vec<String>,
        indices: u32,
        recursive: bool,
        ctors: Vec<String>,
        ty: u64,
    },
    Rec {
        level_params: Vec<String>,
        params: u32,
        motives: u32,
        minors: u32,
        indices: u32,
        rules: Vec<(String, u64)>,
    },
    Quot,
}

/// A library export's declarations and terms, decoded on demand.
pub struct Library {
    names: HashMap<u64, String>,
    levels: HashMap<u64, LevelNode>,
    nodes: HashMap<u64, Node>,
    decls: HashMap<String, Decl>,
    exprs: RefCell<HashMap<u64, Expr>>,
}

fn num(value: &Value) -> R<u64> {
    value
        .as_u64()
        .ok_or_else(|| format!("an export index is not a number: {value}"))
}

fn get<'v>(value: &'v Value, key: &str) -> R<&'v Value> {
    value
        .get(key)
        .ok_or_else(|| format!("an export line lacks `{key}`"))
}

fn ids(value: &Value) -> R<Vec<u64>> {
    value
        .as_array()
        .map(|items| items.iter().map(num).collect())
        .unwrap_or_else(|| Ok(Vec::new()))
}

impl Library {
    /// Reads an export.
    ///
    /// # Errors
    ///
    /// Returns the first malformed line.
    #[allow(clippy::too_many_lines)]
    pub fn parse(text: &str) -> R<Self> {
        let mut lib = Self {
            names: HashMap::from([(0, String::new())]),
            levels: HashMap::new(),
            nodes: HashMap::new(),
            decls: HashMap::new(),
            exprs: RefCell::new(HashMap::new()),
        };
        let mut pending = Vec::new();
        for line in text.lines().filter(|line| !line.is_empty()) {
            let o: Value =
                serde_json::from_str(line).map_err(|error| format!("an export line: {error}"))?;
            if let Some(id) = o.get("in") {
                let id = num(id)?;
                let (part, pre) = if let Some(s) = o.get("str") {
                    (
                        get(s, "str")?.as_str().unwrap_or_default().to_owned(),
                        num(get(s, "pre")?)?,
                    )
                } else {
                    let n = get(&o, "num")?;
                    (num(get(n, "i")?)?.to_string(), num(get(n, "pre")?)?)
                };
                let prefix = lib.names.get(&pre).cloned().unwrap_or_default();
                let full = if prefix.is_empty() {
                    part
                } else {
                    format!("{prefix}.{part}")
                };
                lib.names.insert(id, full);
            } else if let Some(id) = o.get("il") {
                let id = num(id)?;
                let node = if let Some(v) = o.get("succ") {
                    LevelNode::Succ(num(v)?)
                } else if let Some(v) = o.get("max") {
                    let v = ids(v)?;
                    LevelNode::Max(v[0], v[1])
                } else if let Some(v) = o.get("imax") {
                    let v = ids(v)?;
                    LevelNode::IMax(v[0], v[1])
                } else {
                    LevelNode::Param(num(get(&o, "param")?)?)
                };
                lib.levels.insert(id, node);
            } else if let Some(id) = o.get("ie") {
                let id = num(id)?;
                let node = if let Some(v) = o.get("bvar") {
                    Node::BVar(u32::try_from(num(v)?).map_err(|e| e.to_string())?)
                } else if let Some(v) = o.get("sort") {
                    Node::Sort(num(v)?)
                } else if let Some(v) = o.get("const") {
                    Node::Const(num(get(v, "name")?)?, ids(get(v, "us")?)?)
                } else if let Some(v) = o.get("app") {
                    Node::App(num(get(v, "fn")?)?, num(get(v, "arg")?)?)
                } else if let Some(v) = o.get("lam") {
                    Node::Lam(
                        num(get(v, "name")?)?,
                        num(get(v, "type")?)?,
                        num(get(v, "body")?)?,
                    )
                } else if let Some(v) = o.get("forallE") {
                    Node::Pi(
                        num(get(v, "name")?)?,
                        num(get(v, "type")?)?,
                        num(get(v, "body")?)?,
                    )
                } else if let Some(v) = o.get("letE") {
                    Node::Let(
                        num(get(v, "name")?)?,
                        num(get(v, "type")?)?,
                        num(get(v, "value")?)?,
                        num(get(v, "body")?)?,
                    )
                } else if let Some(v) = o.get("natVal") {
                    Node::Nat(v.as_str().unwrap_or_default().to_owned())
                } else if let Some(v) = o.get("strVal") {
                    Node::Str(v.as_str().unwrap_or_default().to_owned())
                } else if let Some(v) = o.get("proj") {
                    Node::Proj(
                        num(get(v, "typeName")?)?,
                        u32::try_from(num(get(v, "idx")?)?).map_err(|e| e.to_string())?,
                        num(get(v, "struct")?)?,
                    )
                } else {
                    Node::MData(num(get(get(&o, "mdata")?, "expr")?)?)
                };
                lib.nodes.insert(id, node);
            } else if o.get("meta").is_none() {
                pending.push(o);
            }
        }
        for o in pending {
            lib.declare(&o)?;
        }
        Ok(lib)
    }

    fn name(&self, id: u64) -> String {
        self.names.get(&id).cloned().unwrap_or_default()
    }

    fn names(&self, value: &Value) -> R<Vec<String>> {
        Ok(ids(value)?.into_iter().map(|id| self.name(id)).collect())
    }

    fn count(value: &Value, key: &str) -> R<u32> {
        u32::try_from(num(get(value, key)?)?).map_err(|e| e.to_string())
    }

    fn declare(&mut self, o: &Value) -> R<()> {
        if let Some(d) = o.get("def") {
            let name = self.name(num(get(d, "name")?)?);
            let decl = if get(d, "safety")?.as_str() == Some("safe") {
                Decl::Def {
                    params: self.names(get(d, "levelParams")?)?,
                    value: num(get(d, "value")?)?,
                }
            } else {
                Decl::Opaque
            };
            self.decls.insert(name, decl);
        } else if let Some(d) = o
            .get("thm")
            .or_else(|| o.get("axiom"))
            .or_else(|| o.get("opaque"))
        {
            let name = self.name(num(get(d, "name")?)?);
            self.decls.insert(name, Decl::Opaque);
        } else if let Some(d) = o.get("quot") {
            let name = self.name(num(get(d, "name")?)?);
            self.decls.insert(name, Decl::Quot);
        } else if let Some(d) = o.get("inductive") {
            for t in get(d, "types")?.as_array().into_iter().flatten() {
                let name = self.name(num(get(t, "name")?)?);
                let decl = Decl::Induct {
                    level_params: self.names(get(t, "levelParams")?)?,
                    indices: Self::count(t, "numIndices")?,
                    recursive: get(t, "isRec")?.as_bool().unwrap_or(true)
                        || num(get(t, "numNested")?)? > 0,
                    ctors: self.names(get(t, "ctors")?)?,
                    ty: num(get(t, "type")?)?,
                };
                self.decls.insert(name, decl);
            }
            for k in get(d, "ctors")?.as_array().into_iter().flatten() {
                let name = self.name(num(get(k, "name")?)?);
                let decl = Decl::Ctor {
                    induct: self.name(num(get(k, "induct")?)?),
                    params: Self::count(k, "numParams")?,
                    ty: num(get(k, "type")?)?,
                };
                self.decls.insert(name, decl);
            }
            for r in get(d, "recs")?.as_array().into_iter().flatten() {
                let name = self.name(num(get(r, "name")?)?);
                let mut rules = Vec::new();
                for rule in get(r, "rules")?.as_array().into_iter().flatten() {
                    rules.push((self.name(num(get(rule, "ctor")?)?), num(get(rule, "rhs")?)?));
                }
                let decl = Decl::Rec {
                    level_params: self.names(get(r, "levelParams")?)?,
                    params: Self::count(r, "numParams")?,
                    motives: Self::count(r, "numMotives")?,
                    minors: Self::count(r, "numMinors")?,
                    indices: Self::count(r, "numIndices")?,
                    rules,
                };
                self.decls.insert(name, decl);
            }
        }
        Ok(())
    }

    fn level(&self, id: u64) -> Level {
        match self.levels.get(&id) {
            None => Level::Zero,
            Some(LevelNode::Succ(a)) => Level::Succ(Rc::new(self.level(*a))),
            Some(LevelNode::Max(a, b)) => {
                Level::Max(Rc::new(self.level(*a)), Rc::new(self.level(*b)))
            }
            Some(LevelNode::IMax(a, b)) => {
                Level::IMax(Rc::new(self.level(*a)), Rc::new(self.level(*b)))
            }
            Some(LevelNode::Param(n)) => Level::Param(self.name(*n)),
        }
    }

    /// Expression `id`, decoded once and shared.
    fn expr(&self, id: u64) -> R<Expr> {
        if let Some(e) = self.exprs.borrow().get(&id) {
            return Ok(e.clone());
        }
        let node = self
            .nodes
            .get(&id)
            .ok_or_else(|| format!("the export states no expression {id}"))?;
        let e = match node {
            Node::BVar(i) => Expr::BVar(*i),
            Node::Sort(l) => Expr::Sort(self.level(*l)),
            Node::Const(n, us) => {
                Expr::Const(self.name(*n), us.iter().map(|u| self.level(*u)).collect())
            }
            Node::App(f, a) => Expr::App(Rc::new(self.expr(*f)?), Rc::new(self.expr(*a)?)),
            Node::Lam(n, t, b) => {
                Expr::Lam(self.name(*n), Rc::new(self.expr(*t)?), Rc::new(self.expr(*b)?))
            }
            Node::Pi(n, t, b) => {
                Expr::Pi(self.name(*n), Rc::new(self.expr(*t)?), Rc::new(self.expr(*b)?))
            }
            Node::Let(n, t, v, b) => Expr::Let(
                self.name(*n),
                Rc::new(self.expr(*t)?),
                Rc::new(self.expr(*v)?),
                Rc::new(self.expr(*b)?),
            ),
            Node::Nat(v) => Expr::Nat(v.clone()),
            Node::Str(v) => Expr::Str(v.clone()),
            Node::Proj(t, i, s) => Expr::Proj(self.name(*t), *i, Rc::new(self.expr(*s)?)),
            Node::MData(e) => self.expr(*e)?,
        };
        self.exprs.borrow_mut().insert(id, e.clone());
        Ok(e)
    }
}

/// `level` with each named parameter replaced.
fn subst_level(level: &Level, params: &[String], by: &[Level]) -> Level {
    match level {
        Level::Zero => Level::Zero,
        Level::Succ(a) => Level::Succ(Rc::new(subst_level(a, params, by))),
        Level::Max(a, b) => Level::Max(
            Rc::new(subst_level(a, params, by)),
            Rc::new(subst_level(b, params, by)),
        ),
        Level::IMax(a, b) => Level::IMax(
            Rc::new(subst_level(a, params, by)),
            Rc::new(subst_level(b, params, by)),
        ),
        Level::Param(name) => params
            .iter()
            .position(|p| p == name)
            .and_then(|i| by.get(i).cloned())
            .unwrap_or_else(|| level.clone()),
    }
}

fn subst_levels(e: &Expr, params: &[String], by: &[Level], memo: &mut HashMap<usize, Expr>) -> Expr {
    if params.is_empty() {
        return e.clone();
    }
    let key = std::ptr::from_ref(e) as usize;
    if let Some(done) = memo.get(&key) {
        return done.clone();
    }
    let out = match e {
        Expr::Sort(l) => Expr::Sort(subst_level(l, params, by)),
        Expr::Const(n, us) => Expr::Const(
            n.clone(),
            us.iter().map(|u| subst_level(u, params, by)).collect(),
        ),
        Expr::App(f, a) => Expr::App(
            Rc::new(subst_levels(f, params, by, memo)),
            Rc::new(subst_levels(a, params, by, memo)),
        ),
        Expr::Lam(n, t, b) => Expr::Lam(
            n.clone(),
            Rc::new(subst_levels(t, params, by, memo)),
            Rc::new(subst_levels(b, params, by, memo)),
        ),
        Expr::Pi(n, t, b) => Expr::Pi(
            n.clone(),
            Rc::new(subst_levels(t, params, by, memo)),
            Rc::new(subst_levels(b, params, by, memo)),
        ),
        Expr::Let(n, t, v, b) => Expr::Let(
            n.clone(),
            Rc::new(subst_levels(t, params, by, memo)),
            Rc::new(subst_levels(v, params, by, memo)),
            Rc::new(subst_levels(b, params, by, memo)),
        ),
        Expr::Proj(t, i, s) => Expr::Proj(t.clone(), *i, Rc::new(subst_levels(s, params, by, memo))),
        Expr::BVar(_) | Expr::FVar(_) | Expr::Nat(_) | Expr::Str(_) => e.clone(),
    };
    memo.insert(key, out.clone());
    out
}

/// `body` with loose index `depth` replaced by `value`, which has no loose
/// index, and every looser index lowered by one.
fn inst(body: &Expr, value: &Expr, depth: u32, memo: &mut HashMap<(usize, u32), Expr>) -> Expr {
    let key = (std::ptr::from_ref(body) as usize, depth);
    if let Some(done) = memo.get(&key) {
        return done.clone();
    }
    let out = match body {
        Expr::BVar(i) if *i == depth => value.clone(),
        Expr::BVar(i) if *i > depth => Expr::BVar(i - 1),
        Expr::App(f, a) => Expr::App(
            Rc::new(inst(f, value, depth, memo)),
            Rc::new(inst(a, value, depth, memo)),
        ),
        Expr::Lam(n, t, b) => Expr::Lam(
            n.clone(),
            Rc::new(inst(t, value, depth, memo)),
            Rc::new(inst(b, value, depth + 1, memo)),
        ),
        Expr::Pi(n, t, b) => Expr::Pi(
            n.clone(),
            Rc::new(inst(t, value, depth, memo)),
            Rc::new(inst(b, value, depth + 1, memo)),
        ),
        Expr::Let(n, t, v, b) => Expr::Let(
            n.clone(),
            Rc::new(inst(t, value, depth, memo)),
            Rc::new(inst(v, value, depth, memo)),
            Rc::new(inst(b, value, depth + 1, memo)),
        ),
        Expr::Proj(t, i, s) => Expr::Proj(t.clone(), *i, Rc::new(inst(s, value, depth, memo))),
        Expr::BVar(_)
        | Expr::FVar(_)
        | Expr::Sort(_)
        | Expr::Const(..)
        | Expr::Nat(_)
        | Expr::Str(_) => body.clone(),
    };
    memo.insert(key, out.clone());
    out
}

fn instantiate(body: &Expr, value: &Expr) -> Expr {
    inst(body, value, 0, &mut HashMap::new())
}

/// Every occurrence of `target` in `e` replaced by index `depth`.
fn abstract_term(e: &Expr, target: &Expr, depth: u32) -> Expr {
    abstract_in(e, target, depth, &mut HashMap::new())
}

fn abstract_in(e: &Expr, target: &Expr, depth: u32, memo: &mut HashMap<(usize, u32), Expr>) -> Expr {
    let key = (std::ptr::from_ref(e) as usize, depth);
    if let Some(done) = memo.get(&key) {
        return done.clone();
    }
    let out = if e == target {
        Expr::BVar(depth)
    } else {
        match e {
            Expr::App(f, a) => Expr::App(
                Rc::new(abstract_in(f, target, depth, memo)),
                Rc::new(abstract_in(a, target, depth, memo)),
            ),
            Expr::Lam(n, t, b) => Expr::Lam(
                n.clone(),
                Rc::new(abstract_in(t, target, depth, memo)),
                Rc::new(abstract_in(b, target, depth + 1, memo)),
            ),
            Expr::Pi(n, t, b) => Expr::Pi(
                n.clone(),
                Rc::new(abstract_in(t, target, depth, memo)),
                Rc::new(abstract_in(b, target, depth + 1, memo)),
            ),
            Expr::Let(n, t, v, b) => Expr::Let(
                n.clone(),
                Rc::new(abstract_in(t, target, depth, memo)),
                Rc::new(abstract_in(v, target, depth, memo)),
                Rc::new(abstract_in(b, target, depth + 1, memo)),
            ),
            Expr::Proj(t, i, s) => {
                Expr::Proj(t.clone(), *i, Rc::new(abstract_in(s, target, depth, memo)))
            }
            Expr::BVar(_)
            | Expr::FVar(_)
            | Expr::Sort(_)
            | Expr::Const(..)
            | Expr::Nat(_)
            | Expr::Str(_) => e.clone(),
        }
    };
    memo.insert(key, out.clone());
    out
}

/// Whether free variable `id` occurs in `e`.
fn mentions(e: &Expr, id: u32) -> bool {
    mentions_in(e, id, &mut std::collections::HashSet::new())
}

fn mentions_in(e: &Expr, id: u32, seen: &mut std::collections::HashSet<usize>) -> bool {
    if !seen.insert(std::ptr::from_ref(e) as usize) {
        return false;
    }
    match e {
        Expr::FVar(other) => *other == id,
        Expr::App(f, a) => mentions_in(f, id, seen) || mentions_in(a, id, seen),
        Expr::Lam(_, t, b) | Expr::Pi(_, t, b) => mentions_in(t, id, seen) || mentions_in(b, id, seen),
        Expr::Let(_, t, v, b) => {
            mentions_in(t, id, seen) || mentions_in(v, id, seen) || mentions_in(b, id, seen)
        }
        Expr::Proj(_, _, s) => mentions_in(s, id, seen),
        Expr::BVar(_) | Expr::Sort(_) | Expr::Const(..) | Expr::Nat(_) | Expr::Str(_) => false,
    }
}

/// The head of an application and its arguments.
fn spine(e: &Expr) -> (Expr, Vec<Expr>) {
    let mut args = Vec::new();
    let mut head = e;
    while let Expr::App(f, a) = head {
        args.push((**a).clone());
        head = f;
    }
    args.reverse();
    (head.clone(), args)
}

fn nat_of(e: &Expr) -> Option<u128> {
    match e {
        Expr::Nat(digits) => digits.parse().ok(),
        Expr::Const(name, _) if name == "Nat.zero" => Some(0),
        _ => None,
    }
}

fn nat(n: u128) -> Expr {
    Expr::Nat(n.to_string())
}

fn boolean(b: bool) -> Expr {
    c(if b { "Bool.true" } else { "Bool.false" })
}

/// The kernel's arithmetic on numerals.
fn nat_op(name: &str, a: u128, b: u128) -> Option<Expr> {
    Some(match name {
        "Nat.add" => nat(a.checked_add(b)?),
        "Nat.sub" => nat(a.saturating_sub(b)),
        "Nat.mul" => nat(a.checked_mul(b)?),
        "Nat.div" => nat(if b == 0 { 0 } else { a / b }),
        "Nat.mod" => nat(if b == 0 { a } else { a % b }),
        "Nat.gcd" => {
            let (mut x, mut y) = (a, b);
            while y != 0 {
                (x, y) = (y, x % y);
            }
            nat(x)
        }
        "Nat.beq" => boolean(a == b),
        "Nat.ble" => boolean(a <= b),
        "Nat.land" => nat(a & b),
        "Nat.lor" => nat(a | b),
        "Nat.xor" => nat(a ^ b),
        "Nat.shiftLeft" => nat(a.checked_shl(u32::try_from(b).ok()?)?),
        "Nat.shiftRight" => nat(a.checked_shr(u32::try_from(b).ok()?).unwrap_or(0)),
        "Nat.pow" => nat(a.checked_pow(u32::try_from(b).ok()?)?),
        _ => return None,
    })
}

/// A term evaluation stopped at: the major premise `major` of recursor
/// `rec`, at universe levels `levels`, over inductive type `induct` with
/// parameters `params`.
#[derive(Clone)]
struct Stuck {
    major: Expr,
    induct: String,
    levels: Vec<Level>,
    params: Vec<Expr>,
}

enum Compared {
    Equal,
    Split(Stuck),
    Differ(String),
}

/// A split already made: `term = ctor` holds by hypothesis `proof`, over
/// type `ty` at level `level`.
#[derive(Clone)]
struct Hyp {
    term: Expr,
    ctor: Expr,
    proof: Expr,
    ty: Expr,
    level: Level,
}

/// Evaluation over a library.
pub struct Eval<'l> {
    lib: &'l Library,
    fresh: &'l dyn Fn() -> u32,
    steps: Cell<usize>,
    splits: Cell<usize>,
    depth: Cell<usize>,
}

impl<'l> Eval<'l> {
    pub fn new(lib: &'l Library, fresh: &'l dyn Fn() -> u32) -> Self {
        Self {
            lib,
            fresh,
            steps: Cell::new(0),
            splits: Cell::new(0),
            depth: Cell::new(0),
        }
    }

    fn tick(&self) -> R<()> {
        let n = self.steps.get() + 1;
        self.steps.set(n);
        if n > STEPS {
            return Err(format!("evaluation exceeds {STEPS} steps"));
        }
        Ok(())
    }

    fn nest(&self) -> R<Nested<'_>> {
        let n = self.depth.get() + 1;
        if n > DEPTH {
            return Err(format!("evaluation nests deeper than {DEPTH}"));
        }
        self.depth.set(n);
        Ok(Nested(&self.depth))
    }

    fn decl(&self, name: &str) -> Option<&'l Decl> {
        self.lib.decls.get(name)
    }

    fn typed(&self, id: u64, params: &[String], levels: &[Level]) -> R<Expr> {
        Ok(subst_levels(&self.lib.expr(id)?, params, levels, &mut HashMap::new()))
    }

    /// `e` as a constructor application, its constructor and arguments.
    fn as_ctor(&self, e: &Expr) -> Option<(String, Vec<Level>, Vec<Expr>)> {
        if let Some(n) = nat_of(e) {
            return Some(if n == 0 {
                ("Nat.zero".to_owned(), Vec::new(), Vec::new())
            } else {
                ("Nat.succ".to_owned(), Vec::new(), vec![nat(n - 1)])
            });
        }
        let (head, args) = spine(e);
        if let Expr::Const(name, levels) = head {
            if matches!(self.decl(&name), Some(Decl::Ctor { .. })) {
                return Some((name, levels, args));
            }
        }
        None
    }

    /// Weak head normal form.
    #[allow(clippy::too_many_lines)]
    pub fn whnf(&self, e: &Expr) -> R<Expr> {
        let _nested = self.nest()?;
        let mut e = e.clone();
        loop {
            self.tick()?;
            let (head, args) = spine(&e);
            match &head {
                Expr::Lam(..) if !args.is_empty() => {
                    let mut body = head;
                    let mut used = 0;
                    while let (Expr::Lam(_, _, b), Some(arg)) = (&body, args.get(used)) {
                        body = instantiate(b, arg);
                        used += 1;
                    }
                    e = app(body, args[used..].iter().cloned());
                }
                Expr::Let(_, _, v, b) => e = app(instantiate(b, v), args),
                Expr::Proj(_, index, s) => {
                    let s = self.whnf(s)?;
                    let Some((ctor, _, cargs)) = self.as_ctor(&s) else {
                        return Ok(e);
                    };
                    let Some(Decl::Ctor { params, .. }) = self.decl(&ctor) else {
                        return Ok(e);
                    };
                    let field = cargs
                        .get((params + index) as usize)
                        .cloned()
                        .ok_or("a projection past the last field")?;
                    e = app(field, args);
                }
                Expr::Const(name, levels) => match self.decl(name) {
                    Some(Decl::Def { params, value }) => {
                        if args.len() == 2 {
                            let a = nat_of(&self.whnf(&args[0])?);
                            let b = nat_of(&self.whnf(&args[1])?);
                            if let (Some(a), Some(b)) = (a, b) {
                                if let Some(out) = nat_op(name, a, b) {
                                    return Ok(out);
                                }
                            }
                        }
                        if name == "Nat.log2" && args.len() == 1 {
                            if let Some(a) = nat_of(&self.whnf(&args[0])?) {
                                return Ok(nat(if a == 0 { 0 } else { u128::from(127 - a.leading_zeros()) }));
                            }
                        }
                        e = app(self.typed(*value, params, levels)?, args);
                    }
                    Some(Decl::Rec {
                        level_params,
                        params,
                        motives,
                        minors,
                        indices,
                        rules,
                    }) => {
                        let at = (params + motives + minors + indices) as usize;
                        let Some(major) = args.get(at) else {
                            return Ok(e);
                        };
                        let induct = name.strip_suffix(".rec").unwrap_or(name);
                        let major = self.whnf(major)?;
                        if !matches!(self.as_ctor(&major), Some((ref k, ..)) if self.ctor_of(k) == Some(induct))
                        {
                            return Ok(e);
                        }
                        let Some((ctor, _, cargs)) = self.as_ctor(&major) else {
                            return Ok(e);
                        };
                        let Some(Decl::Ctor {
                            params: cparams, ..
                        }) = self.decl(&ctor)
                        else {
                            return Ok(e);
                        };
                        let Some((_, rhs)) = rules.iter().find(|(k, _)| *k == ctor) else {
                            return Ok(e);
                        };
                        let rhs = self.typed(*rhs, level_params, levels)?;
                        let prefix = (params + motives + minors) as usize;
                        e = app(
                            app(
                                app(rhs, args[..prefix].iter().cloned()),
                                cargs[*cparams as usize..].iter().cloned(),
                            ),
                            args[at + 1..].iter().cloned(),
                        );
                    }
                    Some(Decl::Quot) if name == "Quot.lift" && args.len() >= 6 => {
                        let q = self.whnf(&args[5])?;
                        let (qh, qargs) = spine(&q);
                        if !matches!(&qh, Expr::Const(n, _) if n == "Quot.mk") || qargs.len() != 3 {
                            return Ok(e);
                        }
                        e = app(app(args[3].clone(), [qargs[2].clone()]), args[6..].iter().cloned());
                    }
                    Some(Decl::Quot) if name == "Quot.ind" && args.len() >= 5 => {
                        let q = self.whnf(&args[4])?;
                        let (qh, qargs) = spine(&q);
                        if !matches!(&qh, Expr::Const(n, _) if n == "Quot.mk") || qargs.len() != 3 {
                            return Ok(e);
                        }
                        e = app(app(args[3].clone(), [qargs[2].clone()]), args[5..].iter().cloned());
                    }
                    Some(Decl::Ctor { .. }) if name == "Nat.succ" && args.len() == 1 => {
                        let a = self.whnf(&args[0])?;
                        return Ok(match nat_of(&a) {
                            Some(n) => nat(n + 1),
                            None => app(head.clone(), [a]),
                        });
                    }
                    _ => return Ok(e),
                },
                _ => return Ok(e),
            }
        }
    }

    fn ctor_of(&self, ctor: &str) -> Option<&'l str> {
        match self.decl(ctor) {
            Some(Decl::Ctor { induct, .. }) => Some(induct),
            _ => None,
        }
    }

    fn is_prop_induct(&self, induct: &str) -> R<bool> {
        let Some(Decl::Induct { ty, .. }) = self.decl(induct) else {
            return Ok(false);
        };
        let mut t = self.lib.expr(*ty)?;
        while let Expr::Pi(_, _, b) = t {
            t = (*b).clone();
        }
        Ok(matches!(t, Expr::Sort(Level::Zero)))
    }

    /// `e` reduced until a recursor is stuck in it, with the place it is
    /// stuck rebuilt into the term, and the major premise to split, the
    /// outermost one that can be.
    fn expose(&self, e: &Expr) -> R<(Expr, Option<Stuck>)> {
        let _nested = self.nest()?;
        let w = self.whnf(e)?;
        let (head, args) = spine(&w);
        match &head {
            Expr::Const(name, levels) => {
                let Some(Decl::Rec {
                    params,
                    motives,
                    minors,
                    indices,
                    ..
                }) = self.decl(name)
                else {
                    return Ok((w, None));
                };
                let at = (params + motives + minors + indices) as usize;
                if args.len() <= at {
                    return Ok((w, None));
                }
                let induct = name.strip_suffix(".rec").unwrap_or(name).to_owned();
                let Some(Decl::Induct {
                    level_params,
                    indices: type_indices,
                    recursive,
                    ..
                }) = self.decl(&induct)
                else {
                    return Ok((w, None));
                };
                let type_levels = levels[levels.len() - level_params.len()..].to_vec();
                // The innermost place first: a major premise that is itself
                // stuck is split where it is stuck. Splitting it here
                // instead would pair each of its constructors with every
                // case of what it is stuck on, cases no value reaches, and
                // those do not close.
                let (inner, stuck) = self.expose(&args[at])?;
                let mut rebuilt = args.clone();
                rebuilt[at] = inner.clone();
                let rebuilt = app(head.clone(), rebuilt);
                if stuck.is_some() {
                    return Ok((rebuilt, stuck));
                }
                if *type_indices == 0 && !*recursive && !self.is_prop_induct(&induct)? {
                    let stuck = Stuck {
                        major: inner,
                        induct,
                        levels: type_levels,
                        params: args[..*params as usize].to_vec(),
                    };
                    return Ok((rebuilt, Some(stuck)));
                }
                Ok((rebuilt, None))
            }
            Expr::Proj(t, i, s) => {
                let (inner, stuck) = self.expose(s)?;
                Ok((app(Expr::Proj(t.clone(), *i, Rc::new(inner)), args), stuck))
            }
            _ => Ok((w, None)),
        }
    }

    /// Compares two terms by evaluation: equal, or a split that may make
    /// them so, with each side reduced to where that split occurs in it.
    #[allow(clippy::too_many_lines)]
    fn compare(&self, a: &Expr, b: &Expr) -> R<(Expr, Expr, Compared)> {
        let _nested = self.nest()?;
        if a == b {
            return Ok((a.clone(), b.clone(), Compared::Equal));
        }
        let (a, sa) = self.expose(a)?;
        let (b, sb) = self.expose(b)?;
        if a == b {
            return Ok((a, b, Compared::Equal));
        }
        if let Some(stuck) = sa.or(sb) {
            return Ok((a, b, Compared::Split(stuck)));
        }
        if let (Some(x), Some(y)) = (nat_of(&a), nat_of(&b)) {
            let out = if x == y {
                Compared::Equal
            } else {
                Compared::Differ(format!("{x} is not {y}"))
            };
            return Ok((a, b, out));
        }
        let ca = self.as_ctor(&a);
        let cb = self.as_ctor(&b);
        if let (Some((ka, la, xa)), Some((kb, lb, xb))) = (ca, cb) {
            if ka != kb || xa.len() != xb.len() {
                return Ok((a, b, Compared::Differ(format!("`{ka}` is not `{kb}`"))));
            }
            let Some(Decl::Ctor { params, ty, .. }) = self.decl(&ka) else {
                return Ok((a, b, Compared::Differ(format!("`{ka}`"))));
            };
            let props = self.prop_fields(&ka, *ty, &la)?;
            let mut xa = xa;
            let mut xb = xb;
            for i in 0..xa.len() {
                if i >= *params as usize && props.get(i - *params as usize) == Some(&true) {
                    continue;
                }
                let (na, nb, out) = self.compare(&xa[i], &xb[i])?;
                xa[i] = na;
                xb[i] = nb;
                if !matches!(out, Compared::Equal) {
                    let a = app(Expr::Const(ka, la), xa);
                    let b = app(Expr::Const(kb, lb), xb);
                    return Ok((a, b, out));
                }
            }
            return Ok((a, b, Compared::Equal));
        }
        match (&a, &b) {
            (Expr::Lam(n, t, ba), Expr::Lam(_, _, bb)) => {
                let id = (self.fresh)();
                let x = Expr::FVar(id);
                let (na, nb, mut out) = self.compare(&instantiate(ba, &x), &instantiate(bb, &x))?;
                // A split term that names the bound variable is no term of
                // the goal's context.
                if let Compared::Split(stuck) = &out {
                    if mentions(&stuck.major, id) || stuck.params.iter().any(|p| mentions(p, id)) {
                        out = Compared::Differ("a split under a binder".to_owned());
                    }
                }
                let close = |body: &Expr| Expr::Lam(n.clone(), t.clone(), Rc::new(body.abstract_fvar(id, 0)));
                Ok((close(&na), close(&nb), out))
            }
            _ => {
                let (ha, xa) = spine(&a);
                let (hb, xb) = spine(&b);
                let neutral = match &ha {
                    Expr::FVar(_) => true,
                    // A recursor here is stuck on a major premise that is
                    // not split (a recursive type's), so two applications of
                    // it are equal when their arguments are.
                    Expr::Const(name, _) => matches!(
                        self.decl(name),
                        Some(Decl::Opaque | Decl::Quot | Decl::Induct { .. } | Decl::Rec { .. }) | None
                    ),
                    _ => false,
                };
                if !neutral || ha != hb || xa.len() != xb.len() {
                    return Ok((a, b, Compared::Differ("the sides reduce apart".to_owned())));
                }
                let mut xa = xa;
                let mut xb = xb;
                for i in 0..xa.len() {
                    let (na, nb, out) = self.compare(&xa[i], &xb[i])?;
                    xa[i] = na;
                    xb[i] = nb;
                    if !matches!(out, Compared::Equal) {
                        return Ok((app(ha, xa), app(hb, xb), out));
                    }
                }
                Ok((a, b, Compared::Equal))
            }
        }
    }

    /// Which fields of constructor `ctor` are proofs: two values that
    /// differ only in them are equal by proof irrelevance.
    fn prop_fields(&self, ctor: &str, ty: u64, levels: &[Level]) -> R<Vec<bool>> {
        let Some(Decl::Ctor { params, induct, .. }) = self.decl(ctor) else {
            return Ok(Vec::new());
        };
        let Some(Decl::Induct { level_params, .. }) = self.decl(induct) else {
            return Ok(Vec::new());
        };
        let mut t = self.typed(ty, level_params, levels)?;
        let mut out = Vec::new();
        let mut seen = 0;
        while let Expr::Pi(_, binder, body) = t {
            if seen >= *params {
                out.push(self.is_prop_type(&binder));
            }
            seen += 1;
            t = (*body).clone();
        }
        Ok(out)
    }

    /// Whether `ty`, read without its context, is a proposition: an
    /// inductive family in `Prop`, or an equation.
    fn is_prop_type(&self, ty: &Expr) -> bool {
        let mut t = ty;
        while let Expr::Pi(_, _, b) = t {
            t = b;
        }
        let (head, _) = spine(t);
        match head {
            Expr::Const(name, _) => self.is_prop_induct(&name).unwrap_or(false),
            _ => false,
        }
    }

    /// The universe `T ps` lives in, and its constructors' field types.
    fn type_level(&self, stuck: &Stuck) -> R<Level> {
        let Some(Decl::Induct {
            level_params, ty, ..
        }) = self.decl(&stuck.induct)
        else {
            return Err(format!("`{}` is not an inductive type", stuck.induct));
        };
        let mut t = self.typed(*ty, level_params, &stuck.levels)?;
        for p in &stuck.params {
            match t {
                Expr::Pi(_, _, body) => t = instantiate(&body, p),
                _ => return Err("a type with fewer parameters than applied".to_owned()),
            }
        }
        match t {
            Expr::Sort(Level::Succ(l)) => Ok((*l).clone()),
            Expr::Sort(l) => Err(format!("`{}` lives in Sort {l:?}", stuck.induct)),
            _ => Err(format!("`{}` is a family", stuck.induct)),
        }
    }

    /// A proof of `goal`, `@Eq α l r`, by evaluation, splits, and rewrites.
    fn solve(&self, goal: &Expr, hyps: &[Hyp]) -> R<Expr> {
        let g = self.whnf(goal)?;
        let (head, args) = spine(&g);
        let Expr::Const(eq, levels) = &head else {
            return Err("the goal is not an equation".to_owned());
        };
        if eq != "Eq" || args.len() != 3 {
            return Err(format!("the goal is `{eq}`, not an equation"));
        }
        let (l, r, out) = self.compare(&args[1], &args[2])?;
        match out {
            Compared::Equal => Ok(app(
                Expr::Const("Eq.refl".to_owned(), levels.clone()),
                [args[0].clone(), args[1].clone()],
            )),
            Compared::Differ(why) => Err(format!("a case does not close: {why}")),
            Compared::Split(stuck) => {
                let n = self.splits.get() + 1;
                self.splits.set(n);
                if n > SPLITS {
                    return Err(format!("more than {SPLITS} cases"));
                }
                let reduced = app(head.clone(), [args[0].clone(), l, r]);
                let motive_body = abstract_term(&reduced, &stuck.major, 0);
                for hyp in hyps {
                    if hyp.term == stuck.major || self.same(&hyp.term, &stuck.major)? {
                        return self.rewrite(goal, &motive_body, hyp, hyps);
                    }
                }
                self.split(&stuck, &motive_body, hyps)
            }
        }
    }

    fn same(&self, a: &Expr, b: &Expr) -> R<bool> {
        Ok(matches!(self.compare(a, b)?.2, Compared::Equal))
    }

    /// `@Eq.mpr goal (Q c) (congrArg Q h) proof`.
    fn rewrite(&self, goal: &Expr, motive_body: &Expr, hyp: &Hyp, hyps: &[Hyp]) -> R<Expr> {
        let q = Expr::Lam("o".to_owned(), Rc::new(hyp.ty.clone()), Rc::new(motive_body.clone()));
        let next = instantiate(motive_body, &hyp.ctor);
        let proof = self.solve(&next, hyps)?;
        let prop = Expr::Sort(Level::Zero);
        let congruence = app(
            Expr::Const(
                "congrArg".to_owned(),
                vec![hyp.level.clone(), Level::Succ(Rc::new(Level::Zero))],
            ),
            [
                hyp.ty.clone(),
                prop,
                hyp.term.clone(),
                hyp.ctor.clone(),
                q,
                hyp.proof.clone(),
            ],
        );
        Ok(app(
            Expr::Const("Eq.mpr".to_owned(), vec![Level::Zero]),
            [goal.clone(), next, congruence, proof],
        ))
    }

    /// `@T.rec ps (fun o => m = o → Q o) minors m (Eq.refl m)`.
    fn split(&self, stuck: &Stuck, motive_body: &Expr, hyps: &[Hyp]) -> R<Expr> {
        let Some(Decl::Induct {
            ctors, level_params, ..
        }) = self.decl(&stuck.induct)
        else {
            return Err(format!("`{}` is not an inductive type", stuck.induct));
        };
        let level = self.type_level(stuck)?;
        let ty = app(
            Expr::Const(stuck.induct.clone(), stuck.levels.clone()),
            stuck.params.iter().cloned(),
        );
        let eq = |x: Expr, y: Expr| {
            app(
                Expr::Const("Eq".to_owned(), vec![level.clone()]),
                [ty.clone(), x, y],
            )
        };
        // The motive, over a fresh variable closed afterwards.
        let o = (self.fresh)();
        let ov = Expr::FVar(o);
        let motive = Expr::Lam(
            "o".to_owned(),
            Rc::new(ty.clone()),
            Rc::new(
                Expr::Pi(
                    "h".to_owned(),
                    Rc::new(eq(stuck.major.clone(), ov.clone())),
                    Rc::new(instantiate(motive_body, &ov)),
                )
                .abstract_fvar(o, 0),
            ),
        );
        let mut minors = Vec::new();
        for ctor in ctors {
            let Some(Decl::Ctor { ty: cty, .. }) = self.decl(ctor) else {
                return Err(format!("`{ctor}` is not a constructor"));
            };
            let mut t = self.typed(*cty, level_params, &stuck.levels)?;
            for p in &stuck.params {
                match t {
                    Expr::Pi(_, _, body) => t = instantiate(&body, p),
                    _ => return Err(format!("`{ctor}` has fewer parameters than applied")),
                }
            }
            let mut fields = Vec::new();
            while let Expr::Pi(n, binder, body) = t {
                let id = (self.fresh)();
                fields.push((n.clone(), id, (*binder).clone()));
                t = instantiate(&body, &Expr::FVar(id));
            }
            let value = app(
                app(
                    Expr::Const(ctor.clone(), stuck.levels.clone()),
                    stuck.params.iter().cloned(),
                ),
                fields.iter().map(|(_, id, _)| Expr::FVar(*id)),
            );
            let h = (self.fresh)();
            let hyp_ty = eq(stuck.major.clone(), value.clone());
            let mut inner = hyps.to_vec();
            inner.push(Hyp {
                term: stuck.major.clone(),
                ctor: value.clone(),
                proof: Expr::FVar(h),
                ty: ty.clone(),
                level: level.clone(),
            });
            let next = instantiate(motive_body, &value);
            let body = self.solve(&next, &inner)?;
            let mut minor = Expr::Lam("h".to_owned(), Rc::new(hyp_ty), Rc::new(body.abstract_fvar(h, 0)));
            for (n, id, binder) in fields.into_iter().rev() {
                minor = Expr::Lam(n, Rc::new(binder), Rc::new(minor.abstract_fvar(id, 0)));
            }
            minors.push(minor);
        }
        let mut rec_levels = vec![Level::Zero];
        rec_levels.extend(stuck.levels.iter().cloned());
        if let Some(Decl::Rec { level_params: rp, .. }) =
            self.decl(&format!("{}.rec", stuck.induct))
        {
            if rp.len() == stuck.levels.len() {
                rec_levels.remove(0);
            }
        }
        let refl = app(
            Expr::Const("Eq.refl".to_owned(), vec![level.clone()]),
            [ty.clone(), stuck.major.clone()],
        );
        Ok(app(
            Expr::Const(format!("{}.rec", stuck.induct), rec_levels),
            stuck
                .params
                .iter()
                .cloned()
                .chain([motive])
                .chain(minors)
                .chain([stuck.major.clone(), refl]),
        ))
    }

    /// A proof of `statement`. An equation is proved directly; any other
    /// proposition through `decide`, as `@of_decide_eq_true p inst _`,
    /// with `decidable` its instance.
    ///
    /// # Errors
    ///
    /// Refuses a case evaluation cannot close, or one past the budget.
    pub fn prove(&self, statement: &Expr, decidable: Option<Expr>) -> R<Expr> {
        let g = self.whnf(statement)?;
        if matches!(spine(&g).0, Expr::Const(ref n, _) if n == "Eq") {
            return self.solve(statement, &[]);
        }
        let inst = decidable.ok_or("the statement is neither an equation nor decidable")?;
        let decided = app(
            c("Decidable.decide"),
            [statement.clone(), inst.clone()],
        );
        let goal = app(
            Expr::Const("Eq".to_owned(), vec![Level::Succ(Rc::new(Level::Zero))]),
            [c("Bool"), decided, c("Bool.true")],
        );
        let proof = self.solve(&goal, &[])?;
        Ok(app(c("of_decide_eq_true"), [statement.clone(), inst, proof]))
    }
}

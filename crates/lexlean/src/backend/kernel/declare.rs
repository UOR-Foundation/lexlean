//! A project's own declarations as kernel declarations (SPEC.md §18.10),
//! written from the linked IR with no elaborator, so that kernel-term
//! production can run where no Lean has compiled the project: a browser tab
//! that holds only the pinned library.
//!
//! Every type, constructor, projection, definition, and statement is lowered
//! by the same [`World`] that types produced theorems, so a construct has
//! one kernel meaning here and in production. A structure or inductive type
//! is an inductive declaration whose recursor the kernel generates; its
//! `casesOn`, the one eliminator a lowered `match` names, is defined from
//! that recursor, as Lean defines it. Structural recursion is written through
//! the recursor of the argument's type, each recursive call on a field
//! becoming that field's induction hypothesis. Each theorem's statement is
//! the proposition `LexStmt.T`, so a proof found outside LexLean has a fixed
//! statement to prove.
//!
//! A declaration of a kind this does not write is refused with its reason,
//! and so is every declaration that names it; nothing is approximated.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::rc::Rc;

use super::export::Tables;
use super::expr::{app, c, cu, lam, pi, sort, Expr, Level};
use super::lower::{absolute, substitute, Ctx, World, R};
use crate::ir::semantic::{
    SemanticConstructor, SemanticDeclaration, SemanticField, SemanticNestedMember,
    SemanticParameter, SemanticProof, SemanticTerm, SemanticType,
};

/// The namespace of theorem statements: the statement of `T` is `LexStmt.T`.
pub const STATEMENT: &str = "LexStmt";

/// A declaration that was not written, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    pub name: String,
    pub reason: String,
}

/// What [`write`] declared.
#[derive(Debug, Default)]
pub struct Declared {
    /// Each theorem whose statement was declared, and that statement.
    pub statements: BTreeMap<String, String>,
    pub refused: Vec<Refusal>,
}

/// Why one declaration was not written in this pass.
enum Stop {
    /// It names a project declaration not written yet.
    Waits(String),
    Refused(String),
}

impl From<String> for Stop {
    fn from(reason: String) -> Self {
        Self::Refused(reason)
    }
}

/// One declaration to write: its module and Lean name, and with it the
/// other members of its mutual group, which are written together.
struct Pending<'a> {
    module: usize,
    lean: String,
    declaration: &'a SemanticDeclaration,
    group: Vec<(String, &'a SemanticDeclaration)>,
}

/// The mutual group a type or definition belongs to.
fn mutual_of(declaration: &SemanticDeclaration) -> Option<&str> {
    match declaration {
        SemanticDeclaration::Inductive { mutual, .. }
        | SemanticDeclaration::Definition { mutual, .. } => mutual.as_deref(),
        SemanticDeclaration::Structure { .. }
        | SemanticDeclaration::Class { .. }
        | SemanticDeclaration::Instance { .. }
        | SemanticDeclaration::Theorem { .. }
        | SemanticDeclaration::Artifact { .. }
        | SemanticDeclaration::Contract { .. }
        | SemanticDeclaration::Realization { .. }
        | SemanticDeclaration::Evidence { .. }
        | SemanticDeclaration::Model { .. }
        | SemanticDeclaration::Logic { .. }
        | SemanticDeclaration::InferenceRule { .. }
        | SemanticDeclaration::Verifier { .. }
        | SemanticDeclaration::Reasoner { .. }
        | SemanticDeclaration::Tokenizer { .. }
        | SemanticDeclaration::ContextWindow { .. }
        | SemanticDeclaration::TextGenerator { .. }
        | SemanticDeclaration::TextClassifier { .. } => None,
    }
}

/// Writes every declaration of the project the kernel can take as it stands,
/// in dependency order, into `tables`. With `checked`, a constant outside the
/// project must be one `tables` declares.
pub fn write(world: &World<'_>, tables: &mut Tables, checked: bool) -> Declared {
    let mut pending = Vec::new();
    let mut declared = Declared::default();
    for (module, kernel_module) in world.modules.iter().enumerate() {
        let mut here: Vec<&SemanticDeclaration> = kernel_module.semantic.lowered_declarations();
        // A snapshot carries each model declaration's elaboration beside it;
        // an elaborated declaration stands for the source one of its name.
        for elaborated in &kernel_module.elaborated {
            match here.iter().position(|d| d.name() == elaborated.name()) {
                Some(at) => here[at] = elaborated,
                None => here.push(elaborated),
            }
        }
        let elaborates = !kernel_module.elaborated.is_empty();
        for declaration in here {
            let lean = format!("{}.{}", kernel_module.lean_module, declaration.name());
            match declaration {
                SemanticDeclaration::Structure { .. }
                | SemanticDeclaration::Inductive { .. }
                | SemanticDeclaration::Definition { .. }
                | SemanticDeclaration::Theorem { .. } => {
                    // A member of a mutual group joins the group's first member.
                    let joined = mutual_of(declaration).and_then(|group| {
                        pending.iter_mut().find(|p: &&mut Pending<'_>| {
                            p.module == module
                                && mutual_of(p.declaration) == Some(group)
                                && std::mem::discriminant(p.declaration)
                                    == std::mem::discriminant(declaration)
                        })
                    });
                    match joined {
                        Some(first) => first.group.push((lean, declaration)),
                        None => pending.push(Pending {
                            module,
                            group: vec![(lean.clone(), declaration)],
                            lean,
                            declaration,
                        }),
                    }
                }
                SemanticDeclaration::Class { .. } | SemanticDeclaration::Instance { .. } => {
                    declared.refused.push(Refusal {
                        name: lean,
                        reason: format!(
                            "a {} is not among the declarations written here",
                            kind(declaration)
                        ),
                    });
                }
                // A model, reasoning, or text declaration is written as what it elaborates to.
                other @ (SemanticDeclaration::Artifact { .. }
                | SemanticDeclaration::Contract { .. }
                | SemanticDeclaration::Realization { .. }
                | SemanticDeclaration::Evidence { .. }
                | SemanticDeclaration::Model { .. }
                | SemanticDeclaration::Logic { .. }
                | SemanticDeclaration::InferenceRule { .. }
                | SemanticDeclaration::Verifier { .. }
                | SemanticDeclaration::Reasoner { .. }
                | SemanticDeclaration::Tokenizer { .. }
                | SemanticDeclaration::ContextWindow { .. }
                | SemanticDeclaration::TextGenerator { .. }
                | SemanticDeclaration::TextClassifier { .. }) => {
                    if !elaborates {
                        declared.refused.push(Refusal {
                            name: lean,
                            reason: format!("a {} without its elaboration", kind(other)),
                        });
                    }
                }
            }
        }
    }
    let projects: Vec<String> = world
        .modules
        .iter()
        .map(|module| format!("{}.", module.lean_module))
        .collect();
    let mut heights: HashMap<String, u32> = HashMap::new();
    // Each pass writes what no longer waits on an unwritten declaration.
    loop {
        let mut progress = false;
        let mut waiting = Vec::new();
        for item in pending {
            let mark = tables.checkpoint();
            let mut writer = Writer {
                world,
                tables: &mut *tables,
                checked,
                projects: &projects,
                heights: &mut heights,
            };
            match writer.declaration(&item) {
                Ok(statement) => {
                    progress = true;
                    if let Some(statement) = statement {
                        declared.statements.insert(item.lean.clone(), statement);
                    }
                }
                Err(Stop::Waits(name)) => {
                    tables.rewind(&mark);
                    waiting.push((item, name));
                }
                Err(Stop::Refused(reason)) => {
                    tables.rewind(&mark);
                    for (name, _) in item.group {
                        declared.refused.push(Refusal {
                            name,
                            reason: reason.clone(),
                        });
                    }
                }
            }
        }
        if waiting.is_empty() || !progress {
            for (item, name) in waiting {
                for (member, _) in item.group {
                    declared.refused.push(Refusal {
                        name: member,
                        reason: format!("it names `{name}`, which is not declared"),
                    });
                }
            }
            break;
        }
        pending = waiting.into_iter().map(|(item, _)| item).collect();
    }
    declared
}

fn kind(declaration: &SemanticDeclaration) -> String {
    serde_json::to_value(declaration)
        .ok()
        .and_then(|value| {
            value
                .get("kind")
                .and_then(|k| k.as_str().map(str::to_owned))
        })
        .unwrap_or_default()
}

/// A binder: its name, free variable, and type.
type Binder = (String, u32, Expr);

/// One member of a recursor's family: a declared type of the group, or a
/// nested occurrence of one inside a library type.
struct Member {
    lean: Option<String>,
    ty: SemanticType,
    expr: Expr,
    rec: String,
    /// The type's own universe levels, after the motive's.
    rec_levels: Vec<u32>,
    ctors: Vec<FamilyCtor>,
}

/// A constructor of a family member, its fields, and for each field the
/// member it is of, if any.
struct FamilyCtor {
    constant: String,
    short: String,
    levels: Vec<u32>,
    params: Vec<Expr>,
    fields: Vec<SemanticType>,
    member: Vec<Option<usize>>,
}

/// The types a recursor ranges over: the declared ones first.
struct Family {
    members: Vec<Member>,
    declared: usize,
    params: Vec<Expr>,
}

/// One function of a recursive group.
struct Function<'d> {
    lean: String,
    type_parameters: &'d [String],
    parameters: &'d [SemanticParameter],
    result: &'d SemanticType,
    k: Option<usize>,
    body: &'d SemanticTerm,
    measure: Option<&'d SemanticTerm>,
}

/// The recursor of family member `of` at the motive universe `level`,
/// applied to the family's parameters, the motives, and the minors.
fn recursor(
    family: &Family,
    of: usize,
    level: Level,
    motives: Vec<Expr>,
    minors: Vec<Expr>,
) -> Expr {
    let member = &family.members[of];
    let mut levels = vec![level];
    levels.extend(member.rec_levels.iter().map(|l| Level::of(*l)));
    let mut arguments = family.params.clone();
    arguments.extend(motives);
    arguments.extend(minors);
    app(Expr::Const(member.rec.clone(), levels), arguments)
}

/// A family constructor applied to its fields.
fn built(ctor: &FamilyCtor, fields: Vec<Expr>) -> Expr {
    app(
        app(cu(&ctor.constant, &ctor.levels), ctor.params.clone()),
        fields,
    )
}

/// The type of a tuple of values of `tys`: one type alone, else nested pairs.
fn tuple_ty(tys: &[Expr]) -> Expr {
    match tys {
        [] => c("Unit"),
        [one] => one.clone(),
        [first, rest @ ..] => app(cu("Prod", &[0, 0]), [first.clone(), tuple_ty(rest)]),
    }
}

/// A tuple of `values` of `tys`.
fn tuple(values: Vec<Expr>, tys: &[Expr]) -> Expr {
    let mut values = values;
    if values.len() <= 1 {
        return values.pop().unwrap_or_else(|| c("Unit.unit"));
    }
    let first = values.remove(0);
    app(
        cu("Prod.mk", &[0, 0]),
        [
            tys[0].clone(),
            tuple_ty(&tys[1..]),
            first,
            tuple(values, &tys[1..]),
        ],
    )
}

/// Component `i` of a tuple `t` of `tys`.
fn select(t: Expr, i: usize, tys: &[Expr]) -> Expr {
    if tys.len() <= 1 {
        return t;
    }
    let (a, rest) = (tys[0].clone(), tuple_ty(&tys[1..]));
    if i == 0 {
        app(cu("Prod.fst", &[0, 0]), [a, rest, t])
    } else {
        select(app(cu("Prod.snd", &[0, 0]), [a, rest, t]), i - 1, &tys[1..])
    }
}

fn close_pi(binders: &[Binder], body: Expr) -> Expr {
    binders.iter().rev().fold(body, |body, (name, id, ty)| {
        pi(name, *id, ty.clone(), &body)
    })
}

fn close_lam(binders: &[Binder], body: Expr) -> Expr {
    binders.iter().rev().fold(body, |body, (name, id, ty)| {
        lam(name, *id, ty.clone(), &body)
    })
}

fn fvars(binders: &[Binder]) -> Vec<Expr> {
    binders.iter().map(|(_, id, _)| Expr::FVar(*id)).collect()
}

/// The universe a value of `ty` lives in, as the motive of an eliminator
/// into it is sorted: a proposition in `Prop`, a type in `Type 1`, data in
/// `Type`.
fn level_of(ty: &SemanticType) -> u32 {
    match ty {
        SemanticType::Array { .. } => 1,
        SemanticType::Prop => 0,
        SemanticType::Type => 2,
        SemanticType::Parameter { .. }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::Option { .. }
        | SemanticType::Result { .. }
        | SemanticType::List { .. }
        | SemanticType::Named { .. }
        | SemanticType::Product { .. }
        | SemanticType::Function { .. }
        | SemanticType::Map { .. }
        | SemanticType::Set { .. }
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => 1,
    }
}

/// Whether `ty` mentions the document type `name` of module `module`
/// anywhere inside it.
fn mentions(ty: &SemanticType, module: &str, name: &str) -> bool {
    let m = |inner: &SemanticType| mentions(inner, module, name);
    match ty {
        SemanticType::Array { element } => m(element),
        SemanticType::Named { member, arguments } => {
            (member.name == name && member.module.as_deref() == Some(module))
                || arguments.iter().any(m)
        }
        SemanticType::Option { value } => m(value),
        SemanticType::Result { ok, error } => m(ok) || m(error),
        SemanticType::List { element } | SemanticType::Set { element } => m(element),
        SemanticType::Product { left, right } => m(left) || m(right),
        SemanticType::Function { parameters, result } => parameters.iter().any(m) || m(result),
        SemanticType::Map { key, value } => m(key) || m(value),
        SemanticType::Type
        | SemanticType::Parameter { .. }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
        | SemanticType::Int
        | SemanticType::Int8
        | SemanticType::Int16
        | SemanticType::Int32
        | SemanticType::Int64
        | SemanticType::UInt8
        | SemanticType::UInt16
        | SemanticType::UInt32
        | SemanticType::UInt64
        | SemanticType::String
        | SemanticType::Bytes
        | SemanticType::Ordering
        | SemanticType::ContractViolation
        | SemanticType::ReasoningFailure => false,
    }
}

/// `e` with every application whose head is a constant offered to `f`
/// replaced by what `f` returns for it, innermost arguments first.
fn rewrite(e: &Expr, f: &mut dyn FnMut(&str, &[Expr]) -> Option<Expr>) -> Expr {
    match e {
        Expr::App(..) | Expr::Const(..) => {
            let mut args = Vec::new();
            let mut head = e;
            while let Expr::App(function, argument) = head {
                args.push(rewrite(argument, f));
                head = function;
            }
            args.reverse();
            if let Expr::Const(name, _) = head {
                if let Some(replaced) = f(name, &args) {
                    return replaced;
                }
                return app(head.clone(), args);
            }
            app(rewrite(head, f), args)
        }
        Expr::Lam(name, ty, body) => Expr::Lam(
            name.clone(),
            Rc::new(rewrite(ty, f)),
            Rc::new(rewrite(body, f)),
        ),
        Expr::Pi(name, ty, body) => Expr::Pi(
            name.clone(),
            Rc::new(rewrite(ty, f)),
            Rc::new(rewrite(body, f)),
        ),
        Expr::Let(name, ty, value, body) => Expr::Let(
            name.clone(),
            Rc::new(rewrite(ty, f)),
            Rc::new(rewrite(value, f)),
            Rc::new(rewrite(body, f)),
        ),
        Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Nat(_) | Expr::Str(_) => e.clone(),
    }
}

struct Writer<'w, 'a> {
    world: &'w World<'a>,
    tables: &'w mut Tables,
    checked: bool,
    projects: &'w [String],
    heights: &'w mut HashMap<String, u32>,
}

impl Writer<'_, '_> {
    /// Writes one declaration, and returns the statement it declared for a
    /// theorem.
    fn declaration(&mut self, item: &Pending<'_>) -> Result<Option<String>, Stop> {
        match item.declaration {
            SemanticDeclaration::Structure {
                type_parameters,
                parameters,
                fields,
                ..
            } => {
                if !parameters.is_empty() {
                    return Err(Stop::Refused(
                        "a structure with value parameters is not among the declarations written here".to_owned(),
                    ));
                }
                self.structure(item, type_parameters, fields)?;
                Ok(None)
            }
            SemanticDeclaration::Inductive {
                type_parameters,
                parameters,
                constructors,
                mutual,
                ..
            } => {
                if !parameters.is_empty() {
                    return Err(Stop::Refused(
                        "an inductive with value parameters is not among the declarations written here".to_owned(),
                    ));
                }
                if mutual.is_some() {
                    self.inductives(item, type_parameters)?;
                    return Ok(None);
                }
                self.inductive(item, type_parameters, constructors)?;
                Ok(None)
            }
            SemanticDeclaration::Definition {
                type_parameters,
                parameters,
                result,
                recursive_argument,
                body,
                mutual,
                termination,
                ..
            } => {
                if termination.is_some() {
                    self.fuel_group(item)?;
                    return Ok(None);
                }
                if mutual.is_some() {
                    self.structural_group(item)?;
                    return Ok(None);
                }
                // Recursion on a type whose recursor ranges over more than it: a nested or mutual family.
                if let Some(argument) = recursive_argument {
                    let mut ctx = Ctx::new(item.module);
                    self.type_binders(&mut ctx, type_parameters);
                    let module = self.world.modules[item.module].name.clone();
                    let ty = parameters
                        .iter()
                        .find(|p| p.name == *argument)
                        .map(|p| absolute(&p.r#type, &module));
                    if let Some(ty) = ty {
                        if self
                            .family_of(&ctx, &ty)
                            .is_ok_and(|family| family.members.len() > 1)
                        {
                            self.structural_group(item)?;
                            return Ok(None);
                        }
                    }
                }
                self.definition(
                    item,
                    type_parameters,
                    parameters,
                    result,
                    recursive_argument.as_deref(),
                    body,
                )?;
                Ok(None)
            }
            SemanticDeclaration::Theorem {
                type_parameters,
                parameters,
                statement,
                proof,
                ..
            } => {
                let name = self.statement(item, type_parameters, parameters, statement)?;
                if let SemanticProof::NestedInduction {
                    scrutinee, members, ..
                } = proof
                {
                    self.member_statements(item, type_parameters, parameters, scrutinee, members);
                }
                Ok(Some(name))
            }
            other @ (SemanticDeclaration::Class { .. }
            | SemanticDeclaration::Instance { .. }
            | SemanticDeclaration::Artifact { .. }
            | SemanticDeclaration::Contract { .. }
            | SemanticDeclaration::Realization { .. }
            | SemanticDeclaration::Evidence { .. }
            | SemanticDeclaration::Model { .. }
            | SemanticDeclaration::Logic { .. }
            | SemanticDeclaration::InferenceRule { .. }
            | SemanticDeclaration::Verifier { .. }
            | SemanticDeclaration::Reasoner { .. }
            | SemanticDeclaration::Tokenizer { .. }
            | SemanticDeclaration::ContextWindow { .. }
            | SemanticDeclaration::TextGenerator { .. }
            | SemanticDeclaration::TextClassifier { .. }) => Err(Stop::Refused(format!(
                "a {} is not among the declarations written here",
                kind(other)
            ))),
        }
    }

    /// Every constant the terms name must be declared: a project one waits
    /// for its declaration, and with `checked`, any other one must be the
    /// library's. Returns the dependency height of a definition over them.
    fn names(&self, terms: &[&Expr]) -> Result<u32, Stop> {
        let mut named = BTreeSet::new();
        for term in terms {
            term.constants(&mut named);
        }
        let mut height = 0;
        for name in named {
            if self.tables.declares(&name) {
                height = height.max(self.heights.get(&name).copied().unwrap_or(0));
            } else if self
                .projects
                .iter()
                .any(|prefix| name.starts_with(prefix.as_str()))
            {
                return Err(Stop::Waits(name));
            } else if self.checked {
                return Err(Stop::Refused(format!(
                    "the library does not declare `{name}`"
                )));
            }
        }
        Ok(height + 1)
    }

    fn define(
        &mut self,
        name: &str,
        levels: &[String],
        ty: &Expr,
        value: &Expr,
    ) -> Result<(), Stop> {
        let height = self.names(&[ty, value])?;
        self.tables.definition(name, levels, ty, value, height)?;
        self.heights.insert(name.to_owned(), height);
        Ok(())
    }

    /// Each type parameter bound as `(α : Type)`, for the lowering to read.
    fn type_binders(&self, ctx: &mut Ctx, names: &[String]) -> Vec<Binder> {
        names
            .iter()
            .map(|name| {
                let id = self.world.fresh();
                ctx.types.push((name.clone(), Expr::FVar(id)));
                (name.clone(), id, sort(1))
            })
            .collect()
    }

    /// Each parameter bound for the lowering to read, after `binders`.
    fn value_binders(
        &self,
        ctx: &mut Ctx,
        module: usize,
        parameters: &[SemanticParameter],
    ) -> R<Vec<Binder>> {
        let mut out = Vec::new();
        for parameter in parameters {
            let id = self.world.fresh();
            out.push((
                parameter.name.clone(),
                id,
                self.world.ty(ctx, &parameter.r#type)?,
            ));
            ctx.push(
                &parameter.name,
                Expr::FVar(id),
                Some(absolute(
                    &parameter.r#type,
                    &self.world.modules[module].name,
                )),
            );
        }
        Ok(out)
    }

    /// The inductive declaration of a type, its `casesOn`, and nothing else.
    /// `ctors` are `(constant, field types, which fields are the type)`.
    fn inductive_type(
        &mut self,
        lean: &str,
        tb: &[Binder],
        ctors: &[(String, Vec<Expr>, Vec<bool>)],
    ) -> Result<(), Stop> {
        let t = app(c(lean), fvars(tb));
        let mut rows = Vec::new();
        for (constant, fields, _) in ctors {
            let binders: Vec<Binder> = fields
                .iter()
                .enumerate()
                .map(|(i, ty)| (format!("a{i}"), self.world.fresh(), ty.clone()))
                .collect();
            rows.push((
                constant.clone(),
                close_pi(tb, close_pi(&binders, t.clone())),
                fields.len(),
            ));
        }
        let ty = close_pi(tb, sort(1));
        // A field type outside the project must be the library's; the type
        // itself, and its constructors, are being declared.
        for (_, ctor_ty, _) in &rows {
            let mut named = BTreeSet::new();
            ctor_ty.constants(&mut named);
            for name in named {
                if name == lean || self.tables.declares(&name) {
                    continue;
                }
                if self
                    .projects
                    .iter()
                    .any(|prefix| name.starts_with(prefix.as_str()))
                {
                    return Err(Stop::Waits(name));
                }
                if self.checked {
                    return Err(Stop::Refused(format!(
                        "the library does not declare `{name}`"
                    )));
                }
            }
        }
        let recursive = ctors.iter().any(|(_, _, own)| own.iter().any(|o| *o));
        self.tables
            .inductive(&[(lean.to_owned(), ty, recursive, rows)], tb.len(), 0)?;
        self.cases_on(lean, tb, ctors)
    }

    /// `T.casesOn` as Lean defines it from `T.rec`, over one universe `u`:
    /// `Π params {motive : T → Sort u} (t : T), (Π fields, motive (C fields))… → motive t`
    /// `:= fun params motive t minors => T.rec params motive (fun fields hyps => minor fields)… t`.
    fn cases_on(
        &mut self,
        lean: &str,
        tb: &[Binder],
        ctors: &[(String, Vec<Expr>, Vec<bool>)],
    ) -> Result<(), Stop> {
        let u = Level::Param("u".to_owned());
        let params = fvars(tb);
        let t = app(c(lean), params.clone());
        let motive = self.world.fresh();
        let motive_ty = pi("t", self.world.fresh(), t.clone(), &Expr::Sort(u.clone()));
        let major = self.world.fresh();
        let mut minors: Vec<Binder> = Vec::new();
        let mut recursor_minors = Vec::new();
        for (i, (constant, fields, own)) in ctors.iter().enumerate() {
            let binders: Vec<Binder> = fields
                .iter()
                .enumerate()
                .map(|(j, ty)| (format!("a{j}"), self.world.fresh(), ty.clone()))
                .collect();
            let built = app(app(c(constant), params.clone()), fvars(&binders));
            let minor = self.world.fresh();
            minors.push((
                format!("m{i}"),
                minor,
                close_pi(&binders, app(Expr::FVar(motive), [built])),
            ));
            let hypotheses: Vec<Binder> = binders
                .iter()
                .zip(own)
                .filter(|(_, own)| **own)
                .map(|((name, id, _), _)| {
                    (
                        format!("{name}_ih"),
                        self.world.fresh(),
                        app(Expr::FVar(motive), [Expr::FVar(*id)]),
                    )
                })
                .collect();
            let mut all = binders.clone();
            all.extend(hypotheses);
            recursor_minors.push(close_lam(&all, app(Expr::FVar(minor), fvars(&binders))));
        }
        let mut binders = vec![
            ("motive".to_owned(), motive, motive_ty),
            ("t".to_owned(), major, t),
        ];
        binders.extend(minors);
        let ty = close_pi(
            tb,
            close_pi(&binders, app(Expr::FVar(motive), [Expr::FVar(major)])),
        );
        let mut arguments = params;
        arguments.push(Expr::FVar(motive));
        arguments.extend(recursor_minors);
        arguments.push(Expr::FVar(major));
        let value = close_lam(
            tb,
            close_lam(
                &binders,
                app(Expr::Const(format!("{lean}.rec"), vec![u]), arguments),
            ),
        );
        self.define(&format!("{lean}.casesOn"), &["u".to_owned()], &ty, &value)
    }

    /// A structure: one constructor `T.mk` over its fields, and each field's
    /// projection `T.f := fun params self => T.casesOn self (fun fields => f)`.
    fn structure(
        &mut self,
        item: &Pending<'_>,
        type_parameters: &[String],
        fields: &[SemanticField],
    ) -> Result<(), Stop> {
        let mut ctx = Ctx::new(item.module);
        let tb = self.type_binders(&mut ctx, type_parameters);
        let mut field_tys = Vec::new();
        for field in fields {
            field_tys.push(self.world.ty(&ctx, &field.r#type)?);
        }
        let mk = format!("{}.mk", item.lean);
        self.inductive_type(
            &item.lean,
            &tb,
            &[(mk, field_tys.clone(), vec![false; fields.len()])],
        )?;
        let t = app(c(&item.lean), fvars(&tb));
        for (index, field) in fields.iter().enumerate() {
            let this = self.world.fresh();
            let binders: Vec<Binder> = field_tys
                .iter()
                .enumerate()
                .map(|(j, ty)| (format!("a{j}"), self.world.fresh(), ty.clone()))
                .collect();
            let chosen = Expr::FVar(binders[index].1);
            let motive = lam("x", self.world.fresh(), t.clone(), &field_tys[index]);
            let mut arguments = fvars(&tb);
            arguments.push(motive);
            arguments.push(Expr::FVar(this));
            arguments.push(close_lam(&binders, chosen));
            let body = app(
                cu(
                    &format!("{}.casesOn", item.lean),
                    &[level_of(&field.r#type)],
                ),
                arguments,
            );
            let self_binder = [("self".to_owned(), this, t.clone())];
            let ty = close_pi(&tb, close_pi(&self_binder, field_tys[index].clone()));
            let value = close_lam(&tb, close_lam(&self_binder, body));
            self.define(&format!("{}.{}", item.lean, field.name), &[], &ty, &value)?;
        }
        Ok(())
    }

    /// An inductive type: its constructors over their fields. A field that
    /// holds the type inside another type is a nested inductive, whose
    /// recursor Lean builds through an auxiliary type, and is refused.
    fn inductive(
        &mut self,
        item: &Pending<'_>,
        type_parameters: &[String],
        constructors: &[SemanticConstructor],
    ) -> Result<(), Stop> {
        let mut ctx = Ctx::new(item.module);
        let tb = self.type_binders(&mut ctx, type_parameters);
        let module = self.world.modules[item.module].name.clone();
        let short = item.declaration.name().to_owned();
        let mut ctors = Vec::new();
        for constructor in constructors {
            let mut fields = Vec::new();
            let mut own = Vec::new();
            for field in &constructor.fields {
                let field = absolute(field, &module);
                let is_own = matches!(&field, SemanticType::Named { member, .. }
                    if member.name == short && member.module.as_deref() == Some(module.as_str()));
                // The type inside another type: a nested inductive, whose
                // recursors range over the occurrences too.
                if !is_own && mentions(&field, &module, &short) {
                    return self.inductives(item, type_parameters);
                }
                own.push(is_own);
                fields.push(if is_own {
                    app(c(&item.lean), fvars(&tb))
                } else {
                    self.world.ty(&ctx, &field)?
                });
            }
            ctors.push((format!("{}.{}", item.lean, constructor.name), fields, own));
        }
        self.inductive_type(&item.lean, &tb, &ctors)
    }

    /// A definition. Without recursion it is its body; with structural
    /// recursion on argument `k`, whose body must be a match on it, it is
    /// `fun params => T.rec (fun _ => Π others, R) minors k others`, where
    /// each minor rebinds the other parameters and a recursive call on a
    /// field of the branch becomes that field's hypothesis.
    fn definition(
        &mut self,
        item: &Pending<'_>,
        type_parameters: &[String],
        parameters: &[SemanticParameter],
        result: &SemanticType,
        recursive_argument: Option<&str>,
        body: &SemanticTerm,
    ) -> Result<(), Stop> {
        let mut ctx = Ctx::new(item.module);
        let tb = self.type_binders(&mut ctx, type_parameters);
        let outer = self.value_binders(&mut ctx, item.module, parameters)?;
        let result_ty = self.world.ty(&ctx, result)?;
        let mut all = tb.clone();
        all.extend(outer.iter().cloned());
        let ty = close_pi(&all, result_ty.clone());
        let value = match recursive_argument {
            None => self.world.term(&ctx, body)?,
            Some(argument) => self.recursion(
                item,
                &ctx,
                tb.len(),
                parameters,
                &outer,
                result,
                &result_ty,
                argument,
                body,
            )?,
        };
        let value = close_lam(&all, value);
        self.define(&item.lean, &[], &ty, &value)
    }

    #[allow(clippy::too_many_arguments)]
    fn recursion(
        &mut self,
        item: &Pending<'_>,
        ctx: &Ctx,
        generic: usize,
        parameters: &[SemanticParameter],
        outer: &[Binder],
        result: &SemanticType,
        result_ty: &Expr,
        argument: &str,
        body: &SemanticTerm,
    ) -> Result<Expr, Stop> {
        let not_a_match =
            || Stop::Refused("recursion whose body is not a match on its argument".to_owned());
        let k = parameters
            .iter()
            .position(|p| p.name == argument)
            .ok_or_else(|| Stop::Refused("the recursive argument is not a parameter".to_owned()))?;
        let SemanticTerm::Match {
            scrutinee,
            branches,
        } = body
        else {
            return Err(not_a_match());
        };
        if !matches!(scrutinee.as_ref(), SemanticTerm::Var { name } if name == argument) {
            return Err(not_a_match());
        }
        let module = self.world.modules[item.module].name.clone();
        let others: Vec<usize> = (0..parameters.len()).filter(|i| *i != k).collect();
        let motive_body = {
            let binders: Vec<Binder> = others
                .iter()
                .map(|i| (outer[*i].0.clone(), self.world.fresh(), outer[*i].2.clone()))
                .collect();
            close_pi(&binders, result_ty.clone())
        };
        let shape = self
            .world
            .shape(ctx, &absolute(&parameters[k].r#type, &module))?;
        let mut minors = Vec::new();
        for ctor in &shape.ctors {
            let branch = branches
                .iter()
                .find(|b| b.constructor.name.rsplit('.').next() == Some(ctor.short.as_str()))
                .ok_or_else(|| Stop::Refused(format!("a match without `{}`", ctor.short)))?;
            if branch.binders.len() != ctor.fields.len() {
                return Err(Stop::Refused(format!(
                    "branch `{}` binds {} fields",
                    ctor.short,
                    branch.binders.len()
                )));
            }
            let mut inner = ctx.clone();
            let mut fields: Vec<Binder> = Vec::new();
            let mut hypotheses: Vec<Binder> = Vec::new();
            let mut hypothesis_of: HashMap<u32, u32> = HashMap::new();
            for ((binder, field), own) in
                branch.binders.iter().zip(&ctor.fields).zip(&ctor.recursive)
            {
                let id = self.world.fresh();
                fields.push((binder.clone(), id, self.world.ty(ctx, field)?));
                inner.push(binder, Expr::FVar(id), Some(field.clone()));
                if *own {
                    let ih = self.world.fresh();
                    hypotheses.push((format!("{binder}_ih"), ih, motive_body.clone()));
                    hypothesis_of.insert(id, ih);
                }
            }
            // The argument itself, inside its branch, is the constructor applied.
            let built = app(
                app(cu(&ctor.constant, &shape.ctor_levels), shape.params.clone()),
                fvars(&fields),
            );
            inner.push(
                argument,
                built,
                Some(absolute(&parameters[k].r#type, &module)),
            );
            let mut mine: Vec<Binder> = Vec::new();
            for i in &others {
                let id = self.world.fresh();
                mine.push((outer[*i].0.clone(), id, outer[*i].2.clone()));
                inner.push(
                    &parameters[*i].name,
                    Expr::FVar(id),
                    Some(absolute(&parameters[*i].r#type, &module)),
                );
            }
            let lowered = self.world.term(&inner, &branch.body)?;
            let mut stray = false;
            let rewritten = rewrite(&lowered, &mut |head, args| {
                if head != item.lean {
                    return None;
                }
                let ih = match args.get(generic + k) {
                    Some(Expr::FVar(id)) => hypothesis_of.get(id).copied(),
                    _ => None,
                };
                let Some(ih) = ih else {
                    stray = true;
                    return None;
                };
                let rest = args
                    .iter()
                    .enumerate()
                    .skip(generic)
                    .filter(|(i, _)| *i != generic + k)
                    .map(|(_, a)| a.clone());
                Some(app(Expr::FVar(ih), rest))
            });
            if stray {
                return Err(Stop::Refused(
                    "a recursive call that is not on a structurally smaller argument".to_owned(),
                ));
            }
            let mut binders = fields;
            binders.extend(hypotheses);
            minors.push(close_lam(&binders, close_lam(&mine, rewritten)));
        }
        let motive = lam("x", self.world.fresh(), outer[k].2.clone(), &motive_body);
        let mut levels = vec![level_of(result)];
        levels.extend(&shape.levels);
        let mut arguments = shape.params.clone();
        arguments.push(motive);
        arguments.extend(minors);
        arguments.push(Expr::FVar(outer[k].1));
        arguments.extend(others.iter().map(|i| Expr::FVar(outer[*i].1)));
        Ok(app(cu(&shape.rec, &levels), arguments))
    }

    /// The family a recursor of `ty` ranges over, in the kernel's order: the
    /// types of its inductive group (one, or a mutual group), then each nested
    /// occurrence their fields make (`List T`, `Option T`, ...) as an
    /// auxiliary member, in the order the fields first name it. A library
    /// type (`Nat`, `List α`) is a family of itself.
    fn family_of(&self, ctx: &Ctx, ty: &SemanticType) -> R<Family> {
        let SemanticType::Named { member, arguments } = ty else {
            let shape = self.world.shape(ctx, ty)?;
            return Ok(Family {
                members: vec![Member {
                    lean: None,
                    expr: self.world.ty(ctx, ty)?,
                    ty: ty.clone(),
                    rec: shape.rec.clone(),
                    rec_levels: shape.levels.clone(),
                    ctors: shape
                        .ctors
                        .iter()
                        .map(|ctor| FamilyCtor {
                            constant: ctor.constant.clone(),
                            short: ctor.short.clone(),
                            levels: shape.ctor_levels.clone(),
                            params: shape.params.clone(),
                            fields: ctor.fields.clone(),
                            member: ctor.recursive.iter().map(|r| r.then_some(0)).collect(),
                        })
                        .collect(),
                }],
                declared: 1,
                params: shape.params,
            });
        };
        let (module, declaration) = self.world.find(ctx.module, member)?;
        let group: Vec<&SemanticDeclaration> = match declaration {
            SemanticDeclaration::Inductive {
                mutual: Some(label),
                ..
            } => self.world.modules[module]
                .semantic
                .declarations
                .iter()
                .filter(|d| matches!(d, SemanticDeclaration::Inductive { mutual: Some(other), .. } if other == label))
                .collect(),
            other => vec![other],
        };
        let module_name = self.world.modules[module].name.clone();
        let lean_module = self.world.modules[module].lean_module.clone();
        let mut params = Vec::new();
        for argument in arguments {
            params.push(self.world.ty(ctx, argument)?);
        }
        let mut members = Vec::new();
        let mut owners = Vec::new();
        for declaration in &group {
            let (type_parameters, ctors): (&[String], Vec<(String, Vec<SemanticType>)>) =
                match declaration {
                    SemanticDeclaration::Inductive {
                        type_parameters,
                        constructors,
                        ..
                    } => (
                        type_parameters,
                        constructors
                            .iter()
                            .map(|k| (k.name.clone(), k.fields.clone()))
                            .collect(),
                    ),
                    SemanticDeclaration::Structure {
                        type_parameters,
                        fields,
                        ..
                    } => (
                        type_parameters,
                        vec![(
                            "mk".to_owned(),
                            fields.iter().map(|f| f.r#type.clone()).collect(),
                        )],
                    ),
                    other => return Err(format!("`{}` is not an inductive type", other.name())),
                };
            let by: HashMap<String, SemanticType> = type_parameters
                .iter()
                .cloned()
                .zip(arguments.iter().cloned())
                .collect();
            let lean = format!("{lean_module}.{}", declaration.name());
            let ty = SemanticType::Named {
                member: crate::ir::semantic::MemberRef {
                    module: Some(module_name.clone()),
                    name: declaration.name().to_owned(),
                },
                arguments: arguments.clone(),
            };
            owners.push(declaration.name().to_owned());
            members.push(Member {
                lean: Some(lean.clone()),
                expr: self.world.ty(ctx, &ty)?,
                ty,
                rec: format!("{lean}.rec"),
                rec_levels: Vec::new(),
                ctors: ctors
                    .into_iter()
                    .map(|(short, fields)| FamilyCtor {
                        constant: format!("{lean}.{short}"),
                        short,
                        levels: Vec::new(),
                        params: params.clone(),
                        fields: fields
                            .iter()
                            .map(|f| substitute(&absolute(f, &module_name), &by))
                            .collect(),
                        member: Vec::new(),
                    })
                    .collect(),
            });
        }
        let declared = members.len();
        let first = members[0].lean.clone().unwrap_or_default();
        let mut at = 0;
        while at < members.len() {
            for k in 0..members[at].ctors.len() {
                let fields = members[at].ctors[k].fields.clone();
                let mut of = Vec::new();
                for field in &fields {
                    if let Some(index) = members.iter().position(|m| m.ty == *field) {
                        of.push(Some(index));
                    } else if owners
                        .iter()
                        .any(|name| mentions(field, &module_name, name))
                    {
                        let shape = self.world.shape(ctx, field)?;
                        let index = members.len();
                        members.push(Member {
                            lean: None,
                            expr: self.world.ty(ctx, field)?,
                            ty: field.clone(),
                            rec: format!("{first}.rec_{}", index + 1 - declared),
                            rec_levels: Vec::new(),
                            ctors: shape
                                .ctors
                                .iter()
                                .map(|ctor| FamilyCtor {
                                    constant: ctor.constant.clone(),
                                    short: ctor.short.clone(),
                                    levels: shape.ctor_levels.clone(),
                                    params: shape.params.clone(),
                                    fields: ctor.fields.clone(),
                                    member: Vec::new(),
                                })
                                .collect(),
                        });
                        of.push(Some(index));
                    } else {
                        of.push(None);
                    }
                }
                members[at].ctors[k].member = of;
            }
            at += 1;
        }
        Ok(Family {
            members,
            declared,
            params,
        })
    }

    /// An inductive group, mutual or nested: one inductive declaration of
    /// its types, whose recursors the kernel generates, and each type's
    /// `casesOn`.
    fn inductives(&mut self, item: &Pending<'_>, type_parameters: &[String]) -> Result<(), Stop> {
        let mut ctx = Ctx::new(item.module);
        let tb = self.type_binders(&mut ctx, type_parameters);
        let ty = SemanticType::Named {
            member: crate::ir::semantic::MemberRef {
                module: Some(self.world.modules[item.module].name.clone()),
                name: item.declaration.name().to_owned(),
            },
            arguments: type_parameters
                .iter()
                .map(|name| SemanticType::Parameter { name: name.clone() })
                .collect(),
        };
        let family = self.family_of(&ctx, &ty)?;
        let mut types = Vec::new();
        let mut names = BTreeSet::new();
        for member in &family.members[..family.declared] {
            let lean = member.lean.clone().unwrap_or_default();
            names.insert(lean.clone());
            let mut ctors = Vec::new();
            for ctor in &member.ctors {
                names.insert(ctor.constant.clone());
                let mut binders = Vec::new();
                for (j, field) in ctor.fields.iter().enumerate() {
                    binders.push((
                        format!("a{j}"),
                        self.world.fresh(),
                        self.world.ty(&ctx, field)?,
                    ));
                }
                ctors.push((
                    ctor.constant.clone(),
                    close_pi(&tb, close_pi(&binders, member.expr.clone())),
                    ctor.fields.len(),
                ));
            }
            let recursive = member
                .ctors
                .iter()
                .any(|k| k.member.iter().any(Option::is_some));
            types.push((lean, close_pi(&tb, sort(1)), recursive, ctors));
        }
        // A field type outside the project must be the library's; the group itself is being declared.
        for (_, _, _, ctors) in &types {
            for (_, ctor_ty, _) in ctors {
                let mut named = BTreeSet::new();
                ctor_ty.constants(&mut named);
                for name in named {
                    if names.contains(&name) || self.tables.declares(&name) {
                        continue;
                    }
                    if self
                        .projects
                        .iter()
                        .any(|prefix| name.starts_with(prefix.as_str()))
                    {
                        return Err(Stop::Waits(name));
                    }
                    if self.checked {
                        return Err(Stop::Refused(format!(
                            "the library does not declare `{name}`"
                        )));
                    }
                }
            }
        }
        self.tables
            .inductive(&types, tb.len(), family.members.len() - family.declared)?;
        for j in 0..family.declared {
            self.family_cases_on(&ctx, j, &tb, &family)?;
        }
        Ok(())
    }

    /// `T.casesOn` for member `j` of a group, as Lean defines it from the
    /// group's recursor over one universe `u`: every other member's motive is
    /// `fun _ => PUnit` and its minors `PUnit.unit`.
    fn family_cases_on(
        &mut self,
        ctx: &Ctx,
        j: usize,
        tb: &[Binder],
        family: &Family,
    ) -> Result<(), Stop> {
        let u = Level::Param("u".to_owned());
        let punit = Expr::Const("PUnit".to_owned(), vec![u.clone()]);
        let unit = Expr::Const("PUnit.unit".to_owned(), vec![u.clone()]);
        let t = family.members[j].expr.clone();
        let motive = self.world.fresh();
        let motive_ty = pi("t", self.world.fresh(), t.clone(), &Expr::Sort(u.clone()));
        let major = self.world.fresh();
        let motives: Vec<Expr> = family
            .members
            .iter()
            .enumerate()
            .map(|(k, member)| {
                if k == j {
                    Expr::FVar(motive)
                } else {
                    lam("t", self.world.fresh(), member.expr.clone(), &punit)
                }
            })
            .collect();
        let mut minor_binders: Vec<Binder> = Vec::new();
        let mut recursor_minors = Vec::new();
        for (k, member) in family.members.iter().enumerate() {
            for (i, ctor) in member.ctors.iter().enumerate() {
                let mut fields: Vec<Binder> = Vec::new();
                for (f, field) in ctor.fields.iter().enumerate() {
                    fields.push((
                        format!("a{f}"),
                        self.world.fresh(),
                        self.world.ty(ctx, field)?,
                    ));
                }
                let hypotheses: Vec<Binder> = fields
                    .iter()
                    .zip(&ctor.member)
                    .filter_map(|((name, id, _), of)| {
                        of.map(|of| {
                            (
                                format!("{name}_ih"),
                                self.world.fresh(),
                                app(motives[of].clone(), [Expr::FVar(*id)]),
                            )
                        })
                    })
                    .collect();
                let body = if k == j {
                    let minor = self.world.fresh();
                    let built = built(ctor, fvars(&fields));
                    minor_binders.push((
                        format!("m{i}"),
                        minor,
                        close_pi(&fields, app(Expr::FVar(motive), [built])),
                    ));
                    app(Expr::FVar(minor), fvars(&fields))
                } else {
                    unit.clone()
                };
                let mut all = fields;
                all.extend(hypotheses);
                recursor_minors.push(close_lam(&all, body));
            }
        }
        let mut binders = vec![
            ("motive".to_owned(), motive, motive_ty),
            ("t".to_owned(), major, t),
        ];
        binders.extend(minor_binders);
        let ty = close_pi(
            tb,
            close_pi(&binders, app(Expr::FVar(motive), [Expr::FVar(major)])),
        );
        let value = close_lam(
            tb,
            close_lam(
                &binders,
                app(
                    recursor(family, j, u, motives, recursor_minors),
                    [Expr::FVar(major)],
                ),
            ),
        );
        let lean = family.members[j].lean.clone().unwrap_or_default();
        self.define(&format!("{lean}.casesOn"), &["u".to_owned()], &ty, &value)
    }

    /// The functions of a recursive group, each with its recursive argument
    /// (structural) or measure (well-founded), sharing one list of type
    /// parameters by position.
    fn functions<'d>(item: &Pending<'d>) -> Result<Vec<Function<'d>>, Stop> {
        let mut out = Vec::new();
        for (lean, declaration) in &item.group {
            let SemanticDeclaration::Definition {
                type_parameters,
                parameters,
                result,
                recursive_argument,
                body,
                termination,
                ..
            } = declaration
            else {
                return Err(Stop::Refused(
                    "a mutual group of other than definitions".to_owned(),
                ));
            };
            let k = match recursive_argument {
                Some(argument) => Some(
                    parameters
                        .iter()
                        .position(|p| p.name == *argument)
                        .ok_or_else(|| {
                            Stop::Refused("the recursive argument is not a parameter".to_owned())
                        })?,
                ),
                None => None,
            };
            out.push(Function {
                lean: lean.clone(),
                type_parameters,
                parameters,
                result,
                k,
                body,
                measure: termination.as_ref().map(|t| &t.measure),
            });
        }
        if out
            .iter()
            .any(|f| f.type_parameters.len() != out[0].type_parameters.len())
        {
            return Err(Stop::Refused(
                "a mutual group whose members differ in type parameters".to_owned(),
            ));
        }
        Ok(out)
    }

    /// A lowering context for a group's members: the first member's type
    /// parameters, each other member's names bound to the same variables.
    fn group_ctx(&self, module: usize, functions: &[Function<'_>]) -> (Ctx, Vec<Binder>) {
        let mut ctx = Ctx::new(module);
        let tb = self.type_binders(&mut ctx, functions[0].type_parameters);
        for function in &functions[1..] {
            for (name, (_, id, _)) in function.type_parameters.iter().zip(&tb) {
                ctx.types.push((name.clone(), Expr::FVar(*id)));
            }
        }
        (ctx, tb)
    }

    /// Structural recursion over a family, mutual or nested: every member of
    /// the group is the same recursor of the family, whose motive for each
    /// family member is the tuple of the group's functions recursing on it
    /// (`PUnit` for none), and whose minor for a constructor is the tuple of
    /// those functions' branches. A recursive call on a field becomes its
    /// function's component of that field's hypothesis.
    fn structural_group(&mut self, item: &Pending<'_>) -> Result<(), Stop> {
        let functions = Self::functions(item)?;
        let module = self.world.modules[item.module].name.clone();
        for target in 0..functions.len() {
            let (ctx, tb) = self.group_ctx(item.module, &functions);
            let mut arguments = Vec::new();
            for function in &functions {
                let k = function.k.ok_or_else(|| {
                    Stop::Refused("a mutual group member without a recursive argument".to_owned())
                })?;
                if level_of(function.result) != 1 {
                    return Err(Stop::Refused(
                        "a recursive group whose result is not data".to_owned(),
                    ));
                }
                arguments.push(absolute(&function.parameters[k].r#type, &module));
            }
            let family = self.family_of(&ctx, &arguments[0])?;
            let member_of: Vec<usize> = arguments
                .iter()
                .map(|ty| family.members.iter().position(|m| m.ty == *ty))
                .collect::<Option<_>>()
                .ok_or_else(|| {
                    Stop::Refused(
                        "a group member recursive on a type outside its family".to_owned(),
                    )
                })?;
            // What each family member's motive holds: the functions recursing on it, and their types.
            let mut comps: Vec<Vec<usize>> = vec![Vec::new(); family.members.len()];
            for (f, m) in member_of.iter().enumerate() {
                comps[*m].push(f);
            }
            let mut component_ty = Vec::new();
            for function in &functions {
                let k = function.k.unwrap_or_default();
                let mut others = Vec::new();
                for (i, parameter) in function.parameters.iter().enumerate() {
                    if i != k {
                        others.push((
                            parameter.name.clone(),
                            self.world.fresh(),
                            self.world.ty(&ctx, &parameter.r#type)?,
                        ));
                    }
                }
                component_ty.push(close_pi(&others, self.world.ty(&ctx, function.result)?));
            }
            let tys = |m: usize| -> Vec<Expr> {
                comps[m].iter().map(|f| component_ty[*f].clone()).collect()
            };
            let punit = Expr::Const("PUnit".to_owned(), vec![Level::of(1)]);
            let motive_body = |m: usize| -> Expr {
                if comps[m].is_empty() {
                    punit.clone()
                } else {
                    tuple_ty(&tys(m))
                }
            };
            let motives: Vec<Expr> = family
                .members
                .iter()
                .enumerate()
                .map(|(m, member)| {
                    lam(
                        "x",
                        self.world.fresh(),
                        member.expr.clone(),
                        &motive_body(m),
                    )
                })
                .collect();
            let leans: Vec<&str> = functions.iter().map(|f| f.lean.as_str()).collect();
            let mut minors = Vec::new();
            for (m, member) in family.members.iter().enumerate() {
                for ctor in &member.ctors {
                    let mut fields: Vec<Binder> = Vec::new();
                    let mut hypotheses: Vec<Binder> = Vec::new();
                    let mut hypothesis_of: HashMap<u32, (u32, usize)> = HashMap::new();
                    for (field, of) in ctor.fields.iter().zip(&ctor.member) {
                        let id = self.world.fresh();
                        fields.push((
                            format!("a{}", fields.len()),
                            id,
                            self.world.ty(&ctx, field)?,
                        ));
                        if let Some(of) = of {
                            let ih = self.world.fresh();
                            hypotheses.push((
                                format!("ih{}", hypotheses.len()),
                                ih,
                                motive_body(*of),
                            ));
                            hypothesis_of.insert(id, (ih, *of));
                        }
                    }
                    let body = if comps[m].is_empty() {
                        Expr::Const("PUnit.unit".to_owned(), vec![Level::of(1)])
                    } else {
                        let mut values = Vec::new();
                        for f in &comps[m] {
                            let function = &functions[*f];
                            let k = function.k.unwrap_or_default();
                            let SemanticTerm::Match {
                                scrutinee,
                                branches,
                            } = function.body
                            else {
                                return Err(Stop::Refused(
                                    "recursion whose body is not a match on its argument"
                                        .to_owned(),
                                ));
                            };
                            let argument = &function.parameters[k].name;
                            if !matches!(scrutinee.as_ref(), SemanticTerm::Var { name } if name == argument)
                            {
                                return Err(Stop::Refused(
                                    "recursion whose body is not a match on its argument"
                                        .to_owned(),
                                ));
                            }
                            let branch = branches
                                .iter()
                                .find(|b| {
                                    b.constructor.name.rsplit('.').next()
                                        == Some(ctor.short.as_str())
                                })
                                .ok_or_else(|| {
                                    Stop::Refused(format!("a match without `{}`", ctor.short))
                                })?;
                            if branch.binders.len() != ctor.fields.len() {
                                return Err(Stop::Refused(format!(
                                    "branch `{}` binds {} fields",
                                    ctor.short,
                                    branch.binders.len()
                                )));
                            }
                            let mut inner = ctx.clone();
                            for ((binder, field), (_, id, _)) in
                                branch.binders.iter().zip(&ctor.fields).zip(&fields)
                            {
                                inner.push(binder, Expr::FVar(*id), Some(field.clone()));
                            }
                            inner.push(
                                argument,
                                built(ctor, fvars(&fields)),
                                Some(arguments[*f].clone()),
                            );
                            let mut mine: Vec<Binder> = Vec::new();
                            for (i, parameter) in function.parameters.iter().enumerate() {
                                if i != k {
                                    let id = self.world.fresh();
                                    mine.push((
                                        parameter.name.clone(),
                                        id,
                                        self.world.ty(&ctx, &parameter.r#type)?,
                                    ));
                                    inner.push(
                                        &parameter.name,
                                        Expr::FVar(id),
                                        Some(absolute(&parameter.r#type, &module)),
                                    );
                                }
                            }
                            let lowered = self.world.term(&inner, &branch.body)?;
                            let generic = function.type_parameters.len();
                            let mut stray = false;
                            let rewritten = rewrite(&lowered, &mut |head, args| {
                                let h = leans.iter().position(|l| *l == head)?;
                                let kh = functions[h].k.unwrap_or_default();
                                let found = match args.get(generic + kh) {
                                    Some(Expr::FVar(id)) => hypothesis_of.get(id).copied(),
                                    _ => None,
                                };
                                let Some((ih, of)) = found else {
                                    stray = true;
                                    return None;
                                };
                                let Some(position) = comps[of].iter().position(|g| *g == h) else {
                                    stray = true;
                                    return None;
                                };
                                let rest = args
                                    .iter()
                                    .enumerate()
                                    .skip(generic)
                                    .filter(|(i, _)| *i != generic + kh)
                                    .map(|(_, a)| a.clone());
                                Some(app(select(Expr::FVar(ih), position, &tys(of)), rest))
                            });
                            if stray {
                                return Err(Stop::Refused(
                                    "a recursive call that is not on a structurally smaller argument".to_owned(),
                                ));
                            }
                            values.push(close_lam(&mine, rewritten));
                        }
                        tuple(values, &tys(m))
                    };
                    let mut all = fields;
                    all.extend(hypotheses);
                    minors.push(close_lam(&all, body));
                }
            }
            let function = &functions[target];
            let mut outer_ctx = ctx.clone();
            let outer = self.value_binders(&mut outer_ctx, item.module, function.parameters)?;
            let k = function.k.unwrap_or_default();
            let m = member_of[target];
            let position = comps[m]
                .iter()
                .position(|f| *f == target)
                .unwrap_or_default();
            let at = app(
                recursor(&family, m, Level::of(1), motives, minors),
                [Expr::FVar(outer[k].1)],
            );
            let others = outer
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != k)
                .map(|(_, (_, id, _))| Expr::FVar(*id));
            let value = app(select(at, position, &tys(m)), others);
            let mut all = tb.clone();
            all.extend(outer.iter().cloned());
            let ty = close_pi(&all, self.world.ty(&ctx, function.result)?);
            self.define(&function.lean, &[], &ty, &close_lam(&all, value))?;
        }
        Ok(())
    }

    /// Well-founded recursion, mutual or not, written with fuel: the group
    /// is one `Nat.rec` over the fuel whose motive is the tuple of the
    /// group's functions, each recursive call taking its function's
    /// component of the hypothesis (one unit of fuel less), and each
    /// function is that recursor at its measure plus one. The termination
    /// evidence LexLean requires says each call lowers the measure, so the
    /// fuel never runs out and the definition is the function; the value at
    /// no fuel is a default of the result type, never reached.
    fn fuel_group(&mut self, item: &Pending<'_>) -> Result<(), Stop> {
        let functions = Self::functions(item)?;
        let leans: Vec<&str> = functions.iter().map(|f| f.lean.as_str()).collect();
        for target in 0..functions.len() {
            let (ctx, tb) = self.group_ctx(item.module, &functions);
            let mut tys = Vec::new();
            for function in &functions {
                if function.measure.is_none() {
                    return Err(Stop::Refused(
                        "a well-founded group member without a measure".to_owned(),
                    ));
                }
                if level_of(function.result) != 1 {
                    return Err(Stop::Refused(
                        "a recursive group whose result is not data".to_owned(),
                    ));
                }
                let mut params = Vec::new();
                for parameter in function.parameters {
                    params.push((
                        parameter.name.clone(),
                        self.world.fresh(),
                        self.world.ty(&ctx, &parameter.r#type)?,
                    ));
                }
                tys.push(close_pi(&params, self.world.ty(&ctx, function.result)?));
            }
            let all_ty = tuple_ty(&tys);
            let motive = lam("n", self.world.fresh(), c("Nat"), &all_ty);
            let mut zero = Vec::new();
            let mut step = Vec::new();
            let ih = self.world.fresh();
            for function in &functions {
                let mut inner = ctx.clone();
                let params = self.value_binders(&mut inner, item.module, function.parameters)?;
                zero.push(close_lam(
                    &params,
                    self.default_of(&ctx, function.result, 0)?,
                ));
                let lowered = self.world.term(&inner, function.body)?;
                let generic = function.type_parameters.len();
                let rewritten = rewrite(&lowered, &mut |head, args| {
                    let h = leans.iter().position(|l| *l == head)?;
                    Some(app(
                        select(Expr::FVar(ih), h, &tys),
                        args.iter().skip(generic).cloned(),
                    ))
                });
                step.push(close_lam(&params, rewritten));
            }
            let n = self.world.fresh();
            let succ = lam(
                "n",
                n,
                c("Nat"),
                &lam("ih", ih, all_ty.clone(), &tuple(step, &tys)),
            );
            let function = &functions[target];
            let mut outer_ctx = ctx.clone();
            let outer = self.value_binders(&mut outer_ctx, item.module, function.parameters)?;
            let measure = function
                .measure
                .ok_or_else(|| Stop::Refused("no measure".to_owned()))?;
            if self.world.infer(&outer_ctx, measure)? != SemanticType::Nat {
                return Err(Stop::Refused(
                    "a measure that is not a natural number".to_owned(),
                ));
            }
            let fuel = app(c("Nat.succ"), [self.world.term(&outer_ctx, measure)?]);
            let at = app(cu("Nat.rec", &[1]), [motive, tuple(zero, &tys), succ, fuel]);
            let value = app(select(at, target, &tys), fvars(&outer));
            let mut all = tb.clone();
            all.extend(outer.iter().cloned());
            let ty = close_pi(&all, self.world.ty(&ctx, function.result)?);
            self.define(&function.lean, &[], &ty, &close_lam(&all, value))?;
        }
        Ok(())
    }

    /// A value of a type, for fuel that has run out: the first constructor
    /// whose fields all have one, a library type's empty or zero value.
    fn default_of(&self, ctx: &Ctx, ty: &SemanticType, depth: u32) -> R<Expr> {
        if depth > 8 {
            return Err("a result type with no value to start from".to_owned());
        }
        let literal = |representation| {
            self.world.term(
                ctx,
                &SemanticTerm::Integer {
                    representation,
                    value: "0".to_owned(),
                },
            )
        };
        use crate::ir::semantic::SemanticInteger as I;
        Ok(match ty {
            SemanticType::Array { .. } => {
                return Err("a result type with no value to start from".to_owned())
            }
            SemanticType::Nat => self.world.term(
                ctx,
                &SemanticTerm::Nat {
                    value: "0".to_owned(),
                },
            )?,
            SemanticType::Bool => c("Bool.false"),
            SemanticType::Unit => c("Unit.unit"),
            SemanticType::String => Expr::Str(String::new()),
            SemanticType::Int => literal(I::Int)?,
            SemanticType::Int8 => literal(I::Int8)?,
            SemanticType::Int16 => literal(I::Int16)?,
            SemanticType::Int32 => literal(I::Int32)?,
            SemanticType::Int64 => literal(I::Int64)?,
            SemanticType::UInt8 => literal(I::UInt8)?,
            SemanticType::UInt16 => literal(I::UInt16)?,
            SemanticType::UInt32 => literal(I::UInt32)?,
            SemanticType::UInt64 => literal(I::UInt64)?,
            SemanticType::Ordering => c("Ordering.eq"),
            SemanticType::Bytes => self
                .world
                .term(ctx, &SemanticTerm::Bytes { hex: String::new() })?,
            SemanticType::ContractViolation | SemanticType::ReasoningFailure => app(
                cu("Prod.mk", &[0, 0]),
                [c("Bool"), c("Bool"), c("Bool.false"), c("Bool.false")],
            ),
            SemanticType::Option { value } => {
                app(cu("Option.none", &[0]), [self.world.ty(ctx, value)?])
            }
            SemanticType::List { .. } | SemanticType::Map { .. } | SemanticType::Set { .. } => {
                let Expr::App(_, element) = self.world.ty(ctx, ty)? else {
                    return Err("a list type without its element".to_owned());
                };
                app(cu("List.nil", &[0]), [(*element).clone()])
            }
            SemanticType::Product { left, right } => app(
                cu("Prod.mk", &[0, 0]),
                [
                    self.world.ty(ctx, left)?,
                    self.world.ty(ctx, right)?,
                    self.default_of(ctx, left, depth + 1)?,
                    self.default_of(ctx, right, depth + 1)?,
                ],
            ),
            SemanticType::Result { ok, error } => app(
                cu("Except.ok", &[0, 0]),
                [
                    self.world.ty(ctx, error)?,
                    self.world.ty(ctx, ok)?,
                    self.default_of(ctx, ok, depth + 1)?,
                ],
            ),
            SemanticType::Function { parameters, result } => {
                let mut binders = Vec::new();
                for parameter in parameters {
                    binders.push((
                        "x".to_owned(),
                        self.world.fresh(),
                        self.world.ty(ctx, parameter)?,
                    ));
                }
                close_lam(&binders, self.default_of(ctx, result, depth + 1)?)
            }
            SemanticType::Named { .. } => {
                let shape = self.world.shape(ctx, ty)?;
                let mut found = None;
                for ctor in &shape.ctors {
                    if ctor.recursive.iter().any(|r| *r) {
                        continue;
                    }
                    let fields: R<Vec<Expr>> = ctor
                        .fields
                        .iter()
                        .map(|f| self.default_of(ctx, f, depth + 1))
                        .collect();
                    if let Ok(fields) = fields {
                        found = Some(app(
                            app(cu(&ctor.constant, &shape.ctor_levels), shape.params.clone()),
                            fields,
                        ));
                        break;
                    }
                }
                found.ok_or("a result type with no value to start from")?
            }
            SemanticType::Type | SemanticType::Prop | SemanticType::Parameter { .. } => {
                return Err("a result type with no value to start from".to_owned())
            }
        })
    }

    /// The statement of each nested member of a nested induction, as
    /// `LexStmt.T.<binder> : Prop := Π params, Π binder, statement`, every
    /// parameter but the scrutinee: the motive the induction takes for that
    /// member. A member statement that cannot be lowered is left out, and
    /// the proof then cannot be built from its members.
    fn member_statements(
        &mut self,
        item: &Pending<'_>,
        type_parameters: &[String],
        parameters: &[SemanticParameter],
        scrutinee: &str,
        members: &[SemanticNestedMember],
    ) {
        let kept: Vec<SemanticParameter> = parameters
            .iter()
            .filter(|p| p.name != scrutinee)
            .cloned()
            .collect();
        for member in members {
            let mark = self.tables.checkpoint();
            let written = (|| -> Result<(), Stop> {
                let mut ctx = Ctx::new(item.module);
                let tb = self.type_binders(&mut ctx, type_parameters);
                let mut all = tb;
                all.extend(self.value_binders(&mut ctx, item.module, &kept)?);
                all.extend(self.value_binders(
                    &mut ctx,
                    item.module,
                    &[SemanticParameter {
                        name: member.binder.clone(),
                        r#type: member.r#type.clone(),
                    }],
                )?);
                let value = close_pi(&all, self.world.term(&ctx, &member.statement)?);
                let name = format!("{STATEMENT}.{}.{}", item.lean, member.binder);
                self.define(&name, &[], &sort(0), &value)
            })();
            if written.is_err() {
                self.tables.rewind(&mark);
            }
        }
    }

    /// A theorem's statement `LexStmt.T : Prop := Π params, statement`.
    fn statement(
        &mut self,
        item: &Pending<'_>,
        type_parameters: &[String],
        parameters: &[SemanticParameter],
        statement: &SemanticTerm,
    ) -> Result<String, Stop> {
        let mut ctx = Ctx::new(item.module);
        let tb = self.type_binders(&mut ctx, type_parameters);
        let mut all = tb;
        all.extend(self.value_binders(&mut ctx, item.module, parameters)?);
        let value = close_pi(&all, self.world.term(&ctx, statement)?);
        let name = format!("{STATEMENT}.{}", item.lean);
        self.define(&name, &[], &sort(0), &value)?;
        Ok(name)
    }
}

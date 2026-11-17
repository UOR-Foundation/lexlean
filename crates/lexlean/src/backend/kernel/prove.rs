//! The producers: one fixed kernel term for each produced proof form.
//!
//! | form          | term                                                    |
//! |---------------|---------------------------------------------------------|
//! | `reflexivity` | `@Eq.refl τ l` (or `Iff.refl p`) for the goal `l = r`    |
//! | `decide`      | `@of_decide_eq_true p inst (Eq.refl true)`              |
//! | `cases`       | `@T.casesOn params motive x minor…`                     |
//! | `induction`   | `@T.rec params motive minor… x gen…`, IHs per minor     |
//! | `apply`       | `@thm types args`, `thm` a produced prior theorem      |
//! | `law`         | `@law types args premises`, a registered library law    |
//!
//! A branch's goal is the statement with the scrutinee replaced by the
//! constructor applied to fresh field variables, everywhere it occurs, as
//! Lean's `cases` replaces it in every hypothesis.

use std::collections::BTreeMap;

use super::expr::{app, c, cu, lam, pi, Expr};
use super::lower::{Ctx, World, R};
use super::select;
use crate::ir::semantic::{SemanticProof, SemanticProofBranch, SemanticTerm, SemanticType};

/// What the producers need besides the world: the constant each produced
/// prior theorem was written under.
pub struct Produced<'a> {
    pub world: &'a World<'a>,
    /// Lean theorem name to the constant its produced proof is declared as.
    pub theorems: &'a BTreeMap<String, String>,
}

fn kind_of(proof: &SemanticProof) -> String {
    serde_json::to_value(proof)
        .ok()
        .and_then(|value| {
            value
                .get("kind")
                .and_then(|k| k.as_str().map(str::to_owned))
        })
        .unwrap_or_default()
}

impl Produced<'_> {
    /// A proof of `statement` in `ctx`.
    pub fn prove(&self, proof: &SemanticProof, statement: &SemanticTerm, ctx: &Ctx) -> R<Expr> {
        let w = self.world;
        match proof {
            SemanticProof::Reflexivity => self.reflexivity(statement, ctx),
            SemanticProof::Decide => Ok(app(
                c("of_decide_eq_true"),
                [
                    w.term(ctx, statement)?,
                    w.decidable(ctx, statement)?,
                    app(cu("Eq.refl", &[1]), [c("Bool"), c("Bool.true")]),
                ],
            )),
            SemanticProof::Cases {
                scrutinee,
                branches,
            } => self.cases(scrutinee, &[], branches, statement, ctx, false),
            SemanticProof::Induction {
                scrutinee,
                generalizing,
                branches,
            } => self.cases(scrutinee, generalizing, branches, statement, ctx, true),
            SemanticProof::TermCases {
                scrutinee,
                definitions,
            } => match self.term_cases_rewrite(scrutinee, definitions, statement, ctx)? {
                Some(term) => Ok(term),
                None => self.term_cases(scrutinee, statement, ctx),
            },
            SemanticProof::Apply {
                theorem,
                type_arguments,
                arguments,
            } => {
                let lean = w.constant(ctx.module, theorem)?;
                let names: Vec<&String> = self.theorems.keys().collect();
                select::member_check(&names, &&lean)
                    .map_err(|_| format!("it applies `{lean}`, whose proof was not produced"))?;
                let produced = &self.theorems[&lean];
                let mut out = Vec::new();
                for argument in type_arguments {
                    out.push(w.ty(ctx, argument)?);
                }
                for argument in arguments {
                    out.push(w.term(ctx, argument)?);
                }
                Ok(app(c(produced), out))
            }
            SemanticProof::Law {
                name,
                type_arguments,
                arguments,
                premises,
            } => {
                let registered: Vec<String> = crate::ir::semantic::laws::registry()?
                    .laws
                    .iter()
                    .map(|law| law.name.clone())
                    .collect();
                let mut cited = Vec::new();
                for premise in premises {
                    cited.push(w.constant(ctx.module, premise)?);
                }
                let produced: Vec<String> = self.theorems.keys().cloned().collect();
                select::law_check(name, &registered, &cited, &produced).map_err(|refusal| {
                    match refusal {
                        select::Refusal::NotProduced(lean) => {
                            format!("its premise `{lean}` has no produced proof")
                        }
                        _ => format!("`{name}` is not a registered law"),
                    }
                })?;
                let law = crate::ir::semantic::laws::law(name)?;
                let mut out = Vec::new();
                for argument in type_arguments {
                    out.push(w.ty(ctx, argument)?);
                }
                for argument in arguments {
                    out.push(w.term(ctx, argument)?);
                }
                for lean in &cited {
                    out.push(c(&self.theorems[lean]));
                }
                Ok(app(c(&law.lean), out))
            }
            other => Err(format!("proof form `{}`", kind_of(other))),
        }
    }

    /// `term_cases` by rewriting, as `cases h : t <;> simp only [defs, h]`
    /// does: the statement and the split term are unfolded over `defs`
    /// (`reduce`), and when the unfolded term `t` occurs in the unfolded
    /// statement `U`, the motive `P v := U[t := v]` is formed. For each
    /// constructor `c` of `t`'s type the case `h : t = c` proves the
    /// statement by `Eq.mpr (congrArg P h) p`, where `p` proves `P c`, which
    /// is unfolded again and closed by `reflexivity` or `decide`; the kernel
    /// checks `P t` against the statement by definitional equality. `None`
    /// when `t` does not occur, for enumeration to try.
    #[allow(clippy::too_many_lines)]
    fn term_cases_rewrite(
        &self,
        scrutinee: &SemanticTerm,
        definitions: &[crate::ir::semantic::MemberRef],
        statement: &SemanticTerm,
        ctx: &Ctx,
    ) -> R<Option<Expr>> {
        let w = self.world;
        let reducer = super::reduce::Reducer::new(w, ctx.module, definitions)?;
        let unfolded = reducer.reduce(&serde_json::to_value(statement).map_err(|e| e.to_string())?);
        let target = reducer.reduce(&serde_json::to_value(scrutinee).map_err(|e| e.to_string())?);
        let split = "__split";
        let (abstracted, occurrences) = super::reduce::abstract_term(&unfolded, &target, split);
        if occurrences == 0 {
            return Ok(None);
        }
        let ty = w.infer(ctx, scrutinee)?;
        let shape = w.shape(ctx, &ty)?;
        let ty_expr = w.ty(ctx, &ty)?;
        let t_expr = w.term(ctx, scrutinee)?;
        let goal = w.term(ctx, statement)?;
        let abstracted_term = super::reduce::term_of(&abstracted)?;
        // P := fun v => U[t := v]
        let v = w.fresh();
        let mut with_v = ctx.clone();
        with_v.push(split, Expr::FVar(v), Some(ty.clone()));
        let motive_p = lam("v", v, ty_expr.clone(), &w.term(&with_v, &abstracted_term)?);
        // The casesOn motive: fun x => t = x -> G
        let x = w.fresh();
        let equation = |rhs: Expr| app(cu("Eq", &[1]), [ty_expr.clone(), t_expr.clone(), rhs]);
        let motive = lam("x", x, ty_expr.clone(), &super::expr::arrow(equation(Expr::FVar(x)), goal.clone()));
        let mut minors = Vec::new();
        for ctor in &shape.ctors {
            let mut inner = ctx.clone();
            let mut bound: Vec<(String, u32, Expr)> = Vec::new();
            let mut fields = Vec::new();
            let mut field_terms = Vec::new();
            for (index, field) in ctor.fields.iter().enumerate() {
                let id = w.fresh();
                let name = format!("__f{index}_{id}");
                inner.push(&name, Expr::FVar(id), Some(field.clone()));
                bound.push((name.clone(), id, w.ty(ctx, field)?));
                fields.push(Expr::FVar(id));
                field_terms.push(serde_json::json!({"kind": "var", "name": name}));
            }
            let mut arguments = shape.params.clone();
            arguments.extend(fields.iter().cloned());
            let value = app(cu(&ctor.constant, &shape.ctor_levels), arguments);
            // The constructor as an IR term, to unfold the case with.
            let constructor = constructor_term(&ty, &ctor.short, &ctor.constant, field_terms);
            let case = substitute_split(&abstracted, split, &constructor);
            let case = reducer.reduce(&case);
            let case_term = super::reduce::term_of(&case)?;
            let mut leaf = &case_term;
            while let SemanticTerm::Let { body, .. } = leaf {
                leaf = body;
            }
            let closing = if matches!(leaf, SemanticTerm::Eq { .. } | SemanticTerm::Iff { .. }) {
                SemanticProof::Reflexivity
            } else {
                SemanticProof::Decide
            };
            let proof = self.prove(&closing, &case_term, &inner)?;
            let h = w.fresh();
            let rewritten = app(
                cu("Eq.mpr", &[0]),
                [
                    app(motive_p.clone(), [t_expr.clone()]),
                    app(motive_p.clone(), [value.clone()]),
                    app(
                        cu("congrArg", &[1, 1]),
                        [ty_expr.clone(), sort_prop(), t_expr.clone(), value.clone(), motive_p.clone(), Expr::FVar(h)],
                    ),
                    proof,
                ],
            );
            let mut body = lam("h", h, equation(value), &rewritten);
            for (name, id, binder_ty) in bound.into_iter().rev() {
                body = lam(&name, id, binder_ty, &body);
            }
            minors.push(body);
        }
        let mut levels = vec![0];
        levels.extend(&shape.levels);
        let mut arguments = shape.params.clone();
        arguments.push(motive);
        arguments.push(t_expr.clone());
        arguments.extend(minors);
        Ok(Some(app(
            app(cu(&shape.cases_on, &levels), arguments),
            [app(cu("Eq.refl", &[1]), [ty_expr, t_expr])],
        )))
    }

    /// `term_cases` by enumeration: every variable of a finite type (`Bool`,
    /// or a document type whose constructors carry no field) in scope is
    /// split in turn, and each case is closed by evaluation, by
    /// `reflexivity` for an equation and `decide` otherwise. The split
    /// term's own value is never named: once its finite variables are
    /// constructors, the kernel computes as much of it as they fix, and the
    /// kernel refuses a case it cannot close. With no variable of a finite
    /// type, the one case is the statement itself.
    fn term_cases(&self, scrutinee: &SemanticTerm, statement: &SemanticTerm, ctx: &Ctx) -> R<Expr> {
        let w = self.world;
        // Every local of a finite type, the split term's first: a value the
        // split term reaches through a definition can rest on a local it does
        // not name.
        let mut names = Vec::new();
        collect_vars(&serde_json::to_value(scrutinee).unwrap_or_default(), &mut names);
        for local in &ctx.locals {
            if !names.contains(&local.name) {
                names.push(local.name.clone());
            }
        }
        let mut finite: Vec<(String, Vec<String>)> = Vec::new();
        for name in names {
            if finite.iter().any(|(seen, _)| *seen == name) {
                continue;
            }
            // A variable of an infinite type stays a variable: the cases of
            // the finite ones may already fix the split term's value.
            let Ok(local) = ctx.local(&name) else { continue };
            let Some(ty) = local.ty.clone() else { continue };
            let Ok(shape) = w.shape(ctx, &ty) else { continue };
            if shape.ctors.iter().any(|ctor| !ctor.fields.is_empty()) {
                continue;
            }
            finite.push((name, shape.ctors.iter().map(|ctor| ctor.short.clone()).collect()));
        }
        let mut leaf = statement;
        while let SemanticTerm::Let { body, .. } = leaf {
            leaf = body;
        }
        let mut proof = if matches!(leaf, SemanticTerm::Eq { .. } | SemanticTerm::Iff { .. }) {
            SemanticProof::Reflexivity
        } else {
            SemanticProof::Decide
        };
        for (name, ctors) in finite.into_iter().rev() {
            proof = SemanticProof::Cases {
                scrutinee: name,
                branches: ctors
                    .into_iter()
                    .map(|constructor| SemanticProofBranch {
                        constructor,
                        binders: Vec::new(),
                        proof: Box::new(proof.clone()),
                    })
                    .collect(),
            };
        }
        self.prove(&proof, statement, ctx)
    }

    fn reflexivity(&self, statement: &SemanticTerm, ctx: &Ctx) -> R<Expr> {
        let w = self.world;
        match statement {
            // The goal is the `let`; its body, with the value in place, is
            // definitionally the same proposition.
            SemanticTerm::Let {
                binder,
                value,
                body,
            } => {
                let mut inner = ctx.clone();
                inner.push(
                    &binder.name,
                    w.term(ctx, value)?,
                    Some(super::lower::absolute(
                        &binder.r#type,
                        &w.modules[ctx.module].name,
                    )),
                );
                self.reflexivity(body, &inner)
            }
            SemanticTerm::Eq { left, right } => {
                let ty = w.infer(ctx, left).or_else(|_| w.infer(ctx, right))?;
                let level = if ty == SemanticType::Type { 2 } else { 1 };
                Ok(app(
                    cu("Eq.refl", &[level]),
                    [w.ty(ctx, &ty)?, w.term(ctx, left)?],
                ))
            }
            SemanticTerm::Iff { left, .. } => Ok(app(c("Iff.refl"), [w.term(ctx, left)?])),
            _ => Err("reflexivity of a goal that is not an equation".to_owned()),
        }
    }

    /// `cases` (`induct` false) or `induction` (`induct` true).
    #[allow(clippy::too_many_lines)]
    fn cases(
        &self,
        scrutinee: &str,
        generalizing: &[String],
        branches: &[SemanticProofBranch],
        statement: &SemanticTerm,
        ctx: &Ctx,
        induct: bool,
    ) -> R<Expr> {
        let w = self.world;
        let local = ctx.local(scrutinee)?;
        let Expr::FVar(x) = local.expr else {
            return Err(format!("`{scrutinee}` is no longer a variable"));
        };
        let ty = local
            .ty
            .clone()
            .ok_or_else(|| format!("`{scrutinee}` is a proof"))?;
        let shape = w.shape(ctx, &ty)?;
        let ty_expr = w.ty(ctx, &ty)?;
        let goal = w.term(ctx, statement)?;
        // The generalized locals, as the motive quantifies them.
        let mut generals = Vec::new();
        for name in generalizing {
            let general = ctx.local(name)?;
            let Expr::FVar(id) = general.expr else {
                return Err(format!("`{name}` is no longer a variable"));
            };
            let general_ty = general
                .ty
                .clone()
                .ok_or_else(|| format!("`{name}` is a proof"))?;
            generals.push((
                name.clone(),
                id,
                general_ty.clone(),
                w.ty(ctx, &general_ty)?,
            ));
        }
        let mut quantified = goal;
        for (name, id, _, general_ty) in generals.iter().rev() {
            quantified = pi(name, *id, general_ty.clone(), &quantified);
        }
        let motive = lam("x", x, ty_expr, &quantified);
        // Which branch is each constructor's minor premise is decided by the
        // checks `KernelTerms` states and proves its producers refuse
        // exactly: a branch names a constructor by position, and a name the
        // type has no constructor of is a position past the last.
        let shapes: Vec<select::CtorShape> = shape
            .ctors
            .iter()
            .map(|ctor| select::CtorShape {
                fields: ctor.fields.len(),
                recursive: ctor.recursive.iter().filter(|r| **r).count(),
            })
            .collect();
        let heads: Vec<select::BranchHead> = branches
            .iter()
            .enumerate()
            .map(|(at, branch)| select::BranchHead {
                ctor: shape
                    .ctors
                    .iter()
                    .position(|ctor| {
                        branch.constructor.rsplit('.').next() == Some(ctor.short.as_str())
                    })
                    .unwrap_or(shape.ctors.len() + at),
                binders: branch.binders.len(),
            })
            .collect();
        let short = |index: usize| {
            shape.ctors.get(index).map_or_else(
                || branches[index - shape.ctors.len()].constructor.clone(),
                |ctor| ctor.short.clone(),
            )
        };
        select::check_from::<String>(&shapes, 0, &heads, induct).map_err(
            |refusal| match refusal {
                select::Refusal::MissingCase(index) => {
                    format!("no case for constructor `{}`", short(index))
                }
                select::Refusal::RepeatedCase(index) => {
                    format!("constructor `{}` has more than one case", short(index))
                }
                select::Refusal::BinderCount(index) => format!(
                    "case `{}` binds the wrong number of names for {} fields and {} hypotheses",
                    short(index),
                    shapes[index].fields,
                    if induct { shapes[index].recursive } else { 0 }
                ),
                select::Refusal::UnknownCase(index) => {
                    format!("case `{}` names no constructor of the type", short(index))
                }
                other => format!("{other:?}"),
            },
        )?;
        let mut minors = Vec::new();
        for (index, ctor) in shape.ctors.iter().enumerate() {
            let branch = &branches[select::minor_of(index, &heads)
                .ok_or_else(|| format!("no case for constructor `{}`", ctor.short))?];
            let mut inner = ctx.clone();
            let mut bound: Vec<(String, u32, Expr)> = Vec::new();
            let mut fields = Vec::new();
            for (binder, field) in branch.binders.iter().zip(&ctor.fields) {
                let id = w.fresh();
                inner.push(binder, Expr::FVar(id), Some(field.clone()));
                bound.push((binder.clone(), id, w.ty(ctx, field)?));
                fields.push(Expr::FVar(id));
            }
            let mut arguments = shape.params.clone();
            arguments.extend(fields.iter().cloned());
            let value = app(cu(&ctor.constant, &shape.ctor_levels), arguments);
            if induct {
                // One hypothesis per recursive field: the motive at it.
                let recursive_fields = fields
                    .iter()
                    .zip(&ctor.recursive)
                    .filter(|(_, recursive)| **recursive)
                    .map(|(field, _)| field.clone());
                for (binder, field) in branch.binders[ctor.fields.len()..]
                    .iter()
                    .zip(recursive_fields)
                {
                    let id = w.fresh();
                    let hypothesis = quantified.replace_fvar(x, &field);
                    inner.push(binder, Expr::FVar(id), None);
                    bound.push((binder.clone(), id, hypothesis));
                }
            }
            inner.substitute(x, &value);
            // The generalized locals are introduced again in the case.
            for (name, id, general_ty, _) in &generals {
                let fresh = w.fresh();
                inner.substitute(*id, &Expr::FVar(fresh));
                let lowered = w.ty(&inner, general_ty)?;
                inner.push(name, Expr::FVar(fresh), Some(general_ty.clone()));
                bound.push((name.clone(), fresh, lowered));
            }
            let mut body = self.prove(&branch.proof, statement, &inner)?;
            for (name, id, binder_ty) in bound.into_iter().rev() {
                body = lam(&name, id, binder_ty, &body);
            }
            minors.push(body);
        }
        let mut levels = vec![0];
        levels.extend(&shape.levels);
        let mut arguments = shape.params.clone();
        arguments.push(motive);
        if induct {
            arguments.extend(minors);
            arguments.push(Expr::FVar(x));
            arguments.extend(generals.iter().map(|(_, id, _, _)| Expr::FVar(*id)));
            Ok(app(cu(&shape.rec, &levels), arguments))
        } else {
            arguments.push(Expr::FVar(x));
            arguments.extend(minors);
            Ok(app(cu(&shape.cases_on, &levels), arguments))
        }
    }
}

/// The constants a producer itself may name: the export roots the library
/// must hold for the proof forms, besides the statement's vocabulary.
pub const VOCABULARY: &[&str] = &[
    "Eq.refl",
    "Iff.refl",
    "of_decide_eq_true",
    "Eq.mpr",
    "congrArg",
];

/// The variables a term names, in order of first mention.
fn collect_vars(value: &serde_json::Value, out: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            if map.get("kind").and_then(serde_json::Value::as_str) == Some("var") {
                if let Some(name) = map.get("name").and_then(serde_json::Value::as_str) {
                    if !out.iter().any(|seen| seen == name) {
                        out.push(name.to_owned());
                    }
                }
            }
            for inner in map.values() {
                collect_vars(inner, out);
            }
        }
        serde_json::Value::Array(items) => {
            for inner in items {
                collect_vars(inner, out);
            }
        }
        _ => {}
    }
}

fn sort_prop() -> Expr {
    super::expr::sort(0)
}

/// The IR term of a constructor of `ty` applied to `fields`.
fn constructor_term(
    ty: &crate::ir::semantic::SemanticType,
    short: &str,
    constant: &str,
    fields: Vec<serde_json::Value>,
) -> serde_json::Value {
    use crate::ir::semantic::SemanticType as T;
    match ty {
        T::Bool => serde_json::json!({"kind": "bool", "value": short == "true"}),
        T::Nat if short == "zero" => serde_json::json!({"kind": "nat", "value": "0"}),
        T::List { element } if short == "nil" => serde_json::json!({"kind": "nil", "element": element}),
        T::List { .. } => serde_json::json!({"kind": "cons", "head": fields[0], "tail": fields[1]}),
        T::Option { value } => serde_json::json!({"kind": "constructor", "constructor": {"name": format!("Option.{short}")}, "type_arguments": [value], "arguments": fields}),
        T::Named { member, arguments } => {
            let owner = member.name.clone();
            serde_json::json!({
                "kind": "constructor",
                "constructor": {"module": member.module, "name": format!("{owner}.{short}")},
                "type_arguments": arguments,
                "arguments": fields,
            })
        }
        _ => serde_json::json!({"kind": "constructor", "constructor": {"name": constant}, "arguments": fields}),
    }
}

/// `term` with the variable `name` replaced by `by`.
fn substitute_split(term: &serde_json::Value, name: &str, by: &serde_json::Value) -> serde_json::Value {
    match term {
        serde_json::Value::Array(items) => serde_json::Value::Array(items.iter().map(|item| substitute_split(item, name, by)).collect()),
        serde_json::Value::Object(map) => {
            if map.get("kind").and_then(serde_json::Value::as_str) == Some("var")
                && map.get("name").and_then(serde_json::Value::as_str) == Some(name)
            {
                return by.clone();
            }
            serde_json::Value::Object(map.iter().map(|(key, value)| (key.clone(), substitute_split(value, name, by))).collect())
        }
        other => other.clone(),
    }
}

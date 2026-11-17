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
            SemanticProof::Apply {
                theorem,
                type_arguments,
                arguments,
            } => {
                let lean = w.constant(ctx.module, theorem)?;
                let produced = self
                    .theorems
                    .get(&lean)
                    .ok_or_else(|| format!("it applies `{lean}`, whose proof was not produced"))?;
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
                let law = crate::ir::semantic::laws::law(name)?;
                let mut out = Vec::new();
                for argument in type_arguments {
                    out.push(w.ty(ctx, argument)?);
                }
                for argument in arguments {
                    out.push(w.term(ctx, argument)?);
                }
                for premise in premises {
                    let lean = w.constant(ctx.module, premise)?;
                    let produced = self
                        .theorems
                        .get(&lean)
                        .ok_or_else(|| format!("its premise `{lean}` has no produced proof"))?;
                    out.push(c(produced));
                }
                Ok(app(c(&law.lean), out))
            }
            other => Err(format!("proof form `{}`", kind_of(other))),
        }
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
        let mut minors = Vec::new();
        let mut seen = Vec::new();
        for ctor in &shape.ctors {
            let branch = branches
                .iter()
                .find(|branch| branch.constructor.rsplit('.').next() == Some(ctor.short.as_str()))
                .ok_or_else(|| format!("no case for constructor `{}`", ctor.short))?;
            seen.push(branch.constructor.clone());
            let hypotheses = if induct {
                ctor.recursive.iter().filter(|r| **r).count()
            } else {
                0
            };
            if branch.binders.len() != ctor.fields.len() + hypotheses {
                return Err(format!(
                    "case `{}` binds {} names for {} fields and {} hypotheses",
                    ctor.short,
                    branch.binders.len(),
                    ctor.fields.len(),
                    hypotheses
                ));
            }
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
        if seen.len() != branches.len() {
            return Err("a case names no constructor of the type".to_owned());
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
pub const VOCABULARY: &[&str] = &["Eq.refl", "Iff.refl", "of_decide_eq_true"];

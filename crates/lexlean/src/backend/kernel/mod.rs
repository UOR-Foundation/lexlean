//! Kernel-term production (SPEC.md §18.10): for the semantic proof forms
//! listed in [`PRODUCED_FORMS`], the Lean kernel term of a theorem is written
//! directly from the linked IR, with no elaborator and no tactic, as
//! lean4export 3.1.0 declarations that continue a library export.
//!
//! Nothing here is trusted. Each produced declaration is a theorem whose type
//! is the statement lowered from the IR (`lower`) and whose value is the
//! producer's term (`prove`); Lean's kernel judges both. When the library
//! also holds the theorem Lean elaborated from the generated source, a second
//! declaration binds the two: its type is the lowered statement and its value
//! Lean's theorem, so the kernel accepts it only if the statement the
//! producer proved is definitionally the statement as written.

pub mod declare;
mod export;
mod expr;
mod lower;
mod prove;
pub mod select;

use std::collections::{BTreeMap, BTreeSet};

use crate::code;
use crate::diagnostic::Diagnostic;
use crate::ir::semantic::{SemanticDeclaration, SemanticModule, SemanticProof};

use expr::{lam, pi, sort, Expr};
use lower::{Ctx, World};

/// The semantic proof forms a producer exists for, by their IR `kind`.
pub const PRODUCED_FORMS: &[&str] = &[
    "reflexivity",
    "decide",
    "cases",
    "induction",
    "apply",
    "law",
    "term_cases",
];

/// The namespace of produced declarations: a produced proof of `T` is
/// `LexLeanKernel.T`, and its binding to Lean's `T` is `LexLeanKernel.bound.T`.
pub const NAMESPACE: &str = "LexLeanKernel";

/// One linked module, as production reads it.
#[derive(Debug, Clone)]
pub struct KernelModule {
    /// The project's name of the module.
    pub name: String,
    /// The Lean module it generates.
    pub lean_module: String,
    /// The modules it imports, by the project's names.
    pub imports: Vec<String>,
    /// Its semantic declarations.
    pub semantic: SemanticModule,
    /// The declarations a model construct elaborated to.
    pub elaborated: Vec<SemanticDeclaration>,
}

/// What production did with one theorem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// A proof was written as `constant`; `bound` names the binding to Lean's
    /// own theorem when the library holds it.
    Produced {
        constant: String,
        bound: Option<String>,
    },
    /// No proof was written, for the reason given.
    Refused { reason: String },
}

/// One theorem of the project.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    /// The Lean name of the theorem.
    pub theorem: String,
    /// The kind of its whole proof.
    pub form: String,
    /// Its exact declared axioms.
    pub axioms: Vec<String>,
    pub outcome: Outcome,
}

/// The produced declarations and what became of every theorem.
pub struct Production {
    /// lean4export lines continuing the library export: every produced
    /// proof, and every binding.
    pub suffix: String,
    pub rows: Vec<Row>,
    /// The library's tables, before anything was produced.
    pristine: export::Tables,
    /// Each produced theorem's type, value, and the produced theorems it names.
    terms: BTreeMap<String, (Expr, Expr, BTreeSet<String>)>,
}

impl Production {
    /// For each produced theorem, the lines continuing the library export
    /// that declare it and the produced theorems it applies, and nothing
    /// else: what a checker holding the library judges when one theorem is
    /// edited.
    ///
    /// # Errors
    ///
    /// `LLV7103` when a produced term cannot be written again; production
    /// wrote each once, so this does not happen for its own terms.
    pub fn parts(&self) -> Result<Vec<(String, String)>, Diagnostic> {
        let terms = &self.terms;
        let mut parts = Vec::new();
        for theorem in terms.keys() {
            // The theorem after every produced theorem it applies, transitively.
            let mut order: Vec<&String> = Vec::new();
            let mut stack = vec![(theorem, false)];
            while let Some((name, expanded)) = stack.pop() {
                if order.contains(&name) {
                    continue;
                }
                if expanded {
                    order.push(name);
                    continue;
                }
                stack.push((name, true));
                for applied in &terms[name].2 {
                    if let Some((key, _)) = terms.get_key_value(applied) {
                        stack.push((key, false));
                    }
                }
            }
            let mut part = self.pristine.clone();
            for name in order {
                let (ty, value, _) = &terms[name];
                part.theorem(&format!("{NAMESPACE}.{name}"), ty, value)
                    .map_err(refusal)?;
            }
            parts.push((theorem.clone(), part.suffix()));
        }
        Ok(parts)
    }
}

fn refusal(reason: impl Into<String>) -> Diagnostic {
    Diagnostic::new(code!("LLV7103"), reason.into())
}

fn proof_kind(proof: &SemanticProof) -> String {
    serde_json::to_value(proof)
        .ok()
        .and_then(|value| {
            value
                .get("kind")
                .and_then(|k| k.as_str().map(str::to_owned))
        })
        .unwrap_or_default()
}

/// Every Lean module a set of generated modules needs, in an order Lean can
/// compile them in: the runtime modules they import first (the runtime
/// registry is in dependency order and imports no project module), then the
/// generated modules, each after those it imports. Each is `(module, text)`;
/// a checker that holds the project's compiled environment is built from
/// these.
///
/// # Errors
///
/// `LLV7103` when the generated modules import one another in a cycle, and
/// `LLI9001` when the shipped runtime cannot be read.
pub fn lean_sources(generated: Vec<(String, String)>) -> Result<Vec<(String, String)>, Diagnostic> {
    let runtime =
        crate::backend::runtime::imported_by(generated.iter().map(|(_, text)| text.as_str()))?;
    let mut all: Vec<(String, String)> = runtime
        .iter()
        .map(|module| (module.name.clone(), module.text.clone()))
        .collect();
    let names: BTreeSet<String> = generated.iter().map(|(name, _)| name.clone()).collect();
    let mut pending = generated;
    while !pending.is_empty() {
        let placed: BTreeSet<String> = all.iter().map(|(name, _)| name.clone()).collect();
        let (ready, rest): (Vec<_>, Vec<_>) = pending.into_iter().partition(|(_, text)| {
            text.lines()
                .filter_map(|line| {
                    line.strip_prefix("public import ")
                        .or_else(|| line.strip_prefix("import "))
                })
                .map(str::trim)
                .all(|import| placed.contains(import) || !names.contains(import))
        });
        if ready.is_empty() {
            return Err(refusal(
                "the generated modules import one another in a cycle",
            ));
        }
        all.extend(ready);
        pending = rest;
    }
    Ok(all)
}

/// The semantic modules of a semantic snapshot (§24.1), each after the
/// modules it imports. A module without semantic data (a prose or core
/// module) is left out: it has no semantic proof to produce.
///
/// # Errors
///
/// `LLV7103` when the snapshot is not one this compiler writes.
pub fn modules_from_snapshot(
    snapshot: &serde_json::Value,
) -> Result<Vec<KernelModule>, Diagnostic> {
    let rows = snapshot
        .get("modules")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| refusal("the snapshot has no modules"))?;
    let mut modules = BTreeMap::new();
    for row in rows {
        let Some(semantic) = row.get("semantic") else {
            continue;
        };
        let text = |key: &str| {
            row.get(key)
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned)
                .ok_or_else(|| refusal(format!("a snapshot module lacks `{key}`")))
        };
        let semantic: SemanticModule = serde_json::from_value(semantic.clone())
            .map_err(|error| refusal(format!("a snapshot module's semantic data: {error}")))?;
        let mut elaborated = Vec::new();
        for declaration in row
            .get("declarations")
            .and_then(serde_json::Value::as_array)
            .into_iter()
            .flatten()
        {
            for generated in declaration
                .pointer("/elaboration/declarations")
                .and_then(serde_json::Value::as_array)
                .into_iter()
                .flatten()
            {
                elaborated.push(
                    serde_json::from_value(generated.clone())
                        .map_err(|error| refusal(format!("an elaborated declaration: {error}")))?,
                );
            }
        }
        let name = text("name")?;
        modules.insert(
            name.clone(),
            KernelModule {
                name,
                lean_module: text("lean_module")?,
                imports: row
                    .get("imports")
                    .and_then(serde_json::Value::as_array)
                    .into_iter()
                    .flatten()
                    .filter_map(|import| import.as_str().map(str::to_owned))
                    .collect(),
                semantic,
                elaborated,
            },
        );
    }
    // Imports first, so a theorem a later module applies is produced before.
    let mut order: Vec<KernelModule> = Vec::new();
    let mut placed = BTreeSet::new();
    while placed.len() < modules.len() {
        let ready: Vec<String> = modules
            .iter()
            .filter(|(name, module)| {
                !placed.contains(*name)
                    && module
                        .imports
                        .iter()
                        .all(|import| placed.contains(import) || !modules.contains_key(import))
            })
            .map(|(name, _)| name.clone())
            .collect();
        if ready.is_empty() {
            return Err(refusal(
                "the snapshot's modules import one another in a cycle",
            ));
        }
        for name in ready {
            placed.insert(name.clone());
            order.push(modules[&name].clone());
        }
    }
    Ok(order)
}

/// The constants a library export must hold for production over these
/// modules: the producers' fixed vocabulary, the runtime vocabulary of each
/// runtime module in `runtime` (the Lean modules the project imports), every
/// declaration of the modules with the eliminators and projections their
/// types generate, and every registered law a proof applies.
#[must_use]
pub fn export_roots(modules: &[KernelModule], runtime: &[&str]) -> Vec<String> {
    let mut roots: BTreeSet<String> = lower::VOCABULARY
        .iter()
        .chain(prove::VOCABULARY)
        .map(|name| (*name).to_owned())
        .collect();
    for (module, names) in lower::RUNTIME_VOCABULARY {
        if runtime.contains(module) {
            roots.extend(names.iter().map(|name| (*name).to_owned()));
        }
    }
    let mut laws = BTreeSet::new();
    for module in modules {
        for declaration in module
            .semantic
            .declarations
            .iter()
            .chain(&module.elaborated)
        {
            let lean = format!("{}.{}", module.lean_module, declaration.name());
            match declaration {
                SemanticDeclaration::Inductive { .. } => {
                    roots.insert(format!("{lean}.casesOn"));
                }
                SemanticDeclaration::Structure { fields, .. } => {
                    roots.insert(format!("{lean}.casesOn"));
                    for field in fields {
                        roots.insert(format!("{lean}.{}", field.name));
                    }
                }
                SemanticDeclaration::Theorem { proof, .. } => collect_laws(proof, &mut laws),
                _ => {}
            }
            roots.insert(lean);
        }
    }
    for law in laws {
        if let Ok(law) = crate::ir::semantic::laws::law(&law) {
            roots.insert(law.lean.clone());
        }
    }
    roots.into_iter().collect()
}

fn collect_laws(proof: &SemanticProof, out: &mut BTreeSet<String>) {
    match proof {
        SemanticProof::Law { name, .. } => {
            out.insert(name.clone());
        }
        SemanticProof::Constructor { branches } => {
            for branch in branches {
                collect_laws(branch, out);
            }
        }
        SemanticProof::Cases { branches, .. }
        | SemanticProof::Induction { branches, .. }
        | SemanticProof::NestedInduction { branches, .. } => {
            for branch in branches {
                collect_laws(&branch.proof, out);
            }
        }
        _ => {}
    }
}

/// Every constant the lowering and the producers may name in any project:
/// the library a checker holds for projects it has not seen, whose own
/// declarations it is given (§18.10).
#[must_use]
pub fn library_roots() -> Vec<String> {
    let mut roots: BTreeSet<String> = lower::VOCABULARY
        .iter()
        .chain(prove::VOCABULARY)
        .map(|name| (*name).to_owned())
        .collect();
    for (_, names) in lower::RUNTIME_VOCABULARY {
        roots.extend(names.iter().map(|name| (*name).to_owned()));
    }
    roots.into_iter().collect()
}

/// Produces every theorem of `modules` whose proof is in the produced forms,
/// as declarations continuing the export `library`: each produced term may
/// name only constants the library declares, and a binding is written for
/// each theorem the library holds Lean's proof of.
///
/// Without a library the declarations are self-contained (their numbering
/// starts empty), for a checker that holds the project's compiled
/// environment instead; every theorem is then bound to Lean's.
///
/// # Errors
///
/// `LLV7103` when `library` is not lean4export 3.1.0 NDJSON. A theorem the
/// producers do not cover is a refused row, not an error.
pub fn produce(modules: &[KernelModule], library: Option<&str>) -> Result<Production, Diagnostic> {
    let tables = export::Tables::parse(library.unwrap_or_default()).map_err(refusal)?;
    let world = World::new(modules);
    Ok(produce_in(&world, tables, library.is_some(), true, None))
}

/// The project's own declarations (§18.10) and the theorems produced over
/// them.
pub struct Declarations {
    /// lean4export lines continuing the library export that declare the
    /// project's types, eliminators, projections, definitions, and theorem
    /// statements.
    pub declarations: String,
    /// Each theorem whose statement was declared, and that statement.
    pub statements: BTreeMap<String, String>,
    /// What was not declared, and why.
    pub refused: Vec<declare::Refusal>,
    /// Production over the library and the declarations: its suffix holds
    /// the declarations' lines, then the produced theorems'.
    pub production: Production,
}

/// Declares the project's own declarations from the linked IR, continuing
/// the export `library`, and produces every theorem it can over them (§18.10).
/// Nothing is bound to a theorem Lean elaborated: no Lean compiled the
/// project, and each produced theorem's type is the declared statement's
/// body, lowered by the same code.
///
/// Without a library the lines are self-contained, for a checker that holds
/// the pinned library compiled.
///
/// # Errors
///
/// `LLV7103` when `library` is not lean4export 3.1.0 NDJSON. A declaration
/// that is not written is a refusal, not an error.
pub fn declare(
    modules: &[KernelModule],
    library: Option<&str>,
) -> Result<Declarations, Diagnostic> {
    let mut tables = export::Tables::parse(library.unwrap_or_default()).map_err(refusal)?;
    let world = World::new(modules);
    let declared = declare::write(&world, &mut tables, library.is_some());
    let declarations = tables.suffix();
    let production = produce_in(
        &world,
        tables,
        library.is_some(),
        false,
        Some(&declared.statements),
    );
    Ok(Declarations {
        declarations,
        statements: declared.statements,
        refused: declared.refused,
        production,
    })
}

/// The project's kernel declarations as one `lexlean/kernel-declarations/2`
/// document (§18.10, `lexlean declare` in §23.4): `library` says whether the
/// lines continue a library export; `theorems` holds, for each theorem in
/// production order, its name, the kind of its whole proof, its exact
/// declared axioms, its declared statement and semantic proof when it has
/// them, and `produced` (the constant its proof was written as) or `refused`
/// (why none was); `refused` holds each declaration not written, with its
/// reason; `suffix` is the lean4export lines, the declarations' first, then
/// the produced theorems'. An absent member is omitted, never `null` (§21.7).
///
/// Writing a byte literal recurses through each of its cells, so the work
/// runs on the compile stack (§8.3).
///
/// # Errors
///
/// As [`declare`]: `LLV7103` when `library` is not lean4export 3.1.0 NDJSON.
pub fn declarations_document(
    modules: &[KernelModule],
    library: Option<&str>,
) -> Result<crate::artifact::canonical_json::Json, Diagnostic> {
    use crate::artifact::canonical_json::Json;
    crate::on_stack(crate::config::MAX_COMPILE_STACK_BYTES, || {
        let declared = declare(modules, library)?;
        let mut proofs = BTreeMap::new();
        for module in modules {
            for declaration in &module.semantic.declarations {
                if let SemanticDeclaration::Theorem { name, proof, .. } = declaration {
                    proofs.insert(format!("{}.{name}", module.lean_module), proof);
                }
            }
        }
        let mut theorems = Vec::new();
        for row in &declared.production.rows {
            let mut fields = vec![
                ("theorem", Json::Str(row.theorem.clone())),
                ("form", Json::Str(row.form.clone())),
                ("axioms", Json::Arr(row.axioms.iter().cloned().map(Json::Str).collect())),
            ];
            if let Some(statement) = declared.statements.get(&row.theorem) {
                fields.push(("statement", Json::Str(statement.clone())));
            }
            if let Some(proof) = proofs.get(&row.theorem) {
                let value = serde_json::to_value(proof).map_err(|error| proof_refusal(&row.theorem, &error.to_string()))?;
                let proof = Json::from_serde(&value).map_err(|error| proof_refusal(&row.theorem, &error))?;
                fields.push(("proof", proof));
            }
            match &row.outcome {
                Outcome::Produced { constant, .. } => fields.push(("produced", Json::Str(constant.clone()))),
                Outcome::Refused { reason } => fields.push(("refused", Json::Str(reason.clone()))),
            }
            theorems.push(Json::object(fields));
        }
        let refused = declared
            .refused
            .iter()
            .map(|r| Json::object(vec![("name", Json::Str(r.name.clone())), ("reason", Json::Str(r.reason.clone()))]))
            .collect();
        Ok(Json::object(vec![
            ("spec", Json::Str("lexlean/kernel-declarations/2".to_owned())),
            ("library", Json::Bool(library.is_some())),
            ("theorems", Json::Arr(theorems)),
            ("refused", Json::Arr(refused)),
            ("suffix", Json::Str(declared.production.suffix)),
        ]))
    })
}

/// A theorem's semantic proof that does not serialize to canonical JSON: a
/// linked-IR invariant broken, reported rather than panicking (R5).
fn proof_refusal(theorem: &str, reason: &str) -> Diagnostic {
    refusal(format!("the proof of {theorem} is not canonical JSON: {reason}"))
}

/// Production over `tables`. With `checked`, a produced term may name only
/// constants the tables declare; with `bind`, each produced theorem is bound
/// to Lean's when the tables hold it, or always when they are not `checked`.
/// With `stated`, only a theorem whose statement was declared is produced.
fn produce_in(
    world: &World<'_>,
    mut tables: export::Tables,
    checked: bool,
    bind: bool,
    stated: Option<&BTreeMap<String, String>>,
) -> Production {
    let modules = world.modules;
    let pristine = tables.clone();
    // Each produced theorem's type, value, and the produced theorems it names.
    let mut terms: BTreeMap<String, (Expr, Expr, BTreeSet<String>)> = BTreeMap::new();
    let mut produced: BTreeMap<String, String> = BTreeMap::new();
    let mut rows = Vec::new();
    for (index, module) in modules.iter().enumerate() {
        for declaration in &module.semantic.declarations {
            let SemanticDeclaration::Theorem {
                name,
                type_parameters,
                parameters,
                statement,
                proof,
                axioms,
            } = declaration
            else {
                continue;
            };
            let lean = format!("{}.{name}", module.lean_module);
            let constant = format!("{NAMESPACE}.{lean}");
            let mark = tables.checkpoint();
            let attempt = (|| -> Result<Option<String>, String> {
                if stated.is_some_and(|stated| !stated.contains_key(&lean)) {
                    return Err("its statement is not declared".to_owned());
                }
                let mut ctx = Ctx::new(index);
                let mut binders: Vec<(String, u32, Expr)> = Vec::new();
                for parameter in type_parameters {
                    let id = world.fresh();
                    ctx.types.push((parameter.clone(), Expr::FVar(id)));
                    binders.push((parameter.clone(), id, sort(1)));
                }
                for parameter in parameters {
                    let id = world.fresh();
                    let ty = parameter.r#type.clone();
                    binders.push((parameter.name.clone(), id, world.ty(&ctx, &ty)?));
                    ctx.push(
                        &parameter.name,
                        Expr::FVar(id),
                        Some(lower::absolute(&ty, &module.name)),
                    );
                }
                let producers = prove::Produced {
                    world,
                    theorems: &produced,
                };
                let mut ty = world.term(&ctx, statement)?;
                let mut value = producers.prove(proof, statement, &ctx)?;
                for (binder, id, binder_ty) in binders.into_iter().rev() {
                    ty = pi(&binder, id, binder_ty.clone(), &ty);
                    value = lam(&binder, id, binder_ty, &value);
                }
                let mut named = BTreeSet::new();
                ty.constants(&mut named);
                value.constants(&mut named);
                if let Some(missing) = named.iter().find(|name| checked && !tables.declares(name)) {
                    return Err(format!("the library does not declare `{missing}`"));
                }
                tables.theorem(&constant, &ty, &value)?;
                let applied = named
                    .iter()
                    .filter_map(|name| {
                        name.strip_prefix(&format!("{NAMESPACE}."))
                            .map(str::to_owned)
                    })
                    .collect();
                terms.insert(lean.clone(), (ty.clone(), value.clone(), applied));
                if bind && (!checked || tables.declares(&lean)) {
                    let bound = format!("{NAMESPACE}.bound.{lean}");
                    tables.theorem(&bound, &ty, &Expr::Const(lean.clone(), Vec::new()))?;
                    return Ok(Some(bound));
                }
                Ok(None)
            })();
            let outcome = match attempt {
                Ok(bound) => {
                    produced.insert(lean.clone(), constant.clone());
                    Outcome::Produced { constant, bound }
                }
                Err(reason) => {
                    tables.rewind(&mark);
                    Outcome::Refused { reason }
                }
            };
            rows.push(Row {
                theorem: lean,
                form: proof_kind(proof),
                axioms: axioms.clone(),
                outcome,
            });
        }
    }
    Production {
        suffix: tables.suffix(),
        rows,
        pristine,
        terms,
    }
}

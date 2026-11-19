//! The axiom-policy table of a build (§22.6): every declaration whose
//! axioms verification audits, with the policy its observed set must meet and
//! the rule that applies it. Stage 11 of verification evaluates this table,
//! and `lexlean lean-modules` prints it (§23.4), so a host that elaborates the
//! modules itself applies exactly the rules `verify` applies.

use crate::api::RenderedModule;
use crate::backend::runtime::RuntimeModule;
use crate::link::CheckedProject;

/// A declared axiom policy, as §22.6 states it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Policy {
    /// The observed set must be empty.
    None,
    /// The observed set must be a subset.
    Allow(Vec<String>),
    /// The observed set must be equal.
    Exact(Vec<String>),
}

impl Policy {
    /// The policy kind token: `none`, `allow`, or `exact`.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Allow(_) => "allow",
            Self::Exact(_) => "exact",
        }
    }

    /// The axioms the policy names.
    #[must_use]
    pub fn axioms(&self) -> &[String] {
        match self {
            Self::None => &[],
            Self::Allow(names) | Self::Exact(names) => names,
        }
    }

    /// Does the policy permit an observed set? An exact policy compares the
    /// observed set sorted and without repeats.
    #[must_use]
    pub fn permits(&self, observed: &[String]) -> bool {
        match self {
            Self::None => observed.is_empty(),
            Self::Allow(allowed) => observed.iter().all(|axiom| allowed.contains(axiom)),
            Self::Exact(exact) => {
                let mut sorted = observed.to_vec();
                sorted.sort();
                sorted.dedup();
                sorted == *exact
            }
        }
    }

    fn of_declared(policy: &crate::ir::declaration::AxiomPolicy) -> Self {
        use crate::ir::declaration::AxiomPolicy;
        match policy {
            AxiomPolicy::None => Self::None,
            AxiomPolicy::Allow(names) => Self::Allow(names.clone()),
            AxiomPolicy::Exact(names) => Self::Exact(names.clone()),
        }
    }

    fn of_core(policy: &crate::ir::core::CoreAxiomPolicy) -> Self {
        use crate::ir::core::CoreAxiomPolicy;
        match policy {
            CoreAxiomPolicy::None => Self::None,
            CoreAxiomPolicy::Allow(names) => Self::Allow(names.clone()),
            CoreAxiomPolicy::Exact(names) => Self::Exact(names.clone()),
        }
    }

    /// A semantic declaration states its axioms exactly; with none stated,
    /// its kind is `none`.
    fn of_stated(axioms: &[String]) -> Self {
        if axioms.is_empty() {
            Self::None
        } else {
            Self::Exact(axioms.to_vec())
        }
    }
}

/// How an entry's policy is applied to what the audit observes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    /// A document declaration: the policy must permit the observed set, and a
    /// name the audit does not report is observed with no axioms.
    Document,
    /// A declaration of a native core module: the policy must permit the
    /// observed set, and a name the audit does not report is not checked.
    Core,
    /// A declaration of a semantic module: the observed set must equal the
    /// stated axioms, and a name the audit does not report fails the
    /// verification.
    Semantic,
    /// A declaration of a model (§17.12): every Lean declaration it generates,
    /// `members`, must observe a subset of the stated axioms, together they
    /// must observe exactly them, and a member the audit does not report fails
    /// the verification.
    Model {
        /// The full names of the Lean declarations the model generates.
        members: Vec<String>,
    },
    /// A declaration of a shipped runtime module: the observed set must equal
    /// its pinned axioms, and a name the audit does not report fails the
    /// verification.
    Runtime,
}

impl Rule {
    /// The rule token `lexlean lean-modules` prints.
    #[must_use]
    pub const fn token(&self) -> &'static str {
        match self {
            Self::Document => "document",
            Self::Core => "core",
            Self::Semantic => "semantic",
            Self::Model { .. } => "model",
            Self::Runtime => "runtime",
        }
    }
}

/// One declaration of the table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// The full Lean name of the declaration.
    pub name: String,
    /// Its declared policy.
    pub policy: Policy,
    /// The rule that applies the policy.
    pub rule: Rule,
}

/// The table of one Lean module, in the order stage 11 reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleTable {
    /// The Lean module name.
    pub lean_module: String,
    /// Its entries: a project module's document declarations, then its core
    /// declarations, then its semantic declarations, in source order.
    pub entries: Vec<Entry>,
}

/// The policy table of a rendered build: the project's modules in build
/// order, then the runtime modules the build imports (§22.6, §17.12).
#[must_use]
pub fn table(
    checked: &CheckedProject,
    modules: &[RenderedModule],
    runtime: &[&RuntimeModule],
) -> Vec<ModuleTable> {
    let mut tables = Vec::new();
    for module in modules {
        let document = &checked.modules[&module.module].document;
        let mut entries = Vec::new();
        for declaration in document.declarations() {
            entries.push(Entry {
                name: format!("{}.{}", module.lean_module, declaration.lean_name),
                policy: Policy::of_declared(&declaration.policy),
                rule: Rule::Document,
            });
        }
        if let Some(core) = &document.core {
            for declaration in &core.declarations {
                entries.push(Entry {
                    name: declaration.name.clone(),
                    policy: Policy::of_core(&declaration.policy),
                    rule: Rule::Core,
                });
            }
        }
        if let Some(semantic) = &document.semantic {
            for (index, declaration) in semantic.declarations.iter().enumerate() {
                let name = format!("{}.{}", module.lean_module, declaration.name());
                let policy = Policy::of_stated(declaration.axioms());
                if crate::ir::semantic::model::elaborated_construct(declaration).is_some() {
                    let members = semantic
                        .elaborated(index)
                        .unwrap_or_default()
                        .iter()
                        .map(|derived| derived.name().to_owned())
                        .chain(
                            semantic
                                .elaboration
                                .theorems(index)
                                .iter()
                                .map(|theorem| theorem.name.clone()),
                        )
                        .chain(
                            semantic
                                .elaboration
                                .checks(index)
                                .iter()
                                .map(|check| check.name.clone()),
                        )
                        .map(|member| format!("{}.{member}", module.lean_module))
                        .collect();
                    entries.push(Entry {
                        name,
                        policy,
                        rule: Rule::Model { members },
                    });
                } else {
                    entries.push(Entry {
                        name,
                        policy,
                        rule: Rule::Semantic,
                    });
                }
            }
        }
        tables.push(ModuleTable {
            lean_module: module.lean_module.clone(),
            entries,
        });
    }
    for module in runtime {
        tables.push(ModuleTable {
            lean_module: module.name.clone(),
            entries: module
                .declarations
                .iter()
                .map(|(name, axioms)| Entry {
                    name: name.clone(),
                    // A runtime declaration pins its axioms exactly, none
                    // included: its kind is `exact` either way.
                    policy: Policy::Exact(axioms.clone()),
                    rule: Rule::Runtime,
                })
                .collect(),
        });
    }
    tables
}

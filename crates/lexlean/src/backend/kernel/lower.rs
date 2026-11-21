//! The statement of a theorem as a kernel term, written from the linked IR
//! exactly as the Lean backend renders it (SPEC.md §18.4), with the implicit
//! arguments and instances Lean's elaborator fills in made explicit. Each IR
//! construct maps to one fixed term over the pinned library; a construct
//! outside that vocabulary is refused by name, never approximated.
//!
//! The lowering need not be syntactically Lean's elaboration: the produced
//! theorem's type is this term, and binding it to the theorem Lean
//! elaborated is a kernel definitional-equality check (§18.10).

use std::cell::Cell;
use std::collections::HashMap;

use super::expr::{app, arrow, c, cu, lam, let_in, pi, sort, Expr};
use super::KernelModule;
use crate::ir::semantic::{
    MemberRef, SemanticDeclaration, SemanticInteger, SemanticParameter, SemanticPrimitive,
    SemanticTerm, SemanticType,
};

/// A refusal, naming the construct.
pub type R<T> = Result<T, String>;

/// Every module of the project, by its name.
pub struct World<'a> {
    pub modules: &'a [KernelModule],
    by_name: HashMap<&'a str, usize>,
    next: Cell<u32>,
}

/// One constructor of an inductive shape.
pub struct Ctor {
    pub short: String,
    pub constant: String,
    pub fields: Vec<SemanticType>,
    /// Which fields are of the inductive type itself.
    pub recursive: Vec<bool>,
}

/// What `cases`, `induction`, and `match` need of an inductive type.
pub struct Shape {
    pub cases_on: String,
    pub rec: String,
    /// The type's own universe levels, after the motive's.
    pub levels: Vec<u32>,
    /// The type's parameters, as terms.
    pub params: Vec<Expr>,
    pub ctors: Vec<Ctor>,
    /// The constructors' universe levels.
    pub ctor_levels: Vec<u32>,
}

/// A local of the context: a value (or proof) and, for a value, its type.
#[derive(Clone)]
pub struct Local {
    pub name: String,
    pub expr: Expr,
    pub ty: Option<SemanticType>,
}

/// A lowering context: the module the IR is read in, its type parameters,
/// and its locals, innermost last.
#[derive(Clone)]
pub struct Ctx {
    pub module: usize,
    pub types: Vec<(String, Expr)>,
    pub locals: Vec<Local>,
}

impl<'a> World<'a> {
    #[must_use]
    pub fn new(modules: &'a [KernelModule]) -> Self {
        Self {
            modules,
            by_name: modules
                .iter()
                .enumerate()
                .map(|(index, module)| (module.name.as_str(), index))
                .collect(),
            next: Cell::new(0),
        }
    }

    /// A fresh free variable.
    pub fn fresh(&self) -> u32 {
        let id = self.next.get();
        self.next.set(id + 1);
        id
    }

    /// The module index a member names, read in module `at`.
    pub fn module_of(&self, at: usize, member: &MemberRef) -> R<usize> {
        match &member.module {
            None => Ok(at),
            Some(name) => self
                .by_name
                .get(name.as_str())
                .copied()
                .ok_or_else(|| format!("module `{name}` is not in the project")),
        }
    }

    fn declarations(&self, module: usize) -> impl Iterator<Item = &'a SemanticDeclaration> {
        let module = &self.modules[module];
        // An elaborated declaration is what Lean realizes for its name, so it
        // is found before the source declaration it stands for.
        module
            .elaborated
            .iter()
            .chain(module.semantic.declarations.iter())
    }

    /// The declaration a member names, and its module.
    pub fn find(&self, at: usize, member: &MemberRef) -> R<(usize, &'a SemanticDeclaration)> {
        let module = self.module_of(at, member)?;
        self.declarations(module)
            .find(|declaration| declaration.name() == member.name)
            .map(|declaration| (module, declaration))
            .ok_or_else(|| format!("`{}` is not a declaration of the project", member.name))
    }

    /// The Lean constant a member names, as the Lean backend writes it
    /// inside the namespace of module `at`.
    pub fn constant(&self, at: usize, member: &MemberRef) -> R<String> {
        if let Some(module) = &member.module {
            if module == crate::text_laws::PREFIX {
                return Ok(format!("{module}.{}", member.name));
            }
            let index = self.module_of(at, member)?;
            return Ok(format!(
                "{}.{}",
                self.modules[index].lean_module, member.name
            ));
        }
        match member.name.as_str() {
            "Result.ok" => return Ok("Except.ok".to_owned()),
            "Result.error" => return Ok("Except.error".to_owned()),
            _ => {}
        }
        let owner = member.name.split('.').next().unwrap_or_default();
        if crate::ir::semantic::BUILTIN_CONSTRUCTOR_OWNERS.contains(&owner) {
            return Ok(member.name.clone());
        }
        // A bare name resolves in the module's namespace first.
        let declared = self
            .declarations(at)
            .any(|declaration| declaration.name() == owner);
        Ok(if declared {
            format!("{}.{}", self.modules[at].lean_module, member.name)
        } else {
            member.name.clone()
        })
    }
}

/// A type with every document member made absolute to module `module`.
pub fn absolute(ty: &SemanticType, module: &str) -> SemanticType {
    let a = |inner: &SemanticType| Box::new(absolute(inner, module));
    match ty {
        SemanticType::Array { element } => SemanticType::Array {
            element: a(element),
        },
        SemanticType::Named { member, arguments } => SemanticType::Named {
            member: MemberRef {
                module: Some(member.module.clone().unwrap_or_else(|| module.to_owned())),
                name: member.name.clone(),
            },
            arguments: arguments.iter().map(|t| absolute(t, module)).collect(),
        },
        SemanticType::Option { value } => SemanticType::Option { value: a(value) },
        SemanticType::Result { ok, error } => SemanticType::Result {
            ok: a(ok),
            error: a(error),
        },
        SemanticType::List { element } => SemanticType::List {
            element: a(element),
        },
        SemanticType::Product { left, right } => SemanticType::Product {
            left: a(left),
            right: a(right),
        },
        SemanticType::Function { parameters, result } => SemanticType::Function {
            parameters: parameters.iter().map(|t| absolute(t, module)).collect(),
            result: a(result),
        },
        SemanticType::Map { key, value } => SemanticType::Map {
            key: a(key),
            value: a(value),
        },
        SemanticType::Set { element } => SemanticType::Set {
            element: a(element),
        },
        other @ (SemanticType::Type
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
        | SemanticType::ReasoningFailure) => other.clone(),
    }
}

/// A type with its type parameters replaced.
pub(super) fn substitute(ty: &SemanticType, by: &HashMap<String, SemanticType>) -> SemanticType {
    let s = |inner: &SemanticType| Box::new(substitute(inner, by));
    match ty {
        SemanticType::Array { element } => SemanticType::Array {
            element: s(element),
        },
        SemanticType::Parameter { name } => by.get(name).cloned().unwrap_or_else(|| ty.clone()),
        SemanticType::Named { member, arguments } => SemanticType::Named {
            member: member.clone(),
            arguments: arguments.iter().map(|t| substitute(t, by)).collect(),
        },
        SemanticType::Option { value } => SemanticType::Option { value: s(value) },
        SemanticType::Result { ok, error } => SemanticType::Result {
            ok: s(ok),
            error: s(error),
        },
        SemanticType::List { element } => SemanticType::List {
            element: s(element),
        },
        SemanticType::Product { left, right } => SemanticType::Product {
            left: s(left),
            right: s(right),
        },
        SemanticType::Function { parameters, result } => SemanticType::Function {
            parameters: parameters.iter().map(|t| substitute(t, by)).collect(),
            result: s(result),
        },
        SemanticType::Map { key, value } => SemanticType::Map {
            key: s(key),
            value: s(value),
        },
        SemanticType::Set { element } => SemanticType::Set {
            element: s(element),
        },
        other @ (SemanticType::Type
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
        | SemanticType::ReasoningFailure) => other.clone(),
    }
}

/// The universe `u` of `Eq.{u}` over values of a type.
fn eq_level(ty: &SemanticType) -> u32 {
    if *ty == SemanticType::Type {
        2
    } else {
        1
    }
}

fn kind_of_type(ty: &SemanticType) -> String {
    serde_json::to_value(ty)
        .ok()
        .and_then(|value| {
            value
                .get("kind")
                .and_then(|k| k.as_str().map(str::to_owned))
        })
        .unwrap_or_default()
}

fn kind_of_term(term: &SemanticTerm) -> String {
    serde_json::to_value(term)
        .ok()
        .and_then(|value| {
            value
                .get("kind")
                .and_then(|k| k.as_str().map(str::to_owned))
        })
        .unwrap_or_default()
}

/// The Lean name of an integer type, and whether it is signed.
fn integer_type(ty: &SemanticType) -> Option<(&'static str, bool)> {
    Some(match ty {
        SemanticType::Array { .. } => return None,
        SemanticType::Int => ("Int", true),
        SemanticType::Int8 => ("Int8", true),
        SemanticType::Int16 => ("Int16", true),
        SemanticType::Int32 => ("Int32", true),
        SemanticType::Int64 => ("Int64", true),
        SemanticType::UInt8 => ("UInt8", false),
        SemanticType::UInt16 => ("UInt16", false),
        SemanticType::UInt32 => ("UInt32", false),
        SemanticType::UInt64 => ("UInt64", false),
        SemanticType::Type
        | SemanticType::Parameter { .. }
        | SemanticType::Nat
        | SemanticType::Bool
        | SemanticType::Prop
        | SemanticType::Unit
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
        | SemanticType::ReasoningFailure => return None,
    })
}

fn integer_semantic(representation: SemanticInteger) -> SemanticType {
    match representation {
        SemanticInteger::Int => SemanticType::Int,
        SemanticInteger::Int8 => SemanticType::Int8,
        SemanticInteger::Int16 => SemanticType::Int16,
        SemanticInteger::Int32 => SemanticType::Int32,
        SemanticInteger::Int64 => SemanticType::Int64,
        SemanticInteger::UInt8 => SemanticType::UInt8,
        SemanticInteger::UInt16 => SemanticType::UInt16,
        SemanticInteger::UInt32 => SemanticType::UInt32,
        SemanticInteger::UInt64 => SemanticType::UInt64,
    }
}

/// An instance of the pinned Lean, read off the toolchain by `#synth`: the
/// one Lean's elaborator fills in for `class` at the number type `ty`.
fn number_instance(class: &str, ty: &SemanticType) -> R<String> {
    let name = match ty {
        SemanticType::Nat => "Nat",
        SemanticType::String if class == "Key" => "String",
        other => integer_type(other)
            .map(|(name, _)| name)
            .ok_or_else(|| format!("`{class}` at `{}`", kind_of_type(other)))?,
    };
    Ok(match (class, name) {
        ("Add", "Nat") => "instAddNat".to_owned(),
        ("Add", "Int") => "Int.instAdd".to_owned(),
        ("Add", other) => format!("instAdd{other}"),
        ("Mul", "Nat") => "instMulNat".to_owned(),
        ("Mul", "Int") => "Int.instMul".to_owned(),
        ("Mul", other) => format!("instMul{other}"),
        ("Neg", "Int") => "Int.instNegInt".to_owned(),
        // A signed width's own instance; an unsigned width's, Lean's top-level one.
        ("Neg", other) if other.starts_with('U') => format!("instNeg{other}"),
        ("Neg", other) => format!("{other}.instNeg"),
        ("Sub", "Nat") => "instSubNat".to_owned(),
        ("Sub", "Int") => "Int.instSub".to_owned(),
        ("Sub", other) => format!("instSub{other}"),
        ("LE", "Nat") => "instLENat".to_owned(),
        ("LE", "Int") => "Int.instLEInt".to_owned(),
        ("LE", other) => format!("instLE{other}"),
        ("LT", "Nat") => "instLTNat".to_owned(),
        ("LT", "Int") => "Int.instLTInt".to_owned(),
        ("LT", other) => format!("instLT{other}"),
        ("decLe", other) => format!("{other}.decLe"),
        ("decLt", other) => format!("{other}.decLt"),
        ("DecidableEq", "Nat") => "instDecidableEqNat".to_owned(),
        ("DecidableEq", "Int") => "Int.instDecidableEq".to_owned(),
        ("DecidableEq", other) => format!("instDecidableEq{other}"),
        ("Quotient", "Nat" | "Int") => format!("LexLeanRuntime.instQuotient{name}"),
        ("Key", other) => format!("LexLeanCollections.instKey{other}"),
        _ => return Err(format!("`{class}` at `{name}`")),
    })
}

impl Ctx {
    #[must_use]
    pub fn new(module: usize) -> Self {
        Self {
            module,
            types: Vec::new(),
            locals: Vec::new(),
        }
    }

    /// The innermost local of a name.
    pub fn local(&self, name: &str) -> R<&Local> {
        self.locals
            .iter()
            .rev()
            .find(|local| local.name == name)
            .ok_or_else(|| format!("`{name}` is not in scope"))
    }

    pub fn push(&mut self, name: &str, expr: Expr, ty: Option<SemanticType>) {
        self.locals.push(Local {
            name: name.to_owned(),
            expr,
            ty,
        });
    }

    /// Replaces a free variable in every local's value, as `cases` does in
    /// every hypothesis.
    pub fn substitute(&mut self, id: u32, by: &Expr) {
        for local in &mut self.locals {
            local.expr = local.expr.replace_fvar(id, by);
        }
    }
}

impl World<'_> {
    fn module_name(&self, ctx: &Ctx) -> &str {
        &self.modules[ctx.module].name
    }

    /// A type as a kernel term.
    pub fn ty(&self, ctx: &Ctx, ty: &SemanticType) -> R<Expr> {
        Ok(match ty {
            SemanticType::Array { element } => app(cu("Array", &[0]), [self.ty(ctx, element)?]),
            SemanticType::Type => sort(1),
            SemanticType::Prop => sort(0),
            SemanticType::Parameter { name } => ctx
                .types
                .iter()
                .rev()
                .find(|(candidate, _)| candidate == name)
                .map(|(_, expr)| expr.clone())
                .ok_or_else(|| format!("type parameter `{name}` is not in scope"))?,
            SemanticType::Nat => c("Nat"),
            SemanticType::Bool => c("Bool"),
            SemanticType::Unit => c("Unit"),
            SemanticType::Int
            | SemanticType::Int8
            | SemanticType::Int16
            | SemanticType::Int32
            | SemanticType::Int64
            | SemanticType::UInt8
            | SemanticType::UInt16
            | SemanticType::UInt32
            | SemanticType::UInt64 => c(integer_type(ty).map(|(name, _)| name).unwrap_or_default()),
            SemanticType::String => c("String"),
            SemanticType::Bytes => c("ByteArray"),
            SemanticType::Ordering => c("Ordering"),
            SemanticType::Option { value } => app(cu("Option", &[0]), [self.ty(ctx, value)?]),
            SemanticType::Result { ok, error } => app(
                cu("Except", &[0, 0]),
                [self.ty(ctx, error)?, self.ty(ctx, ok)?],
            ),
            SemanticType::List { element } => app(cu("List", &[0]), [self.ty(ctx, element)?]),
            SemanticType::Product { left, right } => app(
                cu("Prod", &[0, 0]),
                [self.ty(ctx, left)?, self.ty(ctx, right)?],
            ),
            SemanticType::Function { parameters, result } => {
                let mut out = self.ty(ctx, result)?;
                for parameter in parameters.iter().rev() {
                    out = arrow(self.ty(ctx, parameter)?, out);
                }
                out
            }
            // §17.12: a map is its canonical ascending entry list, a set its
            // ascending element list.
            SemanticType::Map { key, value } => app(
                cu("List", &[0]),
                [app(
                    cu("Prod", &[0, 0]),
                    [self.ty(ctx, key)?, self.ty(ctx, value)?],
                )],
            ),
            SemanticType::Set { element } => app(cu("List", &[0]), [self.ty(ctx, element)?]),
            SemanticType::Named { member, arguments } => {
                let (_, declaration) = self.find(ctx.module, member)?;
                match declaration {
                    SemanticDeclaration::Inductive { parameters, .. }
                    | SemanticDeclaration::Structure { parameters, .. }
                        if parameters.is_empty() => {}
                    SemanticDeclaration::Inductive { .. }
                    | SemanticDeclaration::Structure { .. } => {
                        return Err(format!("type `{}` has value parameters", member.name))
                    }
                    _ => {
                        return Err(format!(
                            "`{}` is not an inductive type or structure",
                            member.name
                        ))
                    }
                }
                let mut arguments_out = Vec::new();
                for argument in arguments {
                    arguments_out.push(self.ty(ctx, argument)?);
                }
                app(c(&self.constant(ctx.module, member)?), arguments_out)
            }
            // A contract violation and a reasoning failure are pairs of
            // Booleans (§18.4).
            SemanticType::ContractViolation | SemanticType::ReasoningFailure => {
                app(cu("Prod", &[0, 0]), [c("Bool"), c("Bool")])
            }
        })
    }

    /// The constructors and eliminators of a type.
    pub fn shape(&self, ctx: &Ctx, ty: &SemanticType) -> R<Shape> {
        let builtin = |name: &str,
                       levels: Vec<u32>,
                       params: Vec<Expr>,
                       ctors: Vec<(&str, Vec<SemanticType>)>| {
            let ctor_levels = levels.clone();
            Shape {
                cases_on: format!("{name}.casesOn"),
                rec: format!("{name}.rec"),
                levels,
                params,
                ctors: ctors
                    .into_iter()
                    .map(|(short, fields)| Ctor {
                        short: short.to_owned(),
                        constant: format!("{name}.{short}"),
                        recursive: fields.iter().map(|field| field == ty).collect(),
                        fields,
                    })
                    .collect(),
                ctor_levels,
            }
        };
        Ok(match ty {
            SemanticType::Array { .. } => return Err("case analysis on type `array`".to_owned()),
            SemanticType::Nat => builtin(
                "Nat",
                vec![],
                vec![],
                vec![("zero", vec![]), ("succ", vec![SemanticType::Nat])],
            ),
            SemanticType::Bool => builtin(
                "Bool",
                vec![],
                vec![],
                vec![("false", vec![]), ("true", vec![])],
            ),
            SemanticType::List { element } => builtin(
                "List",
                vec![0],
                vec![self.ty(ctx, element)?],
                vec![
                    ("nil", vec![]),
                    ("cons", vec![(**element).clone(), ty.clone()]),
                ],
            ),
            SemanticType::Option { value } => builtin(
                "Option",
                vec![0],
                vec![self.ty(ctx, value)?],
                vec![("none", vec![]), ("some", vec![(**value).clone()])],
            ),
            SemanticType::Product { left, right } => builtin(
                "Prod",
                vec![0, 0],
                vec![self.ty(ctx, left)?, self.ty(ctx, right)?],
                vec![("mk", vec![(**left).clone(), (**right).clone()])],
            ),
            // `Except ε α`, its error constructor first.
            SemanticType::Result { ok, error } => builtin(
                "Except",
                vec![0, 0],
                vec![self.ty(ctx, error)?, self.ty(ctx, ok)?],
                vec![
                    ("error", vec![(**error).clone()]),
                    ("ok", vec![(**ok).clone()]),
                ],
            ),
            SemanticType::Named { member, arguments } => {
                let (module, declaration) = self.find(ctx.module, member)?;
                let owner = self.constant(ctx.module, member)?;
                let module_name = self.modules[module].name.clone();
                let (type_parameters, constructors): (
                    &Vec<String>,
                    Vec<(String, Vec<SemanticType>)>,
                ) = match declaration {
                    SemanticDeclaration::Inductive {
                        type_parameters,
                        parameters,
                        constructors,
                        mutual,
                        ..
                    } => {
                        // A mutual type has its own `casesOn` too (§18.10).
                        let _ = mutual;
                        if !parameters.is_empty() {
                            return Err(format!("type `{}` has value parameters", member.name));
                        }
                        (
                            type_parameters,
                            constructors
                                .iter()
                                .map(|k| (k.name.clone(), k.fields.clone()))
                                .collect(),
                        )
                    }
                    SemanticDeclaration::Structure {
                        type_parameters,
                        parameters,
                        fields,
                        ..
                    } if parameters.is_empty() => (
                        type_parameters,
                        vec![(
                            "mk".to_owned(),
                            fields.iter().map(|field| field.r#type.clone()).collect(),
                        )],
                    ),
                    _ => return Err(format!("`{}` is not an inductive type", member.name)),
                };
                let by: HashMap<String, SemanticType> = type_parameters
                    .iter()
                    .cloned()
                    .zip(arguments.iter().cloned())
                    .collect();
                let mut params = Vec::new();
                for argument in arguments {
                    params.push(self.ty(ctx, argument)?);
                }
                let ctors = constructors
                    .into_iter()
                    .map(|(short, fields)| {
                        let fields: Vec<SemanticType> = fields
                            .iter()
                            .map(|field| substitute(&absolute(field, &module_name), &by))
                            .collect();
                        Ctor {
                            constant: format!("{owner}.{short}"),
                            recursive: fields.iter().map(|field| field == ty).collect(),
                            short,
                            fields,
                        }
                    })
                    .collect();
                Shape {
                    cases_on: format!("{owner}.casesOn"),
                    rec: format!("{owner}.rec"),
                    levels: vec![],
                    params,
                    ctors,
                    ctor_levels: vec![],
                }
            }
            other @ (SemanticType::Type
            | SemanticType::Parameter { .. }
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
            | SemanticType::Function { .. }
            | SemanticType::Map { .. }
            | SemanticType::Set { .. }
            | SemanticType::ContractViolation
            | SemanticType::ReasoningFailure) => {
                return Err(format!("case analysis on type `{}`", kind_of_type(other)))
            }
        })
    }

    /// The signature of a definition: its type parameters, parameters, and
    /// result, made absolute to its module and instantiated.
    fn signature(
        &self,
        ctx: &Ctx,
        function: &MemberRef,
        type_arguments: &[SemanticType],
    ) -> R<(
        usize,
        &SemanticDeclaration,
        Vec<SemanticParameter>,
        SemanticType,
    )> {
        let (module, declaration) = self.find(ctx.module, function)?;
        let SemanticDeclaration::Definition {
            type_parameters,
            parameters,
            result,
            ..
        } = declaration
        else {
            return Err(format!("`{}` is not a definition", function.name));
        };
        if type_parameters.len() != type_arguments.len() {
            return Err(format!(
                "`{}` takes {} type arguments",
                function.name,
                type_parameters.len()
            ));
        }
        let module_name = &self.modules[module].name;
        let by: HashMap<String, SemanticType> = type_parameters
            .iter()
            .cloned()
            .zip(
                type_arguments
                    .iter()
                    .map(|t| absolute(t, self.module_name(ctx))),
            )
            .collect();
        let parameters = parameters
            .iter()
            .map(|parameter| SemanticParameter {
                name: parameter.name.clone(),
                r#type: substitute(&absolute(&parameter.r#type, module_name), &by),
            })
            .collect();
        Ok((
            module,
            declaration,
            parameters,
            substitute(&absolute(result, module_name), &by),
        ))
    }

    /// The fields of a structure type, made absolute and instantiated.
    fn structure_fields(
        &self,
        ctx: &Ctx,
        ty: &SemanticType,
    ) -> R<(String, Vec<(String, SemanticType)>)> {
        let SemanticType::Named { member, arguments } = ty else {
            return Err(format!("projection from `{}`", kind_of_type(ty)));
        };
        let (module, declaration) = self.find(ctx.module, member)?;
        let SemanticDeclaration::Structure {
            type_parameters,
            fields,
            ..
        } = declaration
        else {
            return Err(format!("`{}` is not a structure", member.name));
        };
        let by: HashMap<String, SemanticType> = type_parameters
            .iter()
            .cloned()
            .zip(arguments.iter().cloned())
            .collect();
        let module_name = self.modules[module].name.clone();
        Ok((
            self.constant(ctx.module, member)?,
            fields
                .iter()
                .map(|field| {
                    (
                        field.name.clone(),
                        substitute(&absolute(&field.r#type, &module_name), &by),
                    )
                })
                .collect(),
        ))
    }

    /// The type of a constructor application and its fields.
    fn constructor_type(
        &self,
        ctx: &Ctx,
        constructor: &MemberRef,
        type_arguments: &[SemanticType],
    ) -> R<SemanticType> {
        let here = self.module_name(ctx).to_owned();
        let targs: Vec<SemanticType> = type_arguments.iter().map(|t| absolute(t, &here)).collect();
        if constructor.module.is_none() {
            if let Some(owner) = constructor.name.split('.').next() {
                if crate::backend::semantic::violation_pattern(constructor).is_some() {
                    return Ok(if owner == "ContractViolation" {
                        SemanticType::ContractViolation
                    } else {
                        SemanticType::ReasoningFailure
                    });
                }
            }
            let one = || {
                targs
                    .first()
                    .cloned()
                    .ok_or_else(|| format!("`{}` without its type", constructor.name))
            };
            match constructor.name.as_str() {
                "Nat.zero" | "Nat.succ" => return Ok(SemanticType::Nat),
                "Bool.true" | "Bool.false" => return Ok(SemanticType::Bool),
                "List.nil" | "List.cons" => {
                    return Ok(SemanticType::List {
                        element: Box::new(one()?),
                    })
                }
                "Option.none" | "Option.some" => {
                    return Ok(SemanticType::Option {
                        value: Box::new(one()?),
                    })
                }
                "Result.ok" | "Result.error" => {
                    let [ok, error] = targs.as_slice() else {
                        return Err(format!("`{}` without its two types", constructor.name));
                    };
                    return Ok(SemanticType::Result {
                        ok: Box::new(ok.clone()),
                        error: Box::new(error.clone()),
                    });
                }
                _ => {}
            }
        }
        let (owner, _) = constructor
            .name
            .rsplit_once('.')
            .ok_or_else(|| format!("constructor `{}`", constructor.name))?;
        let member = MemberRef {
            module: Some(constructor.module.clone().unwrap_or_else(|| here.clone())),
            name: owner.to_owned(),
        };
        self.find(ctx.module, &member)?;
        Ok(SemanticType::Named {
            member,
            arguments: targs,
        })
    }

    /// The type of a term, every document member absolute.
    #[allow(clippy::too_many_lines)]
    pub fn infer(&self, ctx: &Ctx, term: &SemanticTerm) -> R<SemanticType> {
        let here = self.module_name(ctx).to_owned();
        Ok(match term {
            SemanticTerm::Var { name } => ctx
                .local(name)?
                .ty
                .clone()
                .ok_or_else(|| format!("`{name}` is a proof, not a value"))?,
            SemanticTerm::Nat { .. } => SemanticType::Nat,
            SemanticTerm::Add { left, right } => {
                let ty = self.infer(ctx, left).or_else(|_| self.infer(ctx, right))?;
                number_instance("Add", &ty)?;
                ty
            }
            SemanticTerm::String { .. } => SemanticType::String,
            SemanticTerm::Integer { representation, .. } => integer_semantic(*representation),
            SemanticTerm::Primitive { result, .. } => absolute(result, &here),
            SemanticTerm::MapLiteral { key, value, .. } => SemanticType::Map {
                key: Box::new(absolute(key, &here)),
                value: Box::new(absolute(value, &here)),
            },
            SemanticTerm::SetLiteral { element, .. } => SemanticType::Set {
                element: Box::new(absolute(element, &here)),
            },
            SemanticTerm::GraphLiteral { node, .. } => {
                let node = absolute(node, &here);
                SemanticType::Map {
                    key: Box::new(node.clone()),
                    value: Box::new(SemanticType::Set {
                        element: Box::new(node),
                    }),
                }
            }
            SemanticTerm::Bool { .. }
            | SemanticTerm::Beq { .. }
            | SemanticTerm::Ble { .. }
            | SemanticTerm::Blt { .. }
            | SemanticTerm::And { .. }
            | SemanticTerm::Or { .. }
            | SemanticTerm::Not { .. } => SemanticType::Bool,
            SemanticTerm::Unit => SemanticType::Unit,
            SemanticTerm::Nil { element } => SemanticType::List {
                element: Box::new(absolute(element, &here)),
            },
            SemanticTerm::Cons { head, .. } => SemanticType::List {
                element: Box::new(self.infer(ctx, head)?),
            },
            SemanticTerm::Record {
                r#type,
                type_arguments,
                ..
            } => SemanticType::Named {
                member: MemberRef {
                    module: Some(r#type.module.clone().unwrap_or_else(|| here.clone())),
                    name: r#type.name.clone(),
                },
                arguments: type_arguments.iter().map(|t| absolute(t, &here)).collect(),
            },
            SemanticTerm::Constructor {
                constructor,
                type_arguments,
                ..
            } => self.constructor_type(ctx, constructor, type_arguments)?,
            SemanticTerm::Project { value, field } => {
                let ty = self.infer(ctx, value)?;
                let (_, fields) = self.structure_fields(ctx, &ty)?;
                fields
                    .into_iter()
                    .find(|(name, _)| name == field)
                    .map(|(_, ty)| ty)
                    .ok_or_else(|| format!("no field `{field}`"))?
            }
            SemanticTerm::Call {
                function,
                type_arguments,
                ..
            } => self.signature(ctx, function, type_arguments)?.3,
            SemanticTerm::If { then_value, .. } => self.infer(ctx, then_value)?,
            SemanticTerm::Match {
                scrutinee,
                branches,
            } => {
                let ty = self.infer(ctx, scrutinee)?;
                let branch = branches.first().ok_or("a match without branches")?;
                // A failure's cases bind nothing.
                if matches!(
                    ty,
                    SemanticType::ContractViolation | SemanticType::ReasoningFailure
                ) {
                    return self.infer(ctx, &branch.body);
                }
                let shape = self.shape(ctx, &ty)?;
                let short = branch
                    .constructor
                    .name
                    .rsplit('.')
                    .next()
                    .unwrap_or_default();
                let ctor = shape
                    .ctors
                    .iter()
                    .find(|k| k.short == short)
                    .ok_or_else(|| format!("constructor `{}`", branch.constructor.name))?;
                let mut inner = ctx.clone();
                for (binder, field) in branch.binders.iter().zip(&ctor.fields) {
                    inner.push(binder, Expr::FVar(u32::MAX), Some(field.clone()));
                }
                self.infer(&inner, &branch.body)?
            }
            SemanticTerm::Eq { .. }
            | SemanticTerm::Le { .. }
            | SemanticTerm::Lt { .. }
            | SemanticTerm::PropAnd { .. }
            | SemanticTerm::Implies { .. }
            | SemanticTerm::Iff { .. }
            | SemanticTerm::Forall { .. } => SemanticType::Prop,
            SemanticTerm::Let { binder, body, .. } => {
                let mut inner = ctx.clone();
                inner.push(
                    &binder.name,
                    Expr::FVar(u32::MAX),
                    Some(absolute(&binder.r#type, &here)),
                );
                self.infer(&inner, body)?
            }
            SemanticTerm::Pair { left, right } => SemanticType::Product {
                left: Box::new(self.infer(ctx, left)?),
                right: Box::new(self.infer(ctx, right)?),
            },
            SemanticTerm::First { value } | SemanticTerm::Second { value } => {
                let SemanticType::Product { left, right } = self.infer(ctx, value)? else {
                    return Err("a projection from a non-pair".to_owned());
                };
                if matches!(term, SemanticTerm::First { .. }) {
                    *left
                } else {
                    *right
                }
            }
            SemanticTerm::Lambda {
                parameters, body, ..
            } => {
                let mut inner = ctx.clone();
                for parameter in parameters {
                    inner.push(
                        &parameter.name,
                        Expr::FVar(u32::MAX),
                        Some(absolute(&parameter.r#type, &here)),
                    );
                }
                SemanticType::Function {
                    parameters: parameters
                        .iter()
                        .map(|parameter| absolute(&parameter.r#type, &here))
                        .collect(),
                    result: Box::new(self.infer(&inner, body)?),
                }
            }
            SemanticTerm::Apply { function, .. } => {
                let SemanticType::Function { result, .. } = self.infer(ctx, function)? else {
                    return Err("an application of a non-function".to_owned());
                };
                *result
            }
            SemanticTerm::FunctionRef {
                function,
                type_arguments,
            } => {
                let (_, _, parameters, result) = self.signature(ctx, function, type_arguments)?;
                SemanticType::Function {
                    parameters: parameters.into_iter().map(|p| p.r#type).collect(),
                    result: Box::new(result),
                }
            }
            SemanticTerm::Bytes { .. } => SemanticType::Bytes,
            other @ (SemanticTerm::InstanceValue { .. } | SemanticTerm::CheckedApply { .. }) => {
                return Err(format!("term `{}`", kind_of_term(other)))
            }
        })
    }

    /// A term as a kernel term.
    #[allow(clippy::too_many_lines)]
    pub fn term(&self, ctx: &Ctx, term: &SemanticTerm) -> R<Expr> {
        let nat_literal = |value: &str| {
            let n = Expr::Nat(value.to_owned());
            app(
                cu("OfNat.ofNat", &[0]),
                [c("Nat"), n.clone(), app(c("instOfNatNat"), [n])],
            )
        };
        let here = self.module_name(ctx).to_owned();
        Ok(match term {
            SemanticTerm::Var { name } => ctx.local(name)?.expr.clone(),
            SemanticTerm::Nat { value } => nat_literal(value),
            SemanticTerm::String { value } => Expr::Str(value.clone()),
            SemanticTerm::Integer {
                representation,
                value,
            } => {
                let ty = integer_semantic(*representation);
                let (name, signed) = integer_type(&ty).unwrap_or(("Int", true));
                let (negative, magnitude) = match value.strip_prefix('-') {
                    Some(magnitude) => (true, magnitude),
                    None => (false, value.as_str()),
                };
                let n = Expr::Nat(magnitude.to_owned());
                let instance = if name == "Int" {
                    "instOfNat".to_owned()
                } else {
                    format!("{name}.instOfNat")
                };
                let literal = app(
                    cu("OfNat.ofNat", &[0]),
                    [c(name), n.clone(), app(c(&instance), [n])],
                );
                if negative {
                    if !signed {
                        return Err(format!("a negative `{name}` literal"));
                    }
                    let neg = if name == "Int" {
                        "Int.instNegInt".to_owned()
                    } else {
                        format!("{name}.instNeg")
                    };
                    app(cu("Neg.neg", &[0]), [c(name), c(&neg), literal])
                } else {
                    literal
                }
            }
            SemanticTerm::Primitive {
                operation,
                arguments,
                ..
            } => self.primitive(ctx, *operation, arguments)?,
            SemanticTerm::MapLiteral {
                key,
                value,
                entries,
            } => {
                let (key, value) = (absolute(key, &here), absolute(value, &here));
                let pair = app(
                    cu("Prod", &[0, 0]),
                    [self.ty(ctx, &key)?, self.ty(ctx, &value)?],
                );
                let mut out = app(cu("List.nil", &[0]), [pair.clone()]);
                for entry in entries.iter().rev() {
                    let element = app(
                        cu("Prod.mk", &[0, 0]),
                        [
                            self.ty(ctx, &key)?,
                            self.ty(ctx, &value)?,
                            self.term(ctx, &entry.key)?,
                            self.term(ctx, &entry.value)?,
                        ],
                    );
                    out = app(cu("List.cons", &[0]), [pair.clone(), element, out]);
                }
                out
            }
            SemanticTerm::SetLiteral { element, elements } => {
                let element = self.ty(ctx, &absolute(element, &here))?;
                let mut out = app(cu("List.nil", &[0]), [element.clone()]);
                for item in elements.iter().rev() {
                    out = app(
                        cu("List.cons", &[0]),
                        [element.clone(), self.term(ctx, item)?, out],
                    );
                }
                out
            }
            SemanticTerm::GraphLiteral { node, nodes, edges } => {
                let node_ty = self.ty(ctx, &absolute(node, &here))?;
                let targets_ty = app(cu("List", &[0]), [node_ty.clone()]);
                let pair = app(cu("Prod", &[0, 0]), [node_ty.clone(), targets_ty.clone()]);
                let mut out = app(cu("List.nil", &[0]), [pair.clone()]);
                for source in nodes.iter().rev() {
                    let mut targets = app(cu("List.nil", &[0]), [node_ty.clone()]);
                    for edge in edges.iter().filter(|edge| edge.source == *source).rev() {
                        targets = app(
                            cu("List.cons", &[0]),
                            [node_ty.clone(), self.term(ctx, &edge.target)?, targets],
                        );
                    }
                    let element = app(
                        cu("Prod.mk", &[0, 0]),
                        [
                            node_ty.clone(),
                            targets_ty.clone(),
                            self.term(ctx, source)?,
                            targets,
                        ],
                    );
                    out = app(cu("List.cons", &[0]), [pair.clone(), element, out]);
                }
                out
            }
            SemanticTerm::Bool { value } => c(if *value { "Bool.true" } else { "Bool.false" }),
            SemanticTerm::Unit => c("Unit.unit"),
            SemanticTerm::Nil { element } => app(
                cu("List.nil", &[0]),
                [self.ty(ctx, &absolute(element, &here))?],
            ),
            SemanticTerm::Cons { head, tail } => {
                let element = self.infer(ctx, head)?;
                app(
                    cu("List.cons", &[0]),
                    [
                        self.ty(ctx, &element)?,
                        self.term(ctx, head)?,
                        self.term(ctx, tail)?,
                    ],
                )
            }
            SemanticTerm::Record { fields, .. } => {
                let ty = self.infer(ctx, term)?;
                let (owner, declared) = self.structure_fields(ctx, &ty)?;
                let SemanticType::Named { arguments, .. } = &ty else {
                    return Err("a record of a non-structure".to_owned());
                };
                let mut out = Vec::new();
                for argument in arguments {
                    out.push(self.ty(ctx, argument)?);
                }
                for (name, _) in &declared {
                    let value = fields
                        .iter()
                        .find(|row| row.field == *name)
                        .ok_or_else(|| format!("a record without `{name}`"))?;
                    out.push(self.term(ctx, &value.value)?);
                }
                app(c(&format!("{owner}.mk")), out)
            }
            SemanticTerm::Constructor { constructor, .. }
                if crate::backend::semantic::violation_pattern(constructor).is_some() =>
            {
                // A contract violation or reasoning failure is its pair of Booleans (§17.12).
                let pattern =
                    crate::backend::semantic::violation_pattern(constructor).unwrap_or_default();
                let flag = |part: &str| {
                    c(if part.contains("true") {
                        "Bool.true"
                    } else {
                        "Bool.false"
                    })
                };
                let (first, second) = pattern.split_once(',').unwrap_or_default();
                app(
                    cu("Prod.mk", &[0, 0]),
                    [c("Bool"), c("Bool"), flag(first), flag(second)],
                )
            }
            SemanticTerm::Constructor {
                constructor,
                type_arguments,
                arguments,
            } => {
                let ty = self.constructor_type(ctx, constructor, type_arguments)?;
                let mut out = Vec::new();
                let levels: &[u32] = match &ty {
                    SemanticType::Array { .. } => return Err("a constructor of `array`".to_owned()),
                    SemanticType::Nat | SemanticType::Bool => &[],
                    SemanticType::List { element } | SemanticType::Option { value: element } => {
                        out.push(self.ty(ctx, element)?);
                        &[0]
                    }
                    SemanticType::Result { ok, error } => {
                        out.push(self.ty(ctx, error)?);
                        out.push(self.ty(ctx, ok)?);
                        &[0, 0]
                    }
                    SemanticType::Named {
                        arguments: targs, ..
                    } => {
                        for argument in targs {
                            out.push(self.ty(ctx, argument)?);
                        }
                        &[]
                    }
                    other @ (SemanticType::Type
                    | SemanticType::Parameter { .. }
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
                    | SemanticType::Product { .. }
                    | SemanticType::Function { .. }
                    | SemanticType::Map { .. }
                    | SemanticType::Set { .. }
                    | SemanticType::ContractViolation
                    | SemanticType::ReasoningFailure) => {
                        return Err(format!("a constructor of `{}`", kind_of_type(other)))
                    }
                };
                for argument in arguments {
                    out.push(self.term(ctx, argument)?);
                }
                app(cu(&self.constant(ctx.module, constructor)?, levels), out)
            }
            SemanticTerm::Project { value, field } => {
                let ty = self.infer(ctx, value)?;
                let (owner, _) = self.structure_fields(ctx, &ty)?;
                let SemanticType::Named { arguments, .. } = &ty else {
                    return Err("a projection from a non-structure".to_owned());
                };
                let mut out = Vec::new();
                for argument in arguments {
                    out.push(self.ty(ctx, argument)?);
                }
                out.push(self.term(ctx, value)?);
                app(c(&format!("{owner}.{field}")), out)
            }
            SemanticTerm::Call {
                function,
                type_arguments,
                arguments,
            } => {
                let mut out = Vec::new();
                for argument in type_arguments {
                    out.push(self.ty(ctx, &absolute(argument, &here))?);
                }
                for argument in arguments {
                    out.push(self.term(ctx, argument)?);
                }
                app(c(&self.constant(ctx.module, function)?), out)
            }
            SemanticTerm::FunctionRef {
                function,
                type_arguments,
            } => {
                let mut out = Vec::new();
                for argument in type_arguments {
                    out.push(self.ty(ctx, &absolute(argument, &here))?);
                }
                app(c(&self.constant(ctx.module, function)?), out)
            }
            SemanticTerm::If {
                condition,
                then_value,
                else_value,
            } => {
                let ty = self.infer(ctx, then_value)?;
                let (proposition, instance) = if self.infer(ctx, condition)? == SemanticType::Bool {
                    let b = self.term(ctx, condition)?;
                    (
                        app(cu("Eq", &[1]), [c("Bool"), b.clone(), c("Bool.true")]),
                        app(c("instDecidableEqBool"), [b, c("Bool.true")]),
                    )
                } else {
                    (self.term(ctx, condition)?, self.decidable(ctx, condition)?)
                };
                app(
                    cu("ite", &[eq_level(&ty)]),
                    [
                        self.ty(ctx, &ty)?,
                        proposition,
                        instance,
                        self.term(ctx, then_value)?,
                        self.term(ctx, else_value)?,
                    ],
                )
            }
            SemanticTerm::Match {
                scrutinee,
                branches,
            } if matches!(
                self.infer(ctx, scrutinee)?,
                SemanticType::ContractViolation | SemanticType::ReasoningFailure
            ) =>
            {
                // A pair of Booleans: `Prod.casesOn`, then each Boolean in turn,
                // `false` first, each case the branch of its pattern (§17.12).
                let result = self.infer(ctx, term)?;
                let level = eq_level(&result);
                let result_ty = self.ty(ctx, &result)?;
                let branch = |first: bool, second: bool| -> R<Expr> {
                    let wanted = format!("({first}, {second})");
                    let found = branches
                        .iter()
                        .find(|b| {
                            crate::backend::semantic::violation_pattern(&b.constructor)
                                == Some(wanted.as_str())
                        })
                        .ok_or_else(|| format!("a match without the case `{wanted}`"))?;
                    self.term(ctx, &found.body)
                };
                let bool_motive = lam("b", self.fresh(), c("Bool"), &result_ty);
                let by_second = |first: bool, b: u32| -> R<Expr> {
                    Ok(app(
                        cu("Bool.casesOn", &[level]),
                        [
                            bool_motive.clone(),
                            Expr::FVar(b),
                            branch(first, false)?,
                            branch(first, true)?,
                        ],
                    ))
                };
                let (a, b) = (self.fresh(), self.fresh());
                let inner = app(
                    cu("Bool.casesOn", &[level]),
                    [
                        bool_motive.clone(),
                        Expr::FVar(a),
                        by_second(false, b)?,
                        by_second(true, b)?,
                    ],
                );
                let pair = app(cu("Prod", &[0, 0]), [c("Bool"), c("Bool")]);
                let minor = lam("a", a, c("Bool"), &lam("b", b, c("Bool"), &inner));
                app(
                    cu("Prod.casesOn", &[level, 0, 0]),
                    [
                        c("Bool"),
                        c("Bool"),
                        lam("x", self.fresh(), pair, &result_ty),
                        self.term(ctx, scrutinee)?,
                        minor,
                    ],
                )
            }
            SemanticTerm::Match {
                scrutinee,
                branches,
            } => {
                let scrutinee_ty = self.infer(ctx, scrutinee)?;
                let result = self.infer(ctx, term)?;
                let shape = self.shape(ctx, &scrutinee_ty)?;
                let x = self.fresh();
                let motive = lam(
                    "x",
                    x,
                    self.ty(ctx, &scrutinee_ty)?,
                    &self.ty(ctx, &result)?,
                );
                let mut minors = Vec::new();
                for ctor in &shape.ctors {
                    let branch = branches
                        .iter()
                        .find(|b| {
                            b.constructor.name.rsplit('.').next() == Some(ctor.short.as_str())
                        })
                        .ok_or_else(|| format!("a match without `{}`", ctor.short))?;
                    if branch.binders.len() != ctor.fields.len() {
                        return Err(format!(
                            "branch `{}` binds {} fields",
                            ctor.short,
                            branch.binders.len()
                        ));
                    }
                    let mut inner = ctx.clone();
                    let mut bound = Vec::new();
                    for (binder, field) in branch.binders.iter().zip(&ctor.fields) {
                        let id = self.fresh();
                        inner.push(binder, Expr::FVar(id), Some(field.clone()));
                        bound.push((binder, id, self.ty(ctx, field)?));
                    }
                    let mut body = self.term(&inner, &branch.body)?;
                    for (binder, id, ty) in bound.into_iter().rev() {
                        body = lam(binder, id, ty, &body);
                    }
                    minors.push(body);
                }
                let mut levels = vec![eq_level(&result)];
                levels.extend(&shape.levels);
                let mut arguments = shape.params.clone();
                arguments.push(motive);
                arguments.push(self.term(ctx, scrutinee)?);
                arguments.extend(minors);
                app(cu(&shape.cases_on, &levels), arguments)
            }
            SemanticTerm::Eq { left, right } => {
                let ty = self.infer(ctx, left).or_else(|_| self.infer(ctx, right))?;
                app(
                    cu("Eq", &[eq_level(&ty)]),
                    [
                        self.ty(ctx, &ty)?,
                        self.term(ctx, left)?,
                        self.term(ctx, right)?,
                    ],
                )
            }
            SemanticTerm::Le { left, right } | SemanticTerm::Lt { left, right } => {
                let ty = self.infer(ctx, left).or_else(|_| self.infer(ctx, right))?;
                let (relation, class) = if matches!(term, SemanticTerm::Le { .. }) {
                    ("LE.le", "LE")
                } else {
                    ("LT.lt", "LT")
                };
                app(
                    cu(relation, &[0]),
                    [
                        self.ty(ctx, &ty)?,
                        c(&number_instance(class, &ty)?),
                        self.term(ctx, left)?,
                        self.term(ctx, right)?,
                    ],
                )
            }
            SemanticTerm::Add { left, right } => {
                let ty = self.infer(ctx, term)?;
                let t = self.ty(ctx, &ty)?;
                app(
                    cu("HAdd.hAdd", &[0, 0, 0]),
                    [
                        t.clone(),
                        t.clone(),
                        t.clone(),
                        app(cu("instHAdd", &[0]), [t, c(&number_instance("Add", &ty)?)]),
                        self.term(ctx, left)?,
                        self.term(ctx, right)?,
                    ],
                )
            }
            SemanticTerm::Beq { left, right }
            | SemanticTerm::Ble { left, right }
            | SemanticTerm::Blt { left, right } => {
                let function = match term {
                    SemanticTerm::Beq { .. } => "Nat.beq",
                    SemanticTerm::Ble { .. } => "Nat.ble",
                    _ => "Nat.blt",
                };
                app(c(function), [self.term(ctx, left)?, self.term(ctx, right)?])
            }
            SemanticTerm::And { left, right } => app(
                c("Bool.and"),
                [self.term(ctx, left)?, self.term(ctx, right)?],
            ),
            SemanticTerm::Or { left, right } => app(
                c("Bool.or"),
                [self.term(ctx, left)?, self.term(ctx, right)?],
            ),
            SemanticTerm::Not { value } => app(c("Bool.not"), [self.term(ctx, value)?]),
            SemanticTerm::PropAnd { left, right } => {
                app(c("And"), [self.term(ctx, left)?, self.term(ctx, right)?])
            }
            SemanticTerm::Iff { left, right } => {
                app(c("Iff"), [self.term(ctx, left)?, self.term(ctx, right)?])
            }
            SemanticTerm::Implies {
                premise,
                conclusion,
            } => arrow(self.term(ctx, premise)?, self.term(ctx, conclusion)?),
            SemanticTerm::Forall { binder, body } => {
                let ty = absolute(&binder.r#type, &here);
                let id = self.fresh();
                let mut inner = ctx.clone();
                inner.push(&binder.name, Expr::FVar(id), Some(ty.clone()));
                pi(
                    &binder.name,
                    id,
                    self.ty(ctx, &ty)?,
                    &self.term(&inner, body)?,
                )
            }
            SemanticTerm::Let {
                binder,
                value,
                body,
            } => {
                let ty = absolute(&binder.r#type, &here);
                let id = self.fresh();
                let mut inner = ctx.clone();
                inner.push(&binder.name, Expr::FVar(id), Some(ty.clone()));
                let_in(
                    &binder.name,
                    id,
                    self.ty(ctx, &ty)?,
                    self.term(ctx, value)?,
                    &self.term(&inner, body)?,
                )
            }
            SemanticTerm::Pair { left, right } => {
                let (l, r) = (self.infer(ctx, left)?, self.infer(ctx, right)?);
                app(
                    cu("Prod.mk", &[0, 0]),
                    [
                        self.ty(ctx, &l)?,
                        self.ty(ctx, &r)?,
                        self.term(ctx, left)?,
                        self.term(ctx, right)?,
                    ],
                )
            }
            SemanticTerm::First { value } | SemanticTerm::Second { value } => {
                let SemanticType::Product { left, right } = self.infer(ctx, value)? else {
                    return Err("a projection from a non-pair".to_owned());
                };
                let function = if matches!(term, SemanticTerm::First { .. }) {
                    "Prod.fst"
                } else {
                    "Prod.snd"
                };
                app(
                    cu(function, &[0, 0]),
                    [
                        self.ty(ctx, &left)?,
                        self.ty(ctx, &right)?,
                        self.term(ctx, value)?,
                    ],
                )
            }
            SemanticTerm::Lambda {
                parameters, body, ..
            } => {
                let mut inner = ctx.clone();
                let mut bound = Vec::new();
                for parameter in parameters {
                    let ty = absolute(&parameter.r#type, &here);
                    let id = self.fresh();
                    inner.push(&parameter.name, Expr::FVar(id), Some(ty.clone()));
                    bound.push((parameter.name.clone(), id, self.ty(ctx, &ty)?));
                }
                let mut out = self.term(&inner, body)?;
                for (name, id, ty) in bound.into_iter().rev() {
                    out = lam(&name, id, ty, &out);
                }
                out
            }
            SemanticTerm::Apply {
                function,
                arguments,
            } => {
                let mut out = Vec::new();
                for argument in arguments {
                    out.push(self.term(ctx, argument)?);
                }
                app(self.term(ctx, function)?, out)
            }
            // A byte literal, as certificate A writes it: `ByteArray.mk [b, …].toArray`.
            SemanticTerm::Bytes { hex } => {
                if hex.len() % 2 != 0 {
                    return Err(format!("byte literal `{hex}` has odd length"));
                }
                let mut list = app(cu("List.nil", &[0]), [c("UInt8")]);
                for at in (0..hex.len()).step_by(2).rev() {
                    let value = u8::from_str_radix(hex.get(at..at + 2).unwrap_or_default(), 16)
                        .map_err(|_| format!("byte literal `{hex}` is not hexadecimal"))?;
                    let byte = self.term(
                        ctx,
                        &SemanticTerm::Integer {
                            representation: SemanticInteger::UInt8,
                            value: value.to_string(),
                        },
                    )?;
                    list = app(cu("List.cons", &[0]), [c("UInt8"), byte, list]);
                }
                app(
                    c("ByteArray.mk"),
                    [app(cu("List.toArray", &[0]), [c("UInt8"), list])],
                )
            }
            other @ (SemanticTerm::InstanceValue { .. } | SemanticTerm::CheckedApply { .. }) => {
                return Err(format!("term `{}`", kind_of_term(other)))
            }
        })
    }

    /// A runtime primitive as Lean's elaborator applies it: the function the
    /// Lean backend spells it with, its implicit type arguments read off the
    /// operands, and its instance from the table of the pinned toolchain.
    #[allow(clippy::too_many_lines)]
    fn primitive(
        &self,
        ctx: &Ctx,
        operation: SemanticPrimitive,
        arguments: &[SemanticTerm],
    ) -> R<Expr> {
        let function = match crate::backend::semantic::spelling(operation) {
            crate::backend::semantic::Spelling::Array(name) => {
                return Err(format!("array function `{name}`"))
            }
            crate::backend::semantic::Spelling::Runtime { name, int64: _ } => {
                format!("LexLeanRuntime.{name}")
            }
            crate::backend::semantic::Spelling::Collection(name) => {
                format!("LexLeanCollections.{name}")
            }
            crate::backend::semantic::Spelling::Widening(name) => {
                return Err(format!("widening `{name}`"))
            }
        };
        let operand = |index: usize| -> R<SemanticType> {
            let argument = arguments
                .get(index)
                .ok_or_else(|| format!("primitive `{operation:?}` without operand {index}"))?;
            self.infer(ctx, argument)
        };
        let entries = |ty: SemanticType| -> R<(SemanticType, SemanticType)> {
            match ty {
                SemanticType::Map { key, value } => Ok((*key, *value)),
                other => Err(format!("a map operand of type `{}`", kind_of_type(&other))),
            }
        };
        let element = |ty: SemanticType| -> R<SemanticType> {
            match ty {
                SemanticType::Set { element } | SemanticType::List { element } => Ok(*element),
                other => Err(format!("a set operand of type `{}`", kind_of_type(&other))),
            }
        };
        let key = |ty: &SemanticType| -> R<Expr> { self.key(ctx, ty) };
        // The implicit type arguments and the instance of each primitive.
        let (types, instance): (Vec<SemanticType>, Option<Expr>) = match operation {
            SemanticPrimitive::Subtract => {
                let ty = operand(0)?;
                let instance = c(&number_instance("Sub", &ty)?);
                (vec![ty], Some(instance))
            }
            SemanticPrimitive::Multiply => {
                let ty = operand(0)?;
                let instance = c(&number_instance("Mul", &ty)?);
                (vec![ty], Some(instance))
            }
            SemanticPrimitive::Negate => {
                let ty = operand(0)?;
                let instance = c(&number_instance("Neg", &ty)?);
                (vec![ty], Some(instance))
            }
            // Byte arrays compare by `ByteArray.instBEq`; every other type by
            // `BEq` from decidable equality, as Lean's `instBEqOfDecidableEq`.
            SemanticPrimitive::Equal if operand(0)? == SemanticType::Bytes => {
                (vec![SemanticType::Bytes], Some(c("ByteArray.instBEq")))
            }
            SemanticPrimitive::Equal => {
                let ty = operand(0)?;
                let instance = app(
                    cu("instBEqOfDecidableEq", &[0]),
                    [self.ty(ctx, &ty)?, self.decidable_eq(ctx, &ty)?],
                );
                (vec![ty], Some(instance))
            }
            SemanticPrimitive::Index => {
                let ty = operand(0)?;
                let (item, instance) = match &ty {
                    SemanticType::Bytes => (
                        SemanticType::UInt8,
                        c("LexLeanRuntime.instIndexableByteArrayUInt8"),
                    ),
                    SemanticType::List { element } => (
                        (**element).clone(),
                        app(
                            c("LexLeanRuntime.instIndexableList"),
                            [self.ty(ctx, element)?],
                        ),
                    ),
                    other => return Err(format!("`Index` of `{}`", kind_of_type(other))),
                };
                (vec![ty, item], Some(instance))
            }
            SemanticPrimitive::Slice => {
                let ty = operand(0)?;
                let instance = match &ty {
                    SemanticType::Bytes => c("LexLeanRuntime.instSliceableByteArray"),
                    SemanticType::List { element } => app(
                        c("LexLeanRuntime.instSliceableList"),
                        [self.ty(ctx, element)?],
                    ),
                    other => return Err(format!("`Slice` of `{}`", kind_of_type(other))),
                };
                (vec![ty], Some(instance))
            }
            SemanticPrimitive::CompareBytes => (vec![], None),
            // A fixed-width operation takes the width's `Fixed` instance.
            SemanticPrimitive::CheckedAdd
            | SemanticPrimitive::CheckedSubtract
            | SemanticPrimitive::CheckedMultiply
            | SemanticPrimitive::CheckedNegate
            | SemanticPrimitive::BitAnd
            | SemanticPrimitive::BitOr
            | SemanticPrimitive::BitXor
            | SemanticPrimitive::BitNot
            | SemanticPrimitive::ShiftLeft
            | SemanticPrimitive::ShiftRight => {
                let ty = operand(0)?;
                let name = match integer_type(&ty) {
                    Some((name, _)) if name != "Int" => name,
                    _ => return Err(format!("`{operation:?}` of `{}`", kind_of_type(&ty))),
                };
                (
                    vec![ty],
                    Some(c(&format!("LexLeanRuntime.instFixed{name}"))),
                )
            }
            SemanticPrimitive::Quotient | SemanticPrimitive::Remainder => {
                let ty = operand(0)?;
                let instance = c(&number_instance("Quotient", &ty)?);
                (vec![ty], Some(instance))
            }
            SemanticPrimitive::Length | SemanticPrimitive::Append => {
                let ty = operand(0)?;
                let length = operation == SemanticPrimitive::Length;
                let instance = match (&ty, length) {
                    (SemanticType::List { element }, true) => app(
                        c("LexLeanRuntime.instLengthableList"),
                        [self.ty(ctx, element)?],
                    ),
                    (SemanticType::List { element }, false) => app(
                        c("LexLeanRuntime.instAppendableList"),
                        [self.ty(ctx, element)?],
                    ),
                    (SemanticType::String, true) => c("LexLeanRuntime.instLengthableString"),
                    (SemanticType::Bytes, false) => c("LexLeanRuntime.instAppendableByteArray"),
                    (SemanticType::Bytes, true) => c("LexLeanRuntime.instLengthableByteArray"),
                    (other, _) => {
                        return Err(format!("`{operation:?}` of `{}`", kind_of_type(other)))
                    }
                };
                (vec![ty], Some(instance))
            }
            SemanticPrimitive::MapInsert
            | SemanticPrimitive::MapRemove
            | SemanticPrimitive::MapLookup
            | SemanticPrimitive::MapContains => {
                let (k, v) = entries(operand(0)?)?;
                let instance = key(&k)?;
                (vec![k, v], Some(instance))
            }
            SemanticPrimitive::MapSize
            | SemanticPrimitive::MapKeys
            | SemanticPrimitive::MapValues
            | SemanticPrimitive::MapEntries => {
                let (k, v) = entries(operand(0)?)?;
                (vec![k, v], None)
            }
            SemanticPrimitive::MapFold => {
                let (k, v) = entries(operand(2)?)?;
                (vec![k, v, operand(1)?], None)
            }
            SemanticPrimitive::SetInsert
            | SemanticPrimitive::SetRemove
            | SemanticPrimitive::SetContains
            | SemanticPrimitive::SetUnion
            | SemanticPrimitive::SetIntersection
            | SemanticPrimitive::SetDifference => {
                let k = element(operand(0)?)?;
                let instance = key(&k)?;
                (vec![k], Some(instance))
            }
            SemanticPrimitive::SetSize | SemanticPrimitive::SetElements => {
                (vec![element(operand(0)?)?], None)
            }
            SemanticPrimitive::SetFold | SemanticPrimitive::ListFold => {
                (vec![element(operand(2)?)?, operand(1)?], None)
            }
            SemanticPrimitive::Iterate | SemanticPrimitive::IterateUntil => {
                (vec![operand(2)?], None)
            }
            SemanticPrimitive::GraphSuccessors
            | SemanticPrimitive::GraphReachable
            | SemanticPrimitive::GraphTopological => {
                let (k, _) = entries(operand(0)?)?;
                let instance = key(&k)?;
                (vec![k], Some(instance))
            }
            SemanticPrimitive::LessThan => {
                let k = operand(0)?;
                let instance = key(&k)?;
                (vec![k], Some(instance))
            }
            other @ (SemanticPrimitive::CheckedConvert
            | SemanticPrimitive::CheckedQuotient
            | SemanticPrimitive::Utf8Encode
            | SemanticPrimitive::Utf8Decode
            | SemanticPrimitive::SplitExact
            | SemanticPrimitive::Join
            | SemanticPrimitive::ParseDecimal
            | SemanticPrimitive::FormatDecimal
            | SemanticPrimitive::ToInt
            | SemanticPrimitive::ArrayFromList
            | SemanticPrimitive::ArrayToList
            | SemanticPrimitive::ArraySet
            | SemanticPrimitive::ToNat) => return Err(format!("primitive `{other:?}`")),
        };
        let mut out = Vec::new();
        for ty in &types {
            out.push(self.ty(ctx, ty)?);
        }
        out.extend(instance);
        for argument in arguments {
            out.push(self.term(ctx, argument)?);
        }
        Ok(app(c(&function), out))
    }

    /// A `LexLeanCollections.Key` instance of a type: a scalar's own, and a
    /// pair's from its components'.
    fn key(&self, ctx: &Ctx, ty: &SemanticType) -> R<Expr> {
        match ty {
            SemanticType::Bool => Ok(c("LexLeanCollections.instKeyBool")),
            SemanticType::Product { left, right } => Ok(app(
                c("LexLeanCollections.instKeyProd"),
                [
                    self.ty(ctx, left)?,
                    self.ty(ctx, right)?,
                    self.key(ctx, left)?,
                    self.key(ctx, right)?,
                ],
            )),
            other => Ok(c(&number_instance("Key", other)?)),
        }
    }

    /// A `DecidableEq` instance of a type.
    fn decidable_eq(&self, ctx: &Ctx, ty: &SemanticType) -> R<Expr> {
        Ok(match ty {
            SemanticType::Array { .. } => return Err("decidable equality of `array`".to_owned()),
            SemanticType::Nat => c("instDecidableEqNat"),
            other if integer_type(other).is_some() => c(&number_instance("DecidableEq", other)?),
            SemanticType::Option { value } => app(
                cu("Option.instDecidableEq", &[0]),
                [self.ty(ctx, value)?, self.decidable_eq(ctx, value)?],
            ),
            SemanticType::Map { key, value } => self.decidable_eq(
                ctx,
                &SemanticType::List {
                    element: Box::new(SemanticType::Product {
                        left: key.clone(),
                        right: value.clone(),
                    }),
                },
            )?,
            SemanticType::Set { element } => self.decidable_eq(
                ctx,
                &SemanticType::List {
                    element: element.clone(),
                },
            )?,
            SemanticType::Bool => c("instDecidableEqBool"),
            SemanticType::String => c("instDecidableEqString"),
            SemanticType::ContractViolation | SemanticType::ReasoningFailure => app(
                cu("instDecidableEqProd", &[0, 0]),
                [
                    c("Bool"),
                    c("Bool"),
                    c("instDecidableEqBool"),
                    c("instDecidableEqBool"),
                ],
            ),
            SemanticType::List { element } => app(
                cu("instDecidableEqList", &[0]),
                [self.ty(ctx, element)?, self.decidable_eq(ctx, element)?],
            ),
            SemanticType::Product { left, right } => app(
                cu("instDecidableEqProd", &[0, 0]),
                [
                    self.ty(ctx, left)?,
                    self.ty(ctx, right)?,
                    self.decidable_eq(ctx, left)?,
                    self.decidable_eq(ctx, right)?,
                ],
            ),
            other @ (SemanticType::Type
            | SemanticType::Parameter { .. }
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
            | SemanticType::Bytes
            | SemanticType::Ordering
            | SemanticType::Result { .. }
            | SemanticType::Named { .. }
            | SemanticType::Function { .. }) => {
                return Err(format!("decidable equality of `{}`", kind_of_type(other)))
            }
        })
    }

    /// A `Decidable` instance of a proposition, one the kernel evaluates.
    pub fn decidable(&self, ctx: &Ctx, term: &SemanticTerm) -> R<Expr> {
        Ok(match term {
            SemanticTerm::Eq { left, right } => {
                let ty = self.infer(ctx, left).or_else(|_| self.infer(ctx, right))?;
                app(
                    self.decidable_eq(ctx, &ty)?,
                    [self.term(ctx, left)?, self.term(ctx, right)?],
                )
            }
            SemanticTerm::Le { left, right } | SemanticTerm::Lt { left, right } => {
                let ty = self.infer(ctx, left).or_else(|_| self.infer(ctx, right))?;
                let class = if matches!(term, SemanticTerm::Le { .. }) {
                    "decLe"
                } else {
                    "decLt"
                };
                app(
                    c(&number_instance(class, &ty)?),
                    [self.term(ctx, left)?, self.term(ctx, right)?],
                )
            }
            SemanticTerm::PropAnd { left, right }
            | SemanticTerm::Iff { left, right }
            | SemanticTerm::Implies {
                premise: left,
                conclusion: right,
            } => {
                let function = match term {
                    SemanticTerm::PropAnd { .. } => "instDecidableAnd",
                    SemanticTerm::Iff { .. } => "instDecidableIff",
                    _ => "instDecidableForall",
                };
                app(
                    c(function),
                    [
                        self.term(ctx, left)?,
                        self.term(ctx, right)?,
                        self.decidable(ctx, left)?,
                        self.decidable(ctx, right)?,
                    ],
                )
            }
            SemanticTerm::Let {
                binder,
                value,
                body,
            } => {
                let mut inner = ctx.clone();
                let ty = absolute(&binder.r#type, self.module_name(ctx));
                inner.push(&binder.name, self.term(ctx, value)?, Some(ty));
                self.decidable(&inner, body)?
            }
            // A propositional definition is reducible, so an instance of its
            // body is an instance of the application.
            SemanticTerm::Call {
                function,
                type_arguments,
                arguments,
            } => {
                let (module, declaration, parameters, result) =
                    self.signature(ctx, function, type_arguments)?;
                let SemanticDeclaration::Definition {
                    type_parameters,
                    body,
                    recursive_argument: None,
                    ..
                } = declaration
                else {
                    return Err(format!("decidability of recursive `{}`", function.name));
                };
                if result != SemanticType::Prop {
                    return Err(format!(
                        "decidability of `{}`, which is not a proposition",
                        function.name
                    ));
                }
                let mut inner = Ctx::new(module);
                for (name, argument) in type_parameters.iter().zip(type_arguments) {
                    inner.types.push((
                        name.clone(),
                        self.ty(ctx, &absolute(argument, self.module_name(ctx)))?,
                    ));
                }
                for (parameter, argument) in parameters.iter().zip(arguments) {
                    inner.push(
                        &parameter.name,
                        self.term(ctx, argument)?,
                        Some(parameter.r#type.clone()),
                    );
                }
                self.decidable(&inner, body)?
            }
            other @ (SemanticTerm::Var { .. }
            | SemanticTerm::Nat { .. }
            | SemanticTerm::Integer { .. }
            | SemanticTerm::String { .. }
            | SemanticTerm::Bytes { .. }
            | SemanticTerm::Primitive { .. }
            | SemanticTerm::Bool { .. }
            | SemanticTerm::Unit
            | SemanticTerm::Nil { .. }
            | SemanticTerm::Cons { .. }
            | SemanticTerm::Record { .. }
            | SemanticTerm::Constructor { .. }
            | SemanticTerm::InstanceValue { .. }
            | SemanticTerm::Project { .. }
            | SemanticTerm::If { .. }
            | SemanticTerm::Match { .. }
            | SemanticTerm::Add { .. }
            | SemanticTerm::Beq { .. }
            | SemanticTerm::Ble { .. }
            | SemanticTerm::Blt { .. }
            | SemanticTerm::And { .. }
            | SemanticTerm::Or { .. }
            | SemanticTerm::Not { .. }
            | SemanticTerm::Forall { .. }
            | SemanticTerm::Pair { .. }
            | SemanticTerm::First { .. }
            | SemanticTerm::Second { .. }
            | SemanticTerm::Lambda { .. }
            | SemanticTerm::Apply { .. }
            | SemanticTerm::FunctionRef { .. }
            | SemanticTerm::MapLiteral { .. }
            | SemanticTerm::SetLiteral { .. }
            | SemanticTerm::GraphLiteral { .. }
            | SemanticTerm::CheckedApply { .. }) => {
                return Err(format!("decidability of `{}`", kind_of_term(other)))
            }
        })
    }
}

/// The constants a statement's own lowering may name, besides the project's
/// declarations: the export roots the library must hold.
pub const VOCABULARY: &[&str] = &[
    "Nat",
    "Bool",
    "Bool.true",
    "Bool.false",
    "Unit",
    "Unit.unit",
    "Int",
    "String",
    "ByteArray",
    "Ordering",
    "Option",
    "Except",
    "Except.ok",
    "Except.error",
    "List",
    "List.nil",
    "List.cons",
    "Prod",
    "Prod.mk",
    "Prod.fst",
    "Prod.snd",
    "Option.none",
    "Option.some",
    "Nat.zero",
    "Nat.succ",
    "OfNat.ofNat",
    "instOfNatNat",
    "Eq",
    "ite",
    "instDecidableEqBool",
    "instDecidableEqNat",
    "instDecidableEqString",
    "instDecidableEqList",
    "instDecidableEqProd",
    "LE.le",
    "LT.lt",
    "instLENat",
    "instLTNat",
    "HAdd.hAdd",
    "instHAdd",
    "instAddNat",
    "Nat.beq",
    "Nat.ble",
    "Nat.blt",
    "Bool.and",
    "Bool.or",
    "Bool.not",
    "And",
    "Iff",
    "Nat.decLe",
    "Nat.decLt",
    "instDecidableAnd",
    "instDecidableIff",
    "instDecidableForall",
    "Nat.casesOn",
    "Nat.rec",
    "Bool.casesOn",
    "Bool.rec",
    "List.casesOn",
    "List.rec",
    "Option.casesOn",
    "Option.rec",
    "Prod.casesOn",
    "Prod.rec",
    "Neg.neg",
    "instOfNat",
    "Int.instNegInt",
    "Int.instAdd",
    "Int.instSub",
    "Int.instLEInt",
    "Int.instLTInt",
    "Int.instDecidableEq",
    "Int.decLe",
    "Int.decLt",
    "instSubNat",
    "Option.instDecidableEq",
    "Int8.instOfNat",
    "instAddInt8",
    "instSubInt8",
    "instLEInt8",
    "instLTInt8",
    "instDecidableEqInt8",
    "Int8.decLe",
    "Int8.decLt",
    "Int8.instNeg",
    "Int16.instOfNat",
    "instAddInt16",
    "instSubInt16",
    "instLEInt16",
    "instLTInt16",
    "instDecidableEqInt16",
    "Int16.decLe",
    "Int16.decLt",
    "Int16.instNeg",
    "Int32.instOfNat",
    "instAddInt32",
    "instSubInt32",
    "instLEInt32",
    "instLTInt32",
    "instDecidableEqInt32",
    "Int32.decLe",
    "Int32.decLt",
    "Int32.instNeg",
    "Int64.instOfNat",
    "instAddInt64",
    "instSubInt64",
    "instLEInt64",
    "instLTInt64",
    "instDecidableEqInt64",
    "Int64.decLe",
    "Int64.decLt",
    "Int64.instNeg",
    "UInt8.instOfNat",
    "instAddUInt8",
    "instSubUInt8",
    "instLEUInt8",
    "instLTUInt8",
    "instDecidableEqUInt8",
    "UInt8.decLe",
    "UInt8.decLt",
    "UInt16.instOfNat",
    "instAddUInt16",
    "instSubUInt16",
    "instLEUInt16",
    "instLTUInt16",
    "instDecidableEqUInt16",
    "UInt16.decLe",
    "UInt16.decLt",
    "UInt32.instOfNat",
    "instAddUInt32",
    "instSubUInt32",
    "instLEUInt32",
    "instLTUInt32",
    "instDecidableEqUInt32",
    "UInt32.decLe",
    "UInt32.decLt",
    "UInt64.instOfNat",
    "instAddUInt64",
    "instSubUInt64",
    "instLEUInt64",
    "instLTUInt64",
    "instDecidableEqUInt64",
    "UInt64.decLe",
    "UInt64.decLt",
    "Except.casesOn",
    "Except.rec",
    "instMulNat",
    "Int.instMul",
    "instBEqOfDecidableEq",
    "ByteArray.instBEq",
    "ByteArray.mk",
    "List.toArray",
    "UInt8",
    "instMulInt8",
    "instMulInt16",
    "instMulInt32",
    "instMulInt64",
    "instMulUInt8",
    "instMulUInt16",
    "instMulUInt32",
    "instMulUInt64",
    "instNegUInt8",
    "instNegUInt16",
    "instNegUInt32",
    "instNegUInt64",
];

/// The runtime constants a statement's lowering may name, by the runtime
/// module that declares them: an export root only when the project imports
/// that module.
pub const RUNTIME_VOCABULARY: &[(&str, &[&str])] = &[
    (
        "LexLeanRuntime.Portable",
        &[
            "LexLeanRuntime.subtract",
            "LexLeanRuntime.quotient",
            "LexLeanRuntime.remainder",
            "LexLeanRuntime.length",
            "LexLeanRuntime.append",
            "LexLeanRuntime.instQuotientNat",
            "LexLeanRuntime.instQuotientInt",
            "LexLeanRuntime.instLengthableList",
            "LexLeanRuntime.instLengthableString",
            "LexLeanRuntime.instLengthableByteArray",
            "LexLeanRuntime.instAppendableList",
            "LexLeanRuntime.multiply",
            "LexLeanRuntime.negate",
            "LexLeanRuntime.equal",
            "LexLeanRuntime.index",
            "LexLeanRuntime.slice",
            "LexLeanRuntime.compareBytes",
            "LexLeanRuntime.checkedAdd",
            "LexLeanRuntime.checkedSubtract",
            "LexLeanRuntime.checkedMultiply",
            "LexLeanRuntime.checkedNegate",
            "LexLeanRuntime.bitAnd",
            "LexLeanRuntime.bitOr",
            "LexLeanRuntime.bitXor",
            "LexLeanRuntime.bitNot",
            "LexLeanRuntime.shiftLeft",
            "LexLeanRuntime.shiftRight",
            "LexLeanRuntime.instAppendableByteArray",
            "LexLeanRuntime.instIndexableList",
            "LexLeanRuntime.instIndexableByteArrayUInt8",
            "LexLeanRuntime.instSliceableByteArray",
            "LexLeanRuntime.instSliceableList",
            "LexLeanRuntime.instFixedInt8",
            "LexLeanRuntime.instFixedInt16",
            "LexLeanRuntime.instFixedInt32",
            "LexLeanRuntime.instFixedInt64",
            "LexLeanRuntime.instFixedUInt8",
            "LexLeanRuntime.instFixedUInt16",
            "LexLeanRuntime.instFixedUInt32",
            "LexLeanRuntime.instFixedUInt64",
        ],
    ),
    (
        "LexLeanRuntime.Collections",
        &[
            "LexLeanCollections.mapInsert",
            "LexLeanCollections.mapRemove",
            "LexLeanCollections.mapLookup",
            "LexLeanCollections.mapContains",
            "LexLeanCollections.mapSize",
            "LexLeanCollections.mapKeys",
            "LexLeanCollections.mapValues",
            "LexLeanCollections.mapEntries",
            "LexLeanCollections.mapFold",
            "LexLeanCollections.setInsert",
            "LexLeanCollections.setRemove",
            "LexLeanCollections.setContains",
            "LexLeanCollections.setUnion",
            "LexLeanCollections.setIntersection",
            "LexLeanCollections.setDifference",
            "LexLeanCollections.setSize",
            "LexLeanCollections.setElements",
            "LexLeanCollections.setFold",
            "LexLeanCollections.listFold",
            "LexLeanCollections.iterate",
            "LexLeanCollections.iterateUntil",
            "LexLeanCollections.graphSuccessors",
            "LexLeanCollections.graphReachable",
            "LexLeanCollections.graphTopological",
            "LexLeanCollections.lessThan",
            "LexLeanCollections.instKeyNat",
            "LexLeanCollections.instKeyString",
            "LexLeanCollections.instKeyInt",
            "LexLeanCollections.instKeyInt8",
            "LexLeanCollections.instKeyInt16",
            "LexLeanCollections.instKeyInt32",
            "LexLeanCollections.instKeyInt64",
            "LexLeanCollections.instKeyUInt8",
            "LexLeanCollections.instKeyUInt16",
            "LexLeanCollections.instKeyUInt32",
            "LexLeanCollections.instKeyUInt64",
            "LexLeanCollections.instKeyBool",
            "LexLeanCollections.instKeyProd",
        ],
    ),
];

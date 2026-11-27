//! Certificate B as kernel-term data (SPEC.md §17.17, `SP-16`).
//!
//! Certificate B of one rendering of a root states that the rendered crate
//! simulates the lowered program: the program and the crate are two
//! definitions, `program` and `krate`, whose terms are the target calculus
//! program and the closed Rust AST; `flags` records how the function types the
//! program closes over or applies are called; one theorem `fun<n>` per
//! function derives `FunOK program krate flags n` from the library's
//! correspondence `Corr`, one rule application per construct; `crate_ok`
//! assembles them into `CrateOK`, and `root` is the library's `simulate`
//! applied to it. Here the module is built as a native core module: a row per
//! declaration, each a kernel term whose every argument is written, the rule
//! types read from the native preservation library and instantiated one
//! binder at a time ([`super::super::kernel`]).
//!
//! What the text generator spelled for Lean's elaborator, this generator
//! writes out: the implicit program, crate, and flags of every rule, the
//! context, the judgment and the arguments the aligner holds from the
//! constructs it relates, the witnesses of `FunOK`, and the case analysis on
//! the function index. An `rfl` is `Eq.refl` at the equation the rule states,
//! which the kernel decides by evaluation. The statements are the text
//! generator's, so the kernel term each theorem states is the one the text
//! elaborated to.

// The shape tests of the aligner and of the proof builders end in a refusal
// (`Err`), never in a default realization: a construct they do not match is
// reported, not certified. The lint that denies a catch-all in this module's
// parent guards the lowering of constructs, which these matches are not, and
// the exhaustive match on every construct that matters is the block
// judgment, which `audit-production` holds to name each one.
#![allow(
    clippy::wildcard_enum_match_arm,
    clippy::match_wildcard_for_single_variants
)]

mod align;
pub mod loopform;
mod syntax;

use std::collections::BTreeMap;

pub use align::RULES;

use super::data_a::{RowKind, Writer};
use super::{Gen, Source};
use crate::calculus::rust::ast::{Crate, ItemDef};
use crate::calculus::{Function, Program};
use crate::ir::core::{CoreBinderInfo, CoreLevel, CoreModule, CoreNode};
use crate::production::kernel::Term as K;
use crate::production::lower::{Layout, LIMIT};
use crate::production::preserve;
use align::Aligner;

/// The library module certificate B imports, which imports the rest.
pub const SOUNDNESS_MODULE: &str = "LexLeanPreservation.RustSound";

const RUST: &str = "LexLeanPreservation.Rust";

/// Rows by name: each with its type and its value.
type Named = Vec<(String, K, K)>;

/// The right side of a step of `findFunction`, from the item's fields, the
/// rest of the items, and the identifier sought.
type StepRight<'r, 'g, 'a> = dyn Fn(&mut Aligner<'g, 'a>, &[K], K, K) -> K + 'r;

/// The rows a certificate B module declares, in the order it declares them.
struct Rows {
    functions: Vec<(String, K, K)>,
}

impl<'g, 'a> Aligner<'g, 'a> {
    fn rust_const(&mut self, name: &str) -> K {
        self.w.b.konst(&format!("{RUST}.{name}"))
    }

    fn row(&mut self, name: &str) -> K {
        self.w.own(name)
    }

    fn level_zero() -> CoreLevel {
        CoreLevel::Zero
    }

    /// `Eq.refl` at the equation `equation` states, whose sides the kernel
    /// compares by evaluation.
    fn refl_of(&mut self, equation: K) -> Result<K, String> {
        let (head, parts) = self.w.b.spine(equation);
        let levels = match self.w.b.node(head) {
            CoreNode::Const { n, u } if n == "Eq" && parts.len() == 3 => u.clone(),
            _ => {
                return Err(format!(
                    "a reflexivity proof at {}, which is not an equation",
                    self.w.b.show(equation)
                ));
            }
        };
        let refl = self.w.b.konst_at("Eq.refl", levels);
        Ok(self.w.b.apps(refl, &[parts[0], parts[1]]))
    }

    /// The two disjuncts `FunOK` states of function `index`, as the
    /// applications of `FunOK0` (the function the crate states directly) and
    /// `FunLoop` (the loop form of a recursive body) they are.
    fn fun_ok_halves(&mut self, index: u64) -> Result<(K, K), String> {
        let (module, declaration) = preserve::native_declaration(&format!("{RUST}.FunOK"))?;
        let value = declaration
            .value
            .ok_or("FunOK is declared without a value")?;
        let mut term = self.w.b.import(module, value);
        let f = self.w.nat_u(index);
        for argument in [self.p, self.c, self.a, f] {
            let CoreNode::Lambda { v, .. } = self.w.b.node(term).clone() else {
                return Err("FunOK is not a function of its four arguments".to_owned());
            };
            term = self.w.b.instantiate_beta(v, argument);
        }
        self.disjuncts(term)
    }

    /// An application of a library definition, unfolded one step over its
    /// arguments.
    fn unfold(&mut self, application: K) -> Result<K, String> {
        let (head, arguments) = self.w.b.spine(application);
        let CoreNode::Const { n, .. } = self.w.b.node(head).clone() else {
            return Err("an unfolded application is not a constant application".to_owned());
        };
        let (module, declaration) = preserve::native_declaration(&n)?;
        let value = declaration
            .value
            .ok_or_else(|| format!("`{n}` is declared without a value"))?;
        let mut term = self.w.b.import(module, value);
        for argument in arguments {
            let CoreNode::Lambda { v, .. } = self.w.b.node(term).clone() else {
                return Err(format!("`{n}` takes fewer arguments than it is applied to"));
            };
            term = self.w.b.instantiate_beta(v, argument);
        }
        Ok(term)
    }

    /// `Exists.intro` of a witness and the proof of the predicate at it.
    fn exists_intro(
        &mut self,
        statement: K,
        witness: K,
        proof: impl FnOnce(&mut Self, K) -> Result<K, String>,
    ) -> Result<K, String> {
        let (head, parts) = self.w.b.spine(statement);
        let CoreNode::Const { n, u } = self.w.b.node(head).clone() else {
            return Err("an existential is not a constant application".to_owned());
        };
        if n != "Exists" || parts.len() != 2 {
            return Err(format!(
                "`{}` is not an existential",
                self.w.b.show(statement)
            ));
        }
        let CoreNode::Lambda { v, .. } = self.w.b.node(parts[1]).clone() else {
            return Err("the predicate of an existential is not a function".to_owned());
        };
        let at = self.w.b.instantiate_beta(v, witness);
        let inner = proof(self, at)?;
        let intro = self.w.b.konst_at("Exists.intro", u);
        Ok(self.w.b.apps(intro, &[parts[0], parts[1], witness, inner]))
    }

    /// The two halves of a conjunction.
    fn conjuncts(&self, statement: K) -> Result<(K, K), String> {
        let (head, parts) = self.w.b.spine(statement);
        match self.w.b.node(head) {
            CoreNode::Const { n, .. } if n == "And" && parts.len() == 2 => Ok((parts[0], parts[1])),
            _ => Err(format!(
                "`{}` is not a conjunction",
                self.w.b.show(statement)
            )),
        }
    }

    fn and_type(&mut self, left: K, right: K) -> K {
        let and = self.w.b.konst("And");
        self.w.b.apps(and, &[left, right])
    }

    fn and_intro(&mut self, left: K, right: K, proof_left: K, proof_right: K) -> K {
        let intro = self.w.b.konst("And.intro");
        self.w
            .b
            .apps(intro, &[left, right, proof_left, proof_right])
    }

    /// The proof of `FunOK program krate flags index` by a derivation, or by
    /// the emptiness of a parameter's type.
    fn function_proof(&mut self, index: u64) -> Result<K, String> {
        let function = self.function(index)?;
        let (parameters, body) = self
            .krate()
            .items
            .iter()
            .find_map(|item| match item {
                ItemDef::Function {
                    name: crate::calculus::rust::ast::Ident::Function(found),
                    parameters,
                    body,
                    ..
                } if *found == index => Some((parameters, body)),
                _ => None,
            })
            .ok_or_else(|| format!("the crate does not render function {index}"))?;
        let (direct, looping) = self.fun_ok_halves(index)?;
        if matches!(body.tail, crate::calculus::rust::ast::Expr::Loop { .. }) {
            let recursive = loopform::recover(index, parameters, body)?;
            let statement = self.unfold(looping)?;
            let proof = self.loop_case(index, function, parameters, body, &recursive, statement)?;
            return Ok(self.or_inr(direct, looping, proof));
        }
        let statement = self.unfold(direct)?;
        let fn_term = self.w.function_term(function)?;
        let empty_parameter = function
            .types
            .iter()
            .position(|ty| self.uninh(&self.empty.clone(), ty));
        let proof = self.exists_intro(statement, fn_term, |this, at| {
            let (found, rest) = this.conjuncts(at)?;
            let refl = this.row(&format!("look{index}"));
            let (left, right) = this.disjuncts(rest)?;
            let choice = match empty_parameter {
                None => {
                    let proof = this.inhabited_case(index, function, parameters, body, left)?;
                    this.or_inl(left, right, proof)
                }
                Some(position) => {
                    let proof = this.empty_case(function, position, right)?;
                    this.or_inr(left, right, proof)
                }
            };
            Ok(this.and_intro(found, rest, refl, choice))
        })?;
        Ok(self.or_inl(direct, looping, proof))
    }

    /// The disjunct of a function rendered as a loop: the recursive body it is
    /// the loop form of, how each parameter is carried, and the derivation of
    /// the correspondence for that body.
    fn loop_case(
        &mut self,
        index: u64,
        function: &Function,
        parameters: &[(
            crate::calculus::rust::ast::Pat,
            crate::calculus::rust::ast::Type,
        )],
        body: &crate::calculus::rust::ast::Block,
        recursive: &loopform::Recursive,
        statement: K,
    ) -> Result<K, String> {
        let patterns: Vec<crate::calculus::rust::ast::Pat> = recursive
            .parameters
            .iter()
            .map(|(pattern, _)| pattern.clone())
            .collect();
        let empty = self.ctx_empty();
        let scope = self.param_ctx(&empty, &function.parameters, &function.types, &patterns)?;
        let fallible = self.fallible_fn(index);
        let derivation = self.bg(
            &scope,
            fallible,
            fallible,
            &function.body,
            &function.result,
            &recursive.body.lets,
            &recursive.body.tail,
        )?;
        let fn_term = self.w.function_term(function)?;
        let recursive_parameters = self.w.rs_parameters(&recursive.parameters);
        let result = self.krate_result(index)?;
        let lets = self.w.rs_lets(&recursive.body.lets);
        let tail = self.w.rs_expr(&recursive.body.tail);
        let gamma = scope.term();
        let positions = {
            let pos = self.rust_const("Loop.Pos");
            let items: Vec<K> = recursive
                .positions
                .iter()
                .map(|position| match position {
                    loopform::Position::Changed => self.rust_const("Loop.Pos.changed"),
                    loopform::Position::Kept(name) => {
                        let kept = self.rust_const("Loop.Pos.kept");
                        let name = self.w.rs_ident(name);
                        self.w.b.app(kept, name)
                    }
                    loopform::Position::Dropped => self.rust_const("Loop.Pos.dropped"),
                })
                .collect();
            self.w.b.list(pos, &items)
        };
        let state = self.w.rs_type(&recursive.state);
        let done = self.w.rs_type(&recursive.done);
        let holder = self.w.nat_u(recursive.holder);
        let renamed = {
            let nat = self.w.b.konst("Nat");
            let items: Vec<K> = recursive.renamed.iter().map(|k| self.w.nat_u(*k)).collect();
            self.w.b.list(nat, &items)
        };
        let looped_parameters = self.w.rs_parameters(parameters);
        let looped_body = self.w.rs_block(body);
        let witnesses = [
            fn_term,
            recursive_parameters,
            result,
            lets,
            tail,
            gamma,
            positions,
            state,
            done,
            holder,
            renamed,
            looped_parameters,
            looped_body,
        ];
        self.nested_exists(statement, &witnesses, &mut |this, innermost| {
            // Five equations that evaluation decides, then the derivation.
            let mut stated = Vec::new();
            let mut rest = innermost;
            for _ in 0..5 {
                let (head, tail) = this.conjuncts(rest)?;
                stated.push(head);
                rest = tail;
            }
            let mut proof = derivation.proof;
            let mut right = rest;
            for equation in stated.iter().rev() {
                let refl = this.refl_of(*equation)?;
                proof = this.and_intro(*equation, right, refl, proof);
                right = this.and_type(*equation, right);
            }
            Ok(proof)
        })
    }

    fn disjuncts(&self, statement: K) -> Result<(K, K), String> {
        let (head, parts) = self.w.b.spine(statement);
        match self.w.b.node(head) {
            CoreNode::Const { n, .. } if n == "Or" && parts.len() == 2 => Ok((parts[0], parts[1])),
            _ => Err(format!(
                "`{}` is not a disjunction",
                self.w.b.show(statement)
            )),
        }
    }

    fn or_inl(&mut self, left: K, right: K, proof: K) -> K {
        let intro = self.w.b.konst("Or.inl");
        self.w.b.apps(intro, &[left, right, proof])
    }

    fn or_inr(&mut self, left: K, right: K, proof: K) -> K {
        let intro = self.w.b.konst("Or.inr");
        self.w.b.apps(intro, &[left, right, proof])
    }

    /// The disjunct of a function whose parameters can all be given: its
    /// parameters, result, body, and context, and the derivation.
    fn inhabited_case(
        &mut self,
        index: u64,
        function: &Function,
        parameters: &[(
            crate::calculus::rust::ast::Pat,
            crate::calculus::rust::ast::Type,
        )],
        body: &crate::calculus::rust::ast::Block,
        statement: K,
    ) -> Result<K, String> {
        let patterns: Vec<crate::calculus::rust::ast::Pat> = parameters
            .iter()
            .map(|(pattern, _)| pattern.clone())
            .collect();
        let empty = self.ctx_empty();
        let scope = self.param_ctx(&empty, &function.parameters, &function.types, &patterns)?;
        let fallible = self.fallible_fn(index);
        let derivation = self.bg(
            &scope,
            fallible,
            fallible,
            &function.body,
            &function.result,
            &body.lets,
            &body.tail,
        )?;
        let params = self.w.rs_parameters(parameters);
        let result = self.krate_result(index)?;
        let lets = self.w.rs_lets(&body.lets);
        let tail = self.w.rs_expr(&body.tail);
        let gamma = scope.term();
        let witnesses = [params, result, lets, tail, gamma];
        self.nested_exists(statement, &witnesses, &mut |this, innermost| {
            let (found, rest) = this.conjuncts(innermost)?;
            let first = this.found_in_crate(index, found)?;
            let (ctx_eq, corr) = this.conjuncts(rest)?;
            let second = this.refl_of(ctx_eq)?;
            let last = this.and_intro(ctx_eq, corr, second, derivation.proof);
            Ok(this.and_intro(found, rest, first, last))
        })
    }

    fn krate_result(&mut self, index: u64) -> Result<K, String> {
        let result = self
            .krate()
            .items
            .iter()
            .find_map(|item| match item {
                ItemDef::Function {
                    name: crate::calculus::rust::ast::Ident::Function(found),
                    result,
                    ..
                } if *found == index => Some(result),
                _ => None,
            })
            .ok_or_else(|| format!("the crate does not render function {index}"))?;
        Ok(self.w.rs_type(result))
    }

    /// Nested existentials introduced by their witnesses, in order, and the
    /// proof of what is left.
    fn nested_exists(
        &mut self,
        statement: K,
        witnesses: &[K],
        finish: &mut dyn FnMut(&mut Self, K) -> Result<K, String>,
    ) -> Result<K, String> {
        match witnesses.split_first() {
            None => finish(self, statement),
            Some((witness, rest)) => self.exists_intro(statement, *witness, |this, at| {
                this.nested_exists(at, rest, finish)
            }),
        }
    }

    /// The disjunct of a function with a parameter whose type no value
    /// inhabits: never called, so the correspondence says nothing of it.
    fn empty_case(
        &mut self,
        function: &Function,
        position: usize,
        statement: K,
    ) -> Result<K, String> {
        let empty = self.empty.clone();
        let u = {
            let nat = self.w.b.konst("Nat");
            let items: Vec<K> = empty.iter().map(|n| self.w.nat_u(*n)).collect();
            self.w.b.list(nat, &items)
        };
        let t = self.w.ty_term(&function.types[position]);
        let ty_ty = self.w.ts("Ty");
        let items: Vec<K> = function.types.iter().map(|ty| self.w.ty_term(ty)).collect();
        let witnesses = [u, t];
        self.nested_exists(statement, &witnesses, &mut |this, innermost| {
            let (valid, rest) = this.conjuncts(innermost)?;
            let first = this.refl_of(valid)?;
            let (member, uninhabited) = this.conjuncts(rest)?;
            let second = this.refl_of(uninhabited)?;
            let mut proof = {
                let head = this.w.b.konst0("List.Mem.head");
                let after = this.w.b.list(ty_ty, &items[position + 1..]);
                this.w.b.apps(head, &[ty_ty, items[position], after])
            };
            for at in (0..position).rev() {
                let tail = this.w.b.konst0("List.Mem.tail");
                let after = this.w.b.list(ty_ty, &items[at + 1..]);
                proof = this
                    .w
                    .b
                    .apps(tail, &[ty_ty, items[position], items[at], after, proof]);
            }
            let tail_pair = this.and_intro(member, uninhabited, proof, second);
            Ok(this.and_intro(valid, rest, first, tail_pair))
        })
    }

    /// `n + d` at `Nat`, as the library writes an addition.
    fn nat_add(&mut self, n: K, d: K) -> K {
        let nat = self.w.b.konst("Nat");
        let zero = [CoreLevel::Zero, CoreLevel::Zero, CoreLevel::Zero];
        let add = self.w.b.konst_at("HAdd.hAdd", zero.to_vec());
        let inst = {
            let head = self.w.b.konst_at("instHAdd", vec![CoreLevel::Zero]);
            let add_nat = self.w.b.konst("instAddNat");
            self.w.b.apps(head, &[nat, add_nat])
        };
        self.w.b.apps(add, &[nat, nat, nat, inst, n, d])
    }

    /// `LexLeanRuntime.index list position` over the program's functions.
    fn function_index(&mut self, list: K, position: K) -> K {
        let function_ty = self.w.ts("Function");
        let list_function = self.w.b.list_ty(function_ty);
        let index_head = self.w.b.konst("LexLeanRuntime.index");
        let indexable = {
            let head = self.w.b.konst("LexLeanRuntime.instIndexableList");
            self.w.b.app(head, function_ty)
        };
        self.w.b.apps(
            index_head,
            &[list_function, function_ty, indexable, list, position],
        )
    }

    /// The rows that look every function up in a constant number of kernel
    /// steps (SPEC.md §17.17, *Lookups*): `idx<k>` states
    /// `∀ m, index functions (m + k) = index T_k m`, where `T_k` is the
    /// suffix of the program's function list from `k`, each from the one
    /// before by `Nat.succ_add` and one step of the lookup; `look<k>` is
    /// `idx<k> 0`, the lookup of function `k`. Evaluating the lookup instead
    /// walks `k` cells of the list, once for each function and again at each
    /// call, so the certificate's checking grew with the square of the
    /// number of functions.
    fn lookup_rows(&mut self) -> Result<(Named, Named), String> {
        let nat = self.w.b.konst("Nat");
        let function_ty = self.w.ts("Function");
        let option_function = {
            let option = self.w.b.konst0("Option");
            self.w.b.app(option, function_ty)
        };
        let functions = {
            let head = self.w.ts("Program.functions");
            self.w.b.app(head, self.p)
        };
        let mut terms = Vec::new();
        for function in &self.program.functions {
            terms.push(self.w.function_term(function)?);
        }
        // Each suffix is a definition, `tl<k> := f_k :: tl<k+1>`: a row whose
        // statement held the suffix as a term would make the kernel infer its
        // type again in every row, which grows with the square of the
        // program's size.
        let suffixes: Vec<K> = (0..=terms.len())
            .map(|k| self.row(&format!("tl{k}")))
            .collect();
        let list_function = self.w.b.list_ty(function_ty);
        let mut definitions = Vec::new();
        {
            let nil = self.w.b.konst0("List.nil");
            let empty = self.w.b.app(nil, function_ty);
            definitions.push((format!("tl{}", terms.len()), list_function, empty));
            let cons = self.w.b.konst0("List.cons");
            for k in (0..terms.len()).rev() {
                let value = self
                    .w
                    .b
                    .apps(cons, &[function_ty, terms[k], suffixes[k + 1]]);
                definitions.push((format!("tl{k}"), list_function, value));
            }
        }
        let eq = self.w.b.konst1("Eq");
        let succ = self.w.b.konst("Nat.succ");
        let one = vec![CoreLevel::Succ(Box::new(CoreLevel::Zero))];
        let mut rows = Vec::new();
        for (k, &suffix) in suffixes.iter().enumerate().take(terms.len()) {
            let m = self.w.b.free();
            let d = self.w.nat_u(k as u64);
            let position = self.nat_add(m, d);
            let left = self.function_index(functions, position);
            let right = self.function_index(suffix, m);
            let equation = self.w.b.apps(eq, &[option_function, left, right]);
            let statement = self
                .w
                .b
                .pi_over("m", CoreBinderInfo::Explicit, nat, m, equation);
            let body = if k == 0 {
                // `m + 0` is `m`, and the functions are their own suffix.
                let refl = self.w.b.konst_at("Eq.refl", one.clone());
                let at = self.function_index(functions, m);
                self.w.b.apps(refl, &[option_function, at])
            } else {
                let before = self.w.nat_u(k as u64 - 1);
                let succ_m = self.w.b.app(succ, m);
                // `Nat.succ_add m (k-1) : succ m + (k-1) = succ (m + (k-1))`.
                let moved = self.nat_add(succ_m, before);
                let inner = self.nat_add(m, before);
                let stepped = self.w.b.app(succ, inner);
                let succ_add = {
                    let head = self.w.b.konst("Nat.succ_add");
                    self.w.b.apps(head, &[m, before])
                };
                let i = self.w.b.free();
                let lookup = {
                    let at = self.function_index(functions, i);
                    self.w.b.lam_over("i", CoreBinderInfo::Explicit, nat, i, at)
                };
                let congr = {
                    let head = self
                        .w
                        .b
                        .konst_at("congrArg", vec![one[0].clone(), one[0].clone()]);
                    self.w.b.apps(
                        head,
                        &[nat, option_function, moved, stepped, lookup, succ_add],
                    )
                };
                let at_moved = self.function_index(functions, moved);
                let at_stepped = self.function_index(functions, stepped);
                let symm = {
                    let head = self.w.b.konst_at("Eq.symm", one.clone());
                    self.w
                        .b
                        .apps(head, &[option_function, at_moved, at_stepped, congr])
                };
                let previous = self.row(&format!("idx{}", k - 1));
                let shifted = self.w.b.app(previous, succ_m);
                let reached = self.function_index(suffixes[k - 1], succ_m);
                let trans = self.w.b.konst_at("Eq.trans", one.clone());
                self.w.b.apps(
                    trans,
                    &[
                        option_function,
                        at_stepped,
                        at_moved,
                        reached,
                        symm,
                        shifted,
                    ],
                )
            };
            let proof = self
                .w
                .b
                .lam_over("m", CoreBinderInfo::Explicit, nat, m, body);
            rows.push((format!("idx{k}"), statement, proof));
        }
        for (k, term) in terms.iter().enumerate() {
            let d = self.w.nat_u(k as u64);
            let left = self.function_index(functions, d);
            let some = self.w.b.konst0("Option.some");
            let found = self.w.b.apps(some, &[function_ty, *term]);
            let statement = self.w.b.apps(eq, &[option_function, left, found]);
            let zero = self.w.nat_u(0);
            let chain = self.row(&format!("idx{k}"));
            let proof = self.w.b.app(chain, zero);
            rows.push((format!("look{k}"), statement, proof));
        }
        Ok((definitions, rows))
    }

    /// `LexLeanTarget.RustSemantics.<name>`.
    fn semantics(&mut self, name: &str) -> K {
        self.w
            .b
            .konst(&format!("LexLeanTarget.RustSemantics.{name}"))
    }

    /// `Option (List Parameter × RustType × Block)`, what `findFunction`
    /// answers.
    fn found_ty(&mut self) -> K {
        let parameters = {
            let parameter = self.w.rs("Parameter");
            self.w.b.list_ty(parameter)
        };
        let rty = self.w.rs("RustType");
        let block = self.w.rs("Block");
        let prod = self.w.b.konst00("Prod");
        let inner = self.w.b.apps(prod, &[rty, block]);
        let pair = self.w.b.apps(prod, &[parameters, inner]);
        let option = self.w.b.konst0("Option");
        self.w.b.app(option, pair)
    }

    /// `Eq.trans` at `α`: `a = b → b = c → a = c`.
    fn trans(&mut self, alpha: K, a: K, b: K, c: K, first: K, second: K) -> K {
        let one = vec![CoreLevel::Succ(Box::new(CoreLevel::Zero))];
        let head = self.w.b.konst_at("Eq.trans", one);
        self.w.b.apps(head, &[alpha, a, b, c, first, second])
    }

    fn eq_refl(&mut self, alpha: K, a: K) -> K {
        let one = vec![CoreLevel::Succ(Box::new(CoreLevel::Zero))];
        let head = self.w.b.konst_at("Eq.refl", one);
        self.w.b.apps(head, &[alpha, a])
    }

    /// `n ≤ m` at `Nat`.
    fn nat_le(&mut self, n: K, m: K) -> K {
        let nat = self.w.b.konst("Nat");
        let le = self.w.b.konst0("LE.le");
        let inst = self.w.b.konst("instLENat");
        self.w.b.apps(le, &[nat, inst, n, m])
    }

    /// A proof of `n ≤ m` for literals, which the kernel decides by
    /// evaluating `Nat.ble`.
    fn le_literal(&mut self, n: u64, m: u64) -> K {
        let n = self.w.nat_u(n);
        let m = self.w.nat_u(m);
        let tru = self.w.b.konst("Bool.true");
        let boolean = self.w.b.konst("Bool");
        let refl = self.eq_refl(boolean, tru);
        let head = self.w.b.konst("Nat.le_of_ble_eq_true");
        self.w.b.apps(head, &[n, m, refl])
    }

    /// A proof of `low ≤ w` from `hw : high ≤ w`, for literals `low ≤ high`.
    fn le_down(&mut self, low: u64, high: u64, w: K, hw: K) -> K {
        if low == high {
            return hw;
        }
        let below = self.le_literal(low, high);
        let low_term = self.w.nat_u(low);
        let high_term = self.w.nat_u(high);
        let head = self.w.b.konst("Nat.le_trans");
        self.w.b.apps(head, &[low_term, high_term, w, below, hw])
    }

    /// The crate's items and, for each item, the least bound past every
    /// function the items before it name.
    fn item_bounds(&self) -> Vec<u64> {
        let mut bounds = Vec::new();
        let mut bound = 0;
        for item in &self.krate().items {
            bounds.push(bound);
            if let ItemDef::Function {
                name: crate::calculus::rust::ast::Ident::Function(index),
                ..
            } = item
            {
                bound = bound.max(index + 1);
            }
        }
        bounds.push(bound);
        bounds
    }

    /// The position of function `index`'s item in the crate.
    fn item_of(&self, index: u64) -> Result<usize, String> {
        self.krate()
            .items
            .iter()
            .position(|item| {
                matches!(item, ItemDef::Function {
                    name: crate::calculus::rust::ast::Ident::Function(found),
                    ..
                } if *found == index)
            })
            .ok_or_else(|| format!("the crate does not render function {index}"))
    }

    /// The rows that find each function's item in a constant number of kernel
    /// steps (SPEC.md §17.17, *Lookups*). `it<i>` is the suffix of the
    /// crate's items from `i`; `fnd<i>` states that every function at or
    /// past the bound of the items before `i` is found in `it<i>` as in the
    /// whole crate, each from the one before by one step of `findFunction`:
    /// the generic `stepE`, `stepA`, and `stepF`, and, past a function the
    /// crate names, `iteF` at the refuted comparison. `fal<f>` is the
    /// fallibility of function `f` the calls state, from `fnd` at its item.
    fn find_rows(&mut self) -> Result<(Named, Named), String> {
        let nat = self.w.b.konst("Nat");
        let boolean = self.w.b.konst("Bool");
        let tru = self.w.b.konst("Bool.true");
        let fls = self.w.b.konst("Bool.false");
        let item_ty = self.w.rs("ItemDef");
        let items_ty = self.w.b.list_ty(item_ty);
        let ident_ty = self.w.rs("Ident");
        let rty = self.w.rs("RustType");
        let block = self.w.rs("Block");
        let parameters_ty = {
            let parameter = self.w.rs("Parameter");
            self.w.b.list_ty(parameter)
        };
        let found_ty = self.found_ty();
        let eq = self.w.b.konst1("Eq");
        let cons = self.w.b.konst0("List.cons");
        let find = self.semantics("findFunction");
        let same = self.semantics("sameIdent");
        let decide = self.w.b.konst("instDecidableEqBool");
        let one = vec![CoreLevel::Succ(Box::new(CoreLevel::Zero))];
        let ite = self.w.b.konst_at("ite", one.clone());
        let krate = self.krate().clone();
        let mut items = Vec::new();
        for item in &krate.items {
            items.push(self.w.rs_item_def(item));
        }
        let suffixes: Vec<K> = (0..=items.len())
            .map(|i| self.row(&format!("it{i}")))
            .collect();
        let mut definitions = Vec::new();
        {
            let nil = self.w.b.konst0("List.nil");
            let empty = self.w.b.app(nil, item_ty);
            definitions.push((format!("it{}", items.len()), items_ty, empty));
            for i in (0..items.len()).rev() {
                let value = self.w.b.apps(cons, &[item_ty, items[i], suffixes[i + 1]]);
                definitions.push((format!("it{i}"), items_ty, value));
            }
        }
        let mut rows = Vec::new();
        // `ite (c = true) a b` at what `findFunction` answers.
        let branch = |this: &mut Self, c: K, a: K, b: K| -> K {
            let is_true = this.w.b.apps(eq, &[boolean, c, tru]);
            let decided = this.w.b.apps(decide, &[c, tru]);
            this.w.b.apps(ite, &[found_ty, is_true, decided, a, b])
        };
        // stepE, stepA, stepF: one step of `findFunction` past an item.
        let step = |this: &mut Self,
                    name: &str,
                    ctor: &str,
                    binders: &[(&str, K)],
                    right: &StepRight<'_, 'g, 'a>|
         -> (String, K, K) {
            let frees: Vec<K> = binders.iter().map(|_| this.w.b.free()).collect();
            let rest = this.w.b.free();
            let w = this.w.b.free();
            let head = this.w.rs(ctor);
            let item = this.w.b.apps(head, &frees);
            let list = this.w.b.apps(cons, &[item_ty, item, rest]);
            let left = this.w.b.apps(find, &[list, w]);
            let rhs = right(this, &frees, rest, w);
            let mut statement = this.w.b.apps(eq, &[found_ty, left, rhs]);
            let mut proof = this.eq_refl(found_ty, left);
            let mut all: Vec<(&str, K, K)> = binders
                .iter()
                .zip(&frees)
                .map(|((name, ty), free)| (*name, *ty, *free))
                .collect();
            all.push(("rest", items_ty, rest));
            all.push(("w", ident_ty, w));
            for (name, ty, free) in all.into_iter().rev() {
                statement = this
                    .w
                    .b
                    .pi_over(name, CoreBinderInfo::Explicit, ty, free, statement);
                proof = this
                    .w
                    .b
                    .lam_over(name, CoreBinderInfo::Explicit, ty, free, proof);
            }
            (name.to_owned(), statement, proof)
        };
        let skip =
            |this: &mut Self, _: &[K], rest: K, w: K| -> K { this.w.b.apps(find, &[rest, w]) };
        let variants_ty = {
            let variant = self.w.rs("Variant");
            self.w.b.list_ty(variant)
        };
        let types_ty = self.w.b.list_ty(rty);
        let dispatches_ty = {
            let dispatch = self.w.rs("Dispatch");
            self.w.b.list_ty(dispatch)
        };
        rows.push(step(
            self,
            "stepE",
            "ItemDef.enum",
            &[("t", rty), ("vs", variants_ty)],
            &skip,
        ));
        rows.push(step(
            self,
            "stepA",
            "ItemDef.apply",
            &[
                ("n", nat),
                ("ps", types_ty),
                ("r", rty),
                ("fl", boolean),
                ("ds", dispatches_ty),
            ],
            &skip,
        ));
        let prod_mk = self.w.b.konst00("Prod.mk");
        let some = self.w.b.konst0("Option.some");
        let pair_ty = {
            let prod = self.w.b.konst00("Prod");
            self.w.b.apps(prod, &[rty, block])
        };
        let hit = |this: &mut Self, fields: &[K], w: K| -> K {
            let inner = this.w.b.apps(prod_mk, &[rty, block, fields[2], fields[3]]);
            let pair = this
                .w
                .b
                .apps(prod_mk, &[parameters_ty, pair_ty, fields[1], inner]);
            let held = {
                let (_, args) = this.w.b.spine(found_ty);
                args[0]
            };
            let _ = w;
            this.w.b.apps(some, &[held, pair])
        };
        let found_step = |this: &mut Self, fields: &[K], rest: K, w: K| -> K {
            let c = this.w.b.apps(same, &[fields[0], w]);
            let a = hit(this, fields, w);
            let b = this.w.b.apps(find, &[rest, w]);
            branch(this, c, a, b)
        };
        rows.push(step(
            self,
            "stepF",
            "ItemDef.function",
            &[
                ("name", ident_ty),
                ("ps", parameters_ty),
                ("r", rty),
                ("b", block),
            ],
            &found_step,
        ));
        // iteF : ∀ a b, ite (false = true) a b = b.
        {
            let a = self.w.b.free();
            let b = self.w.b.free();
            let left = branch(self, fls, a, b);
            let statement = self.w.b.apps(eq, &[found_ty, left, b]);
            let proof = self.eq_refl(found_ty, b);
            let statement = self
                .w
                .b
                .pi_over("b", CoreBinderInfo::Explicit, found_ty, b, statement);
            let statement = self
                .w
                .b
                .pi_over("a", CoreBinderInfo::Explicit, found_ty, a, statement);
            let proof = self
                .w
                .b
                .lam_over("b", CoreBinderInfo::Explicit, found_ty, b, proof);
            let proof = self
                .w
                .b
                .lam_over("a", CoreBinderInfo::Explicit, found_ty, a, proof);
            rows.push(("iteF".to_owned(), statement, proof));
        }
        let fn_ident = self.rust_const("fnIdent");
        let crate_items = {
            let head = self.w.rs("Crate.items");
            self.w.b.app(head, self.c)
        };
        let bounds = self.item_bounds();
        let fnd_statement = |this: &mut Self, i: usize, w: K, hw: K| -> (K, K, K) {
            let x = this.w.b.app(fn_ident, w);
            let whole = this.w.b.apps(find, &[crate_items, x]);
            let part = this.w.b.apps(find, &[suffixes[i], x]);
            let equation = this.w.b.apps(eq, &[found_ty, whole, part]);
            let bound = this.w.nat_u(bounds[i]);
            let guard = this.nat_le(bound, w);
            let inner = this
                .w
                .b
                .pi_over("hw", CoreBinderInfo::Explicit, guard, hw, equation);
            let statement = this
                .w
                .b
                .pi_over("w", CoreBinderInfo::Explicit, nat, w, inner);
            (statement, guard, x)
        };
        for i in 0..=items.len() {
            let w = self.w.b.free();
            let hw = self.w.b.free();
            let (statement, guard, x) = fnd_statement(self, i, w, hw);
            let whole = self.w.b.apps(find, &[crate_items, x]);
            let body = if i == 0 {
                self.eq_refl(found_ty, whole)
            } else {
                let before = i - 1;
                let previous = {
                    let head = self.row(&format!("fnd{before}"));
                    let lowered = self.le_down(bounds[before], bounds[i], w, hw);
                    self.w.b.apps(head, &[w, lowered])
                };
                let from = self.w.b.apps(find, &[suffixes[before], x]);
                let to = self.w.b.apps(find, &[suffixes[i], x]);
                let (head, fields) = self.w.b.spine(items[before]);
                let _ = head;
                let stepped = match &krate.items[before] {
                    ItemDef::Enum { .. } => {
                        let lemma = self.row("stepE");
                        let mut args = fields.clone();
                        args.extend([suffixes[i], x]);
                        self.w.b.apps(lemma, &args)
                    }
                    ItemDef::Apply { .. } => {
                        let lemma = self.row("stepA");
                        let mut args = fields.clone();
                        args.extend([suffixes[i], x]);
                        self.w.b.apps(lemma, &args)
                    }
                    ItemDef::Function { name, .. } => {
                        let lemma = self.row("stepF");
                        let mut args = fields.clone();
                        args.extend([suffixes[i], x]);
                        let unfolded = self.w.b.apps(lemma, &args);
                        let c = self.w.b.apps(same, &[fields[0], x]);
                        let a = hit(self, &fields, x);
                        let compared = branch(self, c, a, to);
                        let refuted = branch(self, fls, a, to);
                        // `sameIdent name (fnIdent w) = false`.
                        let differs = match name {
                            crate::calculus::rust::ast::Ident::Function(j) => {
                                let j_term = self.w.nat_u(*j);
                                let is_true = self.w.b.apps(eq, &[boolean, c, tru]);
                                let h = self.w.b.free();
                                let equal = {
                                    let head = self.w.b.konst("Nat.eq_of_beq_eq_true");
                                    self.w.b.apps(head, &[j_term, w, h])
                                };
                                let below = self.le_down(j + 1, bounds[i], w, hw);
                                let distinct = {
                                    let head = self.w.b.konst("Nat.ne_of_lt");
                                    self.w.b.apps(head, &[j_term, w, below, equal])
                                };
                                let refute = self.w.b.lam_over(
                                    "h",
                                    CoreBinderInfo::Explicit,
                                    is_true,
                                    h,
                                    distinct,
                                );
                                let head = self.w.b.konst("Bool.of_not_eq_true");
                                self.w.b.apps(head, &[c, refute])
                            }
                            _ => self.eq_refl(boolean, fls),
                        };
                        let rewrite = {
                            let cv = self.w.b.free();
                            let body = branch(self, cv, a, to);
                            let motive =
                                self.w
                                    .b
                                    .lam_over("c", CoreBinderInfo::Explicit, boolean, cv, body);
                            let head = self
                                .w
                                .b
                                .konst_at("congrArg", vec![one[0].clone(), one[0].clone()]);
                            self.w
                                .b
                                .apps(head, &[boolean, found_ty, c, fls, motive, differs])
                        };
                        let reduced = {
                            let head = self.row("iteF");
                            self.w.b.apps(head, &[a, to])
                        };
                        let tail = self.trans(found_ty, compared, refuted, to, rewrite, reduced);
                        self.trans(found_ty, from, compared, to, unfolded, tail)
                    }
                };
                self.trans(found_ty, whole, from, to, previous, stepped)
            };
            let inner = self
                .w
                .b
                .lam_over("hw", CoreBinderInfo::Explicit, guard, hw, body);
            let proof = self
                .w
                .b
                .lam_over("w", CoreBinderInfo::Explicit, nat, w, inner);
            rows.push((format!("fnd{i}"), statement, proof));
        }
        // fal<f> : fnFallible krate f = some F.
        let fallible = self.rust_const("fnFallible");
        let option_bool = {
            let option = self.w.b.konst0("Option");
            self.w.b.app(option, boolean)
        };
        for index in 0..self.program.functions.len() as u64 {
            let position = self.item_of(index)?;
            let f = self.w.nat_u(index);
            let left = self.w.b.apps(fallible, &[self.c, f]);
            let flag = self.w.boolean(self.fallible_fn(index));
            let answer = self.w.b.apps(some, &[boolean, flag]);
            let statement = self.w.b.apps(eq, &[option_bool, left, answer]);
            let unfolded = self.unfold(left)?;
            let (map_head, map_args) = self.w.b.spine(unfolded);
            if map_args.len() != 4 {
                return Err("fnFallible is not an Option.map of a lookup".to_owned());
            }
            let mapped = self.w.b.apps(map_head, &map_args[..3]);
            let x = self.w.b.app(fn_ident, f);
            let whole = self.w.b.apps(find, &[crate_items, x]);
            let part = self.w.b.apps(find, &[suffixes[position], x]);
            let guard = self.le_literal(bounds[position], index);
            let chained = {
                let head = self.row(&format!("fnd{position}"));
                self.w.b.apps(head, &[f, guard])
            };
            let congr = {
                let head = self
                    .w
                    .b
                    .konst_at("congrArg", vec![one[0].clone(), one[0].clone()]);
                self.w
                    .b
                    .apps(head, &[found_ty, option_bool, whole, part, mapped, chained])
            };
            let mapped_part = self.w.b.app(mapped, part);
            let evaluated = self.eq_refl(option_bool, answer);
            let proof = self.trans(option_bool, left, mapped_part, answer, congr, evaluated);
            rows.push((format!("fal{index}"), statement, proof));
        }
        Ok((definitions, rows))
    }

    /// FunOK0's `findFunction (Crate.items krate) (fnIdent f) = some …`, from
    /// `fnd` at the function's item and one evaluation of the lookup there.
    fn found_in_crate(&mut self, index: u64, found: K) -> Result<K, String> {
        let (_, parts) = self.w.b.spine(found);
        let [alpha, whole, answer] = parts[..] else {
            return Err("the crate lookup is not an equation".to_owned());
        };
        let position = self.item_of(index)?;
        let bounds = self.item_bounds();
        let find = self.semantics("findFunction");
        let fn_ident = self.rust_const("fnIdent");
        let f = self.w.nat_u(index);
        let x = self.w.b.app(fn_ident, f);
        let suffix = self.row(&format!("it{position}"));
        let part = self.w.b.apps(find, &[suffix, x]);
        let guard = self.le_literal(bounds[position], index);
        let chained = {
            let head = self.row(&format!("fnd{position}"));
            self.w.b.apps(head, &[f, guard])
        };
        let evaluated = self.eq_refl(alpha, answer);
        Ok(self.trans(alpha, whole, part, answer, chained, evaluated))
    }

    /// `crate_ok`'s value: the case analysis on the function index that
    /// hands each function its theorem and refutes an index past the last.
    fn crate_ok_value(&mut self, count: u64) -> Result<K, String> {
        let nat = self.w.b.konst("Nat");
        let function_ty = self.w.ts("Function");
        let option = self.w.b.konst0("Option");
        let option_function = self.w.b.app(option, function_ty);
        let functions = {
            let head = self.w.ts("Program.functions");
            self.w.b.app(head, self.p)
        };
        let list_ty = self.w.b.list_ty(function_ty);
        let _ = list_ty;
        let index_head = self.w.b.konst("LexLeanRuntime.index");
        let indexable = {
            let head = self.w.b.konst("LexLeanRuntime.instIndexableList");
            self.w.b.app(head, function_ty)
        };
        let list_function = self.w.b.list_ty(function_ty);
        let fun_ok = self.rust_const("FunOK");
        let at = |this: &mut Self, x: K, d: u64| -> K {
            let zero = [CoreLevel::Zero, CoreLevel::Zero, CoreLevel::Zero];
            let add = this.w.b.konst_at("HAdd.hAdd", zero.to_vec());
            let inst = {
                let head = this.w.b.konst_at("instHAdd", vec![CoreLevel::Zero]);
                let add_nat = this.w.b.konst("instAddNat");
                this.w.b.apps(head, &[nat, add_nat])
            };
            let d = this.w.nat_u(d);
            this.w.b.apps(add, &[nat, nat, nat, inst, x, d])
        };
        let statement = |this: &mut Self, x: K, d: u64, fun: K, h: K| -> (K, K, K) {
            // The type of `h` and the conclusion, over `x`, `fun`.
            let position = at(this, x, d);
            let looked = this.w.b.apps(
                index_head,
                &[list_function, function_ty, indexable, functions, position],
            );
            let some = this.w.b.konst0("Option.some");
            let found = this.w.b.apps(some, &[function_ty, fun]);
            let eq = this.w.b.konst1("Eq");
            let hypothesis = this.w.b.apps(eq, &[option_function, looked, found]);
            let conclusion = this.w.b.apps(fun_ok, &[this.p, this.c, this.a, position]);
            let _ = h;
            (hypothesis, conclusion, position)
        };
        // The innermost level: an index past the last function looks up
        // nothing.
        let x = self.w.b.free();
        let fun = self.w.b.free();
        let h = self.w.b.free();
        let (hypothesis, conclusion, _) = statement(self, x, count, fun, h);
        let none = {
            let head = self.w.b.konst0("Option.none");
            self.w.b.app(head, function_ty)
        };
        let some = self.w.b.konst0("Option.some");
        let found = self.w.b.apps(some, &[function_ty, fun]);
        let refuted = {
            // `Option.noConfusion` takes the equality of the carriers and the
            // heterogeneous equality of the options: the hypothesis, which
            // the kernel reads as `none = some fn` by evaluating the lookup
            // past the last function, is the second.
            let head = self
                .w
                .b
                .konst_at("Option.noConfusion", vec![CoreLevel::Zero, CoreLevel::Zero]);
            let type_level = CoreLevel::Succ(Box::new(CoreLevel::Succ(Box::new(CoreLevel::Zero))));
            let refl = self.w.b.konst_at("Eq.refl", vec![type_level]);
            let type0 = self.w.b.type0();
            let carriers = self.w.b.apps(refl, &[type0, function_ty]);
            let heq = {
                let head = self.w.b.konst_at(
                    "heq_of_eq",
                    vec![CoreLevel::Succ(Box::new(CoreLevel::Zero))],
                );
                self.w.b.apps(head, &[option_function, none, found, h])
            };
            self.w.b.apps(
                head,
                &[
                    conclusion,
                    function_ty,
                    none,
                    function_ty,
                    found,
                    carriers,
                    heq,
                ],
            )
        };
        let with_h = self
            .w
            .b
            .lam_over("h", CoreBinderInfo::Explicit, hypothesis, h, refuted);
        let with_fun = self
            .w
            .b
            .lam_over("fn", CoreBinderInfo::Explicit, function_ty, fun, with_h);
        let mut level = self
            .w
            .b
            .lam_over("x", CoreBinderInfo::Explicit, nat, x, with_fun);
        for d in (0..count).rev() {
            let x = self.w.b.free();
            let y = self.w.b.free();
            let fun = self.w.b.free();
            let h = self.w.b.free();
            // The motive: the statement at `y`.
            let (hypothesis_y, conclusion_y, _) = statement(self, y, d, fun, h);
            let pi_h =
                self.w
                    .b
                    .pi_over("h", CoreBinderInfo::Explicit, hypothesis_y, h, conclusion_y);
            let pi_fun = self
                .w
                .b
                .pi_over("fn", CoreBinderInfo::Explicit, function_ty, fun, pi_h);
            let motive = self
                .w
                .b
                .lam_over("y", CoreBinderInfo::Explicit, nat, y, pi_fun);
            // The zero case hands over the theorem of function `d`.
            let fun0 = self.w.b.free();
            let h0 = self.w.b.free();
            let zero = self.w.b.konst("Nat.zero");
            let (hypothesis_0, _, _) = statement(self, zero, d, fun0, h0);
            let theorem = self.row(&format!("fun{d}"));
            let zero_h =
                self.w
                    .b
                    .lam_over("h", CoreBinderInfo::Explicit, hypothesis_0, h0, theorem);
            let zero_case =
                self.w
                    .b
                    .lam_over("fn", CoreBinderInfo::Explicit, function_ty, fun0, zero_h);
            let cases = self.w.b.konst_at("Nat.casesOn", vec![Self::level_zero()]);
            let applied = self.w.b.apps(cases, &[motive, x, zero_case, level]);
            level = self
                .w
                .b
                .lam_over("x", CoreBinderInfo::Explicit, nat, x, applied);
        }
        Ok(level)
    }

    fn crate_ok_type(&mut self) -> K {
        let head = self.rust_const("CrateOK");
        self.w.b.apps(head, &[self.p, self.c, self.a])
    }

    /// `∀ n f, FunSem program krate flags n f` and its proof by `simulate`.
    fn root_row(&mut self) -> Result<(K, K), String> {
        let nat = self.w.b.konst("Nat");
        let n = self.w.b.free();
        let f = self.w.b.free();
        let fun_sem = self.rust_const("FunSem");
        let statement = self.w.b.apps(fun_sem, &[self.p, self.c, self.a, n, f]);
        let simulate = self.rust_const("simulate");
        let crate_ok = self.row("crate_ok");
        let proof = self
            .w
            .b
            .apps(simulate, &[self.p, self.c, self.a, crate_ok, n, f]);
        let inner_ty = self
            .w
            .b
            .pi_over("f", CoreBinderInfo::Explicit, nat, f, statement);
        let ty = self
            .w
            .b
            .pi_over("n", CoreBinderInfo::Explicit, nat, n, inner_ty);
        let inner_value = self
            .w
            .b
            .lam_over("f", CoreBinderInfo::Explicit, nat, f, proof);
        let value = self
            .w
            .b
            .lam_over("n", CoreBinderInfo::Explicit, nat, n, inner_value);
        Ok((ty, value))
    }
}

/// Certificate B of one lowered root's rendering as a native core module
/// (SPEC.md §17.17).
///
/// # Errors
///
/// The reason the aligner cannot relate the crate to the program, or, prefixed
/// with [`LIMIT`], that the certificate would exceed `max_bytes`.
pub fn certificate_b_data(
    program: &Program,
    krate: &Crate,
    module: &str,
    max_bytes: u64,
) -> Result<CoreModule, String> {
    let work = || build(program, krate, module, max_bytes);
    // A derivation nests as deeply as a match has arms, and the kernel terms
    // it is built from are walked recursively: the work has a stack of its
    // own.
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .stack_size(1 << 30)
            .spawn_scoped(scope, work)
            .map_err(|error| error.to_string())?
            .join()
            .map_err(|_| "certificate B overran its stack".to_owned())?
    })
}

fn build(
    program: &Program,
    krate: &Crate,
    module: &str,
    max_bytes: u64,
) -> Result<CoreModule, String> {
    let modules = BTreeMap::new();
    let layout = Layout {
        functions: Vec::new(),
        adts: Vec::new(),
    };
    let mut generator = Gen {
        source: Source { modules: &modules },
        layout: &layout,
        program,
        instances: BTreeMap::new(),
        lambdas: BTreeMap::new(),
        templates: BTreeMap::new(),
        adts: BTreeMap::new(),
        subjects: BTreeMap::new(),
        found: BTreeMap::new(),
        nested: BTreeMap::new(),
        limit: max_bytes,
        module: module.to_owned(),
    };
    let writer = Writer::new(&mut generator, module);
    let mut aligner = Aligner::new(writer, program, krate, max_bytes, module)?;
    let program_value = aligner.w.program_value()?;
    let crate_value = aligner.w.rs_crate(krate);
    aligner.count_new(program_value);
    aligner.count_new(crate_value);
    let weighed = aligner.weighed();
    if weighed > max_bytes {
        return Err(format!(
            "{LIMIT}the program and crate terms are at least {weighed} bytes, beyond the {max_bytes} bytes of max_file_bytes"
        ));
    }
    let (suffixes, lookups) = aligner.lookup_rows()?;
    let (item_suffixes, finds) = aligner.find_rows()?;
    let mut rows = Rows {
        functions: Vec::new(),
    };
    for index in 0..program.functions.len() as u64 {
        let proof = aligner
            .function_proof(index)
            .map_err(|reason| match aligner.over.clone() {
                Some(refusal) => refusal,
                None => format!("function {index}: {reason}"),
            })?;
        let fun_ok = aligner.rust_const("FunOK");
        let at = aligner.w.nat_u(index);
        let statement = aligner
            .w
            .b
            .apps(fun_ok, &[aligner.p, aligner.c, aligner.a, at]);
        rows.functions
            .push((format!("fun{index}"), statement, proof));
    }
    let flags_value = aligner.flags()?;
    let count = program.functions.len() as u64;
    let crate_ok_value = aligner.crate_ok_value(count)?;
    let crate_ok_ty = aligner.crate_ok_type();
    let (root_ty, root_value) = aligner.root_row()?;
    let program_ty = aligner.w.ts("Program");
    let crate_ty = aligner.w.rs("Crate");
    let flags_ty = aligner.rust_const("Flags");
    let theorem = RowKind::Theorem;
    let definition = RowKind::Definition { compiled: false };
    aligner
        .w
        .push_row("program", definition, program_ty, program_value)?;
    aligner
        .w
        .push_row("krate", definition, crate_ty, crate_value)?;
    aligner
        .w
        .push_row("flags", definition, flags_ty, flags_value)?;
    for (name, ty, value) in suffixes {
        aligner.w.push_row(&name, definition, ty, value)?;
    }
    for (name, ty, value) in item_suffixes {
        aligner.w.push_row(&name, definition, ty, value)?;
    }
    for (name, statement, proof) in finds {
        aligner.w.push_row(&name, theorem, statement, proof)?;
    }
    for (name, statement, proof) in lookups {
        aligner.w.push_row(&name, theorem, statement, proof)?;
    }
    for (name, statement, proof) in rows.functions {
        aligner.w.push_row(&name, theorem, statement, proof)?;
    }
    aligner
        .w
        .push_row("crate_ok", theorem, crate_ok_ty, crate_ok_value)?;
    aligner.w.push_row("root", theorem, root_ty, root_value)?;
    aligner.w.finish(vec![SOUNDNESS_MODULE.to_owned()])
}

/// Certificate B as data, its Lean text, and its record, under `max_bytes`.
///
/// # Errors
///
/// As [`certificate_b_data`]; or, prefixed with [`LIMIT`], that the text
/// exceeds `max_bytes`.
pub fn render(
    program: &Program,
    krate: &Crate,
    module: &str,
    max_bytes: u64,
) -> Result<(CoreModule, String, String), String> {
    let data = certificate_b_data(program, krate, module, max_bytes)?;
    let text = crate::backend::core::lean_text_legacy(&data, module)
        .map_err(|diagnostic| diagnostic.message)?;
    let source = super::record_source(module, &data, "B")?;
    // A certificate held as data weighs what it is recorded as, the notation
    // (as certificate A does): the text carries the decoder and the data a
    // second time, escaped, which is the backend's form and not the
    // certificate's size.
    if source.len() as u64 > max_bytes {
        return Err(format!(
            "{LIMIT}the certificate is {} bytes, beyond the {max_bytes} bytes of max_file_bytes",
            source.len()
        ));
    }
    Ok((data, text, source))
}

/// The crates of `crates` as computable definitions `__crate<i> :
/// RustSyntax.Crate` of one native core module, rendered like a certificate: the Rust machine the
/// conformance cases run evaluates a crate by its term, which is the same
/// term certificate B states its theorem about.
///
/// # Errors
///
/// A module that does not validate.
pub fn crates_module(module: &str, crates: &[&Crate]) -> Result<(CoreModule, String), String> {
    let program = Program {
        spec: String::new(),
        adts: Vec::new(),
        functions: Vec::new(),
    };
    let modules = BTreeMap::new();
    let layout = Layout {
        functions: Vec::new(),
        adts: Vec::new(),
    };
    let mut generator = Gen {
        source: Source { modules: &modules },
        layout: &layout,
        program: &program,
        instances: BTreeMap::new(),
        lambdas: BTreeMap::new(),
        templates: BTreeMap::new(),
        adts: BTreeMap::new(),
        subjects: BTreeMap::new(),
        found: BTreeMap::new(),
        nested: BTreeMap::new(),
        limit: u64::MAX,
        module: module.to_owned(),
    };
    let mut writer = Writer::new(&mut generator, module);
    for (position, krate) in crates.iter().enumerate() {
        let value = writer.rs_crate(krate);
        let ty = writer.rs("Crate");
        writer.push_row(
            &format!("__crate{position}"),
            RowKind::Definition { compiled: true },
            ty,
            value,
        )?;
    }
    let data = writer.finish(vec!["LexLeanTarget.RustSyntax".to_owned()])?;
    let text = crate::backend::core::lean_text_legacy(&data, module)
        .map_err(|diagnostic| diagnostic.message)?;
    Ok((data, text))
}

/// The crate's own rows of certificate B for `program` and `krate`: the
/// program, the crate, the crate items' suffixes, the steps of
/// `findFunction`, its chain, and each function's fallibility (SPEC.md
/// §17.17, *Lookups*). They state facts of the crate alone, decided from it.
fn crate_rows(program: &Program, krate: &Crate, module: &str) -> Result<CoreModule, String> {
    let modules = BTreeMap::new();
    let layout = Layout {
        functions: Vec::new(),
        adts: Vec::new(),
    };
    let mut generator = Gen {
        source: Source { modules: &modules },
        layout: &layout,
        program,
        instances: BTreeMap::new(),
        lambdas: BTreeMap::new(),
        templates: BTreeMap::new(),
        adts: BTreeMap::new(),
        subjects: BTreeMap::new(),
        found: BTreeMap::new(),
        nested: BTreeMap::new(),
        limit: u64::MAX,
        module: module.to_owned(),
    };
    let writer = Writer::new(&mut generator, module);
    let mut aligner = Aligner::new(writer, program, krate, u64::MAX, module)?;
    let program_value = aligner.w.program_value()?;
    let crate_value = aligner.w.rs_crate(krate);
    let (item_suffixes, finds) = aligner.find_rows()?;
    let program_ty = aligner.w.ts("Program");
    let crate_ty = aligner.w.rs("Crate");
    let definition = RowKind::Definition { compiled: false };
    aligner
        .w
        .push_row("program", definition, program_ty, program_value)?;
    aligner
        .w
        .push_row("krate", definition, crate_ty, crate_value)?;
    for (name, ty, value) in item_suffixes {
        aligner.w.push_row(&name, definition, ty, value)?;
    }
    for (name, statement, proof) in finds {
        aligner
            .w
            .push_row(&name, RowKind::Theorem, statement, proof)?;
    }
    aligner.w.finish(vec![SOUNDNESS_MODULE.to_owned()])
}

/// A certificate B module restated over another crate: every row as it is,
/// but the crate's own rows, the term of `krate` and the rows decided from it
/// alone (its items' suffixes, the `findFunction` chain, and the
/// fallibilities), which are stated again over `krate`. The derivations
/// `fun<n>` then claim facts of a crate they were not written for, which the
/// kernel refuses at the derivation of the function whose rendering changed.
///
/// # Errors
///
/// A module that does not validate, or a crate whose own rows cannot be
/// stated for `program`.
pub fn restated_over(
    data: &CoreModule,
    module: &str,
    program: &Program,
    krate: &Crate,
) -> Result<CoreModule, String> {
    let fresh = crate_rows(program, krate, module)?;
    let own_rows: BTreeMap<&str, &crate::ir::core::CoreDeclaration> = fresh
        .declarations
        .iter()
        .map(|row| (row.name.as_str(), row))
        .collect();
    let program = Program {
        spec: String::new(),
        adts: Vec::new(),
        functions: Vec::new(),
    };
    let modules = BTreeMap::new();
    let layout = Layout {
        functions: Vec::new(),
        adts: Vec::new(),
    };
    let mut generator = Gen {
        source: Source { modules: &modules },
        layout: &layout,
        program: &program,
        instances: BTreeMap::new(),
        lambdas: BTreeMap::new(),
        templates: BTreeMap::new(),
        adts: BTreeMap::new(),
        subjects: BTreeMap::new(),
        found: BTreeMap::new(),
        nested: BTreeMap::new(),
        limit: u64::MAX,
        module: module.to_owned(),
    };
    let mut writer = Writer::new(&mut generator, module);
    let own = format!("{module}.krate");
    for row in &data.declarations {
        let Some(root) = row.r#type else {
            return Err(format!("the row `{}` states no type", row.name));
        };
        let (ty, value) = match own_rows.get(row.name.as_str()) {
            Some(fresh_row) if row.name != own => {
                let (Some(fresh_ty), Some(fresh_value)) = (fresh_row.r#type, fresh_row.value)
                else {
                    return Err(format!("the row `{}` is not stated again", row.name));
                };
                (
                    writer.b.import(&fresh, fresh_ty),
                    writer.b.import(&fresh, fresh_value),
                )
            }
            _ => {
                let ty = writer.b.import(data, root);
                let value = match row.value {
                    Some(_) if row.name == own => writer.rs_crate(krate),
                    Some(value) => writer.b.import(data, value),
                    None => return Err(format!("the row `{}` has no value", row.name)),
                };
                (ty, value)
            }
        };
        let kind = match row.kind {
            crate::ir::core::CoreDeclKind::Theorem => RowKind::Theorem,
            _ => RowKind::Definition {
                compiled: row.compiled,
            },
        };
        let short = row
            .name
            .strip_prefix(&format!("{module}."))
            .ok_or_else(|| format!("the row `{}` is outside the module", row.name))?;
        writer.push_row(short, kind, ty, value)?;
    }
    writer.finish(data.imports.clone())
}

/// `data` with its module renamed: every row and every constant of the module
/// under `to` instead of `from`.
pub fn renamed(data: &CoreModule, from: &str, to: &str) -> CoreModule {
    let prefix = format!("{from}.");
    let swap = |name: &str| -> String {
        match name.strip_prefix(&prefix) {
            Some(rest) => format!("{to}.{rest}"),
            None => name.to_owned(),
        }
    };
    let mut out = data.clone();
    for node in &mut out.nodes {
        if let CoreNode::Const { n, .. } = node {
            *n = swap(n);
        }
    }
    for row in &mut out.declarations {
        row.name = swap(&row.name);
        if let Some(all) = &mut row.all {
            for member in all {
                *member = swap(member);
            }
        }
    }
    out
}

/// Run `work` over a writer that has no source: the kernel terms of the Rust
/// syntax need only the builder.
fn with_writer<R>(module: &str, work: impl FnOnce(&mut Writer<'_, '_>) -> R) -> R {
    let program = Program {
        spec: String::new(),
        adts: Vec::new(),
        functions: Vec::new(),
    };
    let modules = BTreeMap::new();
    let layout = Layout {
        functions: Vec::new(),
        adts: Vec::new(),
    };
    let mut generator = Gen {
        source: Source { modules: &modules },
        layout: &layout,
        program: &program,
        instances: BTreeMap::new(),
        lambdas: BTreeMap::new(),
        templates: BTreeMap::new(),
        adts: BTreeMap::new(),
        subjects: BTreeMap::new(),
        found: BTreeMap::new(),
        nested: BTreeMap::new(),
        limit: u64::MAX,
        module: module.to_owned(),
    };
    let mut writer = Writer::new(&mut generator, module);
    work(&mut writer)
}

/// The constructor of `RustSyntax.Item` a runtime item is written as, and the
/// number of arguments it takes.
#[must_use]
pub fn item_constructor(item: crate::calculus::rust::runtime::Item) -> Option<(String, usize)> {
    with_writer("Scratch", |writer| {
        let term = writer.rs_item(item);
        let (head, arguments) = writer.b.spine(term);
        match writer.b.node(head) {
            CoreNode::Const { n, .. } => n
                .strip_prefix("LexLeanTarget.RustSyntax.Item.")
                .map(|name| (name.to_owned(), arguments.len())),
            _ => None,
        }
    })
}

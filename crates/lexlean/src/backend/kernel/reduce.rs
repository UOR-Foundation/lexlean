//! Unfolding a statement as `simp only [definitions]` does, at the level of
//! the linked IR, so that a term a proof splits on or rewrites with appears
//! in it syntactically (SPEC.md §18.10). Nothing here is trusted: a
//! producer states the unfolded term only as the motive of a rewrite, and
//! the kernel checks that the motive at the original term is definitionally
//! the statement.
//!
//! The reduction works on the IR as JSON, so that one capture-avoiding
//! traversal serves every binding form (`forall`, `let`, `fun`, and a match
//! branch's binders). It unfolds a listed definition at a call, a recursive
//! one only when its recursive argument is a constructor or a numeral;
//! reduces a `match` on a constructor, an `if` on a Boolean literal, a `let`,
//! a projection of a record or a pair, and Boolean and natural-number
//! operations on literals. It stops after a fixed number of steps.

use std::cell::Cell;

use serde_json::{json, Map, Value};

use super::lower::World;
use crate::ir::semantic::{MemberRef, SemanticDeclaration, SemanticTerm};

/// The most unfoldings one reduction performs.
const BUDGET: usize = 20_000;

/// A reduction of the terms of one module, unfolding exactly `unfold`.
pub struct Reducer<'a> {
    world: &'a World<'a>,
    module: usize,
    unfold: Vec<(usize, String)>,
    steps: Cell<usize>,
    fresh: Cell<usize>,
}

fn kind(value: &Value) -> &str {
    value.get("kind").and_then(Value::as_str).unwrap_or_default()
}

fn name_of(value: &Value) -> Option<&str> {
    value.get("name").and_then(Value::as_str)
}

fn nat(value: &Value) -> Option<u128> {
    (kind(value) == "nat")
        .then(|| value.get("value").and_then(Value::as_str))
        .flatten()
        .and_then(|digits| digits.parse().ok())
}

fn boolean(value: &Value) -> Option<bool> {
    (kind(value) == "bool")
        .then(|| value.get("value").and_then(Value::as_bool))
        .flatten()
}

/// The last component of a constructor's name.
fn short(constructor: &Value) -> String {
    constructor
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .rsplit('.')
        .next()
        .unwrap_or_default()
        .to_owned()
}

/// A term with variable `from` replaced by `by`, renaming every binder the
/// replacement would be captured by.
fn substitute(term: &Value, from: &str, by: &Value, fresh: &Cell<usize>) -> Value {
    let free = free_vars(by);
    let mut rename = |binder: &str| -> String {
        if free.iter().any(|name| name == binder) {
            let n = fresh.get();
            fresh.set(n + 1);
            format!("{binder}__r{n}")
        } else {
            binder.to_owned()
        }
    };
    match term {
        Value::Array(items) => Value::Array(items.iter().map(|item| substitute(item, from, by, fresh)).collect()),
        Value::Object(map) => {
            match kind(term) {
                "var" if name_of(term) == Some(from) => return by.clone(),
                "forall" | "let" => {
                    let binder = map["binder"]["name"].as_str().unwrap_or_default().to_owned();
                    let mut out = map.clone();
                    if let Some(value) = map.get("value") {
                        out.insert("value".to_owned(), substitute(value, from, by, fresh));
                    }
                    if binder == from {
                        return Value::Object(out);
                    }
                    let renamed = rename(&binder);
                    let mut body = map["body"].clone();
                    if renamed != binder {
                        body = substitute(&body, &binder, &json!({"kind": "var", "name": renamed}), fresh);
                        out["binder"]["name"] = Value::String(renamed);
                    }
                    out.insert("body".to_owned(), substitute(&body, from, by, fresh));
                    return Value::Object(out);
                }
                "lambda" => {
                    let names: Vec<String> = map["parameters"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .filter_map(|p| p["name"].as_str().map(str::to_owned))
                        .collect();
                    if names.iter().any(|n| n == from) {
                        return term.clone();
                    }
                    let mut out = map.clone();
                    let mut body = map["body"].clone();
                    for (index, name) in names.iter().enumerate() {
                        let renamed = rename(name);
                        if renamed != *name {
                            body = substitute(&body, name, &json!({"kind": "var", "name": renamed}), fresh);
                            out["parameters"][index]["name"] = Value::String(renamed);
                        }
                    }
                    out.insert("body".to_owned(), substitute(&body, from, by, fresh));
                    out.insert("captures".to_owned(), json!([]));
                    return Value::Object(out);
                }
                "match" => {
                    let mut out = map.clone();
                    out.insert("scrutinee".to_owned(), substitute(&map["scrutinee"], from, by, fresh));
                    let mut branches = Vec::new();
                    for branch in map["branches"].as_array().into_iter().flatten() {
                        let mut branch = branch.clone();
                        let binders: Vec<String> = branch["binders"]
                            .as_array()
                            .into_iter()
                            .flatten()
                            .filter_map(|b| b.as_str().map(str::to_owned))
                            .collect();
                        if !binders.iter().any(|b| b == from) {
                            let mut body = branch["body"].clone();
                            for (index, binder) in binders.iter().enumerate() {
                                let renamed = rename(binder);
                                if renamed != *binder {
                                    body = substitute(&body, binder, &json!({"kind": "var", "name": renamed}), fresh);
                                    branch["binders"][index] = Value::String(renamed);
                                }
                            }
                            branch["body"] = substitute(&body, from, by, fresh);
                        }
                        branches.push(branch);
                    }
                    out.insert("branches".to_owned(), Value::Array(branches));
                    return Value::Object(out);
                }
                _ => {}
            }
            Value::Object(
                map.iter()
                    .map(|(key, value)| (key.clone(), substitute(value, from, by, fresh)))
                    .collect::<Map<String, Value>>(),
            )
        }
        other => other.clone(),
    }
}

/// The variables a term names free, in order of first mention.
pub fn free_vars(term: &Value) -> Vec<String> {
    fn go(term: &Value, bound: &mut Vec<String>, out: &mut Vec<String>) {
        match term {
            Value::Array(items) => items.iter().for_each(|item| go(item, bound, out)),
            Value::Object(map) => match kind(term) {
                "var" => {
                    if let Some(name) = name_of(term) {
                        if !bound.iter().any(|b| b == name) && !out.iter().any(|o| o == name) {
                            out.push(name.to_owned());
                        }
                    }
                }
                "forall" | "let" => {
                    if let Some(value) = map.get("value") {
                        go(value, bound, out);
                    }
                    bound.push(map["binder"]["name"].as_str().unwrap_or_default().to_owned());
                    go(&map["body"], bound, out);
                    bound.pop();
                }
                "lambda" => {
                    let names: Vec<String> = map["parameters"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .filter_map(|p| p["name"].as_str().map(str::to_owned))
                        .collect();
                    let depth = bound.len();
                    bound.extend(names);
                    go(&map["body"], bound, out);
                    bound.truncate(depth);
                }
                "match" => {
                    go(&map["scrutinee"], bound, out);
                    for branch in map["branches"].as_array().into_iter().flatten() {
                        let depth = bound.len();
                        bound.extend(
                            branch["binders"]
                                .as_array()
                                .into_iter()
                                .flatten()
                                .filter_map(|b| b.as_str().map(str::to_owned)),
                        );
                        go(&branch["body"], bound, out);
                        bound.truncate(depth);
                    }
                }
                _ => map.values().for_each(|value| go(value, bound, out)),
            },
            _ => {}
        }
    }
    let mut out = Vec::new();
    go(term, &mut Vec::new(), &mut out);
    out
}

/// A constructor application or literal as `(constructor short name,
/// fields)`, when the term is one.
fn constructor_of(term: &Value) -> Option<(String, Vec<Value>)> {
    match kind(term) {
        "constructor" => Some((
            short(&term["constructor"]),
            term["arguments"].as_array().cloned().unwrap_or_default(),
        )),
        "bool" => Some((if boolean(term)? { "true" } else { "false" }.to_owned(), Vec::new())),
        "nat" => {
            let n = nat(term)?;
            Some(if n == 0 {
                ("zero".to_owned(), Vec::new())
            } else {
                ("succ".to_owned(), vec![json!({"kind": "nat", "value": (n - 1).to_string()})])
            })
        }
        "nil" => Some(("nil".to_owned(), Vec::new())),
        "cons" => Some(("cons".to_owned(), vec![term["head"].clone(), term["tail"].clone()])),
        "record" => Some((
            "mk".to_owned(),
            term["fields"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|row| row["value"].clone())
                .collect(),
        )),
        "pair" => Some(("mk".to_owned(), vec![term["left"].clone(), term["right"].clone()])),
        _ => None,
    }
}

impl<'a> Reducer<'a> {
    /// A reducer unfolding `definitions`, read in module `module`.
    ///
    /// # Errors
    ///
    /// A definition that is not one of the project.
    pub fn new(world: &'a World<'a>, module: usize, definitions: &[MemberRef]) -> Result<Self, String> {
        let mut unfold = Vec::new();
        for member in definitions {
            let (at, declaration) = world.find(module, member)?;
            unfold.push((at, declaration.name().to_owned()));
        }
        Ok(Self {
            world,
            module,
            unfold,
            steps: Cell::new(0),
            fresh: Cell::new(0),
        })
    }

    fn tick(&self) -> bool {
        let n = self.steps.get();
        self.steps.set(n + 1);
        n < BUDGET
    }

    /// The definition a call names, when it is to be unfolded.
    fn definition(&self, function: &Value, at: usize) -> Option<(usize, &'a SemanticDeclaration)> {
        let member: MemberRef = serde_json::from_value(function.clone()).ok()?;
        let (module, declaration) = self.world.find(at, &member).ok()?;
        self.unfold
            .iter()
            .any(|(m, n)| *m == module && n == declaration.name())
            .then_some((module, declaration))
    }

    /// The term reduced as far as the rules go.
    pub fn reduce(&self, term: &Value) -> Value {
        self.reduce_in(term, self.module)
    }

    #[allow(clippy::too_many_lines)]
    fn reduce_in(&self, term: &Value, at: usize) -> Value {
        if !self.tick() {
            return term.clone();
        }
        let Value::Object(map) = term else {
            return term.clone();
        };
        // Children first; binders are left in place.
        let mut reduced = Map::new();
        for (key, value) in map {
            let child = match value {
                Value::Object(_) if key != "type" && key != "result" && key != "function" && key != "constructor" && key != "element" && key != "binder" => self.reduce_in(value, at),
                Value::Array(items) if key == "arguments" || key == "fields" || key == "branches" || key == "elements" => Value::Array(
                    items
                        .iter()
                        .map(|item| {
                            if key == "fields" {
                                let mut row = item.clone();
                                row["value"] = self.reduce_in(&item["value"], at);
                                row
                            } else if key == "branches" {
                                let mut branch = item.clone();
                                branch["body"] = self.reduce_in(&item["body"], at);
                                branch
                            } else {
                                self.reduce_in(item, at)
                            }
                        })
                        .collect(),
                ),
                other => other.clone(),
            };
            reduced.insert(key.clone(), child);
        }
        let term = Value::Object(reduced);
        match kind(&term) {
            "call" => {
                let Some((module, declaration)) = self.definition(&term["function"], at) else {
                    return term;
                };
                let SemanticDeclaration::Definition {
                    type_parameters,
                    parameters,
                    body,
                    recursive_argument,
                    ..
                } = declaration
                else {
                    return term;
                };
                let arguments = term["arguments"].as_array().cloned().unwrap_or_default();
                if parameters.len() != arguments.len() {
                    return term;
                }
                if let Some(recursive) = recursive_argument {
                    let position = parameters.iter().position(|p| p.name == *recursive);
                    if position.and_then(|p| constructor_of(&arguments[p])).is_none() {
                        return term;
                    }
                }
                let mut unfolded = serde_json::to_value(body).unwrap_or(Value::Null);
                // Simultaneous substitution through fresh names first.
                let mut temporaries = Vec::new();
                for parameter in parameters {
                    let n = self.fresh.get();
                    self.fresh.set(n + 1);
                    let temporary = format!("{}__p{n}", parameter.name);
                    unfolded = substitute(&unfolded, &parameter.name, &json!({"kind": "var", "name": temporary}), &self.fresh);
                    temporaries.push(temporary);
                }
                for (temporary, argument) in temporaries.iter().zip(&arguments) {
                    unfolded = substitute(&unfolded, temporary, argument, &self.fresh);
                }
                let type_arguments = term["type_arguments"].as_array().cloned().unwrap_or_default();
                if type_arguments.len() != type_parameters.len() {
                    return term;
                }
                for (parameter, argument) in type_parameters.iter().zip(&type_arguments) {
                    unfolded = replace_type(&unfolded, parameter, argument);
                }
                // The body is read in its own module.
                let unfolded = qualify(&unfolded, &self.world.modules[module].name, module == at);
                self.reduce_in(&unfolded, at)
            }
            "match" => {
                let Some((ctor, fields)) = constructor_of(&term["scrutinee"]) else {
                    return term;
                };
                let Some(branch) = term["branches"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .find(|branch| short(&branch["constructor"]) == ctor)
                else {
                    return term;
                };
                let binders: Vec<String> = branch["binders"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(|b| b.as_str().map(str::to_owned))
                    .collect();
                if binders.len() != fields.len() {
                    return term;
                }
                let mut body = branch["body"].clone();
                for (binder, field) in binders.iter().zip(&fields) {
                    body = substitute(&body, binder, field, &self.fresh);
                }
                self.reduce_in(&body, at)
            }
            "if" => match boolean(&term["condition"]) {
                Some(true) => term["then_value"].clone(),
                Some(false) => term["else_value"].clone(),
                None => term,
            },
            "let" => {
                let body = substitute(&term["body"], term["binder"]["name"].as_str().unwrap_or_default(), &term["value"], &self.fresh);
                self.reduce_in(&body, at)
            }
            "project" => {
                if kind(&term["value"]) == "record" {
                    let field = term["field"].as_str().unwrap_or_default();
                    if let Some(row) = term["value"]["fields"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .find(|row| row["field"] == field)
                    {
                        return row["value"].clone();
                    }
                }
                term
            }
            "first" | "second" if kind(&term["value"]) == "pair" => {
                term["value"][if kind(&term) == "first" { "left" } else { "right" }].clone()
            }
            "add" => match (nat(&term["left"]), nat(&term["right"])) {
                (Some(l), Some(r)) => json!({"kind": "nat", "value": (l + r).to_string()}),
                _ => term,
            },
            "beq" | "ble" | "blt" => match (nat(&term["left"]), nat(&term["right"])) {
                (Some(l), Some(r)) => json!({"kind": "bool", "value": match kind(&term) { "beq" => l == r, "ble" => l <= r, _ => l < r }}),
                _ => term,
            },
            "and" | "or" => match (boolean(&term["left"]), boolean(&term["right"])) {
                (Some(l), Some(r)) => json!({"kind": "bool", "value": if kind(&term) == "and" { l && r } else { l || r }}),
                _ => term,
            },
            "not" => boolean(&term["value"]).map_or(term.clone(), |b| json!({"kind": "bool", "value": !b})),
            _ => term,
        }
    }
}

/// A body with the type parameter `name` replaced by the type `by`.
fn replace_type(term: &Value, name: &str, by: &Value) -> Value {
    match term {
        Value::Array(items) => Value::Array(items.iter().map(|item| replace_type(item, name, by)).collect()),
        Value::Object(map) => {
            if kind(term) == "parameter" && name_of(term) == Some(name) {
                return by.clone();
            }
            Value::Object(map.iter().map(|(key, value)| (key.clone(), replace_type(value, name, by))).collect())
        }
        other => other.clone(),
    }
}

/// A body read in its own module, made readable in the caller's: a
/// document member with no module gains the body's module.
fn qualify(term: &Value, module: &str, same: bool) -> Value {
    if same {
        return term.clone();
    }
    match term {
        Value::Array(items) => Value::Array(items.iter().map(|item| qualify(item, module, same)).collect()),
        Value::Object(map) => {
            let mut out = Map::new();
            for (key, value) in map {
                let qualified = if matches!(key.as_str(), "function" | "constructor" | "type" | "class" | "resolved" | "member")
                    && value.is_object()
                    && value.get("module").is_none()
                    && value.get("name").is_some()
                    && value.get("kind").is_none()
                    && !is_builtin(value)
                {
                    let mut member = value.clone();
                    member["module"] = Value::String(module.to_owned());
                    member
                } else {
                    qualify(value, module, same)
                };
                out.insert(key.clone(), qualified);
            }
            Value::Object(out)
        }
        other => other.clone(),
    }
}

fn is_builtin(member: &Value) -> bool {
    member
        .get("name")
        .and_then(Value::as_str)
        .and_then(|name| name.split('.').next())
        .is_some_and(|owner| crate::ir::semantic::BUILTIN_CONSTRUCTOR_OWNERS.contains(&owner))
}

/// `term` with every occurrence of `target` replaced by the variable `name`.
#[must_use]
pub fn abstract_term(term: &Value, target: &Value, name: &str) -> (Value, usize) {
    if term == target {
        return (json!({"kind": "var", "name": name}), 1);
    }
    match term {
        Value::Array(items) => {
            let mut count = 0;
            let items = items
                .iter()
                .map(|item| {
                    let (out, n) = abstract_term(item, target, name);
                    count += n;
                    out
                })
                .collect();
            (Value::Array(items), count)
        }
        Value::Object(map) => {
            let mut count = 0;
            let map = map
                .iter()
                .map(|(key, value)| {
                    let (out, n) = abstract_term(value, target, name);
                    count += n;
                    (key.clone(), out)
                })
                .collect();
            (Value::Object(map), count)
        }
        other => (other.clone(), 0),
    }
}

/// A JSON term as an IR term.
///
/// # Errors
///
/// A term reduction made that is not one the IR states.
pub fn term_of(value: &Value) -> Result<SemanticTerm, String> {
    serde_json::from_value(value.clone()).map_err(|error| format!("a reduced term: {error}"))
}

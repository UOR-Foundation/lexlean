//! lean4export 3.1.0 NDJSON: reading a library export's tables and writing
//! lines that continue its numbering, so the produced declarations are a
//! suffix that a checker holding the library resident judges alone.
//!
//! Every object is written with its keys sorted, as lean4export writes them,
//! and a name, level, or expression the export already states is reused by
//! its number rather than stated twice.

use std::collections::{BTreeSet, HashMap};

use serde_json::{json, Value};

use super::expr::{Expr, Level};

/// The tables of an export and the lines appended after it.
#[derive(Clone)]
pub struct Tables {
    names: HashMap<String, u64>,
    next_name: u64,
    levels: HashMap<String, u64>,
    next_level: u64,
    exprs: HashMap<String, u64>,
    next_expr: u64,
    declared: BTreeSet<String>,
    lines: Vec<String>,
}

fn field<'a>(object: &'a Value, key: &str) -> Result<&'a Value, String> {
    object
        .get(key)
        .ok_or_else(|| format!("an export line lacks `{key}`: {object}"))
}

fn number(value: &Value) -> Result<u64, String> {
    value
        .as_u64()
        .ok_or_else(|| format!("an export index is not a number: {value}"))
}

impl Tables {
    /// Reads an export's tables.
    ///
    /// # Errors
    ///
    /// Returns the first malformed line.
    pub fn parse(text: &str) -> Result<Self, String> {
        let mut names: HashMap<u64, String> = HashMap::from([(0, String::new())]);
        let mut tables = Self {
            names: HashMap::new(),
            next_name: 1,
            levels: HashMap::new(),
            next_level: 1,
            exprs: HashMap::new(),
            next_expr: 0,
            declared: BTreeSet::new(),
            lines: Vec::new(),
        };
        for line in text.lines().filter(|line| !line.is_empty()) {
            let mut object: Value =
                serde_json::from_str(line).map_err(|error| format!("an export line: {error}"))?;
            if let Some(id) = object.get("in") {
                let id = number(id)?;
                let (part, pre) = if let Some(s) = object.get("str") {
                    (
                        field(s, "str")?.as_str().unwrap_or_default().to_owned(),
                        number(field(s, "pre")?)?,
                    )
                } else {
                    let n = field(&object, "num")?;
                    (
                        number(field(n, "i")?)?.to_string(),
                        number(field(n, "pre")?)?,
                    )
                };
                let prefix = names.get(&pre).cloned().unwrap_or_default();
                let full = if prefix.is_empty() {
                    part
                } else {
                    format!("{prefix}.{part}")
                };
                tables.names.insert(full.clone(), id);
                names.insert(id, full);
                tables.next_name = tables.next_name.max(id + 1);
            } else if let Some(id) = object.get("il").cloned() {
                let id = number(&id)?;
                if let Some(map) = object.as_object_mut() {
                    map.remove("il");
                }
                tables.levels.insert(object.to_string(), id);
                tables.next_level = tables.next_level.max(id + 1);
            } else if let Some(id) = object.get("ie").cloned() {
                let id = number(&id)?;
                if let Some(map) = object.as_object_mut() {
                    map.remove("ie");
                }
                tables.exprs.insert(object.to_string(), id);
                tables.next_expr = tables.next_expr.max(id + 1);
            } else if object.get("meta").is_none() {
                let record = object
                    .as_object()
                    .and_then(|map| map.values().next())
                    .ok_or("an export declaration line is empty")?;
                let mut add = |row: &Value| -> Result<(), String> {
                    let id = number(field(row, "name")?)?;
                    tables
                        .declared
                        .insert(names.get(&id).cloned().unwrap_or_default());
                    Ok(())
                };
                if object.get("inductive").is_some() {
                    for key in ["types", "ctors", "recs"] {
                        for row in field(record, key)?.as_array().into_iter().flatten() {
                            add(row)?;
                        }
                    }
                } else if object.get("quot").is_some() {
                    for name in ["Quot", "Quot.mk", "Quot.lift", "Quot.ind"] {
                        tables.declared.insert(name.to_owned());
                    }
                } else {
                    add(record)?;
                }
            }
        }
        Ok(tables)
    }

    /// Whether the export, or a line appended since, declares `name`.
    #[must_use]
    pub fn declares(&self, name: &str) -> bool {
        self.declared.contains(name)
    }

    fn push(&mut self, value: &Value) {
        self.lines.push(value.to_string());
    }

    /// The number of a name, stating it and its prefixes when new.
    pub fn name(&mut self, full: &str) -> u64 {
        if full.is_empty() {
            return 0;
        }
        if let Some(id) = self.names.get(full) {
            return *id;
        }
        let (pre, part) = match full.rfind('.') {
            Some(dot) => (self.name(&full[..dot]), &full[dot + 1..]),
            None => (0, full),
        };
        let id = self.next_name;
        self.next_name += 1;
        let line = match part.parse::<u64>() {
            Ok(i) if !part.starts_with('0') || part == "0" => {
                json!({"in": id, "num": {"i": i, "pre": pre}})
            }
            _ => json!({"in": id, "str": {"pre": pre, "str": part}}),
        };
        self.push(&line);
        self.names.insert(full.to_owned(), id);
        id
    }

    /// The number of a level.
    pub fn level(&mut self, level: &Level) -> u64 {
        match level {
            Level::Zero => 0,
            Level::Succ(inner) => {
                let inner = self.level(inner);
                let body = json!({"succ": inner});
                let key = body.to_string();
                if let Some(id) = self.levels.get(&key) {
                    return *id;
                }
                let id = self.next_level;
                self.next_level += 1;
                self.push(&json!({"il": id, "succ": inner}));
                self.levels.insert(key, id);
                id
            }
        }
    }

    fn intern(&mut self, body: Value) -> u64 {
        let key = body.to_string();
        if let Some(id) = self.exprs.get(&key) {
            return *id;
        }
        let id = self.next_expr;
        self.next_expr += 1;
        let mut line = body;
        if let Some(map) = line.as_object_mut() {
            map.insert("ie".to_owned(), json!(id));
        }
        self.push(&line);
        self.exprs.insert(key, id);
        id
    }

    /// The number of a closed expression.
    ///
    /// # Errors
    ///
    /// Refuses an expression with a free variable: a producer left a binder
    /// open.
    pub fn expr(&mut self, expr: &Expr) -> Result<u64, String> {
        let body = match expr {
            Expr::BVar(index) => json!({"bvar": index}),
            Expr::FVar(id) => return Err(format!("a produced term has free variable {id}")),
            Expr::Sort(level) => json!({"sort": self.level(level)}),
            Expr::Const(name, levels) => {
                let name = self.name(name);
                let us: Vec<u64> = levels.iter().map(|level| self.level(level)).collect();
                json!({"const": {"name": name, "us": us}})
            }
            Expr::App(f, a) => {
                let f = self.expr(f)?;
                let a = self.expr(a)?;
                json!({"app": {"arg": a, "fn": f}})
            }
            Expr::Lam(name, ty, body) | Expr::Pi(name, ty, body) => {
                let key = if matches!(expr, Expr::Lam(..)) {
                    "lam"
                } else {
                    "forallE"
                };
                let ty = self.expr(ty)?;
                let body = self.expr(body)?;
                let name = self.name(name);
                json!({key: {"binderInfo": "default", "body": body, "name": name, "type": ty}})
            }
            Expr::Let(name, ty, value, body) => {
                let ty = self.expr(ty)?;
                let value = self.expr(value)?;
                let body = self.expr(body)?;
                let name = self.name(name);
                json!({"letE": {"body": body, "name": name, "nondep": false, "type": ty, "value": value}})
            }
            Expr::Nat(value) => json!({"natVal": value}),
            Expr::Str(value) => json!({"strVal": value}),
        };
        Ok(self.intern(body))
    }

    /// Appends a theorem declaration.
    ///
    /// # Errors
    ///
    /// Refuses a name already declared, or an open term.
    pub fn theorem(&mut self, name: &str, ty: &Expr, value: &Expr) -> Result<(), String> {
        if self.declared.contains(name) {
            return Err(format!("`{name}` is already declared"));
        }
        let ty = self.expr(ty)?;
        let value = self.expr(value)?;
        let id = self.name(name);
        self.push(&json!({"thm": {"all": [id], "levelParams": [], "name": id, "type": ty, "value": value}}));
        self.declared.insert(name.to_owned());
        Ok(())
    }

    /// The lines appended after the export, each ending in a newline.
    #[must_use]
    pub fn suffix(&self) -> String {
        let mut out = String::new();
        for line in &self.lines {
            out.push_str(line);
            out.push('\n');
        }
        out
    }

    /// Discards the lines appended since `mark`, for a theorem whose
    /// production failed after it began writing.
    pub fn rewind(&mut self, mark: &Checkpoint) {
        self.lines.truncate(mark.lines);
        self.names.retain(|_, id| *id < mark.next_name);
        self.levels.retain(|_, id| *id < mark.next_level);
        self.exprs.retain(|_, id| *id < mark.next_expr);
        self.next_name = mark.next_name;
        self.next_level = mark.next_level;
        self.next_expr = mark.next_expr;
    }

    /// A point to rewind to.
    #[must_use]
    pub fn checkpoint(&self) -> Checkpoint {
        Checkpoint {
            lines: self.lines.len(),
            next_name: self.next_name,
            next_level: self.next_level,
            next_expr: self.next_expr,
        }
    }
}

/// The table state at a point of writing.
pub struct Checkpoint {
    lines: usize,
    next_name: u64,
    next_level: u64,
    next_expr: u64,
}

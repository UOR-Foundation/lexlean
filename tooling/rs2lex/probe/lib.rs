use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::sync::OnceLock;

pub fn run_nat(ops: Vec<(usize, usize, String)>) -> (Vec<Option<String>>, Vec<(usize, String)>) {
    let mut table: BTreeMap<usize, String> = BTreeMap::new();
    let mut seen = Vec::new();
    for (code, key, value) in ops {
        seen.push(table.get(&key).cloned());
        if code == 0 {
            table.insert(key, value);
        } else if code == 1 {
            table.remove(&key);
        }
    }
    (seen, table.into_iter().collect())
}

pub fn run_pair(ops: Vec<(usize, usize, usize, String)>) -> (Vec<Option<String>>, Vec<((usize, usize), String)>) {
    let mut table: BTreeMap<(usize, usize), String> = BTreeMap::new();
    let mut seen = Vec::new();
    for (code, a, b, value) in ops {
        seen.push(table.get(&(a, b)).cloned());
        if code == 0 {
            table.insert((a, b), value);
        } else if code == 1 {
            table.remove(&(a, b));
        }
    }
    (seen, table.into_iter().collect())
}

pub fn run_tail(ops: Vec<(usize, usize, String, String)>) -> (Vec<Option<String>>, Vec<((usize, String), String)>) {
    let mut table: BTreeMap<(usize, String), String> = BTreeMap::new();
    let mut seen = Vec::new();
    for (code, a, b, value) in ops {
        seen.push(table.get(&(a, b.clone())).cloned());
        if code == 0 {
            table.insert((a, b), value);
        } else if code == 1 {
            table.remove(&(a, b));
        }
    }
    (seen, table.into_iter().collect())
}

fn visit(xs: &[usize], depth: usize, f: &mut impl FnMut(usize)) {
    for x in xs {
        f(*x + depth);
    }
    if depth < 3 {
        visit(xs, depth + 1, f);
    }
}

pub fn count_over(xs: Vec<usize>, limit: usize) -> (usize, Vec<usize>) {
    let mut over = 0;
    let mut seen = Vec::new();
    visit(&xs, 0, &mut |v| {
        if v > limit {
            over += 1;
        }
        seen.push(v);
    });
    (over, seen)
}

pub fn combos(xs: Vec<usize>) -> (usize, Vec<usize>) {
    let mut count = 0;
    let mut out = Vec::new();
    for x in xs {
        let even: Option<usize> = if x % 2 == 0 { Some(x) } else { None };
        let y = even
            .map(|v| {
                count += 1;
                v * 2
            })
            .unwrap_or_else(|| {
                count += 10;
                x
            });
        let big: Result<usize, usize> = if x > 5 { Ok(y) } else { Err(y) };
        let z = match big.map_err(|e| {
            count += 100;
            e + 1
        }) {
            Ok(a) => a,
            Err(b) => b,
        };
        out.push(z);
    }
    (count, out)
}

pub fn ordering_probe(xs: Vec<(usize, String)>) -> (Vec<(usize, String)>, Option<(usize, String)>, Option<(usize, String)>, Vec<(usize, String)>) {
    let mut stable = xs.clone();
    stable.sort_by(|p, q| p.0.cmp(&q.0));
    let most = xs.iter().cloned().max_by_key(|p| p.0);
    let least = xs.iter().cloned().min_by_key(|p| p.0);
    let mut both = xs.clone();
    both.sort_by(|p, q| q.0.cmp(&p.0).then_with(|| p.1.cmp(&q.1)));
    (stable, most, least, both)
}

pub fn slices(xs: Vec<usize>, wrapped: Option<Vec<usize>>) -> (usize, usize, usize, usize) {
    let shape = match xs.as_slice() {
        [] => 0,
        [x] => 100 + x,
        [x, y] if x > y => 200 + x - y,
        [x, y, rest @ ..] => 300 + x + y + rest.len(),
        _ => 999,
    };
    let last = match xs.as_slice() {
        [.., last] => *last,
        [] => 7,
    };
    let inner = match &wrapped {
        Some(v) => match v.as_slice() {
            [a] => *a,
            [a, b, ..] => a + b,
            [] => 1,
        },
        None => 2,
    };
    let triple = [xs.len(), last, inner];
    let [p, q, r] = triple;
    (shape, last, inner, p + q * 10 + r * 100)
}

pub fn char_probe(s: String) -> (Vec<usize>, usize, usize, String, bool) {
    let starts: Vec<usize> = s.char_indices().map(|(i, _)| i).collect();
    let white = s.chars().filter(|c| c.is_whitespace()).count();
    let alpha = s.chars().filter(|c| c.is_ascii_alphabetic()).count();
    let mut masked = String::new();
    for c in s.chars() {
        if c.is_ascii_digit() {
            masked.push('#');
        } else {
            masked.push(c);
        }
    }
    let upper = match s.as_bytes().first() {
        Some(b) => *b == b'Q' || b.is_ascii_uppercase(),
        None => false,
    };
    (starts, white, alpha, masked, upper)
}

pub struct Counter {
    log: std::cell::RefCell<Vec<usize>>,
    total: std::cell::Cell<usize>,
}

impl Counter {
    fn note(&self, x: usize) {
        let mut log = self.log.borrow_mut();
        log.push(x);
        if x > 3 {
            log.push(x * 2);
        }
        self.total.set(self.total.get() + x);
    }
}

pub fn alias_probe(xs: Vec<usize>) -> (Vec<usize>, usize) {
    let counter = Counter { log: std::cell::RefCell::new(Vec::new()), total: std::cell::Cell::new(0) };
    for x in xs {
        counter.note(x);
    }
    let total = counter.total.get();
    let log = counter.log.borrow().clone();
    (log, total)
}

pub fn eager_probe(xs: Vec<usize>, limit: usize) -> (Option<usize>, Vec<usize>, Vec<usize>, Option<usize>, usize) {
    let mut seen = 0;
    let first = xs.iter().find_map(|x| {
        seen += 1;
        if *x > limit { Some(*x * 2) } else { None }
    });
    let mut kept = 0;
    let evens: Vec<usize> = xs.iter().copied().filter(|x| {
        kept += 1;
        x % 2 == 0
    }).collect();
    let mut total = 0;
    let halves: Vec<usize> = xs.iter().filter_map(|x| {
        total += *x;
        if *x % 2 == 0 { Some(*x / 2) } else { None }
    }).collect();
    let mut bumped = 0;
    let then = (xs.len() > 2).then(|| {
        bumped += 7;
        xs.len()
    });
    (first, evens, halves, then, seen * 1000000 + kept * 10000 + total * 100 + bumped)
}

pub fn named_probe(xs: Vec<usize>) -> (usize, usize, Option<usize>, usize, bool, usize) {
    let mut count = 0;
    let mut bump = |x: usize| {
        count += x;
    };
    bump(1);
    for x in xs.iter() {
        bump(*x);
    }
    let pick = |x: usize| -> usize {
        if x > 3 {
            return 100;
        }
        x
    };
    let mut picked = 0;
    for x in xs.iter() {
        picked += pick(*x);
    }
    let mut looked = 0;
    let found = xs.iter().copied().find(|x| {
        looked += 1;
        *x > 4
    });
    let mut steps = 0;
    let folded = xs.iter().fold(0, |acc, x| {
        steps += 1;
        if *x == 0 {
            return acc;
        }
        acc * 2 + *x
    });
    let small = xs.iter().all(|x| {
        if *x > 7 {
            return false;
        }
        *x != 5
    });
    let first_big: Result<usize, usize> = xs.first().copied().ok_or(0).and_then(|x| {
        if x > 5 {
            return Err(x);
        }
        Ok(x + 1)
    });
    let coded = match first_big {
        Ok(v) => v,
        Err(e) => 1000 + e,
    };
    (count, picked, found, folded * 1000 + steps * 100 + looked, small, coded)
}

pub fn shadow_probe(xs: Vec<usize>, wrapped: Option<Vec<usize>>) -> (usize, Vec<usize>, usize, usize) {
    let mut v = xs.clone();
    let mut n = 0;
    if xs.len() > 1 {
        let mut v = Vec::new();
        v.push(9);
        n = v.len();
    }
    let mut total = 0;
    for x in xs.iter() {
        let mut total = *x;
        total += 1;
        n += total;
    }
    total += v.len();
    v.push(n);
    let mut wrapped = wrapped;
    let inner = if let Some(mut wrapped) = wrapped.clone() {
        wrapped.push(1);
        wrapped.len()
    } else {
        0
    };
    n += inner;
    let n = n + 1;
    let shown = format!("{n}{total}");
    (n, v, total + wrapped.map_or(0, |w| w.len()), shown.len())
}

pub fn index_probe(keys: Vec<String>, at: usize) -> (usize, bool, bool) {
    let mut table: BTreeMap<String, usize> = BTreeMap::new();
    for (i, key) in keys.iter().enumerate() {
        table.insert(key.clone(), i);
    }
    let first = if keys.is_empty() { 0 } else { table[&keys[at % keys.len()]] };
    let sorted = keys.windows(2).all(|pair| pair[0] <= pair[1]);
    let clash = keys.windows(2).any(|pair| pair[0] >= pair[1]);
    (first, sorted, clash)
}

pub fn entry_probe(words: Vec<String>) -> (Vec<(String, usize)>, Vec<(String, Vec<usize>)>, usize) {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut places: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    let mut firsts: BTreeMap<String, usize> = BTreeMap::new();
    for (i, word) in words.iter().enumerate() {
        *counts.entry(word.clone()).or_insert(0) += 1;
        places.entry(word.clone()).or_default().push(i);
        firsts.entry(word.clone()).or_insert(i);
    }
    let total = firsts.values().copied().sum::<usize>();
    (counts.into_iter().collect(), places.into_iter().collect(), total)
}

pub fn map_probe(pairs: Vec<(String, usize)>, more: Vec<(String, usize)>, ask: String) -> (Vec<(String, usize)>, Option<(String, usize)>) {
    let mut table: BTreeMap<String, usize> = BTreeMap::new();
    for (key, value) in pairs.iter() {
        table.insert(key.clone(), *value);
    }
    table.extend(more.iter().map(|(key, value)| (key.clone(), *value + 100)));
    let found = table.get_key_value(&ask).map(|(key, value)| (key.clone(), *value));
    (table.into_iter().collect(), found)
}

fn write_all(out: &mut String, parts: &[String]) -> std::fmt::Result {
    use std::fmt::Write;
    for part in parts {
        out.write_str(part)?;
        out.write_char('|')?;
    }
    Ok(())
}

pub fn write_probe(parts: Vec<String>) -> (String, bool) {
    let mut out = String::from(">");
    let ok = write_all(&mut out, &parts).is_ok();
    (out, ok)
}

pub enum Shape {
    Leaf(usize),
    Pair(Box<Shape>, Box<Shape>),
    Many(Vec<Shape>),
}

impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Leaf(n) => write!(f, "{n}"),
            Shape::Pair(a, b) => write!(f, "({a} {b})"),
            Shape::Many(items) => {
                f.write_str("[")?;
                for item in items {
                    write!(f, "{item};")?;
                }
                f.write_str("]")
            }
        }
    }
}

fn shape_of(codes: &[usize], depth: usize) -> Shape {
    match codes {
        [] => Shape::Leaf(depth),
        [x] => Shape::Leaf(*x),
        [x, rest @ ..] => {
            if x % 3 == 0 && depth < 4 {
                Shape::Pair(Box::new(Shape::Leaf(*x)), Box::new(shape_of(rest, depth + 1)))
            } else if x % 3 == 1 && depth < 4 {
                Shape::Many(vec![shape_of(rest, depth + 1), Shape::Leaf(*x)])
            } else {
                shape_of(rest, depth + 1)
            }
        }
    }
}

pub fn display_probe(codes: Vec<usize>) -> String {
    shape_of(&codes, 0).to_string()
}

fn grow(v: &mut Vec<usize>) -> Vec<usize> {
    v.push(v.len());
    v.clone()
}

/// `w` is made by a call that borrows `v` mutably, and is a value of its own: its type says so, where its syntax
/// does not. A write to it is no write to `v`. Only the front end from rustc's tree knows the type, so only it
/// translates this function (probe/run.py).
pub fn typed_probe(xs: Vec<usize>) -> (usize, Vec<usize>, Vec<usize>) {
    let mut v = xs;
    let mut w = grow(&mut v);
    w.push(7);
    let n = w.len() + v.len();
    (n, v, w)
}

fn early(v: &mut Vec<usize>, limit: usize) -> usize {
    for x in v.iter_mut() {
        if *x > limit {
            *x = limit;
            return 1;
        }
        *x += 10;
    }
    0
}

/// writes through a `&mut` to an element (writeback.py): a `for` over `&mut v` and over `iter_mut()`, with a `break`,
/// a `continue` and a `return` leaving the body after a write, and `last_mut()` in an `if let`, with and without the
/// element there.
pub fn writeback_probe(xs: Vec<usize>, stop: usize) -> (Vec<usize>, Vec<Vec<usize>>, usize, Vec<usize>, usize) {
    let mut v = xs.clone();
    let mut rows: Vec<Vec<usize>> = Vec::new();
    let mut seen = 0;
    for x in &mut v {
        if *x == stop {
            *x = 100;
            break;
        }
        if *x % 2 == 0 {
            *x += 1;
            continue;
        }
        *x = *x * 2;
        seen += 1;
    }
    for x in xs.iter() {
        if *x > 5 {
            rows.push(Vec::new());
        }
        if let Some(last) = rows.last_mut() {
            last.push(*x);
            if *x == 3 {
                continue;
            }
            last.push(0);
        } else {
            seen += 100;
        }
    }
    for row in rows.iter_mut() {
        row.push(seen);
    }
    let mut w = xs.clone();
    let hit = early(&mut w, stop);
    (v, rows, seen, w, hit)
}

/// a map extended by a hash map given whole: the hash map's order cannot be seen, since its keys are distinct and
/// the insertions commute; a later value replaces the one held.
pub fn extend_probe(ps: Vec<(String, usize)>, more: Vec<(String, usize)>) -> Vec<(String, usize)> {
    let mut base: BTreeMap<String, usize> = BTreeMap::new();
    for (k, v) in ps {
        base.insert(k, v);
    }
    let mut extra: HashMap<String, usize> = HashMap::new();
    for (k, v) in more {
        extra.insert(k, v);
    }
    base.extend(extra);
    base.into_iter().collect()
}

/// a collected `map` whose closure writes what it captures, inside another: each is a loop with a list of its own.
pub fn nested_collect_probe(rows: Vec<Vec<usize>>) -> (Vec<Vec<usize>>, usize) {
    let mut count = 0;
    let out: Vec<Vec<usize>> = rows
        .iter()
        .map(|row| {
            count += 1;
            row.iter()
                .map(|x| {
                    count += *x;
                    *x + count
                })
                .collect()
        })
        .collect();
    (out, count)
}

fn squares() -> &'static Vec<usize> {
    static TABLE: OnceLock<Vec<usize>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut made = Vec::new();
        for i in 0..5 {
            made.push(i * i);
        }
        made
    })
}

/// a value computed once (once.py): the closure reads nothing of its function, so the cell is the value.
pub fn once_probe(n: usize) -> usize {
    let table = squares();
    table[n % 5] + table.len()
}

pub struct Failure {
    pub code: usize,
    pub reason: String,
}

impl From<String> for Failure {
    fn from(reason: String) -> Self {
        Self { code: 7, reason }
    }
}

fn bounded(n: usize) -> Result<usize, String> {
    if n > 3 {
        Err(format!("big {n}"))
    } else {
        Ok(n + 1)
    }
}

fn doubled(n: usize) -> Result<usize, Failure> {
    let a = bounded(n)?;
    Ok(a * 2)
}

/// `?` on an error of another type: the error goes through the crate's own `From` impl.
pub fn convert_probe(n: usize) -> (usize, usize, String) {
    match doubled(n) {
        Ok(v) => (v, 0, String::new()),
        Err(f) => (0, f.code, f.reason),
    }
}

pub const LARGE: u64 = 1 << 32;

fn labelled(message: impl std::fmt::Display) -> String {
    format!("label: {message}")
}

/// a shift of two literals, in a constant and in an expression, and a parameter written `impl Display`, given a
/// string by one call and a number by another.
pub fn shift_probe(n: u64) -> (u64, String, String) {
    (n + (48 << 10) + LARGE, labelled("x"), labelled(n))
}

fn lengths(ps: &[(String, usize)]) -> BTreeMap<String, usize> {
    ps.iter().map(|p| (p.0.clone(), p.0.len())).collect()
}

/// pairs collected into a map, as a function's result and under a `let` with the map's type: a later pair replaces
/// an earlier one of its key.
pub fn collect_map_probe(ps: Vec<(String, usize)>) -> (Vec<(String, usize)>, usize) {
    let m: BTreeMap<String, usize> = ps.iter().map(|p| (p.0.clone(), p.1 + 1)).collect();
    let n = lengths(&ps).len();
    (m.into_iter().collect(), n)
}

/// a `char` cast to its code point (one to four bytes of UTF-8), a range used as an iterator (`rev`, `filter`,
/// `chain`), and an iterator of one item.
pub fn range_probe(s: String, n: usize) -> (Vec<u32>, Vec<usize>, Vec<usize>, Vec<usize>) {
    let codes: Vec<u32> = s.chars().map(|c| c as u32).collect();
    let down: Vec<usize> = (0..n).rev().collect();
    let tail: Vec<usize> = (1..=n).chain(std::iter::once(99)).collect();
    let evens: Vec<usize> = (0..n).filter(|x| x % 2 == 0).collect();
    (codes, down, tail, evens)
}

fn signed(negative: bool, magnitude: u64) -> i128 {
    if negative {
        -(magnitude as i128)
    } else {
        magnitude as i128
    }
}

/// arithmetic on i128, of either sign: sum, difference, product, quotient and remainder (both toward zero), the
/// Euclidean remainder, a shift, and the text of each.
pub fn wide_probe(na: bool, x: u64, nb: bool, y: u64, bits: u32) -> String {
    let a = signed(na, x);
    let b = signed(nb, y);
    let modulus = 1_i128 << bits;
    format!("{};{};{};{};{};{};{}", a + b, a - b, a * b, a / b, a % b, a.rem_euclid(b), modulus - a)
}

/// numbers in hexadecimal of either case and padded with zeros to a width, and a code point so written.
pub fn hex_probe(n: usize, s: String) -> String {
    let codes: Vec<String> = s.chars().map(|c| format!("\\u{:04x}", c as u32)).collect();
    format!("{:04X}|{:04x}|{n:02x}|{:x}|{n:03}|{}", n, n, n, codes.join(""))
}

pub enum Nest {
    Pi { binders: Vec<usize>, body: Box<Nest> },
    Leaf(usize),
}

fn nest(xs: &[usize], depth: usize) -> Nest {
    if depth == 0 {
        return Nest::Leaf(xs.len());
    }
    Nest::Pi { binders: xs.iter().map(|x| x + depth).collect(), body: Box::new(nest(xs, depth - 1)) }
}

fn flatten(ty: Nest) -> (Vec<usize>, usize) {
    let mut binders = Vec::new();
    let mut probe = ty;
    loop {
        match probe {
            Nest::Pi { binders: group, body } => {
                binders.extend(group);
                probe = *body;
            }
            Nest::Leaf(n) => return (binders, n),
        }
    }
}

#[derive(Default)]
pub struct Seen {
    pub seen: Vec<usize>,
}

fn noted(a: usize, b: usize, seen: &mut Seen) -> bool {
    seen.seen.push(a);
    seen.seen.len() == 1 && a == b
}

fn zipped(ps: &[String], args: &[usize]) -> Result<BTreeMap<String, usize>, String> {
    if ps.len() != args.len() {
        return Err(format!("{} and {}", ps.len(), args.len()));
    }
    Ok(ps.iter().cloned().zip(args.iter().cloned()).collect())
}

/// a `loop` left only by `return`, a write into a temporary the call drops (`&mut Seen::default()`), and
/// `Ok(pairs.collect())` into a map.
pub fn loop_probe(xs: Vec<usize>, depth: usize, names: Vec<String>) -> (Vec<usize>, usize, bool, Vec<(String, usize)>, String) {
    let (binders, leaf) = flatten(nest(&xs, depth));
    let same = noted(depth, depth, &mut Seen::default());
    match zipped(&names, &xs) {
        Ok(m) => (binders, leaf, same, m.into_iter().collect(), String::new()),
        Err(e) => (binders, leaf, same, Vec::new(), e),
    }
}

/// a set iterated: in its own order, whatever the order of insertion, for strings, numbers and pairs, by `for` and
/// by `iter()`.
pub fn set_probe(ws: Vec<String>, xs: Vec<usize>) -> (Vec<String>, Vec<usize>, Vec<(usize, String)>) {
    let mut words: BTreeSet<String> = BTreeSet::new();
    for w in ws.iter() {
        words.insert(w.clone());
    }
    let mut seen: Vec<String> = Vec::new();
    for w in &words {
        seen.push(w.clone());
    }
    let mut nums: BTreeSet<usize> = BTreeSet::new();
    for x in xs.iter() {
        nums.insert(*x);
    }
    let mut pairs: BTreeSet<(usize, String)> = BTreeSet::new();
    for (x, w) in xs.iter().zip(ws.iter()) {
        pairs.insert((*x, w.clone()));
    }
    (seen, nums.iter().copied().collect(), pairs.into_iter().collect())
}

/// `if let Some(x) = m.get_mut(k)` on a map and on a list, with a key that is there and one that is not, and an
/// early `return` after a write.
pub fn getmut_probe(ws: Vec<String>, xs: Vec<usize>, at: usize) -> (Vec<(String, usize)>, Vec<usize>, usize) {
    let mut m: BTreeMap<String, usize> = BTreeMap::new();
    for w in ws.iter() {
        if w.len() > 1 {
            m.insert(w.clone(), 0);
        }
    }
    for w in ws.iter() {
        if let Some(n) = m.get_mut(w) {
            *n += w.len();
        }
    }
    let mut v = xs.clone();
    if let Some(y) = v.get_mut(at) {
        *y = *y * 10;
        if *y > 50 {
            return (m.into_iter().collect(), v, 1);
        }
        *y += 1;
    }
    (m.into_iter().collect(), v, 0)
}

/// rsplit by patterns that overlap themselves and do not, a bool and a char as numbers, a bool shown, a set from an
/// array, and standard types named by their full path.
pub fn misc_probe(s: String, b: bool, c: String) -> (Vec<String>, Vec<String>, Vec<String>, u64, u32, String, usize) {
    let by_colons: Vec<String> = s.rsplit("::").map(|p| p.to_string()).collect();
    let by_aa: Vec<String> = s.rsplit("aa").map(|p| p.to_string()).collect();
    let by_comma: Vec<String> = s.rsplit(",").map(|p| p.to_string()).collect();
    let first = c.chars().next().unwrap_or('x');
    let set = std::collections::BTreeSet::from([s.clone(), c.clone(), s.clone()]);
    let m: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    (by_colons, by_aa, by_comma, u64::from(b), u32::from(first), format!("{b}"), set.len() + m.len())
}

fn half(x: usize) -> Option<usize> {
    if x % 2 == 0 {
        Some(x / 2)
    } else {
        None
    }
}

/// closures with their own exits in positions with no function to return from: `?` in filter_map, a loop and a
/// `return` in map, and a `return` in any.
pub fn closure_exit_probe(rows: Vec<Vec<usize>>) -> (Vec<usize>, Vec<usize>, bool) {
    let halves: Vec<usize> = rows
        .iter()
        .filter_map(|row| {
            let first = row.first()?;
            let h = half(*first)?;
            Some(h + row.len())
        })
        .collect();
    let capped: Vec<usize> = rows
        .iter()
        .map(|row| {
            let mut total = 0;
            for x in row {
                total += x;
                if total > 12 {
                    return 12;
                }
            }
            total
        })
        .collect();
    let any_big = rows.iter().any(|row| {
        for x in row {
            if *x > 7 {
                return true;
            }
        }
        false
    });
    (halves, capped, any_big)
}

pub const SORTED_WORDS: &[&str] = &["a", "abc", "b", "do", "fun", "z"];

/// a binary search on a constant, literal and sorted: whether it finds the item, either way round.
pub fn search_probe(w: String) -> (bool, bool) {
    (SORTED_WORDS.binary_search(&w.as_str()).is_ok(), SORTED_WORDS.binary_search(&w.as_str()).is_err())
}

#[derive(Clone, Copy)]
pub enum Tagged {
    Eq,
    ExistsUnique,
}

impl Tagged {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Eq => "eq",
            Self::ExistsUnique => "exists-unique",
        }
    }
}

/// a method of the crate's own type named as a standard one (`as_str`) is the crate's.
pub fn tag_probe(b: bool) -> String {
    let t = if b { Tagged::Eq } else { Tagged::ExistsUnique };
    format!("core::{}", t.as_str())
}

/// a number type's method named as a function, folded.
pub fn fold_probe(xs: Vec<u64>) -> (u64, u64, u64) {
    (xs.iter().copied().fold(1, u64::saturating_add), xs.iter().copied().fold(0, u64::max), xs.iter().copied().fold(99, u64::min))
}

use std::collections::BTreeMap;

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

"""Behaviour cases over the compiler's own types: each case is a Rust function both sides compile from the same text.

    python3 tooling/rs2lex/behaviour/typed.py fns.json SCRATCH_SRC OUT_DIR [MODULE...]

For each target (a translated function free of `self` and generics, in MODULEs when given) a few cases are written
into the target's own file of SCRATCH_SRC (a copy of crates/lexlean/src), each a function

    pub(crate) fn __behaviour_case_N() -> String { format!("{:?}", target(<values>)) }

where the values are built of the crate's structs and enums as their definitions allow, and of strings, numbers,
bools, Vec, Option, Box and tuples. The Rust side runs them in a test module of the same file; the translator
translates them with the rest of the copy, and the Lean side prints each one's value. Both print Rust's `{:?}` text,
the Lean side by the translator's derived Debug, so the comparison is of the bytes Rust's own Debug gives.

Writes OUT_DIR/cases.json: {N: {"target", "file"}}; and into each file, a #[cfg(test)] module that writes
`N<TAB><hex of the text>` lines (or PANIC) to $BEHAVIOUR_OUT/<file>.txt.
"""
import collections
import json
import os
import random
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))
import crate_load  # noqa: E402

fns_path, src, out_dir = sys.argv[1], sys.argv[2], sys.argv[3]
scope = set(sys.argv[4:])
PER_TARGET = 12
rng = random.Random(20261127)
items = crate_load.load(src, with_aliases=True)[0]
types = {it['name']: it for it in items if it['k'] in ('struct', 'enum')}
NUMS = {'usize', 'u64', 'u32', 'u16', 'u8', 'i64', 'i32', 'i16', 'i8', 'isize'}
WORDS = ['', 'a', 'x', 'Nat', 'f', 'a.b', 'Foo.bar', 'main', 'lemma', '0', '_', 'é', 'κ', 'a b', 'x\'', 'lexlean.std.nat', 'T']


def rust_str(s):
    return '"' + ''.join(c if c.isascii() and c not in '"\\' and c.isprintable() else '\\u{%x}' % ord(c) for c in s) + '"'


def path_of(name):
    it = types[name]
    return 'crate::' + '::'.join(list(it['mp']) + [it.get('orig_name', name)])


class Give(Exception):
    pass


def size(name, seen=()):
    """how deep the smallest value of a crate type goes, for choosing the variants that end a value."""
    if name in seen: return 99
    it = types[name]
    parts = [fs for _, fs in it['variants']] if it['k'] == 'enum' else [it['fields']]
    best = 99
    for fs in parts:
        d = 0
        for _, t in fs: d = max(d, tsize(t, seen + (name,)))
        best = min(best, d + 1)
    return best


def tsize(t, seen):
    if t[0] in ('ref', 'mut'): return tsize(t[1], seen)
    if t[0] == 'tuple': return max([tsize(x, seen) for x in t[1]] or [0])
    if t[0] != 'path': return 99
    n, a = t[1], t[2]
    if n in ('Vec', 'Option', 'BTreeMap', 'BTreeSet'): return 0           # empty, or none
    if n in ('Box', 'Rc', 'Arc'): return tsize(a[0], seen)
    if n in types: return size(n, seen)
    return 0


def value(t, depth):
    """Rust source of a value of type t, or Give."""
    if depth > 6: raise Give
    if t[0] == 'ref':
        inner = t[1]
        if inner[0] == 'path' and inner[1] == 'str': return rust_str(rng.choice(WORDS))
        if inner[0] == 'path' and inner[1] == 'Vec' or inner[0] == 'slice':
            return '&' + value(('path', 'Vec', inner[2] if inner[0] == 'path' else [inner[1]]), depth)
        return '&' + value(inner, depth)
    if t[0] == 'mut': raise Give
    if t[0] == 'tuple': return '(' + ', '.join(value(x, depth + 1) for x in t[1]) + (',)' if len(t[1]) == 1 else ')')
    if t[0] != 'path': raise Give
    n, a = t[1], t[2]
    if n in ('String',): return rust_str(rng.choice(WORDS)) + '.to_string()'
    if n == 'str': raise Give
    if n in NUMS:
        v = rng.choice([0, 1, 2, 3, 7, 10, 64, 255, rng.randint(0, 40)])
        if n[0] == 'i' and rng.random() < 0.3: v = -v
        return f'{v}{n}' if v >= 0 else f'({v}{n})'
    if n == 'bool': return rng.choice(['true', 'false'])
    if n == 'char': return repr(rng.choice('ax_0κ'))
    if n == 'Vec':
        k = 0 if depth >= 3 else rng.choice([0, 1, 1, 2, 3])
        return 'vec![' + ', '.join(value(a[0], depth + 1) for _ in range(k)) + ']'
    if n == 'Option':
        return 'None' if depth >= 3 or rng.random() < 0.3 else f'Some({value(a[0], depth + 1)})'
    if n in ('Box', 'Rc', 'Arc'):
        return {'Box': 'Box::new', 'Rc': 'std::rc::Rc::new', 'Arc': 'std::sync::Arc::new'}[n] + f'({value(a[0], depth)})'
    if n == 'BTreeMap':
        k = 0 if depth >= 3 else rng.choice([0, 1, 2])
        return 'std::collections::BTreeMap::from([' + ', '.join(f'({value(a[0], depth + 1)}, {value(a[1], depth + 1)})' for _ in range(k)) + '])'
    if n == 'BTreeSet':
        k = 0 if depth >= 3 else rng.choice([0, 1, 2])
        return 'std::collections::BTreeSet::from([' + ', '.join(value(a[0], depth + 1) for _ in range(k)) + '])'
    if n not in types or types[n].get('generic'): raise Give
    it = types[n]
    if it['k'] == 'struct':
        fs = it['fields']
        if fs and all(isinstance(f, int) or str(f).isdigit() or re.fullmatch(r'f\d+', str(f)) for f, _ in fs):
            return f'{path_of(n)}(' + ', '.join(value(ft, depth + 1) for _, ft in fs) + ')'
        return f'{path_of(n)} {{ ' + ', '.join(f'{f}: {value(ft, depth + 1)}' for f, ft in fs) + ' }'
    variants = it['variants']
    if depth >= 3:                                               # end the value: the variants that end soonest
        least = min(1 + max([tsize(ft, (n,)) for _, ft in fs] or [0]) for _, fs in variants)
        variants = [(v, fs) for v, fs in variants if 1 + max([tsize(ft, (n,)) for _, ft in fs] or [0]) == least]
    v, fs = rng.choice(variants)
    if not fs: return f'{path_of(n)}::{v}'
    if all(isinstance(f, int) or str(f).isdigit() for f, _ in fs):
        return f'{path_of(n)}::{v}(' + ', '.join(value(ft, depth + 1) for _, ft in fs) + ')'
    return f'{path_of(n)}::{v} {{ ' + ', '.join(f'{f}: {value(ft, depth + 1)}' for f, ft in fs) + ' }'


def owner_name(file, fn_name):
    text = open(os.path.join(src, file), encoding='utf-8').read()
    at = text.find(f'fn {fn_name}')
    impls = [m for m in re.finditer(r'\nimpl(?:<[^>]*>)?\s+([A-Za-z_]\w*)', text) if m.start() < at]
    return impls[-1].group(1) if impls else None


fns = json.load(open(fns_path, encoding='utf-8'))
per_file = collections.defaultdict(list)
cases, why = {}, collections.Counter()
n = 0
for f in fns:
    top = f['file'].split('/')[0].replace('.rs', '')
    if scope and top not in scope: continue
    if f['nested'] or f['generics']: why['nested or generic'] += 1; continue
    if f['params'] and f['params'][0][0] == 'self': why['method'] += 1; continue
    if tuple(f['mp']) != crate_load.module_path(f['file']): why['in an inline module'] += 1; continue
    if f['ret'] == ('tuple', []) or f['ret'] == ['tuple', []]: why['returns ()'] += 1; continue
    callee = f['orig_name']
    if f['owner']:
        owner = owner_name(f['file'], f['orig_name'])
        if owner is None: why['owner not found'] += 1; continue
        callee = f'{owner}::{f["orig_name"]}'
    made = set()
    for _ in range(PER_TARGET * 4):
        try: args = [value(t, 0) for _, t in f['params']]
        except Give: why['a parameter type the generator cannot build'] += 1; break
        text = f'{callee}({", ".join(args)})'
        if text in made: continue
        made.add(text)
        per_file[f['file']].append((n, text)); cases[n] = {'target': f['key'], 'file': f['file']}; n += 1
        if len(made) >= (1 if not f['params'] else PER_TARGET): break

# cases the Rust side does not compile (a field or a module out of sight of the target's file), left out by the run
exclude = set()
if os.environ.get('BEHAVIOUR_EXCLUDE') and os.path.exists(os.environ['BEHAVIOUR_EXCLUDE']):
    exclude = {int(x) for x in open(os.environ['BEHAVIOUR_EXCLUDE']).read().split()}
for file in list(per_file):
    per_file[file] = [(i, c) for i, c in per_file[file] if i not in exclude]
    if not per_file[file]: del per_file[file]
cases = {i: c for i, c in cases.items() if i not in exclude}
for file, cs in per_file.items():
    body = ['', '// behaviour cases (tooling/rs2lex/behaviour/typed.py)']
    for i, call in cs:
        body.append(f'#[allow(dead_code, unused, clippy::all)]\npub(crate) fn __behaviour_case_{i}() -> String {{ format!("{{:?}}", {call}) }}')
    ids = ', '.join(str(i) for i, _ in cs)
    arms = '\n'.join(f'            {i} => __behaviour_case_{i}(),' for i, _ in cs)
    body.append('#[cfg(test)]\nmod behaviour_cases {\n    use super::*;\n    #[test]\n    fn behaviour_cases() {\n'
                '        let out_dir = std::env::var("BEHAVIOUR_OUT").expect("BEHAVIOUR_OUT");\n'
                '        std::panic::set_hook(Box::new(|_| {}));\n        let mut out = String::new();\n'
                f'        for id in [{ids}] {{\n'
                '            let shown = std::panic::catch_unwind(|| match id {\n' + arms + '\n            _ => unreachable!(),\n'
                '            }).map(|s| s.bytes().map(|b| format!("{b:02x}")).collect::<String>()).unwrap_or_else(|_| "PANIC".to_owned());\n'
                '            out.push_str(&format!("{id}\\t{shown}\\n"));\n        }\n'
                f'        std::fs::write(format!("{{out_dir}}/{file.replace("/", "_")}.txt"), out).unwrap();\n    }}\n}}\n')
    with open(os.path.join(src, file), 'a', encoding='utf-8', newline='\n') as fh: fh.write('\n'.join(body))
json.dump(cases, open(os.path.join(out_dir, 'cases.json'), 'w'), indent=0)
print(f'{len(cases)} cases for {len({c["target"] for c in cases.values()})} targets in {len(per_file)} files; skipped: {dict(why)}')

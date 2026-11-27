"""The behaviour harness: translated functions of the compiler run against the compiler itself, case by case.

    python3 tooling/rs2lex/behaviour/gen.py OUT_DIR SRC_ROOT [MODULE...]

OUT_DIR holds a translation (Crate.lex.tex and fns.json, from translate.py); SRC_ROOT is the crate source it was made
from (crates/lexlean/src). A function is a target when it is free or associated (no `self`), and its parameters and
result are built from strings, unsigned integers, bools, Vec, Option, Result and pairs; MODULEs, when given, keep only
the functions of those top-level modules. Writes into OUT_DIR/behaviour:

  cases.jsonl                one case per line: {"id", "t" (the target), "args"}
  targets.json               the targets, by number
  rust/<file>.probe.rs       a test module per source file, appended to that file in a scratch copy of the crate, so
                             it can call private functions; rust/harness.rs, the shared printing and case reading
  lean/BehaviourMain.lean    the Lean driver over the translation

Both sides print each result in one canonical form, `id<TAB>value`:
  S<hex>  N<decimal>  T F  L[a,b]  O(a) O-  R+(a) R-(a)  P(a,b)  PANIC (Rust only)
"""
import collections
import json
import os
import random
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))
import crate_load       # noqa: E402
import rs2lex_core as core  # noqa: E402

out_dir, src = sys.argv[1], sys.argv[2]
scope = set(sys.argv[3:])
work = os.path.join(out_dir, 'behaviour')
os.makedirs(os.path.join(work, 'rust'), exist_ok=True)
os.makedirs(os.path.join(work, 'lean'), exist_ok=True)
rng = random.Random(20261127)

# ------------------------------------------------------------------------------------------------------- targets
UNSIGNED = {'usize': 64, 'u64': 64, 'u32': 32, 'u16': 16, 'u8': 8}


def shape(t):
    """a type as a small tree both drivers read and print, or None."""
    if t is None: return None
    if t[0] == 'ref': return shape(t[1])
    if t[0] == 'mut': return None
    if t[0] == 'tuple':
        if not t[1]: return ['unit']
        if len(t[1]) == 2:
            a, b = shape(t[1][0]), shape(t[1][1])
            return ['pair', a, b] if a and b else None
        return None
    if t[0] != 'path': return None
    n, args = t[1], t[2]
    if n in ('str', 'String'): return ['str']
    if n in UNSIGNED: return ['nat', UNSIGNED[n]]
    if n == 'bool': return ['bool']
    if n == 'Box' and args: return shape(args[0])
    if n == 'Vec' and args:
        e = shape(args[0]); return ['list', e] if e else None
    if n == 'Option' and args:
        e = shape(args[0]); return ['option', e] if e else None
    if n == 'Result' and len(args) == 2:
        a, b = shape(args[0]), shape(args[1]); return ['result', a, b] if a and b else None
    return None


fns = json.load(open(os.path.join(out_dir, 'fns.json'), encoding='utf-8'))
targets, why = [], collections.Counter()
for f in fns:
    top = f['file'].split('/')[0].replace('.rs', '')
    if scope and top not in scope: continue
    if f['nested']: why['nested fn'] += 1; continue
    if f['generics']: why['generic'] += 1; continue
    if f['params'] and f['params'][0][0] == 'self': why['method'] += 1; continue
    ps = [shape(t) for _, t in f['params']]
    rt = shape(f['ret'])
    if any(p is None for p in ps) or rt is None: why['types'] += 1; continue
    if 'unit' in json.dumps(ps) or 'unit' in json.dumps(rt): why['unit'] += 1; continue
    if any(p[0] == 'result' for p in ps): why['a Result parameter'] += 1; continue
    if tuple(f['mp']) != crate_load.module_path(f['file']): why['in an inline module'] += 1; continue
    targets.append(dict(f, shapes=ps, ret_shape=rt))

# -------------------------------------------------------------------------------------------------------- inputs
POOL = ['', ' ', 'a', 'A', '_', '0', '1', '007', '-1', '42', '18446744073709551615', '18446744073709551616', 'x.y', 'Nat', 'Init',
        'LexLean', 'Foo.Bar.baz', 'src/Main.lex.tex', 'a/b', '/abs', '../up', 'a//b', 'theorem', 'def', 'fun', 'λ', 'κ', 'α',
        'é', '\t', '\n', 'a b', ' a ', 'propext', 'true', 'false', 'none', 'some', 'u8', 'usize', 'Vec', "'", "x'", '#', '«x»']


def literals(f):
    """the string literals of the function's own body, and of its file: the forms it is written to accept."""
    text = open(os.path.join(src, f['file']), encoding='utf-8').read()
    m = re.search(r'fn\s+' + re.escape(f['orig_name']) + r'\b', text)
    body = ''
    if m and '{' in text[m.end():]:
        j = text.index('{', m.end()); depth = 0; k = j
        while k < len(text):
            if text[k] == '{': depth += 1
            elif text[k] == '}':
                depth -= 1
                if depth == 0: break
            k += 1
        body = text[m.start():k + 1]
    return re.findall(r'"([^"\\\n]{0,60})"', body), re.findall(r'"([^"\\\n]{1,60})"', text)


def value(s, a_string, depth=0):
    k = s[0]
    if k == 'str': return a_string()
    if k == 'nat':
        top = (1 << s[1]) - 1
        return rng.choice([0, 1, 2, 3, 7, 10, 25, 64, 100, 255, 256, 1000, rng.randint(0, 50), rng.randint(0, 50), min(top, 65535), top]
                          if s[1] <= 16 else [0, 1, 2, 3, 7, 10, 25, 64, 100, 255, 256, 1000, 4096, rng.randint(0, 50), rng.randint(0, 10 ** 6)])
    if k == 'bool': return rng.random() < 0.5
    if k == 'list': return [value(s[1], a_string, depth + 1) for _ in range(rng.choice([0, 1, 1, 2, 3, 4, 6]) if depth < 2 else rng.randint(0, 2))]
    if k == 'option': return None if rng.random() < 0.3 else value(s[1], a_string, depth + 1)
    if k == 'pair': return [value(s[1], a_string, depth + 1), value(s[2], a_string, depth + 1)]
    raise ValueError(k)


cases = []
for i, f in enumerate(targets):
    own, near = literals(f)

    def a_string(own=own, near=near):
        r = rng.random()
        pick = lambda: rng.choice(own) if own and rng.random() < 0.7 else (rng.choice(near) if near else rng.choice(POOL))
        if r < 0.3 and (own or near): return pick()
        if r < 0.4 and (own or near):
            x = pick(); extra = ''.join(rng.choice('abz_09.') for _ in range(rng.randint(1, 6)))
            return x + extra if rng.random() < 0.6 else extra + x
        if r < 0.5 and (own or near):
            x = pick(); cut = rng.randint(0, len(x)); return x[:cut] if rng.random() < 0.5 else x[cut:]
        if r < 0.65 and (own or near): return pick() + rng.choice(['', '.', '/', ' ', '_', '-', ':', '0', 'x']) + pick()
        if r < 0.85: return rng.choice(POOL)
        return ''.join(rng.choice('abcXYZ_09.-/\\ \'"#κ{}') for _ in range(rng.randint(0, 12)))
    n = 1 if not f['shapes'] else 150
    seen = set()
    for _ in range(n * 3):
        args = [value(p, a_string) for p in f['shapes']]
        key = json.dumps(args, ensure_ascii=False)
        if key in seen: continue
        seen.add(key); cases.append({'id': len(cases), 't': i, 'args': args})
        if len(seen) >= n: break
with open(os.path.join(work, 'cases.jsonl'), 'w', encoding='utf-8', newline='\n') as out:
    for c in cases: out.write(json.dumps(c, ensure_ascii=False) + '\n')

# ----------------------------------------------------------------------------------------------------- Rust side
def rust_type(t):
    """the owned Rust type a parameter is read into, from its written type."""
    if t[0] in ('ref', 'mut'): return rust_type(t[1])
    if t[0] == 'tuple': return '(' + ', '.join(rust_type(x) for x in t[1]) + (',)' if len(t[1]) == 1 else ')')
    n, a = t[1], t[2]
    if n == 'str': return 'String'
    if n == 'Box': return rust_type(a[0])
    return n + ('<' + ', '.join(rust_type(x) for x in a) + '>' if a else '')


def rust_elems(et, v):
    if et[0] == 'ref' and et[1][0] == 'path' and et[1][1] == 'str': return f'{v}.iter().map(|x| x.as_str()).collect::<Vec<&str>>()'
    if et[0] == 'ref': return f'{v}.iter().collect::<Vec<_>>()'
    return v


def rust_arg(t, v):
    """an expression of the parameter's own type, from v of its owned type."""
    if t[0] == 'ref':
        inner = t[1]
        if inner[0] == 'path' and inner[1] == 'str': return f'&*Box::leak({v}.clone().into_boxed_str())'
        if inner[0] == 'path' and inner[1] == 'Vec': return f'&{rust_elems(inner[2][0], v)}[..]'
        return f'&{v}'
    if t[0] == 'path' and t[1] == 'Vec': return rust_elems(t[2][0], v)
    if t[0] == 'path' and t[1] == 'Option' and t[2] and t[2][0][0] == 'ref':
        inner = t[2][0][1]
        return f'{v}.as_deref()' if inner[0] == 'path' and inner[1] == 'str' else f'{v}.as_ref()'
    return v


def signature(f):
    """the function's signature as written, parsed again: the translator's has its types renamed."""
    text = open(os.path.join(src, f['file']), encoding='utf-8').read()
    m = re.search(r'fn\s+' + re.escape(f['orig_name']) + r'\s*(<[^>]*>)?\s*\(', text)
    if not m: return None
    depth, j = 0, m.end() - 1
    while True:
        if text[j] == '(': depth += 1
        elif text[j] == ')':
            depth -= 1
            if depth == 0: break
        j += 1
    k = j
    while text[k] not in '{;': k += 1
    try: sig = core.Parser(core.lex(text[m.start():k] + '{}')).fn()
    except core.Unsupported: return None
    params, depth, cur = [], 0, ''
    for ch in text[m.end():j]:
        if ch in '<([': depth += 1
        elif ch in '>)]': depth -= 1
        if ch == ',' and depth == 0: params.append(cur); cur = ''
        else: cur += ch
    if cur.strip(): params.append(cur)
    sig['iter_params'] = {n for n, p in enumerate(params) if 'impl Iterator' in p or 'impl IntoIterator' in p}
    return sig


def owner_name(f):
    """the type the function is written in the impl of: the last `impl Name` before it in its file."""
    text = open(os.path.join(src, f['file']), encoding='utf-8').read()
    at = text.find(f'fn {f["orig_name"]}')
    impls = [m for m in re.finditer(r'\nimpl(?:<[^>]*>)?\s+([A-Za-z_]\w*)', text) if m.start() < at]
    return impls[-1].group(1) if impls else None


per_file, skipped = collections.defaultdict(list), []
for i, f in enumerate(targets):
    sig = signature(f)
    if sig is None or (sig['params'] and sig['params'][0][0] == 'self'): skipped.append(f['key']); continue
    callee = f['orig_name']
    if f['owner']:
        owner = owner_name(f)
        if owner is None: skipped.append(f['key']); continue
        callee = f'{owner}::{f["orig_name"]}'
    lets = ' '.join(f'let a{j}: {rust_type(pt)} = serde_json::from_value(args[{j}].clone()).unwrap();' for j, (_, pt) in enumerate(sig['params']))
    call = ', '.join(rust_arg(pt, f'a{j}') + ('.into_iter()' if j in sig['iter_params'] else '') for j, (_, pt) in enumerate(sig['params']))
    per_file[f['file']].append((i, f'            {i} => {{ {lets} let r = {callee}({call}); crate::behaviour_harness::Show::show(&r) }}'))

HARNESS = r'''// the behaviour harness (tooling/rs2lex/behaviour): canonical printing and case reading, in test builds only
pub trait Show { fn show(&self) -> String; }
impl Show for String { fn show(&self) -> String { format!("S{}", self.bytes().map(|b| format!("{b:02x}")).collect::<String>()) } }
impl Show for str { fn show(&self) -> String { format!("S{}", self.bytes().map(|b| format!("{b:02x}")).collect::<String>()) } }
macro_rules! nat { ($($t:ty),*) => { $(impl Show for $t { fn show(&self) -> String { format!("N{}", self) } })* } }
nat!(u8, u16, u32, u64, usize);
impl Show for bool { fn show(&self) -> String { if *self { "T".into() } else { "F".into() } } }
impl<T: Show> Show for Vec<T> { fn show(&self) -> String { format!("L[{}]", self.iter().map(|x| x.show()).collect::<Vec<_>>().join(",")) } }
impl<T: Show> Show for [T] { fn show(&self) -> String { format!("L[{}]", self.iter().map(|x| x.show()).collect::<Vec<_>>().join(",")) } }
impl<T: Show + ?Sized> Show for &T { fn show(&self) -> String { (**self).show() } }
impl<T: Show> Show for Option<T> { fn show(&self) -> String { match self { Some(x) => format!("O({})", x.show()), None => "O-".into() } } }
impl<T: Show, E: Show> Show for Result<T, E> { fn show(&self) -> String { match self { Ok(x) => format!("R+({})", x.show()), Err(e) => format!("R-({})", e.show()) } } }
impl<A: Show, B: Show> Show for (A, B) { fn show(&self) -> String { format!("P({},{})", self.0.show(), self.1.show()) } }
impl<T: Show + ?Sized> Show for Box<T> { fn show(&self) -> String { (**self).show() } }
impl<'a> Show for std::borrow::Cow<'a, str> { fn show(&self) -> String { self.as_ref().show() } }

pub fn run(tag: &str, wanted: &[usize], call: &dyn Fn(usize, &[serde_json::Value]) -> String) {
    let cases = std::env::var("BEHAVIOUR_CASES").expect("BEHAVIOUR_CASES");
    let out_dir = std::env::var("BEHAVIOUR_OUT").expect("BEHAVIOUR_OUT");
    let mut out = String::new();
    std::panic::set_hook(Box::new(|_| {}));
    for line in std::fs::read_to_string(cases).unwrap().lines() {
        let case: serde_json::Value = serde_json::from_str(line).unwrap();
        let t = case["t"].as_u64().unwrap() as usize;
        if !wanted.contains(&t) { continue; }
        let id = case["id"].as_u64().unwrap();
        let args = case["args"].as_array().unwrap().clone();
        let shown = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| call(t, &args))).unwrap_or_else(|_| "PANIC".to_owned());
        out.push_str(&format!("{id}\t{shown}\n"));
    }
    std::fs::write(format!("{out_dir}/{}.txt", tag.replace('/', "_")), out).unwrap();
}
'''
open(os.path.join(work, 'rust', 'harness.rs'), 'w', encoding='utf-8', newline='\n').write(HARNESS)
index = {}
for file, arms in per_file.items():
    ids = ', '.join(str(i) for i, _ in arms)
    module = ('\n#[cfg(test)]\n#[allow(unused_imports, clippy::all)]\nmod behaviour_probe {\n    use super::*;\n    #[test]\n    fn behaviour_probe() {\n'
              f'        crate::behaviour_harness::run("{file}", &[{ids}], &|t, args| match t {{\n' + '\n'.join(a for _, a in arms) +
              '\n            _ => unreachable!(),\n        });\n    }\n}\n')
    name = file.replace('/', '__') + '.probe.rs'
    open(os.path.join(work, 'rust', name), 'w', encoding='utf-8', newline='\n').write(module)
    index[file] = name
json.dump(index, open(os.path.join(work, 'rust', 'index.json'), 'w'), indent=1)

# ----------------------------------------------------------------------------------------------------- Lean side
def lean_dec(s, j):
    k = s[0]
    if k == 'str': return f'(do let s ← {j}.getStr?; pure s.toUTF8)'
    if k == 'nat': return f'{j}.getNat?'
    if k == 'bool': return f'{j}.getBool?'
    if k == 'list': return f'(do let a ← {j}.getArr?; a.toList.mapM (fun x => {lean_dec(s[1], "x")}))'
    if k == 'option': return f'(if {j}.isNull then pure none else (do let v ← {lean_dec(s[1], j)}; pure (some v)))'
    if k == 'pair': return f'(do let a ← {j}.getArrVal? 0; let b ← {j}.getArrVal? 1; let x ← {lean_dec(s[1], "a")}; let y ← {lean_dec(s[2], "b")}; pure (x, y))'
    raise ValueError(k)


def lean_show(s, v):
    k = s[0]
    if k == 'str': return f'("S" ++ hex {v})'
    if k == 'nat': return f'("N" ++ toString {v})'
    if k == 'bool': return f'(if {v} then "T" else "F")'
    if k == 'list': return f'("L[" ++ ",".intercalate ({v}.map (fun x => {lean_show(s[1], "x")})) ++ "]")'
    if k == 'option': return f'(match {v} with | some x => "O(" ++ {lean_show(s[1], "x")} ++ ")" | none => "O-")'
    if k == 'pair': return f'(match {v} with | (x, y) => "P(" ++ {lean_show(s[1], "x")} ++ "," ++ {lean_show(s[2], "y")} ++ ")")'
    if k == 'result': return f'(match {v} with | .ok x => "R+(" ++ {lean_show(s[1], "x")} ++ ")" | .error e => "R-(" ++ {lean_show(s[2], "e")} ++ ")")'
    raise ValueError(k)


lines = ['import Lean.Data.Json', 'import Mech.Crate', 'open Lean (Json)', '',
         'def hexDigit (n : UInt8) : Char := Char.ofNat (if n < 10 then 48 + n.toNat else 87 + n.toNat)',
         'def hex (b : ByteArray) : String := Id.run do', '  let mut s := ""', '  for x in b.toList do',
         '    s := (s.push (hexDigit (x / 16))).push (hexDigit (x % 16))', '  return s', '']
dispatch = []
for i, f in enumerate(targets):
    decs = '\n'.join(f'  let a{j} ← {lean_dec(p, f"args[{j}]!")}' for j, p in enumerate(f['shapes']))
    call = ' '.join((['18446744073709551615'] if f['fuel'] else []) + [f'a{j}' for j in range(len(f['shapes']))])
    lines.append(f'def run{i} (args : Array Json) : Except String String := do')
    if decs: lines.append(decs)
    lines.append(f'  pure {lean_show(f["ret_shape"], f"(Mech.Crate.{f[chr(108) + chr(101) + chr(120)]} {call})")}')
    dispatch.append(f'  | {i} => run{i} args')
lines += ['', 'def dispatch (t : Nat) (args : Array Json) : Except String String :=', '  match t with'] + dispatch + [
          '  | _ => .error "no target"', '',
          'def main (argv : List String) : IO Unit := do',
          '  for line in (← IO.FS.lines argv[0]!) do',
          '    match Json.parse line with',
          '    | .error e => IO.println s!"?\\t{e}"',
          '    | .ok j =>',
          '      let id := (j.getObjValAs? Nat "id").toOption.getD 0',
          '      let t := (j.getObjValAs? Nat "t").toOption.getD 0',
          '      let args := ((j.getObjVal? "args").toOption.bind (fun a => a.getArr?.toOption)).getD #[]',
          '      match dispatch t args with',
          '      | .ok s => IO.println s!"{id}\\t{s}"',
          '      | .error e => IO.println s!"{id}\\tDECODE {e}"']
open(os.path.join(work, 'lean', 'BehaviourMain.lean'), 'w', encoding='utf-8', newline='\n').write('\n'.join(lines) + '\n')
json.dump([{k: f[k] for k in ('key', 'file', 'lex', 'bytes')} for f in targets], open(os.path.join(work, 'targets.json'), 'w'), indent=1)
print(f'{len(cases)} cases for {len(targets)} targets in {len(per_file)} files; not targets: {dict(why)}; no Rust call: {len(skipped)}')

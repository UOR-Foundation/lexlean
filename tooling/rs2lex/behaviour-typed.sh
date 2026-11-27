#!/usr/bin/env bash
# Behaviour cases over the compiler's own types (behaviour/typed.py): the cases are written into a scratch copy of the
# crate, run there by Rust, and translated with the copy (from rustc's typed tree) to run in Lean.
#
#   tooling/rs2lex/behaviour-typed.sh FNS_JSON CRATE_WORKSPACE OUT_DIR [MODULE...]
#
# FNS_JSON is a translation's fns.json (translate.py): its functions are the targets. LEXLEAN names the compiler;
# CARGO_TARGET_DIR, if set, keeps the scratch crate's builds between runs.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../.." && pwd)"
fns="$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"; crate="$(cd "$2" && pwd)"; mkdir -p "$3"; out="$(cd "$3" && pwd)"; shift 3
lexlean="${LEXLEAN:-$repo/target/release/lexlean}"
toolchain="$(cat "$repo/bootstrap/lean-toolchain")"
scratch="$out/crate"; exclude="$out/exclude.txt"; : > "$exclude"

fresh_copy() {
  rm -rf "$scratch"; mkdir -p "$scratch"
  (cd "$crate" && tar --exclude=./target --exclude=./.git --exclude='./.lexlean' -cf - .) | (cd "$scratch" && tar -xf -)
  BEHAVIOUR_EXCLUDE="$exclude" python3 "$here/behaviour/typed.py" "$fns" "$scratch/crates/lexlean/src" "$out" "$@"
}

# -- the cases the Rust side compiles: those that do not are left out, a round at a time
for round in 1 2 3 4 5 6; do
  fresh_copy
  if (cd "$scratch" && cargo test -q -p lexlean --lib --no-run --message-format=short > "$out/cargo.log" 2>&1); then break; fi
  python3 - "$scratch/crates/lexlean/src" "$out/cargo.log" "$exclude" <<'PY'
import re, sys
src, log, exclude = sys.argv[1:4]
bad = set()
for m in re.finditer(r'^crates/lexlean/src/(\S+?):(\d+):\d+: error', open(log).read(), re.M):
    lines = open(f'{src}/{m.group(1)}', encoding='utf-8').read().split('\n')
    for k in range(int(m.group(2)) - 1, -1, -1):
        c = re.search(r'fn __behaviour_case_(\d+)\(', lines[k])
        if c: bad.add(c.group(1)); break
if not bad: print(open(log).read()[-3000:]); sys.exit('errors outside the cases')
open(exclude, 'a').write('\n'.join(sorted(bad)) + '\n')
print('left out', len(bad), 'cases the Rust side does not compile')
PY
done
rm -rf "$out/out-rust"; mkdir -p "$out/out-rust"
(cd "$scratch" && BEHAVIOUR_OUT="$out/out-rust" cargo test -q -p lexlean --lib behaviour_cases -- --test-threads=4 > "$out/test.log" 2>&1 || true)

# -- the copy, cases and all, translated from rustc's tree
(cd "$scratch" && RUSTC_BOOTSTRAP=1 cargo rustc -q -p lexlean --lib --profile dev -- -Zunpretty=thir-tree 2>/dev/null | gzip > "$out/thir.gz")
python3 "$here/translate.py" "$scratch/crates/lexlean/src" "$out/translation" "$out/thir.gz" "$scratch" > "$out/translate.log" 2>&1
tail -2 "$out/translate.log"

# -- Lean: each translated case's text
python3 - "$out" <<'PY'
import json, sys
out = sys.argv[1]
cases = json.load(open(f'{out}/cases.json'))
lex = {f['name']: f['lex'] for f in json.load(open(f'{out}/translation/fns.json')) if f['name'].startswith('__behaviour_case_')}
lines = ['import Mech.Crate', '',
         'def hexDigit (n : UInt8) : Char := Char.ofNat (if n < 10 then 48 + n.toNat else 87 + n.toNat)',
         'def hex (b : ByteArray) : String := Id.run do', '  let mut s := ""', '  for x in b.toList do',
         '    s := (s.push (hexDigit (x / 16))).push (hexDigit (x % 16))', '  return s', '', 'def main : IO Unit := do']
for i in sorted(cases, key=int):
    name = lex.get(f'__behaviour_case_{i}')
    lines.append(f'  IO.println s!"{i}\\t{{hex Mech.Crate.{name}}}"' if name else f'  IO.println "{i}\\tNOT TRANSLATED"')
open(f'{out}/TypedMain.lean', 'w').write('\n'.join(lines) + '\n')
PY
ws="$out/workspace"
bash "$here/workspace.sh" "$ws" "$out/translation/Crate.lex.tex"
(cd "$ws/mech" && "$lexlean" lock >/dev/null && "$lexlean" check >/dev/null && "$lexlean" build >/dev/null)
built="$(ls -td "$ws/mech/.lexlean/build"/*/ | head -1)"
rt="$out/runtime"; rm -rf "$rt"; cp -r "$repo/language/runtime-1.2/library" "$rt"; rm -rf "$rt/.lexlean"
(cd "$rt" && "$lexlean" lock >/dev/null && "$lexlean" build >/dev/null)
rt_built="$(ls -td "$rt/.lexlean/build"/*/ | head -1)"
lean="$out/lean"; rm -rf "$lean"; mkdir -p "$lean/gen"
cp -r "$built/modules/." "$rt_built/modules/." "$lean/gen/"; cp "$out/TypedMain.lean" "$lean/"
echo "$toolchain" > "$lean/lean-toolchain"
printf 'name = "rs2lex_typed"\ndefaultTargets = ["typed"]\n\n[[lean_lib]]\nname = "Gen"\nsrcDir = "gen"\nglobs = ["LexLeanRuntime.+", "LexLeanStd.+", "Mech.+"]\n\n[[lean_exe]]\nname = "typed"\nroot = "TypedMain"\n' > "$lean/lakefile.toml"
(cd "$lean" && { lake build typed > "$out/lake.log" 2>&1 || { grep -E "^error" -A4 "$out/lake.log" | head -40; exit 1; }; }
  ./.lake/build/bin/typed > "$out/out-lean.txt")

# -- compare
python3 - "$out" <<'PY'
import collections, glob, json, sys
out = sys.argv[1]
cases = json.load(open(f'{out}/cases.json'))
def read(path):
    d = {}
    for line in open(path, encoding='utf-8'):
        if '\t' in line: i, v = line.rstrip('\n').split('\t', 1); d[i] = v
    return d
rust = {}
for f in glob.glob(f'{out}/out-rust/*.txt'): rust.update(read(f))
lean = read(f'{out}/out-lean.txt')
per = collections.defaultdict(collections.Counter); ex = {}
for i, c in cases.items():
    r, l = rust.get(i, 'NOT RUN'), lean.get(i, 'MISSING')
    k = 'Rust panics' if r == 'PANIC' else 'not run in Rust' if r == 'NOT RUN' else 'not translated' if l == 'NOT TRANSLATED' else 'same' if r == l else 'differ'
    per[c['target']][k] += 1
    if k == 'differ': ex.setdefault(c['target'], (i, r, l))
tot = collections.Counter()
for v in per.values(): tot.update(v)
print(f'{len(per)} targets, {sum(tot.values())} cases: ' + ', '.join(f'{k} {v}' for k, v in sorted(tot.items())))
print('targets with every translated case the same as Rust:', sum(1 for v in per.values() if v['same'] and not v['differ']))
for t, (i, r, l) in sorted(ex.items()):
    dec = lambda h: bytes.fromhex(h).decode('utf-8', 'replace') if all(c in '0123456789abcdef' for c in h) else h
    print(f'  {t}: case {i}\n    rust {dec(r)[:160]}\n    lean {dec(l)[:160]}')
sys.exit(1 if ex else 0)
PY

"""The keyed-map probe: seeded operation sequences for run_nat, run_pair and run_tail, written as a Lean driver
(lean/ProbeMain.lean) and as the outcomes a BTreeMap gives (probe_expected.txt). Keys reach both ends of u64 and are
decimal prefixes of one another (1, 10, 100, 19), where a wrong padding would reorder them."""
import random

rng = random.Random(0x0B1E)
NATS = [0, 1, 2, 9, 10, 11, 19, 100, 101, 999, 1000, 10**19, 10**19 + 3, 2**63, 2**64 - 1]
WORDS = ['', 'a', 'b', 'ab', 'ba', 'abc']

def nat(): return rng.choice(NATS) if rng.random() < 0.6 else rng.randrange(0, 40)
def word(): return rng.choice(WORDS)
def code(): return rng.choice([0, 0, 0, 1, 2])

def hx(s): return s.encode().hex()
def opt(v): return 'N' if v is None else 'S' + hx(v)

cases = {'nat': [], 'pair': [], 'tail': []}
for _ in range(150):
    n = rng.randrange(0, 30)
    cases['nat'].append([(code(), nat(), 'v' + str(rng.randrange(9))) for _ in range(n)])
    cases['pair'].append([(code(), nat(), nat(), 'v' + str(rng.randrange(9))) for _ in range(n)])
    cases['tail'].append([(code(), nat(), word(), 'v' + str(rng.randrange(9))) for _ in range(n)])

def run(ops, key):
    table, seen = {}, []
    for op in ops:
        k = key(op); seen.append(opt(table.get(k)))
        if op[0] == 0: table[k] = op[-1]
        elif op[0] == 1: table.pop(k, None)
    return seen, sorted(table.items(), key=lambda kv: (kv[0] if not isinstance(kv[0], tuple) else
                                                       tuple(x.encode() if isinstance(x, str) else x for x in kv[0])))

out = []
for ops in cases['nat']:
    seen, entries = run(ops, lambda op: op[1])
    out.append('nat ' + ','.join(seen) + ';' + ','.join(f'{k}={hx(v)}' for k, v in entries))
for ops in cases['pair']:
    seen, entries = run(ops, lambda op: (op[1], op[2]))
    out.append('pair ' + ','.join(seen) + ';' + ','.join(f'{k[0]}.{k[1]}={hx(v)}' for k, v in entries))
for ops in cases['tail']:
    seen, entries = run(ops, lambda op: (op[1], op[2]))
    out.append('tail ' + ','.join(seen) + ';' + ','.join(f'{k[0]}.{hx(k[1])}={hx(v)}' for k, v in entries))
# count_over: a recursive FnMut visitor whose closure counts and records, at depths 0 to 3
visits = [([rng.randrange(0, 12) for _ in range(rng.randrange(0, 7))], rng.randrange(0, 14)) for _ in range(150)]
for xs, limit in visits:
    seen = [x + d for d in range(4) for x in xs]
    out.append('visit ' + str(sum(1 for v in seen if v > limit)) + ';' + ','.join(map(str, seen)))
# combos: Option::map, Option::unwrap_or_else and Result::map_err, each closure bumping one counter
combos = [[rng.randrange(0, 12) for _ in range(rng.randrange(0, 9))] for _ in range(150)]
for xs in combos:
    count, res = 0, []
    for x in xs:
        if x % 2 == 0: count += 1; y = x * 2
        else: count += 10; y = x
        if x > 5: z = y
        else: count += 100; z = y + 1
        res.append(z)
    out.append('combos ' + str(count) + ';' + ','.join(map(str, res)))
# ordering_probe: a stable sort, max_by_key (last greatest), min_by_key (first least), a reversed key then a tiebreak
orders = [[(rng.randrange(0, 4), rng.choice(WORDS)) for _ in range(rng.randrange(0, 9))] for _ in range(150)]
def pairs_text(ps): return ','.join(f'{k}.{hx(w)}' for k, w in ps)
def opt_pair(p): return 'N' if p is None else 'S' + pairs_text([p])
for xs in orders:
    stable = sorted(xs, key=lambda p: p[0])
    most = None
    for p in xs:
        if most is None or not (p[0] < most[0]): most = p
    least = None
    for p in xs:
        if least is None or p[0] < least[0]: least = p
    both = sorted(xs, key=lambda p: (-p[0], p[1].encode()))
    out.append('order ' + pairs_text(stable) + ';' + opt_pair(most) + ';' + opt_pair(least) + ';' + pairs_text(both))
# slices: match on [], [x], [x, y] with a guard, [x, y, rest @ ..], [.., last], a slice inside an Option, an array let
slice_cases = [([rng.randrange(0, 9) for _ in range(rng.randrange(0, 5))],
                rng.choice([None, [], [rng.randrange(0, 9)], [rng.randrange(0, 9) for _ in range(rng.randrange(2, 5))]])) for _ in range(150)]
for xs, wrapped in slice_cases:
    if not xs: shape = 0
    elif len(xs) == 1: shape = 100 + xs[0]
    elif len(xs) == 2 and xs[0] > xs[1]: shape = 200 + xs[0] - xs[1]
    elif len(xs) >= 2: shape = 300 + xs[0] + xs[1] + len(xs) - 2
    else: shape = 999
    last = xs[-1] if xs else 7
    if wrapped is None: inner = 2
    elif len(wrapped) == 1: inner = wrapped[0]
    elif len(wrapped) >= 2: inner = wrapped[0] + wrapped[1]
    else: inner = 1
    out.append(f'slices {shape},{last},{inner},{len(xs) + last * 10 + inner * 100}')
# char_probe: char_indices, is_whitespace (Rust's White_Space set, written out), ASCII tests, push of chars
RUST_WHITE = set('\t\n\x0b\x0c\r \x85\xa0                　')
ALPHABET = ['a', 'Q', 'z', '0', '7', ' ', '\t', '\n', '\xa0', ' ', '　', '​', 'é', 'Ж', '日', '😀', '_', '#', '\x1c']
char_cases = [''.join(rng.choice(ALPHABET) for _ in range(rng.randrange(0, 8))) for _ in range(150)]
for s_ in char_cases:
    starts, at = [], 0
    for ch in s_: starts.append(at); at += len(ch.encode())
    white = sum(1 for ch in s_ if ch in RUST_WHITE)
    alpha = sum(1 for ch in s_ if ch.isascii() and ch.isalpha())
    masked = ''.join('#' if (ch.isascii() and ch.isdigit()) else ch for ch in s_)
    upper = bool(s_) and (s_.encode()[0] == ord('Q') or 65 <= s_.encode()[0] <= 90)
    out.append('chars ' + ','.join(map(str, starts)) + ';' + str(white) + ';' + str(alpha) + ';' + hx(masked) + ';' + str(upper).lower())
# alias_probe: a RefCell log written through a held borrow_mut guard, and a Cell total
alias_cases = [[rng.randrange(0, 8) for _ in range(rng.randrange(0, 7))] for _ in range(150)]
for xs in alias_cases:
    log = []
    for x in xs:
        log.append(x)
        if x > 3: log.append(x * 2)
    out.append('alias ' + ','.join(map(str, log)) + ';' + str(sum(xs)))
# eager_probe: find_map stops at the first hit; filter and filter_map collected; bool.then; each closure counts
eager_cases = [([rng.randrange(0, 10) for _ in range(rng.randrange(0, 7))], rng.randrange(0, 10)) for _ in range(150)]
def opt_nat(v): return 'N' if v is None else 'S' + str(v)
for xs, limit in eager_cases:
    seen, first = 0, None
    for x in xs:
        seen += 1
        if x > limit: first = x * 2; break
    evens = [x for x in xs if x % 2 == 0]
    halves = [x // 2 for x in xs if x % 2 == 0]
    then = len(xs) if len(xs) > 2 else None
    code = seen * 1000000 + len(xs) * 10000 + sum(xs) * 100 + (7 if len(xs) > 2 else 0)
    out.append('eager ' + opt_nat(first) + ';' + ','.join(map(str, evens)) + ';' + ','.join(map(str, halves)) + ';' + opt_nat(then) + ';' + str(code))
open('probe_expected.txt', 'w', newline='\n').write('\n'.join(out) + '\n')

def lit(s): return f'"{s}".toUTF8'
def lean_list(items): return '[' + ', '.join(items) + ']'
lean = ['import Mech.Probe', 'open Mech.Probe', '',
        'def hx (b : ByteArray) : String := b.toList.foldl (fun s x => s ++ (let h := Nat.toDigits 16 x.toNat; String.mk (if h.length == 1 then \'0\' :: h else h))) ""',
        'def opt (o : Option ByteArray) : String := match o with | some b => "S" ++ hx b | none => "N"',
        'def seenOf (s : List (Option ByteArray)) : String := String.intercalate "," (s.map opt)', '']
lean.append('def natCases : List (List (Nat × Nat × ByteArray)) := ' + lean_list(
    [lean_list([f'({c}, {k}, {lit(v)})' for c, k, v in ops]) for ops in cases['nat']]))
lean.append('def pairCases : List (List (Nat × Nat × Nat × ByteArray)) := ' + lean_list(
    [lean_list([f'({c}, {a}, {b}, {lit(v)})' for c, a, b, v in ops]) for ops in cases['pair']]))
lean.append('def tailCases : List (List (Nat × Nat × ByteArray × ByteArray)) := ' + lean_list(
    [lean_list([f'({c}, {a}, {lit(b)}, {lit(v)})' for c, a, b, v in ops]) for ops in cases['tail']]))
lean.append('def visitCases : List (List Nat × Nat) := ' + lean_list([f'({lean_list([str(x) for x in xs])}, {limit})' for xs, limit in visits]))
lean.append('def orderCases : List (List (Nat × ByteArray)) := ' + lean_list([lean_list([f'({k}, {lit(w)})' for k, w in xs]) for xs in orders]))
lean.append('def pairsText (ps : List (Nat × ByteArray)) : String := String.intercalate "," (ps.map fun p => toString p.1 ++ "." ++ hx p.2)')
lean.append('def optPair (o : Option (Nat × ByteArray)) : String := match o with | some p => "S" ++ pairsText [p] | none => "N"')
def lean_opt_list(w): return 'none' if w is None else f'(some {lean_list([str(x) for x in w])})'
lean.append('def sliceCases : List (List Nat × Option (List Nat)) := ' + lean_list([f'({lean_list([str(x) for x in xs])}, {lean_opt_list(w)})' for xs, w in slice_cases]))
def lean_bytes(s_): return '(ByteArray.mk #[' + ', '.join(str(b) for b in s_.encode()) + '])'
lean.append('def charCases : List ByteArray := ' + lean_list([lean_bytes(s_) for s_ in char_cases]))
lean.append('def aliasCases : List (List Nat) := ' + lean_list([lean_list([str(x) for x in xs]) for xs in alias_cases]))
lean.append('def eagerCases : List (List Nat × Nat) := ' + lean_list([f'({lean_list([str(x) for x in xs])}, {limit})' for xs, limit in eager_cases]))
lean.append('def optNat (o : Option Nat) : String := match o with | some v => "S" ++ toString v | none => "N"')
lean.append('def comboCases : List (List Nat) := ' + lean_list([lean_list([str(x) for x in xs]) for xs in combos]))
lean += ['',
         'def main : IO Unit := do',
         '  for ops in natCases do',
         '    let r := fRunNat ops',
         '    IO.println ("nat " ++ seenOf r.1 ++ ";" ++ String.intercalate "," (r.2.map fun e => toString e.1 ++ "=" ++ hx e.2))',
         '  for ops in pairCases do',
         '    let r := fRunPair ops',
         '    IO.println ("pair " ++ seenOf r.1 ++ ";" ++ String.intercalate "," (r.2.map fun e => toString e.1.1 ++ "." ++ toString e.1.2 ++ "=" ++ hx e.2))',
         '  for ops in tailCases do',
         '    let r := fRunTail ops',
         '    IO.println ("tail " ++ seenOf r.1 ++ ";" ++ String.intercalate "," (r.2.map fun e => toString e.1.1 ++ "." ++ hx e.1.2 ++ "=" ++ hx e.2))',
         '  for (xs, limit) in visitCases do',
         '    let r := fCountOver xs limit',
         '    IO.println ("visit " ++ toString r.1 ++ ";" ++ String.intercalate "," (r.2.map toString))',
         '  for xs in comboCases do',
         '    let r := fCombos xs',
         '    IO.println ("combos " ++ toString r.1 ++ ";" ++ String.intercalate "," (r.2.map toString))',
         '  for xs in orderCases do',
         '    let r := fOrderingProbe xs',
         '    IO.println ("order " ++ pairsText r.1 ++ ";" ++ optPair r.2.1 ++ ";" ++ optPair r.2.2.1 ++ ";" ++ pairsText r.2.2.2)',
         '  for (xs, w) in sliceCases do',
         '    let r := fSlices xs w',
         '    IO.println ("slices " ++ toString r.1 ++ "," ++ toString r.2.1 ++ "," ++ toString r.2.2.1 ++ "," ++ toString r.2.2.2)',
         '  for s in charCases do',
         '    let r := fCharProbe s',
         '    IO.println ("chars " ++ String.intercalate "," (r.1.map toString) ++ ";" ++ toString r.2.1 ++ ";" ++ toString r.2.2.1 ++ ";" ++ hx r.2.2.2.1 ++ ";" ++ (if r.2.2.2.2 then "true" else "false"))',
         '  for xs in aliasCases do',
         '    let r := fAliasProbe xs',
         '    IO.println ("alias " ++ String.intercalate "," (r.1.map toString) ++ ";" ++ toString r.2)',
         '  for (xs, limit) in eagerCases do',
         '    let r := fEagerProbe xs limit',
         '    IO.println ("eager " ++ optNat r.1 ++ ";" ++ String.intercalate "," (r.2.1.map toString) ++ ";" ++ String.intercalate "," (r.2.2.1.map toString) ++ ";" ++ optNat r.2.2.2.1 ++ ";" ++ toString r.2.2.2.2)',
         '']
open('lean/ProbeMain.lean', 'w', newline='\n', encoding='utf-8').write('\n'.join(lean))
print(len(out), 'cases')

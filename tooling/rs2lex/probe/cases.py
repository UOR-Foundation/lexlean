"""The keyed-map probe: seeded operation sequences for run_nat, run_pair and run_tail, written as a Lean driver
(lean/ProbeMain.lean) and as the outcomes a BTreeMap gives (probe_expected.txt). Keys reach both ends of u64 and are
decimal prefixes of one another (1, 10, 100, 19), where a wrong padding would reorder them."""
import random
import sys

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
# named_probe: named closures that write and that return early; find, fold, all and and_then with such closures
named_cases = [[rng.randrange(0, 10) for _ in range(rng.randrange(0, 7))] for _ in range(150)]
for xs in named_cases:
    count = 1 + sum(xs)
    picked = sum(100 if x > 3 else x for x in xs)
    looked, found = 0, None
    for x in xs:
        looked += 1
        if x > 4: found = x; break
    folded = 0
    for x in xs:
        if x != 0: folded = folded * 2 + x
    small = all(x <= 7 and x != 5 for x in xs)
    coded = 1000 if not xs else (1000 + xs[0] if xs[0] > 5 else xs[0] + 1)
    out.append('named ' + ';'.join([str(count), str(picked), opt_nat(found), str(folded * 1000 + len(xs) * 100 + looked), str(small).lower(), str(coded)]))
# shadow_probe: variables bound again in nested scopes, in an if-let, and by a later let; a format capture after it
shadow_cases = [([rng.randrange(0, 9) for _ in range(rng.randrange(0, 6))],
                 rng.choice([None, [], [rng.randrange(0, 9) for _ in range(rng.randrange(1, 4))]])) for _ in range(150)]
for xs, w in shadow_cases:
    n = (1 if len(xs) > 1 else 0) + sum(x + 1 for x in xs)
    v = xs + [n]
    extra = 0 if w is None else len(w)
    n2 = n + (0 if w is None else len(w) + 1) + 1
    out.append('shadow ' + str(n2) + ';' + ','.join(map(str, v)) + ';' + str(len(xs) + extra) + ';' + str(len(str(n2) + str(len(xs)))))
# index_probe: `map[&key]`, and <= and >= on strings (byte order)
index_cases = [([rng.choice(WORDS + ['B', 'aB', '\u00e9']) for _ in range(rng.randrange(0, 6))], rng.randrange(0, 9)) for _ in range(150)]
for keys, at in index_cases:
    first = 0 if not keys else max(i for i, k in enumerate(keys) if k == keys[at % len(keys)])
    bs = [k.encode() for k in keys]
    out.append('index ' + str(first) + ';' + str(all(x <= y for x, y in zip(bs, bs[1:]))).lower() + ';' + str(any(x >= y for x, y in zip(bs, bs[1:]))).lower())
# entry_probe: the three shapes of the map entry API
entry_cases = [[rng.choice(WORDS) for _ in range(rng.randrange(0, 8))] for _ in range(150)]
for ws in entry_cases:
    keys = sorted(set(ws), key=lambda w: w.encode())
    counts = ','.join(hx(k) + '=' + str(ws.count(k)) for k in keys)
    places = ','.join(hx(k) + '=' + '.'.join(str(i) for i, w in enumerate(ws) if w == k) for k in keys)
    out.append('entry ' + counts + ';' + places + ';' + str(sum(ws.index(k) for k in keys)))
# map_probe: extend replaces earlier entries; get_key_value
def some_pairs(): return [(rng.choice(WORDS), rng.randrange(0, 9)) for _ in range(rng.randrange(0, 5))]
map_cases = [(some_pairs(), some_pairs(), rng.choice(WORDS)) for _ in range(150)]
for pairs, more, ask in map_cases:
    table = dict(pairs); table.update({k: v + 100 for k, v in more})
    items = sorted(table.items(), key=lambda kv: kv[0].encode())
    found = 'N' if ask not in table else 'S' + hx(ask) + '=' + str(table[ask])
    out.append('map ' + ','.join(hx(k) + '=' + str(v) for k, v in items) + ';' + found)
# write_probe: fmt::Write into a string through a &mut parameter, with ?
write_cases = [[rng.choice(WORDS + ['\u00e9', 'x y']) for _ in range(rng.randrange(0, 6))] for _ in range(150)]
for parts in write_cases:
    out.append('write ' + hx('>' + ''.join(part + '|' for part in parts)) + ';true')
# display_probe: a recursive Display implementation, reached through write! and to_string
def shape_of(codes, depth):
    if not codes: return ('L', depth)
    if len(codes) == 1: return ('L', codes[0])
    x, rest = codes[0], codes[1:]
    if x % 3 == 0 and depth < 4: return ('P', ('L', x), shape_of(rest, depth + 1))
    if x % 3 == 1 and depth < 4: return ('M', [shape_of(rest, depth + 1), ('L', x)])
    return shape_of(rest, depth + 1)
def shown(sh):
    if sh[0] == 'L': return str(sh[1])
    if sh[0] == 'P': return '(' + shown(sh[1]) + ' ' + shown(sh[2]) + ')'
    return '[' + ''.join(shown(i) + ';' for i in sh[1]) + ']'
display_cases = [[rng.randrange(0, 12) for _ in range(rng.randrange(0, 9))] for _ in range(150)]
for codes in display_cases:
    out.append('display ' + hx(shown(shape_of(codes, 0))))
# extend_probe: a map extended by a hash map given whole
for pairs, more, _ in map_cases:
    d = dict(pairs); d.update(dict(more))
    out.append('extend ' + ','.join(f'{hx(k)}={v}' for k, v in sorted(d.items(), key=lambda kv: kv[0].encode())))
# nested_collect_probe: a collected map that writes a capture, inside another
nested_cases = [[[rng.randrange(0, 9) for _ in range(rng.randrange(0, 4))] for _ in range(rng.randrange(0, 4))] for _ in range(150)]
for rows in nested_cases:
    count, made = 0, []
    for row in rows:
        count += 1; got = []
        for x in row:
            count += x; got.append(x + count)
        made.append(got)
    out.append('nested ' + '|'.join(','.join(map(str, r)) for r in made) + ';' + str(count))
# once_probe: a table computed once
once_cases = list(range(12))
for n in once_cases: out.append('once ' + str((n % 5) ** 2 + 5))
# convert_probe: `?` through a From impl
convert_cases = list(range(8))
for n in convert_cases: out.append('convert ' + (f'{2 * (n + 1)};0;' if n <= 3 else f'0;7;{hx("big " + str(n))}'))
# shift_probe: literal shifts, and an `impl Display` parameter at two types
shift_cases = [0, 1, 7, 1000, 123456]
for n in shift_cases: out.append('shift ' + str(n + 49152 + 4294967296) + ';' + hx('label: x') + ';' + hx('label: ' + str(n)))
# collect_map_probe: pairs collected into a map
for pairs, _, _ in map_cases:
    d = {}
    for k, v in pairs: d[k] = v + 1
    out.append('collectmap ' + ','.join(f'{hx(k)}={v}' for k, v in sorted(d.items(), key=lambda kv: kv[0].encode())) + ';' + str(len(d)))
# range_probe: code points, ranges as iterators, an iterator of one item
range_cases = [(t, k) for k, t in enumerate(['', 'a', 'Az09 ~', '\u00e9', 'x\u00e9y', '\u20ac', '\u4e2d\u6587', '\U0001F600', 'a\U0001F600\u00e9\u20ac', '\x7f\u0080\u07ff\u0800\uffff\U00010000\U0010ffff'])]
for t, k in range_cases:
    js = lambda xs: ','.join(map(str, xs))
    out.append('range ' + js([ord(c) for c in t]) + ';' + js(reversed(range(k))) + ';' + js(list(range(1, k + 1)) + [99]) + ';' + js([x for x in range(k) if x % 2 == 0]))
# wide_probe: i128 arithmetic of either sign
def some_u64(): return rng.choice([0, 1, 2, 3, 7, 10, 255, 2**32, 2**63 - 1, 2**63, 2**64 - 1, rng.randrange(0, 1000), rng.randrange(0, 2**64)])
wide_cases = [(rng.random() < 0.5, some_u64(), rng.random() < 0.5, max(1, some_u64()), rng.choice([0, 1, 8, 31, 32, 63, 64, 100, 126])) for _ in range(300)]
for na, x, nb, y, bits in wide_cases:
    a, b = (-x if na else x), (-y if nb else y)
    q = abs(a) // abs(b) * (-1 if (a < 0) != (b < 0) else 1)
    out.append('wide ' + hx(';'.join(map(str, [a + b, a - b, a * b, q, a - q * b, a % abs(b), (1 << bits) - a]))))
# hex_probe: hexadecimal and zero-padded numbers
hex_cases = [(n, t) for n, (t, _) in zip([0, 1, 9, 10, 15, 16, 255, 256, 4095, 4096, 65535, 65536, 1048576, 3735928559, 2**40 + 11], range_cases + range_cases)]
for n, t in hex_cases:
    out.append('hex ' + hx(f'{n:04X}|{n:04x}|{n:02x}|{n:x}|{n:03}|' + ''.join(f'\\u{ord(c):04x}' for c in t)))
# loop_probe: a loop left only by return, a temporary written, Ok(pairs.collect())
loop_cases = [([rng.randrange(0, 9) for _ in range(rng.randrange(0, 4))], rng.randrange(0, 4), [rng.choice(WORDS) for _ in range(rng.randrange(0, 4))]) for _ in range(150)]
for xs, depth, names in loop_cases:
    binders = [x + d for d in range(depth, 0, -1) for x in xs]
    if len(names) == len(xs):
        d = {}
        for k, v in zip(names, xs): d[k] = v
        m, err = sorted(d.items(), key=lambda kv: kv[0].encode()), ''
    else: m, err = [], f'{len(names)} and {len(xs)}'
    out.append('loop ' + ','.join(map(str, binders)) + ';' + str(len(xs)) + ';true;' + ','.join(f'{hx(k)}={v}' for k, v in m) + ';' + hx(err))
# set_probe: a set iterated in its own order
set_cases = [([rng.choice(WORDS + ['B', 'aB', '\u00e9']) for _ in range(rng.randrange(0, 6))], [rng.randrange(0, 9) for _ in range(rng.randrange(0, 6))]) for _ in range(150)]
for ws, xs in set_cases:
    ps = sorted(set(zip(xs, ws)), key=lambda p: (p[0], p[1].encode()))
    out.append('set ' + ','.join(hx(w) for w in sorted(set(ws), key=lambda w: w.encode())) + ';' + ','.join(map(str, sorted(set(xs)))) + ';' + ','.join(f'{x}.{hx(w)}' for x, w in ps))
# getmut_probe: a value changed through get_mut, on a map and on a list
getmut_cases = [([rng.choice(WORDS + ['B', 'aB']) for _ in range(rng.randrange(0, 6))], [rng.randrange(0, 9) for _ in range(rng.randrange(0, 5))], rng.randrange(0, 6)) for _ in range(150)]
for ws, xs, at in getmut_cases:
    d = {}
    for w in ws:
        if len(w.encode()) > 1: d[w] = 0
    for w in ws:
        if w in d: d[w] += len(w.encode())
    v, hit = list(xs), 0
    if at < len(v):
        v[at] *= 10
        if v[at] > 50: hit = 1
        else: v[at] += 1
    out.append('getmut ' + ','.join(f'{hx(k)}={n}' for k, n in sorted(d.items(), key=lambda kv: kv[0].encode())) + ';' + ','.join(map(str, v)) + ';' + str(hit))
# misc_probe: rsplit, conversions, a bool shown, a set from an array
misc_cases = [(t, rng.random() < 0.5, rng.choice(['', 'q', '\u00e9t', '\U0001F600'])) for t in ['', 'a', 'a::b', 'a:::b', '::', 'a::::b::c', 'aaa', 'aaaa', 'baaab', 'x,y,,z', ',', 'aa::aa,aa'] for _ in range(3)]
def rsplit(t, p):
    out, rest = [], t
    while True:
        i = rest.rfind(p)
        if i < 0: out.append(rest); return out
        out.append(rest[i + len(p):]); rest = rest[:i]
for t, b, c in misc_cases:
    first = c[0] if c else 'x'
    out.append('misc ' + '|'.join(hx(x) for x in rsplit(t, '::')) + ';' + '|'.join(hx(x) for x in rsplit(t, 'aa')) + ';' + '|'.join(hx(x) for x in rsplit(t, ',')) + ';' + str(int(b)) + ';' + str(ord(first)) + ';' + ('true' if b else 'false') + ';' + str(len({t, c})))
# closure_exit_probe: closures that leave themselves early
for rows in nested_cases:
    halves = [r[0] // 2 + len(r) for r in rows if r and r[0] % 2 == 0]
    capped = []
    for r in rows:
        t = 0
        for x in r:
            t += x
            if t > 12: t = 12; break
        capped.append(t)
    big = any(x > 7 for r in rows for x in r)
    out.append('cexit ' + ','.join(map(str, halves)) + ';' + ','.join(map(str, capped)) + ';' + ('true' if big else 'false'))
# search_probe: a binary search on a sorted constant
search_cases = ['', 'a', 'ab', 'abc', 'b', 'c', 'do', 'fun', 'z', 'zz', 'A']
for w in search_cases:
    hit = w in ['a', 'abc', 'b', 'do', 'fun', 'z']
    out.append('search ' + ('true' if hit else 'false') + ';' + ('false' if hit else 'true'))
out.append('tag ' + hx('core::eq')); out.append('tag ' + hx('core::exists-unique'))
# free_probe: a closure's &mut parameters standing for the caller's sets
def build_bound(xs, depth):
    if not xs: return ('var', f'v{depth % 3}')
    x, rest = xs[0], xs[1:]; name = f'v{(x // 3) % 3}'
    if x % 3 == 0: return ('bind', name, build_bound(rest, depth + 1))
    if x % 3 == 1: return ('pair', ('var', name), build_bound(rest, depth + 1))
    return ('pair', build_bound(rest, depth + 1), ('var', name))
def free_of(t, bound, out):
    if t[0] == 'var':
        if t[1] not in bound: out.add(t[1])
    elif t[0] == 'bind':
        added = t[1] not in bound; bound.add(t[1]); free_of(t[2], bound, out)
        if added: bound.discard(t[1])
    else: free_of(t[1], bound, out); free_of(t[2], bound, out)
free_cases = [[rng.randrange(0, 12) for _ in range(rng.randrange(0, 7))] for _ in range(200)]
for xs in free_cases:
    o = set(); free_of(build_bound(xs, 0), set(), o)
    out.append('free ' + ','.join(sorted(o)))
for xs in free_cases:
    out.append('cmut ' + ','.join(map(str, xs + [1, len(xs)])))
chunk_cases = [(rng.choice(['', 'a', 'abc', 'banana', 'xaéay', 'aaa']), [rng.randrange(0, 9) for _ in range(rng.randrange(0, 7))], [rng.choice(['', 'b', 'n', 'bn', 'x', 'y']) for _ in range(rng.randrange(0, 4))]) for _ in range(100)]
for t, xs, keep in chunk_cases:
    pairs = [xs[i] * 10 + xs[i + 1] for i in range(0, len(xs) - 1, 2)]
    kept = sorted({w for w in t.split('a') if w in keep}, key=lambda w: w.encode())
    out.append('chunk ' + hx('_y' + t[2:]) + ';' + ','.join(map(str, pairs)) + ';' + ','.join(hx(w) for w in kept))
for n in convert_cases: out.append('into ' + (f'{n};0;' if 0 < n <= 3 else '0;7;' + hx('zero' if n == 0 else f'big {n}')))
# writeback_probe: writes through a `&mut` to an element, on every way out of the body
writeback_cases = [([rng.randrange(0, 9) for _ in range(rng.randrange(0, 7))], rng.randrange(0, 9)) for _ in range(200)]
for xs, _ in writeback_cases:
    out.append('fold ' + str(1 + sum(xs)) + ';' + str(max([0] + xs)) + ';' + str(min([99] + xs)))
for xs, stop in writeback_cases:
    v, seen = list(xs), 0
    for i, x in enumerate(v):
        if x == stop: v[i] = 100; break
        if x % 2 == 0: v[i] = x + 1; continue
        v[i] = x * 2; seen += 1
    rows = []
    for x in xs:
        if x > 5: rows.append([])
        if rows:
            rows[-1].append(x)
            if x == 3: continue
            rows[-1].append(0)
        else: seen += 100
    for row in rows: row.append(seen)
    w, hit = list(xs), 0
    for i, x in enumerate(w):
        if x > stop: w[i] = stop; hit = 1; break
        w[i] = x + 10
    out.append('writeback ' + ','.join(map(str, v)) + ';' + '|'.join(','.join(map(str, r)) for r in rows) + ';' + str(seen) + ';' + ','.join(map(str, w)) + ';' + str(hit))
# typed_probe: a value made by a call that borrows mutably, then written (only with the typed front end: `typed`)
typed_cases = [[rng.randrange(0, 9) for _ in range(rng.randrange(0, 6))] for _ in range(100)] if 'typed' in sys.argv else []
for xs in typed_cases:
    v = xs + [len(xs)]
    out.append('typed ' + str(2 * len(v) + 1) + ';' + ','.join(map(str, v)) + ';' + ','.join(map(str, v + [7])))
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
lean.append('def namedCases : List (List Nat) := ' + lean_list([lean_list([str(x) for x in xs]) for xs in named_cases]))
lean.append('def shadowCases : List (List Nat × Option (List Nat)) := ' + lean_list([f'({lean_list([str(x) for x in xs])}, {lean_opt_list(w)})' for xs, w in shadow_cases]))
lean.append('def indexCases : List (List ByteArray × Nat) := ' + lean_list([f'({lean_list([lean_bytes(k) for k in keys])}, {at})' for keys, at in index_cases]))
lean.append('def entryCases : List (List ByteArray) := ' + lean_list([lean_list([lean_bytes(w) for w in ws]) for ws in entry_cases]))
def lean_pairs(ps): return lean_list([f'({lean_bytes(k)}, {v})' for k, v in ps])
lean.append('def mapCases : List (List (ByteArray × Nat) × List (ByteArray × Nat) × ByteArray) := ' + lean_list([f'({lean_pairs(a_)}, {lean_pairs(b_)}, {lean_bytes(q)})' for a_, b_, q in map_cases]))
lean.append('def writeCases : List (List ByteArray) := ' + lean_list([lean_list([lean_bytes(w) for w in ws]) for ws in write_cases]))
lean.append('def displayCases : List (List Nat) := ' + lean_list([lean_list([str(x) for x in xs]) for xs in display_cases]))
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
         '  for xs in namedCases do',
         '    let r := fNamedProbe xs',
         '    IO.println ("named " ++ toString r.1 ++ ";" ++ toString r.2.1 ++ ";" ++ optNat r.2.2.1 ++ ";" ++ toString r.2.2.2.1 ++ ";" ++ (if r.2.2.2.2.1 then "true" else "false") ++ ";" ++ toString r.2.2.2.2.2)',
         '  for (xs, w) in shadowCases do',
         '    let r := fShadowProbe xs w',
         '    IO.println ("shadow " ++ toString r.1 ++ ";" ++ String.intercalate "," (r.2.1.map toString) ++ ";" ++ toString r.2.2.1 ++ ";" ++ toString r.2.2.2)',
         '  for (keys, pos) in indexCases do',
         '    let r := fIndexProbe keys pos',
         '    IO.println ("index " ++ toString r.1 ++ ";" ++ (if r.2.1 then "true" else "false") ++ ";" ++ (if r.2.2 then "true" else "false"))',
         '  for ws in entryCases do',
         '    let r := fEntryProbe ws',
         '    IO.println ("entry " ++ String.intercalate "," (r.1.map fun e => hx e.1 ++ "=" ++ toString e.2) ++ ";" ++ String.intercalate "," (r.2.1.map fun e => hx e.1 ++ "=" ++ String.intercalate "." (e.2.map toString)) ++ ";" ++ toString r.2.2)',
         '  for (ps, more, ask) in mapCases do',
         '    let r := fMapProbe ps more ask',
         '    IO.println ("map " ++ String.intercalate "," (r.1.map fun e => hx e.1 ++ "=" ++ toString e.2) ++ ";" ++ (match r.2 with | some e => "S" ++ hx e.1 ++ "=" ++ toString e.2 | none => "N"))',
         '  for ws in writeCases do',
         '    let r := fWriteProbe ws',
         '    IO.println ("write " ++ hx r.1 ++ ";" ++ (if r.2 then "true" else "false"))',
         '  for xs in displayCases do',
         '    IO.println ("display " ++ hx (fDisplayProbe xs))',
         '  for (ps, more, _) in mapCases do',
         '    IO.println ("extend " ++ String.intercalate "," ((fExtendProbe ps more).map fun e => hx e.1 ++ "=" ++ toString e.2))',
         '  for rows in nestedCases do',
         '    let r := fNestedCollectProbe rows',
         '    IO.println ("nested " ++ String.intercalate "|" (r.1.map fun row => String.intercalate "," (row.map toString)) ++ ";" ++ toString r.2)',
         '  for n in [' + ', '.join(str(n) for n in once_cases) + '] do',
         '    IO.println ("once " ++ toString (fOnceProbe n))',
         '  for n in [' + ', '.join(str(n) for n in convert_cases) + '] do',
         '    let r := fConvertProbe n',
         '    IO.println ("convert " ++ toString r.1 ++ ";" ++ toString r.2.1 ++ ";" ++ hx r.2.2)',
         '  for n in [' + ', '.join(str(n) for n in shift_cases) + '] do',
         '    let r := fShiftProbe n',
         '    IO.println ("shift " ++ toString r.1 ++ ";" ++ hx r.2.1 ++ ";" ++ hx r.2.2)',
         '  for (ps, _, _) in mapCases do',
         '    let r := fCollectMapProbe ps',
         '    IO.println ("collectmap " ++ String.intercalate "," (r.1.map fun e => hx e.1 ++ "=" ++ toString e.2) ++ ";" ++ toString r.2)',
         '  for (t, k) in rangeCases do',
         '    let r := fRangeProbe t k',
         '    let js := fun (xs : List Nat) => String.intercalate "," (xs.map toString)',
         '    IO.println ("range " ++ js r.1 ++ ";" ++ js r.2.1 ++ ";" ++ js r.2.2.1 ++ ";" ++ js r.2.2.2)',
         '  for (na, x, nb, y, bits) in wideCases do',
         '    IO.println ("wide " ++ hx (fWideProbe na x nb y bits))',
         '  for (n, t) in hexCases do',
         '    IO.println ("hex " ++ hx (fHexProbe n t))',
         '  for (xs, depth, names) in loopCases do',
         '    let r := fLoopProbe xs depth names',
         '    IO.println ("loop " ++ String.intercalate "," (r.1.map toString) ++ ";" ++ toString r.2.1 ++ ";" ++ (if r.2.2.1 then "true" else "false") ++ ";" ++ String.intercalate "," (r.2.2.2.1.map fun e => hx e.1 ++ "=" ++ toString e.2) ++ ";" ++ hx r.2.2.2.2)',
         '  for (ws, xs) in setCases do',
         '    let r := fSetProbe ws xs',
         '    IO.println ("set " ++ String.intercalate "," (r.1.map hx) ++ ";" ++ String.intercalate "," (r.2.1.map toString) ++ ";" ++ String.intercalate "," (r.2.2.map fun p => toString p.1 ++ "." ++ hx p.2))',
         '  for (ws, xs, pos) in getmutCases do',
         '    let r := fGetmutProbe ws xs pos',
         '    IO.println ("getmut " ++ String.intercalate "," (r.1.map fun e => hx e.1 ++ "=" ++ toString e.2) ++ ";" ++ String.intercalate "," (r.2.1.map toString) ++ ";" ++ toString r.2.2)',
         '  for (t, b, c) in miscCases do',
         '    let r := fMiscProbe t b c',
         '    IO.println ("misc " ++ String.intercalate "|" (r.1.map hx) ++ ";" ++ String.intercalate "|" (r.2.1.map hx) ++ ";" ++ String.intercalate "|" (r.2.2.1.map hx) ++ ";" ++ toString r.2.2.2.1 ++ ";" ++ toString r.2.2.2.2.1 ++ ";" ++ String.fromUTF8! r.2.2.2.2.2.1 ++ ";" ++ toString r.2.2.2.2.2.2)',
         '  for rows in nestedCases do',
         '    let r := fClosureExitProbe rows',
         '    IO.println ("cexit " ++ String.intercalate "," (r.1.map toString) ++ ";" ++ String.intercalate "," (r.2.1.map toString) ++ ";" ++ (if r.2.2 then "true" else "false"))',
         '  for w in [' + ', '.join(lean_bytes(w) for w in search_cases) + '] do',
         '    let r := fSearchProbe w',
         '    IO.println ("search " ++ (if r.1 then "true" else "false") ++ ";" ++ (if r.2 then "true" else "false"))',
         '  for b in [true, false] do',
         '    IO.println ("tag " ++ hx (fTagProbe b))',
         '  for xs in freeCases do',
         '    IO.println ("free " ++ String.intercalate "," ((fFreeProbe xs).map fun b => String.fromUTF8! b))',
         '  for xs in freeCases do',
         '    IO.println ("cmut " ++ String.intercalate "," ((fClosureMutProbe xs).map toString))',
         '  for (t, xs, keep) in chunkCases do',
         '    let r := fChunkProbe t xs keep',
         '    IO.println ("chunk " ++ hx r.1 ++ ";" ++ String.intercalate "," (r.2.1.map toString) ++ ";" ++ String.intercalate "," (r.2.2.map hx))',
         '  for n in [' + ', '.join(str(n) for n in convert_cases) + '] do',
         '    let r := fIntoProbe n',
         '    IO.println ("into " ++ toString r.1 ++ ";" ++ toString r.2.1 ++ ";" ++ hx r.2.2)',
         '  for (xs, _) in writebackCases do',
         '    let r := fFoldProbe xs',
         '    IO.println ("fold " ++ toString r.1 ++ ";" ++ toString r.2.1 ++ ";" ++ toString r.2.2)',
         '  for (xs, stop) in writebackCases do',
         '    let r := fWritebackProbe xs stop',
         '    IO.println ("writeback " ++ String.intercalate "," (r.1.map toString) ++ ";" ++ String.intercalate "|" (r.2.1.map fun row => String.intercalate "," (row.map toString)) ++ ";" ++ toString r.2.2.1 ++ ";" ++ String.intercalate "," (r.2.2.2.1.map toString) ++ ";" ++ toString r.2.2.2.2)',
         '']
at = next(i for i, line in enumerate(lean) if line.startswith('def shadowCases'))
lean.insert(at, 'def nestedCases : List (List (List Nat)) := ' + lean_list([lean_list([lean_list([str(x) for x in row]) for row in rows]) for rows in nested_cases]))
lean.insert(at, 'def rangeCases : List (ByteArray × Nat) := ' + lean_list([f'({lean_bytes(t)}, {k})' for t, k in range_cases]))
lean.insert(at, 'def wideCases : List (Bool × Nat × Bool × Nat × Nat) := ' + lean_list([f'({str(na).lower()}, {x}, {str(nb).lower()}, {y}, {bits})' for na, x, nb, y, bits in wide_cases]))
lean.insert(at, 'def hexCases : List (Nat × ByteArray) := ' + lean_list([f'({n}, {lean_bytes(t)})' for n, t in hex_cases]))
lean.insert(at, 'def loopCases : List (List Nat × Nat × List ByteArray) := ' + lean_list([f'({lean_list([str(x) for x in xs])}, {d}, {lean_list([lean_bytes(n) for n in ns])})' for xs, d, ns in loop_cases]))
lean.insert(at, 'def setCases : List (List ByteArray × List Nat) := ' + lean_list([f'({lean_list([lean_bytes(w) for w in ws])}, {lean_list([str(x) for x in xs])})' for ws, xs in set_cases]))
lean.insert(at, 'def getmutCases : List (List ByteArray × List Nat × Nat) := ' + lean_list([f'({lean_list([lean_bytes(w) for w in ws])}, {lean_list([str(x) for x in xs])}, {a})' for ws, xs, a in getmut_cases]))
lean.insert(at, 'def miscCases : List (ByteArray × Bool × ByteArray) := ' + lean_list([f'({lean_bytes(t)}, {str(b).lower()}, {lean_bytes(c)})' for t, b, c in misc_cases]))
lean.insert(at, 'def freeCases : List (List Nat) := ' + lean_list([lean_list([str(x) for x in xs]) for xs in free_cases]))
lean.insert(at, 'def chunkCases : List (ByteArray × List Nat × List ByteArray) := ' + lean_list([f'({lean_bytes(t)}, {lean_list([str(x) for x in xs])}, {lean_list([lean_bytes(w) for w in k])})' for t, xs, k in chunk_cases]))
lean.insert(at, 'def writebackCases : List (List Nat × Nat) := ' + lean_list([f'({lean_list([str(x) for x in xs])}, {stop})' for xs, stop in writeback_cases]))
if typed_cases:
    lean[-1:] = ['  for xs in typedCases do',
                 '    let r := fTypedProbe xs',
                 '    IO.println ("typed " ++ toString r.1 ++ ";" ++ String.intercalate "," (r.2.1.map toString) ++ ";" ++ String.intercalate "," (r.2.2.map toString))',
                 '']
    at = next(i for i, line in enumerate(lean) if line.startswith('def shadowCases'))
    lean.insert(at, 'def typedCases : List (List Nat) := ' + lean_list([lean_list([str(x) for x in xs]) for xs in typed_cases]))
open('lean/ProbeMain.lean', 'w', newline='\n', encoding='utf-8').write('\n'.join(lean))
print(len(out), 'cases')

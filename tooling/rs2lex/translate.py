"""Translate a Rust crate's functions to one language-1.2 module, and report what translated and why the rest did not.

    python tooling/rs2lex/translate.py [CRATE_SRC] [OUT_DIR]

CRATE_SRC defaults to crates/lexlean/src and OUT_DIR to target/rs2lex. OUT_DIR receives:

  Crate.lex.tex       the module: every function that translated, with the types and helpers it needs
  report.json         one row per function: its file, its size in bytes, and `ok` or the reason it did not translate
  summary.txt         the totals and the root causes, each with everything it blocks through its callers

The translation runs twice. The first pass learns which functions each function really calls; the second uses that
call graph for recursion groups, since grouping by method name alone joins functions that never call each other."""
import collections, json, os, re, sys, time, traceback

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.setrecursionlimit(20000)
import crate_load      # noqa: E402
import thir_ast        # noqa: E402
import unshadow        # noqa: E402
import rs2lex          # noqa: E402

IMPORTS = ['AxiomOutput', 'RustStd', 'LexLeanStd.Text', 'LexLeanStd.OrdMap']


def _translator(items, aliases):
    tr = rs2lex.Translator(items, aliases)
    orig = tr.function

    def guarded(f):
        try:
            return orig(f)
        except rs2lex.Unsupported:
            raise
        except RecursionError:
            raise rs2lex.Unsupported('translator: recursion depth')
        except Exception as x:          # a defect of the translator, reported against the function, not a crash
            frame = traceback.extract_tb(x.__traceback__)[-1]
            raise rs2lex.Unsupported(f'translator crash: {type(x).__name__} at line {frame.lineno}')
    tr.function = guarded
    return tr


def local_const(e):
    if isinstance(e, list): return any(local_const(x) for x in e)
    if not isinstance(e, tuple) or not e: return False
    if e[0] == 'let' and len(e) == 5 and isinstance(e[1], tuple) and e[1][:1] == ('pid',) and e[1][1][:1].isupper(): return True
    return any(local_const(x) for x in e if isinstance(x, (tuple, list)))


def renamed_calls(e, names):
    if isinstance(e, list): return [renamed_calls(x, names) for x in e]
    if not isinstance(e, tuple) or not e: return e
    if e[0] == 'path' and isinstance(e[1], list) and len(e[1]) == 1 and e[1][0] in names: return ('path', [names[e[1][0]]])
    return tuple(renamed_calls(x, names) if isinstance(x, (tuple, list)) else x for x in e)


def run(root, module='Crate', thir_dump=None, thir_root='.'):
    """(rows, text, seconds, translator) for the crate at `root`. With `thir_dump`, the dump rustc made of the same
    crate (see thir.py), a function's body is taken from rustc's typed tree where thir_ast converts it, and from the
    source parser where it does not; `run.thir` then holds how many came from each, and why not. `thir_root` is the
    folder rustc ran in, which the file names in its spans are relative to."""
    t0 = time.time()
    converted = {}
    if thir_dump:
        first = crate_load.load(root, with_aliases=True)[0]
        converted = thir_ast.convert(thir_dump, items=first, src_root=thir_root, resolver=crate_load.load.resolver)
    run.thir = collections.Counter()
    def load():
        items, skipped, aliases = crate_load.load(root, with_aliases=True)
        if not converted: return items, skipped, aliases
        by_place = collections.defaultdict(list)
        for path, got in converted.items():
            segs = [x for x in path.split('::') if not x.startswith('{')]
            by_place[(tuple(segs[:-1]), segs[-1])].append(got)
        for it in items:
            if it['k'] != 'fn' or it.get('lifted_from'): continue
            name = it.get('orig_name', it['name'])
            got = by_place.get((tuple(it['mp']), 'fmt' if name in ('fmt_display', 'fmt_debug') else name), [])
            if len(got) != 1: run.thir['no single body in the dump'] += 1; continue
            if isinstance(got[0], str): run.thir[got[0]] += 1; continue
            params, body = got[0]
            names = [p[1] if p[0] == 'pid' else None for p in params]
            if names != [n for n, _ in it['params']]: run.thir['parameters that are patterns'] += 1; continue
            # an item declared in the body is no part of rustc's tree of it: a constant stays with the source's
            # body, and a nested function, which the loader lifted out, is called by the name it was lifted to
            if local_const(it['body']): run.thir['a constant declared in the body'] += 1; continue
            nested = {x['name'][len(it['name']) + 1:]: x['name'] for x in items if x.get('lifted_from') == it['name']}
            it['body'] = renamed_calls(body, nested) if nested else body
            it['body'] = unshadow.entries(it); it.pop('refuse', None)
            if unshadow.written_alias(it): it['refuse'] = 'a write through a reference bound by a pattern or a let'
            run.thir['from rustc'] += 1
        return items, skipped, aliases
    items, skipped, aliases = load()
    tr = _translator(items, aliases)
    tr.module(module, IMPORTS)
    exact = dict(tr.key_calls)
    for k in tr.fns:
        if k not in exact and tr.status.get(k) == 'ok':
            exact[k] = set()
    items, skipped, aliases = load()
    tr = _translator(items, aliases)
    tr.exact_deps = exact
    text, _report = tr.module(module, IMPORTS)
    fns = {(f['owner'], f['name']): f for f in items if f['k'] == 'fn' and not f.get('generics')}
    rows = []
    for key, f in fns.items():
        rows.append({'key': f'{key[0]}::{key[1]}' if key[0] else key[1], 'file': f['file'], 'bytes': f.get('bytes', 0),
                     'status': tr.status.get(key, 'UNSUPPORTED: not reached')})
    for rel, name, kw, nbytes, why in skipped:
        if kw == 'fn' or why.startswith('trait impl') or why.startswith('method of'):
            rows.append({'key': name, 'file': rel, 'bytes': nbytes, 'status': f'SKIPPED: {why}'})
    return rows, text, time.time() - t0, tr


def cause(st):
    if st == 'ok':
        return 'ok'
    s = st.split(': ', 1)[1] if ': ' in st else st
    if s.startswith('calls `') or 'recursion group failed' in s:
        return 'blocked by a callee'
    s = re.sub(r'`[^`]*`', '`X`', s)
    s = re.sub(r'\d+', 'N', s)
    return s[:90]


def summary(rows, tr, elapsed, text):
    total = sum(r['bytes'] for r in rows) or 1
    ok = [r for r in rows if r['status'] == 'ok']
    out = [f'{len(rows)} functions, {total:,} bytes; translated {len(ok)} ({100 * len(ok) / len(rows):.1f}%), '
           f'{sum(r["bytes"] for r in ok):,} bytes ({100 * sum(r["bytes"] for r in ok) / total:.1f}%); '
           f'{elapsed:.0f}s; output {len(text):,} bytes']
    status = {k: tr.status.get(k) for k in tr.fns}
    sizes = {k: f.get('bytes', 0) for k, f in tr.fns.items()}

    def root(k):
        seen = set()
        while k in tr.blocked_by and k not in seen:
            seen.add(k); k = tr.blocked_by[k]
        return k
    blame, blame_n = collections.Counter(), collections.Counter()
    for k in tr.fns:
        st = status.get(k)
        if st and st != 'ok':
            c = cause(status.get(root(k)) or 'UNSUPPORTED: unknown')
            blame[c] += sizes[k]; blame_n[c] += 1
    out.append('root causes, each with everything it blocks through its callers (functions, bytes):')
    out += [f'  {blame_n[c]:5d} {b:>10,}  {c}' for c, b in blame.most_common(40)]
    return '\n'.join(out) + '\n'


def main(argv):
    repo = os.path.abspath(os.path.join(HERE, '..', '..'))
    root = argv[1] if len(argv) > 1 else os.path.join(repo, 'crates', 'lexlean', 'src')
    out_dir = argv[2] if len(argv) > 2 else os.path.join(repo, 'target', 'rs2lex')
    os.makedirs(out_dir, exist_ok=True)
    dump = argv[3] if len(argv) > 3 else None              # rustc's dump of the same crate, and where rustc ran
    rows, text, elapsed, tr = run(root, thir_dump=dump, thir_root=argv[4] if len(argv) > 4 else '.')
    open(os.path.join(out_dir, 'Crate.lex.tex'), 'w', encoding='utf-8', newline='\n').write(text)
    json.dump(rows, open(os.path.join(out_dir, 'report.json'), 'w', encoding='utf-8'), indent=0)
    s = summary(rows, tr, elapsed, text)
    open(os.path.join(out_dir, 'summary.txt'), 'w', encoding='utf-8', newline='\n').write(s)
    print(s.splitlines()[0])
    if dump:
        json.dump(dict(run.thir), open(os.path.join(out_dir, 'thir.json'), 'w', encoding='utf-8'), indent=0)
        print('bodies from rustc:', run.thir['from rustc'], 'of', sum(run.thir.values()))


if __name__ == '__main__':
    main(sys.argv)

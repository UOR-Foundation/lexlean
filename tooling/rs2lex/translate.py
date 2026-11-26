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


def run(root, module='Crate'):
    """(rows, text, seconds, translator) for the crate at `root`."""
    t0 = time.time()
    items, skipped, aliases = crate_load.load(root, with_aliases=True)
    tr = _translator(items, aliases)
    tr.module(module, IMPORTS)
    exact = dict(tr.key_calls)
    for k in tr.fns:
        if k not in exact and tr.status.get(k) == 'ok':
            exact[k] = set()
    items, skipped, aliases = crate_load.load(root, with_aliases=True)
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
    rows, text, elapsed, tr = run(root)
    open(os.path.join(out_dir, 'Crate.lex.tex'), 'w', encoding='utf-8', newline='\n').write(text)
    json.dump(rows, open(os.path.join(out_dir, 'report.json'), 'w', encoding='utf-8'), indent=0)
    s = summary(rows, tr, elapsed, text)
    open(os.path.join(out_dir, 'summary.txt'), 'w', encoding='utf-8', newline='\n').write(s)
    print(s.splitlines()[0])


if __name__ == '__main__':
    main(sys.argv)

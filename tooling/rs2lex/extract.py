"""Cut a standalone module out of a translation: the named roots and every declaration they reach, in the crate's
own order, with only the roots renamed and the production attribute added to each root. This is how
`bootstrap/src/AxiomPolicy.lex.tex` (BS-74) was made from the translation of `crates/lexlean/src/verify/policy.rs`.

    python tooling/rs2lex/extract.py CRATE.lex.tex OUT.lex.tex MODULE root=newName[:effect,...] ...

Each definition states the union of the axioms the definitions it calls state, to a fixed point; an imported
module's definitions are read from its committed source in the bootstrap project (BOOT overrides the directory)."""
import os, re, sys

src = open(sys.argv[1], encoding='utf-8').read()
body = src.split('\\begin{semanticmodule}\n', 1)[1].rsplit('\\end{semanticmodule}', 1)[0]
blocks = [b.strip() + '\n}' for b in body.split('\n}\n') if b.strip()]
blocks[-1] = blocks[-1][:-2] if blocks[-1].endswith('}\n}') else blocks[-1]
decl = {}
order = []
for b in blocks:
    m = re.search(r'^\s*name: ([A-Za-z0-9_]+),', b, re.M)
    if not m: continue
    decl[m.group(1)] = b; order.append(m.group(1))

out_path, module = sys.argv[2], sys.argv[3]
roots = {}
for a in sys.argv[4:]:
    old, new = a.split('=')
    new, _, eff = new.partition(':')
    roots[old] = (new, [e for e in eff.split(',') if e])

ident = re.compile(r'[A-Za-z_][A-Za-z0-9_]*')
def refs(b):
    text = re.sub(r'"[^"]*"', '', b)
    return {t for t in ident.findall(text) if t in decl}

keep, pending = set(), list(roots)
while pending:
    n = pending.pop()
    if n in keep: continue
    keep.add(n)
    pending.extend(refs(decl[n]) - keep)

# Axioms: a definition states the union of what the definitions it calls state, to a fixed point. An imported
# module's definitions are read from its committed source (BOOT, the bootstrap project's src directory).
BOOT = os.environ.get('BOOT', os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'bootstrap', 'src'))
imported = {}
for mod_name in re.findall(r'\\importmodule\{([A-Za-z0-9_.]+)\}', src):
    path = os.path.join(BOOT, mod_name + '.lex.tex')
    if not os.path.exists(path): continue
    for b in open(path, encoding='utf-8').read().split('\ndefinition{')[1:]:
        n = re.search(r'name: ([A-Za-z0-9_]+),', b)
        a = re.search(r'\n  axioms: \[([^\]]*)\]', b.split('\n}', 1)[0])
        if n: imported[f'{mod_name}::{n.group(1)}'] = {x.strip() for x in a.group(1).split(',') if x.strip()} if a else set()
qual = re.compile(r'([A-Za-z][A-Za-z0-9_.]*::[A-Za-z_][A-Za-z0-9_]*)')
axioms = {n: set() for n in keep}
changed = True
while changed:
    changed = False
    for n in keep:
        text = re.sub(r'"[^"]*"', '', decl[n])
        new = set(axioms[n])
        for q in qual.findall(text): new |= imported.get(q, set())
        for r in refs(decl[n]) & keep: new |= axioms[r]
        if new != axioms[n]: axioms[n] = new; changed = True
def with_axioms(n, b):
    if not axioms[n] or not b.lstrip().startswith('definition{') or '\n  axioms:' in b: return b
    stated = ', '.join(sorted(axioms[n], key=lambda s: s.encode()))
    return b.replace('\n  executable: true', f'\n  axioms: [{stated}],\n  executable: true', 1)
decl = {n: with_axioms(n, b) if n in keep else b for n, b in decl.items()}

rename = {old: new for old, (new, _) in roots.items()}
def apply(b):
    return ident.sub(lambda m: rename.get(m.group(0), m.group(0)), b)

parts = []
for n in order:
    if n not in keep: continue
    b = apply(decl[n])
    if n in roots and roots[n][1]:
        eff = ', '.join(roots[n][1])
        b = b[:-2].rstrip() + f',\n  production: {{effects: [{eff}], targets: [rust-std]}}\n}}'
    parts.append(b)
head = src.split('\\begin{semanticmodule}\n', 1)[0].replace('{Crate}', '{' + module + '}')
for used in re.findall(r'\\importmodule\{([A-Za-z0-9_.]+)\}', head):      # an import nothing in the cut uses goes
    if not any(used + '::' in p for p in parts):
        head = head.replace('\\importmodule{' + used + '}\n', '')
open(out_path, 'w', encoding='utf-8', newline='\n').write(head + '\\begin{semanticmodule}\n' + '\n\n'.join(parts) + '\n\\end{semanticmodule}\n\\end{lexlean}\n')
print(len(keep), 'declarations:', ' '.join(n for n in order if n in keep))

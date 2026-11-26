"""Interior mutability as state threading.

Cell<T> and RefCell<T> are translated as T. A `&T` parameter (`&self` included) that is the root of a mutation can
only be mutated in Rust through interior mutability, so it is upgraded to `&mut T`, and the translator's `&mut` rule
then threads its value through every caller. The upgrade is a fixpoint over the call graph: passing such a parameter
to a callee whose parameter was upgraded is a mutation too.

Sound when the same cell is not reachable through two different references during one call (no aliasing), which is
the shape of a context object passed down; the differential against the Rust is what checks it. A mutation through
a local variable bound to part of a parameter (a held `borrow_mut()` guard, `let c = &self.cache`) is an alias the
threading would lose, so the function is marked refused instead."""
import collections
import rs2lex_core as core

THROUGH = {'borrow', 'borrow_mut', 'get_mut', 'as_mut', 'as_ref', 'deref', 'deref_mut'}
MUTATING = set(core.MUTATORS) | {'set', 'replace', 'take', 'borrow_mut', 'get_mut', 'update', 'swap', 'clear', 'entry'}

def walk(e):
    if isinstance(e, list):
        for x in e: yield from walk(x)
    elif isinstance(e, tuple) and e:
        yield e
        for x in e:
            if isinstance(x, (tuple, list)): yield from walk(x)

def root(a):
    while a and isinstance(a, tuple):
        if a[0] in ('mutref', 'deref', 'field', 'index'): a = a[1]; continue
        if a[0] == 'mcall' and a[2] in THROUGH and not a[3]: a = a[1]; continue
        break
    return a[1][0] if a and a[0] == 'path' and len(a[1]) == 1 else None

def infer(items):
    fns = [i for i in items if i['k'] == 'fn']
    by_name = collections.defaultdict(list)
    for f in fns: by_name[f['name']].append(f)
    def mut_positions(name):
        return {i for f in by_name.get(name, []) for i, (_, t) in enumerate(f['params']) if t[0] == 'mut'}
    upgraded = 0
    changed = True
    while changed:
        changed = False
        for f in fns:
            refs = {p for p, t in f['params'] if t[0] == 'ref'}
            roots = set()
            for n in walk(f['body']):
                if n[0] == 'mcall' and n[2] in MUTATING: roots.add(root(n[1]))
                elif n[0] == 'assign': roots.add(root(n[1]))
                if n[0] == 'call' and n[1][0] == 'path' and len(n[1][1]) == 1:
                    for i in mut_positions(n[1][1][0]):
                        if i < len(n[2]): roots.add(root(n[2][i]))
                if n[0] == 'mcall':
                    asts = [n[1]] + list(n[3])
                    for i in mut_positions(n[2]):
                        if i < len(asts): roots.add(root(asts[i]))
            hit = refs & roots
            if hit:
                f['params'] = [(p, ('mut', t[1]) if p in hit else t) for p, t in f['params']]
                f['interior'] = True; upgraded += len(hit); changed = True
    # a local that only names a place in a parameter (`let g = self.cache.borrow_mut()`, `let c = &mut self.state`)
    # is that place for as long as it lives (the borrow rules keep the place from changing under it): every use of
    # it becomes the place, so its writes reach the threaded parameter. Bound once, never assigned, and a place made
    # of fields, borrows and dereferences only.
    def placelike(e, params):
        while isinstance(e, tuple) and e:
            if e[0] in ('mutref', 'deref', 'field'): e = e[1]; continue
            if e[0] == 'mcall' and e[2] in THROUGH and not e[3]: e = e[1]; continue
            return e[0] == 'path' and len(e[1]) == 1 and e[1][0] in params
        return False
    def substitute(e, name, place):
        if isinstance(e, list): return [substitute(x, name, place) for x in e]
        if not isinstance(e, tuple): return e
        if e == ('path', [name]): return place
        if e and e[0] == 'block':
            stmts = [x for x in e[1] if not (x[0] == 'let' and x[1] == ('pid', name))]
            return ('block', [substitute(x, name, place) for x in stmts], substitute(e[2], name, place))
        return tuple(substitute(x, name, place) for x in e)
    for f in fns:
        params = {p for p, _ in f['params']}
        lets = collections.Counter(n[1][1] for n in walk(f['body']) if n[0] == 'let' and n[1] and n[1][0] == 'pid')
        assigned = {root(n[1]) for n in walk(f['body']) if n[0] == 'assign' and n[1][0] == 'path'}
        for n in list(walk(f['body'])):
            if n[0] == 'let' and n[1] and n[1][0] == 'pid' and n[3] is not None and n[3][0] != 'path' and placelike(n[3], params):
                name = n[1][1]
                written = any((m[0] == 'mcall' and m[2] in MUTATING and root(m[1]) == name) or (m[0] == 'assign' and root(m[1]) == name)
                              for m in walk(f['body']))
                in_format = any(m[0] in ('str', 'macro') and ('{' + name) in repr(m) for m in walk(f['body']))
                if written and not in_format and lets[name] == 1 and name not in assigned and name not in params:
                    f['body'] = substitute(f['body'], name, n[3]); f['aliases_inlined'] = f.get('aliases_inlined', 0) + 1
    # a local bound to part of a parameter, then mutated: an alias the threading cannot follow
    for f in fns:
        params = {p for p, _ in f['params']}
        bound = {}
        for n in walk(f['body']):
            if n[0] == 'let' and n[1] and n[1][0] == 'pid' and n[3] is not None and root(n[3]) in params and n[3][0] != 'path':
                bound[n[1][1]] = n[3]
        if not bound: continue
        for n in walk(f['body']):
            r = None
            if n[0] == 'mcall' and n[2] in MUTATING: r = root(n[1])
            elif n[0] == 'assign': r = root(n[1])
            if r in bound:
                f['refuse'] = 'a mutation through a local alias of a parameter (interior mutability)'; break
    return upgraded

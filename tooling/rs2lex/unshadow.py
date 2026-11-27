"""Rename a function's local variables apart, so that no binding shadows another.

The translation carries a scope's writes out of it by name, so a name bound twice (`let mut v` inside an `if` where an
outer `v` is written, `if let Some(x) = x`, `let items = items.iter()...` over a `&mut` parameter) would leave a
scope as the wrong variable. After this pass every binding in a function has its own name: a binding whose name is
already bound anywhere in the function gets a fresh one, and every use in its scope follows it, a format string's
implicit capture (`"{x}"`) included. The pass changes names only; Rust's scoping decides which binding a use means."""
import re

BINDERS = ('pid', 'pat_at')


def _names(e, out):
    if isinstance(e, tuple):
        if e and e[0] in BINDERS and isinstance(e[1], str): out.add(e[1])
        if len(e) == 2 and e[0] == 'path' and isinstance(e[1], list) and len(e[1]) == 1: out.add(e[1][0])
        for x in e: _names(x, out)
    elif isinstance(e, list):
        for x in e: _names(x, out)


class Renamer:
    def __init__(self, f):
        self.taken = set()
        _names(f['body'], self.taken)
        self.seen = {name for name, _ in f['params']}
        self.taken |= self.seen
        self.n = 0

    def fresh(self, name):
        while True:
            self.n += 1
            new = f'{name}_r{self.n}'
            if new not in self.taken:
                self.taken.add(new); return new

    # a pattern: its binders, renamed where the name is already bound; `local` keeps the alternatives of an or-pattern,
    # which bind the same names, on the same new names
    def pat(self, p, scope, local):
        if isinstance(p, list): return [self.pat(x, scope, local) for x in p]
        if not isinstance(p, tuple) or not p: return p
        if p[0] in BINDERS and isinstance(p[1], str):
            name = p[1]
            if name[:1].isupper() or name == '_':             # a constant or a unit variant, not a binding
                return p if p[0] == 'pid' else ('pat_at', name, self.pat(p[2], scope, local))
            if name not in local:
                if name in self.seen: local[name] = self.fresh(name)
                else: local[name] = name; self.seen.add(name)
            new = local[name]
            return ('pid', new) if p[0] == 'pid' else ('pat_at', new, self.pat(p[2], scope, local))
        if p[0] == 'pctor': return ('pctor', p[1], [(f, self.pat(q, scope, local)) for f, q in p[2]])
        if p[0] in ('plit', 'prange', 'pconst'): return p
        return tuple(self.pat(x, scope, local) if isinstance(x, (tuple, list)) else x for x in p)

    def bind(self, p, scope):
        local = {}
        new = self.pat(p, scope, local)
        inner = dict(scope); inner.update(local)
        return new, inner

    def fmt(self, text, scope):
        def capture(m):
            return '{' + scope.get(m.group(1), m.group(1))
        return re.sub(r'(?<!\{)\{([a-z_]\w*)(?=[:}])', capture, text)

    def expr(self, e, scope):
        if isinstance(e, list): return [self.expr(x, scope) for x in e]
        if not isinstance(e, tuple) or not e: return e
        h = e[0]
        if h == 'path' and isinstance(e[1], list):
            return ('path', [scope.get(e[1][0], e[1][0])]) if len(e[1]) == 1 else e
        if h == 'block' and len(e) == 3 and isinstance(e[1], list):
            inner = dict(scope); stmts = []
            for s in e[1]:
                if s[0] == 'let':
                    init = self.expr(s[3], inner); els = self.expr(s[4], inner)
                    pat, inner = self.bind(s[1], inner)
                    stmts.append(('let', pat, s[2], init, els))
                elif s[0] == 'item': stmts.append(s)           # a nested function: its own names
                else: stmts.append(tuple(self.expr(x, inner) if isinstance(x, (tuple, list)) else x for x in s))
            return ('block', stmts, self.expr(e[2], inner))
        if h == 'iflet':
            scrut = self.expr(e[2], scope); pat, inner = self.bind(e[1], scope)
            return ('iflet', pat, scrut, self.expr(e[3], inner), self.expr(e[4], scope)) + tuple(e[5:])
        if h == 'whilelet':
            scrut = self.expr(e[2], scope); pat, inner = self.bind(e[1], scope)
            return ('whilelet', pat, scrut, self.expr(e[3], inner)) + tuple(e[4:])
        if h == 'for':
            it = self.expr(e[2], scope); pat, inner = self.bind(e[1], scope)
            return ('for', pat, it, self.expr(e[3], inner)) + tuple(e[4:])
        if h == 'match':
            arms = []
            for pat, guard, body in e[2]:
                p2, inner = self.bind(pat, scope)
                arms.append((p2, self.expr(guard, inner), self.expr(body, inner)))
            return ('match', self.expr(e[1], scope), arms) + tuple(e[3:])
        if h == 'closure':
            inner = dict(scope); params = []
            for p, t in e[1]:
                p2, inner = self.bind(p, inner); params.append((p2, t))
            return ('closure', params, self.expr(e[2], inner)) + tuple(e[3:])
        if h == 'macro' and isinstance(e[2], list):
            args = [('str', self.fmt(a[1], scope)) if isinstance(a, tuple) and len(a) == 2 and a[0] == 'str' else self.expr(a, scope) for a in e[2]]
            return (h, e[1], args) + tuple(e[3:])
        if h == 'struct':                                   # field names are not variables
            out = ('struct', e[1], [(f, self.expr(v, scope)) for f, v in e[2]])
            return out + tuple(self.expr(x, scope) for x in e[3:])
        if h in ('let', 'item'): return e
        return tuple(self.expr(x, scope) if isinstance(x, (tuple, list)) else x for x in e)


def unshadow(f):
    """f's body with every local binding on a name of its own."""
    r = Renamer(f)
    return r.expr(f['body'], {})


# -- writes through a reference a pattern or a `let` binds

PRODUCERS = {'as_mut', 'iter_mut', 'get_mut', 'values_mut', 'last_mut', 'first_mut', 'as_deref_mut', 'entry', 'peek_mut',
             'split_at_mut', 'chunks_mut', 'split_first_mut', 'split_last_mut', 'get_or_insert', 'get_or_insert_with',
             'or_insert', 'or_insert_with', 'or_default', 'as_mut_slice', 'borrow_mut', 'deref_mut', 'make_mut'}
WRITERS = {'write_str', 'write_char', 'push', 'push_str', 'insert', 'sort', 'pop', 'truncate', 'extend', 'remove', 'retain', 'clear', 'dedup',
           'sort_unstable', 'sort_by', 'sort_by_key', 'sort_unstable_by', 'extend_from_slice', 'append', 'swap', 'reverse',
           'take', 'insert_str', 'push_back', 'push_front', 'pop_front', 'pop_back', 'drain', 'set', 'replace', 'entry'}


def _walk(e):
    if isinstance(e, tuple):
        yield e
        for x in e:
            if isinstance(x, (tuple, list)): yield from _walk(x)
    elif isinstance(e, list):
        for x in e: yield from _walk(x)


def _borrows(e):
    return any(n and (n[0] == 'mutref' or (n[0] == 'mcall' and isinstance(n[2], str) and n[2] in PRODUCERS)) for n in _walk(e))


def _root(a):
    while isinstance(a, tuple) and a:
        if a[0] in ('mutref', 'deref', 'field', 'index'): a = a[1]; continue
        if a[0] == 'mcall' and not a[3] and a[2] in PRODUCERS | {'borrow', 'as_ref'}: a = a[1]; continue
        break
    return a[1][0] if isinstance(a, tuple) and len(a) == 2 and a[0] == 'path' and isinstance(a[1], list) and len(a[1]) == 1 else None


def written_alias(f):
    """a name bound to a mutable reference into another value (`if let Some(x) = y.as_mut()`, `for x in v.iter_mut()`,
    `let x = &mut y.z`) and then written, or None. The translation binds such a name to a copy, so the write would
    not reach the value it refers to. Run after `unshadow`: every name is one binding."""
    bound = set(); exempt = set(); params = {name for name, _ in f['params']}
    def binders(p):
        return {n[1] for n in _walk(p) if n and n[0] in BINDERS and isinstance(n[1], str) and not n[1][:1].isupper()}
    for n in _walk(f['body']):
        if not n: continue
        if n[0] == 'let' and len(n) == 5 and n[3] is not None and n[3][0] != 'closure' and _borrows(n[3]):
            # a `let` naming a place inside a parameter is `interior`'s: it makes the name that place, or refuses
            if _root(n[3]) not in params: bound |= binders(n[1])
            else: exempt |= binders(n[1])
        elif n[0] in ('iflet', 'whilelet') and _borrows(n[2]): bound |= binders(n[1])
        elif n[0] == 'for' and _borrows(n[2]): bound |= binders(n[1])
        elif n[0] == 'match' and _borrows(n[1]):
            for arm in n[2]: bound |= binders(arm[0])
    if f.get('mut_refs') is not None:
        # rustc's own answer, where the body came from its tree: a name is another name for a place exactly when
        # its type is a mutable reference or holds one, however it was bound
        # a name rustc gave no type for here (one this pass made up) keeps the answer its syntax gives
        # a closure's own parameters are left to the translator, which makes one the caller's place where it is
        # given a place, and refuses the call where it is not (call_closure)
        closure_params = {n[1] for c in _walk(f['body']) if c and c[0] == 'closure' for p, _ in c[1] for n in _walk(p)
                          if n and n[0] in BINDERS and isinstance(n[1], str)}
        bound = (bound - set(f['plain'])) | (set(f['mut_refs']) - params - exempt - closure_params)
    if not bound: return None
    for n in _walk(f['body']):
        if not n: continue
        target = None
        if n[0] == 'mcall' and isinstance(n[2], str) and n[2] in WRITERS: target = _root(n[1])
        elif n[0] == 'assign': target = _root(n[1])
        if target in bound: return target
    return None


# -- the map entry API

OR = ('or_default', 'or_insert', 'or_insert_with')


def _chain(e):
    """(map, key, default) of `map.entry(key).or_default()` and its `or_insert` forms, or None."""
    if not (isinstance(e, tuple) and len(e) == 4 and e[0] == 'mcall' and e[2] in OR): return None
    inner = e[1]
    if not (isinstance(inner, tuple) and len(inner) == 4 and inner[0] == 'mcall' and inner[2] == 'entry' and len(inner[3]) == 1): return None
    if e[2] == 'or_default': default = ('call', ('path', ['Default', 'default']), [])
    elif e[2] == 'or_insert' and len(e[3]) == 1: default = e[3][0]
    elif e[2] == 'or_insert_with' and len(e[3]) == 1 and e[3][0][0] == 'closure' and not e[3][0][1]: default = e[3][0][2]
    else: return None
    return inner[1], inner[3][0], default


class Entries:
    """`map.entry(k).or_default().push(v)` and `*map.entry(k).or_insert(0) += 1` as what they do: take the value at
    the key or the default, update it, and insert it back. A bare `map.entry(k).or_insert(v);` inserts when absent.
    An entry bound to a name (`let slot = map.entry(k).or_default();`) is left as it is, and refused where written."""
    def __init__(self): self.n = 0

    def take(self, chain):
        m, key, default = chain
        self.n += 1; k, e, v = f'__key{self.n}', f'__entry{self.n}', f'__held{self.n}'
        P = lambda name: ('path', [name])
        found = (('pctor', ['Some'], [(0, ('pid', v))]), None, ('mcall', P(v), 'clone', []))
        absent = (('pctor', ['None'], []), None, default)
        lets = [('let', ('pid', k), None, key, None),
                ('let', ('pid', e), None, ('match', ('mcall', m, 'get', [P(k)]), [found, absent]), None)]
        back = ('expr', ('mcall', m, 'insert', [P(k), P(e)]))
        return lets, P(e), back

    def expr(self, e):
        if isinstance(e, list): return [self.expr(x) for x in e]
        if not isinstance(e, tuple) or not e: return e
        if e[0] == 'block' and len(e) == 3 and isinstance(e[1], list):     # a block, not a field named `block`
            stmts = []
            for st in e[1]:
                chain = _chain(st[1]) if st[0] == 'expr' else None
                if chain is not None:                       # a bare entry: insert the default when the key is absent
                    lets, held, back = self.take(tuple(self.expr(x) for x in chain))
                    stmts.append(('expr', ('block', lets + [back], None)))
                else: stmts.append(self.expr(st) if st[0] != 'item' else st)
            return ('block', stmts, self.expr(e[2]))
        if e[0] == 'mcall' and len(e) == 4 and _chain(e[1]) is not None:
            lets, held, back = self.take(tuple(self.expr(x) for x in _chain(e[1])))
            self.n += 1; r = f'__result{self.n}'
            call = ('mcall', held, e[2], self.expr(e[3]))
            return ('block', lets + [('let', ('pid', r), None, call, None), back], ('path', [r]))
        if e[0] == 'assign':
            lhs = e[1]
            while isinstance(lhs, tuple) and lhs and lhs[0] == 'deref': lhs = lhs[1]
            if _chain(lhs) is not None:
                lets, held, back = self.take(tuple(self.expr(x) for x in _chain(lhs)))
                def swap(x):
                    if x == e[1] or x == lhs: return held
                    if isinstance(x, tuple): return tuple(swap(y) for y in x)
                    if isinstance(x, list): return [swap(y) for y in x]
                    return x
                return ('block', lets + [('expr', ('assign', held, self.expr(swap(e[2])))), back], None)
        return tuple(self.expr(x) if isinstance(x, (tuple, list)) else x for x in e)


def entries(f):
    return Entries().expr(f['body'])

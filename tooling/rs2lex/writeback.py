"""A write through a mutable reference to an element, as a write to the element.

The translation binds a name to a value, never to a place, so a name that Rust binds to a `&mut` into a list would take
a copy and lose its writes (unshadow.written_alias refuses those). Two bindings of that kind have an exact reading as
plain values, because Rust's own borrow rule holds the list untouched for as long as the reference lives:

    for x in &mut xs { body }                      for i in 0..xs.len() { let x = xs[i].clone(); body; xs[i] = x; }
    for x in xs.iter_mut() { body }                the same
    if let Some(x) = xs.last_mut() { a } else { b }
                                                   if !xs.is_empty() { let i = xs.len() - 1; let x = xs[i].clone(); a; xs[i] = x; } else { b }

The element is written back on every way out of the body: at its end, and before a `break`, a `continue` or a `return`
that leaves it. A `?` leaves it too, at a point this pass does not split, so a body with one is left alone, as is a
labelled jump; the refusal stands for those. `xs` must be a place named without a call or an index (a variable, a
field of one), so that naming it twice evaluates nothing twice.

Run after `unshadow`: every name is one binding, and the names made here (`__wbN_i`, `__wbN_v`, `__wbrN`) are new.
"""
from unshadow import WRITERS, _root, _walk

LOOPS = ('for', 'while', 'whilelet', 'loop')


class Skip(Exception):
    pass


def _place(e):
    """a variable, or a field of a place: evaluating it has no effect and gives the same place each time."""
    while isinstance(e, tuple) and e and e[0] == 'field': e = e[1]
    return isinstance(e, tuple) and len(e) == 2 and e[0] == 'path' and isinstance(e[1], list) and len(e[1]) == 1


def _bare(e):
    """a place without the borrows around it: rustc writes the reborrow of a receiver (`(&mut *v).iter_mut()`)."""
    while isinstance(e, tuple) and e and e[0] in ('mutref', 'deref', 'ref'): e = e[1]
    return e


def _written(name, body):
    for n in _walk(body):
        if not n: continue
        if n[0] == 'assign' and _root(n[1]) == name: return True
        if n[0] == 'mcall' and isinstance(n[2], str) and n[2] in WRITERS and _root(n[1]) == name: return True
        if n[0] == 'mutref' and _root(n[1]) == name: return True
        if n[0] == 'call' and any(isinstance(a, tuple) and a[:1] == ('path',) and a[1] == [name] for a in n[2]): return True
    return False


class Pass:
    def __init__(self): self.n = 0; self.released = set()

    def fresh(self, what):
        return f'__wb{self.n}_{what}'

    def exits(self, e, back, in_loop):
        """`e` with `back` (statements) run before each jump out of it: a `return` anywhere, a `break` or `continue`
        of the loop around it."""
        if isinstance(e, list): return [self.exits(x, back, in_loop) for x in e]
        if not isinstance(e, tuple) or not e: return e
        h = e[0]
        if h in ('closure', 'item'): return e                      # its jumps are its own
        if h == 'try': raise Skip
        if h == 'labelled': raise Skip
        if h == 'return':
            # the element is put back first: in Rust the write is already in place when the returned value is made,
            # which may read it. A returned value that itself writes is not reordered
            if len(e) > 1 and e[1] is not None:
                if any(n and (n[0] == 'assign' or (n[0] == 'mcall' and isinstance(n[2], str) and n[2] in WRITERS)) for n in _walk(e[1])):
                    raise Skip
                return ('block', list(back), ('return', e[1]))
            return ('block', list(back), e)
        if h in ('break', 'continue'):
            if in_loop: return e
            if e[1] is not None or (h == 'break' and len(e) > 2 and e[2] is not None): raise Skip
            return ('block', list(back), e)
        if h in LOOPS:
            return tuple(self.exits(x, back, True) if isinstance(x, (tuple, list)) else x for x in e)
        return tuple(self.exits(x, back, in_loop) if isinstance(x, (tuple, list)) else x for x in e)

    def element(self, xs, i, x, body):
        """the statements of `body` with `x` the element `xs[i]`, read first and written back on every way out."""
        back = [('expr', ('assign', ('index', xs, ('path', [i])), ('path', [x])))]
        # a `break` or `continue` written in the body leaves it, whether the loop is this one or one around it
        stmts = self.exits(list(body[1]), back, False)
        tail = self.exits(body[2], back, False) if body[2] is not None else None
        read = ('let', ('pid', x), None, ('mcall', ('index', xs, ('path', [i])), 'clone', []), None)
        return read, stmts, tail, back

    def go(self, e):
        if isinstance(e, list): return [self.go(x) for x in e]
        if not isinstance(e, tuple) or not e: return e
        e = tuple(self.go(x) if isinstance(x, (tuple, list)) else x for x in e)
        try:
            if e[0] == 'for' and len(e) == 4 and e[1][0] == 'pid' and e[3][0] == 'block':
                it = e[2]
                xs = it[1] if it[0] == 'mutref' else it[1] if it[0] == 'mcall' and it[2] == 'iter_mut' and not it[3] else None
                xs = _bare(xs) if xs is not None else None
                if xs is not None and _place(xs) and _written(e[1][1], e[3]):
                    self.n += 1; i = self.fresh('i'); x = e[1][1]
                    read, stmts, tail, back = self.element(xs, i, x, e[3])
                    if tail is not None: stmts = stmts + [('expr', tail)]
                    self.released.add(x)
                    return ('for', ('pid', i), ('range', ('int', 0), ('mcall', xs, 'len', []), False), ('block', [read] + stmts + back, None))
            if e[0] == 'iflet' and len(e) == 5 and e[3][0] == 'block':
                pat, scrut = e[1], e[2]
                some = pat[0] == 'pctor' and pat[1] == ['Some'] and len(pat[2]) == 1 and pat[2][0][1][0] == 'pid'
                if some and scrut[0] == 'mcall' and scrut[2] == 'last_mut' and not scrut[3] and _place(_bare(scrut[1])):
                    x = pat[2][0][1][1]; xs = _bare(scrut[1])
                    if _written(x, e[3]):
                        self.n += 1; i = self.fresh('i')
                        read, stmts, tail, back = self.element(xs, i, x, e[3])
                        at = ('let', ('pid', i), None, ('bin', '-', ('mcall', xs, 'len', []), ('int', 1)), None)
                        if tail is None: then = ('block', [at, read] + stmts + back, None)
                        else:
                            r = self.fresh('v')
                            then = ('block', [at, read] + stmts + [('let', ('pid', r), None, tail, None)] + back, ('path', [r]))
                        self.released.add(x)
                        return ('if', ('not', ('mcall', xs, 'is_empty', [])), then, e[4])
            if e[0] == 'iflet' and len(e) == 5 and e[3][0] == 'block':
                pat, scrut = e[1], e[2]
                some = pat[0] == 'pctor' and pat[1] == ['Some'] and len(pat[2]) == 1 and pat[2][0][1][0] == 'pid'
                if some and scrut[0] == 'mcall' and scrut[2] == 'get_mut' and len(scrut[3]) == 1 and _place(_bare(scrut[1])):
                    x = pat[2][0][1][1]; xs = _bare(scrut[1]); key = _bare(scrut[3][0])
                    if (_place(key) or key[0] == 'int') and _written(x, e[3]):
                        # `if let Some(x) = m.get_mut(k)`: the value at k read, changed, and put back at k (a map's
                        # insert, a list's index; the translator picks by the type)
                        self.n += 1
                        back = [('expr', ('assign', ('index', xs, key), ('path', [x])))]
                        stmts = self.exits(list(e[3][1]), back, False)
                        tail = self.exits(e[3][2], back, False) if e[3][2] is not None else None
                        if tail is None: then = ('block', stmts + back, None)
                        else:
                            r = self.fresh('t')
                            then = ('block', stmts + [('let', ('pid', r), None, tail, None)] + back, ('path', [r]))
                        self.released.add(x)
                        found = ('mcall', ('mcall', xs, 'get', [scrut[3][0]]), 'cloned', [])
                        return ('iflet', ('pctor', ['Some'], [(0, ('pid', x))]), found, then, e[4])
        except Skip:
            pass
        return e


def writeback(f):
    """the body of `f` with those bindings read as values, and the names so released (no longer references)."""
    p = Pass()
    body = p.go(f['body'])
    return body, p.released

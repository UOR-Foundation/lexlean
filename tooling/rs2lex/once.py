"""A value computed once, as the value.

    static TABLE: OnceLock<T> = OnceLock::new();
    ... TABLE.get_or_init(|| make()) ...

keeps the first `make()` for every later call. Where the closure reads nothing of the function it is written in (no
parameter, no local), every call would make the same value, so the cell cannot be told from making it each time:

    let __onceN = || make();
    ... __onceN() ...

The closure stays a closure, called where the cell was read, so a `?` or a `return` in it leaves the closure as it did.
A closure that reads a parameter or a local is left alone (the cell would keep the first call's answer), and so is a
cell used in any other way (`get`, `set`); the refusal of the type stands for those.

Run after `unshadow`: every name is one binding.
"""
from unshadow import BINDERS, _walk


def _binders(e):
    return {n[1] for n in _walk(e) if n and n[0] in BINDERS and isinstance(n[1], str)}


def _reads(e):
    return {n[1][0] for n in _walk(e) if n and len(n) == 2 and n[0] == 'path' and isinstance(n[1], list) and len(n[1]) == 1}


def once(f):
    """the body of `f` with each such cell read as its closure called."""
    body = f['body']
    cells = {}
    for n in _walk(body):
        if (n and n[0] == 'let' and len(n) == 5 and n[1][0] == 'pid' and n[1][1][:1].isupper() and n[3] is not None
                and n[3][0] == 'call' and n[3][1][0] == 'path' and n[3][1][1][-2:] == ['OnceLock', 'new'] and not n[3][2]):
            cells[n[1][1]] = []
    if not cells: return body
    for n in _walk(body):                                        # every use of a cell
        if n and len(n) == 2 and n[0] == 'path' and n[1] in ([c] for c in cells): cells[n[1][0]].append('use')
    inits = {}
    for n in _walk(body):
        if (n and n[0] == 'mcall' and n[2] == 'get_or_init' and len(n[3]) == 1 and n[3][0][0] == 'closure' and not n[3][0][1]
                and n[1][0] == 'path' and n[1][1][0] in cells and len(n[1][1]) == 1):
            inits.setdefault(n[1][1][0], []).append(n)
    outer = _binders(body) | {name for name, _ in f['params']}
    ready = {}
    for cell, uses in cells.items():
        got = inits.get(cell, [])
        if len(got) != 1 or len(uses) != 1: continue             # read once, by get_or_init, and nowhere else
        closure = got[0][3][0]
        if _reads(closure[2]) & (outer - _binders(closure[2])): continue
        ready[cell] = (got[0], closure)
    if not ready: return body

    def go(e):
        if isinstance(e, list):
            out = []
            for x in e:
                if isinstance(x, tuple) and x and x[0] == 'let' and len(x) == 5 and x[1][0] == 'pid' and x[1][1] in ready:
                    out.append(('let', ('pid', '__once_' + x[1][1].lower()), None, ready[x[1][1]][1], None))
                else: out.append(go(x))
            return out
        if not isinstance(e, tuple) or not e: return e
        for cell, (read, _) in ready.items():
            if e is read or e == read: return ('call', ('path', ['__once_' + cell.lower()]), [])
        return tuple(go(x) if isinstance(x, (tuple, list)) else x for x in e)
    return go(body)

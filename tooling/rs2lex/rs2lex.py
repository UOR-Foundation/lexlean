"""rs2lex: the round-four translator. The core (rs2lex_core) is round three with a full Rust parser, loops without
an iterator, break, `if` without `else`, assignment to any place, n-tuples, tuple structs and literal patterns.
This file adds, through the core's hooks, the rules for the standard library and for the crate's own constructors:

  * constructors: tuple variants `E::V(a)`, tuple structs `X(a)`, unit structs, associated fns `T::f(a)`, associated consts,
  * types: Rc<T>, Arc<T> and Box<T> as T (sound for the immutable sharing they give here; Cell/RefCell stay refused),
    HashMap/HashSet as the ordered map/set (iterating a hash collection is refused: its order would matter), type aliases,
  * maps with any key type, kept sorted by a derived order (BTreeMap iteration order is that order),
  * ~60 methods of Option, Result, integers, strings, slices and maps,
  * macros: code!, write!/writeln! into a String, assert!s (dropped), unreachable!/panic!/todo!.

Panics (unwrap/expect on the failing case, panic!, an index out of range, assert! failures) are not preserved: the
translation continues with a default value of the expected type. They are bugs in the Rust either way; the
differential against the oracle is what checks that no tested input reaches one."""
import re
import rs2lex_core as core
from rs2lex_core import (Unsupported, Translator as Core, Ctx, BYTES, NAT, BOOL, UNIT, I128, UINT8, text, cat,
                          is_atom, FUEL, WILD)

core.EXTERN_TYPES.clear(); core.EXTERN_FNS.clear()       # the crate's own Diagnostic and Lean-name check are translated

ORDERING = ('Named', 'Ordering')       # LexLean's own Ordering, the result of a comparison primitive

class Translator(Core):
    def __init__(self, items, aliases=None):
        self.aliases = aliases or {}
        import interior
        self.interior_upgrades = interior.infer(items)        # &T parameters mutated through Cell/RefCell become &mut T
        self.assoc_consts = {(i['owner'], i['name']): i for i in items if i['k'] == 'const' and i.get('owner')}
        super().__init__(items)
        self.deferred_n = 0

    # ----------------------------------------------------------------------------------------------- types
    def hook_ty(self, n, a, owner):
        if n in ('Rc', 'Arc') and a: return a[0]
        if n == 'RawPtr': return ('Ptr',)
        if n == 'DynError': return BYTES                     # a host error, as the message it displays
        if n == 'char': return BYTES                         # one character, as its UTF-8 bytes (as a char literal is)
        if n == 'Formatter': return BYTES                  # a Display/Debug impl writes into a string buffer
        if n in ('i8', 'i16', 'i32', 'i64', 'isize'):       # LexLean's fixed-width signed integers
            return ('Fixed', {'i8': 'Int8', 'i16': 'Int16', 'i32': 'Int32', 'i64': 'Int64', 'isize': 'Int64'}[n])
        if n == 'HashMap': return ('Map', a[0], a[1])
        if n == 'VecDeque': return ('List', a[0])              # a double-ended queue as a list
        if n == 'HashSet': return ('Set', a[0])
        if n in ('Cell', 'RefCell') and a: return a[0]       # interior mutability as state threading (see interior.py)
        if n in ('Mutex', 'RwLock', 'OnceLock', 'OnceCell'): raise Unsupported(f'interior mutability ({n})')
        if n in self.aliases:
            params, body = self.aliases[n]
            return self.ty(body, owner, dict(zip(params, a)))
        return None

    # ----------------------------------------------------------------------------------------------- declaration order
    def module(self, name, imports):
        """order every declaration after what it references, each mutual group as one contiguous unit."""
        text, report = super().module(name, imports)
        marker = '\\begin{semanticmodule}\n'
        head_end = text.index(marker) + len(marker); tail_start = text.rindex('\n\\end{semanticmodule}')
        head, body, tail = text[:head_end], text[head_end:tail_start], text[tail_start:]
        decls = [d for d in re.split(r'\n\n(?=(?:definition|inductive|structure)\{)', body) if d.strip()]
        names = [re.search(r'name: (\w+)', d).group(1) for d in decls]
        unit_of = {}
        for i, d in enumerate(decls):
            m = re.search(r'mutual: (\w+)', d)
            unit_of[names[i]] = ('group', m.group(1)) if m else ('one', names[i])
        units = {}
        for i, n in enumerate(names): units.setdefault(unit_of[n], []).append(i)
        known = set(names)
        deps = {}
        for u, idxs in units.items():
            refs = set()
            for i in idxs:
                body_i = re.sub(r'name: \w+', '', decls[i].split('\n', 2)[-1])     # constructor and field names are not references
                body_i = re.sub(r'\b\w+\.\w+', '', body_i)                         # nor are qualified constructors
                refs |= {w for w in re.findall(r'[A-Za-z_]\w*', body_i) if w in known}
            deps[u] = {unit_of[r] for r in refs} - {u}
        order, state = [], {}
        def visit(u):
            if state.get(u) == 2: return
            if state.get(u) == 1: return                      # a reference cycle outside a mutual group: keep the order found
            state[u] = 1
            for d in sorted(deps[u], key=lambda x: min(units[x])): visit(d)
            state[u] = 2; order.append(u)
        for u in sorted(units, key=lambda x: min(units[x])): visit(u)
        out = [decls[i] for u in order for i in units[u]]
        sizes = {}
        for d in out:
            m = re.search(r'mutual: (\w+)', d)
            if m: sizes[m.group(1)] = sizes.get(m.group(1), 0) + 1
        out = [re.sub(r',\n  mutual: (\w+)', lambda m: '' if sizes.get(m.group(1)) == 1 else m.group(0), d) for d in out]
        return head + '\n\n'.join(out) + tail, report

    # ----------------------------------------------------------------------------------------------- first-class functions
    def fn_type_of(self, key):
        fn = self.fns[key]
        if fn['generics']: raise Unsupported('a generic function passed as a value')
        if any(t[0] == 'mut' for _, t in fn['params']): raise Unsupported('a function with `&mut` parameters passed as a value')
        params = [self.ty(t, key[0]) for _, t in fn['params']]
        return params, self.ty(fn['ret'], key[0])
    def fn_value(self, key, env):
        """a named function as a value: `ref`, or a lambda that supplies its fuel."""
        T, show = self.T, self.T.show
        if key in self.cur_scc: raise Unsupported('a function of the current recursion group passed as a value')
        fn = self.user_fn(key)
        params, ret = self.fn_type_of(key)
        fty = ('Fn', ('Args',) + tuple(params or [UNIT]), ret)
        if key not in self.rec_keys and params:
            return f'(ref {fn["lex"]})', fty
        names = [self.fresh('arg') for _ in params] or [self.fresh('unused')]
        binders = ' '.join(f'({n} : {show(t)})' for n, t in zip(names, params or [UNIT]))
        args = ([FUEL] if key in self.rec_keys else []) + (names if params else [])
        return f'(fun {binders} => ({fn["lex"]} {" ".join(args)}))', fty
    def closure_value(self, c, cenv, env, hint=None):
        """a closure as a value: a lambda with its captures. It may return early with `?`; it may not assign
        what it captures, nor call back into the current recursion group (LexLean cannot check that in a lambda)."""
        T, show = self.T, self.T.show
        _, params, body, ret_ann = c
        if self.assigned(body, cenv): raise Unsupported('a closure that mutates what it captures')
        hint = T.res(hint) if hint is not None else None
        hint_args = list(hint[1][1:]) if hint is not None and hint[0] == 'Fn' else None
        ptypes = []
        for i, (p, ann) in enumerate(params):
            t = self.ty(ann, self.owner) if ann is not None else T.var()
            if hint_args is not None and i < len(hint_args): T.unify(t, hint_args[i], 'closure parameter')
            ptypes.append(t)
        rt = self.ty(ret_ann, self.owner) if ret_ann is not None else T.var()
        if hint is not None and hint[0] == 'Fn': T.unify(rt, hint[2], 'closure result')
        inner = dict(cenv)
        if '__fuel' in env: inner['__fuel'] = env['__fuel']
        names = []
        for (p, _), t in zip(params, ptypes):
            n = self.fresh(p[1] if p[0] == 'pid' else 'arg'); names.append(n)
            if p[0] == 'pid': inner[p[1]] = (n, t)
        mark = len(self.call_log)
        def translate(env2):
            if self.effectful(body):
                local = Ctx(ret=lambda v, e2: v, ret_t=rt)
                tail = lambda v, t, e2: (T.unify(t, rt, 'closure result'), v)[1]
                return self.block(body, env2, local, tail) if body[0] == 'block' else self.expr(body, env2, local, tail)
            v, t = self.pure_block(body, env2) if body[0] == 'block' else self.pure(body, env2)
            T.unify(t, rt, 'closure result'); return v
        def bind_patterns(i, env2):
            if i == len(params): return translate(env2)
            p = params[i][0]
            if p[0] in ('pid', 'pwild'): return bind_patterns(i + 1, env2)
            return self.bind(p, names[i], ptypes[i], env2, lambda env3: bind_patterns(i + 1, env3))
        text_ = bind_patterns(0, inner)
        if any(key in self.cur_scc for key in self.call_log[mark:]):
            raise Unsupported('a closure that calls back into its recursion group')
        if not names: names, ptypes = [self.fresh('unused')], [UNIT]
        caps = self.captures(text_, cenv, set(names))
        binders = ' '.join(f'({n} : {show(t)})' for n, t in zip(names, ptypes))
        return f'(fun {caps}{binders} => {text_})', ('Fn', ('Args',) + tuple(ptypes), rt)
    def value_of_function(self, e, env):
        """e as a function value, when it is one: a closure literal, a closure bound by `let`, or a named function."""
        hint = getattr(self, 'hints', {}).get(id(e))
        if e[0] == 'closure': return self.closure_value(e, env, env, hint)
        if e[0] == 'path':
            segs = [self.owner if x == 'Self' else x for x in e[1]]
            if len(segs) == 1 and segs[0] in env and env[segs[0]][0] is None:
                _, cl, cenv = env[segs[0]][1]; return self.closure_value(cl, cenv, env, hint)
            if len(segs) == 1 and segs[0] not in env and (None, segs[0]) in self.fns: return self.fn_value((None, segs[0]), env)
            if len(segs) == 2 and (segs[0], segs[1]) in self.fns:
                return self.fn_value((segs[0], segs[1]), env)
        return None
    def apply_value(self, f, ft, vs, env, k):
        T, show = self.T, self.T.show
        ft = T.res(ft)
        if ft[0] != 'Fn': raise Unsupported(f'a call of a value of type {show(ft)}')
        params = list(ft[1][1:])
        if params == [UNIT] and not vs: return k(f'(apply {f} ())', ft[2], env)
        if len(params) != len(vs): raise Unsupported('a function value applied to the wrong number of arguments')
        for (v, t), pt in zip(vs, params): T.unify(t, pt, 'argument of a function value')
        return k(f'(apply {f} {" ".join(v for v, _ in vs)})', ft[2], env)
    def closure_arg(self, c, env, vs):
        """a combinator's function argument: a closure is inlined as before; a function value is applied."""
        if c[0] == 'closure' or (c[0] == 'path' and len(c[1]) == 1 and c[1][0] in env and env[c[1][0]][0] is None):
            return super().closure_arg(c, env, vs)
        if c[0] == 'path' and len(c[1]) == 1 and c[1][0] in env:
            f, ft = env[c[1][0]]
        else:
            # a named function given to a combinator: a direct call on the combinator's arguments
            args = [('__value', v, t) for v, t in vs]
            segs = [self.owner if x == 'Self' else x for x in c[1]] if c[0] == 'path' else None
            candidates = []
            if segs is not None:
                candidates.append(('call', ('path', segs), args))
                if len(segs) == 2 and args and not (segs[0], segs[1]) in self.fns:
                    candidates.append(('mcall', args[0], segs[1], args[1:]))     # `String::as_str`: a method of the first argument
            for ast in candidates:
                try:
                    return self.pure(ast, env)
                except Unsupported:
                    continue
            got = self.value_of_function(c, env)
            if got is None: raise Unsupported('a function passed by name')
            f, ft = got
        box = {}
        def k(v, t, env2): box['t'] = t; return v
        text_ = self.apply_value(f, ft, vs, env, k)
        return text_, box['t']

    # ----------------------------------------------------------------------------------------------- deferred defaults
    def deferred(self):
        """a value of the expected type, for a panic path: the type is known only after inference."""
        t = self.T.var(); self.deferred_n += 1
        return f'¥{t[1]}¥', t
    def function(self, f):
        if f.get('refuse'): raise Unsupported(f['refuse'])
        if f['k'] == 'fn' and f['ret'][0] == 'fnty': raise Unsupported('returns a function (an escaping closure)')
        out = super().function(f)
        if '¥' in out:
            out = re.sub('¥(\\d+)¥', lambda m: self.default(('Var', int(m.group(1)))), out)
            out = self.T.patch(out)
        return out

    # ----------------------------------------------------------------------------------------------- expressions
    # ----------------------------------------------------------------------------------------------- Display, Debug, ==
    def fmt_call(self, key, v):
        fn = self.user_fn(key)
        args = ([FUEL] if key in self.rec_keys else []) + [v, text('')]
        return f'(second ({fn["lex"]} {" ".join(args)}))'
    def fixed_text(self, v, t): return f'(prim utf8_encode (prim format_decimal {v} : String) : Bytes)'
    def debug(self, v, t):
        if self.T.res(t)[0] == 'Fixed': return self.fixed_text(v, t)
        return super().debug(v, t)
    def default(self, t, depth=0):
        if self.T.res(t)[0] == 'Fixed': return f'(0 : {self.T.res(t)[1]})'
        return super().default(t, depth)
    def mangle(self, t):
        if self.T.res(t)[0] == 'Fixed': return self.T.res(t)[1]
        return super().mangle(t)
    def int_path(self, segs):
        if len(segs) == 2 and segs[0] in ('i8', 'i16', 'i32', 'i64', 'isize') and segs[1] in ('MAX', 'MIN'):
            width = {'i8': 8, 'i16': 16, 'i32': 32, 'i64': 64, 'isize': 64}[segs[0]]
            ty = ('Fixed', f'Int{width}')
            value = (1 << (width - 1)) - 1 if segs[1] == 'MAX' else -(1 << (width - 1))
            return f'({value} : {ty[1]})', ty
        return super().int_path(segs)
    def display(self, v, t):
        t = self.T.res(t)
        if t[0] == 'Fixed': return self.fixed_text(v, t)
        if t[0] == 'Named' and (t[1], 'fmt_display') in self.fns: return self.fmt_call((t[1], 'fmt_display'), v)
        return super().display(v, t)
    def debug_value(self, v, t):
        t = self.T.res(t)
        if t[0] == 'Named' and (t[1], 'fmt_debug') in self.fns: return self.fmt_call((t[1], 'fmt_debug'), v)
        return self.debug(v, t)
    def equal(self, a, b, t):
        T, t = self.T, self.T.nat(self.T.res(t))
        if t[0] == 'Fixed': return f'(prim equal {a} {b} : Bool)'
        if t[0] in ('Option', 'Result'):
            # inline, not a helper: a helper for a recursive element type would form a cycle with the element's equality
            ctors = self.ctors_of(t); arms = []
            l0, r0 = self.fresh('optleft'), self.fresh('optright')
            for c, _, fs in ctors:
                inner = []
                for c2, _, fs2 in ctors:
                    if c2 == c and fs: test = self.equal(l0, r0, fs[0][1])
                    else: test = 'true' if c2 == c else 'false'
                    inner.append(f'{c2}{" " + r0 if fs2 else ""} => {test}')
                arms.append(f'{c}{" " + l0 if fs else ""} => (match {b} | {" | ".join(inner)})')
            return f'(match {a} | {" | ".join(arms)})'
        if t[0] == 'Map' and self.T.map_rep(t[1], t[2]) == 'keyed':
            KV = f'({self.T.show(t[1])} * {self.T.show(t[2])})'
            A, E = self.fresh('acc'), self.fresh('entry')
            back = lambda x: f'(AxiomOutput::reversed [{KV}] (prim list_fold (fun ({A} : (List {KV})) ({E} : (Bytes * {KV})) => (cons (second {E}) {A})) (nil {KV}) (LexLeanStd.OrdMap::mapEntries [{KV}] {x}) : (List {KV})))'
            return super().equal(back(a), back(b), ('List', ('Prod', t[1], t[2])))
        if t[0] == 'Map' and self.T.map_rep(t[1], t[2]) == 'tree':
            V = self.T.show(t[2])
            return super().equal(f'(LexLeanStd.OrdMap::mapEntries [{V}] {a})', f'(LexLeanStd.OrdMap::mapEntries [{V}] {b})', ('List', ('Prod', t[1], t[2])))
        if t[0] in ('Map', 'Set'): return super().equal(a, b, ('List', ('Prod', t[1], t[2])) if t[0] == 'Map' else ('List', t[1]))
        return super().equal(a, b, t)

    def unit_struct(self, name):
        st = self.structs.get(name)
        return st is not None and not st['fields']
    def hook_expr(self, e, env, ctx, k):
        T, show = self.T, self.T.show
        h = e[0]
        if h == 'closure' or (h == 'path' and len(e[1]) <= 2):
            got = self.value_of_function(e, env)
            if got is not None: return k(got[0], got[1], env)
        if h == 'path':
            segs = [self.owner if x == 'Self' else x for x in e[1]]
            if len(segs) == 1 and segs[0] not in env and self.unit_struct(segs[0]):
                return k(f'(record {segs[0]} {{unit := ()}})', ('Named', segs[0]), env)
            if len(segs) == 2 and (segs[0], segs[1]) in self.assoc_consts:
                c = self.assoc_consts[(segs[0], segs[1])]
                saved = self.owner; self.owner = segs[0]
                try: return self.expr(c['value'], env, ctx, lambda v, t, env2: (T.unify(t, self.ty(c['type'], segs[0])), k(v, t, env2))[1])
                finally: self.owner = saved
            return None
        if h == 'bstr':
            return k(f'(RustStd::natsOf {text(e[1])})', ('List', NAT), env)
        if h == 'macro':
            name, args = e[1], e[2]
            if name == 'code' and 'DiagnosticCode' in self.structs:
                fields = self.field_types('DiagnosticCode')
                if len(fields) == 1: return k(f'(record DiagnosticCode {{{fields[0][0]} := {text(args[0][1])}}})', ('Named', 'DiagnosticCode'), env)
            if name in ('assert', 'assert_eq', 'assert_ne', 'debug_assert', 'debug_assert_eq', 'debug_assert_ne'):
                return k('()', UNIT, env)
            if name in ('unreachable', 'panic', 'todo', 'unimplemented'):
                v, t = self.deferred(); return k(v, t, env)
            if name in ('write', 'writeln') and len(args) >= 2 and args[1][0] == 'str':
                fmt = ('macro', 'format', [('str', args[1][1] + ('\n' if name == 'writeln' else ''))] + list(args[2:]))
                dst = args[0]
                def wrote(v, t, env2):
                    def after(env3):
                        err = T.var()
                        return k(f'(ctor Result.ok [Unit, {show(err)}] ())', ('Result', UNIT, err), env3)
                    cur, ct = self.pure(dst, env2); T.unify(ct, BYTES, 'write! destination')
                    return self.assign_into(dst, f'(AxiomOutput::cat {cur} {v})', BYTES, env2, after)
                return self.expr(fmt, env, ctx, wrote)
            if name == 'format' and args and args[0][0] == 'str' and '{' not in args[0][1].replace('{{', ''):
                return None
        if h == 'range' and e[1] is not None and e[2] is not None:
            def rng(vs, env2):
                (lo, lt), (hi, ht) = vs; T.unify(lt, NAT, 'range'); T.unify(ht, NAT, 'range')
                return k(f'(RustStd::range {lo} {f"({hi} + 1)" if e[3] else hi})', ('List', NAT), env2)
            return self.exprs([e[1], e[2]], env, ctx, rng)
        if h == 'index' and e[2][0] == 'range':
            lo_e, hi_e, inc = e[2][1], e[2][2], e[2][3]
            def sliced(xs, xt, env2):
                def bounds(vs, env3):
                    lo = vs[0][0] if lo_e is not None else '0'
                    xt2 = T.res(xt)
                    length = f'(prim length {xs} : Nat)' if xt2 == BYTES else f'(AxiomOutput::listLength [{show(xt2[1])}] {xs})' if xt2[0] == 'List' else None
                    if length is None: raise Unsupported(f'slicing {show(xt2)}')
                    hi = (vs[-1][0] if not inc else f'({vs[-1][0]} + 1)') if hi_e is not None else length
                    for _, t in vs: T.unify(t, NAT, 'slice bound')
                    count = f'(prim subtract {hi} {lo} : Nat)'
                    if xt2 == BYTES: return k(f'(LexLeanStd.Text::sliceOr {xs} {lo} {count})', BYTES, env3)
                    el = xt2[1]
                    return k(f'(RustStd::takeN [{show(el)}] (RustStd::dropN [{show(el)}] {xs} {lo}) {count})', xt2, env3)
                parts = [p for p in (lo_e, hi_e) if p is not None]
                return self.exprs(parts, env2, ctx, bounds)
            return self.expr(e[1], env, ctx, sliced)
        if h == 'arrayrep':
            return self.exprs([e[1], e[2]], env, ctx, lambda vs, env2: (T.unify(vs[1][1], NAT, 'repeat count'),
                              k(f'(RustStd::repeatItem [{show(vs[0][1])}] {vs[0][0]} {vs[1][0]})', ('List', vs[0][1]), env2))[1])
        if h == 'neg':
            if e[1][0] == 'int':                                   # a negative literal: its type comes from inference
                t = T.var(); return k(f'¦{t[1]}|-{e[1][1]}¦', t, env)
            def negated(v, t, env2):
                t = T.res(t)
                if t[0] == 'Fixed':
                    return k(self.panic_default(f'(prim checked_negate {v} : (Option {t[1]}))', t), t, env2)
                T.unify(t, I128, 'negation'); return k(f'(RustStd::i128Neg {v})', I128, env2)
            return self.expr(e[1], env, ctx, negated)
        if h == 'bin' and e[1] in ('+', '-', '*', '/', '%', '<', '<=', '>', '>=', '==', '!='):
            return self.exprs([e[2], e[3]], env, ctx, lambda vs, env2: self.binary(e[1], vs, env2, k))
        if h == 'cast':
            target = self.ty(e[2], self.owner)
            return self.expr(e[1], env, ctx, lambda v, t, env2: self.cast(v, t, target, env2, k))
        return None
    def panic_default(self, checked, t):
        """the value of a checked operation; its failure is a panic in Rust, which the translation does not preserve."""
        n = self.fresh('result')
        return f'(match {checked} | Option.some {n} => {n} | Option.none => {self.default(t)})'
    def binary(self, op, vs, env, k):
        T, show = self.T, self.T.show
        (a, ta), (b, tb) = vs
        if op == '+' and (T.res(ta) == BYTES or T.res(tb) == BYTES):
            T.unify(ta, BYTES, '+ on strings'); T.unify(tb, BYTES, '+ on strings')
            return k(f'(AxiomOutput::cat {a} {b})', BYTES, env)
        # a byte literal against a byte read as a number (`s.as_bytes()[i] == b'x'`): the literal's value
        import re as _re
        if T.res(ta) == NAT and T.res(tb) == UINT8 and _re.fullmatch(r'\((\d+) : UInt8\)', b): b, tb = _re.fullmatch(r'\((\d+) : UInt8\)', b).group(1), NAT
        if T.res(tb) == NAT and T.res(ta) == UINT8 and _re.fullmatch(r'\((\d+) : UInt8\)', a): a, ta = _re.fullmatch(r'\((\d+) : UInt8\)', a).group(1), NAT
        T.unify(ta, tb, f'operands of {op}')
        # an integer whose type nothing has fixed is an i32 in Rust and may go below zero; as a Nat a subtraction would
        # stop at zero instead, so it is refused rather than translated to a different answer
        if op == '-' and (T.res(ta)[0] == 'Var' or T.was_defaulted(ta) or T.was_defaulted(tb)): raise Unsupported('a subtraction on an integer whose type Rust infers as i32')
        t = T.nat(ta)
        if op in ('==', '!='):
            if is_atom(a) and is_atom(b) or t in (NAT, BYTES, UINT8, I128) or t[0] == 'Fixed': eq = self.equal(a, b, t)
            else: eq = self.atom(a, t, lambda x, env2: self.atom(b, t, lambda y, env3: self.equal(x, y, t), env2), env)
            return k(eq if op == '==' else f'(not {eq})', BOOL, env)
        if op in ('<', '<=', '>', '>='):
            lo, hi = (a, b) if op in ('<', '<=') else (b, a)
            strict = op in ('<', '>')
            if t == NAT: return k(f'({lo} {"<?" if strict else "<=?"} {hi})', BOOL, env)
            if t == I128: return k(f'(RustStd::{"i128Lt" if strict else "i128Le"} {lo} {hi})', BOOL, env)
            if t[0] == 'Fixed':
                if strict: return k(f'(prim less_than {lo} {hi} : Bool)', BOOL, env)
                return k(f'(not (prim less_than {hi} {lo} : Bool))', BOOL, env)
            raise Unsupported(f'{op} on {show(t)}')
        if t == BOOL and op in ('|', '&', '^'):
            # both operands are already evaluated, in order, so the non-short-circuit form is a choice between values
            form = {'|': f'(if {a} then true else {b})', '&': f'(if {a} then {b} else false)', '^': f'(if {a} then (not {b}) else {b})'}
            return k(form[op], BOOL, env)
        if t == NAT and op in ('|', '&', '^', '<<', '>>'): raise Unsupported(f'the bit operator {op} on an unbounded integer')
        if t == NAT:
            form = {'+': f'({a} + {b})', '-': f'(prim subtract {a} {b} : Nat)', '*': f'(prim multiply {a} {b} : Nat)',
                    '/': f'(prim quotient {a} {b} 0 : Nat)', '%': f'(prim remainder {a} {b} 0 : Nat)'}
            return k(form[op], NAT, env)
        if t[0] == 'Fixed':
            prim = {'+': 'checked_add', '-': 'checked_subtract', '*': 'checked_multiply', '/': 'checked_quotient'}.get(op)
            if prim is None: raise Unsupported(f'{op} on {t[1]}')
            return k(self.panic_default(f'(prim {prim} {a} {b} : (Option {t[1]}))', t), t, env)
        raise Unsupported(f'{op} on {show(t)}')
    def cast(self, v, t, target, env, k):
        T, show = self.T, self.T.show
        t = T.nat(t)
        if t == target: return k(v, t, env)
        if t == NAT and target == I128: return k(f'(RustStd::i128Of false {v})', I128, env)
        if t == NAT and target[0] == 'Fixed': return k(f'(RustStd::{"natTo" + target[1]} {v})', target, env)
        if t[0] == 'Fixed' and target == I128:
            n, f = self.fresh('wide'), self.fresh('failure')
            return k(f'(match (RustStd::parseI128 {self.fixed_text(v, t)}) | Result.ok {n} => {n} | Result.error {f} => (RustStd::i128Of false 0))', I128, env)
        if t[0] == 'Fixed' and target == NAT: return k(f'(RustStd::natOfDecimal (prim utf8_encode (prim format_decimal {v} : String) : Bytes))', NAT, env)
        if t[0] == 'Fixed' and target[0] == 'Fixed':
            return k(self.panic_default(f'(prim checked_convert {v} : (Option {target[1]}))', target), target, env)
        if t == UINT8 and target == NAT: return k(f'(prim to_nat {v} : Nat)', NAT, env)
        raise Unsupported(f'cast {show(t)} as {show(target)}')
    def unsupported(self, why): raise Unsupported(why)

    # ----------------------------------------------------------------------------------------------- calls
    def hook_call(self, segs, vs, e, env, ctx, k):
        T, show = self.T, self.T.show
        if len(segs) == 1 and segs[0] in env and env[segs[0]][0] is not None and T.res(env[segs[0]][1])[0] == 'Fn':
            return self.apply_value(env[segs[0]][0], env[segs[0]][1], vs, env, k)
        segs = [self.owner if x == 'Self' else x for x in segs]
        if segs[:2] in (['std', 'cell'], ['core', 'cell'], ['std', 'rc'], ['std', 'sync']) and len(segs) > 2: segs = segs[2:]   # the same types
        name = '::'.join(segs)
        a = [v for v, _ in vs]
        if len(segs) == 2:
            owner, f = segs
            if owner in self.enums and any(v == f for v, _ in self.enums[owner]['variants']):
                t = ('Named', owner); return k(self.ctor_term(t, f, vs), t, env)
            if (owner, f) in self.fns:
                fn = self.fns[(owner, f)]
                if fn['params'] and fn['params'][0][0] == 'self': return None
                if fn['generics']: return None
                return self.apply_user((owner, f), vs, e[2], env, k)
        if len(segs) == 1 and segs[0] in self.structs and self.structs[segs[0]].get('tuple_struct'):
            st = segs[0]; fts = self.field_types(st)
            if len(fts) != len(vs): raise Unsupported(f'tuple struct {st} arity')
            for (fname, ft), (v, vt) in zip(fts, vs): T.unify(ft, vt, f'field {fname}')
            if self.as_inductive(st): return k(f'(ctor {st}.mk {" ".join(a)})', ('Named', st), env)
            return k(f'(record {st} {{{", ".join(f"{fn_} := {v}" for (fn_, _), v in zip(fts, a))}}})', ('Named', st), env)
        if name in ('u64::try_from', 'u32::try_from', 'usize::try_from', 'u16::try_from') and len(vs) == 1:
            T.unify(vs[0][1], NAT, name)
            return k(f'(ctor Result.ok [Nat, Unit] {a[0]})', ('Result', NAT, UNIT), env)
        if name in ('u64::from', 'usize::from', 'u32::from') and len(vs) == 1:
            T.unify(vs[0][1], NAT, name); return k(a[0], NAT, env)
        if name in ('String::new', 'String::with_capacity'): return k(text(''), BYTES, env)
        if name in ('std::ptr::from_ref', 'ptr::from_ref', 'std::ptr::from_mut', 'ptr::from_mut'): return k('()', ('Ptr',), env)
        if name in ('String::from', 'Rc::new', 'Arc::new', 'Box::new', 'Cow::Borrowed', 'Cow::Owned', 'Cell::new', 'RefCell::new'): return k(a[0], vs[0][1], env)
        if name in ('Vec::with_capacity', 'VecDeque::new', 'VecDeque::with_capacity'): t = T.var(); return k(f'(nil {show(t)})', ('List', t), env)
        if name in ('VecDeque::from', 'Vec::from') and len(vs) == 1: return k(a[0], vs[0][1], env)
        if name in ('HashMap::new', 'HashMap::with_capacity', 'BTreeMap::new'):
            x, y = T.var(), T.var(); return k(T.empty_map(x, y), ('Map', x, y), env)
        if name in ('HashSet::new', 'HashSet::with_capacity', 'BTreeSet::new'): t = T.var(); return k(f'(nil {show(t)})', ('Set', t), env)
        if len(segs) == 2 and segs[1] == 'default' and not vs and segs[0] in self.structs:
            custom = next((it.get('custom_default') for it in self.items if it['k'] == 'fn' and it.get('custom_default') is not None), set())
            def plain(n, seen=()):
                if n in custom or n in seen: return False
                return all(self.T.res(ft)[0] != 'Named' or (self.T.res(ft)[1] in self.structs and plain(self.T.res(ft)[1], seen + (n,)))
                           for _, ft in self.field_types(n))
            if plain(segs[0]):                                 # a derived Default: every field's own default
                return k(self.default(('Named', segs[0])), ('Named', segs[0]), env)
        if name in ('Default::default',) or (len(segs) == 2 and segs[1] == 'default' and segs[0] in ('String', 'Vec', 'BTreeMap', 'BTreeSet', 'HashMap', 'HashSet', 'u64', 'usize', 'u32', 'bool', 'Option')):
            v, t = self.deferred(); return k(v, t, env)
        if name in ('std::cmp::max', 'max', 'cmp::max', 'std::cmp::min', 'min', 'cmp::min') and len(vs) == 2:
            T.unify(vs[0][1], vs[1][1], name); t = T.nat(vs[0][1])
            if t != NAT: raise Unsupported(f'{name} on {show(t)}')
            return k(f'(if ({a[0]} <=? {a[1]}) then {a[1] if "max" in name else a[0]} else {a[0] if "max" in name else a[1]})', NAT, env)
        if name in ('std::mem::take', 'mem::take') and len(vs) == 1:
            v, t = self.deferred(); T.unify(t, vs[0][1], 'mem::take')
            return self.assign_into(e[2][0], v, vs[0][1], env, lambda env2: k(a[0], vs[0][1], env2))
        if name in ('std::mem::replace', 'mem::replace') and len(vs) == 2:
            T.unify(vs[0][1], vs[1][1], 'mem::replace')
            return self.assign_into(e[2][0], a[1], vs[0][1], env, lambda env2: k(a[0], vs[0][1], env2))
        return None

    # ----------------------------------------------------------------------------------------------- derived order and maps
    def ascii_on(self, test, b):
        """an ASCII test written over `b`, applied to the byte value b names."""
        import re as _re
        return _re.sub(r'\bb\b', b, test)
    def ordering(self, a, b, t):
        """a.cmp(&b), as an Ordering."""
        return f'(if {self.less(a, b, t)} then (prim compare_bytes 0x 0x01 : Ordering) else (if {self.equal(a, b, t)} then (prim compare_bytes 0x 0x : Ordering) else (prim compare_bytes 0x01 0x : Ordering)))'
    def sort_by_list(self, r, el, m, c, env):
        T, show = self.T, self.T.show
        lt = ('List', el); st = ('Prod', BOOL, lt)
        def before(x, h, env3):
            """x goes before h: the comparator says Less, or x's key is less than h's."""
            if m in ('sort_by', 'sort_unstable_by'):
                body, bt = self.closure_arg(c, env3, [(x, el), (h, el)]); T.unify(bt, ORDERING, 'comparator')
                return f'(prim equal {body} (prim compare_bytes 0x 0x01 : Ordering) : Bool)'
            kx, kt = self.closure_arg(c, env3, [(x, el)]); kh, kt2 = self.closure_arg(c, env3, [(h, el)]); T.unify(kt, kt2, 'sort key')
            return self.less(kx, kh, kt)
        def outer(acc, x, env2):
            env2 = {**env2, '__sorting_item': (x, el)}
            def inner(state, h, env3):
                return (f'(if (first {state}) then (true, (cons {h} (second {state}))) else '
                        f'(if {before(x, h, env3)} then (true, (cons {h} (cons {x} (second {state})))) else (false, (cons {h} (second {state})))))')
            placed = self.fold(acc, el, st, f'(false, (nil {show(el)}))', inner, env2)
            p = self.fresh('placed')
            return f'(let {p} : {show(st)} := {placed}; (AxiomOutput::reversed [{show(el)}] (if (first {p}) then (second {p}) else (cons {x} (second {p})))))'
        return self.fold(r, el, lt, f'(nil {show(el)})', outer, env)
    def less(self, a, b, t):
        T, t = self.T, self.T.nat(self.T.res(t))
        if t == NAT: return f'({a} <? {b})'
        if t[0] == 'Fixed': return f'(prim less_than {a} {b} : Bool)'
        if t == BYTES: return f'(AxiomOutput::lessBytes {a} {b})'
        if t == BOOL: return f'(if {a} then false else {b})'
        if t == I128: return f'(RustStd::i128Lt {a} {b})'
        if t[0] == 'Prod':
            return f'(if {self.less(f"(first {a})", f"(first {b})", t[1])} then true else (if {self.equal(f"(first {a})", f"(first {b})", t[1])} then {self.less(f"(second {a})", f"(second {b})", t[2])} else false))'
        if t[0] == 'Tup':
            # lexicographic, as Rust orders tuples
            test = 'false'
            for i in reversed(range(len(t) - 1)):
                x, y = self.tup_get(a, t, i), self.tup_get(b, t, i)
                test = f'(if {self.less(x, y, t[1 + i])} then true else (if {self.equal(x, y, t[1 + i])} then {test} else false))'
            return test
        if t[0] == 'Named' and t[1] in self.structs and not self.as_inductive(t[1]) and 'Ord' in self.structs[t[1]].get('derives', ()):
            # a struct that derives Ord: its fields, in declaration order
            fts = self.field_types(t[1]); test = 'false'
            for f, ft in reversed(fts):
                x, y = f'(project {a} {f})', f'(project {b} {f})'
                test = f'(if {self.less(x, y, ft)} then true else (if {self.equal(x, y, ft)} then {test} else false))'
            return test
        if t[0] == 'Named' and self.as_inductive(t[1]) or t[0] in ('List', 'Option'):
            def body(name):
                if t[0] == 'List':
                    return (f'(match left | List.nil => (match right | List.nil => false | List.cons rh rt => true) | List.cons lh lt => '
                            f'(match right | List.nil => false | List.cons rh rt => (if {self.less("lh", "rh", t[1])} then true else (if {self.equal("lh", "rh", t[1])} then ({name} lt rt) else false))))')
                if t[0] == 'Option':
                    return (f'(match left | Option.none => (match right | Option.none => false | Option.some r => true) | Option.some l => '
                            f'(match right | Option.none => false | Option.some r => {self.less("l", "r", t[1])}))')
                ctors = self.ctors_of(t); arms = []
                for i, (c, _, fs) in enumerate(ctors):
                    ls = [f'l{j}' for j in range(len(fs))]; inner = []
                    for i2, (c2, _, fs2) in enumerate(ctors):
                        rs = [f'r{j}' for j in range(len(fs2))]
                        if i2 == i:
                            test = 'false'
                            for (_, ft), l, r in reversed(list(zip(fs, ls, rs))):
                                test = f'(if {self.less(l, r, ft)} then true else (if {self.equal(l, r, ft)} then {test} else false))'
                        else: test = 'true' if i < i2 else 'false'
                        inner.append(f'{c2}{"".join(" " + r for r in rs)} => {test}')
                    arms.append(f'{c}{"".join(" " + l for l in ls)} => (match right | {" | ".join(inner)})')
                return f'(match left | {" | ".join(arms)})'
            name = self.define('lt', t, f'left : {T.show(t)}, right : {T.show(t)}', 'Bool', body, 'left')
            return f'({name} {a} {b})'
        if t[0] == 'Named':
            fts = self.field_types(t[1]); test = 'false'
            for f, ft in reversed(fts):
                x, y = f'(project {a} {f})', f'(project {b} {f})'
                test = f'(if {self.less(x, y, ft)} then true else (if {self.equal(x, y, ft)} then {test} else false))'
            return test
        raise Unsupported(f'ordering of {T.show(t)}')
    def key_bytes(self, v, kt):
        """the order-preserving Bytes of a key (see `key_encodes`)."""
        T = self.T; kt = T.res(kt)
        if kt == BYTES: return v
        if kt == NAT: return f'(RustStd::natKey {v})'
        if kt[0] == 'Prod':
            return f'(prim append {self.key_bytes(f"(first {v})", kt[1])} {self.key_bytes(f"(second {v})", kt[2])} : Bytes)'
        if kt[0] == 'Tup':
            parts = [self.key_bytes(self.tup_get(v, kt, i), kt[1 + i]) for i in range(len(kt) - 1)]
            out = parts[-1]
            for p in reversed(parts[:-1]): out = f'(prim append {p} {out} : Bytes)'
            return out
        raise Unsupported(f'a map key of type {T.show(kt)} has no order-preserving encoding')
    def map_op(self, op, kt, vt):
        T, show = self.T, self.T.show
        lt = ('List', ('Prod', kt, vt)); K, V, L = show(kt), show(vt), show(lt)
        if op == 'get':
            def body(name): return f'(match items | List.nil => (ctor Option.none [{V}]) | List.cons head tail => (if {self.equal("(first head)", "key", kt)} then (ctor Option.some [{V}] (second head)) else ({name} tail key)))'
            return self.define('mapget', lt, f'items : {L}, key : {K}', f'(Option {V})', body, 'items', grouped=False)
        if op == 'insert':
            R = f'((Option {V}) * {L})'
            def body(name):
                return (f'(match items | List.nil => ((ctor Option.none [{V}]), (cons (key, value) (nil ({K} * {V})))) | List.cons head tail => '
                        f'(if {self.equal("(first head)", "key", kt)} then ((ctor Option.some [{V}] (second head)), (cons (key, value) tail)) else '
                        f'(if {self.less("key", "(first head)", kt)} then ((ctor Option.none [{V}]), (cons (key, value) (cons head tail))) else '
                        f'(let below : {R} := ({name} tail key value); ((first below), (cons head (second below)))))))')
            return self.define('mapinsert', lt, f'items : {L}, key : {K}, value : {V}', R, body, 'items', grouped=False)
        if op == 'remove':
            R = f'((Option {V}) * {L})'
            def body(name):
                return (f'(match items | List.nil => ((ctor Option.none [{V}]), (nil ({K} * {V}))) | List.cons head tail => '
                        f'(if {self.equal("(first head)", "key", kt)} then ((ctor Option.some [{V}] (second head)), tail) else '
                        f'(let below : {R} := ({name} tail key); ((first below), (cons head (second below))))))')
            return self.define('mapremove', lt, f'items : {L}, key : {K}', R, body, 'items', grouped=False)
        raise Unsupported(op)

    # ----------------------------------------------------------------------------------------------- methods
    def hook_method(self, r, rt, m, args, recv, env, ctx, k):
        T, show = self.T, self.T.show
        head = rt[0]
        done = lambda v, t: k(v, t, env)
        def ca(vs, at=None): return self.closure_arg(args[0], at or env, vs)
        def rebind(new, result, result_t):
            return self.assign_into(recv, new, rt, env, lambda env2: k(result, result_t, env2))
        def arg_vals(cont):
            return self.exprs(args, env, ctx, lambda vs, env2: cont(vs, env2))
        if head == 'Named' and (rt[1], m) in self.fns: return None          # the crate's own methods come first
        if head == 'Named' and m == 'to_string' and not args and (rt[1], 'fmt_display') in self.fns:
            return done(self.display(r, rt), BYTES)
        if m in ('as_slice', 'as_mut_slice', 'as_str', 'as_mut', 'as_deref', 'borrow', 'borrow_mut', 'get_mut', 'by_ref', 'into_owned', 'iter_mut') and not args:
            return done(r, rt)
        # Cell and RefCell, now plain values: get, set, replace, take
        if m == 'get' and not args: return done(r, rt)
        if m == 'set' and len(args) == 1:
            return self.expr(args[0], env, ctx, lambda v2, t2, env2: (T.unify(t2, rt, 'Cell::set'), self.assign_into(recv, v2, rt, env2, lambda env3: k('()', UNIT, env3)))[1])
        if m == 'replace' and len(args) == 1 and head != 'Bytes':
            def replaced(v2, t2, env2):
                if head == 'Option' and T.res(t2) != rt:            # Option::replace(x): Some(x)
                    T.unify(t2, rt[1], 'Option::replace'); new = f'(ctor Option.some [{show(rt[1])}] {v2})'
                else: T.unify(t2, rt, 'Cell::replace'); new = v2
                return self.atom(r, rt, lambda old, env3: self.assign_into(recv, new, rt, env3, lambda env4: k(old, rt, env4)), env2, 'old')
            return self.expr(args[0], env, ctx, replaced)
        if m == 'take' and not args and head not in ('Option', 'List', 'Iter'):
            d, dt = self.deferred(); T.unify(dt, rt, 'Cell::take')
            return self.atom(r, rt, lambda old, env2: self.assign_into(recv, d, rt, env2, lambda env3: k(old, rt, env3)), env, 'old')
        # Option
        if head == 'Option':
            el = rt[1]; n = self.fresh('got')
            if m == 'is_none': return done(f'(match {r} | Option.some {n} => false | Option.none => true)', BOOL)
            if m in ('is_some_and', 'is_none_or'):
                body, _ = ca([(n, el)])
                return done(f'(match {r} | Option.some {n} => {body} | Option.none => {"false" if m == "is_some_and" else "true"})', BOOL)
            if m in ('unwrap', 'expect', 'unwrap_or_default'):
                d, dt = self.deferred(); T.unify(dt, el, m)
                return done(f'(match {r} | Option.some {n} => {n} | Option.none => {d})', el)
            if m == 'unwrap_or_else':
                body, bt = ca([]); T.unify(bt, el, m)
                return done(f'(match {r} | Option.some {n} => {n} | Option.none => {body})', el)
            if m == 'map_or':
                def with_default(d, dt, env2):
                    body, bt = self.closure_arg(args[1], env2, [(n, el)]); T.unify(bt, dt, 'map_or')
                    return k(f'(match {r} | Option.some {n} => {body} | Option.none => {d})', dt, env2)
                return self.expr(args[0], env, ctx, with_default)
            if m == 'map_or_else':
                d, dt = self.closure_arg(args[0], env, []); body, bt = self.closure_arg(args[1], env, [(n, el)]); T.unify(dt, bt, m)
                return done(f'(match {r} | Option.some {n} => {body} | Option.none => {d})', bt)
            if m == 'ok_or':
                return arg_vals(lambda vs, env2: k(f'(match {r} | Option.some {n} => (ctor Result.ok [{show(el)}, {show(vs[0][1])}] {n}) | Option.none => (ctor Result.error [{show(el)}, {show(vs[0][1])}] {vs[0][0]}))', ('Result', el, vs[0][1]), env2))
            if m == 'or':
                return arg_vals(lambda vs, env2: (T.unify(vs[0][1], rt, 'or'), k(f'(match {r} | Option.some {n} => {r} | Option.none => {vs[0][0]})', rt, env2))[1])
            if m == 'or_else':
                body, bt = ca([]); T.unify(bt, rt, m)
                return done(f'(match {r} | Option.some {n} => {r} | Option.none => {body})', rt)
            if m == 'flatten':
                inner = T.var(); T.unify(el, ('Option', inner), 'flatten')
                return done(f'(match {r} | Option.some {n} => {n} | Option.none => (ctor Option.none [{show(inner)}]))', ('Option', inner))
            if m == 'take':
                return rebind(f'(ctor Option.none [{show(el)}])', r, rt)
            if m == 'iter' or m == 'into_iter':
                return done(f'(match {r} | Option.some {n} => (cons {n} (nil {show(el)})) | Option.none => (nil {show(el)}))', ('List', el))
            if m == 'zip':
                return arg_vals(lambda vs, env2: self.zip_opt(r, el, vs[0], env2, k))
        # Result
        if head == 'Result':
            ok_t, err_t = rt[1], rt[2]; n, f = self.fresh('got'), self.fresh('failure')
            if m in ('is_ok', 'is_err'):
                return done(f'(match {r} | Result.ok {n} => {"true" if m == "is_ok" else "false"} | Result.error {f} => {"false" if m == "is_ok" else "true"})', BOOL)
            if m in ('unwrap', 'expect', 'unwrap_or_default'):
                d, dt = self.deferred(); T.unify(dt, ok_t, m)
                return done(f'(match {r} | Result.ok {n} => {n} | Result.error {f} => {d})', ok_t)
            if m == 'unwrap_or':
                return arg_vals(lambda vs, env2: (T.unify(vs[0][1], ok_t, m), k(f'(match {r} | Result.ok {n} => {n} | Result.error {f} => {vs[0][0]})', ok_t, env2))[1])
            if m == 'unwrap_or_else':
                body, bt = ca([(f, err_t)]); T.unify(bt, ok_t, m)
                return done(f'(match {r} | Result.ok {n} => {n} | Result.error {f} => {body})', ok_t)
            if m == 'err':
                return done(f'(match {r} | Result.ok {n} => (ctor Option.none [{show(err_t)}]) | Result.error {f} => (ctor Option.some [{show(err_t)}] {f}))', ('Option', err_t))
            if m == 'and_then':
                body, bt = ca([(n, ok_t)]); u = T.var(); T.unify(bt, ('Result', u, err_t), m)
                return done(f'(match {r} | Result.ok {n} => {body} | Result.error {f} => (ctor Result.error [{show(u)}, {show(err_t)}] {f}))', bt)
        # integers
        if head == 'Fixed':
            ft = rt[1]
            if m == 'to_string' and not args: return done(self.fixed_text(r, rt), BYTES)
            if m in ('is_negative', 'is_positive') and not args:
                return done(f'(prim less_than {r} (0 : {ft}) : Bool)' if m == 'is_negative' else f'(prim less_than (0 : {ft}) {r} : Bool)', BOOL)
            if m == 'abs' and not args:
                return done(f'(if (prim less_than {r} (0 : {ft}) : Bool) then {self.panic_default(f"(prim checked_negate {r} : (Option {ft}))", rt)} else {r})', rt)
            if m in ('max', 'min') and len(args) == 1:
                return self.expr(args[0], env, ctx, lambda v2, t2, env2: (T.unify(t2, rt, m), k(f'(if (prim less_than {r} {v2} : Bool) then {v2 if m == "max" else r} else {r if m == "max" else v2})', rt, env2))[1])
        if head == 'Bytes' and m in ('parse_i64', 'parse_i32', 'parse_isize') and not args:
            ft = {'parse_i64': 'Int64', 'parse_i32': 'Int32', 'parse_isize': 'Int64'}[m]
            return done(f'(RustStd::parse{ft} {r})', ('Result', ('Fixed', ft), UNIT))
        if head in ('Nat', 'Var') and m in ('saturating_add', 'saturating_sub', 'checked_add', 'checked_sub', 'checked_mul', 'saturating_mul',
                                           'wrapping_add', 'max', 'min', 'abs_diff', 'to_string', 'pow', 'is_power_of_two'):
            T.unify(rt, NAT, m)
            def num(vs, env2):
                a = [v for v, _ in vs]
                for _, t in vs: T.unify(t, NAT, m)
                if m in ('saturating_add', 'wrapping_add'): return k(f'({r} + {a[0]})', NAT, env2)
                if m == 'saturating_sub': return k(f'(prim subtract {r} {a[0]} : Nat)', NAT, env2)
                if m in ('saturating_mul',): return k(f'(prim multiply {r} {a[0]} : Nat)', NAT, env2)
                if m == 'checked_add': return k(f'(ctor Option.some [Nat] ({r} + {a[0]}))', ('Option', NAT), env2)
                if m == 'checked_mul': return k(f'(ctor Option.some [Nat] (prim multiply {r} {a[0]} : Nat))', ('Option', NAT), env2)
                if m == 'checked_sub': return k(f'(if ({a[0]} <=? {r}) then (ctor Option.some [Nat] (prim subtract {r} {a[0]} : Nat)) else (ctor Option.none [Nat]))', ('Option', NAT), env2)
                if m == 'max': return k(f'(if ({r} <=? {a[0]}) then {a[0]} else {r})', NAT, env2)
                if m == 'min': return k(f'(if ({r} <=? {a[0]}) then {r} else {a[0]})', NAT, env2)
                if m == 'abs_diff': return k(f'(if ({r} <=? {a[0]}) then (prim subtract {a[0]} {r} : Nat) else (prim subtract {r} {a[0]} : Nat))', NAT, env2)
                if m == 'to_string': return k(f'(AxiomOutput::decimal {r})', BYTES, env2)
                raise Unsupported(f'integer method {m}')
            return arg_vals(num)
        # strings
        if head == 'Bytes':
            if m == 'as_bytes' or m == 'into_bytes': return done(f'(RustStd::natsOf {r})', ('List', NAT))
            if m in ('parse_usize', 'parse_u64', 'parse_u32') and not args:
                return done(f'(RustStd::parseUnsigned {r} {(1 << {"parse_u32": 32}.get(m, 64)) - 1})', ('Result', NAT, UNIT))
            if m == 'to_string' or m == 'to_owned' or m == 'clone': return done(r, BYTES)
            def strs(vs, env2):
                a = [v for v, _ in vs]
                if m == 'contains' and vs[0][1] in (BYTES,) or (m == 'contains' and T.res(vs[0][1]) == BYTES):
                    return k(f'(match (LexLeanStd.Text::find {r} {a[0]}) | Option.some at => true | Option.none => false)', BOOL, env2)
                if m == 'find' and T.res(vs[0][1]) == BYTES: return k(f'(LexLeanStd.Text::find {r} {a[0]})', ('Option', NAT), env2)
                if m == 'replace': return k(f'(RustStd::replaceAll {r} {a[0]} {a[1]})', BYTES, env2)
                if m == 'repeat': return k(f'(RustStd::repeatBytes {r} {a[0]})', BYTES, env2)
                if m == 'split' and T.res(vs[0][1]) == BYTES: return k(f'(RustStd::splitStr {r} {a[0]})', ('List', BYTES), env2)
                return None
            if m in ('contains', 'find', 'replace', 'repeat', 'split'):
                res = arg_vals(strs)
                if res is not None: return res
        # lists
        if head in ('List', 'Iter'):
            el = rt[1]
            if m in ('sum_u64', 'sum_usize') and not args:
                T.unify(el, NAT, m); A, X = self.fresh('acc'), self.fresh('x')
                return done(f'(prim list_fold (fun ({A} : Nat) ({X} : Nat) => ({A} + {X})) 0 {r} : Nat)', NAT)
            if m == 'find_map' and len(args) == 1:
                # the first element the closure maps to Some, in order
                out_t = T.var()
                def pick(acc, x, env2):
                    body, bt = self.closure_arg(args[0], {**env2, '__found_item': (x, el)}, [(x, el)]); T.unify(bt, ('Option', out_t), 'find_map')
                    return f'(match {acc} | Option.some {self.fresh("seen")} => {acc} | Option.none => {body})'
                return done(self.fold(r, el, ('Option', out_t), f'(ctor Option.none [{show(out_t)}])', pick, env), ('Option', out_t))
            if m == 'fold' and len(args) == 2 and args[1][0] == 'closure':
                def with_init(vs, env2):
                    acc_t = vs[0][1]
                    def step(acc, x, env3):
                        env3 = {**env3, '__folding_item': (x, el)}
                        body, bt = self.closure_arg(args[1], env3, [(acc, acc_t), (x, el)]); T.unify(bt, acc_t, 'fold'); return body
                    return k(self.fold(r, el, acc_t, vs[0][0], step, env2), acc_t, env2)
                return self.exprs([args[0]], env, ctx, with_init)
            if m == 'extend' and len(args) == 1:
                return arg_vals(lambda vs, env2: (T.unify(vs[0][1], ('List', el), 'extend'), self.assign_into(recv, f'(RustStd::appendList [{show(el)}] {r} {vs[0][0]})', rt, env2, lambda env3: k('()', UNIT, env3)))[1])
            if m == 'starts_with' and len(args) == 1:
                def prefix(vs, env2):
                    T.unify(vs[0][1], ('List', el), 'starts_with'); p = vs[0][0]
                    n = f'(AxiomOutput::listLength [{show(el)}] {p})'
                    return k(f'(if ({n} <=? (AxiomOutput::listLength [{show(el)}] {r})) then {self.equal(f"(RustStd::takeN [{show(el)}] {r} {n})", p, ("List", el))} else false)', BOOL, env2)
                return arg_vals(prefix)
            if m in ('join', 'concat') and T.res(el) == BYTES:
                if m == 'concat': return done(f'(RustStd::joinWith {r} {text("")})', BYTES)
                return arg_vals(lambda vs, env2: k(f'(RustStd::joinWith {r} {vs[0][0]})', BYTES, env2))
            if m == 'concat':
                x = T.var(); T.unify(el, ('List', x), 'concat'); return done(f'(RustStd::concat [{show(x)}] {r})', ('List', x))
            if m == 'last': return done(f'(RustStd::lastOf [{show(el)}] {r})', ('Option', el))
            if m == 'count': return done(f'(AxiomOutput::listLength [{show(el)}] {r})', NAT)
            if m in ('take', 'skip'):
                return arg_vals(lambda vs, env2: (T.unify(vs[0][1], NAT, m), k(f'(RustStd::{"takeN" if m == "take" else "dropN"} [{show(el)}] {r} {vs[0][0]})', rt, env2))[1])
            if m == 'nth':
                return arg_vals(lambda vs, env2: k(f'(RustStd::getAt [{show(el)}] {r} {vs[0][0]})', ('Option', el), env2))
            if m == 'chain':
                return arg_vals(lambda vs, env2: (T.unify(vs[0][1], ('List', el), 'chain'), k(f'(RustStd::appendList [{show(el)}] {r} {vs[0][0]})', ('List', el), env2))[1])
            if m == 'extend_from_slice':
                return arg_vals(lambda vs, env2: (T.unify(vs[0][1], ('List', el), m), self.assign_into(recv, f'(RustStd::appendList [{show(el)}] {r} {vs[0][0]})', rt, env2, lambda env3: k('()', UNIT, env3)))[1])
            if m == 'clear': return rebind(f'(nil {show(el)})', '()', UNIT)
            if m in ('front', 'back') and not args:
                return done(f'(RustStd::{"headOf" if m == "front" else "lastOf"} [{show(el)}] {r})', ('Option', el))
            if m == 'pop_front' and not args:
                return rebind(f'(RustStd::tailOf [{show(el)}] {r})', f'(RustStd::headOf [{show(el)}] {r})', ('Option', el))
            if m == 'pop_back' and not args:
                return rebind(f'(RustStd::dropLast [{show(el)}] {r})', f'(RustStd::lastOf [{show(el)}] {r})', ('Option', el))
            if m in ('push_back', 'push_front') and len(args) == 1:
                return self.expr(args[0], env, ctx, lambda v2, t2, env2: (T.unify(t2, el, m), self.assign_into(recv, f'(RustStd::push [{show(el)}] {r} {v2})' if m == 'push_back' else f'(cons {v2} {r})', rt, env2, lambda env3: k('()', UNIT, env3)))[1])
            if m in ('sum', 'max', 'min') and not args:
                T.unify(el, NAT, m)
                if m == 'sum':
                    A, X = self.fresh('acc'), self.fresh('x')
                    return done(f'(prim list_fold (fun ({A} : Nat) ({X} : Nat) => ({A} + {X})) 0 {r} : Nat)', NAT)
                pick = '(if (acc <=? x) then x else acc)' if m == 'max' else '(if (acc <=? x) then acc else x)'
                return done(f'(prim list_fold (fun (best : (Option Nat)) (x : Nat) => (match best | Option.some acc => (ctor Option.some [Nat] {pick}) | Option.none => (ctor Option.some [Nat] x))) (ctor Option.none [Nat]) {r} : (Option Nat))', ('Option', NAT))
            if m in ('position', 'filter', 'filter_map', 'flat_map', 'retain', 'any', 'all'):
                if m == 'position':
                    acc_t = ('Prod', NAT, ('Option', NAT))
                    def step(acc, x, env2):
                        body, _ = ca([(x, el)], env2); f = self.fresh('found')
                        return f'(match (second {acc}) | Option.some {f} => {acc} | Option.none => (((first {acc}) + 1), (if {body} then (ctor Option.some [Nat] (first {acc})) else (ctor Option.none [Nat]))))'
                    return done(f'(second {self.fold(r, el, acc_t, "(0, (ctor Option.none [Nat]))", step, env)})', ('Option', NAT))
                if m in ('filter', 'retain'):
                    def step(acc, x, env2):
                        body, _ = ca([(x, el)], env2); return f'(if {body} then (cons {x} {acc}) else {acc})'
                    kept = f'(AxiomOutput::reversed [{show(el)}] {self.fold(r, el, ("List", el), f"(nil {show(el)})", step, env)})'
                    if m == 'retain': return rebind(kept, '()', UNIT)
                    return done(kept, ('List', el))
                if m == 'filter_map':
                    out_t = T.var()
                    def step(acc, x, env2):
                        body, bt = ca([(x, el)], env2); T.unify(bt, ('Option', out_t), m); g = self.fresh('kept')
                        return f'(match {body} | Option.some {g} => (cons {g} {acc}) | Option.none => {acc})'
                    return done(f'(AxiomOutput::reversed [{show(out_t)}] {self.fold(r, el, ("List", out_t), f"(nil {show(out_t)})", step, env)})', ('List', out_t))
                if m == 'flat_map':
                    out_t = T.var()
                    def step(acc, x, env2):
                        body, bt = ca([(x, el)], env2); T.unify(bt, ('List', out_t), m)
                        return f'(RustStd::appendList [{show(out_t)}] {acc} {body})'
                    return done(self.fold(r, el, ('List', out_t), f'(nil {show(out_t)})', step, env), ('List', out_t))
            if m == 'unzip':
                x, y = T.var(), T.var(); T.unify(el, ('Prod', x, y), 'unzip')
                return done(f'(RustStd::unzip [{show(x)}, {show(y)}] {r})', ('Prod', ('List', x), ('List', y)))
            if m in ('split_first', 'split_last'):
                fn_ = 'splitFirst' if m == 'split_first' else 'splitLast'
                return done(f'(RustStd::{fn_} [{show(el)}] {r})', ('Option', ('Prod', el, ('List', el))))
            if m == 'windows':
                return arg_vals(lambda vs, env2: k(f'(RustStd::windows [{show(el)}] {r} {vs[0][0]})', ('List', ('List', el)), env2))
            if m in ('max_by_key', 'min_by_key') and len(args) == 1:
                # Rust's ties: max_by_key keeps the last greatest, min_by_key the first least
                ot = ('Option', el)
                def pick(acc, x, env2):
                    env2 = {**env2, '__picking_item': (x, el)}
                    best = self.fresh('best')
                    kx, kt = self.closure_arg(args[0], env2, [(x, el)])
                    kb, kt2 = self.closure_arg(args[0], {**env2, '__picking_best': (best, el)}, [(best, el)]); T.unify(kt, kt2, m)
                    better = f'(not {self.less(kx, kb, kt)})' if m == 'max_by_key' else self.less(kx, kb, kt)
                    return f'(match {acc} | Option.none => (ctor Option.some [{show(el)}] {x}) | Option.some {best} => (if {better} then (ctor Option.some [{show(el)}] {x}) else {acc}))'
                return done(self.fold(r, el, ot, f'(ctor Option.none [{show(el)}])', pick, env), ot)
            if m in ('sort_by', 'sort_unstable_by', 'sort_by_key', 'sort_unstable_by_key', 'sort_by_cached_key') and len(args) == 1:
                return rebind(self.sort_by_list(r, el, m, args[0], env), '()', UNIT)
            if m in ('sort', 'sort_unstable') and not args:
                return rebind(self.sort_list(r, el), '()', UNIT)
            if m == 'dedup' and not args:
                def body(name):
                    return (f'(match items | List.nil => (nil {show(el)}) | List.cons head tail => (match tail | List.nil => (cons head (nil {show(el)})) | '
                            f'List.cons next more => (if {self.equal("head", "next", el)} then ({name} tail) else (cons head ({name} tail)))))')
                def fold():
                    return (f'(AxiomOutput::reversed [{show(el)}] (prim list_fold (fun (acc : (List {show(el)})) (x : {show(el)}) => (match acc | List.nil => (cons x acc) | '
                            f'List.cons last rest => (if {self.equal("last", "x", el)} then acc else (cons x acc)))) (nil {show(el)}) items : (List {show(el)})))')
                fn_ = self.define('dedup', ('List', el), f'items : (List {show(el)})', f'(List {show(el)})', body, 'items', grouped=False, fold=fold)
                return rebind(f'({fn_} {r})', '()', UNIT)
            if m == 'insert' and len(args) == 2:
                return arg_vals(lambda vs, env2: (T.unify(vs[1][1], el, 'insert'), self.assign_into(recv, f'(RustStd::insertAt [{show(el)}] {r} {vs[0][0]} {vs[1][0]})', rt, env2, lambda env3: k('()', UNIT, env3)))[1])
            if m == 'remove' and len(args) == 1:
                return arg_vals(lambda vs, env2: (T.unify(vs[0][1], NAT, 'remove'),
                    self.assign_into(recv, f'(RustStd::removeAt [{show(el)}] {r} {vs[0][0]})', rt, env2,
                                     lambda env3: k(f'(match (RustStd::getAt [{show(el)}] {r} {vs[0][0]}) | Option.some x => x | Option.none => {self.default(el)})', el, env3)))[1])
        if m in ('cmp', 'total_cmp') and len(args) == 1:
            return arg_vals(lambda vs, env2: (T.unify(vs[0][1], rt, 'cmp'), k(self.ordering(r, vs[0][0], rt), ORDERING, env2))[1])
        if rt == ORDERING and m == 'reverse' and not args:
            return done(f'(if (prim equal {r} (prim compare_bytes 0x 0x01 : Ordering) : Bool) then (prim compare_bytes 0x01 0x : Ordering) else (if (prim equal {r} (prim compare_bytes 0x01 0x : Ordering) : Bool) then (prim compare_bytes 0x 0x01 : Ordering) else {r}))', ORDERING)
        if rt == ORDERING and m == 'then' and len(args) == 1:
            return arg_vals(lambda vs, env2: k(f'(if (prim equal {r} (prim compare_bytes 0x 0x : Ordering) : Bool) then {vs[0][0]} else {r})', ORDERING, env2))
        if rt == ORDERING and m == 'then_with' and len(args) == 1:
            body, bt = self.closure_arg(args[0], env, []); T.unify(bt, ORDERING, 'then_with')
            return done(f'(if (prim equal {r} (prim compare_bytes 0x 0x : Ordering) : Bool) then {body} else {r})', ORDERING)
        if rt == ORDERING and m in ('is_lt', 'is_eq', 'is_gt', 'is_le', 'is_ge', 'is_ne') and not args:
            o = {'is_lt': '(prim compare_bytes 0x 0x01 : Ordering)', 'is_eq': '(prim compare_bytes 0x 0x : Ordering)', 'is_gt': '(prim compare_bytes 0x01 0x : Ordering)'}
            if m in o: return done(f'(prim equal {r} {o[m]} : Bool)', BOOL)
            neg = {'is_le': '(prim compare_bytes 0x01 0x : Ordering)', 'is_ge': '(prim compare_bytes 0x 0x01 : Ordering)', 'is_ne': '(prim compare_bytes 0x 0x : Ordering)'}[m]
            return done(f'(not (prim equal {r} {neg} : Bool))', BOOL)
        ascii_tests = {'is_ascii': '(b <? 128)', 'is_ascii_digit': '(if (47 <? b) then (b <? 58) else false)', 'is_ascii_hexdigit': '(if (if (47 <? b) then (b <? 58) else false) then true else (if (if (64 <? b) then (b <? 71) else false) then true else (if (96 <? b) then (b <? 103) else false)))', 'is_ascii_lowercase': '(if (96 <? b) then (b <? 123) else false)', 'is_ascii_uppercase': '(if (64 <? b) then (b <? 91) else false)', 'is_ascii_alphabetic': '(if (if (64 <? b) then (b <? 91) else false) then true else (if (96 <? b) then (b <? 123) else false))', 'is_ascii_alphanumeric': '(if (if (47 <? b) then (b <? 58) else false) then true else (if (if (64 <? b) then (b <? 91) else false) then true else (if (96 <? b) then (b <? 123) else false)))', 'is_ascii_whitespace': '(if (b =? 32) then true else (if (b =? 9) then true else (if (b =? 10) then true else (if (b =? 12) then true else (b =? 13)))))', 'is_ascii_punctuation': '(if (if (32 <? b) then (b <? 48) else false) then true else (if (if (57 <? b) then (b <? 65) else false) then true else (if (if (90 <? b) then (b <? 97) else false) then true else (if (122 <? b) then (b <? 127) else false))))', 'is_ascii_control': '(if (b <? 32) then true else (b =? 127))'}
        if m in ascii_tests and not args and rt in (NAT, BYTES):
            test = ascii_tests[m]
            if rt == NAT:
                if m == 'is_ascii': return done(f'({r} <? 128)', BOOL)
                return done(f'(if ({r} <? 256) then {test.replace("b", r) if False else self.ascii_on(test, r)} else false)', BOOL)
            b = self.fresh('code')
            return done(f'(match (RustStd::asciiByte {r}) | Option.some {b} => {self.ascii_on(test, b)} | Option.none => false)', BOOL)
        if rt == BYTES and m == 'chars' and not args: return done(f'(RustStd::utf8Chars {r})', ('List', BYTES))
        if rt == BYTES and m == 'is_control' and not args:
            # Unicode's Cc: U+0000 to U+001F and U+007F to U+009F, one byte or C2 80 to C2 9F
            b = self.fresh('code')
            return done(f'(match (RustStd::asciiByte {r}) | Option.some {b} => (if ({b} <? 32) then true else ({b} =? 127)) | Option.none => (RustStd::isC1Control {r}))', BOOL)
        if rt == BOOL and m == 'then_some' and len(args) == 1:
            return arg_vals(lambda vs, env2: k(f'(if {r} then (ctor Option.some [{show(vs[0][1])}] {vs[0][0]}) else (ctor Option.none [{show(vs[0][1])}]))', ('Option', vs[0][1]), env2))
        if rt == NAT and m == 'is_multiple_of' and len(args) == 1:
            # Rust: n.is_multiple_of(0) is n == 0
            return arg_vals(lambda vs, env2: (T.unify(vs[0][1], NAT, m), k(f'(if ({vs[0][0]} =? 0) then ({r} =? 0) else ((prim remainder {r} {vs[0][0]} 0 : Nat) =? 0))', BOOL, env2))[1])
        if head == 'Result' and m == 'map_or' and len(args) == 2:
            def mapped(vs, env2):
                v = self.fresh('ok'); body, bt = self.closure_arg(args[1], {**env2, '__ok_value': (v, rt[1])}, [(v, rt[1])]); T.unify(bt, vs[0][1], 'map_or')
                return k(f'(match {r} | Result.ok {v} => {body} | Result.error {self.fresh("failure")} => {vs[0][0]})', vs[0][1], env2)
            return self.exprs([args[0]], env, ctx, mapped)
        if rt == BYTES and m == 'char_indices' and not args: return done(f'(RustStd::charIndices {r})', ('List', ('Prod', NAT, BYTES)))
        if rt == BYTES and m == 'is_whitespace' and not args: return done(f'(RustStd::isWhitespaceChar {r})', BOOL)
        if rt == BYTES and m == 'len_utf8' and not args: return done(f'(prim length {r} : Nat)', NAT)
        # maps
        if head == 'Map' and T.res(rt[1]) == ('Ptr',):
            vt = rt[2]; none = f'(ctor Option.none [{show(vt)}])'
            if m == 'get': return arg_vals(lambda vs, env2: k(none, ('Option', vt), env2))
            if m == 'contains_key': return arg_vals(lambda vs, env2: k('false', BOOL, env2))
            if m == 'insert': return arg_vals(lambda vs, env2: (T.unify(vs[1][1], vt, 'cache value'), k(none, ('Option', vt), env2))[1])
            if m == 'remove': return arg_vals(lambda vs, env2: k(none, ('Option', vt), env2))
            raise Unsupported(f'method `{m}` on a cache keyed by address')
        if head == 'Map' and m in ('clone', 'to_owned'): return done(r, rt)
        if head == 'Map' and m in ('get', 'contains_key', 'insert', 'remove', 'iter', 'into_iter', 'keys', 'values', 'into_keys', 'into_values', 'len', 'is_empty') and T.map_rep(rt[1], rt[2]) is None and m not in ('get', 'contains_key', 'insert', 'remove'):
            raise Unsupported('a map whose key type is not known where it is used')
        if head == 'Map' and (T.map_rep(rt[1], rt[2]) in ('tree', 'keyed') or (T.map_rep(rt[1], rt[2]) is None and m in ('get', 'contains_key', 'insert', 'remove'))):
            kt, vt = rt[1], rt[2]
            def stored(rep):
                """the tree's value type, and how an entry found in it gives back the map's value."""
                V = show(vt)
                if rep == 'tree': return V, (lambda found: found)
                return f'({show(kt)} * {V})', (lambda found: f'(second {found})')
            def tree_ops(vs, env2):
                a = [v for v, _ in vs]
                T.unify(vs[0][1], kt, f'map key of {m}')
                if m == 'insert': T.unify(vs[1][1], vt, 'map value')
                rep = T.map_rep(kt, vt)
                if rep == 'list':
                    # the key turned out to have no tree: the list form, as before these rules
                    if m == 'get': return k(f'({self.map_op("get", kt, vt)} {r} {a[0]})', ('Option', vt), env2)
                    if m == 'contains_key':
                        n = self.fresh('got'); return k(f'(match ({self.map_op("get", kt, vt)} {r} {a[0]}) | Option.some {n} => true | Option.none => false)', BOOL, env2)
                    p = self.fresh('updated'); pt = ('Prod', ('Option', vt), rt)
                    call = f'({self.map_op(m, kt, vt)} {r} {" ".join(a)})'
                    return f'(let {p} : {show(pt)} := {call}; {self.assign_into(recv, f"(second {p})", rt, env2, lambda env3: k(f"(first {p})", ("Option", vt), env3))})'
                if rep not in ('tree', 'keyed'): raise Unsupported('a map whose key type is not known where it is used')
                V = show(vt); SV, unwrap = stored(rep); key = self.key_bytes(a[0], kt)
                raw = f'(LexLeanStd.OrdMap::mapGet [{SV}] {r} {key})'
                if rep == 'tree': got = raw
                else:
                    found = self.fresh('found')
                    got = f'(match {raw} | Option.some {found} => (ctor Option.some [{V}] {unwrap(found)}) | Option.none => (ctor Option.none [{V}]))'
                if m == 'get': return k(got, ('Option', vt), env2)
                if m == 'contains_key':
                    n = self.fresh('got'); return k(f'(match {raw} | Option.some {n} => true | Option.none => false)', BOOL, env2)
                if m == 'insert':
                    held = a[1] if rep == 'tree' else f'({a[0]}, {a[1]})'
                    after = f'(LexLeanStd.OrdMap::mapInsert [{SV}] {r} {key} {held})'
                else: after = f'(LexLeanStd.OrdMap::mapRemove [{SV}] {r} {key})'
                p = self.fresh('updated'); pt = ('Prod', ('Option', vt), rt)
                return f'(let {p} : {show(pt)} := ({got}, {after}); {self.assign_into(recv, f"(second {p})", rt, env2, lambda env3: k(f"(first {p})", ("Option", vt), env3))})'
            if m in ('get', 'contains_key', 'insert', 'remove'): return arg_vals(tree_ops)
            rep = T.map_rep(kt, vt); V = show(vt); SV, _ = stored(rep)
            raw_entries = f'(LexLeanStd.OrdMap::mapEntries [{SV}] {r})'
            KV = f'({show(kt)} * {V})'
            A, E = self.fresh('acc'), self.fresh('entry')
            entries = raw_entries if rep == 'tree' else f'(AxiomOutput::reversed [{KV}] (prim list_fold (fun ({A} : (List {KV})) ({E} : (Bytes * {KV})) => (cons (second {E}) {A})) (nil {KV}) {raw_entries} : (List {KV})))'
            if m in ('iter', 'into_iter'):
                if self.hash_receiver(recv, env): raise Unsupported('iteration over a hash map (its order would matter)')
                return done(entries, ('List', ('Prod', kt, vt)))
            if m in ('keys', 'values', 'into_keys', 'into_values'):
                if self.hash_receiver(recv, env): raise Unsupported('iteration over a hash map (its order would matter)')
                part = 'first' if 'keys' in m else 'second'; out_t = kt if 'keys' in m else vt
                A2, E2 = self.fresh('acc'), self.fresh('entry')
                return done(f'(AxiomOutput::reversed [{show(out_t)}] (prim list_fold (fun ({A2} : (List {show(out_t)})) ({E2} : {KV}) => (cons ({part} {E2}) {A2})) (nil {show(out_t)}) {entries} : (List {show(out_t)})))', ('List', out_t))
            if m == 'len': return done(f'(LexLeanStd.OrdMap::mapSize [{SV}] {r})', NAT)
            if m == 'is_empty': return done(f'((LexLeanStd.OrdMap::mapSize [{SV}] {r}) =? 0)', BOOL)
            raise Unsupported(f'method `{m}` on a map')
        if head == 'Map':
            kt, vt = rt[1], rt[2]
            if m == 'iter' or m == 'into_iter':
                if self.hash_receiver(recv, env): raise Unsupported('iteration over a hash map (its order would matter)')
                return done(r, ('List', ('Prod', kt, vt)))
            if m in ('keys', 'values', 'into_keys', 'into_values'):
                if self.hash_receiver(recv, env): raise Unsupported('iteration over a hash map (its order would matter)')
                part = 'first' if 'keys' in m else 'second'; out_t = kt if 'keys' in m else vt
                A, E = self.fresh('acc'), self.fresh('entry')
                return done(f'(AxiomOutput::reversed [{show(out_t)}] (prim list_fold (fun ({A} : (List {show(out_t)})) ({E} : ({show(kt)} * {show(vt)})) => (cons ({part} {E}) {A})) (nil {show(out_t)}) {r} : (List {show(out_t)})))', ('List', out_t))
            if m == 'len': return done(f'(AxiomOutput::listLength [({show(kt)} * {show(vt)})] {r})', NAT)
            if m == 'is_empty': return done(f'((AxiomOutput::listLength [({show(kt)} * {show(vt)})] {r}) =? 0)', BOOL)
            def mp(vs, env2):
                a = [v for v, _ in vs]
                T.unify(vs[0][1], kt, f'map key of {m}')
                if m == 'get': return k(f'({self.map_op("get", kt, vt)} {r} {a[0]})', ('Option', vt), env2)
                if m == 'contains_key':
                    n = self.fresh('got'); return k(f'(match ({self.map_op("get", kt, vt)} {r} {a[0]}) | Option.some {n} => true | Option.none => false)', BOOL, env2)
                if m in ('insert', 'remove'):
                    if m == 'insert': T.unify(vs[1][1], vt, 'map value')
                    p = self.fresh('updated'); pt = ('Prod', ('Option', vt), rt)
                    call = f'({self.map_op(m, kt, vt)} {r} {" ".join(a)})'
                    return f'(let {p} : {show(pt)} := {call}; {self.assign_into(recv, f"(second {p})", rt, env2, lambda env3: k(f"(first {p})", ("Option", vt), env3))})'
                raise Unsupported(f'method `{m}` on a map')
            if m in ('get', 'contains_key', 'insert', 'remove'): return arg_vals(mp)
        if head == 'Set':
            if m == 'remove':
                el = rt[1]
                return arg_vals(lambda vs, env2: (T.unify(vs[0][1], el, 'set remove'),
                    self.assign_into(recv, self.set_without(r, vs[0][0], el), rt, env2, lambda env3: k(self.member(r, vs[0][0], el), BOOL, env3)))[1])
        return None

    def zip_opt(self, r, el, other, env, k):
        T, show = self.T, self.T.show
        o_t = T.var(); T.unify(other[1], ('Option', o_t), 'zip')
        n1, n2 = self.fresh('got'), self.fresh('got')
        pt = ('Prod', el, o_t)
        return k(f'(match {r} | Option.some {n1} => (match {other[0]} | Option.some {n2} => (ctor Option.some [{show(pt)}] ({n1}, {n2})) | Option.none => (ctor Option.none [{show(pt)}])) | Option.none => (ctor Option.none [{show(pt)}]))', ('Option', pt), env)
    def hash_receiver(self, recv, env):
        return False
    def set_without(self, s, x, el):
        show = self.T.show
        return f'(prim list_fold (fun [{x}] (acc : (List {show(el)})) (y : {show(el)}) => (if {self.equal("y", x, el)} then acc else (cons y acc))) (nil {show(el)}) (AxiomOutput::reversed [{show(el)}] {s}) : (List {show(el)}))' if is_atom(x) else self.unsupported('set remove of a non-variable')
    def sort_list(self, r, el):
        show = self.T.show
        def ins(name):
            return (f'(match sorted | List.nil => (cons item (nil {show(el)})) | List.cons head tail => '
                    f'(if {self.less("item", "head", el)} then (cons item (cons head tail)) else (cons head ({name} tail item))))')
        def fold():
            st = f'(Bool * (List {show(el)}))'
            step = (f'(fun [item] (acc : {st}) (head : {show(el)}) => (if (first acc) then (true, (cons head (second acc))) else '
                    f'(if {self.less("item", "head", el)} then (true, (cons head (cons item (second acc)))) else (false, (cons head (second acc))))))')
            return (f'(let placed : {st} := (prim list_fold {step} (false, (nil {show(el)})) sorted : {st}); '
                    f'(AxiomOutput::reversed [{show(el)}] (if (first placed) then (second placed) else (cons item (second placed)))))')
        insert = self.define('insertsorted', ('List', el), f'sorted : (List {show(el)}), item : {show(el)}', f'(List {show(el)})', ins, 'sorted', grouped=False, fold=fold)
        A, X = self.fresh('acc'), self.fresh('x')
        return f'(prim list_fold (fun ({A} : (List {show(el)})) ({X} : {show(el)}) => ({insert} {A} {X})) (nil {show(el)}) {r} : (List {show(el)}))'

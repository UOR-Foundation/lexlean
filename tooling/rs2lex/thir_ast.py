"""A body of rustc's typed tree as the translator's syntax.

What rustc has already decided is taken from it: which variable a name means (each binding has an id, so each gets a
name of its own), which function a call reaches, where a borrow or a dereference is, how a `for` or a `?` unfolds.
The `for` loop and `?` are folded back to the forms the translator has rules for. A construct this file has no rule
for raises `Unhandled`, and the caller keeps the body the source parser made: nothing is guessed."""
import re
import thir

WILD = ('pwild',)
BIN = {'Add': '+', 'Sub': '-', 'Mul': '*', 'Div': '/', 'Rem': '%', 'Eq': '==', 'Ne': '!=', 'Lt': '<', 'Le': '<=', 'Gt': '>',
       'Ge': '>=', 'BitAnd': '&', 'BitOr': '|', 'BitXor': '^', 'Shl': '<<', 'Shr': '>>'}
# associated functions of the standard library, called on the type and not on a first argument
ASSOCIATED = {'new', 'with_capacity', 'from', 'default', 'from_utf8', 'from_iter', 'from_str', 'try_from', 'from_utf8_lossy'}
TRAIT_OPS = {('Add', 'add'): '+', ('Sub', 'sub'): '-', ('Mul', 'mul'): '*', ('Div', 'div'): '/', ('Rem', 'rem'): '%',
             ('PartialEq', 'eq'): '==', ('PartialEq', 'ne'): '!=', ('PartialOrd', 'lt'): '<', ('PartialOrd', 'le'): '<=',
             ('PartialOrd', 'gt'): '>', ('PartialOrd', 'ge'): '>=', ('BitAnd', 'bitand'): '&', ('BitOr', 'bitor'): '|',
             ('BitXor', 'bitxor'): '^'}
TRAIT_ASSIGN = {('AddAssign', 'add_assign'): '+', ('SubAssign', 'sub_assign'): '-', ('MulAssign', 'mul_assign'): '*',
                ('DivAssign', 'div_assign'): '/', ('RemAssign', 'rem_assign'): '%', ('BitOrAssign', 'bitor_assign'): '|',
                ('BitAndAssign', 'bitand_assign'): '&'}
WRAPPERS = ('Scope', 'Use', 'NeverToAny', 'PlaceTypeAscription', 'ValueTypeAscription', 'PlaceUnwrapUnsafeBinder', 'PointerCoercion')
FNDEF = re.compile(r'FnDef\(DefId\(\d+:\d+ ~ (\w+)\[\w+\]::([^)]*)\)')
VARID = re.compile(r'LocalVarId\(HirId\(DefId\([^)]*\)\.(\d+)\)\)')


class Unhandled(Exception):
    pass


def head_type(ty):
    """the name a type is known by: `std::vec::Vec<usize>` is Vec, `&'{erased} mut lib::Shape` is Shape."""
    ty = re.sub(r"&'\{erased\} (mut )?|&(mut )?", '', ty).strip()
    ty = ty.split('<')[0]
    return ty.split('::')[-1].strip()


def deep(node, key, depth=3):
    """the first field `key:` at or within `depth` levels below a node (some kinds nest their fields one level down)."""
    level = [node]
    for _ in range(depth):
        nxt = []
        for n in level:
            f = n.field(key)
            if f is not None: return f
            nxt += [k for k in n.kids if not k.text.startswith(('Expr', 'Pat ', 'Pat{', 'Arm', 'Stmt'))]
        level = nxt
    return None


def deep_value(node, key, default=None):
    f = deep(node, key)
    return f.text[len(key) + 1:].strip() if f is not None else default


def owner_of(node, key):
    """the node whose own field `key:` is (see `deep`)."""
    level = [node]
    for _ in range(3):
        nxt = []
        for n in level:
            if n.field(key) is not None: return n
            nxt += n.kids
        level = nxt
    return node


def split_top(text, sep=','):
    out, depth, cur = [], 0, ''
    for ch in text:
        if ch in '<([': depth += 1
        elif ch in '>)]': depth -= 1
        if ch == sep and depth == 0: out.append(cur.strip()); cur = ''
        else: cur += ch
    if cur.strip(): out.append(cur.strip())
    return out


def type_ast(text):
    """a type as rustc prints it, in the translator's type syntax; None where it has no such form (a closure, a
    function item, `dyn`), so that no annotation is made of it."""
    t = text.strip()
    if not t or any(x in t for x in ('Closure(', 'FnDef(', 'dyn ', 'impl ', '{closure', 'fn(', 'Binder')): return None
    m = re.match(r"&('\{erased\} |'\w+ )?(mut )?(.*)$", t)
    if m:
        inner = type_ast(m.group(3))
        return None if inner is None else (('mut', inner) if m.group(2) else ('ref', inner))
    if t.startswith('(') and t.endswith(')'):
        parts = [type_ast(x) for x in split_top(t[1:-1])]
        return None if any(x is None for x in parts) else ('tuple', parts)
    if t.startswith('[') and t.endswith(']'):
        inner = type_ast(split_top(t[1:-1], ';')[0])
        return None if inner is None else ('path', 'Vec', [inner])
    if t == 'str': return ('path', 'str', [])
    head, args = t, []
    if '<' in t and t.endswith('>'):
        head = t[:t.index('<')]
        for a in split_top(t[t.index('<') + 1:-1]):
            if a.startswith("'") or a.endswith('alloc::Global') or a.endswith('RandomState'): continue
            x = type_ast(a)
            if x is None: return None
            args.append(x)
    name = head.split('::')[-1]
    if not re.fullmatch(r'\w+', name): return None
    return ('path', name, args)


class Body:
    def __init__(self, tree, all_bodies, names=None, taken=None):
        self.tree = tree; self.all = all_bodies
        self.names = {} if names is None else names          # a variable's id -> its own name
        self.taken = set() if taken is None else taken

    # -- names
    def name_of(self, node_text, declared=None):
        m = VARID.search(node_text)
        if not m: raise Unhandled('a variable without an id')
        vid = m.group(1)
        if vid not in self.names:
            base = declared or f'v{vid}'
            name, n = base, 1
            while name in self.taken:
                n += 1; name = f'{base}_r{n}'
            self.taken.add(name); self.names[vid] = name
        return self.names[vid]

    # -- helpers on nodes
    def expr_in(self, field):
        if field is None: raise Unhandled('a missing field')
        e = field.find('Expr')
        if e is None: raise Unhandled(f'no expression in `{field.text[:30]}`')
        return self.expr(e)

    def peel(self, node):
        kind, holder = thir.kind_of(node)
        while kind in WRAPPERS:
            inner = holder.find('Expr')
            if inner is None: raise Unhandled(kind)
            node = inner; kind, holder = thir.kind_of(node)
        return node, kind, holder

    def user_code(self, node):
        # code a standard-library macro expanded to (`format!`, `vec!`) points into the library's own source; a `for`
        # or a `?` that rustc unfolded, and a macro of the crate itself, point at the crate's files
        span = node.value('span', '')
        if span.startswith('/rustc/') or '/library/' in span.split(':')[0]: raise Unhandled('a macro of the standard library')

    # -- expressions
    def expr(self, node):
        node, kind, h = self.peel(node)
        self.user_code(node)
        ty = node.value('ty', '')
        if kind == 'VarRef': return ('path', [self.name_of(h.value('id', ''))])
        if kind == 'UpvarRef': return ('path', [self.name_of(h.value('var_hir_id', ''))])
        if kind == 'Literal': return self.literal(h.text)
        if kind == 'Binary':
            return ('bin', BIN[h.value('op')], self.expr_in(h.field('lhs')), self.expr_in(h.field('rhs')))
        if kind == 'LogicalOp':
            return ('bin', '&&' if h.value('op') == 'And' else '||', self.expr_in(h.field('lhs')), self.expr_in(h.field('rhs')))
        if kind == 'Unary':
            if h.value('op') != 'Not': raise Unhandled('negation')
            return ('not', self.expr_in(h.field('arg')))
        if kind == 'Deref': return self.expr_in(h)
        if kind == 'Borrow':
            inner = self.expr_in(h.field('arg'))
            return ('mutref', inner) if h.value('borrow_kind', '').startswith('Mut') else inner
        if kind == 'Tuple': return ('tuple', [self.expr(x) for x in h.field('fields').all('Expr')])
        if kind == 'Assign': return ('assign', self.expr_in(h.field('lhs')), self.expr_in(h.field('rhs')))
        if kind == 'AssignOp':
            lhs = self.expr_in(h.field('lhs'))
            op = h.value('op').replace('Assign', '')
            return ('assign', lhs, ('bin', BIN[op], lhs, self.expr_in(h.field('rhs'))))
        if kind == 'Index': return ('index', self.expr_in(h.field('lhs')), self.expr_in(h.field('index')))
        if kind == 'Field':
            lhs_node = h.field('lhs').find('Expr')
            lhs_ty = self.peel(lhs_node)[0].value('ty', '')
            if not lhs_ty.lstrip("&'{erased} mut").startswith('('): raise Unhandled('a struct field (its name is not in the tree)')
            return ('field', self.expr(lhs_node), int(h.value('name')))
        if kind == 'If': return self.if_(h)
        if kind == 'Match': return self.match(h)
        if kind == 'Loop': return ('loop', self.expr_in(h.field('body')))
        if kind == 'Break':
            value = h.field('value')
            return ('break', None, self.expr_in(value) if value is not None and value.find('Expr') is not None else None)
        if kind == 'Continue': return ('continue', None)
        if kind == 'Return':
            value = h.field('value')
            return ('return', self.expr_in(value) if value is not None and value.find('Expr') is not None else None)
        if kind == 'Block': return self.block(h)
        if kind == 'Adt': return self.adt(h, ty)
        if kind == 'Call': return self.call(h, ty)
        if kind == 'Closure': return self.closure(h)
        raise Unhandled(kind)

    def literal(self, text):
        m = re.search(r'node: (\w+)\((.*?)\), span:', text) or re.search(r'node: (\w+)\((.*)\) *\}', text)
        if not m: raise Unhandled('a literal')
        if 'neg: true' in text: raise Unhandled('a negative literal')
        what, inner = m.group(1), m.group(2)
        if what == 'Int': return ('int', int(re.search(r'Pu128\((\d+)\)', inner).group(1)))
        if what == 'Bool': return ('bool', inner.strip())
        if what == 'Str':
            s = re.match(r'"((?:[^"\\]|\\.)*)"', inner)
            if not s: raise Unhandled('a string literal')
            return ('str', bytes(s.group(1), 'utf-8').decode('unicode_escape').encode('latin-1', 'ignore').decode('utf-8', 'replace') if '\\' in s.group(1) else s.group(1))
        raise Unhandled(f'a {what} literal')

    def block(self, h):
        stmts = []
        field = h.field('stmts')
        for st in (field.all('Stmt') if field is not None else []):
            k = st.field('kind')
            if k.text.startswith('kind: Let'):
                init = k.field('initializer')
                els = k.field('else_block')
                if els is not None and els.find('Block') is not None: raise Unhandled('let-else')
                value = self.expr_in(init) if init is not None and init.find('Expr') is not None else None
                pat_node = k.field('pattern').find('Pat')
                ty = type_ast(pat_node.value('ty', '')) if value is not None and value[0] != 'closure' else None
                stmts.append(('let', self.pat(pat_node), ty, value, None))
            else:
                stmts.append(('expr', self.expr_in(k.field('expr'))))
        tail = h.field('expr')
        last = self.expr_in(tail) if tail is not None and tail.find('Expr') is not None else None
        if last is not None and (last[0] in ('for', 'loop', 'while', 'assign') or (last[0] == 'if' and last[3] is None)
                                 or (last[0] == 'iflet' and last[4] is None)):
            stmts.append(('expr', last)); last = None
        return ('block', stmts, last)

    def as_block(self, e):
        return e if e[0] == 'block' else ('block', [], e)

    def if_(self, h):
        cond_node, ckind, ch = self.peel(h.field('cond').find('Expr'))
        then = self.as_block(self.expr_in(h.field('then')))
        els = h.field('else')
        other = self.as_block(self.expr_in(els)) if els is not None and els.find('Expr') is not None else None
        def close(block):
            if block is not None and not block[1] and block[2] is not None and block[2][0] in ('if', 'iflet'):
                inner = block[2]; last = inner[3] if inner[0] == 'if' else inner[4]
                fixed = close(last) if last is not None else ('block', [], None)
                return ('block', [], inner[:3] + (fixed,) if inner[0] == 'if' else inner[:4] + (fixed,))
            return block
        other = close(other)
        if ckind == 'Let':
            return ('iflet', self.pat(ch.field('pat').find('Pat')), self.expr_in(ch.field('expr')), then, other)
        return ('if', self.expr(cond_node), then, other)

    def callee(self, h):
        fun = self.peel(h.field('fun').find('Expr'))[0]
        m = FNDEF.search(fun.value('ty', ''))
        if not m: raise Unhandled('a call of a value')
        return m.group(1), m.group(2)

    def match(self, h):
        scrut_node, skind, sh = self.peel(h.field('scrutinee').find('Expr'))
        arms = h.field('arms').all('Arm')
        if skind == 'Call':
            krate, path = self.callee(sh)
            # `for pat in iter { body }`: match into_iter(iter) { mut it => loop { match next(&mut it) { None => break, Some(pat) => body } } }
            if path.endswith('IntoIterator::into_iter') and len(arms) == 1:
                loop = self.peel(arms[0].field('body').find('Expr'))
                if loop[1] == 'Loop':
                    inner = self.peel(loop[2].field('body').find('Expr'))
                    stmt = inner[2].field('stmts').all('Stmt') if inner[1] == 'Block' and inner[2].field('stmts') is not None else []
                    if len(stmt) == 1:
                        m2 = self.peel(stmt[0].field('kind').field('expr').find('Expr'))
                        if m2[1] == 'Match' and len(m2[2].field('arms').all('Arm')) == 2:
                            some = m2[2].field('arms').all('Arm')[1]
                            sub = some.field('pattern').find('Pat')
                            pats = thir.kind_of(sub)[1].field('subpatterns')
                            pat = self.pat(pats.find('Pat'))
                            item = self.expr(sh.field('args').all('Expr')[0])
                            return ('for', pat, item, self.as_block(self.expr_in(some.field('body'))))
            # `e?`: match branch(e) { Continue(v) => v, Break(r) => return from_residual(r) }
            if path.endswith('Try::branch') and len(arms) == 2:
                return ('try', self.expr(sh.field('args').all('Expr')[0]))
        out = []
        for arm in arms:
            guard = arm.field('guard')
            g = self.expr_in(guard) if guard is not None and guard.find('Expr') is not None else None
            out.append((self.pat(arm.field('pattern').find('Pat')), g, self.expr_in(arm.field('body'))))
        return ('match', self.expr(scrut_node), out)

    def variants(self, h):
        """[(variant name, [field names])] of the type an Adt node or a Variant pattern is of, and the type's name."""
        line = None
        for n in h.kids:
            for k in ([n] + n.kids + [x for y in n.kids for x in y.kids]):
                if k.text.startswith('variants:'): line = k.text
        did = None
        for n in h.kids:
            for k in ([n] + n.kids + [x for y in n.kids for x in y.kids]):
                if k.text.startswith('did:'): did = k.text
        if line is None or did is None: raise Unhandled('a type definition')
        out = []
        for part in line.split('VariantDef {')[1:]:
            name = re.search(r'name: "(\w+)"', part)
            fields = re.findall(r'FieldDef \{[^}]*?name: "(\w+)"', part)
            out.append((name.group(1) if name else '?', fields))
        return re.search(r'~ \w+\[\w+\]::([^)]*)\)', did).group(1).split('::')[-1], out

    def adt(self, h, ty):
        type_name, variants = self.variants(h)
        index = int(deep_value(h, 'variant_index'))
        if deep_value(h, 'base', 'None') not in ('None',): raise Unhandled('a struct with a base')
        owner = owner_of(h, 'variant_index')
        fields = [(re.match(r'field (\d+):', k.text), k) for k in owner.kids if k.text.startswith('field ')]
        args = [self.expr_in(k) for _, k in sorted(fields, key=lambda mk: int(mk[0].group(1)))]
        vname, fnames = variants[index]
        if type_name in ('Option', 'Result'):
            return ('path', [vname]) if not args else ('call', ('path', [vname]), args)
        segs = [type_name] if len(variants) == 1 and vname == type_name else [type_name, vname]
        if not fnames: return ('path', segs)
        if all(f.isdigit() for f in fnames): return ('call', ('path', segs), args)
        return ('struct', segs, list(zip(fnames, args)))

    def call(self, h, ty):
        krate, path = self.callee(h)
        args = [self.expr(x) for x in h.field('args').all('Expr')]
        segs = [s for s in path.split('::') if not s.startswith('{')]
        name = segs[-1]
        if path.endswith('Deref::deref') or path.endswith('DerefMut::deref_mut'): return args[0]
        strip = lambda a: a[1] if a[0] == 'mutref' else a          # the borrow rustc takes of a receiver
        if len(segs) >= 2 and (segs[-2], name) in TRAIT_OPS and len(args) == 2:
            return ('bin', TRAIT_OPS[(segs[-2], name)], strip(args[0]), args[1])
        if len(segs) >= 2 and (segs[-2], name) in TRAIT_ASSIGN and len(args) == 2:
            lhs = strip(args[0]); return ('assign', lhs, ('bin', TRAIT_ASSIGN[(segs[-2], name)], lhs, args[1]))
        if path.endswith('Not::not') and len(args) == 1: return ('not', args[0])
        if (path.endswith('Index::index') or path.endswith('IndexMut::index_mut')) and len(args) == 2:
            return ('index', strip(args[0]), args[1])
        if re.search(r'ops::function::(Fn|FnMut|FnOnce)::(call|call_mut|call_once)$', path):
            f = strip(args[0])
            if f[0] != 'path' or args[1][0] != 'tuple': raise Unhandled('a call of a function value that is not a variable')
            return ('call', f, args[1][1])
        if krate not in ('core', 'alloc', 'std'):
            if '{impl#' in path:
                if not args: raise Unhandled('an associated function of the crate')
                return ('mcall', strip(args[0]), name, args[1:])
            return ('call', ('path', [name]), args)
        if '{impl#' in path or len(segs) < 2 or not segs[-2][:1].isupper():
            # an inherent function of a standard type: on the type when it has no receiver
            if name in ASSOCIATED or not args: return ('call', ('path', [head_type(ty), name]), args)
            return ('mcall', strip(args[0]), name, args[1:])
        # a trait method: on its first argument
        if name in ASSOCIATED or not args: return ('call', ('path', [head_type(ty), name]), args)
        return ('mcall', strip(args[0]), name, args[1:])

    def closure(self, h):
        m = re.search(r'DefId\(\d+:\d+ ~ \w+\[\w+\]::([^)]*)\)', deep_value(h, 'closure_id', ''))
        if not m or m.group(1) not in self.all: raise Unhandled('a closure whose body is not in the dump')
        sub = Body(self.all[m.group(1)], self.all, self.names, self.taken)
        params = sub.params()[1:]                               # the first parameter is the closure itself
        return ('closure', [(p, None) for p in params], sub.body(), None)

    # -- patterns
    def pat(self, node):
        if node is None: raise Unhandled('a missing pattern')
        kind, h = thir.kind_of(node)
        ty = node.value('ty', '')
        if kind == 'Wild': return WILD
        if kind == 'Binding':
            name = self.name_of(h.value('var', ''), h.value('name', '').strip('"'))
            sub = h.field('subpattern')
            if sub is not None and sub.find('Pat') is not None: return ('pat_at', name, self.pat(sub.find('Pat')))
            return ('pid', name)
        if kind in ('Deref', 'DerefPattern'): return self.pat(h.find('Pat'))
        if kind == 'Or': return ('por', [self.pat(p) for p in h.field('pats').all('Pat')])
        if kind in ('Variant', 'Leaf'):
            subs = h.field('subpatterns')
            pats = [self.pat(p) for p in (subs.all('Pat') if subs is not None else [])]
            if kind == 'Leaf' and ty.lstrip("&'{erased} mut").startswith('('): return ('ptuple', pats)
            if kind == 'Leaf': raise Unhandled('a struct pattern')
            type_name, variants = self.variants(h)
            vname, fnames = variants[int(deep_value(h, 'variant_index'))]
            if len(pats) != len(fnames): raise Unhandled('a variant pattern with fields left out')
            keys = [int(f) if f.isdigit() else f for f in fnames]
            segs = [vname] if type_name in ('Option', 'Result') else [type_name, vname]
            return ('pctor', segs, list(zip(keys, pats)))
        raise Unhandled(f'a {kind} pattern')

    # -- the body
    def params(self):
        out = []
        field = self.tree.field('params')
        for p in (field.all('Param') if field is not None else []):
            pat = p.field('param')
            out.append(self.pat(pat.find('Pat')) if pat is not None and pat.find('Pat') is not None else WILD)
        return out

    def body(self):
        return self.as_block(self.expr_in(self.tree.field('body')))


def convert(dump_path, want=None):
    """{path: (parameter patterns, body) or the reason it was not converted} for the function bodies of a dump."""
    trees = {path: tree for _, path, tree in thir.bodies(open(dump_path, encoding='utf-8', errors='replace'), want)}
    out = {}
    for path, tree in trees.items():
        if '{closure#' in path or '{constant#' in path: continue
        b = Body(tree, trees)
        try:
            params = b.params()
            out[path] = (params, b.body())
        except Unhandled as x:
            out[path] = str(x)
        except (AttributeError, KeyError, IndexError, ValueError, TypeError) as x:
            out[path] = f'converter error: {type(x).__name__}: {x}'
    return out


if __name__ == '__main__':
    import sys, collections
    got = convert(sys.argv[1])
    ok = [p for p, v in got.items() if not isinstance(v, str)]
    print(len(ok), 'of', len(got), 'bodies converted')
    why = collections.Counter(v for v in got.values() if isinstance(v, str))
    for w, n in why.most_common(12): print(f'  {n:4} {w[:90]}')
    if len(sys.argv) > 2: print(got[sys.argv[2]])

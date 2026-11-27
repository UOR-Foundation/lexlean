"""A body of rustc's typed tree as the translator's syntax.

What rustc has already decided is taken from it: which variable a name means (each binding has an id, so each gets a
name of its own), which function a call reaches, where a borrow or a dereference is, how a `for` or a `?` unfolds.
The `for` loop and `?` are folded back to the forms the translator has rules for. A construct this file has no rule
for raises `Unhandled`, and the caller keeps the body the source parser made: nothing is guessed."""
import os
import re
import thir
import rs2lex_core
import unshadow

WILD = ('pwild',)
BIN = {'Add': '+', 'Sub': '-', 'Mul': '*', 'Div': '/', 'Rem': '%', 'Eq': '==', 'Ne': '!=', 'Lt': '<', 'Le': '<=', 'Gt': '>',
       'Ge': '>=', 'BitAnd': '&', 'BitOr': '|', 'BitXor': '^', 'Shl': '<<', 'Shr': '>>'}
# associated functions of the standard library, called on the type and not on a first argument
RANGES = ('Range', 'RangeFrom', 'RangeTo', 'RangeFull')
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


STD_TYPES = {'Vec', 'String', 'Option', 'Result', 'Box', 'BTreeMap', 'BTreeSet', 'HashMap', 'HashSet', 'Rc', 'Arc', 'Cow',
             'VecDeque', 'PathBuf', 'Path', 'Ordering'}
KNOWN = None            # the crate's names, for the run of `convert` at hand


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
    if '::' in head:
        if head.split('::')[0] in ('std', 'core', 'alloc'):
            if name not in STD_TYPES: return None            # an iterator, a guard: a type no annotation should name
        else:
            name = KNOWN.type_name(head) if KNOWN is not None else None
            if name is None: return None
    return ('path', name, args)


MUT_REF = re.compile(r"&(?:'\{erased\} |'\w+ )?mut ")


class Names:
    """the crate's items as the loader names them, found by the path rustc prints (`krate::module::Item`)."""
    def __init__(self, items, resolver=None):
        self.at = {}; self.bare = {}; self.resolver = resolver; self.held = None
        self.modules = {tuple(it['mp']) for it in items}
        for it in items:
            if it['k'] not in ('struct', 'enum', 'const') and not (it['k'] == 'fn' and not it['owner']): continue
            name = it.get('orig_name', it['name'])
            self.at[(it['k'] in ('struct', 'enum'), tuple(it['mp']), name)] = it
            self.bare.setdefault((it['k'] in ('struct', 'enum'), name), []).append(it)

    def item(self, path, is_type=True):
        path = re.sub(r"&'\{erased\} (mut )?|&(mut )?", '', path).strip().split('<')[0]
        segs = [x for x in path.split('::') if not x.startswith('{')]
        if not segs: return None
        # a type is printed from the crate's root (`ir::term::Binder`), a definition under the crate's name
        hit = self.at.get((is_type, tuple(segs[:-1]), segs[-1])) or self.at.get((is_type, tuple(segs[1:-1]), segs[-1]))
        if hit is None:
            same = self.bare.get((is_type, segs[-1]), [])
            if len(same) == 1 and len(segs) <= 2: hit = same[0]
        return hit

    def holders(self):
        """the crate's types that hold a mutable reference, in a field or in a field's type (`Sink<'a>` over
        `&'a mut String`): a value of one is another name for a place, as the reference itself is."""
        if self.held is None:
            def has(t, held):
                if isinstance(t, list): return any(has(x, held) for x in t)
                if not isinstance(t, tuple) or not t: return False
                if t[0] == 'mut': return True
                if t[0] == 'path' and isinstance(t[1], str) and t[1] in held: return True
                return any(has(x, held) for x in t[1:] if isinstance(x, (tuple, list)))
            types = [it for (is_type, _, _), it in self.at.items() if is_type]
            held, grew = set(), True
            while grew:
                grew = False
                for it in types:
                    if it['name'] in held: continue
                    fields = it['fields'] if it['k'] == 'struct' else [f for _, fs in it['variants'] for f in fs]
                    if any(has(t, held) for _, t in fields): held.add(it['name']); grew = True
            self.held = held
        return self.held

    def holds_mut(self, text):
        """whether a type, as rustc prints it, is a mutable reference or holds one."""
        if MUT_REF.search(text): return True
        for token in re.findall(r'[A-Za-z_][\w:]*', text):
            if token.split('::')[0] in ('std', 'core', 'alloc'):
                if re.search(r'Mut|Entry|Drain|Guard', token.split('::')[-1]): return True     # IterMut, Entry, RefMut ..
            elif self.type_name(token) in self.holders(): return True
        return False

    def home(self, path):
        """the module a body at `path` is written in: the longest module the path begins with."""
        segs = [x for x in path.split('::') if not x.startswith('{')]
        for n in range(len(segs) - 1, -1, -1):
            if tuple(segs[:n]) in self.modules: return tuple(segs[:n])
        return None

    def resolve(self, mp, ast, pattern):
        """a parsed piece with its paths resolved from module `mp`, by the loader's own pass."""
        body = ('match', ('tuple', []), [(ast, None, ('tuple', []))]) if pattern else ast
        fake = {'k': 'fn', 'name': '_', 'uname': '_', 'params': [], 'ret': None, 'body': body, 'owner': None, 'mp': mp}
        out = self.resolver.rewrite(fake)['body']
        return out[2][0][0] if pattern else out

    def type_name(self, path):
        it = self.item(path)
        return it['name'] if it is not None else None

    def field(self, ty, index):
        it = self.item(ty)
        if it is None or it['k'] != 'struct' or index >= len(it['fields']): return None
        return it['fields'][index][0]

    def variant(self, path, index):
        """(the enum's name, the variant's name, its field names) of the crate's enum at `path`."""
        it = self.item(path)
        if it is None or it['k'] != 'enum' or index >= len(it['variants']): return None
        v, fs = it['variants'][index]
        return it['name'], v, [str(k) for k, _ in fs]


def split_debug(text):
    """the top-level, comma-separated parts of a Debug-printed body."""
    out, depth, cur, quote = [], 0, '', False
    for i, ch in enumerate(text):
        if ch == '"' and (i == 0 or text[i - 1] != '\\'): quote = not quote
        if not quote:
            if ch in '([{<': depth += 1
            elif ch in ')]}>' and not (ch == '>' and i and text[i - 1] in '-='): depth -= 1
            if ch == ',' and depth == 0: out.append(cur.strip()); cur = ''; continue
        cur += ch
    if cur.strip(): out.append(cur.strip())
    return out


def debug_struct(text):
    """(name, {field: text}) of `Name { field: value, ... }` as derived Debug prints it; (text, {}) for a bare name."""
    m = re.match(r'^(\w+) \{(.*)\}$', text.strip(), re.S)
    if not m: return text.strip(), {}
    fields = {}
    for part in split_debug(m.group(2)):
        if ': ' in part:
            k, v = part.split(': ', 1); fields[k.strip()] = v.strip()
    return m.group(1), fields


SPAN = re.compile(r'^(.*?):(\d+):(\d+): (\d+):(\d+) \(#(\d+)\)$')
STD_MACROS = {'format', 'vec', 'write', 'writeln', 'assert', 'assert_eq', 'assert_ne', 'debug_assert', 'debug_assert_eq',
              'debug_assert_ne', 'panic', 'unreachable', 'matches', 'print', 'println', 'eprint', 'eprintln'}


KEYWORDS = {'type', 'match', 'mod', 'ref', 'in', 'fn', 'let', 'loop', 'move', 'use', 'where', 'impl', 'enum', 'struct', 'trait',
            'const', 'static', 'as', 'box', 'do', 'final', 'macro', 'override', 'priv', 'typeof', 'unsized', 'virtual', 'yield',
            'abstract', 'become', 'try', 'async', 'await', 'dyn', 'pub', 'if', 'else', 'for', 'while', 'return', 'break',
            'continue', 'true', 'false', 'unsafe', 'extern', 'mut'}


def ident(name):
    """a name as the lexer spells it: a raw identifier, which rustc prints bare, is `r_name`."""
    return 'r_' + name if name in KEYWORDS else name


def in_library(span): return span.startswith('/rustc/') or '/library/' in span.split(':')[0]


class Sources:
    """the crate's files as rustc names them in a span, and the macro calls written in each."""
    def __init__(self, root): self.root = root; self.files = {}

    def file(self, path):
        if path not in self.files:
            text = open(os.path.join(self.root, path), encoding='utf-8').read().replace('\r\n', '\n')
            lines = [0]
            for i, ch in enumerate(text):
                if ch == '\n': lines.append(i + 1)
            toks = [(m.lastgroup, m.group(), m.start(), m.end()) for m in rs2lex_core.TOKEN.finditer(text) if m.lastgroup != 'ws']
            calls = []
            for i in range(len(toks) - 2):
                if toks[i][0] == 'id' and toks[i + 1][1] == '!' and toks[i + 2][1] in ('(', '[', '{'):
                    depth = 0
                    for j in range(i + 2, len(toks)):
                        if toks[j][0] not in ('op', 'id'): continue
                        if toks[j][1] in ('(', '[', '{'): depth += 1
                        elif toks[j][1] in (')', ']', '}'):
                            depth -= 1
                            if depth == 0: calls.append((toks[i][2], toks[j][3], toks[i][1])); break
            self.files[path] = (text, lines, calls)
        return self.files[path]

    def place(self, span):
        """(file, start, end) as offsets into the file's text, of a span as rustc prints it."""
        m = SPAN.match(span)
        if not m or in_library(span): return None
        try: _, lines, _ = self.file(m.group(1))
        except OSError: return None
        at = lambda line, col: lines[int(line) - 1] + int(col) - 1
        return m.group(1), at(m.group(2), m.group(3)), at(m.group(4), m.group(5))

    def call(self, path, lo, hi, within):
        """the outermost macro call inside `within` that holds lo..hi: (its text, its name)."""
        text, _, calls = self.file(path)
        for a, b, name in calls:                       # in the order written: an outer call comes before the calls in it
            if within[0] <= a <= lo and hi <= b <= within[1]: return text[a:b], name
        return None


CLOSURE = re.compile(r'DefId\(\d+:\d+ ~ \w+\[\w+\]::([^)]*)\)')
BINDS = re.compile(r'name: "(\w+)",?\s+mode: .{0,60}?,?\s+var: (LocalVarId\(HirId\(DefId\([^)]*\)\.\d+\)\))')
BOUND_AS = re.compile(r'name: "\w+",?\s+mode: .{0,60}?,?\s+var: LocalVarId\(HirId\(DefId\([^)]*\)\.(\d+)\)\),?\s+ty: (.*?),?\s+(?:is_primary|subpattern):')
USES = re.compile(r'(?<![\w])(?:id|var_hir_id): LocalVarId\(HirId\(DefId\([^)]*\)\.(\d+)\)\)')


def rename(e, scope):
    """a parsed piece with its variables, bound or used, on the names `scope` gives them."""
    if isinstance(e, list): return [rename(x, scope) for x in e]
    if not isinstance(e, tuple) or not e: return e
    if e[0] == 'path' and isinstance(e[1], list):
        return ('path', [scope.get(e[1][0], e[1][0])]) if len(e[1]) == 1 else e
    if e[0] in unshadow.BINDERS and isinstance(e[1], str): return (e[0], scope.get(e[1], e[1])) + tuple(rename(x, scope) for x in e[2:])
    if e[0] == 'macro' and len(e) > 2 and isinstance(e[2], list):
        capture = lambda m: '{' + scope.get(m.group(1), m.group(1))
        args = [('str', re.sub(r'(?<!\{)\{([a-z_]\w*)(?=[:}])', capture, a[1])) if isinstance(a, tuple) and len(a) == 2 and a[0] == 'str'
                else rename(a, scope) for a in e[2]]
        return (e[0], e[1], args) + tuple(e[3:])
    return tuple(rename(x, scope) if isinstance(x, (tuple, list)) else x for x in e)


class Body:
    def __init__(self, tree, all_bodies, names=None, taken=None, crate=None, declared=None, src=None, home=None):
        self.home = home                                     # the module the body is written in
        self.src = src; self.around = []                     # the crate's own spans around the expression at hand
        self.tree = tree; self.all = all_bodies; self.crate = crate
        self.declared = {} if declared is None else declared     # a variable's id -> the name its source gives it
        self.names = {} if names is None else names          # a variable's id -> its own name
        self.taken = set() if taken is None else taken

    # -- names
    def name_of(self, node_text, declared=None):
        m = VARID.search(node_text)
        if not m: raise Unhandled('a variable without an id')
        vid = m.group(1)
        if declared: declared = ident(declared)
        if vid not in self.names:
            base = declared or f'v{vid}'
            name, n = base, 1
            while name in self.taken:
                n += 1; name = f'{base}_r{n}'
            self.taken.add(name); self.names[vid] = name; self.declared[vid] = declared or name
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

    def lines_of(self, node):
        out, todo = [], [node]
        while todo:
            n = todo.pop(); todo.extend(reversed(n.kids)); out.append(n.text)
        return out

    def lines(self, node):
        """every line of a tree, and of the bodies of the closures written in it."""
        out, todo = [], [node]
        while todo:
            n = todo.pop(); todo.extend(reversed(n.kids)); out.append(n.text)     # in the order printed
            if 'closure_id' in n.text:
                m = CLOSURE.search(n.text)
                if not m or m.group(1) not in self.all: raise Unhandled('a closure whose body is not in the dump')
                todo.append(self.all[m.group(1)])
        return out

    def reread(self, lines, text, pattern=False):
        """a piece of the source, parsed again and put on this body's names. rustc resolved every variable in it to an
        id, and `lines`, the piece's tree, holds them: the names it binds and the ones it uses. Where each name in the
        piece stands for one variable, renaming by name is renaming by id; where one name stands for two, no."""
        whole = ' '.join(x.strip() for x in lines)
        scope = {}
        def put(vid):
            # a name the text does not write is the expansion's own (`args`, in what `format!` unfolds to)
            written = self.declared[vid]
            if written.startswith('r_') and written[2:] in KEYWORDS: written = 'r#' + written[2:]
            if not re.search(r'(?<![A-Za-z0-9_#])' + re.escape(written) + r'(?![A-Za-z0-9_])', text): return
            if scope.setdefault(self.declared[vid], self.names[vid]) != self.names[vid]:
                raise Unhandled('source read again with two variables of one name in it')
        for m in BINDS.finditer(whole):
            self.name_of(m.group(2), m.group(1)); put(VARID.search(m.group(2)).group(1))
        for m in USES.finditer(whole):
            if m.group(1) not in self.names: raise Unhandled(f'source read again with a variable not seen before it')
            put(m.group(1))
        try:
            parser = rs2lex_core.Parser(rs2lex_core.lex(text))
            ast = parser.pattern() if pattern else parser.expr()
        except rs2lex_core.Unsupported as x: raise Unhandled(f'source read again: {x}')
        except (IndexError, KeyError, TypeError, ValueError): raise Unhandled('source read again and not parsed')
        if parser.peek()[0] != 'eof': raise Unhandled('source read again and not read to its end')
        if self.crate is not None and self.crate.resolver is not None:
            if self.home is None: raise Unhandled('source read again in a module the loader does not have')
            ast = self.crate.resolve(self.home, ast, pattern)
        return rename(ast, scope)

    def macro(self, node):
        """the call of a standard-library macro, read again from the source. What rustc expanded it to (`format!`, `vec!`)
        points into the library's own files and is the library's business; what the crate wrote inside it keeps its own
        spans, and the outermost macro call around those, inside the nearest expression of the crate, is the call."""
        no = 'a macro of the standard library'
        if self.src is None or not self.around: raise Unhandled(no)
        spans, todo = [], [node]
        while todo:
            n = todo.pop(); todo.extend(n.kids)
            if n.text.startswith(('Expr', 'Pat')):
                at = self.src.place(n.value('span', ''))
                if at is not None: spans.append(at)
        path, lo, hi = self.around[-1]
        if not spans or any(f != path or a < lo or b > hi for f, a, b in spans): raise Unhandled(no + ' with nothing of the crate in it')
        found = self.src.call(path, min(a for _, a, _ in spans), max(b for _, _, b in spans), (lo, hi))
        if found is None: raise Unhandled(no + ' whose call was not found')
        text, name = found
        if name not in STD_MACROS: raise Unhandled(f'the macro `{name}!`')
        return self.reread(self.lines(node), text)

    # -- expressions
    def expr(self, node):
        node, kind, h = self.peel(node)
        raw = node.value('span', '')
        if in_library(raw): return self.macro(node)
        at = self.src.place(raw) if self.src is not None and raw.endswith('(#0)') else None
        if at is None: return self.expr1(node, kind, h)
        self.around.append(at)
        try: return self.expr1(node, kind, h)
        except Unhandled as first:
            # what the tree does not give, its own text does, where the text is the crate's and not a macro's
            try: return self.reread(self.lines(node), self.src.file(at[0])[0][at[1]:at[2]])
            except Unhandled: raise first
        finally: self.around.pop()

    def expr1(self, node, kind, h):
        ty = node.value('ty', '')
        if kind == 'VarRef': return ('path', [self.name_of(h.value('id', ''))])
        if kind == 'UpvarRef': return ('path', [self.name_of(h.value('var_hir_id', ''))])
        if kind == 'Literal':
            lit = self.literal(h.text)
            # rustc has typed every literal: one of type i128 is said to be, written with the suffix or not
            if lit[0] == 'int' and ty.strip() == 'i128': return ('cast', lit, ('path', 'i128', []))
            return lit
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
        if kind == 'Pointer': return self.expr_in(h.field('source'))
        if kind == 'Array': return ('array', [self.expr(x) for x in h.field('fields').all('Expr')])
        if kind == 'Assign': return ('assign', self.expr_in(h.field('lhs')), self.expr_in(h.field('rhs')))
        if kind == 'AssignOp':
            lhs = self.expr_in(h.field('lhs'))
            op = h.value('op').replace('Assign', '')
            return ('assign', lhs, ('bin', BIN[op], lhs, self.expr_in(h.field('rhs'))))
        if kind == 'Index': return ('index', self.expr_in(h.field('lhs')), self.expr_in(h.field('index')))
        if kind == 'Field':
            lhs_node = h.field('lhs').find('Expr')
            lhs_ty = self.peel(lhs_node)[0].value('ty', '')
            index = int(h.value('name'))
            if re.sub(r"&'\{erased\} (mut )?", '', lhs_ty).strip().startswith('('): return ('field', self.expr(lhs_node), index)
            fname = self.crate.field(lhs_ty, index) if self.crate is not None else None
            if fname is None: raise Unhandled('a field of a type the loader does not have')
            return ('field', self.expr(lhs_node), int(fname[1:]) if re.fullmatch(r'f\d+', fname) else fname)
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
        if what == 'Byte': return ('byte', int(inner))
        if what == 'Str':
            s = re.match(r'"((?:[^"\\]|\\.)*)"', inner)
            if not s: raise Unhandled('a string literal')
            return ('str', bytes(s.group(1), 'utf-8').decode('unicode_escape').encode('latin-1', 'ignore').decode('utf-8', 'replace') if '\\' in s.group(1) else s.group(1))
        if what == 'Char':
            c = re.match(r"'((?:[^'\\\\]|\\\\.)+)'", inner)
            if not c or '\\\\' in c.group(1): raise Unhandled('an escaped character literal')
            return ('chr', c.group(1))
        raise Unhandled(f'a {what} literal')

    def block(self, h):
        stmts = []
        field = h.field('stmts')
        for st in (field.all('Stmt') if field is not None else []):
            k = st.field('kind')
            if k.text.startswith('kind: Let'):
                init = k.field('initializer')
                els = k.field('else_block')
                value = self.expr_in(init) if init is not None and init.find('Expr') is not None else None
                pat_node = k.field('pattern').find('Pat')
                ty = type_ast(pat_node.value('ty', '')) if value is not None and value[0] != 'closure' else None
                other = self.block(els.find('Block')) if els is not None and els.find('Block') is not None else None
                stmts.append(('let', self.pat(pat_node), ty, value, other))
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
        pat = None
        if ckind == 'Let':                                     # first: the branch taken uses the pattern's names
            pf = ch.field('pat')
            pat = self.pat(pf.find('Pat')) if pf.kids else self.inline_pat(pf.text[4:].strip())
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
            return ('iflet', pat, self.expr_in(ch.field('expr')), then, other)
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
            pat = self.pat(arm.field('pattern').find('Pat'))       # first: the guard and the body use its names
            guard = arm.field('guard')
            g = self.expr_in(guard) if guard is not None and guard.find('Expr') is not None else None
            out.append((pat, g, self.expr_in(arm.field('body'))))
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
            out.append((name.group(1) if name else '?', [ident(f) for f in fields]))
        full = re.search(r'~ (\w+)\[\w+\]::([^)]*)\)', did)
        if full.group(1) in ('core', 'alloc', 'std'): return full.group(2).split('::')[-1], out
        name = self.crate.type_name(full.group(2)) if self.crate is not None else None
        if name is None: raise Unhandled('a type the loader does not have')
        return name, out

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
        if type_name in RANGES and len(args) == len(fnames):
            part = dict(zip(fnames, args))
            return ('range', part.get('start'), part.get('end'), False)
        # a struct is its one variant; the type's name is the loader's by now, so the flags say which it is
        segs = [type_name] if len(variants) == 1 and 'IS_ENUM' not in deep_value(h, 'flags', '') else [type_name, vname]
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
        written = self.written_callee(h)
        if written is not None: return ('call', written, args)
        if krate not in ('core', 'alloc', 'std'):
            if '{impl#' in path:
                if not args: raise Unhandled('an associated function of the crate')
                return ('mcall', strip(args[0]), name, args[1:])
            found = self.crate.item(path, is_type=False) if self.crate is not None else None
            if found is None or found['k'] != 'fn': raise Unhandled('a function the loader does not have')
            return ('call', ('path', [found['name']]), args)
        if path.endswith('RangeInclusive::new') or (segs[-1] == 'new' and head_type(ty) == 'RangeInclusive' and len(args) == 2):
            return ('range', args[0], args[1], True)
        if (name in ASSOCIATED or not args) and name not in ('new', 'with_capacity', 'default'):
            raise Unhandled('a function of a library type, named on the type')      # the source says which type
        if '{impl#' in path or len(segs) < 2 or not segs[-2][:1].isupper():
            # an inherent function of a standard type: on the type when it has no receiver
            if name in ASSOCIATED or not args: return ('call', ('path', [head_type(ty), name]), args)
            return ('mcall', strip(args[0]), name, args[1:])
        # a trait method: on its first argument
        if name in ASSOCIATED or not args: return ('call', ('path', [head_type(ty), name]), args)
        return ('mcall', strip(args[0]), name, args[1:])

    def written_callee(self, h):
        """the function of a call as the source names it, where the call is written `path(args)` and not
        `receiver.method(args)`: rustc's path for `Type::function` names the impl block, not the type, and the
        source's is the one the loader resolves."""
        fun = h.field('fun')
        fun = fun.find('Expr') if fun is not None else None
        if fun is None or self.src is None: return None
        raw = fun.value('span', '')
        at = self.src.place(raw) if raw.endswith('(#0)') else None
        if at is None: return None
        text = self.src.file(at[0])[0]
        if text[:at[1]].rstrip().endswith('.'): return None
        name = text[at[1]:at[2]]
        if not re.fullmatch(r'[\w:<>, &\[\]\']+', name): return None
        try: got = self.reread([], name + '()')
        except Unhandled: return None
        return got[1] if got[0] == 'call' and got[2] == [] else None

    def closure(self, h):
        m = re.search(r'DefId\(\d+:\d+ ~ \w+\[\w+\]::([^)]*)\)', deep_value(h, 'closure_id', ''))
        if not m or m.group(1) not in self.all: raise Unhandled('a closure whose body is not in the dump')
        sub = Body(self.all[m.group(1)], self.all, self.names, self.taken, self.crate, self.declared, self.src, self.home)
        sub.around = list(self.around)
        params = sub.params()[1:]                               # the first parameter is the closure itself
        return ('closure', [(p, None) for p in params], sub.body(), None)

    # -- patterns
    def pat(self, node):
        if node is None: raise Unhandled('a missing pattern')
        try: return self.pat1(node)
        except Unhandled as first:
            raw = node.value('span', '')
            at = self.src.place(raw) if self.src is not None and raw.endswith('(#0)') else None
            if at is None: raise
            try: return self.reread(self.lines(node), self.src.file(at[0])[0][at[1]:at[2]], pattern=True)
            except Unhandled: raise first

    def pat1(self, node):
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
            def shorthand(nodes):
                keys = []
                for n in nodes:
                    kk, hh = thir.kind_of(n)
                    if kk != 'Binding' or hh.value('is_shorthand') != 'true': raise Unhandled('a struct pattern not written in shorthand')
                    keys.append(hh.value('name', '').strip('"'))
                return keys
            nodes = subs.all('Pat') if subs is not None else []
            if kind == 'Leaf':
                tname = self.crate.type_name(ty) if self.crate is not None else None
                if tname is None: raise Unhandled('a struct pattern of a type the loader does not have')
                return ('pctor', [tname], list(zip(shorthand(nodes), pats)))
            type_name, variants = self.variants(h)
            vname, fnames = variants[int(deep_value(h, 'variant_index'))]
            if len(pats) != len(fnames): raise Unhandled('a variant pattern with fields left out')
            keys = [int(f) if f.isdigit() else f for f in fnames]
            if any(isinstance(x, str) for x in keys): keys = shorthand(nodes)
            segs = [vname] if type_name in ('Option', 'Result') else [type_name, vname]
            return ('pctor', segs, list(zip(keys, pats)))
        if kind in ('Slice', 'Array'):
            prefix = [self.pat(x) for x in (h.field('prefix').all('Pat') if h.field('prefix') is not None else [])]
            suffix = [self.pat(x) for x in (h.field('suffix').all('Pat') if h.field('suffix') is not None else [])]
            mid = h.field('slice')
            rest = self.pat(mid.find('Pat')) if mid is not None and mid.find('Pat') is not None else None
            return self.slice_pat(prefix, rest, suffix)
        raise Unhandled(f'a {kind} pattern')

    def slice_pat(self, prefix, rest, suffix):
        """a list is a cons list: `[a, b]` is cons a (cons b nil), `[a, rest @ ..]` is cons a rest; a rest before the
        end keeps the slice form, as the source parser writes it."""
        if rest is not None and rest[0] == 'pat_at' and rest[2] == WILD: rest = ('pid', rest[1])
        if rest is not None and rest[0] not in ('pid', 'pwild'): raise Unhandled('a slice pattern with a pattern for its rest')
        if not suffix:
            tail = ('pctor', ['__Nil'], []) if rest is None else rest
            for x in reversed(prefix): tail = ('pctor', ['__Cons'], [(0, x), (1, tail)])
            return tail
        mark = ('prest',) if rest is None or rest == WILD else ('pat_at', rest[1], ('prest',))
        return ('pslice', prefix + [mark] + suffix)

    def inline_pat(self, text):
        try: return self.inline_pat1(text)
        except Unhandled as first:
            raw = debug_struct(text)[1].get('span', '')
            at = self.src.place(raw) if self.src is not None and raw.endswith('(#0)') else None
            if at is None: raise
            try: return self.reread([text], self.src.file(at[0])[0][at[1]:at[2]], pattern=True)
            except Unhandled: raise first

    def inline_pat1(self, text):
        """a pattern as derived Debug prints it on one line (the pattern of an `if let`)."""
        name, f = debug_struct(text)
        if name != 'Pat' or 'kind' not in f: raise Unhandled('a pattern in a form not read')
        kind, k = debug_struct(f['kind'])
        ty = f.get('ty', '')
        if kind == 'Wild': return WILD
        if kind == 'Binding':
            var = self.name_of(k.get('var', ''), k.get('name', '').strip('"'))
            if k.get('subpattern', 'None') != 'None': return ('pat_at', var, self.inline_pat(k['subpattern'][5:-1]))
            return ('pid', var)
        if kind in ('Deref', 'DerefPattern'): return self.inline_pat(k['subpattern'])
        if kind == 'Or': return ('por', [self.inline_pat(x) for x in split_debug(k['pats'][1:-1])])
        if kind in ('Variant', 'Leaf'):
            subs = []
            for part in split_debug(k.get('subpatterns', '[]')[1:-1]):
                _, fp = debug_struct(part)
                subs.append((int(fp['field']), self.inline_pat(fp['pattern'])))
            subs.sort(key=lambda x: x[0])
            if kind == 'Leaf':
                if ty.startswith('(') and [i for i, _ in subs] == list(range(len(subs))): return ('ptuple', [q for _, q in subs])
                raise Unhandled('a struct pattern')
            path, index = k['adt_def'], int(k['variant_index'])
            base = path.split('::')[-1]
            if base in ('Option', 'Result'):
                vname = {('Option', 0): 'None', ('Option', 1): 'Some', ('Result', 0): 'Ok', ('Result', 1): 'Err'}[(base, index)]
                return ('pctor', [vname], subs)
            found = self.crate.variant(path, index) if self.crate is not None else None
            if found is None: raise Unhandled('a pattern of a type the loader does not have')
            enum, vname, fnames = found
            return ('pctor', [enum, vname], [(int(fnames[i]) if fnames[i].isdigit() else fnames[i], q) for i, q in subs])
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


def convert(dump_path, want=None, items=None, src_root=None, resolver=None):
    """{path: (parameter patterns, body) or the reason it was not converted} for the function bodies of a dump (plain
    or gzipped). The dump of a crate runs to gigabytes, so it is read as a stream: a function's closures follow it, and
    a function is converted, and its trees dropped, when the next one begins. With `items`, the loader's, only the
    functions the loader has are read."""
    out = {}
    global KNOWN
    names = Names(items, resolver) if items is not None else None
    KNOWN = names
    sources = Sources(src_root) if src_root is not None else None
    if want is None and items is not None:
        have = {it.get('orig_name', it['name']) for it in items if it['k'] == 'fn'} | {'fmt'}
        want = lambda path: parent(path).split('::')[-1] in have

    def parent(path): return re.split(r'::\{(?:closure|constant)#', path)[0]

    def finish(family):
        for path, tree in family.items():
            if path != parent(path): continue
            body = Body(tree, family, crate=names, src=sources, home=names.home(path) if names is not None else None)
            try:
                params = body.params()
                converted = body.body()
                # the names rustc types as a mutable reference, or as a value holding one: these, and only these,
                # are other names for a place; a write through one must reach the place
                whole = ' '.join(x.strip() for t in family.values() for x in body.lines_of(t))
                typed = {body.names[m.group(1)] for m in BOUND_AS.finditer(whole) if m.group(1) in body.names}
                muts = {body.names[m.group(1)] for m in BOUND_AS.finditer(whole)
                        if m.group(1) in body.names and (names.holds_mut(m.group(2)) if names is not None else MUT_REF.search(m.group(2)))}
                out[path] = (params, converted, (muts, typed - muts))
            except Unhandled as x:
                out[path] = str(x)
            except (AttributeError, KeyError, IndexError, ValueError, TypeError) as x:
                out[path] = f'converter error: {type(x).__name__}: {x}'

    if dump_path.endswith('.gz'):
        import gzip
        stream = gzip.open(dump_path, 'rt', encoding='utf-8', errors='replace')
    else: stream = open(dump_path, encoding='utf-8', errors='replace')
    family, head = {}, None
    for _, path, tree in thir.bodies(stream, want):
        if parent(path) != head:
            finish(family); family, head = {}, parent(path)
        family[path] = tree
    finish(family)
    return out


if __name__ == '__main__':
    import sys, collections
    got = convert(sys.argv[1])
    ok = [p for p, v in got.items() if not isinstance(v, str)]
    print(len(ok), 'of', len(got), 'bodies converted')
    why = collections.Counter(v for v in got.values() if isinstance(v, str))
    for w, n in why.most_common(12): print(f'  {n:4} {w[:90]}')
    if len(sys.argv) > 2: print(got[sys.argv[2]])

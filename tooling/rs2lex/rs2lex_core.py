"""rs2lex3: spike translator from a Rust subset to LexLean language-1.2 authoring notation (third round).

Rules added over round two, each general rather than per function:
  * recursion: every recursive call group takes `fuel : Nat` and is structural on it; outside callers pass u64::MAX,
  * `&mut` parameters: a function returns its result paired with the final value of each, and callers rebind them,
  * closures may return early (`?`) and may be passed by name; a mutating `map(..).collect()` becomes a loop,
  * integer literals take their type from inference (Nat or the shim's I128); `as` casts between them,
  * BTreeSet is a list without duplicates under derived equality; iterating one is refused (its order would matter),
  * constants are inlined; `uN::MAX` and `iN::MIN/MAX` are literals.
"""
import re, sys, json, itertools

class Unsupported(Exception): pass

TOKEN = re.compile(r'''
  (?P<ws>\s+|//[^\n]*|/\*(?:.|\n)*?\*/)
 |(?P<rstr>b?r(?P<hashes>\#*)"(?:.|\n)*?"(?P=hashes))
 |(?P<bstr>b"(?:[^"\\]|\\.)*")
 |(?P<str>"(?:[^"\\]|\\.)*")
 |(?P<byte>b'(?:[^'\\]|\\(?:x[0-9a-fA-F]{2}|.))')
 |(?P<chr>'(?:[^'\\]|\\(?:u\{[0-9a-fA-F]+\}|x[0-9a-fA-F]{2}|.))')
 |(?P<life>'[A-Za-z_]\w*)
 |(?P<int>(?:0x[0-9a-fA-F_]+|0b[01_]+|0o[0-7_]+|\d[\d_]*)(?:u8|u16|u32|u64|u128|usize|i8|i16|i32|i64|i128|isize)?)
 |(?P<rid>r\#[A-Za-z_]\w*)
 |(?P<id>[A-Za-z_][A-Za-z0-9_]*)
 |(?P<op><<=|>>=|\.\.=|\.\.\.|\.\.|::|->|=>|==|!=|<=|>=|&&|\|\||\+=|-=|\*=|/=|%=|\^=|&=|\|=|<<|>>|[-+*/%!&|^=<>.,;:(){}\[\]?\#@~$])
''', re.X)

def lex(src):
    out, i = [], 0
    while i < len(src):
        m = TOKEN.match(src, i)
        if not m: raise Unsupported(f'lex at {src[i:i+20]!r}')
        i = m.end()
        g = m.lastgroup
        if g == 'ws': continue
        v = m.group()
        if g == 'hashes': g = 'rstr'
        if g == 'rid': g, v = 'id', 'r_' + v[2:]
        elif g == 'rstr':
            body = v[v.index('"') + 1:v.rindex('"')]
            g = 'bstr' if v.startswith('b') else 'str'
            v = '"' + body.replace('\\', '\\\\').replace('"', '\\"') + '"'
        out.append((g, v))
    out.append(('eof', ''))
    return out

def int_value(v):
    v = re.sub(r'(u8|u16|u32|u64|u128|usize|i8|i16|i32|i64|i128|isize)$', '', v).replace('_', '')
    return int(v, 0)

def unescape(body):
    def one(m):
        e = m.group(1)
        if e.startswith('u{'): return chr(int(e[2:-1], 16))
        if e.startswith('x'): return chr(int(e[1:], 16))
        if e == '\n': return ''
        return {'n': '\n', 't': '\t', 'r': '\r', '0': '\0'}.get(e, e)
    body = re.sub(r'\\\n\s*', '', body)
    return re.sub(r'\\(u\{[0-9a-fA-F]+\}|x[0-9a-fA-F]{2}|.)', one, body, flags=re.S)

WILD = ('pwild',)
FUEL = '18446744073709551615'
INT_LIMITS = {'u8': (0, 255), 'u16': (0, 65535), 'u32': (0, 2**32 - 1), 'u64': (0, 2**64 - 1), 'usize': (0, 2**64 - 1), 'u128': (0, 2**128 - 1),
              'i8': (-2**7, 2**7 - 1), 'i16': (-2**15, 2**15 - 1), 'i32': (-2**31, 2**31 - 1), 'i64': (-2**63, 2**63 - 1), 'i128': (-2**127, 2**127 - 1)}

# ----------------------------------------------------------------------------- parser
class Parser:
    """The full surface of the crate's Rust; constructs outside the translator's subset become ('opaque', why) nodes,
    refused with that reason at translation rather than at parsing."""
    def __init__(self, toks): self.t, self.i = toks, 0
    def peek(self, n=0): return self.t[min(self.i + n, len(self.t) - 1)]
    def at(self, v, n=0):
        k, x = self.peek(n)
        return x == v and k in ('op', 'id')
    def eat(self, v=None):
        tok = self.t[self.i]
        if v is not None and tok[1] != v:
            ctx = ' '.join(x[1] for x in self.t[max(0, self.i - 8):self.i + 4])
            raise Unsupported(f'parse: expected {v!r}, found {tok[1]!r} near `{ctx}`')
        self.i += 1
        return tok[1]
    def opt(self, v):
        if self.at(v): self.i += 1; return True
        return False
    def ident(self):
        k, v = self.t[self.i]
        if k != 'id':
            ctx = ' '.join(x[1] for x in self.t[max(0, self.i - 8):self.i + 4])
            raise Unsupported(f'parse: expected identifier, found {v!r} near `{ctx}`')
        self.i += 1
        return v
    def attrs(self):
        while self.at('#'):
            self.eat(); self.opt('!'); self.skip_group('[', ']')
    def vis(self):
        if self.opt('pub') and self.at('('): self.skip_group('(', ')')
    def skip_group(self, o, c):
        self.eat(o); depth = 1
        while depth:
            v = self.eat(); depth += (v == o) - (v == c)
    def skip_angle(self):
        """skip a <...> group, nesting-aware (-> and >> handled)."""
        self.eat('<'); depth = 1
        while depth:
            v = self.eat()
            if v == '<': depth += 1
            elif v == '>': depth -= 1
            elif v == '>>': depth -= 2
            elif v in ('<<',): depth += 2
        return
    def generics(self):
        names = []
        if self.at('<'):
            start = self.i; self.skip_angle(); inner = self.t[start + 1:self.i - 1]
            depth = 0; expect = True
            for k, v in inner:
                if v in ('<', '('): depth += 1
                elif v in ('>', ')'): depth -= 1
                elif v == ',' and depth == 0: expect = True; continue
                if expect and depth == 0 and k == 'id' and v != 'const': names.append(v); expect = False
                elif expect and k == 'life': expect = False
        return names

    def file(self):
        items = []
        while self.peek()[0] != 'eof': items += self.item()
        return items
    def item(self):
        self.attrs(); self.vis()
        if self.at('use') or self.at('mod'):
            while not self.at(';'): self.eat()
            self.eat(';'); return []
        if self.at('const') and not self.at('fn', 1):
            self.eat(); name = self.ident(); self.eat(':'); t = self.type(); self.eat('='); e = self.expr(); self.eat(';')
            return [{'k': 'const', 'name': name, 'type': t, 'value': e, 'owner': None}]
        if self.at('struct'):
            self.eat(); name = self.ident(); gen = self.generics()
            if self.at('('):
                self.eat('('); fields = []
                while not self.at(')'): self.attrs(); self.vis(); fields.append((f'f{len(fields)}', self.type())); self.opt(',')
                self.eat(')'); self.where_clause(); self.opt(';')
                return [{'k': 'struct', 'name': name, 'fields': fields, 'generic': bool(gen), 'tuple_struct': True}]
            self.where_clause()
            if self.opt(';'): return [{'k': 'struct', 'name': name, 'fields': [], 'generic': bool(gen), 'unit_struct': True}]
            self.eat('{'); fields = []
            while not self.at('}'):
                self.attrs(); self.vis(); f = self.ident(); self.eat(':'); fields.append((f, self.type())); self.opt(',')
            self.eat('}'); return [{'k': 'struct', 'name': name, 'fields': fields, 'generic': bool(gen)}]
        if self.at('enum'):
            self.eat(); name = self.ident(); gen = self.generics(); self.where_clause(); self.eat('{'); variants = []
            while not self.at('}'):
                self.attrs(); v = self.ident(); fields = []
                if self.opt('{'):
                    while not self.at('}'):
                        self.attrs(); f = self.ident(); self.eat(':'); fields.append((f, self.type())); self.opt(',')
                    self.eat('}')
                elif self.opt('('):
                    while not self.at(')'): self.attrs(); fields.append((len(fields), self.type())); self.opt(',')
                    self.eat(')')
                if self.opt('='): self.expr()
                variants.append((v, fields)); self.opt(',')
            self.eat('}'); return [{'k': 'enum', 'name': name, 'variants': variants, 'generic': bool(gen)}]
        if self.at('impl'):
            self.eat(); name = self.ident(); self.generics(); self.eat('{'); out = []
            while not self.at('}'):
                for it in self.item(): it['owner'] = name; out.append(it)
            self.eat('}'); return out
        while self.at('unsafe') or self.at('async') or self.at('extern'):
            self.eat()
            if self.peek()[0] == 'str': self.eat()
        if self.at('fn') or (self.at('const') and self.at('fn', 1)): return [self.fn()]
        raise Unsupported(f'item {self.peek()[1]!r}')
    def where_clause(self):
        if self.opt('where'):
            depth = 0
            while not ((self.at('{') or self.at(';')) and depth == 0):
                v = self.eat()
                if v in ('<', '(', '['): depth += 1
                elif v in ('>', ')', ']'): depth -= 1
                elif v == '>>': depth -= 2
    def fn(self):
        self.opt('const'); self.eat('fn'); name = self.ident(); gen = self.generics(); self.eat('('); params = []; prelude = []
        while not self.at(')'):
            self.attrs()
            if self.at('&') or self.at('&&'):
                save = self.i; self.eat()
                if self.peek()[0] == 'life': self.eat()
                is_mut = self.opt('mut')
                if self.at('self'):
                    self.eat(); params.append(('self', ('mut', ('path', 'Self', [])) if is_mut else ('ref', ('path', 'Self', [])))); self.opt(','); continue
                self.i = save
            self.opt('mut')
            if self.at('self'):
                self.eat()
                if self.opt(':'): self.type()
                params.append(('self', ('path', 'Self', []))); self.opt(','); continue
            pat = self.pattern()
            self.eat(':'); t = self.type()
            if pat[0] == 'pid': params.append((pat[1], t))
            else:
                synthetic = f'arg{len(params)}'; params.append((synthetic, t))
                prelude.append(('let', pat, None, ('path', [synthetic]), None))
            self.opt(',')
        self.eat(')')
        ret = ('tuple', [])
        if self.opt('->'): ret = self.type()
        self.where_clause()
        body = self.block()
        if prelude: body = ('block', prelude + body[1], body[2])
        return {'k': 'fn', 'name': name, 'generics': gen, 'params': params, 'ret': ret, 'body': body, 'owner': None}

    def fn_sugar(self):
        """`(A, B) -> R` after Fn/FnMut/FnOnce/fn: the argument types and the result type."""
        self.eat('('); args = []
        while not self.at(')'): args.append(self.type()); self.opt(',')
        self.eat(')')
        ret = self.type() if self.opt('->') else ('tuple', [])
        return args, ret
    def bounds(self, why):
        """a trait bound list. `Fn(A) -> B` (with any `+ Send` and lifetimes after it) is a function type;
        `IntoIterator<Item = T>` and `Iterator<Item = T>` are a list; anything else is an opaque type."""
        k0, v0 = self.peek()
        # `dyn std::error::Error + Send + Sync`: a host error, which the compiler only ever displays; its message
        save = self.i; segs = []
        while self.peek()[0] == 'id':
            segs.append(self.eat())
            if not self.opt('::'): break
        if segs and segs[-1] == 'Error' and (len(segs) == 1 or segs[0] in ('std', 'core')):
            while self.opt('+'):
                if self.peek()[0] == 'life': self.eat()
                else: self.ident()
            return ('path', 'DynError', [])
        self.i = save
        if v0 in ('Fn', 'FnMut', 'FnOnce') and self.at('(', 1):
            kind = self.eat(); args, ret = self.fn_sugar()
            while self.opt('+'):
                if self.peek()[0] == 'life': self.eat()
                else: self.ident()
            return ('fnty', args, ret, kind)
        if v0 in ('IntoIterator', 'Iterator', 'ExactSizeIterator', 'DoubleEndedIterator') and self.at('<', 1):
            save = self.i
            self.eat(); self.eat('<')
            if self.peek()[0] == 'id' and self.peek()[1] == 'Item' and self.at('=', 1):
                self.eat(); self.eat('='); item = self.type()
                if self.opt('>'):
                    while self.opt('+'):
                        if self.peek()[0] == 'life': self.eat()
                        else: self.ident()
                    return ('path', 'Vec', [item])
            self.i = save
        depth = 0
        while True:
            k, v = self.peek()
            if depth == 0 and v in (',', ')', '>', '>>', ';', '=', '{', ']', '|') and not (v == '>' and depth > 0): break
            if v in ('<', '(', '['): depth += 1
            elif v in ('>', ')', ']'): depth -= 1
            elif v == '>>': depth -= 2
            if depth < 0: break
            self.eat()
        return ('opaque', why)
    def type(self):
        if self.at('&&'):
            self.eat()
            if self.peek()[0] == 'life': self.eat()
            self.opt('mut'); return self.type()
        if self.opt('&'):
            if self.peek()[0] == 'life': self.eat()
            if self.opt('mut'): return ('mut', self.type())
            inner = self.type()
            return ('ref', inner) if inner[0] != 'ref' else inner      # a shared reference, kept apart from an owned value
        if self.at('*'): self.eat(); self.opt('const'); self.opt('mut'); return ('path', 'RawPtr', [self.type()])
        if self.opt('!'): return ('opaque', 'never type')
        if self.opt('dyn'): return self.bounds('trait object (dyn)')
        if self.opt('impl'):
            save = self.i
            try:
                name = self.ident()
                if name == 'Into' and self.at('<'):
                    self.eat('<'); t = self.type(); self.eat('>'); return t
            except Unsupported: pass
            self.i = save
            return self.bounds('impl Trait type')
        if self.at('fn'):
            self.eat(); args, ret = self.fn_sugar()
            return ('fnty', args, ret, 'fn')
        if self.at('<'): self.skip_angle(); self.opt('::'); self.ident(); return ('opaque', 'qualified type path')
        if self.opt('['):
            t = self.type()
            if self.opt(';'): self.expr()
            self.eat(']'); return ('path', 'Vec', [t])
        if self.opt('('):
            ts = []
            while not self.at(')'): ts.append(self.type()); self.opt(',')
            self.eat(')'); return ('tuple', ts)
        if self.peek()[0] == 'life': self.eat(); return ('opaque', 'lifetime')
        name = self.ident(); args = []
        while self.opt('::'): name = self.ident()
        if self.at('<'):
            self.eat()
            while not (self.at('>') or self.at('>>')):
                if self.peek()[0] == 'life': self.eat()
                elif self.peek()[0] == 'id' and self.at('=', 1): self.eat(); self.eat('='); args.append(self.type())
                else: args.append(self.type())
                self.opt(',')
            if self.at('>>'): self.t[self.i] = ('op', '>')       # split `>>` closing two generic lists
            else: self.eat('>')
        if name == 'Self' and self.at('::'): self.eat(); self.ident(); return ('opaque', 'associated type')
        return ('path', name, args)

    # patterns
    def pattern(self):
        self.opt('|'); alts = [self.pat1()]
        while self.at('|') and not self.at('||'): self.eat(); alts.append(self.pat1())
        return alts[0] if len(alts) == 1 else ('por', alts)
    def lit_pattern(self):
        neg = self.opt('-')
        e = self.primary(True)
        if neg: e = ('neg', e)
        if self.at('..=') or self.at('...'):
            self.eat(); hi = self.primary(True); return ('prange', e, hi, True)
        if self.at('..') and self.peek(1)[0] in ('int', 'chr', 'byte'):
            self.eat(); hi = self.primary(True); return ('prange', e, hi, False)
        return ('plit', e)
    def pat1(self):
        if self.at('&') or self.at('&&'): self.eat(); self.opt('mut'); return self.pat1()
        k, v = self.peek()
        if k in ('int', 'str', 'chr', 'byte', 'bstr') or (v == '-' and self.peek(1)[0] == 'int'): return self.lit_pattern()
        if self.opt('..'): return ('prest',)
        if self.opt('['):
            ps = []
            while not self.at(']'): ps.append(self.pattern()); self.opt(',')
            self.eat(']')
            # a list is a cons list: `[p1, p2]` is cons p1 (cons p2 nil), and `[p, rest @ ..]` is cons p rest. A rest
            # before the end (`[.., last]`) has no such form and stays a slice pattern
            rests = [i for i, x in enumerate(ps) if x == ('prest',) or (x[0] == 'pat_at' and x[2] == ('prest',))]
            if not rests or rests == [len(ps) - 1]:
                if not rests: tail = ('pctor', ['__Nil'], [])
                else: tail = ('pwild',) if ps[-1] == ('prest',) else ('pid', ps[-1][1])
                for x in reversed(ps[:-1] if rests else ps): tail = ('pctor', ['__Cons'], [(0, x), (1, tail)])
                return tail
            return ('pslice', ps)
        if self.opt('('):
            ps = []
            while not self.at(')'): ps.append(self.pattern()); self.opt(',')
            self.eat(')'); return ps[0] if len(ps) == 1 else ('ptuple', ps)
        self.opt('box'); self.opt('ref'); self.opt('mut')
        segs = [self.ident()]
        while self.at('::'):
            self.eat()
            if self.at('<'): self.skip_angle(); continue
            segs.append(self.ident())
        if segs == ['_']: return WILD
        if segs[0] in ('true', 'false') and len(segs) == 1: return ('plit', ('bool', segs[0]))
        if self.opt('@'): return ('pat_at', segs[0], self.pat1())
        if self.opt('{'):
            fields = []
            while not self.at('}'):
                if self.opt('..'): continue
                self.opt('ref'); self.opt('mut')
                f = self.ident(); fields.append((f, self.pattern() if self.opt(':') else ('pid', f))); self.opt(',')
            self.eat('}'); return ('pctor', segs, fields)
        if self.opt('('):
            fields = []; rest = False
            while not self.at(')'):
                if self.opt('..'): rest = True; self.opt(','); continue
                fields.append((len(fields), self.pattern())); self.opt(',')
            self.eat(')'); return ('pctor', segs, fields)
        if len(segs) > 1 and segs[-1][:1].isupper() and segs[-1].isupper(): return ('pconst', segs)
        if len(segs) > 1 or segs[0][0].isupper():
            if len(segs) == 1 and segs[0].isupper() and len(segs[0]) > 1: return ('pconst', segs)
            return ('pctor', segs, [])
        return ('pid', segs[0])

    def block(self):
        self.eat('{'); stmts, tail = [], None
        while not self.at('}'):
            self.attrs()
            if self.at('}'): break
            if self.opt(';'): continue
            if self.at('use'):
                while not self.at(';'): self.eat()
                self.eat(';'); continue
            if self.at('fn') or (self.at('const') and self.at('fn', 1)): stmts.append(('item', self.fn())); continue
            if (self.at('const') or self.at('static')) and self.peek(1)[0] == 'id' and self.at(':', 2):
                # a local `const NAME: T = value;` is a named constant: an immutable let
                self.eat(); name = self.ident(); self.eat(':'); ty = self.type(); self.eat('='); init = self.expr(); self.eat(';')
                stmts.append(('let', ('pid', name), ty, init, None)); continue
            if self.at('const') or self.at('static'):
                while not self.at(';'): self.eat()
                self.eat(';'); stmts.append(('expr', ('opaque', 'item inside a function'))); continue
            if self.at('struct') or self.at('enum') or self.at('impl') or self.at('trait'):
                kw = self.eat()
                while not self.at('{') and not self.at(';'): self.eat()
                if self.at('{'): self.skip_group('{', '}')
                else: self.eat(';')
                stmts.append(('expr', ('opaque', f'{kw} inside a function'))); continue
            if self.at('let'):
                self.eat(); pat = self.pattern(); ty = None
                if self.opt(':'): ty = self.type()
                if self.opt(';'): stmts.append(('let', pat, ty, None, None)); continue
                self.eat('='); init = self.expr(); els = None
                if self.opt('else'): els = self.block()
                self.eat(';'); stmts.append(('let', pat, ty, init, els)); continue
            # an expression that ends in a block is a whole statement, as Rust parses it: `for x in y { .. } (a, b)` is a
            # loop and then a tuple, not a call of the loop
            block_like = self.at('for') or self.at('while') or self.at('loop') or self.at('if') or self.at('match') or self.at('unsafe') or (self.peek()[0] == 'life' and self.at(':', 1))
            e = self.primary(False) if block_like else self.expr()
            if block_like and e[0] not in ('while', 'whilelet', 'for', 'loop', 'labelled', 'if', 'iflet', 'match', 'unsafe', 'block'): e = self.postfix(e, False)
            # `if a { .. } else if b { .. }` with no final `else` is `()`, as an `if` without `else` is: close the chain
            # with an empty `else`, so that at the end of a block it is a unit value and not a missing branch
            if e[0] in ('if', 'iflet') and (e[3] if e[0] == 'if' else e[4]) is not None: e = self.close_chain(e)
            unit_like = e[0] in ('while', 'whilelet', 'for', 'loop', 'labelled') or (e[0] == 'if' and e[3] is None) or (e[0] == 'iflet' and e[4] is None)
            if self.opt(';') or unit_like:
                if unit_like and self.at('}') and e[0] == 'loop': tail = e
                else: stmts.append(('expr', e))
            elif self.at('}'): tail = e
            elif e[0] in ('if', 'iflet', 'match', 'block', 'unsafe'): stmts.append(('expr', e))
            else: raise Unsupported(f'parse: statement end near {self.peek()[1]!r}')
        self.eat('}')
        return ('block', stmts, tail)

    BIN = [('||',), ('&&',), ('==', '!=', '<', '>', '<=', '>='), ('|',), ('^',), ('&',), ('<<', '>>'), ('+', '-'), ('*', '/', '%')]
    COMPOUND = {'+=': '+', '-=': '-', '*=': '*', '/=': '/', '%=': '%', '|=': '|', '&=': '&', '^=': '^', '<<=': '<<', '>>=': '>>'}
    def expr(self, ns=False):
        if self.at('return'):
            self.eat()
            return ('return', None if self.at(';') or self.at('}') or self.at(',') or self.at(')') else self.expr(ns))
        if self.at('continue'):
            self.eat(); label = self.eat() if self.peek()[0] == 'life' else None; return ('continue', label)
        if self.at('break'):
            self.eat(); label = self.eat() if self.peek()[0] == 'life' else None
            value = None if self.at(';') or self.at('}') or self.at(',') or self.at(')') else self.expr(ns)
            return ('break', label, value)
        if self.at('..') or self.at('..='):
            inc = self.eat() == '..='
            hi = None if self.at(']') or self.at(')') or self.at(',') or self.at('}') else self.binary(0, ns)
            return ('range', None, hi, inc)
        lhs = self.binary(0, ns)
        if self.at('..') or self.at('..='):
            inc = self.eat() == '..='
            hi = None if self.at(']') or self.at(')') or self.at(',') or self.at('}') or self.at('{') else self.binary(0, ns)
            return ('range', lhs, hi, inc)
        if self.at('='):
            self.eat(); return ('assign', lhs, self.expr(ns))
        if self.peek()[0] == 'op' and self.peek()[1] in self.COMPOUND:
            op = self.COMPOUND[self.eat()]; return ('assign', lhs, ('bin', op, lhs, self.expr(ns)))
        return lhs
    def binary(self, level, ns):
        if level == len(self.BIN): return self.unary(ns)
        lhs = self.binary(level + 1, ns)
        while self.peek()[0] == 'op' and self.peek()[1] in self.BIN[level]:
            op = self.eat(); lhs = ('bin', op, lhs, self.binary(level + 1, ns))
        return lhs
    def unary(self, ns):
        if self.at('!'): self.eat(); return ('not', self.unary(ns))
        if self.at('&&'): self.eat(); self.opt('mut'); return self.unary(ns)
        if self.at('&'):
            self.eat()
            if self.opt('mut'): return ('mutref', self.unary(ns))
            return self.unary(ns)
        if self.at('*'): self.eat(); return ('deref', self.unary(ns))
        if self.at('-'): self.eat(); return ('neg', self.unary(ns))
        e = self.postfix(self.primary(ns), ns)
        while self.opt('as'): e = ('cast', e, self.type())
        return e
    def args(self, o='(', c=')'):
        self.eat(o); a = []
        while not self.at(c):
            self.attrs(); a.append(self.expr()); self.opt(',')
        self.eat(c); return a
    def postfix(self, e, ns):
        while True:
            if self.at('?'): self.eat(); e = ('try', e)
            elif self.at('.'):
                self.eat(); k, v = self.peek()
                if k == 'int': self.eat(); e = ('field', e, int_value(v)); continue
                if v == 'await': self.eat(); e = ('opaque', 'async'); continue
                name = self.ident()
                if self.at('::'):
                    self.eat(); start = self.i; self.skip_angle()
                    inner = [t[1] for t in self.t[start + 1:self.i - 1]]
                    if len(inner) == 1 and name in ('parse', 'collect', 'into', 'try_into', 'sum', 'product'): name = f'{name}_{inner[0]}'
                if self.at('('): e = ('mcall', e, name, self.args())
                else: e = ('field', e, name)
            elif self.at('('): e = ('call', e, self.args())
            elif self.at('['): e = ('index', e, self.args('[', ']')[0])
            else: return e
    def loop_expr(self, label=None):
        if self.at('while'):
            self.eat()
            if self.opt('let'):
                pat = self.pattern(); self.eat('='); scrut = self.expr(True)
                node = ('whilelet', pat, scrut, self.block())
            else:
                cond = self.expr(True); node = ('while', cond, self.block())
        elif self.at('for'):
            self.eat(); pat = self.pattern(); self.eat('in'); it = self.expr(True)
            node = ('for', pat, it, self.block())
        else:
            self.eat('loop'); node = ('loop', self.block())
        return ('labelled', label, node) if label else node
    def primary(self, ns):
        k, v = self.peek()
        if k == 'int': self.eat(); return ('int', int_value(v))
        if k == 'str': self.eat(); return ('str', unescape(v[1:-1]))
        if k == 'bstr': self.eat(); return ('bstr', unescape(v[2:-1]))
        if k == 'chr': self.eat(); return ('chr', unescape(v[1:-1]))
        if k == 'byte': self.eat(); return ('byte', ord(unescape(v[2:-1])))
        if k == 'life' and self.at(':', 1):
            label = self.eat(); self.eat(':')
            if self.at('{'): return ('labelled', label, ('block_label', self.block()))
            return self.loop_expr(label)
        if self.at('('):
            self.eat(); es = []; comma = False
            while not self.at(')'):
                es.append(self.expr())
                if self.opt(','): comma = True
            self.eat(')')
            return es[0] if len(es) == 1 and not comma else ('tuple', es)
        if self.at('['):
            self.eat()
            if self.at(']'): self.eat(); return ('array', [])
            first = self.expr()
            if self.opt(';'):
                n = self.expr(); self.eat(']'); return ('arrayrep', first, n)
            items = [first]
            while self.opt(','):
                if self.at(']'): break
                items.append(self.expr())
            self.eat(']'); return ('array', items)
        if self.at('{'): return self.block()
        if self.at('unsafe'): self.eat(); return ('opaque', 'unsafe block')
        if self.at('async'): self.eat(); self.opt('move'); self.block(); return ('opaque', 'async block')
        if self.at('move'): self.eat()
        if self.at('||') or self.at('|'):
            ps = []
            if self.eat() == '|':
                while not self.at('|'):
                    p = self.pat1()
                    ps.append((p, self.type() if self.opt(':') else None)); self.opt(',')
                self.eat('|')
            ret = self.type() if self.opt('->') else None
            body = self.block() if ret is not None else self.expr()
            return ('closure', ps, body, ret)
        if self.at('if'):
            self.eat()
            if self.opt('let'):
                pat = self.pattern(); self.eat('='); scrut = self.expr(True)
                if self.at('&&'): raise Unsupported('parse: let chain')
                then = self.block()
                return ('iflet', pat, scrut, then, self.else_part())
            cond = self.expr(True); then = self.block()
            return ('if', cond, then, self.else_part())
        if self.at('match'):
            self.eat(); scrut = self.expr(True); self.eat('{'); arms = []
            while not self.at('}'):
                self.attrs()
                pat = self.pattern(); guard = self.expr() if self.opt('if') else None
                self.eat('=>')
                if self.at('{'):
                    body = self.block()
                    body = self.postfix(body, False) if self.at('.') or self.at('?') else body
                else: body = self.expr()
                self.opt(','); arms.append((pat, guard, body))
            self.eat('}'); return ('match', scrut, arms)
        if self.at('while') or self.at('for') or self.at('loop'): return self.loop_expr()
        if self.at('<'):
            self.skip_angle()
            while self.opt('::'): self.ident()
            return ('opaque', 'qualified path')
        if k == 'id':
            if v in ('true', 'false'): self.eat(); return ('bool', v)
            segs = [self.ident()]
            while self.at('::'):
                self.eat()
                if self.at('<'): self.skip_angle()
                else: segs.append(self.ident())
            if self.at('!') and not self.at('=', 1):
                self.eat(); name = segs[-1]
                if name == 'matches':
                    self.eat('('); e = self.expr(); self.eat(','); pat = self.pattern()
                    guard = self.expr() if self.opt('if') else None
                    self.opt(','); self.eat(')'); return ('match', e, [(pat, guard, ('bool', 'true')), (WILD, None, ('bool', 'false'))])
                if self.at('['):
                    a = self.primary(ns)
                    if name == 'vec': return a
                    return ('macro', name, [a])
                if self.at('{'): self.skip_group('{', '}'); return ('opaque', f'macro {name}! with braces')
                return ('macro', name, self.args())
            if self.at('{') and not ns and segs[-1][0].isupper():
                self.eat(); fields = []; base = None
                while not self.at('}'):
                    self.attrs()
                    if self.opt('..'): base = self.expr(); continue
                    f = self.ident() if self.peek()[0] == 'id' else str(int_value(self.eat()))
                    fields.append((f, self.expr() if self.opt(':') else ('path', [f]))); self.opt(',')
                self.eat('}'); return ('struct', segs, fields, base) if base is not None else ('struct', segs, fields)
            return ('path', segs)
        raise Unsupported(f'parse: expression at {v!r}')
    def close_chain(self, e):
        els = e[3] if e[0] == 'if' else e[4]
        if els is None: new = ('block', [], None)
        elif els[0] == 'block' and not els[1] and els[2] is not None and els[2][0] in ('if', 'iflet'):
            inner = els[2]
            if (inner[3] if inner[0] == 'if' else inner[4]) is None or self.chain_open(inner): new = ('block', [], self.close_chain(inner))
            else: return e
        else: return e
        return e[:3] + (new,) if e[0] == 'if' else e[:4] + (new,)
    def chain_open(self, e):
        els = e[3] if e[0] == 'if' else e[4]
        if els is None: return True
        return els[0] == 'block' and not els[1] and els[2] is not None and els[2][0] in ('if', 'iflet') and self.chain_open(els[2])
    def else_part(self):
        if not self.opt('else'): return None
        if self.at('if'): return ('block', [], self.primary(False))
        return self.block()

# ----------------------------------------------------------------------------- types
BYTES, NAT, BOOL, UNIT, I128, UINT8 = ('Bytes',), ('Nat',), ('Bool',), ('Unit',), ('I128',), ('UInt8',)
class Types:
    def __init__(self): self.bind, self.n = {}, 0
    def var(self): self.n += 1; return ('Var', self.n)
    def res(self, t):
        while t[0] == 'Var' and t[1] in self.bind: t = self.bind[t[1]]
        return t
    def deep(self, t):
        t = self.res(t)
        return tuple(self.deep(x) if isinstance(x, tuple) else x for x in t)
    def nat(self, t):
        """an undetermined numeric type defaults to Nat where an operation needs to know."""
        t = self.res(t)
        if t[0] == 'Var':
            self.bind[t[1]] = NAT
            if not hasattr(self, 'defaulted'): self.defaulted = set()
            self.defaulted.add(t[1]); return NAT
        return t
    def was_defaulted(self, t):
        """True when t reached Nat by `nat`'s default rather than by a declared type."""
        seen = getattr(self, 'defaulted', set())
        while t[0] == 'Var':
            if t[1] in seen: return True
            if t[1] not in self.bind: return False
            t = self.bind[t[1]]
        return False
    def _unused_nat_tail(self, t):
        return t
    def unify(self, a, b, why=''):
        a, b = self.res(a), self.res(b)
        if a == b: return
        if a[0] == 'Var': self.bind[a[1]] = b; return
        if b[0] == 'Var': self.bind[b[1]] = a; return
        if {a[0], b[0]} == {'List', 'Iter'}: return self.unify(a[1], b[1], why)
        if a[0] != b[0] or len(a) != len(b): raise Unsupported(f'type mismatch {self.show(a)} vs {self.show(b)} {why}')
        for x, y in zip(a[1:], b[1:]):
            if isinstance(x, tuple): self.unify(x, y, why)
            elif x != y: raise Unsupported(f'type mismatch {self.show(a)} vs {self.show(b)} {why}')
    def show(self, t):
        t = self.res(t); h = t[0]
        if h == 'Var': return f'\u00a7{t[1]}\u00a7'
        if h in ('Bytes', 'Nat', 'Bool', 'Unit', 'UInt8'): return h
        if h == 'Ptr' or (h == 'Map' and self.res(t[1]) == PTR): return 'Unit'
        if h == 'Fixed': return t[1]
        if h == 'Fn': return f'(Fn ({" ".join(self.show(x) for x in t[1][1:])}) {self.show(t[2])})'
        if h == 'I128': return 'RustStd::I128'
        if h == 'Named': return t[1]
        if h in ('List', 'Iter', 'Set'): return f'(List {self.show(t[1])})'
        if h == 'Option': return f'(Option {self.show(t[1])})'
        if h == 'Result': return f'(Result {self.show(t[1])} {self.show(t[2])})'
        if h == 'Prod': return f'({self.show(t[1])} * {self.show(t[2])})'
        if h == 'Tup':
            out = self.show(t[-1])
            for x in reversed(t[1:-1]): out = f'({self.show(x)} * {out})'
            return out
        if h == 'Map':
            key = self.res(t[1]); rep = self.map_rep(t[1], t[2])
            if rep == 'tree': return f'(LexLeanStd.OrdMap::OrdMap {self.show(t[2])})'
            if rep == 'keyed': return f'(LexLeanStd.OrdMap::OrdMap ({self.show(t[1])} * {self.show(t[2])}))'
            if rep is None: return f'(@MAPTY {self.show(key) if key[0] == "Var" else self.marker(t[1])} {self.show(t[2])})'
            return f'(List ({self.show(t[1])} * {self.show(t[2])}))'
        raise Unsupported(f'type {t[0]}')
    def holds_recursive(self, t):
        """True when t mentions a recursive type of the crate, None while part of it is unresolved."""
        t = self.res(t)
        if t[0] == 'Var': return None
        if t[0] == 'Named' and t[1] in getattr(self, 'rec_names', ()): return True
        seen = [self.holds_recursive(x) for x in t[1:] if isinstance(x, tuple)]
        if True in seen: return True
        return None if None in seen else False
    def key_encodes(self, key):
        """True when the key has an order-preserving Bytes encoding, None while it is unresolved."""
        key = self.res(key)
        if key[0] == 'Var': return None
        if key == NAT: return True
        if key[0] in ('Prod', 'Tup'):
            parts = [self.res(x) for x in (key[1:] if key[0] == 'Tup' else (key[1], key[2]))]
            if any(x[0] == 'Var' for x in parts): return None
            return all(x == NAT for x in parts[:-1]) and parts[-1] in (NAT, BYTES)
        return False
    def map_rep(self, key, value=None):
        """'tree' for a key of Bytes and a value holding no recursive type, 'list' for another known map, None while
        the key or the value is unresolved."""
        key = self.res(key)
        if key[0] == 'Var': return None
        if key == BYTES:
            if value is None: return 'tree'
            held = self.holds_recursive(value)
            return None if held is None else ('list' if held else 'tree')
        encodes = self.key_encodes(key)
        if encodes is None: return None
        if not encodes: return 'list'
        if value is None: return 'keyed'
        held = self.holds_recursive(value)
        return None if held is None else ('list' if held else 'keyed')
    def marker(self, key):
        """a resolved key as a fresh variable bound to it, so a placeholder always names its key by a variable."""
        v = self.var(); self.bind[v[1]] = self.res(key); return self.show(v) if False else f'\u00a7{v[1]}\u00a7'
    def empty_map(self, key, value):
        rep = self.map_rep(key, value)
        if rep == 'tree': return f'(LexLeanStd.OrdMap::emptyMap [{self.show(value)}])'
        if rep == 'keyed': return f'(LexLeanStd.OrdMap::emptyMap [({self.show(key)} * {self.show(value)})])'
        if rep is None:
            k = self.res(key)
            return f'(@MAPNIL {self.show(k) if k[0] == "Var" else self.marker(key)} {self.show(value)})'
        if self.res(key) == PTR: return '()'
        return f'(nil ({self.show(key)} * {self.show(value)}))'
    def settle_maps(self, text):
        """each placeholder map, its key now resolved, as the tree or the list it is."""
        def balanced(at):
            depth, k = 0, at
            while True:
                depth += {'(': 1, ')': -1}.get(text[k], 0)
                if depth == 0: return k
                k += 1
        for _ in range(100000):
            j = min([x for x in (text.find('(@MAPTY \u00a7'), text.find('(@MAPNIL \u00a7')) if x >= 0], default=-1)
            if j < 0: return text
            nil = text.startswith('(@MAPNIL', j)
            start = j + (9 if nil else 8)
            close = text.index('\u00a7', start + 1)
            var = int(text[start + 1:close])
            end = balanced(j)
            value = text[close + 2:end]
            key = self.res(('Var', var))
            if key[0] == 'Var': raise Unsupported('a map whose key type stayed undetermined')
            held = any(self.holds_recursive(('Var', int(n))) for n in re.findall('\u00a7(\\d+)\u00a7', value)) or \
                bool(set(re.findall(r'[A-Za-z_][A-Za-z0-9_]*', re.sub('\u00a7\\d+\u00a7', '', value))) & getattr(self, 'rec_names', set()))
            if key != BYTES and not held and self.key_encodes(key) and not self.holds_recursive(key):
                kv = f'({self.show(key)} * {value})'
                new = f'(LexLeanStd.OrdMap::emptyMap [{kv}])' if nil else f'(LexLeanStd.OrdMap::OrdMap {kv})'
            elif key == BYTES and not held: new = f'(LexLeanStd.OrdMap::emptyMap [{value}])' if nil else f'(LexLeanStd.OrdMap::OrdMap {value})'
            elif key == PTR and nil: new = '()'
            elif key == PTR: new = 'Unit'
            else: new = f'(nil ({self.show(key)} * {value}))' if nil else f'(List ({self.show(key)} * {value}))'
            text = text[:j] + new + text[end + 1:]
        raise Unsupported('too many maps to settle')
    def drop_caches(self, text):
        out, i = [], 0
        while True:
            j = text.find('(nil (\u00a7', i)
            if j < 0: out.append(text[i:]); return ''.join(out)
            n = text[j + 7:text.index('\u00a7', j + 7)]
            if not n.isdigit() or self.res(('Var', int(n))) != PTR:
                out.append(text[i:j + 1]); i = j + 1; continue
            depth, k = 0, j
            while True:
                depth += {'(': 1, ')': -1}.get(text[k], 0)
                if depth == 0: break
                k += 1
            out.append(text[i:j] + '()'); i = k + 1
    def patch(self, text):
        text = text.replace('@FUEL@', FUEL)
        def lit(m):
            t = self.nat(('Var', int(m.group(1))))
            value = int(m.group(2))
            if t == NAT and value >= 0: return m.group(2)
            if t == I128: return wide(value < 0, abs(value))
            if t == UINT8 and value >= 0: return f'({m.group(2)} : UInt8)'
            if t[0] == 'Fixed': return f'({m.group(2)} : {t[1]})'
            raise Unsupported(f'an integer literal {value} of type {self.show(t)}')
        text = re.sub('\u00a6(\\d+)\\|(-?\\d+)\u00a6', lit, text)
        text = self.settle_maps(self.drop_caches(text))
        for _ in range(30):
            new = re.sub('\u00a7(\\d+)\u00a7', lambda m: self.show(self.nat(('Var', int(m.group(1))))) if self.res(('Var', int(m.group(1))))[0] == 'Var' else self.show(('Var', int(m.group(1)))), text)
            if new == text: break
            text = new
        if '\u00a7' in text: raise Unsupported('a type stayed undetermined')
        return text

KEYWORDS = set('let if match fun forall nil cons first second not apply ctor record project prim ref checked then else with true false'.split())
KEYWORDS |= set('id ite dite cond absurd flip default decide panic toString repr max min compare pure bind map seq fst snd get set modify and or xor sorry rfl trivial show have at by do from in open end section namespace instance class structure inductive def theorem lemma example variable universe mutual where deriving return for unless mut try catch finally break continue this Type Prop Sort some none ok error unit private public protected noncomputable partial unsafe opaque abbrev attribute export local scoped macro syntax notation infix prefix postfix calc suffices obtain Id Option List Nat String Bool Unit Int Array ByteArray Except Prod Sum'.split())   # Lean names a binder must not shadow
MUTATORS = {'push', 'push_str', 'insert', 'sort', 'next', 'pop', 'truncate', 'extend', 'remove', 'retain', 'clear', 'dedup', 'sort_unstable', 'sort_by', 'sort_by_key', 'sort_unstable_by', 'extend_from_slice', 'append', 'swap', 'reverse', 'take', 'insert_str', 'push_back', 'push_front', 'pop_front', 'pop_back', 'drain'}
IDENTITY = {'to_owned', 'clone', 'cloned', 'copied', 'to_string', 'into', 'into_iter', 'iter', 'as_str', 'as_ref', 'to_vec'}
EXTERN_TYPES = {'Diagnostic': BYTES}
PTR = ('Ptr',)
EXTERN_FNS = {'is_closed_lean_name': ('AxiomOutput::isClosedLeanName', [BYTES], BOOL)}
NAT_KINDS = {'usize', 'u8', 'u16', 'u32', 'u64'}

def camel(s):
    p = s.split('_'); return p[0] + ''.join(x.capitalize() for x in p[1:])
def lexstr(s): return json.dumps(s, ensure_ascii=False)
def text(s):
    if '%' in s:                                   # LexLean source forbids a raw percent sign, string literals included
        parts = s.split('%'); out = None
        for i, part in enumerate(parts):
            pieces = ([f'(AxiomOutput::text {lexstr(part)})'] if part else []) + (['0x25'] if i < len(parts) - 1 else [])
            for x in pieces: out = x if out is None else f'(AxiomOutput::cat {out} {x})'
        return out
    return f'(AxiomOutput::text {lexstr(s)})'
def cat(parts):
    merged = []
    for p in parts:
        if isinstance(p, tuple) and merged and isinstance(merged[-1], tuple): merged[-1] = ('s', merged[-1][1] + p[1])
        else: merged.append(p)
    out = None
    for p in merged:
        v = text(p[1]) if isinstance(p, tuple) else p
        out = v if out is None else f'(AxiomOutput::cat {out} {v})'
    return out if out is not None else text('')
def wide(negative, magnitude):
    if magnitude < 2**64: return f'(RustStd::i128Of {"true" if negative else "false"} {magnitude})'
    return f'(RustStd::i128Big {"true" if negative else "false"} {text(str(magnitude))})'
def is_atom(v): return re.fullmatch(r'[A-Za-z_][A-Za-z0-9_]*', v) is not None and v not in ('true', 'false')

class Ctx:
    def __init__(self, ret, ret_t, fall=None): self.ret, self.ret_t, self.fall, self.brk = ret, ret_t, fall, None

# ----------------------------------------------------------------------------- translator
class Translator:
    def __init__(self, items):
        self.T = Types(); self.items = items
        self.structs = {i['name']: i for i in items if i['k'] == 'struct'}
        self.enums = {i['name']: i for i in items if i['k'] == 'enum'}
        self.consts = {i['name']: i for i in items if i['k'] == 'const'}
        self.fns = {}
        for f in items:
            if f['k'] == 'fn':
                base = camel(f['name']) if not f['owner'] else camel(f['owner'][0].lower() + f['owner'][1:] + '_' + f['name'])
                f['lex'] = ('f' if not f['owner'] else 'm') + base[:1].upper() + base[1:]   # never a generated binder; methods apart from free fns
                self.fns[(f['owner'], f['name'])] = f
        self.by_name = {}
        for key in self.fns: self.by_name.setdefault(key[1], key)
        self.derived, self.derived_names = [], {}
        self.status = {}
        self.rec_keys, self.scc = set(), {}
        self.eff_closures = set(); self.group_has_helpers = {}; self.cur_scc = set(); self.blocked_by = {}; self.key_calls = {}
        self.spec_texts, self.spec_of, self.spec_n = [], {}, 0
        self.type_groups()

    # -- types
    def ty(self, t, owner=None, generics=None):
        if t[0] in ('mut', 'ref'): return self.ty(t[1], owner, generics)
        if t[0] == 'lexty': return t[1]
        if t[0] == 'opaque': raise Unsupported(t[1])
        if t[0] == 'fnty':
            if getattr(self, 'in_type_groups', False): raise Unsupported('a function stored in a type (an escaping closure)')
            if t[3] == 'FnMut': raise Unsupported('an FnMut parameter (a closure that mutates what it captures)')
            if any(x[0] == 'mut' for x in t[1]): raise Unsupported('a function type with `&mut` parameters (its effects would be lost)')
            args = [self.ty(x, owner, generics) for x in t[1]] or [UNIT]
            return ('Fn', ('Args',) + tuple(args), self.ty(t[2], owner, generics))
        if t[0] == 'tuple':
            ts = [self.ty(x, owner, generics) for x in t[1]]
            if not ts: return UNIT
            if len(ts) == 1: return ts[0]
            if len(ts) == 2: return ('Prod', ts[0], ts[1])
            return ('Tup',) + tuple(ts)
        _, n, a = t
        if generics and n in generics: return generics[n]
        a = [self.ty(x, owner, generics) for x in a]
        hooked = self.hook_ty(n, a, owner)
        if hooked is not None: return hooked
        if n in ('String', 'str'): return BYTES
        if n in NAT_KINDS: return NAT
        if n in ('i128', 'u128'): return I128
        if n == 'bool': return BOOL
        if n == 'Box': return a[0]
        if n == 'Vec': return ('List', a[0])
        if n == 'Option': return ('Option', a[0])
        if n == 'Result': return ('Result', a[0], a[1]) if len(a) == 2 else ('Result', a[0] if a else UNIT, UNIT)
        if n == 'BTreeMap': return ('Map', a[0], a[1])
        if n == 'BTreeSet': return ('Set', a[0])
        if n == 'Self':
            if owner in getattr(self, 'type_error', {}) and not getattr(self, 'in_type_groups', False): raise Unsupported(f'uses type {owner} ({self.type_error[owner]})')
            if owner not in self.structs and owner not in self.enums: raise Unsupported(f'method of a type that is not translated ({owner})')
            return ('Named', owner)
        if n in EXTERN_TYPES: return EXTERN_TYPES[n]
        if n in self.structs or n in self.enums:
            if n in getattr(self, 'type_error', {}) and not getattr(self, 'in_type_groups', False): raise Unsupported(f'uses type {n} ({self.type_error[n]})')
            return ('Named', n)
        raise Unsupported(f'type {n}')
    LEAN_MEMBERS = {'rec', 'recOn', 'casesOn', 'noConfusion', 'noConfusionType', 'below', 'brecOn', 'binductionOn', 'ibelow',
                    'mk', 'sizeOf', 'ctorIdx', 'toCtorIdx', 'injEq', 'inj', 'ext', 'ext_iff', 'mk_injEq'}
    def fname(self, f):
        """a field's name in LexLean: one Lean declares for every type gets a suffix."""
        return f + 'Field' if isinstance(f, str) and (f in self.LEAN_MEMBERS or f in KEYWORDS) else f
    def field_types(self, name):
        fields = [(self.fname(f), self.ty(t, name)) for f, t in self.structs[name]['fields']]
        return fields or [('unit', UNIT)]           # LexLean has no empty structure: an empty struct holds a unit
    def named_in(self, t):
        if t[0] == 'Named': yield t[1]
        for x in t[1:]:
            if isinstance(x, tuple): yield from self.named_in(x)
    def type_fields(self, name):
        if name in self.structs: return [t for _, t in self.field_types(name)]
        return [self.ty(t, name) for _, fs in self.enums[name]['variants'] for _, t in fs]
    def type_groups(self):
        self.in_type_groups = True
        try: return self.type_groups_inner()
        finally: self.in_type_groups = False
    def type_groups_inner(self):
        names = [i['name'] for i in self.items if i['k'] in ('struct', 'enum')]
        self.type_error = {}
        deps = {}
        for n in names:
            try: deps[n] = [d for t in self.type_fields(n) for d in self.named_in(t) if d in self.structs or d in self.enums]
            except Unsupported as x: self.type_error[n] = str(x); deps[n] = []
        changed = True
        while changed:
            changed = False
            for n in names:
                if n not in self.type_error and any(d in self.type_error for d in deps[n]):
                    self.type_error[n] = 'uses ' + next(d for d in deps[n] if d in self.type_error); changed = True
        reach = {n: set(deps[n]) for n in names}
        for _ in names:
            for n in names:
                for d in list(reach[n]): reach[n] |= reach.get(d, set())
        self.type_rec = {n: n in reach[n] for n in names}
        self.T.rec_names = {n for n, r in self.type_rec.items() if r}
        self.type_group = {}
        for n in names:
            group = sorted(m for m in names if m in reach[n] and n in reach[m])
            if len(group) > 1: self.type_group[n] = ''.join(group)
        order, seen = [], set()
        def visit(n):
            if n in seen: return
            seen.add(n)
            for d in deps[n]: visit(d)
            order.append(n)
        for n in names: visit(n)
        self.type_order = order
    def as_inductive(self, name): return name in self.enums or name in self.type_group or self.type_rec.get(name, False)
    def ctors_of(self, t):
        t = self.T.res(t)
        if t[0] == 'Option': return [('Option.some', 'Some', [(0, t[1])]), ('Option.none', 'None', [])]
        if t[0] in ('List', 'Iter'): return [('List.cons', '__Cons', [(0, t[1]), (1, ('List', t[1]))]), ('List.nil', '__Nil', [])]
        if t[0] == 'Result': return [('Result.ok', 'Ok', [(0, t[1])]), ('Result.error', 'Err', [(0, t[2])])]
        if t[0] == 'Named' and t[1] in self.enums:
            return [(f'{t[1]}.{v}', v, [(k, self.ty(ft, t[1])) for k, ft in fs]) for v, fs in self.enums[t[1]]['variants']]
        if t[0] == 'Named' and t[1] in self.structs and self.as_inductive(t[1]): return [(f'{t[1]}.mk', t[1], self.field_types(t[1]))]
        raise Unsupported(f'pattern on {self.T.show(t)}')
    def declare(self, name):
        group = f',\n  mutual: {self.type_group[name]}' if name in self.type_group else ''
        if not self.as_inductive(name):
            fields = ', '.join(f'{{name: {f}, type: {self.T.show(t)}}}' for f, t in self.field_types(name))
            return 'structure{\n  name: %s,\n  type_parameters: [],\n  parameters: [],\n  fields: [%s]\n}' % (name, fields)
        ctors = ', '.join('{name: %s, fields: [%s]}' % (c.split('.')[1], ', '.join(self.T.show(t) for _, t in fs)) for c, _, fs in self.ctors_of(('Named', name)))
        return 'inductive{\n  name: %s,\n  type_parameters: [],\n  parameters: [],\n  constructors: [%s]%s\n}' % (name, ctors, group)
    def default(self, t, depth=0):
        """a value of type t, for the branch fuel exhaustion and out-of-range indexing take; both unreachable in practice."""
        T, t = self.T, self.T.nat(t)
        if depth > 6: raise Unsupported(f'no default value for {T.show(t)}')
        if t == NAT: return '0'
        if t == BOOL: return 'false'
        if t == BYTES: return text('')
        if t == UNIT: return '()'
        if t == I128: return '(RustStd::i128Of false 0)'
        if t == UINT8: return '(0 : UInt8)'
        if t[0] in ('List', 'Iter', 'Set'): return f'(nil {T.show(t[1])})'
        if t == PTR or (t[0] == 'Map' and T.res(t[1]) == PTR): return '()'
        if t[0] == 'Map': return T.empty_map(t[1], t[2])
        if t[0] == 'Option': return f'(ctor Option.none [{T.show(t[1])}])'
        if t[0] == 'Result': return f'(ctor Result.error [{T.show(t[1])}, {T.show(t[2])}] {self.default(t[2], depth + 1)})'
        if t[0] == 'Prod': return f'({self.default(t[1], depth + 1)}, {self.default(t[2], depth + 1)})'
        if t[0] == 'Tup': return self.tup_make([self.default(x, depth + 1) for x in t[1:]])
        if t[0] == 'Named':
            if not self.as_inductive(t[1]):
                return f'(record {t[1]} {{{", ".join(f"{f} := {self.default(ft, depth + 1)}" for f, ft in self.field_types(t[1]))}}})'
            for lexc, _, fs in sorted(self.ctors_of(t), key=lambda c: len(c[2])):
                try: return f'(ctor {lexc}{"".join(" " + self.default(ft, depth + 1) for _, ft in fs)})'
                except Unsupported: continue
        raise Unsupported(f'no default value for {T.show(t)}')

    # -- derived Debug, equality and set membership
    def display(self, v, t): raise Unsupported(f'Display of {self.T.show(t)}')
    def deferred(self): raise Unsupported('a placeholder value')
    def debug_value(self, v, t): return self.debug(v, t)
    def mangle(self, t):
        t = self.T.nat(t)
        if t[0] == 'Named': return t[1]
        # bracketed, so that different types never mangle alike: (List (A * B)) is `ListOfPairOfAAndBEndEnd`
        if t[0] in ('List', 'Iter', 'Set'): return 'ListOf' + self.mangle(t[1]) + 'End'
        if t[0] in ('Option', 'Result', 'Map'): return t[0] + 'Of' + 'And'.join(self.mangle(x) for x in t[1:]) + 'End'
        if t[0] == 'Prod': return 'PairOf' + self.mangle(t[1]) + 'And' + self.mangle(t[2]) + 'End'
        if t[0] == 'Tup': return 'TupleOf' + 'And'.join(self.mangle(x) for x in t[1:]) + 'End'
        if t == I128: return 'I128'
        return t[0]
    def nests_in_list(self, name):
        return any(self.T.res(ft)[0] == 'List' and name in self.named_in(ft) for ft in self.type_fields(name))
    def define(self, kind, t, params, result, body, rec_arg, grouped=True, fold=None):
        """a derived helper. One over a recursive type recurses on fuel, like the translated functions: LexLean's
        structural check does not follow a recursive value through a nested Option. Callers get `name @FUEL@`."""
        t = self.T.deep(t); key = (kind, t)
        if key in self.derived_names: return self.derived_names[key]
        elem = t[1] if t[0] in ('List', 'Iter', 'Set') else t
        owner = elem[1] if elem[0] == 'Named' else None
        if grouped and not (owner and self.type_rec.get(owner)):
            # a helper over a type that merely contains a recursive type (a list of pairs holding one) joins that type's group
            owner = next((n for n in self.named_in(t) if self.type_rec.get(n)), owner)
        fueled = bool(grouped and owner and self.type_rec.get(owner))
        name = kind + self.mangle(t)
        call = name + ' @FUEL@' if fueled else name
        if name in self.derived_names.values() or call in self.derived_names.values():
            self.derived_names[key] = call; return call       # the same helper, requested before its type was resolved
        self.derived_names[key] = call
        attrs = ''
        if grouped and owner and owner in self.type_group: attrs += f',\n  mutual: {kind.capitalize()}{self.type_group[owner]}'
        elif fueled and (self.nests_in_list(owner) or t[0] in ('List', 'Iter', 'Set')): attrs += f',\n  mutual: {kind.capitalize()}{owner}'
        mark, names_before = len(self.derived), set(self.derived_names)
        if fold is not None and not fueled: body, rec_arg = (lambda _name: fold()), None
        try: made = body(call)
        except Exception:
            # a helper that cannot be made leaves nothing behind: neither its name nor helpers made for it
            del self.derived[mark:]
            for k2 in list(self.derived_names):
                if k2 not in names_before or k2 == key: del self.derived_names[k2]
            raise
        if fueled:
            stop = {'Bool': 'false', 'Bytes': text('')}.get(result)
            if stop is None and result.startswith('(List '): stop = f'(nil {result[6:-1]})'
            if stop is None:
                del self.derived_names[key]; raise Unsupported(f'derived {kind} on a recursive type')
            made = f'(match fuel | Nat.zero => {stop} | Nat.succ fuelp => {made.replace("@FUEL@", "fuelp")})'
            params = 'fuel : Nat, ' + params
            attrs += ',\n  recursive_argument: fuel'
        elif t[0] in ('List', 'Iter', 'Set') and rec_arg: attrs += f',\n  recursive_argument: {rec_arg}'
        text_ = 'definition{\n  name: %s,\n  parameters: [%s],\n  result: %s,\n  body: %s,\n  executable: true%s\n}' % (name, params, result, made, attrs)
        self.derived.append(text_)
        return call
    def debug(self, v, t):
        T, t = self.T, self.T.nat(self.T.res(t))
        if t == NAT: return f'(AxiomOutput::decimal {v})'
        if t == BOOL: return f'(if {v} then {text("true")} else {text("false")})'
        if t == UNIT: return text('()')
        if t == BYTES: return f'(RustStd::debugStr {v})'
        if t[0] == 'Prod': return cat([('s', '('), self.debug(f'(first {v})', t[1]), ('s', ', '), self.debug(f'(second {v})', t[2]), ('s', ')')])
        if t[0] == 'Tup':
            parts = [('s', '(')]
            for i in range(len(t) - 1): parts += ([('s', ', ')] if i else []) + [self.debug(self.tup_get(v, t, i), t[1 + i])]
            return cat(parts + [('s', ')')])
        if t[0] == 'Option':
            n = self.fresh('shown')
            return f'(match {v} | Option.some {n} => {cat([("s", "Some("), self.debug(n, t[1]), ("s", ")")])} | Option.none => {text("None")})'
        if t[0] in ('List', 'Iter'):
            el = t[1]
            def items(name):
                one = self.debug('head', el)
                return f'(match items | List.nil => {text("")} | List.cons head tail => (match tail | List.nil => {one} | List.cons next more => {cat([one, ("s", ", "), f"({name} tail)"])}))'
            name = self.define('debug', t, f'items : {T.show(t)}', 'Bytes', items, 'items')
            return cat([('s', '['), f'({name} {v})', ('s', ']')])
        if t[0] == 'Named':
            n = t[1]
            def body(name):
                if not self.as_inductive(n):
                    parts = [('s', n + ' { ')]
                    for i, (f, ft) in enumerate(self.field_types(n)):
                        parts += [('s', (', ' if i else '') + f + ': '), self.debug(f'(project value {f})', ft)]
                    return cat(parts + [('s', ' }')])
                arms = []
                for c, rust, fs in self.ctors_of(t):
                    bs = [f'd{i}' for i in range(len(fs))]
                    named = fs and not isinstance(fs[0][0], int)
                    parts = [('s', rust + (' { ' if named else '(' if fs else ''))]
                    for i, ((f, ft), b) in enumerate(zip(fs, bs)):
                        parts += [('s', (', ' if i else '') + (f + ': ' if named else '')), self.debug(b, ft)]
                    parts += [('s', ' }' if named else ')' if fs else '')]
                    arms.append(f'{c}{"".join(" " + b for b in bs)} => {cat(parts)}')
                return f'(match value | {" | ".join(arms)})'
            return f'({self.define("debug", t, f"value : {n}", "Bytes", body, "value")} {v})'
        raise Unsupported(f'Debug of {T.show(t)}')
    def equal(self, a, b, t):
        T, t = self.T, self.T.nat(self.T.res(t))
        if t == NAT: return f'({a} =? {b})'
        if t in (BYTES, UINT8): return f'(prim equal {a} {b} : Bool)'
        if t == I128: return f'(RustStd::i128Eq {a} {b})'
        if t == BOOL: return f'(if {a} then {b} else (not {b}))'
        if t == UNIT: return 'true'
        if t[0] == 'Prod':
            return f'(if {self.equal(f"(first {a})", f"(first {b})", t[1])} then {self.equal(f"(second {a})", f"(second {b})", t[2])} else false)'
        if t[0] == 'Tup':
            test = 'true'
            for i in reversed(range(len(t) - 1)):
                e = self.equal(self.tup_get(a, t, i), self.tup_get(b, t, i), t[1 + i]); test = e if test == 'true' else f'(if {e} then {test} else false)'
            return test
        if t[0] in ('List', 'Iter'):
            el = t[1]
            def items(name):
                return (f'(match left | List.nil => (match right | List.nil => true | List.cons rhead rtail => false) | List.cons lhead ltail => '
                        f'(match right | List.nil => false | List.cons rhead rtail => (if {self.equal("lhead", "rhead", el)} then ({name} ltail rtail) else false)))')
            def fold():
                st = f'(Bool * (List {T.show(el)}))'
                step = (f'(fun (acc : {st}) (lhead : {T.show(el)}) => (if (first acc) then (match (second acc) | List.nil => (false, (nil {T.show(el)})) | '
                        f'List.cons rhead rtail => (if {self.equal("lhead", "rhead", el)} then (true, rtail) else (false, rtail))) else acc))')
                return (f'(let walked : {st} := (prim list_fold {step} (true, right) left : {st}); '
                        f'(if (first walked) then (match (second walked) | List.nil => true | List.cons rhead rtail => false) else false))')
            name = self.define('eq', t, f'left : {T.show(t)}, right : {T.show(t)}', 'Bool', items, 'left', fold=fold)
            return f'({name} {a} {b})'
        if t[0] == 'Named' and self.as_inductive(t[1]):
            def body(name):
                ctors = self.ctors_of(t); arms = []
                for c, _, fs in ctors:
                    ls = [f'l{i}' for i in range(len(fs))]; inner = []
                    for c2, _, fs2 in ctors:
                        rs = [f'r{i}' for i in range(len(fs2))]
                        if c2 == c:
                            test = 'true'
                            for (_, ft), l, r in reversed(list(zip(fs, ls, rs))):
                                e = self.equal(l, r, ft); test = e if test == 'true' else f'(if {e} then {test} else false)'
                        else: test = 'false'
                        inner.append(f'{c2}{"".join(" " + r for r in rs)} => {test}')
                    arms.append(f'{c}{"".join(" " + l for l in ls)} => (match right | {" | ".join(inner)})')
                return f'(match left | {" | ".join(arms)})'
            return f'({self.define("eq", t, f"left : {t[1]}, right : {t[1]}", "Bool", body, "left")} {a} {b})'
        if t[0] == 'Named':
            fts = self.field_types(t[1]); test = 'true'
            for f, ft in reversed(fts):
                e = self.equal(f'(project {a} {f})', f'(project {b} {f})', ft); test = e if test == 'true' else f'(if {e} then {test} else false)'
            return test
        raise Unsupported(f'== on {T.show(t)}')
    def member(self, s, x, el):
        T = self.T
        def body(name):
            return f'(match items | List.nil => false | List.cons head tail => (if {self.equal("head", "item", el)} then true else ({name} tail item)))'
        def fold():
            return f'(prim list_fold (fun [item] (acc : Bool) (head : {T.show(el)}) => (if acc then true else {self.equal("head", "item", el)})) false items : Bool)'
        name = self.define('mem', ('List', el), f'items : (List {T.show(el)}), item : {T.show(el)}', 'Bool', body, 'items', grouped=False, fold=fold)
        return f'({name} {s} {x})'
    def dedup(self, v, el):
        T = self.T
        A, X = self.fresh('acc'), self.fresh('x')
        return f'(prim list_fold (fun ({A} : (List {T.show(el)})) ({X} : {T.show(el)}) => (if {self.member(A, X, el)} then {A} else (cons {X} {A}))) (nil {T.show(el)}) {v} : (List {T.show(el)}))'

    # -- names
    def fresh(self, base):
        base = re.sub(r'\W', '', str(base)).lstrip('_') or 'unused'
        if base[0].isdigit(): base = 'f' + base
        while True:
            self.used[base] = self.used.get(base, 0) + 1
            n = self.used[base]
            name = base if n == 1 else f'{base}{n}'
            if name not in KEYWORDS and name not in self.reserved: return name
    def atom(self, v, t, k, env, base='tmp'):
        if is_atom(v): return k(v, env)
        if self.T.res(t)[0] == 'Fn': raise Unsupported('a function value bound to a name (LexLean refuses a let-bound function)')
        n = self.fresh(base)
        return f'(let {n} : {self.T.show(t)} := {v}; {k(n, env)})'
    def lit(self, n):
        t = self.T.var(); return f'\u00a6{t[1]}|{n}\u00a6', t

    # -- effect analysis
    def walk(self, e):
        if isinstance(e, tuple):
            yield e
            for x in e:
                if isinstance(x, (tuple, list)): yield from self.walk(x)
        elif isinstance(e, list):
            for x in e: yield from self.walk(x)
    def mut_positions(self, name):
        key = self.by_name.get(name)
        if key is None: return []
        return [i for i, (_, t) in enumerate(self.fns[key]['params']) if t[0] == 'mut']
    def mutated_args(self, n):
        """the variables a call node passes to `&mut` parameters."""
        if n[0] == 'call' and n[1][0] == 'path' and len(n[1][1]) == 1: asts = n[2]; pos = self.mut_positions(n[1][1][0])
        elif n[0] == 'mcall': asts = [n[1]] + n[3]; pos = self.mut_positions(n[2])
        else: return []
        out = []
        for i in pos:
            if i < len(asts):
                r = self.root_var(asts[i])
                if r: out.append(r)
        return out
    def root_var(self, a):
        while a and (a[0] in ('mutref', 'deref', 'field', 'index') or (a[0] == 'mcall' and a[2] in ('borrow_mut', 'get_mut', 'as_mut', 'borrow') and not a[3])): a = a[1]
        return a[1][0] if a and a[0] == 'path' and len(a[1]) == 1 else None
    def effectful(self, e):
        for n in self.walk(e):
            if not n or not isinstance(n[0], str): continue
            if n[0] in ('try', 'return', 'continue', 'break', 'assign', 'while', 'whilelet', 'for', 'loop'): return True
            if n[0] == 'mcall' and n[2] in MUTATORS: return True
            if n[0] == 'let' and len(n) == 5 and n[4] is not None: return True
            if n[0] in ('call', 'mcall') and self.mutated_args(n): return True
            if n[0] == 'call' and n[1][0] == 'path' and len(n[1][1]) == 1 and n[1][1][0] in self.eff_closures: return True
        return False
    def diverges(self, blk):
        _, stmts, tail = blk
        last = tail if tail is not None else (stmts[-1][1] if stmts and stmts[-1][0] == 'expr' else None)
        return last is not None and last[0] in ('return', 'continue')
    def assigned(self, body, env):
        out = []
        for n in self.walk(body):
            targets = []
            if n and n[0] == 'mcall' and n[2] in MUTATORS and self.root_var(n[1]): targets.append(self.root_var(n[1]))
            if n and n[0] == 'assign' and self.root_var(n[1]): targets.append(self.root_var(n[1]))
            if n and n[0] in ('call', 'mcall'): targets += self.mutated_args(n)
            for t in targets:
                if t in env and env[t][0] is not None and t not in out: out.append(t)
        if out:
            # A scope's writes are carried out of it by name. A name it both writes and binds again (`let mut v`
            # inside, an `if let Some(v) = v`) would carry the inner variable's value out as the outer's, so such a
            # function is refused, not translated to a different answer.
            rebound = {n[1] for n in self.walk(body) if n and n[0] in ('pid', 'pat_at') and isinstance(n[1], str)}
            clash = [t for t in out if t in rebound]
            if clash: raise Unsupported(f'a written variable shadowed in a nested scope')
        return sorted(out)

    # -- state packing
    def pack_type(self, vs, env):
        if not vs: return UNIT
        ts = [env[v][1] for v in vs]; t = ts[-1]
        for x in reversed(ts[:-1]): t = ('Prod', x, t)
        return t
    def pack(self, vs, env):
        if not vs: return '()'
        t = env[vs[-1]][0]
        for v in reversed(vs[:-1]): t = f'({env[v][0]}, {t})'
        return t
    def unpack(self, s, vs, env, body):
        env = dict(env); lets = []; cur = s
        for i, v in enumerate(vs):
            n = self.fresh('this' if v == 'self' else v); t = env[v][1]; last = i == len(vs) - 1
            lets.append(f'(let {n} : {self.T.show(t)} := {cur if last else f"(first {cur})"}; ')
            env[v] = (n, t)
            if not last:
                r = self.fresh('rest_state'); rt = self.pack_type(vs[i + 1:], env)
                lets.append(f'(let {r} : {self.T.show(rt)} := (second {cur}); '); cur = r
        return ''.join(lets) + body(env) + ')' * len(lets)
    def free(self, body, env, exclude=()):
        bare = re.sub(r'"(?:[^"\\]|\\.)*"', '""', body)
        bare = re.sub(r'\(project (\S+) \w+\)', r'(project \1)', bare)       # a field name is not a variable
        bare = re.sub(r'([{,]\s*)\w+ :=', r'\1:=', bare)
        return sorted({n for n, t in env.values() if n is not None and is_atom(n) and n not in exclude and re.search(r'(?<![\w:.])' + re.escape(n) + r'(?![\w:])', bare)})
    def captures(self, body, env, params):
        names = self.free(body, env, params)
        return f'[{", ".join(names)}] ' if names else ''

    # -- pattern matching
    def alts(self, p):
        if p[0] == 'por': return [a for q in p[1] for a in self.alts(q)]
        if p[0] == 'pat_at': return [('pat_at', p[1], a) for a in self.alts(p[2])]
        if p[0] == 'ptuple': return [('ptuple', list(c)) for c in itertools.product(*[self.alts(q) for q in p[1]])]
        if p[0] == 'pctor':
            keys = [k for k, _ in p[2]]
            return [('pctor', p[1], list(zip(keys, c))) for c in itertools.product(*[self.alts(q) for _, q in p[2]])]
        return [p]
    def peel(self, p, occ):
        binds = []
        while p[0] in ('pid', 'pat_at'):
            binds.append((p[1], occ[0], occ[1]))
            p = WILD if p[0] == 'pid' else p[2]
        return p, binds
    def scrutinee_type(self, p, t):
        t = self.T.res(t)
        if t[0] != 'Var': return
        if p[0] == 'ptuple': self.T.unify(t, ('Prod', self.T.var(), self.T.var()))
        if p[0] == 'pctor':
            last = p[1][-1]
            if len(p[1]) > 1: self.T.unify(t, ('Named', self.owner if p[1][0] == 'Self' else p[1][0]))
            elif last in ('Some', 'None'): self.T.unify(t, ('Option', self.T.var()))
            elif last in ('Ok', 'Err'): self.T.unify(t, ('Result', self.T.var(), self.T.var()))
    def lit_rows(self, occs, rows, env):
        """literal, range and constant patterns: a wildcard plus a condition on the row."""
        out = []
        for ps, g, bs, lf in rows:
            conds = []; new = []
            for p, (o, t) in zip(ps, occs):
                q, more = self.peel(p, (o, t))
                if q[0] == 'pconst':
                    rt = self.T.res(t)
                    if rt[0] == 'Named' and rt[1] in self.enums and any(v == q[1][-1] for v, _ in self.enums[rt[1]]['variants']):
                        new.append(('pctor', q[1], [])); continue
                    v, vt = self.pure(('path', q[1]), env); self.T.unify(vt, t, 'constant pattern')
                    conds.append(self.equal(o, v, t)); new.append(self.wrap_binds(p)); continue
                if q[0] == 'plit' and q[1][0] == 'byte' and self.T.res(t) == NAT:
                    # a byte pattern against a byte read as a number (`s.as_bytes()[i]`): its value
                    conds.append(self.equal(o, str(q[1][1]), NAT)); new.append(self.wrap_binds(p)); continue
                if q[0] == 'plit' and q[1][0] != 'bool':
                    v, vt = self.pure(q[1], env); self.T.unify(vt, t, 'literal pattern')
                    conds.append(self.equal(o, v, t)); new.append(self.wrap_binds(p)); continue
                if q[0] == 'prange' and q[1][0] == 'byte' and q[2][0] == 'byte' and self.T.res(t) == NAT:
                    conds.append(self.within(str(q[1][1]), str(q[2][1]), o, NAT, q[3])); new.append(self.wrap_binds(p)); continue
                if q[0] == 'prange':
                    lo, lt = self.pure(q[1], env); hi, ht = self.pure(q[2], env); self.T.unify(lt, t, 'range pattern'); self.T.unify(ht, t, 'range pattern')
                    conds.append(self.within(lo, hi, o, self.T.nat(t), q[3])); new.append(self.wrap_binds(p)); continue
                if q[0] in ('pslice', 'prest'): raise Unsupported('slice pattern')
                new.append(p)
            if conds: g = ('__conds', conds, g)
            out.append((new, g, bs, lf))
        return out
    def wrap_binds(self, p):
        """keep the `name @` bindings of a pattern whose test became a condition."""
        if p[0] == 'pat_at': return ('pat_at', p[1], self.wrap_binds(p[2]))
        if p[0] == 'pid': return p
        return WILD
    def compile(self, occs, rows, env):
        T = self.T
        if not rows: raise Unsupported('a match is not exhaustive after translation')
        rows = self.lit_rows(occs, rows, env)
        pats, guard, binds, leaf = rows[0]
        peeled = [self.peel(p, o) for p, o in zip(pats, occs)]
        col = next((i for i, (p, _) in enumerate(peeled) if p != WILD), None)
        if col is None:
            env2 = dict(env)
            for name, v, t in binds + [b for _, bs in peeled for b in bs]: env2[name] = (v, t)
            if guard is None: return leaf(env2)
            conds = []
            while guard is not None and guard[0] == '__conds': conds += guard[1]; guard = guard[2]
            if guard is not None: conds.append(self.pure(guard, env2)[0])
            test = conds[-1]
            for c in reversed(conds[:-1]): test = f'(if {c} then {test} else false)'
            return f'(if {test} then {leaf(env2)} else {self.compile(occs, rows[1:], env)})'
        o, t = occs[col]; self.scrutinee_type(peeled[col][0], t); t = T.res(t)
        def spread(arity, select):
            out = []
            for ps, g, bs, lf in rows:
                p, more = self.peel(ps[col], occs[col])
                sub = [WILD] * arity if p == WILD else select(p)
                if sub is not None: out.append((ps[:col] + sub + ps[col + 1:], g, bs + more, lf))
            return out
        if t[0] == 'Prod':
            a, b = self.fresh('left'), self.fresh('right')
            rest = self.compile(occs[:col] + [(a, t[1]), (b, t[2])] + occs[col + 1:], spread(2, lambda p: list(p[1]) if p[0] == 'ptuple' else None), env)
            return f'(let {a} : {T.show(t[1])} := (first {o}); (let {b} : {T.show(t[2])} := (second {o}); {rest}))'
        if t == BOOL:
            side = lambda val: self.compile(occs[:col] + occs[col + 1:], spread(0, lambda p: [] if p == ('plit', ('bool', val)) else None), env)
            return f'(if {o} then {side("true")} else {side("false")})'
        if t[0] == 'Tup':
            n = len(t) - 1; names = [self.fresh('part') for _ in range(n)]
            rest = self.compile(occs[:col] + [(nm, t[1 + i]) for i, nm in enumerate(names)] + occs[col + 1:],
                                spread(n, lambda p: list(p[1]) if p[0] == 'ptuple' and len(p[1]) == n else None), env)
            lets = ''.join(f'(let {nm} : {T.show(t[1 + i])} := {self.tup_get(o, t, i)}; ' for i, nm in enumerate(names))
            return lets + rest + ')' * n
        if t[0] == 'Named' and t[1] in self.structs and not self.as_inductive(t[1]):
            fts = self.field_types(t[1]); names = [self.fresh(f) for f, _ in fts]
            def select(p):
                if p[0] != 'pctor': raise Unsupported(f'pattern {p[0]} on {t[1]}')
                given = {(fts[k][0] if isinstance(k, int) else self.fname(k)): q for k, q in p[2]}
                return [given.get(f, WILD) for f, _ in fts]
            rest = self.compile(occs[:col] + [(nm, ft) for nm, (_, ft) in zip(names, fts)] + occs[col + 1:], spread(len(fts), select), env)
            lets = ''.join(f'(let {nm} : {T.show(ft)} := (project {o} {f}); ' for nm, (f, ft) in zip(names, fts))
            return lets + rest + ')' * len(fts)
        arms = []
        for lexc, rust, fields in self.ctors_of(t):
            names = [self.fresh(k if isinstance(k, str) else 'got') for k, _ in fields]
            def select(p, rust=rust, fields=fields):
                if p[0] != 'pctor': raise Unsupported(f'pattern {p[0]} on {T.show(t)}')
                if p[1][-1] != rust and not (p[1][-1] in ('Self',) or (len(p[1]) == 1 and p[1][0] == t[1])): return None
                keys = [k for k, _ in fields]
                given = {(keys[k] if isinstance(k, int) and k < len(keys) and not isinstance(keys[k], int) else (self.fname(k) if t[0] == 'Named' and t[1] in self.structs else k)): q for k, q in p[2]}
                return [given.get(k, WILD) for k, _ in fields]
            sub = spread(len(fields), select)
            if not sub: raise Unsupported(f'no arm covers {lexc}')
            body = self.compile(occs[:col] + [(n, ft) for n, (_, ft) in zip(names, fields)] + occs[col + 1:], sub, env)
            arms.append(f'{lexc}{"".join(" " + n for n in names)} => {body}')
        return f'(match {o} | {" | ".join(arms)})'
    def match(self, scrut, arms, env, ctx, k):
        T = self.T
        parts = scrut[1] if scrut[0] == 'tuple' and all(a[0][0] in ('ptuple', 'pwild') for a in arms) else None
        def run(vs, env):
            occs = []
            def have(i, env):
                if i == len(vs): return build(occs, env)
                return self.atom(vs[i][0], vs[i][1], lambda n, env: (occs.append((n, vs[i][1])), have(i + 1, env))[1], env, 'scrutinee')
            return have(0, env)
        def build(occs, env):
            asts = [a[2] for a in arms if not callable(a[2])] + [a[1] for a in arms if a[1] is not None]
            is_pure = not any(callable(a[2]) for a in arms) and not any(self.effectful(a) for a in asts)
            if not is_pure and ctx is None: raise Unsupported('effect in a pure position')
            joined = T.var()
            def k2(v, t, env): T.unify(joined, t, 'match arms'); return k(v, t, env)
            def leaf_of(body):
                if callable(body): return body
                if is_pure:
                    def leaf(env):
                        v, t = self.pure_block(body, env) if body[0] == 'block' else self.pure(body, env)
                        T.unify(joined, t, 'match arms'); return v
                    return leaf
                return lambda env: self.block(body, env, ctx, k2) if body[0] == 'block' else self.expr(body, env, ctx, k2)
            def rows(trial):
                out = []
                for pat, guard, body in arms:
                    leaf = leaf_of(body)
                    if trial:
                        def leaf(env, inner=leaf):
                            try: return inner(env)
                            except Unsupported: return '?'
                    for alt in self.alts(pat):
                        ps = (list(alt[1]) if alt[0] == 'ptuple' else [WILD] * len(occs)) if parts is not None else [alt]
                        out.append((ps, guard, [], leaf))
                return out
            try: self.compile(occs, rows(True), env)
            except Unsupported: pass
            term = self.compile(occs, rows(False), env)
            return k(term, joined, env) if is_pure else term
        if parts is not None: return self.exprs(parts, env, ctx, run)
        return self.expr(scrut, env, ctx, lambda v, t, env: run([(v, t)], env))
    def bind(self, pat, v, t, env, body):
        if pat == WILD: return body(env)
        if pat[0] == 'pid':
            n = self.fresh(pat[1]); env = dict(env); env[pat[1]] = (n, t)
            return f'(let {n} : {self.T.show(t)} := {v}; {body(env)})'
        return self.atom(v, t, lambda n, env: self.compile([(n, t)], [([a], None, [], body) for a in self.alts(pat)], env), env, 'pair')

    # -- blocks and statements
    def block(self, blk, env, ctx, k):
        _, stmts, tail = blk
        def go(i, env):
            if i == len(stmts):
                if tail is None: return k('()', UNIT, env)
                return self.expr(tail, env, ctx, k)
            s = stmts[i]; rest = lambda env: go(i + 1, env)
            if s[0] == 'item': raise Unsupported('a function nested in a function')
            if s[0] == 'let':
                _, pat, ty, init, els = s
                if els is None and any(n and n[0] == 'pctor' and n[1] in (['__Cons'], ['__Nil']) for n in self.walk(pat)):
                    # `let [a, b] = array`: Rust knows the length from the array's type, so the other case never comes
                    els = ('block', [], ('return', ('macro', 'unreachable', [])))
                if init is None:                              # `let x;`: assigned before any use, so a placeholder is unobservable
                    if pat[0] != 'pid': raise Unsupported('a deferred let of a pattern')
                    init = ('__deferred',)
                if init[0] == 'closure' and pat[0] == 'pid':
                    env2 = dict(env); env2[pat[1]] = (None, ('Closure', init, env))
                    if self.effectful(init[2]): self.eff_closures.add(pat[1])
                    return rest(env2)
                def bound(v, t, env):
                    if ty is not None:
                        want = self.ty(ty, self.owner)
                        if want[0] == 'Set' and self.T.res(t)[0] in ('List', 'Iter'):
                            self.T.unify(want[1], self.T.res(t)[1], 'collected set'); v, t = self.dedup(v, want[1]), want
                        self.T.unify(t, want, 'let annotation')
                    if els is None: return self.bind(pat, v, t, env, rest)
                    other = lambda env: self.block(els, env, ctx, self.unreachable)
                    return self.atom(v, t, lambda n, env: self.compile([(n, t)], [([a], None, [], rest) for a in self.alts(pat)] + [([WILD], None, [], other)], env), env, 'scrutinee')
                return self.expr(init, env, ctx, bound)
            e = s[1]
            if e[0] == 'if' and e[3] is None:
                if self.diverges(e[2]):
                    return self.expr(e[1], env, ctx, lambda c, t, env:
                        f'(if {c} then {self.block(e[2], env, ctx, self.unreachable)} else {rest(env)})')
                return self.expr(e[1], env, ctx, lambda c, t, env: self.when(lambda then, other: f'(if {c} then {then} else {other})',
                                                                            e[2], s, env, ctx, rest))
            if e[0] == 'iflet' and e[4] is None:
                if self.diverges(e[3]):
                    then = lambda env: self.block(e[3], env, ctx, self.unreachable)
                    return self.match(e[2], [(e[1], None, then), (WILD, None, rest)], env, ctx, None)
                def two(then_fn, other_fn):
                    return self.match(e[2], [(e[1], None, then_fn), (WILD, None, other_fn)], env, ctx, None)
                return self.when_match(two, e[3], s, env, ctx, rest)
            if e[0] in ('while', 'whilelet', 'for', 'loop'): return self.loop(e, env, ctx, rest)
            if e[0] == 'labelled': raise Unsupported('labelled loop or block')
            return self.expr(e, env, ctx, lambda v, t, env: rest(env))
        return go(0, env)
    def unreachable(self, v, t, env): raise Unsupported('a diverging block fell through')
    def exits(self, e):
        return any(n and isinstance(n[0], str) and n[0] in ('return', 'try', 'break', 'continue') for n in self.walk(e))
    def when(self, build, then_blk, stmt, env, ctx, rest):
        """an `if` statement without `else` that falls through: the variables it assigns are joined after it.
        A branch that can exit early instead carries the rest of the block in each branch."""
        if self.exits(then_blk):
            if ctx is None: raise Unsupported('an early exit in a pure position')
            return build(self.block(then_blk, env, ctx, lambda v, t, env2: rest(env2)), rest(env))
        vs = self.assigned(stmt, env)
        if not vs: return rest(env)            # no early exit and nothing assigned: the branch has no effect on what follows
        return self.joined(build, lambda e2: self.block(then_blk, e2, ctx, lambda v, t, env3: self.pack(vs, env3)), vs, env, rest)
    def joined(self, build, then_fn, vs, env, rest):
        st = self.pack_type(vs, env); n = self.fresh('joined')
        value = build(then_fn(env), self.pack(vs, env))
        return f'(let {n} : {self.T.show(st)} := {value}; {self.unpack(n, vs, env, rest)})'
    def when_match(self, two, then_blk, stmt, env, ctx, rest):
        if self.exits(then_blk):
            return two(lambda env2: self.block(then_blk, env2, ctx, lambda v, t, env3: rest(env3)), rest)
        vs = self.assigned(stmt, env)
        if not vs: return rest(env)
        st = self.pack_type(vs, env); n = self.fresh('joined')
        value = two(lambda env2: self.block(then_blk, env2, ctx, lambda v, t, env3: self.pack(vs, env3)), lambda env2: self.pack(vs, env2))
        return f'(let {n} : {self.T.show(st)} := {value}; {self.unpack(n, vs, env, rest)})'

    # -- loops: the state is the variables the loop assigns; an early exit carries its value and that state
    def loop(self, e, env, ctx, rest):
        if ctx is None: raise Unsupported('loop in a pure position')
        body = e[-1]; show = self.T.show
        vs = self.assigned(e, env); st = self.pack_type(vs, env)
        RO = ('Option', ctx.ret_t)                 # some: a `return` leaving the function; none: a `break`
        EX = ('Prod', RO, st); ST = ('Result', st, EX)
        ok = lambda env: f'(ctor Result.ok [{show(st)}, {show(EX)}] {self.pack(vs, env)})'
        inner = Ctx(ret=lambda v, env: f'(ctor Result.error [{show(st)}, {show(EX)}] ((ctor Option.some [{show(ctx.ret_t)}] {v}), {self.pack(vs, env)}))', ret_t=ctx.ret_t, fall=ok)
        inner.brk = lambda env: f'(ctor Result.error [{show(st)}, {show(EX)}] ((ctor Option.none [{show(ctx.ret_t)}]), {self.pack(vs, env)}))'
        end = lambda v, t, env: ok(env)
        state = self.fresh('state'); s = self.fresh('s'); r = self.fresh('exit')
        if e[0] == 'for':
            item_t = self.T.var(); item = self.fresh('item')
            def with_iter(it, t, env):
                if self.T.res(t)[0] == 'Set': raise Unsupported('iteration over a set (its order would matter)')
                self.T.unify(t, ('List', item_t), 'for iterator')
                def step(acc, x, env2):
                    inside = self.unpack(s, vs, env2, lambda env3: self.bind(e[1], x, item_t, env3, lambda env4: self.block(body, env4, inner, end)))
                    return f'(match {acc} | Result.ok {s} => {inside} | Result.error {r} => {acc})'
                return self.fold(it, item_t, ST, ok(env), step, env)
            folded = self.expr(e[2], env, ctx, with_iter)
        else:
            iters = [v for v in vs if self.T.res(env[v][1])[0] == 'Iter']
            if iters:
                bound = ' + '.join(f'(AxiomOutput::listLength [{show(self.T.res(env[v][1])[1])}] {env[v][0]})' for v in iters)
                bound = f'(({bound}) + 1)' if len(iters) > 1 else f'({bound} + 1)'
            else: bound = FUEL                     # a loop with no iterator to bound it runs at most u64::MAX steps
            some = lambda term: f'(ctor Option.some [{show(ST)}] {term})'
            none = f'(ctor Option.none [{show(ST)}])'
            def step(env):
                if e[0] == 'loop':
                    return some(self.block(body, env, inner, end))
                if e[0] == 'while':
                    return self.expr(e[1], env, inner, lambda c, t, env: f'(if {c} then {some(self.block(body, env, inner, end))} else {none})')
                then = lambda env: some(self.block(body, env, inner, end))
                return self.match(e[2], [(e[1], None, then), (WILD, None, lambda env: none)], env, inner, None)
            mark = len(self.call_log)
            fn_body = f'(match {state} | Result.ok {s} => {self.unpack(s, vs, env, step)} | Result.error {r} => {none})'
            if any(key in self.cur_scc for key in self.call_log[mark:]): raise Unsupported('a while loop that calls back into its recursion group')
            caps = self.captures(fn_body, env, {state})
            folded = f'(first (prim iterate_until (fun {caps}({state} : {show(ST)}) => {fn_body}) {bound} {ok(env)} : ({show(ST)} * Bool)))'
        r2 = self.fresh('exit'); s2 = self.fresh('s'); rv = self.fresh('returned')
        def after_exit(env2):
            if not any(isinstance(n, tuple) and n and n[0] in ('return', 'try') for n in self.walk(body)):
                return rest(env2)
            return f'(match (first {r2}) | Option.some {rv} => {ctx.ret(rv, env2)} | Option.none => {rest(env2)})'
        out = self.unpack(f"(second {r2})", vs, env, after_exit)
        return f'(match {folded} | Result.ok {s2} => {self.unpack(s2, vs, env, rest)} | Result.error {r2} => {out})'

    # -- expressions
    def pure(self, e, env):
        box = {}
        def k(v, t, env2): box['t'] = t; return v
        return self.expr(e, env, None, k), box['t']
    def pure_block(self, blk, env):
        box = {}
        def k(v, t, env2): box['t'] = t; return v
        return self.block(blk, env, None, k), box['t']
    def exprs(self, es, env, ctx, k, acc=None):
        acc = acc or []
        if len(acc) == len(es): return k(acc, env)
        return self.expr(es[len(acc)], env, ctx, lambda v, t, env: self.exprs(es, env, ctx, k, acc + [(v, t)]))
    def branch(self, parts, env, ctx, k, build):
        if not any(self.effectful(p) for p in parts):
            t = self.T.var()
            def one(part, env):
                v, pt = self.pure(part, env) if part[0] != 'block' else self.pure_block(part, env)
                self.T.unify(t, pt, 'branches'); return v
            return k(build(one), t, env)
        if ctx is None: raise Unsupported('effect in a pure position')
        joined = self.T.var()
        def k2(v, t, env): self.T.unify(joined, t, 'branches'); return k(v, t, env)
        def one(part, env):
            return self.block(part, env, ctx, k2) if part[0] == 'block' else self.expr(part, env, ctx, k2)
        def trial(part, env):
            try: return one(part, env)
            except Unsupported: return '?'
        build(trial)
        return build(one)
    def ctor_term(self, t, variant, vals):
        for lexc, rust, fields in self.ctors_of(t):
            if rust == variant:
                if len(vals) != len(fields): raise Unsupported(f'constructor {variant} arity')
                for (_, ft), (_, vt) in zip(fields, vals): self.T.unify(ft, vt, f'field of {variant}')
                return f'(ctor {lexc}{"".join(" " + v for v, _ in vals)})'
        raise Unsupported(f'constructor {variant}')
    def atom_expr(self, e, env, ctx, k):
        return self.expr(e, env, ctx, lambda v, t, env: self.atom(v, t, lambda n, env: k(n, t, env), env, 'base'))
    def int_path(self, segs):
        if len(segs) == 2 and segs[0] in INT_LIMITS and segs[1] in ('MAX', 'MIN'):
            value = INT_LIMITS[segs[0]][1 if segs[1] == 'MAX' else 0]
            if segs[0].startswith('i') or segs[0] == 'u128': return wide(value < 0, abs(value)), I128
            return self.lit(value)
        return None

    def hook_expr(self, e, env, ctx, k): return None
    def callee_key(self, segs):
        segs = [self.owner if x == 'Self' else x for x in segs]
        if len(segs) == 1 and (None, segs[0]) in self.fns: return (None, segs[0])
        if len(segs) == 2 and (segs[0], segs[1]) in self.fns: return (segs[0], segs[1])
        return None
    def hint_args(self, key, asts, offset):
        """the parameter types of a known callee, for closures passed to it (their own parameters are unannotated)."""
        if key is None or self.fns[key]['generics']: return
        params = self.fns[key]['params'][offset:]
        if not hasattr(self, 'hints'): self.hints = {}
        for ast, (_, pt) in zip(asts, params):
            if ast[0] in ('closure', 'path'):
                try: self.hints[id(ast)] = self.ty(pt, key[0])
                except Unsupported: pass
    def hook_call(self, segs, vs, e, env, ctx, k): return None
    def hook_method(self, r, rt, m, args, recv, env, ctx, k): return None
    def hook_ty(self, n, a, owner): return None
    def expr(self, e, env, ctx, k):
        T, show = self.T, self.T.show
        h = e[0]
        hooked = self.hook_expr(e, env, ctx, k)
        if hooked is not None: return hooked
        if h == 'int': v, t = self.lit(e[1]); return k(v, t, env)
        if h == 'byte': return k(f'({e[1]} : UInt8)', UINT8, env)
        if h in ('str', 'chr'): return k(text(e[1]), BYTES, env)
        if h == 'bool': return k(e[1], BOOL, env)
        if h == 'mutref': return self.expr(e[1], env, ctx, k)
        if h == '__value': return k(e[1], e[2], env)
        if h == '__deferred':
            v, t = self.deferred(); return k(v, t, env)
        if h == 'path':
            segs = [s for s in e[1] if s not in ('super', 'crate', 'self')] or e[1]
            if len(segs) == 1:
                n = segs[0]
                if n in env:
                    if env[n][0] is None: raise Unsupported('a closure used as a value')
                    return k(env[n][0], env[n][1], env)
                if n == 'None': t = T.var(); return k(f'(ctor Option.none [{show(t)}])', ('Option', t), env)
                if n in self.consts:
                    c = self.consts[n]
                    return self.expr(c['value'], env, ctx, lambda v, t, env: (T.unify(t, self.ty(c['type'])), k(v, t, env))[1])
            ip = self.int_path(segs)
            if ip: return k(ip[0], ip[1], env)
            if len(segs) == 2:
                en = self.owner if segs[0] == 'Self' else segs[0]
                if en in self.enums and any(v == segs[1] and not fs for v, fs in self.enums[en]['variants']):
                    return k(f'(ctor {en}.{segs[1]})', ('Named', en), env)
            raise Unsupported(f'path {"::".join(e[1])}')
        if h == 'not': return self.expr(e[1], env, ctx, lambda v, t, env: k(f'(not {v})', BOOL, env))
        if h == 'tuple':
            if not e[1]: return k('()', UNIT, env)
            if len(e[1]) == 2: return self.exprs(e[1], env, ctx, lambda vs, env: k(f'({vs[0][0]}, {vs[1][0]})', ('Prod', vs[0][1], vs[1][1]), env))
            return self.exprs(e[1], env, ctx, lambda vs, env: k(self.tup_make([v for v, _ in vs]), ('Tup',) + tuple(t for _, t in vs), env))
        if h == 'array':
            def made(vs, env):
                el = T.var(); out = f'(nil {show(el)})'
                for v, t in reversed(vs): T.unify(el, t, 'array element'); out = f'(cons {v} {out})'
                return k(out, ('List', el), env)
            return self.exprs(e[1], env, ctx, made)
        if h == 'index':
            def at(vs, env):
                (xs, xt), (i, it) = vs; T.unify(it, NAT, 'index'); el = T.var(); T.unify(xt, ('List', el), 'indexing')
                n = self.fresh('got')
                return k(f'(match (RustStd::getAt [{show(el)}] {xs} {i}) | Option.some {n} => {n} | Option.none => {self.default(el)})', el, env)
            return self.exprs([e[1], e[2]], env, ctx, at)
        if h == 'cast':
            target = self.ty(e[2], self.owner)
            def cast(v, t, env):
                t = T.nat(t)
                if t == NAT and target == NAT: return k(v, NAT, env)
                if t == NAT and target == I128: return k(f'(RustStd::i128Of false {v})', I128, env)
                if t == I128 and target == I128: return k(v, I128, env)
                raise Unsupported(f'cast {show(t)} as {show(target)}')
            return self.expr(e[1], env, ctx, cast)
        if h == 'bin':
            op = e[1]
            if op in ('&&', '||'):
                if self.effectful(e[3]):                      # an effect on the right: the same short-circuit as an `if`
                    other = ('bool', 'false') if op == '&&' else ('bool', 'true')
                    then, els = (('block', [], e[3]), ('block', [], other)) if op == '&&' else (('block', [], other), ('block', [], e[3]))
                    return self.expr(('if', e[2], then, els), env, ctx, k)
                def sc(a, t, env):
                    b, _ = self.pure(e[3], env)
                    return k(f'(if {a} then {b} else false)' if op == '&&' else f'(if {a} then true else {b})', BOOL, env)
                return self.expr(e[2], env, ctx, sc)
            def cmp(vs, env):
                (a, ta), (b, tb) = vs; T.unify(ta, tb, f'operands of {op}'); t = T.res(ta)
                if op in ('==', '!='):
                    if is_atom(a) and is_atom(b) or T.nat(t) in (NAT, BYTES, UINT8, I128): eq = self.equal(a, b, t)
                    else: eq = self.atom(a, t, lambda x, env: self.atom(b, t, lambda y, env: self.equal(x, y, t), env), env)
                    return k(eq if op == '==' else f'(not {eq})', BOOL, env)
                if t == BOOL and op in ('|', '&', '^'):
                    # both operands are already evaluated, in order: the non-short-circuit form is a choice between values
                    form = {'|': f'(if {a} then true else {b})', '&': f'(if {a} then {b} else false)', '^': f'(if {a} then (not {b}) else {b})'}
                    return k(form[op], BOOL, env)
                t = T.nat(t)
                if t == I128:
                    form = {'<': f'(RustStd::i128Lt {a} {b})', '<=': f'(RustStd::i128Le {a} {b})', '>': f'(RustStd::i128Lt {b} {a})', '>=': f'(RustStd::i128Le {b} {a})'}
                    if op not in form: raise Unsupported(f'i128 operator {op}')
                    return k(form[op], BOOL, env)
                if t != NAT and op in ('<', '<=', '>', '>='):
                    # any type with an order the translation derives (Bytes by their bytes, tuples, derived Ord)
                    lo, hi = (a, b) if op in ('<', '<=') else (b, a)
                    if op in ('<', '>'): return k(self.less(lo, hi, t), BOOL, env)
                    return k(f'(not {self.less(hi, lo, t)})', BOOL, env)
                if t != NAT: raise Unsupported(f'{op} on {show(t)}')
                form = {'<': f'({a} <? {b})', '<=': f'({a} <=? {b})', '>': f'({b} <? {a})', '>=': f'({b} <=? {a})', '+': f'({a} + {b})',
                        '%': f'(prim remainder {a} {b} 0 : Nat)', '/': f'(prim quotient {a} {b} 0 : Nat)', '*': f'(prim multiply {a} {b} : Nat)'}
                if op not in form: raise Unsupported(f'operator {op}')
                return k(form[op], BOOL if op in ('<', '<=', '>', '>=') else NAT, env)
            return self.exprs([e[2], e[3]], env, ctx, cmp)
        if h == 'field':
            def proj(v, t, env):
                t = T.res(t)
                if isinstance(e[2], int) and t[0] == 'Tup':
                    return self.atom(v, t, lambda n, env: k(self.tup_get(n, t, e[2]), t[1 + e[2]], env), env, 'tuple')
                if isinstance(e[2], int) and not (t[0] == 'Named' and t[1] in self.structs):
                    a, b = T.var(), T.var(); T.unify(t, ('Prod', a, b), 'tuple field')
                    return k(f'({"first" if e[2] == 0 else "second"} {v})', a if e[2] == 0 else b, env)
                if t[0] != 'Named' or t[1] not in self.structs: raise Unsupported(f'field {e[2]} of {show(t)}')
                fname = self.fname(e[2]) if isinstance(e[2], str) else f'f{e[2]}'
                fts = self.field_types(t[1])
                if fname not in dict(fts): raise Unsupported(f'field {fname} of {t[1]}')
                ft = dict(fts)[fname]
                if self.as_inductive(t[1]):
                    names = [self.fresh(f) for f, _ in fts]
                    return k(f'(match {v} | {t[1]}.mk {" ".join(names)} => {names[[f for f, _ in fts].index(fname)]})', ft, env)
                return k(f'(project {v} {fname})', ft, env)
            return self.expr(e[1], env, ctx, proj)
        if h == 'struct':
            segs = [self.owner if x == 'Self' else x for x in e[1]]; given = {self.fname(f): v for f, v in e[2]}; base = e[3] if len(e) > 3 else None
            if base is not None and len(segs) == 1:
                fts = self.field_types(segs[0])
                return self.atom_expr(base, env, ctx, lambda bv, bt, env: self.expr(('struct', segs, [(f, given[f] if f in given else ('__value', f'(project {bv} {f})', ft)) for f, ft in fts]), env, ctx, k))
            if len(segs) == 2:
                en = self.owner if segs[0] == 'Self' else segs[0]; t = ('Named', en)
                keys = next(([fk for fk, _ in fs] for v, fs in self.enums[en]['variants'] if v == segs[1]), None)
                if keys is None: raise Unsupported(f'constructor {segs[1]}')
                raw = dict(e[2])                              # a variant's fields are positional: no renaming
                if any(f not in raw for f in keys): raise Unsupported('variant literal with missing fields')
                return self.exprs([raw[f] for f in keys], env, ctx, lambda vs, env: k(self.ctor_term(t, segs[1], vs), t, env))
            name = segs[0]
            if name not in self.structs:                      # a variant named without its enum (`use Enum::*`)
                owners = [en for en, it in self.enums.items() if any(v == name for v, _ in it['variants'])]
                if len(owners) != 1: raise Unsupported(f'struct literal of {name}')
                return self.expr(('struct', [owners[0], name], e[2]), env, ctx, k)
            fts = self.field_types(name)
            if any(f not in given for f, _ in fts if f != 'unit'): raise Unsupported('struct literal with missing fields')
            def made(vs, env):
                for (f, ft), (v, vt) in zip(fts, vs): T.unify(ft, vt, f'field {f}')
                if self.as_inductive(name): return k(f'(ctor {name}.mk{"".join(" " + v for v, _ in vs)})', ('Named', name), env)
                body = ', '.join(f'{f} := {v}' for (f, _), (v, _) in zip(fts, vs))
                return k(f'(record {name} {{{body}}})', ('Named', name), env)
            if fts == [('unit', UNIT)] and 'unit' not in given: given['unit'] = ('tuple', [])
            return self.exprs([given[f] for f, _ in fts], env, ctx, made)
        if h == 'macro':
            if e[1] == 'code': return k(text(e[2][0][1]), BYTES, env)
            if e[1] != 'format': raise Unsupported(f'macro {e[1]}!')
            fmt = e[2][0][1].replace('{{', '').replace('}}', '')
            if re.search(r'\{[^}]*:[^?}][^}]*\}|\{\d+\}', fmt): raise Unsupported('a format spec beyond {} and {:?}')
            pieces = [p.replace('', '{').replace('', '}') if p else p for p in re.split(r'\{(\w*)(:\?)?\}', fmt)]
            pos = list(e[2][1:]); parts = []
            if sum(1 for i in range(1, len(pieces), 3) if not pieces[i]) != len(pos): raise Unsupported('format arguments do not match the format string')
            for i in range(0, len(pieces), 3):
                if pieces[i]: parts.append((None, ('str', pieces[i])))
                if i + 1 < len(pieces):
                    parts.append((pieces[i + 2] or '', ('path', [pieces[i + 1]]) if pieces[i + 1] else pos.pop(0)))
            def joined(vs, env):
                out = []
                for (mode, ast), (v, t) in zip(parts, vs):
                    t = T.nat(t)
                    if mode is None: out.append(('s', ast[1]))
                    elif mode == ':?': out.append(self.debug_value(v, t))
                    elif t == NAT: out.append(f'(AxiomOutput::decimal {v})')
                    elif t == BYTES: out.append(v)
                    else: out.append(self.display(v, t))
                return k(cat(out), BYTES, env)
            def atoms(vs, env, acc=None):
                acc = acc or []
                if len(acc) == len(vs): return joined(acc, env)
                v, t = vs[len(acc)]
                if parts[len(acc)][0] != ':?': return atoms(vs, env, acc + [(v, t)])
                return self.atom(v, t, lambda n, env: atoms(vs, env, acc + [(n, t)]), env, 'shown')
            return self.exprs([a for _, a in parts], env, ctx, atoms)
        if h == 'block': return self.block(e, env, ctx, k)
        if h == 'if':
            if e[3] is None: raise Unsupported('if without else as a value')
            return self.expr(e[1], env, ctx, lambda c, t, env:
                self.branch([e[2], e[3]], env, ctx, k, lambda one: f'(if {c} then {one(e[2], env)} else {one(e[3], env)})'))
        if h == 'iflet' and self.slice_arm(e[1]) is not None:
            return self.expr(self.unslice(e[2], [(e[1], None, e[3]), (WILD, None, e[4] if e[4] is not None else ('block', [], None))]), env, ctx, k)
        if h == 'iflet':
            if e[4] is None: raise Unsupported('if-let without else as a value')
            return self.match(e[2], [(e[1], None, e[3]), (WILD, None, e[4])], env, ctx, k)
        if h == 'match' and not e[2][-1][0] == WILD and any(n and ((n[0] == 'pctor' and n[1] in (['__Cons'], ['__Nil'])) or n[0] == 'pslice') for a in e[2] for n in self.walk(a[0])):
            # Rust checked this match exhaustive; with list patterns the compiler here cannot always see that (a guarded
            # arm never covers), so the case it cannot place is the one that never comes
            return self.expr(('match', e[1], list(e[2]) + [(WILD, None, ('macro', 'unreachable', []))]), env, ctx, k)
        if h == 'match' and any(self.slice_arm(a[0]) is not None for a in e[2]):
            return self.expr(self.unslice(e[1], e[2]), env, ctx, k)
        if h == 'match': return self.match(e[1], e[2], env, ctx, k)
        if h == 'return':
            if ctx is None: raise Unsupported('return in a pure position')
            def out(v, t, env): T.unify(t, ctx.ret_t, 'return'); return ctx.ret(v, env)
            if e[1] is None: return out('()', UNIT, env)
            return self.expr(e[1], env, ctx, out)
        if h == 'continue':
            if len(e) > 1 and e[1]: raise Unsupported('labelled continue')
            if ctx is None or ctx.fall is None: raise Unsupported('continue outside a loop')
            return ctx.fall(env)
        if h == 'break':
            if e[1]: raise Unsupported('labelled break')
            if e[2] is not None: raise Unsupported('break with a value')
            if ctx is None or ctx.brk is None: raise Unsupported('break outside a loop')
            return ctx.brk(env)
        if h == 'deref': return self.expr(e[1], env, ctx, k)
        if h == 'opaque': raise Unsupported(e[1])
        if h in ('loop', 'while', 'whilelet', 'for'):
            if ctx is None: raise Unsupported('loop in a pure position')
            return self.loop(e, env, ctx, lambda env2: k('()', UNIT, env2))
        if h == 'labelled': raise Unsupported('labelled loop or block')
        if h == 'try':
            if ctx is None: raise Unsupported('? in a pure position')
            def on(v, t, env):
                t = T.res(t); R = T.res(ctx.ret_t); n = self.fresh('got')
                if t[0] == 'Option' and R[0] == 'Option':
                    return f'(match {v} | Option.some {n} => {k(n, t[1], env)} | Option.none => {ctx.ret(f"(ctor Option.none [{show(R[1])}])", env)})'
                if t[0] == 'Result' and R[0] in ('Result', 'Var'):
                    if R[0] == 'Var': T.unify(R, ('Result', T.var(), t[2])); R = T.res(R)
                    T.unify(t[2], R[2], '? error type'); f = self.fresh('failure')
                    return f'(match {v} | Result.ok {n} => {k(n, t[1], env)} | Result.error {f} => {ctx.ret(f"(ctor Result.error [{show(R[1])}, {show(R[2])}] {f})", env)})'
                raise Unsupported(f'? on {show(t)} in a function returning {show(R)}')
            return self.expr(e[1], env, ctx, on)
        if h == 'assign':
            lhs = e[1]
            return self.expr(e[2], env, ctx, lambda v, t, env: self.assign_into(lhs, v, t, env, lambda env2: k('()', UNIT, env2)))
        if h == 'call': return self.call(e, env, ctx, k)
        if h == 'mcall': return self.mcall(e, env, ctx, k)
        if h == 'closure': raise Unsupported('a closure used as a value')
        raise Unsupported(f'expression {h}')

    # -- closures: inlined at each call
    def inline(self, params, ptypes, body, ret, benv, vs, env, ctx, k):
        if len(params) != len(vs): raise Unsupported('closure arity')
        T = self.T
        if '__fuel' in env: benv = dict(benv); benv['__fuel'] = env['__fuel']
        def bound(i, cenv):
            if i == len(vs):
                rt = ret if ret is not None else T.var()
                if self.effectful(body):
                    if ctx is None: raise Unsupported('a closure that returns early, in a pure position')
                    local = Ctx(ret=lambda v, e2: k(v, rt, env), ret_t=rt)
                    def tail(v, t, e2): T.unify(t, rt, 'closure result'); return k(v, rt, env)
                    return self.block(body, cenv, local, tail) if body[0] == 'block' else self.expr(body, cenv, local, tail)
                v, t = self.pure_block(body, cenv) if body[0] == 'block' else self.pure(body, cenv)
                T.unify(t, rt, 'closure result')
                return k(v, rt, env)
            (v, t), p = vs[i], params[i]
            if ptypes[i] is not None: T.unify(t, ptypes[i], 'closure argument')
            if p == WILD: return bound(i + 1, cenv)
            if p[0] == 'pid':
                def named(n, _env):
                    c2 = dict(cenv); c2[p[1]] = (n, t); return bound(i + 1, c2)
                return self.atom(v, t, named, env, p[1])
            return self.bind(p, v, t, cenv, lambda c2: bound(i + 1, c2))
        return bound(0, benv)
    def closure_of(self, c, env):
        if c[0] == 'closure': return c, env
        if c[0] == 'path' and len(c[1]) == 1 and c[1][0] in env and env[c[1][0]][0] is None:
            _, cl, cenv = env[c[1][0]][1]; return cl, cenv
        raise Unsupported('a function passed by name')
    def call_closure(self, c, cenv, vs, env, ctx, k):
        ptypes = [self.ty(t, self.owner) if t is not None else None for _, t in c[1]]
        ret = self.ty(c[3], self.owner) if c[3] is not None else None
        return self.inline([p for p, _ in c[1]], ptypes, c[2], ret, cenv, vs, env, ctx, k)
    def closure_arg(self, c, env, vs):
        """a closure passed to a combinator, applied at `vs`: (term, type). It must not mutate what it captures."""
        cl, cenv = self.closure_of(c, env)
        if self.assigned(cl[2], cenv): raise Unsupported('a closure that mutates what it captures')
        box = {}
        def k(v, t, env2): box['t'] = t; return v
        return self.call_closure(cl, cenv, vs, env, None, k), box['t']

    # -- calls
    def user_fn(self, key):
        fn = self.fns[key]
        if fn['generics']: return fn
        if self.status.get(key, 'ok') != 'ok' and key not in self.cur_scc and self.fnmut_index(key) is None:
            self.blocked_by.setdefault(self.key, key)      # so the report can follow the chain to its root
            raise Unsupported(f'calls `{fn["name"]}`, which did not translate')
        self.calls.add(key); self.call_log.append(key)
        return fn
    def apply_user(self, key, vs, asts, env, k, asts_only=False, ctx=None):
        T, show = self.T, self.T.show
        if asts_only:
            return self.exprs(asts, env, ctx, lambda vs2, env2: self.apply_user(key, vs2, asts, env2, k))
        fn = self.user_fn(key); owner = key[0]
        params = fn['params']
        if len(params) != len(vs): raise Unsupported(f'arity of {fn["name"]}')
        for (v, t), (_, pt) in zip(vs, params): T.unify(t, self.ty(pt, owner), f'argument of {fn["name"]}')
        args = [v for v, _ in vs]
        if key in self.rec_keys:
            args = [env['__fuel'][0] if key in self.cur_scc and '__fuel' in env else FUEL] + args
        rt = self.ty(fn['ret'], owner)
        call = f'({fn["lex"]} {" ".join(args)})'
        pos = [i for i, (_, t) in enumerate(params) if t[0] == 'mut']
        if not pos: return k(call, rt, env)
        targets = [(asts[i], vs[i][1]) for i in pos]
        mt = [t for _, t in targets]
        packed = mt[-1]
        for x in reversed(mt[:-1]): packed = ('Prod', x, packed)
        p = self.fresh('called'); pt = ('Prod', rt, packed)
        def store(i, cur, env2):
            if i == len(targets): return k(f'(first {p})', rt, env2)
            last = i == len(targets) - 1
            part = cur if last else f'(first {cur})'
            return self.assign_into(targets[i][0], part, targets[i][1], env2, lambda env3: store(i + 1, f'(second {cur})', env3))
        return f'(let {p} : {show(pt)} := {call}; {store(0, f"(second {p})", env)})'
    def assign_into(self, lhs, v, t, env, k):
        """assign v (of type t) to an assignable expression; k(env) continues with the variables rebound."""
        T, show = self.T, self.T.show
        if lhs[0] in ('mutref', 'deref'): return self.assign_into(lhs[1], v, t, env, k)
        if lhs[0] == 'mcall' and lhs[2] in ('borrow_mut', 'get_mut', 'as_mut', 'borrow', 'deref_mut') and not lhs[3]:
            return self.assign_into(lhs[1], v, t, env, k)     # a RefCell borrow is the place itself
        if lhs[0] == 'path' and len(lhs[1]) == 1 and lhs[1][0] in env and env[lhs[1][0]][0] is not None:
            name = lhs[1][0]; old = env[name]; T.unify(old[1], t, 'assignment')
            n = self.fresh(name); env2 = dict(env); env2[name] = (n, old[1])
            return f'(let {n} : {show(old[1])} := {v}; {k(env2)})'
        if lhs[0] == 'field':
            base_v, base_t = self.pure(lhs[1], env); bt = T.res(base_t); f = lhs[2]
            if bt[0] == 'Prod' and isinstance(f, int):
                T.unify(bt[1 + f], t, 'tuple field assignment')
                new = f'({v}, (second {base_v}))' if f == 0 else f'((first {base_v}), {v})'
            elif bt[0] == 'Tup' and isinstance(f, int):
                T.unify(bt[1 + f], t, 'tuple field assignment')
                parts = [v if i == f else self.tup_get(base_v, bt, i) for i in range(len(bt) - 1)]
                new = self.tup_make(parts)
            elif bt[0] == 'Named' and bt[1] in self.structs:
                fname = self.fname(f) if isinstance(f, str) else f'f{f}'
                fts = self.field_types(bt[1]); T.unify(dict(fts)[fname], t, 'field assignment')
                if self.as_inductive(bt[1]):
                    names = [self.fresh(x) for x, _ in fts]
                    args = ' '.join(v if x == fname else n for (x, _), n in zip(fts, names))
                    new = f'(match {base_v} | {bt[1]}.mk {" ".join(names)} => (ctor {bt[1]}.mk {args}))'
                else:
                    body = ', '.join(f'{x} := {v if x == fname else f"(project {base_v} {x})"}' for x, _ in fts)
                    new = f'(record {bt[1]} {{{body}}})'
            else: raise Unsupported(f'assignment to a field of {show(bt)}')
            return self.assign_into(lhs[1], new, bt, env, k)
        if lhs[0] == 'index':
            xs_v, xs_t = self.pure(lhs[1], env); i_v, i_t = self.pure(lhs[2], env); T.unify(i_t, NAT, 'index')
            el = T.var(); T.unify(xs_t, ('List', el), 'indexed assignment'); T.unify(el, t, 'indexed assignment')
            return self.assign_into(lhs[1], f'(RustStd::setAt [{show(el)}] {xs_v} {i_v} {v})', xs_t, env, k)
        # a temporary (a literal, a constant): Rust drops its final value after the call, so the write-back is dropped too
        if lhs[0] in ('struct', 'str', 'bstr', 'array', 'arrayrep', 'macro', 'tuple', 'int', 'bool', 'chr', 'byte'): return k(env)
        if lhs[0] == 'path' and lhs[1] and lhs[1][0] not in env and (lhs[1][0] in self.consts or lhs[1][0].isupper()): return k(env)
        what = lhs[0] + (f' .{lhs[2]}()' if lhs[0] == 'mcall' else f' {lhs[1][0]}' if lhs[0] == 'path' and lhs[1] and lhs[1][0] not in env else '')
        raise Unsupported(f'assignment target ({what})')
    def tup_make(self, parts):
        out = parts[-1]
        for x in reversed(parts[:-1]): out = f'({x}, {out})'
        return out
    def tup_get(self, v, tt, i):
        n = len(tt) - 1
        cur = v
        for _ in range(i): cur = f'(second {cur})'
        return cur if i == n - 1 else f'(first {cur})'

    def fnmut_index(self, key):
        """the position of a function's one `FnMut` parameter, or None."""
        fn = self.fns.get(key)
        if fn is None or fn['generics'] or fn.get('spec_of'): return None
        found = []
        for i, (_, t) in enumerate(fn['params']):
            while t[0] in ('mut', 'ref'): t = t[1]
            if t[0] == 'fnty' and t[3] == 'FnMut': found.append(i)
        return found[0] if len(found) == 1 else None
    def closure_ast(self, a, env):
        """the closure an argument is, with the environment it captures from, or None."""
        while a[0] in ('mutref', 'ref'): a = a[1]
        if a[0] == 'closure': return a, env
        if a[0] == 'path' and len(a[1]) == 1 and a[1][0] in env and env[a[1][0]][0] is None and env[a[1][0]][1][0] == 'Closure':
            _, cl, cenv = env[a[1][0]][1]; return cl, cenv
        return None
    def captures_of(self, cl, cenv):
        """the variables a closure reads from where it was made, those it mutates first."""
        own = set()
        for p, _ in cl[1]:
            for n in self.walk(p):
                if n and n[0] == 'pid': own.add(n[1])
        mutated = [c for c in self.assigned(cl[2], cenv) if c not in own]
        read = []
        for n in self.walk(cl[2]):
            if n and n[0] == 'path' and len(n[1]) == 1:
                c = n[1][0]
                if c in cenv and cenv[c][0] is not None and c not in own and c not in mutated and c not in read: read.append(c)
        return mutated, read
    def rename_paths(self, e, names):
        if isinstance(e, tuple):
            if len(e) == 2 and e[0] == 'path' and isinstance(e[1], list) and len(e[1]) == 1 and e[1][0] in names: return ('path', [names[e[1][0]]])
            return tuple(self.rename_paths(x, names) for x in e)
        if isinstance(e, list): return [self.rename_paths(x, names) for x in e]
        return e
    def spec_body(self, e, vname, gkey, spec_name, cl, cap_args):
        """g's body with its FnMut parameter `vname` replaced by the closure `cl` (captures already renamed)."""
        rec = lambda x: self.spec_body(x, vname, gkey, spec_name, cl, cap_args)
        is_v = lambda a: (lambda b: b[0] == 'path' and b[1] == [vname])(self.strip_refs(a))
        if isinstance(e, list): return [rec(x) for x in e]
        if not isinstance(e, tuple): return e
        if e and e[0] == 'call' and e[1][0] == 'path' and e[1][1] == [vname]:
            args = [rec(a) for a in e[2]]
            if len(args) != len(cl[1]): raise Unsupported('a closure called with another number of arguments')
            return ('block', [('let', p, ann, a, None) for (p, ann), a in zip(cl[1], args)], cl[2])
        if e and e[0] == 'call' and e[1][0] == 'path' and any(is_v(a) for a in e[2]):
            segs = [s for s in e[1][1] if s not in ('super', 'crate')]
            if self.callee_key(segs) == gkey:
                return ('call', ('path', [spec_name]), [rec(a) for a in e[2] if not is_v(a)] + cap_args)
            return ('call', e[1], [cl if is_v(a) else rec(a) for a in e[2]])
        if e and e[0] == 'path' and e[1] == [vname]: raise Unsupported('an FnMut parameter used other than by a call or passed on')
        return tuple(rec(x) for x in e)
    def strip_refs(self, a):
        while isinstance(a, tuple) and a and a[0] in ('mutref', 'ref'): a = a[1]
        return a
    def specialize_call(self, gkey, cl, cenv, e, env, ctx, k):
        T = self.T
        g = self.fns[gkey]; idx = self.fnmut_index(gkey)
        if gkey in self.rec_keys and len(self.scc.get(gkey, [gkey])) > 1: raise Unsupported('an FnMut parameter in mutual recursion')
        if any(n and n[0] == 'return' for n in self.walk(cl[2])): raise Unsupported('a closure that returns, passed as FnMut')
        mutated, read = self.captures_of(cl, cenv)
        if any(c not in env or env[c][0] is None for c in mutated + read): raise Unsupported('a closure capturing what the call cannot pass')
        # one copy per (callee, closure, capture types): the same closure passed at several call sites shares it
        ckey = (gkey, repr(cl), tuple((c, repr(T.deep(cenv[c][1]))) for c in mutated + read))
        if not hasattr(self, 'spec_cache'): self.spec_cache = {}
        if ckey in self.spec_cache:
            skey, cached_texts, cached_calls = self.spec_cache[ckey]
            self.spec_texts += cached_texts
            self.calls |= cached_calls; self.call_log += sorted(cached_calls, key=str)
            args = [a for i, a in enumerate(e[2]) if i != idx] + [('mutref', ('path', [c])) if c in mutated else ('path', [c]) for c in mutated + read]
            return self.apply_user(skey, None, args, env, k, asts_only=True, ctx=ctx)
        self.spec_n += 1
        spec_name = f"{g['name']}__k{self.spec_n}"
        names = {c: f'kc{self.spec_n}_{c}' for c in mutated + read}
        cl2 = ('closure', cl[1], self.rename_paths(cl[2], names), cl[3])
        cap_args = [('mutref', ('path', [names[c]])) if c in mutated else ('path', [names[c]]) for c in mutated + read]
        vname = g['params'][idx][0]
        params = [p for i, p in enumerate(g['params']) if i != idx]
        params += [(names[c], ('mut', ('lexty', cenv[c][1])) if c in mutated else ('lexty', cenv[c][1])) for c in mutated + read]
        spec = dict(g); spec.update(name=spec_name, lex=f"{g['lex']}K{self.spec_n}", params=params, spec_of=gkey,
                                    body=self.spec_body(g['body'], vname, gkey, spec_name, cl2, cap_args))
        skey = (None, spec_name); self.fns[skey] = spec; self.spec_of[skey] = gkey
        self.by_name[spec_name] = skey      # so that a call of the copy is seen to write its `&mut` arguments
        # recursive when the copy calls itself: decided from its own body, since `g` alone never translated and the
        # call graph of the first pass does not have it
        if any(n and n[0] == 'call' and n[1][0] == 'path' and n[1][1] == [spec_name] for n in self.walk(spec['body'])):
            self.rec_keys.add(skey); self.scc[skey] = [skey]
        saved = {a: getattr(self, a) for a in ('used', 'reserved', 'owner', 'key', 'calls', 'call_log', 'helpers', 'helper_n', 'cur_scc', 'spec_texts')}
        saved = {a: (set(v) if isinstance(v, set) else list(v) if isinstance(v, list) else dict(v) if isinstance(v, dict) else v) for a, v in saved.items()}
        try:
            self.cur_scc = {skey}; self.spec_texts = []
            text_ = self.function(spec)
            inner = self.spec_texts
            self.status[skey] = 'ok'
        except Unsupported as x:
            for a, v in saved.items(): setattr(self, a, v)
            self.status[skey] = f'UNSUPPORTED: {x}'
            raise Unsupported(f"`{g['name']}` with this closure: {x}")
        spec_calls = set(self.calls) - {skey}
        for a, v in saved.items(): setattr(self, a, v)
        self.calls |= spec_calls; self.call_log += sorted(spec_calls, key=str)   # the copy's calls are the caller's too
        self.spec_texts += inner + [text_]
        self.spec_cache[ckey] = (skey, inner + [text_], set(spec_calls))
        outer = [c for c in mutated + read]
        args = [a for i, a in enumerate(e[2]) if i != idx] + [('mutref', ('path', [c])) if c in mutated else ('path', [c]) for c in outer]
        return self.apply_user(skey, None, args, env, k, asts_only=True, ctx=ctx)
    def call(self, e, env, ctx, k):
        T, show = self.T, self.T.show
        f = e[1]
        if f[0] != 'path': raise Unsupported('call of a value')
        segs = [s for s in f[1] if s not in ('super', 'crate')]; name = '::'.join(segs)
        gkey = self.callee_key(segs)
        if gkey is not None and gkey[0] is None and self.fnmut_index(gkey) is not None and self.fnmut_index(gkey) < len(e[2]):
            found = self.closure_ast(e[2][self.fnmut_index(gkey)], env)
            if found is not None: return self.specialize_call(gkey, found[0], found[1], e, env, ctx, k)
        self.hint_args(self.callee_key(segs), e[2], 0)
        if name == 'Vec::new': t = T.var(); return k(f'(nil {show(t)})', ('List', t), env)
        if name == 'BTreeSet::new': t = T.var(); return k(f'(nil {show(t)})', ('Set', t), env)
        if name == 'BTreeMap::new':
            a, b = T.var(), T.var(); return k(T.empty_map(a, b), ('Map', a, b), env)
        def applied(vs, env):
            hooked = self.hook_call(segs, vs, e, env, ctx, k)
            if hooked is not None: return hooked
            if len(segs) == 1 and name in env and env[name][0] is None:
                _, cl, cenv = env[name][1]; return self.call_closure(cl, cenv, vs, env, ctx, k)
            if name == 'Some': return k(f'(ctor Option.some [{show(vs[0][1])}] {vs[0][0]})', ('Option', vs[0][1]), env)
            if name in ('Ok', 'Err'):
                other = T.var(); ok, er = (vs[0][1], other) if name == 'Ok' else (other, vs[0][1])
                return k(f'(ctor Result.{"ok" if name == "Ok" else "error"} [{show(ok)}, {show(er)}] {vs[0][0]})', ('Result', ok, er), env)
            if name == 'Box::new': return k(vs[0][0], vs[0][1], env)
            if name == 'usize::try_from':
                T.unify(vs[0][1], NAT, 'try_from'); return k(f'(ctor Option.some [Nat] {vs[0][0]})', ('Option', NAT), env)
            if name == 'std::slice::from_ref': return k(f'(cons {vs[0][0]} (nil {show(vs[0][1])}))', ('List', vs[0][1]), env)
            if name == 'Diagnostic::new' and 'Diagnostic' in EXTERN_TYPES: return k(vs[1][0], BYTES, env)
            if name in EXTERN_FNS:
                lexname, pts, rt = EXTERN_FNS[name]
                for (v, t), pt in zip(vs, pts): T.unify(t, pt, f'argument of {name}')
                return k(f'({lexname} {" ".join(v for v, _ in vs)})', rt, env)
            if (None, name) in self.fns:
                fn = self.fns[(None, name)]
                if fn['generics']:
                    g = {n: T.var() for n in fn['generics']}
                    return self.inline([('pid', p) for p, _ in fn['params']], [self.ty(t, None, g) for _, t in fn['params']], fn['body'], self.ty(fn['ret'], None, g), {}, vs, env, ctx, k)
                return self.apply_user((None, name), vs, e[2], env, k)
            raise Unsupported(f'call {name}')
        return self.exprs(e[2], env, ctx, applied)

    def fold(self, r, el, acc_t, init, step, env):
        """a left fold of `step` over list r. When the step calls back into the current recursion group, LexLean
        cannot check that call inside a lambda, so the fold becomes a helper definition in the group, recursing on fuel."""
        show = self.T.show
        rec = '__fuel' in env
        env2 = dict(env)
        q = self.fresh('fuel'); env2['__fuel'] = (q, NAT)   # the step's own fuel, in case it calls back into the group
        acc, x = self.fresh('acc'), self.fresh('item')
        mark = len(self.call_log)
        body = step(acc, x, env2)
        if not any(key in self.cur_scc for key in self.call_log[mark:]):
            body = re.sub(r'(?<![\w:.])' + q + r'(?![\w:])', env['__fuel'][0] if rec else FUEL, body)
            caps = self.captures(body, env, {acc, x})
            return f'(prim list_fold (fun {caps}({acc} : {show(acc_t)}) ({x} : {show(el)}) => {body}) {init} {r} : {show(acc_t)})'
        types = {n: t for n, t in env2.values() if n is not None}
        caps = self.free(body, env2, {acc, x, q})
        self.helper_n += 1
        name = f'{self.fns[self.key]["lex"]}Fold{self.helper_n}'
        self.reserved.add(name)
        fh, rest, more = self.fresh('fuel'), self.fresh('rest'), self.fresh('more')
        params = [f'{fh} : Nat'] + [f'{c} : {show(types[c])}' for c in caps] + [f'{acc} : {show(acc_t)}', f'{rest} : (List {show(el)})']
        recur = '(' + ' '.join([name, q] + caps + [body, more]) + ')'
        helper = ('definition{\n  name: %s,\n  parameters: [%s],\n  result: %s,\n  body: (match %s | Nat.zero => %s | Nat.succ %s => (match %s | List.nil => %s | List.cons %s %s => %s)),'
                  '\n  executable: true,\n  mutual: @GROUP@,\n  recursive_argument: %s\n}') % (
                  name, ', '.join(params), show(acc_t), fh, acc, q, rest, acc, x, more, recur, fh)
        self.helpers.append(helper)
        return '(' + ' '.join([name, env['__fuel'][0] if rec else FUEL] + caps + [init, r]) + ')'

    def irrefutable(self, p):
        if p[0] in ('pid', 'pwild'): return True
        if p[0] == 'ptuple': return all(self.irrefutable(x) for x in p[1])
        if p[0] == 'pat_at': return self.irrefutable(p[2])
        return False
    def slice_arm(self, p):
        """(prefix, rest name or '' or None, suffix) of a slice pattern this rewrite takes, else None."""
        if p[0] != 'pslice': return None
        rests = [i for i, x in enumerate(p[1]) if x == ('prest',) or (x[0] == 'pat_at' and x[2] == ('prest',))]
        if len(rests) > 1: return None
        if rests:
            r = rests[0]; x = p[1][r]
            prefix, rest, suffix = p[1][:r], (x[1] if x[0] == 'pat_at' else ''), p[1][r + 1:]
        else: prefix, rest, suffix = p[1], None, []
        if any(x[0] in ('pslice', 'prest') for x in prefix + suffix): return None
        return prefix, rest, suffix
    def unslice(self, scrut, arms):
        self.spec_n += 1; name = f'__sliced{self.spec_n}'
        S = ('path', [name]); n_ = ('mcall', S, 'len', [])
        out = []
        for pat, guard, body in arms:
            shape = self.slice_arm(pat)
            if shape is None:
                if pat[0] == 'pslice': raise Unsupported('slice pattern')
                out.append((pat, guard, body)); continue
            prefix, rest, suffix = shape; m = len(suffix)
            cond = ('bin', '>=' if rest is not None else '==', n_, ('int', len(prefix) + m))
            at = [(p, ('index', S, ('int', i))) for i, p in enumerate(prefix)]
            at += [(p, ('index', S, ('bin', '-', n_, ('int', m - j)))) for j, p in enumerate(suffix)]
            lets = [('let', p, None, v, None) for p, v in at if self.irrefutable(p)]
            # elements whose pattern can fail: tested in the guard, then bound by the same match around the body
            tested = [(p, v) for p, v in at if not self.irrefutable(p)]
            if tested:
                tp = tested[0][0] if len(tested) == 1 else ('ptuple', [p for p, _ in tested])
                tv = tested[0][1] if len(tested) == 1 else ('tuple', [v for _, v in tested])
                fits = ('match', tv, [(tp, None, ('bool', 'true')), (WILD, None, ('bool', 'false'))])
                cond = ('bin', '&&', cond, fits)
                body = ('match', tv, [(tp, None, body), (WILD, None, ('macro', 'unreachable', []))])
                if guard is not None: guard = ('match', tv, [(tp, None, guard), (WILD, None, ('bool', 'false'))])
            if rest:
                end = None if m == 0 else ('bin', '-', n_, ('int', m))
                lets.append(('let', ('pid', rest), None, ('index', S, ('range', ('int', len(prefix)), end, False)), None))
            g = cond if guard is None else ('bin', '&&', cond, ('block', list(lets), guard))
            out.append((WILD, g, ('block', list(lets), body)))
        return ('block', [('let', ('pid', name), None, scrut, None)], ('match', S, out))
    def combinator_arms(self, head, m, args):
        """the arms of the match an Option or Result combinator stands for, or None."""
        def body(c, value=None):
            if c[0] != 'closure': return None
            if value is None:
                return c[2] if not c[1] else None
            if len(c[1]) != 1: return None
            return ('block', [('let', c[1][0][0], c[1][0][1], value, None)], c[2])
        v = ('path', ['__held']); held = ('pid', '__held')
        P = lambda ctor, pat=None: ('pctor', [ctor], [] if pat is None else [(0, pat)])
        C = lambda ctor, e: ('call', ('path', [ctor]), [e])
        one = args[0] if args else None
        if head == 'Option':
            some, none = P('Some', held), P('None')
            table = {
                'map': lambda: [(some, None, C('Some', body(one, v))), (none, None, ('path', ['None']))],
                'and_then': lambda: [(some, None, body(one, v)), (none, None, ('path', ['None']))],
                'is_some_and': lambda: [(some, None, body(one, v)), (none, None, ('bool', 'false'))],
                'unwrap_or_else': lambda: [(some, None, v), (none, None, body(one))],
                'or_else': lambda: [(some, None, C('Some', v)), (none, None, body(one))],
                'ok_or_else': lambda: [(some, None, C('Ok', v)), (none, None, C('Err', body(one)))],
                'map_or': lambda: [(some, None, body(args[1], v)), (none, None, args[0])],
                'map_or_else': lambda: [(some, None, body(args[1], v)), (none, None, body(args[0]))],
            }
        else:
            ok, err = P('Ok', held), P('Err', held)
            table = {
                'map': lambda: [(ok, None, C('Ok', body(one, v))), (err, None, C('Err', v))],
                'map_err': lambda: [(ok, None, C('Ok', v)), (err, None, C('Err', body(one, v)))],
                'and_then': lambda: [(ok, None, body(one, v)), (err, None, C('Err', v))],
                'or_else': lambda: [(ok, None, C('Ok', v)), (err, None, body(one, v))],
                'unwrap_or_else': lambda: [(ok, None, v), (err, None, body(one, v))],
            }
        if m not in table: return None
        arms = table[m]()
        if any(e is None for _, _, e in arms): return None
        return arms
    def adapter_loop(self, m, recv, pat, body):
        """the loop an adapter stands for, as syntax: `recv` the iterator, `pat` the closure's parameter."""
        flag = ('path', ['__adapted'])
        stop = lambda value: ('block', [('expr', ('assign', flag, value)), ('expr', ('break', None, None))], None)
        if m == 'for_each':
            return ('block', [('expr', ('for', pat, recv, ('block', [('expr', body)], None)))], None)
        if m in ('any', 'all'):
            found = ('bool', 'true') if m == 'any' else ('bool', 'false')
            start = ('bool', 'false') if m == 'any' else ('bool', 'true')
            test = body if m == 'any' else ('not', body)
            inner = ('block', [('expr', ('if', test, stop(found), None))], None)
            return ('block', [('let', ('pid', '__adapted'), None, start, None), ('expr', ('for', pat, recv, inner))], flag)
        # try_for_each: the first error, or Ok(())
        ok = ('call', ('path', ['Ok']), [('tuple', [])])
        failed = (('pctor', ['Err'], [(0, ('pid', '__failure'))]), None, stop(('call', ('path', ['Err']), [('path', ['__failure'])])))
        inner = ('block', [('expr', ('match', body, [failed, (WILD, None, ('block', [], None))]))], None)
        return ('block', [('let', ('pid', '__adapted'), None, ok, None), ('expr', ('for', pat, recv, inner))], flag)
    def mcall(self, e, env, ctx, k):
        T, show = self.T, self.T.show
        _, recv, m, args = e
        # `any`, `all`, `for_each`, `try_for_each` whose closure mutates what it captures: the loop it stands for
        if m in ('any', 'all', 'for_each', 'try_for_each') and len(args) == 1 and args[0][0] == 'closure' and len(args[0][1]) == 1 \
                and self.assigned(args[0][2], env):
            if ctx is None: raise Unsupported('a mutating closure in a pure position')
            return self.expr(self.adapter_loop(m, recv, args[0][1][0][0], args[0][2]), env, ctx, k)
        # more adapters whose closure mutates what it captures, where running the closure eagerly is what Rust does too:
        # `find_map` (it stops at the first Some), `filter`/`filter_map` consumed whole by `collect`, and `bool.then`
        mutating = lambda c: c[0] == 'closure' and self.assigned(c[2], env)
        if m == 'then' and len(args) == 1 and args[0][0] == 'closure' and not args[0][1] and mutating(args[0]):
            if ctx is None: raise Unsupported('a mutating closure in a pure position')
            some = ('block', [], ('call', ('path', ['Some']), [args[0][2]]))
            return self.expr(('if', recv, some, ('block', [], ('path', ['None']))), env, ctx, k)
        if m == 'find_map' and len(args) == 1 and len(args[0]) > 1 and args[0][0] == 'closure' and len(args[0][1]) == 1 and mutating(args[0]):
            if ctx is None: raise Unsupported('a mutating closure in a pure position')
            self.spec_n += 1; out = f'__found{self.spec_n}'; y = f'__hit{self.spec_n}'
            hit = (('pctor', ['Some'], [(0, ('pid', y))]), None,
                   ('block', [('expr', ('assign', ('path', [out]), ('call', ('path', ['Some']), [('path', [y])]))), ('expr', ('break', None, None))], None))
            body = ('block', [('expr', ('match', args[0][2], [hit, (WILD, None, ('block', [], None))]))], None)
            return self.expr(('block', [('let', ('pid', out), None, ('path', ['None']), None), ('expr', ('for', args[0][1][0][0], recv, body))], ('path', [out])), env, ctx, k)
        if m == 'collect' and recv[0] == 'mcall' and recv[2] in ('filter', 'filter_map') and len(recv[3]) == 1 and recv[3][0][0] == 'closure'                 and len(recv[3][0][1]) == 1 and mutating(recv[3][0]):
            if ctx is None: raise Unsupported('a mutating closure in a pure position')
            cl = recv[3][0]; self.spec_n += 1; out = f'__kept{self.spec_n}'; it = f'__each{self.spec_n}'; y = f'__hit{self.spec_n}'
            push = lambda v: ('block', [('expr', ('mcall', ('path', [out]), 'push', [v]))], None)
            if recv[2] == 'filter':
                inner = ('block', [('let', cl[1][0][0], None, ('path', [it]), None), ('expr', ('if', cl[2], push(('path', [it])), None))], None)
                loop = ('for', ('pid', it), recv[1], inner)
            else:
                arm = (('pctor', ['Some'], [(0, ('pid', y))]), None, push(('path', [y])))
                loop = ('for', cl[1][0][0], recv[1], ('block', [('expr', ('match', cl[2], [arm, (WILD, None, ('block', [], None))]))], None))
            return self.expr(('block', [('let', ('pid', out), None, ('call', ('path', ['Vec', 'new']), []), None), ('expr', loop)], ('path', [out])), env, ctx, k)
        # a `map` whose closure mutates what it captures, collected: a loop that threads that state
        if m == 'collect' and recv[0] == 'mcall' and recv[2] == 'map' and recv[3][0][0] == 'closure':
            cl = recv[3][0]
            if self.assigned(cl[2], env):
                if ctx is None: raise Unsupported('a mutating map in a pure position')
                if len(cl[1]) != 1: raise Unsupported('map closure arity')
                pat = cl[1][0][0]
                ok_arm = (('pctor', ['Ok'], [(0, ('pid', '__y'))]), None, ('block', [('expr', ('mcall', ('path', ['__out']), 'push', [('path', ['__y'])]))], None))
                err_arm = (('pctor', ['Err'], [(0, ('pid', '__e'))]), None, ('return', ('call', ('path', ['Err']), [('path', ['__e'])])))
                blk = ('block', [('let', ('pid', '__out'), None, ('call', ('path', ['Vec', 'new']), []), None),
                                 ('expr', ('for', pat, recv[1], ('block', [('expr', ('match', cl[2], [ok_arm, err_arm]))], None)))],
                       ('call', ('path', ['Ok']), [('path', ['__out'])]))
                rt = T.var()
                local = Ctx(ret=lambda v, e2: k(v, rt, e2), ret_t=rt)
                try:
                    return self.block(blk, env, local, lambda v, t, e2: (T.unify(t, rt, 'collect'), k(v, rt, e2))[1])
                except Unsupported:
                    # the elements are not Results: collect them into a list
                    plain = ('block', [('let', ('pid', '__out'), None, ('call', ('path', ['Vec', 'new']), []), None),
                                       ('expr', ('for', pat, recv[1], ('block', [('expr', ('mcall', ('path', ['__out']), 'push', [cl[2]]))], None)))],
                             ('path', ['__out']))
                    rt2 = T.var()
                    local2 = Ctx(ret=lambda v, e2: k(v, rt2, e2), ret_t=rt2)
                    return self.block(plain, env, local2, lambda v, t, e2: (T.unify(t, rt2, 'collect'), k(v, rt2, e2))[1])
        def with_recv(r, rt, env):
            rt = T.res(rt); head = rt[0]
            if r.startswith('(ctor '):                     # a typed name, so that Lean can elaborate a match on it
                return self.atom(r, rt, lambda n, env2: with_recv(n, rt, env2), env, 'value')
            if head in ('Option', 'Result') and ctx is not None and any(a[0] == 'closure' and self.assigned(a[2], env) for a in args):
                arms = self.combinator_arms(head, m, args)
                if arms is not None:
                    self.spec_n += 1; name = f'__scrutinee{self.spec_n}'
                    return self.atom(r, rt, lambda n, env2: self.expr(('match', ('path', [name]), arms), {**env2, name: (n, rt)}, ctx, k), env, 'scrutinee')
            hooked = self.hook_method(r, rt, m, args, recv, env, ctx, k)
            if hooked is not None: return hooked
            done = lambda v, t: k(v, t, env)
            ca = lambda vs, at=None: self.closure_arg(args[0], at or env, vs)
            if head == 'Option' and m == 'ok_or_else':
                n = self.fresh('got'); body, bt = ca([])
                return done(f'(match {r} | Option.some {n} => (ctor Result.ok [{show(rt[1])}, {show(bt)}] {n}) | Option.none => (ctor Result.error [{show(rt[1])}, {show(bt)}] {body}))', ('Result', rt[1], bt))
            if head == 'Option' and m == 'and_then':
                n = self.fresh('got'); body, bt = ca([(n, rt[1])])
                u = T.var(); T.unify(bt, ('Option', u), 'and_then')
                return done(f'(match {r} | Option.some {n} => {body} | Option.none => (ctor Option.none [{show(u)}]))', bt)
            if head == 'Option' and m == 'map':
                n = self.fresh('got'); body, bt = ca([(n, rt[1])])
                return done(f'(match {r} | Option.some {n} => (ctor Option.some [{show(bt)}] {body}) | Option.none => (ctor Option.none [{show(bt)}]))', ('Option', bt))
            if head == 'Option' and m == 'filter':
                n = self.fresh('got'); body, _ = ca([(n, rt[1])])
                none = f'(ctor Option.none [{show(rt[1])}])'
                return done(f'(match {r} | Option.some {n} => (if {body} then (ctor Option.some [{show(rt[1])}] {n}) else {none}) | Option.none => {none})', rt)
            if head == 'Bool' and m == 'then':
                body, bt = ca([])
                return done(f'(if {r} then (ctor Option.some [{show(bt)}] {body}) else (ctor Option.none [{show(bt)}]))', ('Option', bt))
            if head == 'Result' and m == 'map_err':
                n = self.fresh('got'); f = self.fresh('failure'); body, bt = ca([(f, rt[2])])
                return done(f'(match {r} | Result.ok {n} => (ctor Result.ok [{show(rt[1])}, {show(bt)}] {n}) | Result.error {f} => (ctor Result.error [{show(rt[1])}, {show(bt)}] {body}))', ('Result', rt[1], bt))
            if head == 'Result' and m == 'map':
                n = self.fresh('got'); f = self.fresh('failure'); body, bt = ca([(n, rt[1])])
                return done(f'(match {r} | Result.ok {n} => (ctor Result.ok [{show(bt)}, {show(rt[2])}] {body}) | Result.error {f} => (ctor Result.error [{show(bt)}, {show(rt[2])}] {f}))', ('Result', bt, rt[2]))
            if head == 'Result' and m == 'ok':
                n, f = self.fresh('got'), self.fresh('failure')
                return done(f'(match {r} | Result.ok {n} => (ctor Option.some [{show(rt[1])}] {n}) | Result.error {f} => (ctor Option.none [{show(rt[1])}]))', ('Option', rt[1]))
            if head == 'Option' and m == 'ok': return done(r, rt)
            if head == 'Option' and m == 'is_some':
                n = self.fresh('got'); return done(f'(match {r} | Option.some {n} => true | Option.none => false)', BOOL)
            if head in ('List', 'Iter'):
                el = rt[1]
                if m == 'try_for_each':
                    err = T.var(); acc_t = ('Result', UNIT, err)
                    def step(acc, x, env2):
                        body, bt = ca([(x, el)], env2); T.unify(bt, acc_t, 'try_for_each body'); u = self.fresh('done')
                        return f'(match {acc} | Result.ok {u} => {body} | Result.error {self.fresh("failure")} => {acc})'
                    return done(self.fold(r, el, acc_t, f'(ctor Result.ok [Unit, {show(err)}] ())', step, env), acc_t)
                if m == 'map':
                    out_t = T.var(); box = {}
                    def step(acc, x, env2):
                        body, bt = ca([(x, el)], env2); T.unify(bt, out_t, 'map'); return f'(cons {body} {acc})'
                    folded = self.fold(r, el, ('List', out_t), f'(nil {show(out_t)})', step, env)
                    return done(T.patch(f'(AxiomOutput::reversed [{show(out_t)}] {folded})') if False else f'(AxiomOutput::reversed [{show(out_t)}] {folded})', ('List', out_t))
                if m in ('find', 'all', 'any'):
                    if m == 'find':
                        acc_t = ('Option', el)
                        def step(acc, x, env2):
                            body, _ = ca([(x, el)], env2); f = self.fresh('found')
                            return f'(match {acc} | Option.some {f} => {acc} | Option.none => (if {body} then (ctor Option.some [{show(el)}] {x}) else {acc}))'
                        return done(self.fold(r, el, acc_t, f'(ctor Option.none [{show(el)}])', step, env), acc_t)
                    def step(acc, x, env2):
                        body, _ = ca([(x, el)], env2)
                        return f'(if {acc} then {body} else false)' if m == 'all' else f'(if {acc} then true else {body})'
                    return done(self.fold(r, el, BOOL, 'true' if m == 'all' else 'false', step, env), BOOL)
                if m == 'collect':
                    inner = T.res(el)
                    if inner[0] == 'Result':
                        a, er = inner[1], inner[2]; acc_t = ('Result', ('List', a), er)
                        def step(acc, x, env2):
                            l, y, f, g = self.fresh('collected'), self.fresh('got'), self.fresh('failure'), self.fresh('failure')
                            return (f'(match {acc} | Result.ok {l} => (match {x} | Result.ok {y} => (ctor Result.ok [(List {show(a)}), {show(er)}] (cons {y} {l})) '
                                    f'| Result.error {f} => (ctor Result.error [(List {show(a)}), {show(er)}] {f})) | Result.error {g} => {acc})')
                        folded = self.fold(r, el, acc_t, f'(ctor Result.ok [(List {show(a)}), {show(er)}] (nil {show(a)}))', step, env)
                        l, f = self.fresh('collected'), self.fresh('failure')
                        return done(f'(match {folded} | Result.ok {l} => (ctor Result.ok [(List {show(a)}), {show(er)}] (AxiomOutput::reversed [{show(a)}] {l})) | Result.error {f} => (ctor Result.error [(List {show(a)}), {show(er)}] {f}))', acc_t)
                    return done(r, ('List', el))
            if head == 'Set' and m in ('iter', 'into_iter'): raise Unsupported('iteration over a set (its order would matter)')
            if m in IDENTITY and not args: return done(r, rt)
            if head == 'Bytes' and m == 'starts_with' and args[0] == ('path', ['char', 'is_whitespace']):
                return done(f'(AxiomOutput::startsWhite {r})', BOOL)
            if head == 'Bytes' and m == 'split' and args[0][0] == 'chr':
                return done(f'(LexLeanStd.Text::splitOn {r} ({ord(args[0][1])} : UInt8))', ('Iter', BYTES))
            if head == 'Bytes' and m == 'bytes': return done(f'(RustStd::bytesOf {r})', ('List', UINT8))
            if head == 'UInt8' and m == 'is_ascii_digit': return done(f'(AxiomOutput::inRange {r} (48 : UInt8) (57 : UInt8))', BOOL)
            if head == 'I128' and m == 'unsigned_abs': return done(f'(RustStd::i128Abs {r})', I128)
            if head == 'Bytes' and m == 'parse_i128': return done(f'(RustStd::parseI128 {r})', ('Result', I128, UNIT))
            def with_args(vs, env):
                done = lambda v, t: k(v, t, env)
                a = [v for v, _ in vs]
                def rebind(new, result, result_t):
                    return self.assign_into(recv, new, rt, env, lambda env2: k(result, result_t, env2))
                if head == 'Option' and m == 'unwrap_or':
                    n = self.fresh('got'); T.unify(rt[1], vs[0][1], 'unwrap_or')
                    return done(f'(match {r} | Option.some {n} => {n} | Option.none => {a[0]})', rt[1])
                if head == 'Bytes':
                    simple = {'strip_prefix': ('AxiomOutput::stripPrefix', ('Option', BYTES)), 'strip_suffix': ('AxiomOutput::stripSuffix', ('Option', BYTES)),
                              'split_once': ('AxiomOutput::splitOnce', ('Option', ('Prod', BYTES, BYTES))), 'rsplit_once': ('AxiomOutput::splitLast', ('Option', ('Prod', BYTES, BYTES))),
                              'starts_with': ('LexLeanStd.Text::startsWith', BOOL), 'ends_with': ('LexLeanStd.Text::endsWith', BOOL),
                              'trim': ('AxiomOutput::trimmed', BYTES), 'trim_end': ('RustStd::trimEnd', BYTES),
                              'lines': ('LexLeanStd.Text::lines', ('Iter', BYTES))}
                    # `parse()` with its target left to inference: an unsigned integer, the one kind of number here (a
                    # signed or other target fails to unify, and the function is refused)
                    if m == 'parse' and not a: return done(f'(RustStd::parseUnsigned {r} 18446744073709551615)', ('Result', NAT, UNIT))
                    if m in simple: return done(f'({simple[m][0]} {" ".join([r] + a)})', simple[m][1])
                    if m == 'splitn': return done(f'(RustStd::splitN {r} {a[0]} {a[1]})', ('Iter', BYTES))
                    if m == 'is_empty': return done(f'((prim length {r} : Nat) =? 0)', BOOL)
                    if m == 'len': return done(f'(prim length {r} : Nat)', NAT)
                    if m in ('push', 'push_str'): return rebind(f'(AxiomOutput::cat {r} {a[0]})', '()', UNIT)
                if head in ('List', 'Iter'):
                    el = rt[1]
                    if m == 'len': return done(f'(AxiomOutput::listLength [{show(el)}] {r})', NAT)
                    if m == 'is_empty': return done(f'((AxiomOutput::listLength [{show(el)}] {r}) =? 0)', BOOL)
                    if m == 'get': T.unify(vs[0][1], NAT, 'get'); return done(f'(RustStd::getAt [{show(el)}] {r} {a[0]})', ('Option', el))
                    if m == 'first': return done(f'(RustStd::headOf [{show(el)}] {r})', ('Option', el))
                    if m == 'rev': return done(f'(AxiomOutput::reversed [{show(el)}] {r})', rt)
                    if m == 'enumerate': return done(f'(RustStd::enumerate [{show(el)}] {r})', ('List', ('Prod', NAT, el)))
                    if m == 'flatten':
                        x = T.var(); T.unify(el, ('List', x), 'flatten'); return done(f'(RustStd::concat [{show(x)}] {r})', ('List', x))
                    if m == 'split_at': T.unify(vs[0][1], NAT, 'split_at'); return done(f'(RustStd::splitAt [{show(el)}] {r} {a[0]})', ('Prod', ('List', el), ('List', el)))
                    if m == 'contains': T.unify(el, vs[0][1], 'contains'); return done(self.member(r, a[0], el), BOOL)
                    if m == 'zip':
                        o = T.var(); T.unify(vs[0][1], ('List', o), 'zip'); return done(f'(RustStd::zip [{show(el)}, {show(o)}] {r} {a[0]})', ('List', ('Prod', el, o)))
                    if m == 'push': T.unify(el, vs[0][1], 'push'); return rebind(f'(RustStd::push [{show(el)}] {r} {a[0]})', '()', UNIT)
                    if m == 'extend': T.unify(('List', el), vs[0][1], 'extend'); return rebind(f'(RustStd::appendList [{show(el)}] {r} {a[0]})', '()', UNIT)
                    if m == 'truncate': T.unify(vs[0][1], NAT, 'truncate'); return rebind(f'(RustStd::takeN [{show(el)}] {r} {a[0]})', '()', UNIT)
                    if m == 'pop': return rebind(f'(RustStd::dropLast [{show(el)}] {r})', f'(RustStd::lastOf [{show(el)}] {r})', ('Option', el))
                    if m == 'sort': T.unify(el, BYTES, 'sort (bytes only in the shim)'); return rebind(f'(RustStd::sortBytes {r})', '()', UNIT)
                    if m == 'next': return rebind(f'(RustStd::tailOf [{show(el)}] {r})', f'(RustStd::headOf [{show(el)}] {r})', ('Option', el))
                if head == 'Set':
                    el = rt[1]
                    if m == 'contains': T.unify(el, vs[0][1], 'set contains'); return done(self.member(r, a[0], el), BOOL)
                    if m == 'len': return done(f'(AxiomOutput::listLength [{show(el)}] {r})', NAT)
                    if m == 'is_empty': return done(f'((AxiomOutput::listLength [{show(el)}] {r}) =? 0)', BOOL)
                    if m == 'insert':
                        T.unify(el, vs[0][1], 'set insert'); p = self.fresh('inserted'); pt = ('Prod', BOOL, rt)
                        return f'(let {p} : {show(pt)} := (if {self.member(r, a[0], el)} then (false, {r}) else (true, (cons {a[0]} {r}))); {rebind(f"(second {p})", f"(first {p})", BOOL)})'
                if head == 'Map' and m == 'insert':
                    T.unify(rt[1], vs[0][1], 'map key'); T.unify(rt[1], BYTES, 'map key (bytes only in the shim)'); T.unify(rt[2], vs[1][1], 'map value')
                    p = self.fresh('inserted'); pt = ('Prod', ('Option', rt[2]), rt)
                    return f'(let {p} : {show(pt)} := (RustStd::mapInsert [{show(rt[2])}] {r} {a[0]} {a[1]}); {rebind(f"(second {p})", f"(first {p})", ("Option", rt[2]))})'
                if head == 'Named' and (rt[1], m) in self.fns:
                    return self.apply_user((rt[1], m), [(r, rt)] + vs, [recv] + args, env, k)
                raise Unsupported(f'method `{m}` on {show(rt)}')
            if head == 'Named' and (rt[1], m) in self.fns: self.hint_args((rt[1], m), args, 1)
            return self.exprs(args, env, ctx, with_args)
        if recv[0] == 'range':
            def ranged(vs, env):
                (lo, lt), (hi, ht) = vs; T.unify(lt, ht, 'range bounds'); t = T.res(lt)
                if m == 'map':
                    T.unify(t, NAT, 'range'); hi2 = f'({hi} + 1)' if recv[3] else hi
                    return with_recv(f'(RustStd::range {lo} {hi2})', ('List', NAT), env)
                if m == 'contains':
                    return self.expr(args[0], env, ctx, lambda x, xt, env: (T.unify(xt, lt, 'range contains'), k(self.within(lo, hi, x, T.nat(lt), recv[3]), BOOL, env))[1])
                raise Unsupported(f'method `{m}` on a range')
            return self.exprs([recv[1], recv[2]], env, ctx, ranged)
        return self.expr(recv, env, ctx, with_recv)
    def within(self, lo, hi, x, t, inclusive):
        if t == UINT8 and inclusive: return f'(AxiomOutput::inRange {x} {lo} {hi})'
        if t == I128: return f'(if (RustStd::i128Le {lo} {x}) then {"(RustStd::i128Le " + x + " " + hi + ")" if inclusive else "(RustStd::i128Lt " + x + " " + hi + ")"} else false)'
        if t == NAT: return f'(if ({lo} <=? {x}) then ({x} {"<=?" if inclusive else "<?"} {hi}) else false)'
        raise Unsupported(f'range of {self.T.show(t)}')

    # -- definitions
    def function(self, f):
        self.used = {}; self.reserved = {i['lex'] for i in self.fns.values()} | set(self.derived_names.values())
        self.owner = f['owner']; self.key = (f['owner'], f['name']); self.calls = set(); self.call_log = []; self.helpers = []; self.helper_n = 0
        ret_t = self.ty(f['ret'], self.owner)
        env, params = {}, []
        for name, t in f['params']:
            lt = self.ty(t, self.owner); n = self.fresh('this' if name == 'self' else name)
            env[name] = (n, lt); params.append((n, lt))
        muts = [name for name, t in f['params'] if t[0] == 'mut']
        wrap = (lambda v, env: v) if not muts else (lambda v, env: f'({v}, {self.pack(muts, env)})')
        lex_ret = ret_t if not muts else ('Prod', ret_t, self.pack_type(muts, env))
        fuel_attrs, fuel_param = '', []
        if self.key in self.rec_keys:
            fuel = self.fresh('fuel'); pred = self.fresh('fuel')
            env['__fuel'] = (pred, NAT); fuel_param = [(fuel, NAT)]
            fuel_attrs += f',\n  recursive_argument: {fuel}'
        def tail(v, t, env): self.T.unify(t, ret_t, 'result'); return wrap(v, env)
        body = self.block(f['body'], env, Ctx(ret=wrap, ret_t=ret_t), tail)
        if fuel_param:
            body = f'(match {fuel_param[0][0]} | Nat.zero => {self.default(lex_ret)} | Nat.succ {env["__fuel"][0]} => {body})'
        all_params = fuel_param + params
        tag = None
        if fuel_param:
            tag = self.tag_of(self.scc[self.key])
            fuel_attrs = f',\n  mutual: @TAG:{tag}@' + fuel_attrs
        text_ = 'definition{\n  name: %s,\n  parameters: [%s],\n  result: %s,\n  body: %s,\n  executable: true%s\n}' % (
            f['lex'], ', '.join(f'{n} : {self.T.show(t)}' for n, t in all_params), self.T.show(lex_ret), body, fuel_attrs)
        kept, frontier = [], text_
        helpers = {re.search(r'name: (\w+)', h).group(1): h for h in self.helpers}
        changed = True
        while changed:
            changed = False
            for n, h in helpers.items():
                if n not in kept and re.search(r'\(' + n + r' ', frontier):
                    kept.append(n); frontier += h; changed = True
        self.group_has_helpers[tag] = bool(kept) or self.group_has_helpers.get(tag, False) if tag else False
        texts = [text_] + [helpers[n].replace('@GROUP@', f'@TAG:{tag}@') for n in kept]
        return self.T.patch('\n\n'.join(texts))
    def tag_of(self, comp):
        first = min(self.fns[k]['lex'] for k in comp)
        return 'Rec' + first[0].upper() + first[1:]
    def fn_deps(self, f):
        exact = getattr(self, 'exact_deps', None)
        if exact is not None:                          # the calls a previous translation actually made
            return {k for k in exact.get((f['owner'], f['name']), set()) if k in self.fns and not self.fns[k]['generics']}
        out = set()
        for n in self.walk(f['body']):
            if n and n[0] == 'call' and n[1][0] == 'path':
                segs = [s for s in n[1][1] if s not in ('super', 'crate')]
                if len(segs) == 1 and (None, segs[0]) in self.fns: out.add((None, segs[0]))
            if n and n[0] == 'mcall':
                for key in self.fns:
                    if key[1] == n[2] and key[0] is not None: out.add(key)
        return {k for k in out if not self.fns[k]['generics']}
    def module(self, name, imports):
        out, report = [], []
        for n in self.type_order:
            if n in self.type_error: report.append((n, 'UNSUPPORTED: ' + self.type_error[n])); continue
            out.append(self.declare(n)); report.append((n, 'ok'))
        for c in self.consts.values(): report.append((c['name'], 'ok (constant: inlined at its uses)'))
        for f in self.items:
            if f['k'] == 'fn' and f['generics']: report.append((f['name'], 'ok (generic: inlined at its calls)'))
        fns = [f for f in self.items if f['k'] == 'fn' and not f['generics']]
        keys = [(f['owner'], f['name']) for f in fns]
        deps = {k: self.fn_deps(self.fns[k]) for k in keys}
        # strongly connected components, in dependency order (Tarjan)
        index, low, stack, on, order, counter = {}, {}, [], set(), [], [0]
        def strong(v):
            index[v] = low[v] = counter[0]; counter[0] += 1; stack.append(v); on.add(v)
            for w in sorted(deps[v], key=str):
                if w not in index: strong(w); low[v] = min(low[v], low[w])
                elif w in on: low[v] = min(low[v], index[w])
            if low[v] == index[v]:
                comp = []
                while True:
                    w = stack.pop(); on.discard(w); comp.append(w)
                    if w == v: break
                order.append(comp)
        for k in keys:
            if k not in index: strong(k)
        for comp in order:
            if len(comp) > 1 or comp[0] in deps[comp[0]]:
                for k in comp: self.rec_keys.add(k); self.scc[k] = comp
        defs = []
        for comp in order:
            self.cur_scc = set(comp); self.comp_calls = set()
            texts, failure = [], None
            for key in sorted(comp, key=lambda k: keys.index(k)):
                try:
                    if any(self.status.get(d, 'ok') != 'ok' and self.fnmut_index(d) is None for d in deps[key] if d not in self.cur_scc):
                        bad = next(d for d in deps[key] if d not in self.cur_scc and self.status.get(d, 'ok') != 'ok' and self.fnmut_index(d) is None)
                        self.blocked_by[key] = bad
                        raise Unsupported(f'calls `{bad[1]}`, which did not translate')
                    self.spec_texts = []
                    texts.append(self.function(self.fns[key])); self.comp_calls |= self.calls; self.key_calls[key] = set(self.calls)
                    texts.extend(self.spec_texts); self.spec_texts = []
                except Unsupported as x:
                    if self.status.get(key) == 'ok' and key in set(self.spec_of.values()): continue   # translated as its copies
                    failure = failure or f'{key[1]}: {x}'
                    self.status[key] = f'UNSUPPORTED: {x}'
            if failure and len(comp) > 1:
                first_bad = next(k for k in sorted(comp, key=lambda k: keys.index(k)) if self.status.get(k, 'ok') != 'ok')
                for key in comp:
                    if self.status.get(key, 'ok') == 'ok':
                        self.status[key] = f'UNSUPPORTED: its recursion group failed ({failure})'; self.blocked_by[key] = first_bad
            if not failure:
                for key in comp: self.status[key] = 'ok'
                for called in self.comp_calls:
                    if called in self.spec_of: self.status[self.spec_of[called]] = 'ok'      # translated, as this copy
                defs.append((tuple(comp), texts, set(self.comp_calls)))
            for key in sorted(comp, key=lambda k: keys.index(k)): report.append((key[1], self.status[key]))
        head = '\\begin{lexlean}{%s}\n\\useglossary{lexlean.std.nat@1.2.0}\n%s\\title{Natural number addition}\n\\begin{semanticmodule}\n' % (
            name, ''.join('\\importmodule{%s}\n' % m for m in imports))
        # a cycle formed only through calls the call graph cannot see (Display) has no fuel: refuse it
        comp_of = {k: comp for comp, texts, calls in defs for k in comp}
        graph = {comp: {comp_of[c] for c in calls if c in comp_of} for comp, texts, calls in defs}
        def refuse(comp):
            for key in comp: self.status[key] = 'UNSUPPORTED: recursion through an implicit call (Display)'
        for comp, texts, calls in defs:
            if any(c in comp for c in calls) and comp[0] not in self.rec_keys: refuse(comp)
        state = {}
        def visit(c, stack):
            state[c] = 1; stack.append(c)
            for d in graph.get(c, ()):
                if d == c: continue
                if state.get(d) == 1:
                    for e in stack[stack.index(d):]: refuse(e)
                elif d not in state: visit(d, stack)
            stack.pop(); state[c] = 2
        for c in graph:
            if c not in state: visit(c, [])
        # withdraw any group that calls something which ended up failing (calls the call graph did not order)
        changed = True
        while changed:
            changed = False
            for comp, texts, calls in defs:
                if self.status.get(comp[0]) != 'ok': continue
                bad = next((c for c in calls if c not in comp and self.status.get(c, 'ok') != 'ok'), None)
                if bad is not None:
                    for key in comp: self.status[key] = f'UNSUPPORTED: calls `{bad[1]}`, which did not translate'; self.blocked_by[key] = bad
                    changed = True
        report = [(n, self.status.get(next((k for k in self.fns if k[1] == n), None), st) if st == 'ok' else st) for n, st in report]
        defs = [t for comp, texts, calls in defs if self.status.get(comp[0]) == 'ok' for t in texts]
        defs = list(dict.fromkeys(defs))             # a shared copy is listed by each caller: once is enough
        derived = [self.T.patch(d) for d in self.derived]
        sizes = {self.tag_of(comp): len(comp) for comp in order if comp[0] in self.rec_keys}
        def resolve(m):
            tag = m.group(1)
            return f',\n  mutual: {tag}' if sizes.get(tag, 1) > 1 or self.group_has_helpers.get(tag) else ''
        defs = [re.sub(r',\n  mutual: @TAG:(\w+)@', resolve, d) for d in defs]
        return head + '\n\n'.join(out + derived + defs) + '\n\\end{semanticmodule}\n\\end{lexlean}\n', report

if __name__ == '__main__':
    dst, name = sys.argv[1:3]
    src = '\n'.join(open(p, encoding='utf-8').read() for p in sys.argv[3:])
    items = Parser(lex(src)).file()
    text_out, report = Translator(items).module(name, ['AxiomOutput', 'RustStd', 'LexLeanStd.Text'])
    open(dst, 'w', encoding='utf-8', newline='\n').write(text_out)
    ok = sum(1 for _, r in report if r.startswith('ok'))
    for n, r in report: print(f'{n}: {r}')
    print(f'-- {ok} of {len(report)} items translated; {len(text_out)} bytes')

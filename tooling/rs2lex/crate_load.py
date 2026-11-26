"""Load a whole Rust crate for the translator: fault-tolerant per-item parsing, module paths, `use` resolution, and a
rewrite of every item so that each type, function and constant has one crate-unique name.

Output: (items, skipped) where items are translator items carrying 'file' and 'bytes', and skipped lists the items
that could not even be parsed or that are out of scope (traits, generic types, macros), each with a reason."""
import os, re, sys, collections
import rs2lex_core as core
from rs2lex_core import Parser, Unsupported

TOKEN = core.TOKEN
def lex(src):
    out, i = [], 0
    while i < len(src):
        m = TOKEN.match(src, i)
        if not m: out.append(('op', src[i], i)); i += 1; continue
        if m.lastgroup != 'ws':
            conv = core.lex(m.group())[0]
            out.append((conv[0], conv[1], m.start()))
        i = m.end()
    return out

def module_path(rel):
    parts = rel[:-3].split('/')
    if parts[-1] in ('mod', 'lib'): parts = parts[:-1]
    return tuple(parts)

def match_close(toks, i, o, c):
    depth = 0
    while i < len(toks):
        v = toks[i][1]
        if v == o: depth += 1
        elif v == c:
            depth -= 1
            if depth == 0: return i
        i += 1
    return len(toks) - 1

def item_end(toks, i):
    """index of the last token of the item whose keyword is at toks[i] (after attributes and visibility)."""
    depth = 0
    kw = toks[i][1]
    j = i
    while j < len(toks):
        v = toks[j][1]
        if v in ('(', '['): depth += 1
        elif v in (')', ']'): depth -= 1
        elif v == '{' and depth == 0:
            if kw in ('const', 'static', 'type', 'use'):     # a brace inside an expression or a use tree
                j = match_close(toks, j, '{', '}') + 1; continue
            return match_close(toks, j, '{', '}')
        elif v == ';' and depth == 0: return j
        j += 1
    return len(toks) - 1

def parse_use(toks):
    """`use a::b::{c, d as e, f::{g}};` -> [(alias, [segments])]"""
    out = []
    def tree(i, prefix):
        segs = list(prefix)
        while i < len(toks):
            k, v = toks[i][0], toks[i][1]
            if v == '{':
                i += 1
                while toks[i][1] != '}':
                    i = tree(i, segs)
                    if toks[i][1] == ',': i += 1
                return i + 1
            if v == '*': out.append(('*', segs)); return i + 1
            if k == 'id':
                segs.append(v); i += 1
                if i < len(toks) and toks[i][1] == '::': i += 1; continue
                alias = segs[-1]
                if i < len(toks) and toks[i][1] == 'as': alias = toks[i + 1][1]; i += 2
                if alias == 'self': alias = segs[-2] if len(segs) > 1 else alias; segs = segs[:-1]
                out.append((alias, segs)); return i
            i += 1
        return i
    i = 1
    while i < len(toks) and toks[i][1] != ';':
        i = tree(i, [])
        if i < len(toks) and toks[i][1] == ',': i += 1
        else: break
    return out

class Crate:
    def __init__(self, root):
        self.modules = {}                         # modpath -> {'items': {name: (kind, entry)}, 'uses': {alias: segs}, 'file': rel}
        self.entries = []                         # (modpath, rel, item, bytes)
        self.skipped = []                         # (rel, name, kind, bytes, reason)
        self.aliases = []                         # (modpath, name, params, type)
        self.custom_default = set()               # types with a hand-written Default impl
        for d, _, files in os.walk(root):
            for f in sorted(files):
                if not f.endswith('.rs'): continue
                rel = os.path.relpath(os.path.join(d, f), root).replace(os.sep, '/')
                if rel == 'main.rs': continue
                src = open(os.path.join(d, f), encoding='utf-8').read()
                self.load(module_path(rel), rel, lex(src), src)

    def module(self, mp, rel):
        return self.modules.setdefault(mp, {'items': {}, 'uses': {}, 'file': rel})

    def load(self, mp, rel, toks, src):
        mod = self.module(mp, rel)
        i = 0
        while i < len(toks):
            start = i
            attrs = []
            while i < len(toks) and toks[i][1] == '#':
                inner = toks[i + 1][1] == '!'
                j = i + 2 if inner else i + 1
                end = match_close(toks, j, '[', ']')
                attrs.append(' '.join(t[1] for t in toks[j:end + 1])); i = end + 1
            if i >= len(toks): break
            test = any(re.search(r'\bcfg\s*\(\s*(test|any\s*\(\s*test)', a) for a in attrs)
            j = i
            if toks[j][1] == 'pub':
                j += 1
                if toks[j][1] == '(': j = match_close(toks, j, '(', ')') + 1
            while toks[j][1] in ('unsafe', 'async', 'extern', 'default') or toks[j][0] == 'str': j += 1
            kw = toks[j][1]
            if kw == 'macro_rules':
                end = match_close(toks, j, '{', '}') if any(t[1] == '{' for t in toks[j:j + 4]) else item_end(toks, j)
                i = end + 1; continue
            if kw == 'const' and toks[j + 1][1] == 'fn': kw = 'fn'; end = item_end(toks, j + 1)
            else: end = item_end(toks, j)
            name = toks[j + 1][1] if j + 1 < len(toks) else '?'
            nbytes = toks[end][2] - toks[start][2] + 1
            if test: i = end + 1; continue
            if kw == 'use':
                for alias, segs in parse_use(toks[j:end + 1]): mod['uses'][alias] = segs
            elif kw == 'mod':
                if toks[end][1] == '}':
                    body_start = next(n for n in range(j, end) if toks[n][1] == '{')
                    self.load(mp + (name,), rel, toks[body_start + 1:end], src)
            elif kw == 'impl':
                self.load_impl(mp, rel, toks[j:end + 1], attrs)
            elif kw == 'type':
                try:
                    p = Parser([(t[0], t[1]) for t in toks[j + 1:end + 1]] + [('eof', '')])
                    name = p.ident(); params = p.generics(); p.eat('='); body = p.type()
                    self.aliases.append((mp, name, params, body))
                except Unsupported as x: self.skipped.append((rel, name, kw, nbytes, f'type alias: {x}'))
            elif kw in ('trait', 'union', 'static', 'extern'):
                self.skipped.append((rel, name, kw, nbytes, f'{kw} item'))
            elif kw in ('fn', 'struct', 'enum', 'const'):
                before = len(self.entries)
                self.parse_item(mp, rel, toks[start:end + 1], None, nbytes, kw, name)
                if kw in ('struct', 'enum') and len(self.entries) > before:
                    # the traits it derives: an order or an equality the translation may derive only when Rust does
                    self.entries[-1]['derives'] = {w for a in attrs if 'derive' in a for w in re.findall(r'[A-Z]\w*', a)}
            else:
                self.skipped.append((rel, name, kw, nbytes, f'unparsed item `{kw}`'))
            i = end + 1

    def load_impl(self, mp, rel, toks, attrs):
        brace = next(n for n, t in enumerate(toks) if t[1] == '{')
        head = toks[1:brace]
        generic_types = []
        k = 0
        if head and head[0][1] == '<':
            close = 0; depth = 0
            for n, t in enumerate(head):
                if t[1] == '<': depth += 1
                elif t[1] == '>':
                    depth -= 1
                    if depth == 0: close = n; break
            generic_types = [t[1] for t in head[1:close] if t[0] == 'id' and t[1] not in ('const',)]
            generic_types = [g for n, g in enumerate(generic_types)]
            k = close + 1
        vals = [t[1] for t in head[k:]]
        is_trait = 'for' in vals
        owner_part = vals[vals.index('for') + 1:] if is_trait else vals
        owner = next((v for v in owner_part if re.match(r'[A-Za-z_]\w*$', v) and v not in ('dyn', 'mut')), '?')
        body = toks[brace + 1:-1]
        i = 0
        while i < len(body):
            start = i; attrs2 = []
            while i < len(body) and body[i][1] == '#':
                end = match_close(body, i + 1, '[', ']'); attrs2.append(' '.join(t[1] for t in body[i + 1:end + 1])); i = end + 1
            if i >= len(body): break
            j = i
            if body[j][1] == 'pub':
                j += 1
                if body[j][1] == '(': j = match_close(body, j, '(', ')') + 1
            while body[j][1] in ('unsafe', 'async', 'default'): j += 1
            kw = body[j][1]
            if kw == 'const' and body[j + 1][1] == 'fn': kw = 'fn'; end = item_end(body, j + 1); name = body[j + 2][1]
            else: end = item_end(body, j); name = body[j + 1][1]
            nbytes = body[end][2] - body[start][2] + 1
            test = any('cfg' in a and 'test' in a for a in attrs2)
            if not test:
                trait = vals[:vals.index("for")] if is_trait else []
                if is_trait and trait and trait[-1] == 'Default': self.custom_default.add(owner)
                if is_trait and trait and trait[-1] in ('Display', 'Debug') and name == 'fmt' and not generic_types:
                    self.parse_item(mp, rel, body[start:end + 1], owner, nbytes, kw, name, rename=f'fmt_{trait[-1].lower()}')
                elif is_trait: self.skipped.append((rel, f'{owner}::{name}', 'fn', nbytes, f'trait impl ({" ".join(trait)})'))
                elif generic_types: self.skipped.append((rel, f'{owner}::{name}', 'fn', nbytes, 'method of a generic impl'))
                elif kw in ('fn', 'const'): self.parse_item(mp, rel, body[start:end + 1], owner, nbytes, kw, name)
                else: self.skipped.append((rel, f'{owner}::{name}', kw, nbytes, f'impl item `{kw}`'))
            i = end + 1

    def parse_item(self, mp, rel, toks, owner, nbytes, kw, name, rename=None):
        p = Parser([(t[0], t[1]) for t in toks] + [('eof', '')])
        try:
            items = p.item()
        except Unsupported as x:
            self.skipped.append((rel, f'{owner}::{name}' if owner else name, kw, nbytes, f'parse: {x}')); return
        except (IndexError, RecursionError, ValueError) as x:
            self.skipped.append((rel, f'{owner}::{name}' if owner else name, kw, nbytes, f'parse: {type(x).__name__}')); return
        for it in items:
            if rename: it['name'] = rename
            it['owner'] = owner; it['file'] = rel; it['mp'] = mp; it['bytes'] = nbytes
            if it['k'] in ('struct', 'enum') and it.get('generic'):
                self.skipped.append((rel, name, kw, nbytes, 'generic type')); continue
            self.entries.append(it)
            if owner is None: self.module(mp, rel)['items'][it['name']] = it

# ----------------------------------------------------------------------------- resolution and renaming
EXTERNAL = {'std', 'core', 'alloc', 'serde', 'serde_json', 'toml', 'camino', 'clap', 'semver', 'tempfile', 'walkdir',
            'fs4', 'rustix', 'signal_hook', 'same_file', 'unicode_normalization'}

class Resolver:
    def __init__(self, crate):
        self.c = crate
        kinds = collections.defaultdict(list)
        for it in crate.entries:
            if it['owner'] is None: kinds[(it['k'] in ('struct', 'enum'), it['name'])].append(it)
        self.unique = {}
        for (is_type, name), its in kinds.items():
            for it in its:
                if len(its) == 1: it['uname'] = name
                elif is_type: it['uname'] = name + ''.join(s[:1].upper() + s[1:] for s in re.split(r'[_/]', '_'.join(it['mp']) or 'root') if s)
                else: it['uname'] = name + '_' + ('_'.join(it['mp']) or 'root')
        for it in crate.entries:
            if it['owner'] is not None: it['uname'] = it['name']
        counts = collections.Counter(name for _, name, _, _ in crate.aliases)
        self.alias_at = {}
        for mp, name, _, _ in crate.aliases:
            self.alias_at[(mp, name)] = name if counts[name] == 1 else name + ''.join(s[:1].upper() + s[1:] for s in re.split(r'[_/]', '_'.join(mp) or 'root') if s) + 'Alias'

    def alias_of(self, mp, name, depth=0):
        """the unique name of the type alias `name` visible at module mp, or None."""
        if depth > 8: return None
        if (mp, name) in self.alias_at: return self.alias_at[(mp, name)]
        mod = self.c.modules.get(mp)
        if not mod or name in mod['items'] or name not in mod['uses']: return None
        segs = list(mod['uses'][name])
        if not segs or segs[0] in EXTERNAL: return None
        if segs[0] == 'crate': base = (); segs = segs[1:]
        else:
            base = mp
            while segs and segs[0] in ('super', 'self'):
                if segs[0] == 'super': base = base[:-1]
                segs = segs[1:]
        while len(segs) > 1 and base + (segs[0],) in self.c.modules: base = base + (segs[0],); segs = segs[1:]
        return self.alias_of(base, segs[0], depth + 1) if len(segs) == 1 else None

    def lookup(self, mp, name, depth=0):
        """an item named `name` visible at module mp: defined there, or imported by `use`."""
        if depth > 8: return None
        mod = self.c.modules.get(mp)
        if not mod: return None
        if name in mod['items']: return mod['items'][name]
        if name in mod['uses']: return self.path(mp, mod['uses'][name], depth + 1)
        return None

    def path(self, mp, segs, depth=0):
        """resolve a path to an item; returns the item (for Type::Variant paths, the type)."""
        if not segs: return None
        segs = list(segs)
        if segs[0] in EXTERNAL: return None
        if segs[0] == 'crate': base = (); segs = segs[1:]
        elif segs[0] in ('super', 'self'):
            base = mp
            while segs and segs[0] in ('super', 'self'):
                if segs[0] == 'super': base = base[:-1]
                segs = segs[1:]
        else:
            base = mp
            mod = self.c.modules.get(mp, {'uses': {}})
            if len(segs) > 1 and segs[0] in mod['uses'] and not self.lookup(mp, segs[0], depth + 1):
                return self.path(mp, mod['uses'][segs[0]] + segs[1:], depth + 1)
        # walk module segments
        while len(segs) > 1 and base + (segs[0],) in self.c.modules: base = base + (segs[0],); segs = segs[1:]
        if not segs: return None
        found = self.lookup(base, segs[0], depth + 1)
        if found is None and base == mp and len(segs) == 1: return None
        return found

    def resolve_type(self, mp, name):
        it = self.lookup(mp, name)
        if it is not None and it['k'] in ('struct', 'enum'): return it['uname']
        cands = [e for e in self.c.entries if e['owner'] is None and e['k'] in ('struct', 'enum') and e['name'] == name]
        if len(cands) == 1: return cands[0]['uname']
        return None

    def rewrite(self, it):
        mp = it['mp']
        def ty(t):
            if t is None: return t
            if t[0] == 'mut': return ('mut', ty(t[1]))
            if t[0] == 'ref': return ('ref', ty(t[1]))
            if t[0] == 'tuple': return ('tuple', [ty(x) for x in t[1]])
            if t[0] == 'opaque': return t
            if t[0] == 'fnty': return ('fnty', [ty(x) for x in t[1]], ty(t[2]), t[3])
            _, n, args = t
            u = (self.alias_of(mp, n) or self.resolve_type(mp, n)) if n not in ('Self',) else None
            return ('path', u or n, [ty(a) for a in args])
        def segs_of(segs, value_position):
            if not segs: return segs
            if segs[0] == 'Self': return segs
            if len(segs) == 1:
                n = segs[0]
                if value_position and not (n[:1].isupper()): return segs     # a local variable
                found = self.lookup(mp, n)
                return [found['uname']] if found else segs
            # Type::Variant, Type::assoc, module::item
            head = self.path(mp, segs[:-1]) if len(segs) > 1 else None
            if head is not None and head['k'] in ('struct', 'enum'): return [head['uname'], segs[-1]]
            found = self.path(mp, segs)
            if found is not None: return [found['uname']]
            u = self.resolve_type(mp, segs[0]) if len(segs) == 2 else None
            if u: return [u, segs[1]]
            return segs
        def walk(e, callee=False):
            if isinstance(e, list): return [walk(x) for x in e]
            if not isinstance(e, tuple) or not e: return e
            h = e[0]
            if h == 'path' and isinstance(e[1], list): return ('path', segs_of(e[1], not callee))
            if h == 'path' and isinstance(e[1], str): return ty(e)
            if h == 'call': return ('call', walk(e[1], callee=True), walk(e[2]))
            if h == 'struct':
                out = ('struct', segs_of(e[1], False), [(f, walk(v)) for f, v in e[2]])
                return out + (walk(e[3]),) if len(e) > 3 else out
            if h == 'pctor': return ('pctor', segs_of(e[1], False), [(f, walk(p)) for f, p in e[2]])
            if h == 'let': return ('let', walk(e[1]), ty(e[2]) if e[2] else None, walk(e[3]), walk(e[4]))
            if h == 'cast': return ('cast', walk(e[1]), ty(e[2]))
            if h == 'closure': return ('closure', [(walk(p), ty(t) if t else None) for p, t in e[1]], walk(e[2]), ty(e[3]) if e[3] else None)
            if h == 'item': return e
            if h == 'macro' and isinstance(e[2], list) and e[2] and isinstance(e[2][0], tuple) and e[2][0][0] == 'str':
                # a format string's implicit captures (`"{SYNTAX}.{name}"`) name items too: a constant in scope gets its
                # unique name, as a path would; a lower-case name is a local and stays
                def capture(m):
                    n = m.group(1)
                    found = self.lookup(mp, n) if n[:1].isupper() else None
                    return '{' + (found['uname'] if found and found['k'] == 'const' else n)
                fmt = re.sub(r'(?<!\{)\{([A-Za-z_]\w*)(?=[:}])', capture, e[2][0][1])
                return (h, e[1], [('str', fmt)] + [walk(x) for x in e[2][1:]]) + tuple(e[3:])
            return tuple(walk(x) if isinstance(x, (tuple, list)) else x for x in e)
        k = it['k']
        if k == 'struct': it['fields'] = [(f, ty(t)) for f, t in it['fields']]
        elif k == 'enum': it['variants'] = [(v, [(f, ty(t)) for f, t in fs]) for v, fs in it['variants']]
        elif k == 'const': it['type'] = ty(it['type']); it['value'] = walk(it['value'])
        elif k == 'fn':
            it['params'] = [(p, ty(t)) for p, t in it['params']]; it['ret'] = ty(it['ret']); it['body'] = walk(it['body'])
        if it['owner'] is not None:
            it['owner'] = self.resolve_type(mp, it['owner']) or it['owner']
        it['orig_name'] = it['name']; it['name'] = it['uname']
        return it

def lift_nested(items):
    """a nested fn captures nothing in Rust, so it is lifted to the top level under `parent_child`."""
    out = []
    for it in items:
        if it['k'] != 'fn': out.append(it); continue
        lifted = {}
        def take(e):
            if isinstance(e, list): return [take(x) for x in e]
            if not isinstance(e, tuple) or not e: return e
            if e[0] == 'block' and len(e) == 3 and isinstance(e[1], list):
                stmts = []
                for s in e[1]:
                    if s[0] == 'item':
                        f = s[1]; new = f'{it["name"]}_{f["name"]}'
                        lifted[f['name']] = (new, f)
                    else: stmts.append(take(s))
                return ('block', stmts, take(e[2]))
            return tuple(take(x) if isinstance(x, (tuple, list)) else x for x in e)
        it['body'] = take(it['body'])
        def rename(e):
            if isinstance(e, list): return [rename(x) for x in e]
            if not isinstance(e, tuple) or not e: return e
            if e[0] == 'call' and e[1][0] == 'path' and len(e[1][1]) == 1 and e[1][1][0] in lifted:
                return ('call', ('path', [lifted[e[1][1][0]][0]]), rename(e[2]))
            return tuple(rename(x) if isinstance(x, (tuple, list)) else x for x in e)
        it['body'] = rename(it['body'])
        out.append(it)
        for old, (new, f) in lifted.items():
            f = dict(f); f['name'] = new; f['owner'] = None; f['file'] = it['file']; f['mp'] = it['mp']; f['bytes'] = 0
            f['body'] = rename(f['body']); f['lifted_from'] = it['name']
            out += lift_nested([f])
    return out

def load(root, with_aliases=False):
    c = Crate(root)
    r = Resolver(c)
    items = [r.rewrite(it) for it in c.entries]
    aliases = {}
    for mp, name, params, body in c.aliases:
        fake = {'k': 'struct', 'name': name, 'fields': [('f0', body)], 'owner': None, 'mp': mp, 'uname': name}
        aliases[r.alias_at[(mp, name)]] = (params, r.rewrite(fake)['fields'][0][1])
    for it in items: it['custom_default'] = {r.resolve_type(it['mp'], n) or n for n in c.custom_default} if it['k'] == 'fn' else None
    if with_aliases: return lift_nested(items), c.skipped, aliases
    return lift_nested(items), c.skipped

if __name__ == '__main__':
    items, skipped = load(sys.argv[1])
    kinds = collections.Counter(i['k'] for i in items)
    print('parsed items', dict(kinds), 'skipped', len(skipped))
    reasons = collections.Counter(); rb = collections.Counter()
    for rel, name, kw, nbytes, why in skipped: reasons[why] += 1; rb[why] += nbytes
    for why, n in reasons.most_common(30): print(f'  {n:5d} {rb[why]:>9,}  {why}')

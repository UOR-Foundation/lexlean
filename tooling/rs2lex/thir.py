"""rustc's typed tree (`-Zunpretty=thir-tree`) as a front end: the bodies of a crate after name resolution and type
checking, read from the dump's indented text into a tree, and from there into the translator's input.

    RUSTC_BOOTSTRAP=1 rustc --edition 2021 --crate-type lib -Zunpretty=thir-tree lib.rs > lib.thir

The dump is one body after another, each headed `DefId(krate:index ~ crate[hash]::path):`, then `params:` and
`body:`. Every node is `Name {` with its fields on deeper lines, a field `key: value` on one line or `key:` with its
value below. Lines that only close a bracket carry nothing and are dropped."""
import re

CLOSERS = {'}', ')', ']', ',', '},', '),', '],', ') ', '}) '}
HEAD = re.compile(r'^DefId\((\d+):(\d+) ~ ([\w]+)\[\w+\]::(.*)\):$')


class Node:
    __slots__ = ('text', 'kids')
    def __init__(self, text): self.text = text; self.kids = []
    def __repr__(self): return f'Node({self.text[:40]!r}, {len(self.kids)})'

    # a field `key: value`: its node, or None
    def field(self, key):
        prefix = key + ':'
        for k in self.kids:
            if k.text.startswith(prefix): return k
        return None

    def value(self, key, default=None):
        f = self.field(key)
        return f.text[len(key) + 1:].strip() if f is not None else default

    def find(self, name):
        """the first node named `name {` at or below this one, crossing only wrappers (Some(, a field line)."""
        todo = [self]
        while todo:
            n = todo.pop(0)
            if n.text.startswith(name + ' {') or n.text == name: return n
            if n is self or not n.text.endswith('{') or n.text.startswith(('Some(', 'Arm', 'Stmt')) or ':' in n.text.split('{')[0]:
                todo = list(n.kids) + todo if n is not self else list(n.kids)
        return None

    def all(self, name):
        """the direct `name {` nodes under this field (a list field's elements)."""
        return [k for k in self.kids if k.text.startswith(name + ' {')]


def parse_lines(lines):
    """the indentation tree of a body's lines."""
    root = Node('')
    stack = [(-1, root)]
    for raw in lines:
        text = raw.strip()
        if not text or text in CLOSERS: continue
        indent = len(raw) - len(raw.lstrip(' '))
        while stack[-1][0] >= indent: stack.pop()
        node = Node(text)
        stack[-1][1].kids.append(node)
        stack.append((indent, node))
    return root


def bodies(stream, want=None):
    """(crate, path, tree) for each body of a dump; `want(path)` skips the others without building their trees."""
    head, lines, keep = None, [], False
    for raw in stream:
        if raw.startswith('DefId('):
            m = HEAD.match(raw.rstrip('\n'))
            if m:
                if head is not None and keep: yield head[0], head[1], parse_lines(lines)
                head = (m.group(3), m.group(4)); lines = []
                keep = want is None or want(head[1])
                continue
        if keep: lines.append(raw.rstrip('\n'))
    if head is not None and keep: yield head[0], head[1], parse_lines(lines)


# -- a compact view of a body, for reading a construct's shape

NOISE = ('span', 'temp_scope_id', 'hir_id', 'region_scope', 'init_scope', 'remainder_scope', 'safety_mode', 'is_primary',
         'is_shorthand', 'lint_level', 'from_hir_call', 'fn_span', 'targeted_by_break', 'ty_span', 'user_ty', 'scope')

def kind_of(node):
    """(kind name, the node holding the kind's fields) of an Expr, Pat or Stmt node."""
    k = node.field('kind')
    if k is None: return None, None
    inline = k.text[5:].strip()
    if inline and inline != 'PatKind {':
        return inline.split(' ')[0].split('(')[0].rstrip('{').strip(), k
    holder = k.kids[0] if k.kids else k
    if inline == 'PatKind {': holder = k.kids[0] if k.kids else k
    return holder.text.split(' ')[0].split('(')[0].rstrip('{').strip(), holder


def show(node, depth=0, out=None):
    out = [] if out is None else out
    pad = '  ' * depth
    name = node.text.split(' ')[0]
    if name in ('Expr', 'Pat'):
        kind, holder = kind_of(node)
        while kind in ('Scope', 'Use') and holder is not None:          # wrappers: the value inside
            inner = holder.find('Expr')
            if inner is None: break
            node = inner; kind, holder = kind_of(node)
        out.append(f'{pad}{name}.{kind} : {node.value("ty", "?")[:60]}')
        for k in (holder.kids if holder is not None else []):
            key = k.text.split(':')[0]
            if key in NOISE: continue
            show(k, depth + 1, out)
        return out
    key = node.text.split(':')[0]
    if key in NOISE: return out
    out.append(f'{pad}{node.text[:110]}')
    for k in node.kids: show(k, depth + 1, out)
    return out


if __name__ == '__main__':
    import sys
    for crate, path, tree in bodies(open(sys.argv[1], encoding='utf-8', errors='replace'),
                                    (lambda p: p == sys.argv[2]) if len(sys.argv) > 2 else None):
        print('==', crate, path)
        body = tree.field('body')
        print('\n'.join(show(body.find('Expr'))))

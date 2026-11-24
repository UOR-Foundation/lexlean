"""Language-independent Git input closure, advisory only (CI #603)."""
import hashlib
import json
import re
import subprocess
from pathlib import Path


class Hold(ValueError):
    pass


def require(ok, message):
    if not ok:
        raise Hold(message)


def path_ok(value):
    return (isinstance(value, str) and 0 < len(value) < 4096
            and not any(c in value for c in ('\\', '\x00', '\n', '\r', '\t'))
            and all(p not in ('', '.', '..') for p in value.split('/')))


def git_tree(root, commit):
    """Read committed paths, modes and object identities, never working-tree bytes."""
    require(isinstance(commit, str) and re.fullmatch(r'[0-9a-f]{40}|[0-9a-f]{64}', commit),
            'exact Git commit required')
    def run(*args):
        try:
            return subprocess.run(['git', '-C', str(Path(root).resolve()), *args],
                                  check=True, capture_output=True, timeout=30).stdout
        except (OSError, subprocess.SubprocessError) as error:
            raise Hold('Git input inventory unavailable') from error
    require(run('rev-parse', '--verify', commit + '^{commit}').strip().decode() == commit,
            'commit identity mismatch')
    raw = run('ls-tree', '-r', '-z', '--full-tree', commit)
    require(len(raw) <= 32 * 1024 * 1024 and (not raw or raw.endswith(b'\0')),
            'invalid or oversized tree inventory')
    tree = {}
    for record in raw.split(b'\0')[:-1]:
        try:
            metadata, name = record.split(b'\t', 1)
            mode, kind, oid = metadata.decode('ascii').split(' ')
            name = name.decode('utf-8')
        except (ValueError, UnicodeError) as error:
            raise Hold('unrepresentable Git tree entry') from error
        require(path_ok(name) and name not in tree and len(oid) == len(commit)
                and re.fullmatch(r'[0-9a-f]+', oid), 'invalid tree entry')
        require((mode, kind) in {('100644', 'blob'), ('100755', 'blob'),
                                ('120000', 'blob'), ('160000', 'commit')},
                'unsupported tree entry')
        tree[name] = (mode, oid)
    return tree


def compare(old, new, units, global_roots):
    """Compute deterministic declared closures; never certify source completeness.

    Roots are literal paths or directory prefixes. Any unowned changed path,
    missing declared root, symlink/submodule or changed global input refuses
    candidate reuse. Dependencies propagate both fingerprints and invalidation.
    """
    require(isinstance(units, dict) and 0 < len(units) <= 500, 'invalid unit inventory')
    def roots_ok(roots):
        return (isinstance(roots, list) and all(path_ok(p) for p in roots)
                and len(set(roots)) == len(roots))
    require(roots_ok(global_roots), 'invalid global roots')
    for name, unit in units.items():
        require(isinstance(name, str) and re.fullmatch(r'[A-Za-z0-9_.:-]{1,120}', name)
                and isinstance(unit, dict) and set(unit) == {'roots', 'needs'}
                and roots_ok(unit['roots']) and unit['roots'], 'invalid unit definition')
        deps = unit['needs']
        require(isinstance(deps, list) and all(isinstance(d, str) and d in units
                                             for d in deps)
                and len(set(deps)) == len(deps), 'unknown or duplicate dependency')
    universe = set(old) | set(new)
    changed = {p for p in universe if old.get(p) != new.get(p)}
    def expand(roots):
        found = set()
        for root in roots:
            a = {p for p in old if p == root or p.startswith(root + '/')}
            b = {p for p in new if p == root or p.startswith(root + '/')}
            require(a or b, 'declared root missing: ' + root)
            found.update(a | b)
        return found
    global_paths = expand(global_roots)
    direct = {name: expand(unit['roots']) for name, unit in units.items()}
    owned = global_paths | set().union(*direct.values())
    unknown = changed - owned
    done, active = {}, set()
    def visit(name):
        require(name not in active, 'dependency cycle')
        if name in done:
            return done[name]
        active.add(name)
        paths = direct[name] | global_paths
        for dep in units[name]['needs']:
            paths |= visit(dep)
        active.remove(name)
        done[name] = paths
        return paths
    def digest(tree, paths):
        rows = [[p, *tree[p]] if p in tree else [p, 'ABSENT'] for p in sorted(paths)]
        return 'sha256:' + hashlib.sha256(json.dumps(rows, ensure_ascii=True,
                   separators=(',', ':')).encode('ascii')).hexdigest()
    report = {}
    for name in sorted(units):
        paths = visit(name)
        reasons = []
        if unknown:
            reasons.append('unowned_changed_path')
        if changed & global_paths:
            reasons.append('global_input_changed')
        if changed & paths:
            reasons.append('transitive_input_changed')
        if any(tree[p][0] not in ('100644', '100755') for tree in (old, new)
               for p in paths if p in tree):
            reasons.append('unsupported_input_kind')
        report[name] = {'old': digest(old, paths), 'new': digest(new, paths),
                        'input_count': len(paths), 'reasons': reasons,
                        'action': 'RERUN' if reasons else 'CANDIDATE_REUSE'}
    return {'qualification': 'NONQUALIFYING_DECLARED_INPUT_CLOSURE',
            'source_completeness_proven': False, 'release_gate_authorized': False,
            'changed_paths': sorted(changed), 'unowned_changed_paths': sorted(unknown),
            'units': report}

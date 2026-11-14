"""Bounded diagnostic only: exact retained R6 A, never compiler acceptance."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import sys
import time


def digest(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def save(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')


def prepare(stage, out):
    original = json.loads((stage / 'manifest.json').read_text())
    assert len(original['certificates']) == 1
    for row in original['files']:
        assert digest(stage / 'src' / row['path']) == row['sha256']
    out.mkdir()
    shutil.copytree(stage / 'src', out / 'src')
    module = original['certificates'][0]
    row = next(r for r in original['files'] if r['module'] == module)
    text = (stage / 'src' / row['path']).read_bytes()
    marker = b'\nrun_cmd Lean.Elab.Command.liftCoreM (LexLeanCore.Runtime.decodeAndAdd #['
    footer = ('\nend ' + module + '\n').encode()
    assert text.endswith(footer)
    body = text[:-len(footer)]
    first = body.index(marker)
    header, commands = body[:first], body[first:]
    common = header[:header.index(b'namespace LexLeanCore.Runtime')]
    starts, at = [], 0
    while (at := commands.find(marker, at)) >= 0:
        starts.append(at)
        at += len(marker)
    groups, start = [], 0
    for i, at in enumerate(starts):
        end = starts[i + 1] if i + 1 < len(starts) else len(commands)
        if at > start and end - start > 4 * 1024 * 1024:
            groups.append(commands[start:at])
            start = at
    groups.append(commands[start:])
    assert groups and starts and b''.join(groups) == commands
    parts = []
    previous = None
    for i, group in enumerate(groups):
        name = 'LexLeanPreserve.Parts.' + module.removeprefix('LexLeanPreserve.') + f'_Part{i}'
        source = (header if previous is None else f'import {previous}\n'.encode() + common) + group + footer
        path = name.replace('.', '/') + '.lean'
        target = out / 'src' / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(source)
        parts.append({'module': name, 'path': path, 'sha256': digest(target)})
        previous = name
    wrapper = out / 'src' / row['path']
    wrapper.write_text(f'import {previous}\n')
    wrapped = dict(row, sha256=digest(wrapper))
    files = []
    for old in original['files']:
        files.extend(parts + [wrapped] if old['module'] == module else [old])
    save(out / 'manifest.json', dict(original, files=files,
         certificates=[p['module'] for p in parts] + [module],
         original_manifest_sha256=digest(stage / 'manifest.json'),
         original_certificate_sha256=row['sha256'], command_count=len(starts),
         partition_source_checkpoint='abb7ad9728a68d54d83977f35a25797756ba4bab'))


def group_rss(pid):
    entries = {}
    for path in Path('/proc').glob('[0-9]*/status'):
        try:
            values = dict(line.split(':', 1) for line in path.read_text().splitlines() if ':' in line)
            entries[int(path.parent.name)] = (int(values['PPid']), int(values.get('VmRSS', '0 kB').split()[0]))
        except (OSError, ValueError, KeyError):
            continue
    group = {pid}
    while True:
        more = {p for p, (parent, _) in entries.items() if parent in group}
        if more <= group:
            break
        group |= more
    return sum(entries[p][1] for p in group if p in entries)


def run(root, bin_dir, helper, retained, reuse_from=None):
    manifest = json.loads((root / 'manifest.json').read_text())
    for row in manifest['files']:
        assert digest(root / 'src' / row['path']) == row['sha256']
    toolchain_libraries = {str(p.relative_to(bin_dir.parent)): digest(p) for p in sorted((bin_dir.parent / 'lib').rglob('*')) if p.is_file()}
    assert toolchain_libraries, 'pinned library inventory must be nonempty'
    save(root / 'toolchain-libraries.json', toolchain_libraries)
    key = {'toolchain_libraries': digest(root / 'toolchain-libraries.json'), 'python': digest(sys.executable), 'manifest': digest(root / 'manifest.json'), 'runner': digest(Path(__file__)),
           'lean': digest(bin_dir / 'lean'), 'leanchecker': digest(bin_dir / 'leanchecker'),
           'audit_helper': digest(helper), 'retained_receipt': digest(retained / 'source-receipt.json')}
    logs = root / 'results'
    logs.mkdir(exist_ok=True)
    env = {'LEAN_PATH': str(root / 'build'), 'PATH': str(bin_dir) + os.pathsep + os.defpath,
           'LANG': 'C.UTF-8', 'LC_ALL': 'C.UTF-8', 'HOME': os.environ['HOME']}
    save(root / 'environment.json', env)
    key['environment'] = digest(root / 'environment.json')
    rows = {r['module']: r for r in manifest['files']}
    if reuse_from:
        old_manifest = json.loads((reuse_from / 'manifest.json').read_text())
        old_rows = {r['module']: r for r in old_manifest['files']}
        old_env = json.loads((reuse_from / 'environment.json').read_text())
        assert old_env['LEAN_PATH'] == str(reuse_from / 'build')
        assert {k: v for k, v in old_env.items() if k != 'LEAN_PATH'} == {k: v for k, v in env.items() if k != 'LEAN_PATH'}
        key['reused_environment_manifest'] = digest(reuse_from / 'manifest.json')

    def artifact_inventory(artifact):
        return {str(p.relative_to(root)): digest(p) for p in sorted(artifact.parent.glob(artifact.stem + '.*')) if p.is_file()}

    def child(label, argv, artifact=None, dependencies=None, silent=False):
        receipt = logs / (label + '.json')
        if receipt.exists():
            old = json.loads(receipt.read_text())
            if old.get('exit') == 0 and old.get('reason') is None and old.get('key') == key and old.get('argv') == argv and old.get('dependencies') == dependencies and old.get('silent') == silent:
                if artifact is None or (artifact.exists() and artifact_inventory(artifact) == old.get('artifact_inventory')):
                    assert digest(logs / (label + '.log')) == old['log_sha256']
                    print('reuse', label, flush=True)
                    return
        log = logs / (label + '.log')
        started = time.monotonic()
        peak = 0
        reason = None
        with log.open('wb') as output:
            proc = subprocess.Popen(argv, cwd=root, env=env, stdout=output, stderr=subprocess.STDOUT, start_new_session=True)
            while proc.poll() is None:
                peak = max(peak, group_rss(proc.pid))
                if time.monotonic() - started > 600:
                    reason = '600-second child timeout'
                elif peak > 9 * 1024 * 1024:
                    reason = '9-GiB process-tree RSS limit'
                elif log.stat().st_size > 16 * 1024 * 1024:
                    reason = '16-MiB child output limit'
                if reason:
                    os.killpg(proc.pid, signal.SIGKILL)
                    break
                time.sleep(0.25)
            code = proc.wait()
        if log.stat().st_size > 16 * 1024 * 1024:
            reason = '16-MiB child output limit'
        if silent and log.stat().st_size:
            reason = 'non-silent compilation'
        record = {'dependencies': dependencies, 'silent': silent, 'key': key, 'argv': argv, 'exit': code, 'reason': reason,
                  'seconds': time.monotonic() - started, 'peak_process_tree_rss_kib': peak,
                  'log_sha256': digest(log), 'artifact_inventory': artifact_inventory(artifact) if code == 0 and artifact else None}
        save(receipt, record)
        print(label, code, round(record['seconds'], 2), reason, flush=True)
        if code or reason:
            raise RuntimeError(f'{label} failed; retained {receipt}')

    def adopt_environment(row, target, compiled):
        # Reuse only the successfully elaborated, unchanged environment, never
        # a prior certificate or proof acceptance. Preserve its original receipt.
        old_row = old_rows[row['module']]
        assert row == old_row
        assert row['module'] not in old_manifest['certificates']
        assert row['module'] != old_manifest['audit']
        assert digest(reuse_from / 'src' / row['path']) == row['sha256']
        label = 'compile-' + row['module']
        old_receipt = reuse_from / 'results' / (label + '.json')
        old = json.loads(old_receipt.read_text())
        assert old['key']['runner'] == 'a5dfe29afbbce3663e288174e2dcbea4ab9dad201ab502fc947b8bc45ddc7cd1'
        assert old['key']['manifest'] == key['reused_environment_manifest']
        assert old['key']['environment'] == digest(reuse_from / 'environment.json')
        for field in ['lean', 'leanchecker', 'toolchain_libraries']:
            assert old['key'][field] == key[field]
        assert old['exit'] == 0 and old['reason'] is None and old['silent'] is True
        assert old['dependencies'] == compiled
        old_log = reuse_from / 'results' / (label + '.log')
        assert old_log.stat().st_size == 0 and digest(old_log) == old['log_sha256']
        old_target = reuse_from / 'build' / Path(row['path']).with_suffix('.olean')
        assert old['argv'] == [str(bin_dir / 'lean'), '-R', str(reuse_from / 'src'), '-o', str(old_target), str(reuse_from / 'src' / row['path'])]
        inventory = {str(p.relative_to(reuse_from)): digest(p) for p in sorted(old_target.parent.glob(old_target.stem + '.*')) if p.is_file()}
        assert old_target.is_file() and inventory == old['artifact_inventory']
        for relative, sha in inventory.items():
            destination = root / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(reuse_from / relative, destination)
            assert digest(destination) == sha
        assert artifact_inventory(target) == inventory
        save(logs / ('adopt-' + row['module'] + '.json'), {
            'event': 'adopted-existing-environment-elaboration',
            'source_receipt_sha256': digest(old_receipt), 'source_receipt': old,
            'target_key': key, 'artifact_inventory': inventory})
        print('adopt-environment', row['module'], flush=True)

    compiled = {}
    for row in manifest['files']:
        if row['module'] == manifest['audit']:
            continue
        target = root / 'build' / Path(row['path']).with_suffix('.olean')
        target.parent.mkdir(parents=True, exist_ok=True)
        if reuse_from and row['module'] not in manifest['certificates']:
            adopt_environment(row, target, dict(compiled))
        else:
            child('compile-' + row['module'], [str(bin_dir / 'lean'), '-R', str(root / 'src'), '-o', str(target), str(root / 'src' / row['path'])], target, dict(compiled), silent=True)
        compiled[row['module']] = artifact_inventory(target)
    expected_outputs = {path for outputs in compiled.values() for path in outputs}
    actual_outputs = {str(p.relative_to(root)) for p in (root / 'build').rglob('*') if p.is_file()}
    assert expected_outputs == actual_outputs, 'unaccounted compiled artifact'
    for module in manifest['certificates']:
        child('replay-' + module, [str(bin_dir / 'leanchecker'), module], dependencies=compiled)
    audit = root / 'src' / rows[manifest['audit']]['path']
    child('audit-lean', [str(bin_dir / 'lean'), '-R', str(root / 'src'), str(audit)], dependencies=compiled)
    child('audit-policies', [str(helper), str(retained), str(root), str(logs / 'audit-lean.log')], dependencies=dict(compiled, audit_log=digest(logs / 'audit-lean.log')))
    save(root / 'outcome.json', {'key': key, 'status': 'R6-A-elaboration-replay-audit-passed', 'compiler_accepted': False,
                               'not_tested': ['certificates B and E', 'other roots', 'full production export', 'full compiler qualification']})


if __name__ == '__main__':
    if sys.argv[1] == 'prepare':
        prepare(Path(sys.argv[2]).resolve(), Path(sys.argv[3]).resolve())
    elif sys.argv[1] == 'run':
        run(*(Path(a).resolve() for a in sys.argv[2:]))
    else:
        raise ValueError('prepare or run required')

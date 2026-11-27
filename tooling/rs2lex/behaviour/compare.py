"""Compare the translated functions with the compiler, case by case: python3 compare.py OUT_DIR

A case where Rust panics is counted apart: the translation does not keep panic paths (it goes on with a default
value), so a panic is a known difference, not a finding. Prints one line per target that differs, with an example,
and exits non-zero when any case differs."""
import collections
import glob
import json
import os
import sys

work = os.path.join(sys.argv[1], 'behaviour')
targets = json.load(open(os.path.join(work, 'targets.json'), encoding='utf-8'))
cases = {}
for line in open(os.path.join(work, 'cases.jsonl'), encoding='utf-8'):
    c = json.loads(line); cases[c['id']] = c


def read(lines):
    out = {}
    for line in lines:
        if '\t' in line:
            i, v = line.rstrip('\n').split('\t', 1)
            if i.isdigit(): out[int(i)] = v
    return out


rust = {}
for f in glob.glob(os.path.join(work, 'out-rust', '*.txt')): rust.update(read(open(f, encoding='utf-8')))
lean = read(open(os.path.join(work, 'out-lean.txt'), encoding='utf-8'))
per = collections.defaultdict(collections.Counter)
example = {}
for i, c in cases.items():
    if i not in rust: per[c['t']]['no Rust run'] += 1; continue
    r, l = rust[i], lean.get(i, 'MISSING')
    if r == 'PANIC': per[c['t']]['Rust panics'] += 1
    elif r == l: per[c['t']]['same'] += 1
    else:
        per[c['t']]['differ'] += 1
        example.setdefault(c['t'], (c['args'], r, l))
total = collections.Counter()
for t in per.values(): total.update(t)
print(f"{len(per)} targets, {sum(total.values())} cases: " + ', '.join(f'{k} {v}' for k, v in sorted(total.items())))
bad = sorted(t for t, n in per.items() if n['differ'])
for t in bad:
    args, r, l = example[t]
    print(f"  {targets[t]['key']} ({targets[t]['file']}): {per[t]['differ']} differ; e.g. {json.dumps(args, ensure_ascii=False)[:120]}\n"
          f"      rust {r[:140]}\n      lean {l[:140]}")
sys.exit(1 if bad else 0)

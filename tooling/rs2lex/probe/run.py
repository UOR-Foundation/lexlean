"""Translate the probe crate (probe/lib.rs) to the module `Probe`, and write the Lean driver and the outcomes Rust gives.

    python tooling/rs2lex/probe/run.py OUT_DIR

writes OUT_DIR/Probe.lex.tex, OUT_DIR/lean/ProbeMain.lean, and OUT_DIR/probe_expected.txt; `probe.sh` builds and runs
the driver and compares its output with the expected file line by line. Every probe function must translate."""
import os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))
import translate  # noqa: E402

out = os.path.abspath(sys.argv[1])
os.makedirs(os.path.join(out, 'lean'), exist_ok=True)
rows, text, _, _ = translate.run(HERE, module='Probe')
failed = [r for r in rows if r['status'] != 'ok']
for r in rows:
    print(f"{r['key']}: {r['status'][:100]}")
if failed:
    sys.exit(f'{len(failed)} probe functions did not translate')
open(os.path.join(out, 'Probe.lex.tex'), 'w', encoding='utf-8', newline='\n').write(text)
subprocess.run([sys.executable, os.path.join(HERE, 'cases.py')], cwd=out, check=True)

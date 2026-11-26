#!/usr/bin/env bash
# Run the probe: translate probe/lib.rs, build it with the repository's compiler, run it under Lean, and compare
# every line with the outcome Rust's own semantics give (probe/cases.py writes both the inputs and the expected lines).
#
#   tooling/rs2lex/probe.sh [OUT_DIR]          (OUT_DIR defaults to target/rs2lex-probe; LEXLEAN names the binary)
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../.." && pwd)"
out="${1:-$repo/target/rs2lex-probe}"; mkdir -p "$out"; out="$(cd "$out" && pwd)"
lexlean="${LEXLEAN:-$repo/target/release/lexlean}"
toolchain="$(cat "$repo/bootstrap/lean-toolchain")"

python3 "$here/probe/run.py" "$out"

# the probe module, with the shim and the repository's standard library
ws="$out/workspace"
bash "$here/workspace.sh" "$ws" "$out/Probe.lex.tex"
(cd "$ws/mech" && "$lexlean" lock >/dev/null && "$lexlean" check && "$lexlean" build)
built="$(ls -td "$ws/mech/.lexlean/build"/*/ | head -1)"

# the runtime the generated Lean imports, built from its own project
rt="$out/runtime"; rm -rf "$rt"; cp -r "$repo/language/runtime-1.2/library" "$rt"; rm -rf "$rt/.lexlean"
(cd "$rt" && "$lexlean" lock >/dev/null && "$lexlean" build >/dev/null)
rt_built="$(ls -td "$rt/.lexlean/build"/*/ | head -1)"

# a Lake project that builds the driver against the generated modules
lean="$out/lean"; rm -rf "$lean/gen"; mkdir -p "$lean/gen"
cp -r "$built/modules/." "$rt_built/modules/." "$lean/gen/"
echo "$toolchain" > "$lean/lean-toolchain"
cat > "$lean/lakefile.toml" <<EOF
name = "rs2lex_probe"
defaultTargets = ["probe"]

[[lean_lib]]
name = "Gen"
srcDir = "gen"
globs = ["LexLeanRuntime.+", "LexLeanStd.+", "Mech.+"]

[[lean_exe]]
name = "probe"
root = "ProbeMain"
EOF
(cd "$lean" && { lake build probe > "$out/lake.log" 2>&1 || { grep -E "^error" -A4 "$out/lake.log" | head -40; exit 1; }; }
  ./.lake/build/bin/probe > "$out/probe_lean.txt")
if diff -q "$out/probe_expected.txt" "$out/probe_lean.txt" >/dev/null; then
  echo "probe: all $(wc -l < "$out/probe_expected.txt") cases identical"
else
  diff "$out/probe_expected.txt" "$out/probe_lean.txt" | head -20
  echo "probe: the translation and Rust differ"; exit 1
fi

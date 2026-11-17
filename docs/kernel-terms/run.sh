#!/usr/bin/env bash
# Kernel-term production evidence for one project (SPEC.md §18.10):
#   1. stage   the project's generated Lean and the runtime modules it imports (cargo xtask kernel-terms stage);
#   2. compile every module with the pinned Lean, in dependency order;
#   3. export  the library: the producers' vocabulary and every project declaration (lean4export 3.1.0);
#   4. produce the kernel terms over that export, with no Lean (cargo xtask kernel-terms produce);
#   5. judge   library and produced suffix with Lean's own kernel, natively, and again with the
#              same kernel compiled to WebAssembly under Node (wasm.mjs), library resident;
#   6. plant   faults in the IR and judge them the same way (faults.mjs).
#
#   bash docs/kernel-terms/run.sh <lexlean.toml> <work dir>
#
# Environment:
#   XTASK        the xtask binary (default: target/release/xtask)
#   LEAN         the pinned Lean (default: lean on PATH; must report 4.32.2)
#   LEAN4EXPORT  lean4export 3.1.0 built with that Lean
#   CHECKER      the native checker: Lean 4.32.1's kernel over an export file, one argument for a whole file,
#                two (library, suffix) for a suffix over a resident library (foundry v2/in-tab-kernel/driver)
#   WASM_DIST    the directory holding lean-kernel.mjs and lean-kernel.wasm (foundry v2/in-tab-kernel)
# Writes, under <work>: library.ndjson, suffix.ndjson, parts/, production.json, native.txt, wasm.json, faults.json.
# summarize.mjs gathers the work dirs of several projects into one evidence record.
set -euo pipefail
project=$1
work=$2
here=$(cd "$(dirname "$0")" && pwd)
xtask=${XTASK:-$here/../../target/release/xtask}
lean=${LEAN:-lean}
"$lean" --version | grep -q "version 4.32.2" || { echo "LEAN is not Lean 4.32.2"; exit 1; }
rm -rf "$work"
mkdir -p "$work/oleans"
"$xtask" kernel-terms stage "$project" "$work"
export LEAN_PATH="$work/oleans:$("$lean" --print-prefix)/lib/lean"
modules=()
while IFS=$'\t' read -r module file; do
  target="$work/oleans/${file#src/}"
  target="${target%.lean}.olean"
  mkdir -p "$(dirname "$target")"
  "$lean" -R "$work/src" -o "$target" "$work/$file" > "$work/lean.out" 2>&1 || { cat "$work/lean.out"; exit 1; }
  modules+=("$module")
done < "$work/order.txt"
mapfile -t roots < "$work/roots.txt"
"$LEAN4EXPORT" "${modules[@]}" -- "${roots[@]}" > "$work/library.ndjson" 2> "$work/export.err" || { cat "$work/export.err"; exit 1; }
"$xtask" kernel-terms produce "$work" "$work/library.ndjson"
cat "$work/library.ndjson" "$work/suffix.ndjson" > "$work/whole.ndjson"
{
  echo "== whole"
  start=$(date +%s%N)
  "$CHECKER" "$work/whole.ndjson" || true
  echo "ms $(( ($(date +%s%N) - start) / 1000000 ))"
  echo "== suffix"
  "$CHECKER" "$work/library.ndjson" "$work/suffix.ndjson" || true
} > "$work/native.txt" 2>&1
node "$here/wasm.mjs" "$WASM_DIST" "$work"
node "$here/faults.mjs" "$WASM_DIST" "$work" "$xtask"

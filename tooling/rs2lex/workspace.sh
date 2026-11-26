#!/usr/bin/env bash
# Make a LexLean workspace around translated modules: the project `mech` (prefix `Mech`) holding the shim modules and
# the given .lex.tex files, beside a copy of the repository's standard library, which the translation imports.
#
#   tooling/rs2lex/workspace.sh WS_DIR MODULE.lex.tex...
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../.." && pwd)"
ws="$1"; shift
rm -rf "$ws"; mkdir -p "$ws/mech/src"
cp "$here/shim/AxiomOutput.lex.tex" "$here/shim/RustStd.lex.tex" "$@" "$ws/mech/src/"
cp -r "$repo/stdlib" "$ws/stdlib"; rm -rf "$ws/stdlib/.lexlean" "$ws/stdlib/expected"
printf 'spec = "lexlean/workspace/1"\nmembers = ["mech", "stdlib"]\n' > "$ws/lexlean-workspace.toml"
entrypoints="$(cd "$ws/mech/src" && ls *.lex.tex | LC_ALL=C sort | sed 's|.*|"src/&"|' | paste -sd, - | sed 's/,/, /g')"
cat > "$ws/mech/lexlean.toml" <<EOF
spec = "lexlean/project/2"
name = "mech"
language = "1.2"
module_prefix = "Mech"
source_roots = ["src"]
entrypoints = [$entrypoints]
build_root = ".lexlean"
lockfile = "lexlean.lock"
lean_workspace = "."
lean_toolchain = "$(cat "$repo/bootstrap/lean-toolchain")"

[[lexicon_source]]
package = "lexlean.std.nat"
kind = "builtin"

[[source_package]]
package = "std"
version = "1.0.0"
kind = "path"
path = "stdlib"

[limits]
max_file_bytes = 33554432
max_total_source_bytes = 536870912
max_primitive_atoms = 4000000
max_token_lattice_edges = 4000000
max_parse_states = 4000000
max_ir_nodes = 4000000
max_scope_depth = 1024
max_import_depth = 128
max_diagnostics = 256
max_child_output_bytes = 268435456
child_timeout_ms = 600000
EOF
printf 'name = "mech_host"\nversion = "0.1.0"\n' > "$ws/mech/lakefile.toml"
cp "$repo/bootstrap/lean-toolchain" "$repo/bootstrap/lake-manifest.json" "$ws/mech/"

#!/usr/bin/env bash
# Check a translation with the repository's own compiler: a workspace around OUT_DIR/Crate.lex.tex (workspace.sh),
# then `lexlean lock` and `lexlean check`.
#
#   tooling/rs2lex/check.sh [OUT_DIR]          (OUT_DIR defaults to target/rs2lex; LEXLEAN names the binary)
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../.." && pwd)"
out="$(cd "${1:-$repo/target/rs2lex}" && pwd)"
lexlean="${LEXLEAN:-$repo/target/release/lexlean}"
bash "$here/workspace.sh" "$out/workspace" "$out/Crate.lex.tex"
cd "$out/workspace/mech"
"$lexlean" lock
"$lexlean" check

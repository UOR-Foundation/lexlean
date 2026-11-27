#!/usr/bin/env bash
# The behaviour harness: run translated compiler functions in Lean and the same functions in the compiler, on the same
# cases, and compare (behaviour/gen.py says what is a target and how a value is printed).
#
#   tooling/rs2lex/behaviour.sh OUT_DIR CRATE_WORKSPACE [MODULE...]
#
# OUT_DIR holds a translation of CRATE_WORKSPACE/crates/lexlean/src by translate.py (Crate.lex.tex, fns.json); MODULEs
# keep the targets to those top-level modules. LEXLEAN names the compiler binary; CARGO_TARGET_DIR, if set, is where
# the crate's test build is kept between runs.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../.." && pwd)"
out="$(cd "$1" && pwd)"; crate="$(cd "$2" && pwd)"; shift 2
lexlean="${LEXLEAN:-$repo/target/release/lexlean}"
toolchain="$(cat "$repo/bootstrap/lean-toolchain")"
work="$out/behaviour"

python3 "$here/behaviour/gen.py" "$out" "$crate/crates/lexlean/src" "$@"

# -- Lean: the translation, built, and a driver over it
ws="$work/workspace"
bash "$here/workspace.sh" "$ws" "$out/Crate.lex.tex"
(cd "$ws/mech" && "$lexlean" lock >/dev/null && "$lexlean" check >/dev/null && "$lexlean" build >/dev/null)
built="$(ls -td "$ws/mech/.lexlean/build"/*/ | head -1)"
rt="$work/runtime"; rm -rf "$rt"; cp -r "$repo/language/runtime-1.2/library" "$rt"; rm -rf "$rt/.lexlean"
(cd "$rt" && "$lexlean" lock >/dev/null && "$lexlean" build >/dev/null)
rt_built="$(ls -td "$rt/.lexlean/build"/*/ | head -1)"
lean="$work/lean"; rm -rf "$lean/gen"; mkdir -p "$lean/gen"
cp -r "$built/modules/." "$rt_built/modules/." "$lean/gen/"
echo "$toolchain" > "$lean/lean-toolchain"
cat > "$lean/lakefile.toml" <<EOF
name = "rs2lex_behaviour"
defaultTargets = ["behave"]

[[lean_lib]]
name = "Gen"
srcDir = "gen"
globs = ["LexLeanRuntime.+", "LexLeanStd.+", "Mech.+"]

[[lean_exe]]
name = "behave"
root = "BehaviourMain"
EOF
(cd "$lean" && { lake build behave > "$work/lake.log" 2>&1 || { grep -E "^error" -A4 "$work/lake.log" | head -40; exit 1; }; }
  ./.lake/build/bin/behave "$work/cases.jsonl" > "$work/out-lean.txt")

# -- Rust: a scratch copy of the crate's workspace, each probe module appended to its source file
scratch="$work/crate"; rm -rf "$scratch"; mkdir -p "$scratch"
(cd "$crate" && tar --exclude=./target --exclude=./.git --exclude='./.lexlean' -cf - .) | (cd "$scratch" && tar -xf -)
cp "$work/rust/harness.rs" "$scratch/crates/lexlean/src/behaviour_harness.rs"
printf '\n#[cfg(test)]\npub(crate) mod behaviour_harness;\n' >> "$scratch/crates/lexlean/src/lib.rs"
python3 - "$work" "$scratch/crates/lexlean/src" <<'PY'
import json, os, sys
work, src = sys.argv[1], sys.argv[2]
for file, probe in json.load(open(os.path.join(work, 'rust', 'index.json'))).items():
    with open(os.path.join(src, file), 'a', encoding='utf-8') as f: f.write(open(os.path.join(work, 'rust', probe), encoding='utf-8').read())
PY
rm -rf "$work/out-rust"; mkdir -p "$work/out-rust"
(cd "$scratch" && BEHAVIOUR_CASES="$work/cases.jsonl" BEHAVIOUR_OUT="$work/out-rust" \
  cargo test -q -p lexlean --lib behaviour_probe -- --test-threads=4 > "$work/cargo.log" 2>&1 || { grep -E "^error" -A8 "$work/cargo.log" | head -60; exit 1; })

python3 "$here/behaviour/compare.py" "$out"

#!/usr/bin/env bash
# `just relock-identity` (SPEC.md §9.2, §9.4): what a change that moves the
# compiler-semantics ID (§8.4, §21.2) moves in turn, written from the tree.
#
#   1. Builds the compiler of HEAD and reads its target descriptors
#      (`lexlean production targets`), then writes the Rust backend records of
#      `language/semantics-1.2.toml` (`cargo xtask semantics-records --write`,
#      §17.16), which the ID and the descriptors cover, builds the compiler
#      again, and reads the descriptors it computes.
#   2. Relocks every tracked `lexlean.lock` and keeps the new lock only when
#      the one line it changed is `compiler_semantics`, moving from the ID its
#      language had at HEAD (the value most locks of that language carry).
#      A lock whose other content moved, a lock that does not resolve, and a
#      fixture that pins a deliberately different ID are put back.
#   3. Rewrites `lexlean::LANGUAGE_1_2_SEMANTICS_ID` (RP-10) to the new ID.
#   4. Repins, in every tracked production request, each target pin that
#      named its target's descriptor at HEAD to the new descriptor, every
#      target at once. A pin that named no descriptor of HEAD (a fixture's
#      deliberate one) is left as it is. Nothing is built for a request.
#
# It refuses a tree with tracked changes, so it can only write over its own
# output, and it writes nothing else: the `.lexlean/` build roots it creates
# are removed (those that existed before stay), and any other tracked file a
# step touched is put back. Run it before the other `-write` recipes, whose
# outputs embed these locks and pins; `regenerate.yml` does.
set -euo pipefail

root=$(git rev-parse --show-toplevel)
cd "$root"

if [ -n "$(git status --porcelain --untracked-files=no)" ]; then
    echo "relock-identity: the tree has tracked changes; commit or stash them first" >&2
    exit 1
fi
build_roots() {
    git ls-files --others --ignored --exclude-standard --directory \
        | { grep -E '(^|/)\.lexlean/$' || true; } | sort
}
before_roots=$(mktemp)
build_roots > "$before_roots"

target_dir=$(cargo metadata --format-version 1 --no-deps | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')
lexlean="$target_dir/debug/lexlean"
head_targets=$(mktemp)
new_targets=$(mktemp)
cargo build --quiet -p lexlean --bin lexlean
"$lexlean" production targets > "$head_targets"
cargo xtask semantics-records --write
cargo build --quiet -p lexlean --bin lexlean
"$lexlean" production targets > "$new_targets"

lib=crates/lexlean/src/lib.rs
mapfile -t locks < <(git ls-files -- '*lexlean.lock' 'lexlean.lock')

# The ID each language's locks carry at HEAD: the most common value.
declare -A head_id
for language in 1.0 1.1 1.2; do
    head_id[$language]=$(for lock in "${locks[@]}"; do
        if grep -qx "language = \"$language\"" "$lock"; then
            sed -n 's/^compiler_semantics = "\([0-9a-f]*\)"$/\1/p' "$lock"
        fi
    done | sort | uniq -c | sort -rn | awk 'NR == 1 { print $2 }')
done

relocked=0
kept_back=0
new_1_2=""
for lock in "${locks[@]}"; do
    dir=$(dirname "$lock")
    (cd "$dir" && "$lexlean" lock > /dev/null 2>&1) || true
    if git diff --quiet -- "$lock"; then
        continue
    fi
    changed=$(git diff -U0 -- "$lock" | grep '^[-+]' | grep -v '^\(---\|+++\)' || true)
    language=$(sed -n 's/^language = "\(.*\)"$/\1/p' "$lock")
    old=$(printf '%s\n' "$changed" | sed -n 's/^-compiler_semantics = "\([0-9a-f]*\)"$/\1/p')
    new=$(printf '%s\n' "$changed" | sed -n 's/^+compiler_semantics = "\([0-9a-f]*\)"$/\1/p')
    if [ "$(printf '%s\n' "$changed" | wc -l)" -eq 2 ] && [ -n "$old" ] && [ -n "$new" ] \
        && [ "$old" = "${head_id[$language]:-}" ]; then
        relocked=$((relocked + 1))
        if [ "$language" = "1.2" ]; then
            new_1_2=$new
        fi
    else
        git checkout --quiet -- "$lock"
        kept_back=$((kept_back + 1))
        echo "relock-identity: kept $lock (its relock moves more than the ID)"
    fi
done
echo "relock-identity: relocked $relocked lock(s), kept $kept_back"

if [ -n "$new_1_2" ]; then
    sed -i "s/\"${head_id[1.2]}\"/\"$new_1_2\"/" "$lib"
    grep -q "\"$new_1_2\"" "$lib" || { echo "relock-identity: $lib names no language-1.2 ID" >&2; exit 1; }
    echo "relock-identity: language 1.2 ID ${head_id[1.2]} -> $new_1_2"
fi

# The target pins of the production requests: a pin of HEAD's descriptor of
# its target moves to the new descriptor; any other pin is a fixture's own.
python3 - "$head_targets" "$new_targets" <<'PY'
import json, re, subprocess, sys
head = {t["id"]: t["digest"] for t in json.load(open(sys.argv[1]))["targets"]}
new = {t["id"]: t["digest"] for t in json.load(open(sys.argv[2]))["targets"]}
files = subprocess.run(["git", "ls-files", "--", "*request.json"],
                       capture_output=True, text=True, check=True).stdout.split()
for path in files:
    text = open(path).read()
    try:
        request = json.loads(text)
    except ValueError:
        continue
    if not str(request.get("spec", "")).startswith("lexlean/production-request/"):
        continue
    moved = text
    for target, old in head.items():
        if new.get(target, old) == old:
            continue
        pin = '"id":"%s","digest":"%s"' % (target, old)
        if pin in moved:
            moved = moved.replace(pin, '"id":"%s","digest":"%s"' % (target, new[target]))
            print(f"relock-identity: {path} pins {target} {new[target]} (was {old})")
    if moved != text:
        open(path, "w").write(moved)
PY

# Nothing else is written: build roots this run created are removed, and a
# tracked file outside the four kinds above is put back (the tree had no
# tracked change when it started).
comm -13 "$before_roots" <(build_roots) | while read -r created; do rm -rf "$created"; done
git diff --name-only \
    | { grep -v -E '(^|/)lexlean\.lock$|^crates/lexlean/src/lib\.rs$|request\.json$|^language/semantics-1\.2\.toml$' || true; } \
    | while read -r touched; do git checkout --quiet -- "$touched"; done
rm -f "$before_roots" "$head_targets" "$new_targets"

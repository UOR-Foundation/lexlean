#!/usr/bin/env bash
# `just relock-identity` (SPEC.md §9.2, §9.4): what a change that moves the
# compiler-semantics ID (§8.4, §21.2) moves in turn, written from the tree.
#
#   1. Builds the compiler of the tree.
#   2. Relocks every tracked `lexlean.lock` and keeps the new lock only when
#      the one line it changed is `compiler_semantics`, moving from the ID its
#      language had at HEAD (the value most locks of that language carry).
#      A lock whose other content moved, a lock that does not resolve, and a
#      fixture that pins a deliberately different ID are put back.
#   3. Rewrites `lexlean::LANGUAGE_1_2_SEMANTICS_ID` (RP-10) to the new ID.
#   4. Repins every rust target descriptor digest a production request under
#      `tests/` pins (`*/project/request.json`), from the compiler's LLC0203
#      report of the digest it computed.
#
# It writes nothing else: build roots it created are removed, and any other
# tracked file a step touched is put back. Run it before the other `-write`
# recipes, whose outputs embed these locks and pins; `regenerate.yml` does.
set -euo pipefail

root=$(git rev-parse --show-toplevel)
cd "$root"

cargo build --quiet -p lexlean --bin lexlean
target_dir=$(cargo metadata --format-version 1 --no-deps | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')
lexlean="$target_dir/debug/lexlean"

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

# The descriptor digests a production request pins: the compiler reports the
# one it computed when the pin differs (LLC0203).
for request in $(git ls-files -- 'tests/*/project/request.json'); do
    report=$(cd "$(dirname "$request")" && "$lexlean" production build --request request.json 2>&1 || true)
    { printf '%s\n' "$report" \
        | grep -o 'target `[a-z-]*` has descriptor digest [0-9a-f]*, the request pins [0-9a-f]*' || true; } \
        | sort -u \
        | while read -r _ target _ _ _ computed _ _ _ pinned; do
            computed=${computed%,}
            sed -i "s/$pinned/$computed/g" "$request"
            echo "relock-identity: $request pins $target $computed (was $pinned)"
        done
done

# Nothing else is written: build roots these runs created are removed, and a
# tracked file outside the three kinds above is put back.
git ls-files --others --ignored --exclude-standard --directory \
    | { grep -E '(^|/)\.lexlean/$' || true; } \
    | while read -r created; do rm -rf "$created"; done
git diff --name-only \
    | { grep -v -E '(^|/)lexlean\.lock$|^crates/lexlean/src/lib\.rs$|/project/request\.json$' || true; } \
    | while read -r touched; do git checkout --quiet -- "$touched"; done

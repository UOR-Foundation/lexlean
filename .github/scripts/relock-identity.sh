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
# output. Its write set is the tracked `lexlean.lock` files,
# `crates/lexlean/src/lib.rs`, `language/semantics-1.2.toml`, and tracked
# production requests; the `.lexlean/` build roots it creates are removed,
# a root that existed before keeps its contents (the `.lock` helper a lock
# run may add to it is removed), and any other tracked file a step touched is
# put back. Run it before the other `-write` recipes, whose
# outputs embed these locks and pins; `regenerate.yml` does.
set -euo pipefail

root=$(git rev-parse --show-toplevel)
cd "$root"
source .github/scripts/semantics-id.sh

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
# A root that exists already keeps its contents; `lexlean lock` may add its
# mutation-lock helper (`.lexlean/.lock`) to one, which is removed after.
without_helper=$(mktemp)
while read -r existing; do
    # A dangling symlink is a helper that exists too.
    [ -e "$existing.lock" ] || [ -L "$existing.lock" ] || printf '%s\n' "$existing"
done < "$before_roots" > "$without_helper"

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
            semantics_ids < "$lock"
        fi
    done | sort | uniq -c | sort -rn | awk 'NR == 1 { print $2 }')
    if [ -z "${head_id[$language]}" ] && grep -lqx "language = \"$language\"" "${locks[@]}"; then
        echo "relock-identity: no language-$language lock carries a compiler-semantics ID this step reads" >&2
        exit 1
    fi
done

relocked=0
kept_back=0
new_1_2=""
for lock in "${locks[@]}"; do
    dir=$(dirname "$lock")
    # A lock that does not resolve is expected of some fixtures, and its lock
    # is put back below; a failure to take the build root's mutation lock
    # (`.lock`) is not, and stops the run before any cleanup.
    if ! report=$(cd "$dir" && "$lexlean" lock 2>&1); then
        if printf '%s\n' "$report" | grep -q '/\.lock: '; then
            printf 'relock-identity: %s: the build root lock failed; nothing cleaned up\n%s\n' "$dir" "$report" >&2
            exit 1
        fi
    fi
    if git diff --quiet -- "$lock"; then
        continue
    fi
    changed=$(git diff -U0 -- "$lock" | grep '^[-+]' | grep -v '^\(---\|+++\)' || true)
    language=$(sed -n 's/^language = "\(.*\)"$/\1/p' "$lock")
    old=$(printf '%s\n' "$changed" | semantics_ids -)
    new=$(printf '%s\n' "$changed" | semantics_ids +)
    # A moved ID this step cannot read is its own defect, not a lock to keep.
    if printf '%s\n' "$changed" | grep -q '^[-+]compiler_semantics = ' && { [ -z "$old" ] || [ -z "$new" ]; }; then
        echo "relock-identity: $lock moves a compiler-semantics ID this step does not read" >&2
        exit 1
    fi
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
moved_ids = {target: new[target] for target, old in head.items() if new.get(target, old) != old}

def style(text):
    """The json.dumps arguments that reproduce `text` byte for byte, or None."""
    data = json.loads(text)
    trail = "\n" if text.endswith("\n") else ""
    for indent in (None, 2, 4, "\t", 1, 3):
        for separators in ((",", ":"), (", ", ": "), (",", ": ")):
            for sort_keys in (False, True):
                for ascii_ in (False, True):
                    options = dict(indent=indent, separators=separators,
                                   sort_keys=sort_keys, ensure_ascii=ascii_)
                    if json.dumps(data, **options) + trail == text:
                        return options, trail
    return None

files = subprocess.run(["git", "ls-files", "--", "*request.json"],
                       capture_output=True, text=True, check=True).stdout.split()
for path in files:
    text = open(path, encoding="utf-8").read()
    try:
        request = json.loads(text)
    except ValueError:
        continue
    if not isinstance(request, dict) or not str(request.get("spec", "")).startswith(
            "lexlean/production-request/"):
        continue
    changes = []
    for root in request.get("roots") or []:
        for pin in (root.get("targets") or []) if isinstance(root, dict) else []:
            if not isinstance(pin, dict):
                continue
            target = pin.get("id")
            if target in moved_ids and pin.get("digest") == head[target]:
                pin["digest"] = moved_ids[target]
                changes.append((target, head[target], moved_ids[target]))
    if not changes:
        continue
    written = style(text)
    if written is None:
        sys.exit(f"relock-identity: {path} pins a moved descriptor, but its JSON layout is "
                 "not one this step reproduces; repin it by hand")
    options, trail = written
    with open(path, "w", encoding="utf-8") as out:
        out.write(json.dumps(request, **options) + trail)
    for target, old, digest in changes:
        print(f"relock-identity: {path} pins {target} {digest} (was {old})")
PY

# Nothing else is written: build roots this run created are removed, and a
# tracked file outside the four kinds above is put back (the tree had no
# tracked change when it started).
comm -13 "$before_roots" <(build_roots) | while read -r created; do rm -rf "$created"; done
# The helper is removed only where this run created it: a regular file in a
# root that had none.
while read -r existing; do
    if [ -f "$existing.lock" ] && [ ! -L "$existing.lock" ]; then rm -f "$existing.lock"; fi
done < "$without_helper"
git diff --name-only \
    | { grep -v -E '(^|/)lexlean\.lock$|^crates/lexlean/src/lib\.rs$|request\.json$|^language/semantics-1\.2\.toml$' || true; } \
    | while read -r touched; do git checkout --quiet -- "$touched"; done
rm -f "$before_roots" "$without_helper" "$head_targets" "$new_targets"

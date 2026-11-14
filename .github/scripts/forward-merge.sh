#!/usr/bin/env bash
# The forward merge of a release line (SPEC.md §9.4): the pushed head of
# `release/vX.Y` merged, by merge commit, into the head of `release/vX.(Y+1)`
# on the branch `forward/vXY-to-vX(Y+1)-<head8>`, pushed, with its pull request.
#
# A conflict in a generated projection or a lock takes the next line's side:
# `regenerate.yml` writes those files again from the merged tree. While the
# pushed line has no `spec/` (its SPEC.md and register are hand-written), what
# it changed in SPEC.md and `model/ids.toml` is carried into the next line's
# modules by `cargo xtask spec port --merged … --register …`, since the
# projections are written from the modules and would drop it. Any other
# conflict, and a port that refuses, is committed as the next line's side and
# the pull request is opened as a draft naming the paths, for an agent to
# finish. A newer forward branch of the same lines closes the older ones.
#
# Inputs, from the environment: REPO, LINE or GITHUB_REF_NAME (the line), the
# checkout (the pushed commit, full history, the workflow token), GITHUB_OUTPUT,
# GITHUB_STEP_SUMMARY, GITHUB_SERVER_URL. Outputs: branch, commit, clean.
set -euo pipefail

repo=${REPO:?REPO names owner/name}
summary=${GITHUB_STEP_SUMMARY:-/dev/null}
output=${GITHUB_OUTPUT:-/dev/null}
say() { printf '%s\n' "$1" | tee -a "$summary"; }

line=${LINE:-${GITHUB_REF_NAME:?LINE or GITHUB_REF_NAME names the pushed line}}
if ! [[ "$line" =~ ^release/v(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$ ]]; then
  say "\`$line\` is not a release line: nothing to carry forward."
  exit 0
fi
major=${BASH_REMATCH[1]}
minor=${BASH_REMATCH[2]}
next="release/v$major.$((minor + 1))"
if ! git ls-remote --exit-code --heads origin "$next" >/dev/null; then
  say "There is no \`$next\`: nothing to carry forward."
  exit 0
fi
git fetch --quiet origin "refs/heads/$next:refs/remotes/origin/$next"
source=$(git rev-parse HEAD)
if git merge-base --is-ancestor "$source" "origin/$next"; then
  say "\`$next\` already contains \`$source\`."
  exit 0
fi
prefix="forward/v$major$minor-to-v$major$((minor + 1))-"
branch="$prefix${source:0:8}"
if git ls-remote --exit-code --heads origin "$branch" >/dev/null; then
  say "\`$branch\` already exists: this head was carried forward before."
  exit 0
fi

base=$(git merge-base "origin/$next" "$source")
git checkout --quiet -B "$branch" "origin/$next"
git config user.name 'github-actions[bot]'
git config user.email '41898282+github-actions[bot]@users.noreply.github.com'
git merge --quiet --no-ff --no-commit "$source" || true
git rev-parse --verify --quiet MERGE_HEAD >/dev/null || {
  echo "::error::merging $source into $next did not start"
  exit 1
}

# Only what a writer of §9.2 or `cargo metadata` writes from the merged tree.
# What a writer reads (an export's `roots.json`, a measured row of
# `tests/stack-bounds.json`) is authored, and a conflict in it is listed.
generated() {
  case "$1" in
    SPEC.md | model/ids.toml | model/errors.toml | model/ledger.toml | model/authorities.toml \
      | CONFORMANCE.md | ERRORS.md | Cargo.lock | lexlean.lock | */lexlean.lock | */expected/* \
      | tests/golden/*.txt | tests/production-export/*/packages/*) return 0 ;;
    *) return 1 ;;
  esac
}
unresolved=()
while IFS= read -r path; do
  [ -n "$path" ] || continue
  generated "$path" || unresolved+=("$path")
  if git cat-file -e "HEAD:$path" 2>/dev/null; then
    git checkout --quiet HEAD -- "$path"
  else
    git rm --quiet --cached --ignore-unmatch -- "$path"
    rm -f -- "$path"
  fi
done < <(git diff --name-only --diff-filter=U)

projections=(SPEC.md model/ids.toml model/errors.toml model/ledger.toml model/authorities.toml)
if ! git cat-file -e "$source:spec/src" 2>/dev/null; then
  # The projections the next line writes from its modules keep nothing of the
  # pushed line's own text but what the port carries into the modules.
  git checkout --quiet HEAD -- "${projections[@]}"
  scratch=$(mktemp -d)
  portable=true
  for path in SPEC.md model/ids.toml; do
    name=${path##*/}
    git show "$base:$path" >"$scratch/base-$name"
    git show "HEAD:$path" >"$scratch/next-$name"
    git show "$source:$path" >"$scratch/line-$name"
    if ! git merge-file -p "$scratch/next-$name" "$scratch/base-$name" "$scratch/line-$name" >"$scratch/$name"; then
      unresolved+=("$path (both lines changed one hunk)")
      portable=false
    fi
  done
  for path in model/errors.toml model/ledger.toml model/authorities.toml; do
    if ! git diff --quiet "$base" "$source" -- "$path"; then
      unresolved+=("$path (changed on $line; its module is edited by hand)")
    fi
  done
  if [ "$portable" = true ] && ! git diff --quiet "$base" "$source" -- SPEC.md model/ids.toml; then
    args=(--merged "$scratch/SPEC.md")
    git diff --quiet "$base" "$source" -- model/ids.toml || args+=(--register "$scratch/ids.toml")
    if ! cargo xtask spec port "${args[@]}" 2>&1 | tee "$scratch/port.txt"; then
      unresolved+=("spec/src (the port refused: $(tail -1 "$scratch/port.txt"))")
      git checkout --quiet HEAD -- spec/ "${projections[@]}"
    fi
  fi
  rm -rf "$scratch"
fi
# A lock that took the next line's side gains what the merged manifests add.
cargo metadata --format-version 1 >/dev/null
git add --all

clean=true
[ "${#unresolved[@]}" -eq 0 ] || clean=false
{
  printf 'Merge %s into %s\n\n' "$line" "$next"
  printf 'Forward merge of %s at %s (forward.yml).\n' "$line" "$source"
  if [ "$clean" = false ]; then
    printf '\nCommitted as the side of %s, to finish by hand:\n' "$next"
    printf -- '- %s\n' "${unresolved[@]}"
  fi
} >"${RUNNER_TEMP:-/tmp}/forward-message"
git commit --quiet --no-verify -F "${RUNNER_TEMP:-/tmp}/forward-message"
git push --quiet origin "HEAD:refs/heads/$branch"
commit=$(git rev-parse HEAD)
{
  echo "branch=$branch"
  echo "commit=$commit"
  echo "clean=$clean"
} >>"$output"

# An older forward pull request of these lines is superseded only when its
# branch holds nothing but what this run makes again: the bot's merge and the
# bot's regeneration on it. One carrying any other commit (a conflict finished
# by hand) stays open, linked from the new one, until its work is carried.
bot_pattern='41898282\+github-actions\[bot\]@users\.noreply\.github\.com'
superseded=()
kept=()
while read -r number ref; do
  [ -n "$number" ] || continue
  if ! git fetch --quiet origin "refs/heads/$ref:refs/remotes/origin/$ref"; then
    kept+=("$number")
    continue
  fi
  extra=$(git log --format='%ae %s' "origin/$ref" --not "origin/$next" "$source" |
    grep -v -x -E "$bot_pattern (Merge release/v[0-9.]+ into release/v[0-9.]+|Regenerate the written outputs of [0-9a-f]+)" || true)
  if [ -z "$extra" ]; then
    # The head inspected here is the lease of its deletion below.
    superseded+=("$number $ref $(git rev-parse "origin/$ref")")
  else
    kept+=("$number")
  fi
done < <(gh pr list --repo "$repo" --base "$next" --state open --json number,headRefName \
  --jq ".[] | select(.headRefName | startswith(\"$prefix\")) | select(.headRefName != \"$branch\") | \"\\(.number) \\(.headRefName)\"")

title="Forward-merge $line (${source:0:8}) into $next"
if [ "$clean" = true ]; then
  body="Merges \`$line\` at \`$source\` into \`$next\` (SPEC.md §9.4; opened by \`forward.yml\`). \`regenerate.yml\` writes the generated projections and locks from the merged tree, delivers them here, and dispatches the gate. Land it with a merge commit."
  draft=()
else
  body="Merges \`$line\` at \`$source\` into \`$next\` (SPEC.md §9.4; opened by \`forward.yml\`). These paths are committed as \`$next\`'s side and are finished by hand before this is ready; then push the head to \`regenerate/<name>\`:"$'\n\n'"$(printf -- '- `%s`\n' "${unresolved[@]}")"
  draft=(--draft)
fi
if [ "${#kept[@]}" -gt 0 ]; then
  body+=$'\n\n'"Older forward pull requests with work not yet in \`$next\`, left open: $(printf '#%s ' "${kept[@]}")"
fi
if url=$(gh pr create --repo "$repo" --base "$next" --head "$branch" "${draft[@]}" \
    --title "$title" --body "$body" 2>&1); then
  say "Opened $url"
else
  # The repository may not let the workflow token open pull requests. With
  # no replacement open, nothing older is closed.
  echo "::warning::no pull request opened ($url)"
  say "Open the pull request: $GITHUB_SERVER_URL/$repo/compare/$next...$branch"
  superseded=()
fi
[ "$clean" = true ] || say "Finish by hand: $(printf '`%s` ' "${unresolved[@]}")"
[ "${#kept[@]}" -eq 0 ] || say "Left open, with work not yet in \`$next\`: $(printf '#%s ' "${kept[@]}")"
# A branch is deleted only while its head is still the one inspected (a lease:
# a push to it since then refuses the delete), and its pull request is closed
# only once that delete succeeded. Any other is kept and listed.
moved=()
for older in "${superseded[@]}"; do
  read -r number ref sha <<<"$older"
  if git push --quiet --force-with-lease="refs/heads/$ref:$sha" origin ":refs/heads/$ref"; then
    gh pr close "$number" --repo "$repo" --comment "Superseded by $url." ||
      gh pr comment "$number" --repo "$repo" --body "Superseded by $url." ||
      echo "::warning::#$number not closed"
  else
    moved+=("$number")
  fi
done
if [ "${#moved[@]}" -gt 0 ]; then
  note="Older forward pull requests whose branch moved since this run read it, left open: $(printf '#%s ' "${moved[@]}")"
  say "$note"
  gh pr comment "$url" --repo "$repo" --body "$note" || true
fi

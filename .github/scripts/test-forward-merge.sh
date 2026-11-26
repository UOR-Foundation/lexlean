#!/usr/bin/env bash
# Behavioral tests of `forward-merge.sh` (SPEC.md §9.4, RP-14): each case
# builds two release lines in a scratch repository whose `origin` is a local
# bare repository, runs the script with `gh` and `cargo` replaced by recorders,
# and reads what it pushed, printed and asked `gh` to open.
#
#   clean       a merge without conflicts is committed and opened ready
#   generated   conflicts only in generated paths take the next line's side
#               and are opened ready, for `regenerate.yml` to write
#   source      a conflict in an authored path is committed as the next
#               line's side and opened as a draft that names it
#   lock        a listing of the conflicts that holds `index.lock` after its
#               output (what porcelain `git diff` does when it refreshes) is
#               finished before the index is written (#600)
#
# FORWARD_MERGE names the script under test (default: the one beside this
# file), so a planted copy can be run against these cases (VERIFICATION.md).
set -euo pipefail

here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
script=$(cd "$(dirname "${FORWARD_MERGE:-$here/forward-merge.sh}")" && pwd)/$(basename "${FORWARD_MERGE:-forward-merge.sh}")
real_git=$(command -v git)
root=$(mktemp -d)
trap 'rm -rf "$root"' EXIT
failures=0

# The scratch repositories read no configuration of the machine running them.
export GIT_CONFIG_NOSYSTEM=1
export GIT_CONFIG_GLOBAL="$root/gitconfig"
: >"$GIT_CONFIG_GLOBAL"
git config --global init.defaultBranch main
git config --global core.autocrlf false
git config --global user.name fixture
git config --global user.email fixture@localhost

fail() {
  printf 'not ok - %s: %s\n' "$case_name" "$1"
  failures=$((failures + 1))
}
expect() { # expect <description> <command...>
  local what=$1
  shift
  "$@" >/dev/null 2>&1 || fail "$what"
}
commit_all() { git add --all && git commit --quiet -m "$1"; }

# A recorder for `gh` (the pull request the script opens) and for `cargo`
# (`cargo metadata` relocks; the fixture has no manifest).
write_tools() {
  local bin=$1
  mkdir -p "$bin"
  cat >"$bin/gh" <<'EOF'
#!/usr/bin/env bash
printf '%s\0' "$@" >>"$FIXTURE_LOG/gh.$1-$2"
printf '\n' >>"$FIXTURE_LOG/gh.$1-$2"
case "$1 $2" in
  "pr create") echo "https://github.invalid/pull/1" ;;
esac
EOF
  cat >"$bin/cargo" <<'EOF'
#!/usr/bin/env bash
printf '%s\n' "$*" >>"$FIXTURE_LOG/cargo"
EOF
  chmod +x "$bin/gh" "$bin/cargo"
}

# The `git` of the lock case: a listing (`diff`, `diff-files`) holds
# `index.lock` for two seconds after its output, as porcelain `git diff` does
# when it refreshes a smudged index. A writer that starts while the listing is
# still read fails exactly as the job of #600 did.
write_locking_git() {
  local bin=$1
  cat >"$bin/git" <<EOF
#!/usr/bin/env bash
real=$(printf '%q' "$real_git")
args=("\$@")
sub=
i=0
while [ \$i -lt \${#args[@]} ]; do
  case "\${args[\$i]}" in
    -c | -C) i=\$((i + 2)) ;;
    -*) i=\$((i + 1)) ;;
    *) sub=\${args[\$i]}; break ;;
  esac
done
case "\$sub" in
  diff | diff-files)
    "\$real" "\$@"
    status=\$?
    lock="\$("\$real" rev-parse --git-dir)/index.lock"
    if (set -o noclobber; : >"\$lock") 2>/dev/null; then
      echo held >>"\$FIXTURE_LOG/lock"
      sleep 2
      rm -f "\$lock"
    fi
    exit \$status
    ;;
esac
exec "\$real" "\$@"
EOF
  chmod +x "$bin/git"
}

# setup <case>: origin with release/v0.4 and release/v0.5 from one base, and a
# clone at the v0.4 head; the case then commits to each line through `line`.
setup() {
  case_name=$1
  dir="$root/$1"
  mkdir -p "$dir/log"
  export FIXTURE_LOG="$dir/log"
  git init --quiet --bare "$dir/origin.git"
  git init --quiet "$dir/seed"
  cd "$dir/seed"
  mkdir -p spec/src src compiler/rust/pkg/src tests/case/expected
  echo spec >spec/src/Main.lex.tex
  printf 'fn main() {}\n' >src/main.rs
  printf 'base\n' >src/dropped.rs
  printf 'base\n' >notes.txt
  printf 'base\n' >CONFORMANCE.md
  printf 'base\n' >compiler/rust/pkg/src/lib.rs
  printf 'base\n' >tests/case/expected/out.json
  for i in $(seq 1 40); do printf 'base\n' >"compiler/rust/pkg/src/m$i.rs"; done
  commit_all base
  git branch release/v0.4
  git branch release/v0.5
  git push --quiet "$dir/origin.git" release/v0.4 release/v0.5
  cd "$dir"
  rm -rf seed
  git clone --quiet --no-checkout "$dir/origin.git" work
}
# on <line> <message> <commands...>: commit to a line of origin.
on() {
  local line=$1 message=$2
  shift 2
  (
    cd "$dir/work"
    git checkout --quiet "$line"
    git pull --quiet --ff-only origin "$line"
    "$@"
    commit_all "$message"
    git push --quiet origin "$line"
  )
}
# run [bin]: the script at the v0.4 head, as forward.yml runs it.
run() {
  local bin=${1:-$dir/bin}
  write_tools "$dir/bin"
  cd "$dir/work"
  git fetch --quiet origin
  git checkout --quiet --detach origin/release/v0.4
  source_head=$(git rev-parse HEAD)
  : >"$dir/log/output"
  set +e
  PATH="$bin:$dir/bin:$PATH" REPO=fixture/lexlean LINE=release/v0.4 \
    GITHUB_OUTPUT="$dir/log/output" GITHUB_STEP_SUMMARY="$dir/log/summary" \
    RUNNER_TEMP="$dir" GITHUB_SERVER_URL=https://github.invalid \
    bash "$script" >"$dir/log/stdout" 2>"$dir/log/stderr"
  status=$?
  set -e
  branch="forward/v04-to-v05-${source_head:0:8}"
}
output() { sed -n "s/^$1=//p" "$dir/log/output"; }
pushed() { git -C "$dir/origin.git" rev-parse --verify --quiet "refs/heads/$branch"; }
at() { git -C "$dir/origin.git" show "refs/heads/$branch:$1"; }
create_args() { tr '\0' '\n' <"$dir/log/gh.pr-create"; }
report() {
  if [ "$status" -ne 0 ]; then
    fail "the script exited $status: $(tail -3 "$dir/log/stderr" | tr '\n' ' ')"
  fi
  local before=$failures
  "$@"
  [ "$failures" -ne "$before" ] || [ "$status" -ne 0 ] || printf 'ok - %s\n' "$case_name"
}

checks_merge_commit() {
  local parents
  parents=$(git -C "$dir/origin.git" rev-list --parents -n 1 "refs/heads/$branch" | wc -w)
  [ "$parents" -eq 3 ] || fail "the forward branch head is not a merge of the two lines"
  git -C "$dir/origin.git" merge-base --is-ancestor "$source_head" "refs/heads/$branch" ||
    fail "the forward branch does not contain the v0.4 head"
  git -C "$dir/origin.git" merge-base --is-ancestor refs/heads/release/v0.5 "refs/heads/$branch" ||
    fail "the forward branch does not contain the v0.5 head"
  if git -C "$dir/origin.git" grep -q -E '^(<<<<<<<|>>>>>>>) ' "refs/heads/$branch"; then
    fail "the forward commit holds conflict markers"
  fi
}

# --- clean -----------------------------------------------------------------
case_clean() {
  setup clean
  on release/v0.4 "v0.4 notes" sh -c 'printf "base\nfrom v0.4\n" >notes.txt'
  on release/v0.5 "v0.5 main" sh -c 'printf "fn main() { v05() }\n" >src/main.rs'
  run
  report clean_checks
}
clean_checks() {
  pushed >/dev/null || { fail "no forward branch was pushed"; return; }
  checks_merge_commit
  [ "$(output clean)" = true ] || fail "clean=$(output clean), not true"
  [ "$(output branch)" = "$branch" ] || fail "branch output is not $branch"
  at notes.txt | grep -q 'from v0.4' || fail "v0.4's change is not carried"
  at src/main.rs | grep -q 'v05' || fail "v0.5's change is lost"
  [ -s "$dir/log/gh.pr-create" ] || { fail "no pull request was opened"; return; }
  if create_args | grep -qx -- --draft; then fail "a clean merge was opened as a draft"; fi
}

# --- generated -------------------------------------------------------------
case_generated() {
  setup generated
  on release/v0.4 "v0.4 outputs" sh -c 'for f in CONFORMANCE.md tests/case/expected/out.json compiler/rust/pkg/src/lib.rs; do echo v04 >"$f"; done'
  on release/v0.5 "v0.5 outputs" sh -c 'for f in CONFORMANCE.md tests/case/expected/out.json compiler/rust/pkg/src/lib.rs; do echo v05 >"$f"; done'
  run
  report generated_checks
}
generated_checks() {
  pushed >/dev/null || { fail "no forward branch was pushed"; return; }
  checks_merge_commit
  [ "$(output clean)" = true ] || fail "generated-only conflicts are not clean (clean=$(output clean))"
  local path
  for path in CONFORMANCE.md tests/case/expected/out.json compiler/rust/pkg/src/lib.rs; do
    [ "$(at "$path")" = v05 ] || fail "$path does not take v0.5's side"
  done
  if create_args | grep -qx -- --draft; then fail "generated-only conflicts were opened as a draft"; fi
}

# --- source ----------------------------------------------------------------
case_source() {
  setup source
  on release/v0.4 "v0.4 source" sh -c 'echo "fn main() { v04() }" >src/main.rs; echo v04 >CONFORMANCE.md; echo v04 >src/dropped.rs'
  on release/v0.5 "v0.5 source" sh -c 'echo "fn main() { v05() }" >src/main.rs; echo v05 >CONFORMANCE.md; git rm --quiet src/dropped.rs'
  run
  report source_checks
}
source_checks() {
  pushed >/dev/null || { fail "no forward branch was pushed"; return; }
  checks_merge_commit
  [ "$(output clean)" = false ] || fail "an authored conflict is reported clean"
  at src/main.rs | grep -q v05 || fail "src/main.rs is not committed as v0.5's side"
  if at src/dropped.rs >/dev/null 2>&1; then fail "a path v0.5 deleted is committed"; fi
  [ -s "$dir/log/gh.pr-create" ] || { fail "no pull request was opened"; return; }
  create_args | grep -qx -- --draft || fail "an authored conflict was not opened as a draft"
  grep -q '`src/main.rs`' <<<"$(create_args)" || fail "the draft does not name src/main.rs"
  grep -q '`src/dropped.rs`' <<<"$(create_args)" || fail "the draft does not name src/dropped.rs"
  if grep -q '`CONFORMANCE.md`' <<<"$(create_args)"; then fail "the draft lists a generated path"; fi
  git -C "$dir/origin.git" log -1 --format=%B "refs/heads/$branch" | grep -q -- '- src/main.rs' ||
    fail "the merge message does not list src/main.rs"
}

# --- lock ------------------------------------------------------------------
case_lock() {
  setup lock
  on release/v0.4 "v0.4 many" sh -c 'echo "fn main() { v04() }" >src/main.rs; for f in compiler/rust/pkg/src/m*.rs; do echo v04 >"$f"; done'
  on release/v0.5 "v0.5 many" sh -c 'echo "fn main() { v05() }" >src/main.rs; for f in compiler/rust/pkg/src/m*.rs; do echo v05 >"$f"; done'
  mkdir -p "$dir/locking"
  write_locking_git "$dir/locking"
  run "$dir/locking"
  report lock_checks
}
lock_checks() {
  [ -s "$dir/log/lock" ] || fail "the listing never held index.lock: the case tested nothing"
  pushed >/dev/null || { fail "no forward branch was pushed"; return; }
  checks_merge_commit
  [ "$(at compiler/rust/pkg/src/m40.rs)" = v05 ] || fail "a generated conflict was not resolved"
  [ "$(output clean)" = false ] || fail "the authored conflict is reported clean"
  [ ! -e "$dir/work/.git/index.lock" ] || fail "index.lock is left behind"
}

case_clean
case_generated
case_source
case_lock
if [ "$failures" -ne 0 ]; then
  printf '%s failure(s) in forward-merge.sh\n' "$failures"
  exit 1
fi
echo "forward-merge.sh: 4 cases passed"

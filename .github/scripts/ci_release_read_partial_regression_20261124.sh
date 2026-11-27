#!/usr/bin/env bash
# Advisory reproduction only; never a release authorization.
# Source: v0.4 a5e846de239113ca591f949eb75764d40096f7e9, release-gate blob 5046cbc2fbb13e92824cbf11c6cbf54d753341ea
# Source: v0.5 471237b3c00b6b6a78f8867de190c7cd9eaf4d3c, release-gate blob a084e13c43e42028601140250825ab8be859cf74
# Atomic helper blob 9b187e917695986894c9264a4fbf915c750efdfc; vv-rows blob f35167d5551cec6db851140a6c3aa9955b91fb10
# Exact extracted functions; only renamed the two jobs_of functions.
# Synthetic provider fault: emitted a success-shaped but incomplete listing then exits nonzero.
# No assertion that this exact fault occurred on GitHub or authorized a release.
set -euo pipefail
repo=example/repo
parts='honesty
examples
rest
bdd-bootstrap-check
bdd-bootstrap-gnaf
bdd-bootstrap-print
bdd-bootstrap-runtime
bdd-bootstrap-lexicon-lre
bdd-bootstrap-lexicon-lse
bdd-bootstrap-project-text
bdd-bootstrap-tooling-latex
bdd-bootstrap-validate
bdd-bootstrap-lower
bdd-bootstrap-moves
bdd-bootstrap-bodies
bdd-bootstrap-succeed
bdd-bootstrap-loops
bdd-bootstrap-assemble
bdd-bootstrap-package
bdd-bootstrap-structural-grammar
bdd-bootstrap-surface-lex
bdd-bootstrap-surface-doc
bdd-bootstrap-surface-tooling
bdd-bootstrap-surface-generic
bdd-bootstrap-terms
bdd-compiler
bdd-preservation
bdd-preservation-lists
bdd-preservation-prims
bdd-preservation-std
bdd-preservation-runtime
bdd-preservation-support
bdd-production
bdd-verification
bdd-bootstrap-surface-module
bdd-bootstrap-diagnostics
bdd-bootstrap-cli-output
bdd-bootstrap-lock-text
bdd-bootstrap-content-id
bdd-bootstrap-manifest-text
bdd-bootstrap-grammar-proof
bdd-stdlib
bdd-bootstrap-semantic-print
bdd-bootstrap-semantic-read
bdd-bootstrap-core-json
bdd-bootstrap-core-print
bdd-bootstrap-core-read
bdd-bootstrap-grammar-math
bdd-bootstrap-proposition-terms
bdd-bootstrap-lexicon-schema
bdd-bootstrap-lse-scopes
bdd-bootstrap-lse-types
bdd-bootstrap-toml-read
bdd-bootstrap-package-ref
bdd-bootstrap-lexicon-entry
bdd-language-models
bdd-wasi'
hosts_passed() {
  awk -F'\t' '
    BEGIN {
      expected["build and non-Lean tests (aarch64-unknown-linux-gnu)"] = 1
      expected["build and non-Lean tests (x86_64-apple-darwin)"] = 1
      expected["build and non-Lean tests (aarch64-apple-darwin)"] = 1
      expected["build and non-Lean tests (x86_64-pc-windows-msvc)"] = 1
    }
    /^build and non-Lean tests \(/ {
      if (!($1 in expected) || ++seen[$1] != 1 || $2 != "success") bad = 1
    }
    END {
      for (host in expected) if (seen[host] != 1) bad = 1
      exit bad
    }'
}
whole_gate_passed() {
  local jobs=$1 part job
  [ -n "$jobs" ] || return 1
  printf '%s\n' "$jobs" | awk -F'\t' '$2 != "success" { bad = 1 } END { exit bad }' || return 1
  # Quiet grep can close the pipe early and make printf fail under pipefail.
  printf '%s\n' "$jobs" | grep -xP 'atlas \(join\)\tsuccess\t.*' >/dev/null || return 1
  printf '%s\n' "$jobs" | hosts_passed || return 1
  for part in $parts; do
    job="just vv ($part)"
    [ "$part" != honesty ] || job=honesty
    printf '%s\n' "$jobs" | grep -P "^\Q$job\E\tsuccess\t" >/dev/null || return 1
  done
}
old_jobs_of() {
  gh api "repos/$repo/actions/runs/$1/jobs?per_page=100" --paginate \
    --jq '.jobs[] | "\(.name)\t\(.conclusion)\t\(.started_at)\t\(.completed_at)"' 2>/dev/null || true
}
fixed_jobs_of() {
  ci_provider_read "repos/$repo/actions/runs/$1/jobs?per_page=50" --paginate \
    --jq '.jobs[] | "\(.name)\t\(.conclusion)\t\(.started_at)\t\(.completed_at)"' || true
}
#!/usr/bin/env bash
# Read-only GitHub evidence lookup. A partial paginated response from a failed
# request is never evidence. Retry the entire identical read, then refuse it.
ci_provider_read() {
  local output attempt status=1
  for attempt in 1 2 3; do
    if output=$(timeout 30s gh api "$@"); then
      printf '%s\n' "$output"
      return 0
    else
      status=$?
    fi
    printf 'CI evidence read failed (attempt %s/3, exit %s)\n' "$attempt" "$status" >&2
    if [ "$attempt" -lt 3 ]; then
      sleep "$((attempt * 2))"
    fi
  done
  return "$status"
}

# Simulate provider reads, retry delays, and timeout wrapper without network.
mode=partial
gh() {
  [ "$1" = api ] || return 97
  for part in $parts; do
    local name="just vv ($part)"
    [ "$part" != honesty ] || name=honesty
    printf '%s\tsuccess\tstart\tend\n' "$name"
  done
  printf 'atlas (join)\tsuccess\tstart\tend\n'
  for host in aarch64-unknown-linux-gnu x86_64-apple-darwin aarch64-apple-darwin x86_64-pc-windows-msvc; do
    printf 'build and non-Lean tests (%s)\tsuccess\tstart\tend\n' "$host"
  done
  if [ "$mode" = partial ]; then return 1; fi
  if [ "$mode" = failed ]; then printf 'planted auxiliary job\tfailure\tstart\tend\n'; fi
  return 0
}
timeout() { shift; "$@"; }
sleep() { :; }
old=$(old_jobs_of 1)
whole_gate_passed "$old" || { echo "expected old partial acceptance" >&2; exit 1; }
new=$(fixed_jobs_of 1)
if whole_gate_passed "$new"; then echo "atomic read accepted partial" >&2; exit 1; fi
mode=complete
whole_gate_passed "$(fixed_jobs_of 1)" || exit 1
mode=failed
if whole_gate_passed "$(fixed_jobs_of 1)"; then exit 1; fi
printf '%s\n' '4/4 synthetic cases passed: old accepts partial; fixed refuses partial; fixed accepts success-shaped positive control; fixed refuses visible failure.'

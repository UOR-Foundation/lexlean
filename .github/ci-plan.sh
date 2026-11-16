#!/usr/bin/env bash
# What a run of ci.yml is (SPEC.md §9.4), decided from the event and the
# repository alone, before any job that costs a runner starts.
#
#   pull request, draft       honesty then the `rest` leg only: a development signal
#   pull request, ready, base honesty then the `rest` leg only, the same signal: only a pull
#   not main or release/*     request into a line can land on it
#   pull request, ready, base the whole gate, without the Intel macOS jobs
#   main or release/*         unless the head is a release cut (`cut/v*`) or
#                             the base is `main`
#   workflow_dispatch on the  planned as that pull request, by the rows above
#   head branch of one open   (`regenerate.yml` dispatches this after it
#   pull request              commits to the head)
#   workflow_dispatch, other  the whole gate
#   push to main or to a      the whole gate, unless the push is a merge whose
#   release line (§9.4)
#                             tree is the tree of its second parent and the
#                             whole gate passed on that parent as a pull
#                             request or dispatched run: then nothing, and the run says
#                             which run verified the tree; or when that run
#                             passed every job but the Intel macOS jobs it did
#                             not run, honesty and the Intel jobs only
#
# Why: a pull request is pushed to several times before its head is final, a
# merge of a head that already contains main lands that head's tree
# unchanged, and each of those runs started the whole gate (about 500
# runner-minutes on a pool of 20 shared jobs) for a tree whose verdict was
# known or was not going to be read. A pull request stacked on another feature
# branch is the same waste: it cannot land on a line, and the pull request that
# carries the stack into `main` or a release line runs the whole gate on the
# tree that lands.
#
# Intel macOS (§8.3) has about one hosted runner for this repository and the
# whole of its workload takes about three hours of it, so pull requests that
# each waited for it queued for six hours and more. A pull request into a line
# runs every other job and leaves the Intel jobs to the line: the push that
# lands it runs them on the tree it lands. A release cut, and a pull request
# into `main`, run them as part of their own gate, so the run a tag's evidence
# names (`release-gate.sh`) is still one run with every host.
#
# Inputs, all from the environment: EVENT, DRAFT, BASE (the base branch of a
# pull request), HEAD (the head branch of a pull request), REPO, and REF (the
# branch a dispatch names). The jobs read `full`, `rows` and `intel` from
# GITHUB_OUTPUT. `LEXLEAN_CI_ROWS` names the legs file
# when a test runs the script elsewhere.
set -euo pipefail

rows_file=${LEXLEAN_CI_ROWS:-.github/vv-rows.json}
event=${EVENT:?EVENT names the workflow event}
draft=${DRAFT:-}
base=${BASE:-}
repo=${REPO:?REPO names owner/name}
ref=${REF:-}
head=${HEAD:-}
out=${GITHUB_OUTPUT:-/dev/stdout}
summary=${GITHUB_STEP_SUMMARY:-/dev/null}

# full, rows, and whether the Intel macOS jobs run (default: with the whole gate).
emit() {
  printf 'full=%s\nrows=%s\nhonesty=%s\nfanout=%s\nintel=%s\n' "$1" "$2" \
    "$(jq -c '[.[] | select(.part == "honesty")]' <<< "$2")" \
    "$(jq -c '[.[] | select(.part != "honesty")]' <<< "$2")" "${3:-$1}" >> "$out"
}

# The whole gate; INTEL (default true) says whether its Intel macOS jobs run.
whole_gate() {
  emit true "$(jq -c . "$rows_file")" "${2:-true}"
  printf '%s\n' "$1" >> "$summary"
}

# A pull request whose run carries the Intel macOS jobs: a release cut, whose
# run is the evidence of its tag, and a pull request into `main`.
intel_pull_request() {
  case "$head" in cut/v*) return 0 ;; esac
  [ "$base" = main ]
}

# These are the four auxiliary §8.3 contexts, not the number of matrix rows.
# Linux x86-64 is the normative gate; Intel macOS is a static aggregate. Every
# required context must occur exactly once, so a duplicate or foreign context
# cannot replace a missing host even when the total count remains four.
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

# Whether a jobs listing (`name<TAB>conclusion`) is the whole gate without its
# Intel macOS jobs, passed: every job concluded `success` but those Intel jobs,
# which were skipped, and the run holds the Intel context exactly once and
# skipped, the join of Atlas, the three other supported-host contexts exactly
# once, and every leg of the rows file.
linux_passed() {
  local names=$1 part job
  printf '%s\n' "$names" | awk -F'\t' '
    $1 == "build and non-Lean tests (x86_64-apple-darwin)" { intel++ }
    $1 == "Intel test construction and local targets" || $1 ~ /^Intel conformance \(/ \
      || $1 == "build and non-Lean tests (x86_64-apple-darwin)" {
      if ($2 != "skipped") bad = 1
      next
    }
    $2 != "success" { bad = 1 }
    END { exit bad || intel != 1 }' || return 1
  printf '%s\n' "$names" | grep -xF $'atlas (join)\tsuccess' >/dev/null || return 1
  printf '%s\n' "$names" | awk -F'\t' '
    BEGIN {
      expected["build and non-Lean tests (aarch64-unknown-linux-gnu)"] = 1
      expected["build and non-Lean tests (aarch64-apple-darwin)"] = 1
      expected["build and non-Lean tests (x86_64-pc-windows-msvc)"] = 1
    }
    /^build and non-Lean tests \(/ {
      if ($1 == "build and non-Lean tests (x86_64-apple-darwin)") next
      if (!($1 in expected) || ++seen[$1] != 1 || $2 != "success") bad = 1
    }
    END {
      for (host in expected) if (seen[host] != 1) bad = 1
      exit bad
    }' || return 1
  for part in $(jq -r '.[].part' "$rows_file"); do
    job="just vv ($part)"
    [ "$part" != honesty ] || job=honesty
    printf '%s\n' "$names" | grep -xF "$job"$'\tsuccess' >/dev/null || return 1
  done
}

# The id of a pull request run, or a dispatched run, of this workflow on
# COMMIT that ran every job of the gate and passed every one. A draft run has
# honesty and the `rest` leg, so it is never one: the run must hold the join of
# Atlas, every exact supported-host context, and every leg of the rows file,
# and no job that did not conclude `success`. With MODE `linux`, a run that
# passed every job but the Intel macOS jobs it skipped counts as well.
full_run_of() {
  local commit=$1 mode=${2:-whole} runs id names part job
  runs=$(gh api "repos/$repo/actions/workflows/ci.yml/runs?head_sha=$commit&status=success&per_page=30" \
    --jq ".workflow_runs[] | select(.head_repository.full_name == \"$repo\") | select(.event == \"pull_request\" or .event == \"workflow_dispatch\") | .id") || return 1
  for id in $runs; do
    names=$(gh api "repos/$repo/actions/runs/$id/jobs?per_page=100" --paginate \
      --jq '.jobs[] | "\(.name)\t\(.conclusion)"') || continue
    if [ "$mode" = linux ] && linux_passed "$names"; then
      printf '%s\n' "$id"
      return 0
    fi
    if printf '%s\n' "$names" | awk -F'\t' '$2 != "success" { bad = 1 } END { exit bad }' \
      && printf '%s\n' "$names" | grep -xF $'atlas (join)\tsuccess' >/dev/null \
      && printf '%s\n' "$names" | hosts_passed; then
      for part in $(jq -r '.[].part' "$rows_file"); do
        job="just vv ($part)"
        [ "$part" != honesty ] || job=honesty
        printf '%s\n' "$names" | grep -xF "$job"$'\tsuccess' >/dev/null || continue 2
      done
      printf '%s\n' "$id"
      return 0
    fi
  done
  return 1
}

# The full run (above) that verified the tree of COMMIT: one of COMMIT itself,
# or, while the commit is a merge whose tree is the tree of its second parent,
# one of that parent, at most four merges below the commit the run is for
# (DEPTH counts merges already walked), as `release-gate.sh` walks. A cut lands this way: the
# release merge lands the regenerated head's tree, and the pull request that
# carries the line to `main` lands the release merge's.
verified_tree_run() {
  local commit=$1 depth=${2:-0} mode=${3:-whole}
  while :; do
    if full_run_of "$commit" "$mode"; then
      return 0
    fi
    [ "$depth" -lt 4 ] || return 1
    # The plan's checkout is shallow; deepen it for the commit it walks.
    git rev-parse --verify --quiet "$commit^2" >/dev/null \
      || git fetch --quiet --depth=2 origin "$commit" 2>/dev/null || return 1
    git rev-parse --verify --quiet "$commit^2" >/dev/null || return 1
    [ "$(git rev-parse "$commit^{tree}")" = "$(git rev-parse "$commit^2^{tree}")" ] || return 1
    commit=$(git rev-parse "$commit^2")
    depth=$((depth + 1))
  done
}

# The verified run of the second parent of HEAD, when HEAD is a merge of that
# parent with the same tree (MODE as for `full_run_of`).
verified_parent_run() {
  local parent
  parent=$(git rev-parse --verify --quiet 'HEAD^2') || return 1
  [ "$(git rev-parse 'HEAD^{tree}')" = "$(git rev-parse "$parent^{tree}")" ] || return 1
  # HEAD to its parent is the first of the four merges `release-gate.sh`
  # walks from the tagged commit.
  verified_tree_run "$parent" 1 "${1:-whole}"
}

# A pull request's run: honesty then `rest` for a draft or a stacked base,
# the whole gate for a ready pull request into a line.
pull_request_plan() {
  if [ "$draft" = true ]; then
    emit false "$(jq -c '[.[] | select(.part == "honesty" or .part == "rest")]' "$rows_file")"
    printf '%s\n' 'Draft pull request: honesty then the `rest` leg only. This run is not the gate; marking the pull request ready runs the whole gate.' >> "$summary"
  else
    case "$base" in
      # An unknown base is not known to be stacked: the whole gate.
      main | release/* | '')
        if intel_pull_request; then
          whole_gate 'Pull request ready for review: the whole gate.'
        else
          whole_gate 'Pull request ready for review: the whole gate but its Intel macOS jobs, which the push that lands it runs.' false
        fi
        ;;
      *)
        emit false "$(jq -c '[.[] | select(.part == "honesty" or .part == "rest")]' "$rows_file")"
        printf 'Pull request into `%s`, which is neither `main` nor a release line: honesty then the `rest` leg only. A stacked pull request cannot land on a line; the pull request into `main` or a release line runs the whole gate.\n' "$base" >> "$summary"
        ;;
    esac
  fi
}

# The one open pull request of this repository whose head is the dispatched
# branch, as `draft<TAB>base`; nothing when there is none, several, or the
# list cannot be read.
dispatched_pull_request() {
  local found
  [ -n "$ref" ] || return 1
  found=$(gh api "repos/$repo/pulls?state=open&head=${repo%%/*}:$ref&per_page=2" \
    --jq ".[] | select(.head.repo.full_name == \"$repo\") | \"\(.draft)\t\(.base.ref)\"") || return 1
  [ -n "$found" ] && [ "$(printf '%s\n' "$found" | wc -l)" -eq 1 ] || return 1
  printf '%s\n' "$found"
}

case "$event" in
  pull_request)
    pull_request_plan
    ;;
  push)
    if run=$(verified_parent_run); then
      emit false '[]'
      printf 'This push merges a head whose tree it equals, and the whole gate passed on that head: run %s of %s. Nothing is run again.\n' "$run" "$repo" >> "$summary"
    elif run=$(verified_parent_run linux); then
      emit false "$(jq -c '[.[] | select(.part == "honesty")]' "$rows_file")" true
      printf 'This push merges a head whose tree it equals, and run %s of %s passed the whole gate on it but the Intel macOS jobs: honesty and the Intel jobs only.\n' "$run" "$repo" >> "$summary"
    else
      whole_gate 'Push: the whole gate.'
    fi
    ;;
  workflow_dispatch)
    if found=$(dispatched_pull_request); then
      draft=${found%%$'\t'*}
      base=${found#*$'\t'}
      head=$ref
      printf 'Dispatched on the head of an open pull request into `%s`: planned as that pull request.\n' "$base" >> "$summary"
      # A ready head whose tree a full run already verified (the line a cut
      # carries to `main`) is not verified again.
      if [ "$draft" != true ] && run=$(verified_tree_run "$(git rev-parse HEAD)"); then
        emit false '[]'
        printf 'The tree of this head is the tree run %s of %s verified as the whole gate. Nothing is run again.\n' "$run" "$repo" >> "$summary"
      else
        pull_request_plan
      fi
    else
      whole_gate 'Run on request: the whole gate.'
    fi
    ;;
  *)
    whole_gate 'Run on request: the whole gate.'
    ;;
esac

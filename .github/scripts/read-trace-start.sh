#!/usr/bin/env bash
# Advisory (CI #603 phase 1): start the read recorder for this job. Nothing
# here may fail the job: the action runs this without -e and ignores its
# status, and every path below ends in `exit 0`. A failure costs only the
# trace, which the stop step then reports as missing.
#
# Inputs (environment): READ_TRACE_JOB, READ_TRACE_MATRIX, READ_TRACE_JOB_INDEX.
# Outputs (GITHUB_ENV): LEXLEAN_READ_TRACE_DIR, LEXLEAN_READ_TRACE_NONCE.

start() {
  local scripts="$GITHUB_WORKSPACE/.github/scripts" dir nonce
  # A private directory per job attempt, so that no earlier recording's
  # ready file or report can be mistaken for this one's.
  dir=$(python3 -c 'import sys, tempfile; print(tempfile.mkdtemp(prefix="read-trace.", dir=sys.argv[1]))'     "${RUNNER_TEMP:-/tmp}") || return 1
  python3 "$scripts/ci_read_trace.py" provenance --root "$GITHUB_WORKSPACE" \
    --job "${READ_TRACE_JOB:-}" --matrix "${READ_TRACE_MATRIX:-}" \
    --job-index "${READ_TRACE_JOB_INDEX:-}" --output "$dir/provenance.json" || return 1
  nonce=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["nonce"])' \
    "$dir/provenance.json") || return 1
  { echo "LEXLEAN_READ_TRACE_DIR=$dir"; echo "LEXLEAN_READ_TRACE_NONCE=$nonce"; } \
    >> "$GITHUB_ENV" || return 1
  sudo -n nohup nice -n 19 python3 "$scripts/ci_read_trace.py" record \
    --root "$GITHUB_WORKSPACE" --dir "$dir" < /dev/null > "$dir/recorder.log" 2>&1 &
  local _
  for _ in $(seq 1 300); do
    [ -s "$dir/ready.json" ] && { echo "read recorder: started ($dir)"; return 0; }
    [ -s "$dir/reads.json" ] && break
    sleep 0.1
  done
  echo "::warning::the read recorder did not start (advisory, #603)"
  cat "$dir/recorder.log" 2> /dev/null
  return 0
}

start || echo "::warning::the read recorder could not be prepared (advisory, #603)"
exit 0

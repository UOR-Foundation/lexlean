#!/usr/bin/env bash
# Advisory (CI #603 phase 1): stop this job's read recorder and print its
# report into the job log. Nothing here may fail the job: the action runs this
# without -e and ignores its status, and every path below ends in `exit 0`.
# The log, not an artifact, carries the report, so that no upload, quota or
# artifact-name conflict on a retried attempt can fail the job.

finish() {
  local scripts="$GITHUB_WORKSPACE/.github/scripts"
  local dir="${LEXLEAN_READ_TRACE_DIR:-}" nonce="${LEXLEAN_READ_TRACE_NONCE:-}"
  if [ -z "$dir" ] || [ -z "$nonce" ] || [ ! -d "$dir" ]; then
    echo "::warning::no read recorder ran in this job (advisory, #603)"
    return 0
  fi
  sudo -n timeout -k 10 150 python3 "$scripts/ci_read_trace.py" stop --dir "$dir" --nonce "$nonce" \
    || { echo "::warning::the read recorder did not stop cleanly (advisory, #603)"
         cat "$dir/recorder.log" 2> /dev/null; }
  timeout -k 10 60 python3 "$scripts/ci_read_trace.py" emit --dir "$dir" --nonce "$nonce" \
    || echo "::warning::the read trace could not be printed (advisory, #603)"
  return 0
}

finish || true
exit 0

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

#!/usr/bin/env bash
# The compiler-semantics IDs of the `compiler_semantics = "..."` lines on
# stdin, one per line; `$1` is a line prefix (`-` or `+` for a diff). An ID is
# 64 lowercase hex digits, bare (the SHA-256 IDs) or labelled `blake3:` (the
# BLAKE3 IDs the lock schema moved to on v0.4). Anything else is not an ID:
# the bare-hex pattern this replaces read a `blake3:` ID as no ID at all, and
# relock-identity then kept every lock it should have moved.
SEMANTICS_ID='\(blake3:\)\{0,1\}[0-9a-f]\{64\}'
semantics_ids() {
    sed -n "s/^${1:-}compiler_semantics = \"\(${SEMANTICS_ID}\)\"\$/\1/p"
}

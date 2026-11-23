#!/usr/bin/env python3
"""Fail-closed overlap and binary-patch manifest audit for regenerate.yml.

Check *all* writer artifacts before applying *any* patch. Per-writer .files
manifests and git-generated binary patches must name exactly the same paths.
No writes, network access, shell interpolation, or GitHub API calls.
"""
import argparse
import os
import subprocess
import sys
from pathlib import Path

WRITERS = ("production-export", "records", "oracles")
MAX_MANIFEST_BYTES = 8 * 1024 * 1024
MAX_PATHS = 100000


def require(condition, reason):
    if not condition:
        raise ValueError(reason)


def paths_from_nul(payload, source):
    if not payload:
        return []
    require(payload.endswith(b"\0"), source + ": filenames must be NUL-delimited")
    items = payload[:-1].split(b"\0")
    require(len(items) <= MAX_PATHS and len(items) == len(set(items)),
            source + ": too many or repeated filenames")
    for item in items:
        require(item and not item.startswith(b"/") and
                all(part not in (b"", b".", b"..") for part in item.split(b"/")) and
                item.split(b"/")[0] != b".git",
                source + ": unsafe or ambiguous path " + repr(item))
    return items


def regular(path):
    require(path.is_file() and not path.is_symlink(),
            "writer artifact missing, directory, or symlink: " + path.name)


def inspect(directory):
    directory = Path(directory)
    require(directory.is_dir() and not directory.is_symlink(),
            "writer artifact directory unavailable or linked")
    owners = {}
    rows = []
    for writer in WRITERS:
        manifest = directory / (writer + ".files")
        patch = directory / (writer + ".patch")
        regular(manifest)
        regular(patch)
        require(manifest.stat().st_size <= MAX_MANIFEST_BYTES,
                writer + ": oversized file manifest")
        declared = paths_from_nul(manifest.read_bytes(), writer + ".files")
        try:
            parsed = subprocess.run(
                ["git", "apply", "--name-only", "-z", str(patch)],
                check=False, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                timeout=30,
            )
        except (OSError, subprocess.TimeoutExpired) as error:
            raise ValueError(writer + ": cannot inspect patch") from error
        require(parsed.returncode == 0,
                writer + ": invalid binary/text patch path inventory")
        actual = paths_from_nul(parsed.stdout, writer + ".patch")
        require(set(declared) == set(actual),
                writer + ": patch and declared file inventory disagree")
        for item in declared:
            if item in owners:
                raise ValueError(
                    "writer collision: " + repr(item) +
                    " owned by " + owners[item] + " and " + writer
                )
            owners[item] = writer
        rows.append((writer, len(declared)))
    return rows


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--parts-dir", required=True, type=Path)
    args = parser.parse_args(argv)
    try:
        for writer, count in inspect(args.parts_dir):
            print(writer + ": " + str(count) + " distinct paths")
        print("writer manifests and patch paths disjoint: PASS")
        return 0
    except ValueError as error:
        print("HOLD: " + str(error), file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())

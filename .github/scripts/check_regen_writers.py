#!/usr/bin/env python3
"""Fail-closed overlap and patch-set audit for regenerate.yml.

Check *all* writer artifacts before applying *any* patch. Each writer returns
a bounded patch set (`patch_set.py`): its parts must be whole, below Git's
patch limit, made from the commit being regenerated, and name exactly the
paths of its `.files` manifest; applied to the start tree every writer shares,
in a scratch index, they must give the tree the writer recorded. No path may
belong to two writers. With `--identity NAME`, the writers start from a
relocked identity: that set is checked the same way on the commit's own tree,
and the tree it gives is the start tree every writer must share. No writes to
the checkout, network access, shell interpolation, or GitHub API calls.
"""
import argparse
import sys
from pathlib import Path

from patch_set import check_tree, git, load, object_id, patch_names, paths_from_nul

WRITERS = ("production-export", "records", "oracles")

__all__ = ["WRITERS", "inspect", "patch_names", "paths_from_nul"]


def inspect(directory, base, start_tree=None, repo=None, identity=None):
    directory = Path(directory)
    commit_tree = git("rev-parse", base + "^{tree}", cwd=repo).decode().strip()
    if identity is not None:
        manifest, _ = load(directory, identity, base)
        if manifest["start_tree"] != commit_tree:
            raise ValueError(identity + ": made from tree " + manifest["start_tree"] +
                             ", not the commit's tree " + commit_tree)
        # In a scratch index: it also writes the identity's tree, which the
        # writers' sets start from, into the object store.
        check_tree(directory, identity, manifest, repo=repo)
        if start_tree is not None and start_tree != manifest["tree"]:
            raise ValueError("--start-tree is not the tree " + identity + " gives")
        start_tree = manifest["tree"]
    if start_tree is None:
        start_tree = commit_tree
    owners = {}
    rows = []
    for writer in WRITERS:
        manifest, declared = load(directory, writer, base)
        if manifest["start_tree"] != start_tree:
            raise ValueError(writer + ": made from tree " + manifest["start_tree"] +
                             ", not the shared start tree " + start_tree)
        check_tree(directory, writer, manifest, repo=repo)
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
    parser.add_argument("--base", required=True)
    parser.add_argument("--start-tree")
    parser.add_argument("--identity", help="the set every writer starts from")
    args = parser.parse_args(argv)
    try:
        base = object_id(git("rev-parse", "--verify", args.base + "^{commit}")
                         .decode().strip(), "base")
        if args.identity is not None and args.identity in WRITERS:
            raise ValueError("the identity set cannot be a writer's")
        for writer, count in inspect(args.parts_dir, base, args.start_tree,
                                     identity=args.identity):
            print(writer + ": " + str(count) + " distinct paths")
        print("writer patch sets whole and their paths disjoint: PASS")
        return 0
    except ValueError as error:
        print("HOLD: " + str(error), file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())

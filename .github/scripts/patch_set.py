#!/usr/bin/env python3
"""Bounded Git patch sets for regenerate.yml (#624).

`git apply` refuses any patch of 1023 MiB or more ("patch too large"), and a
whole-tree rewrite of the verification records is larger than that. A patch
set is the same staged change written as several patches, cut only between
whole file diffs, each about TARGET_BYTES and never MAX_BYTES or more, with a
manifest that binds them:

    NAME.files          every path the change touches, NUL-delimited
    NAME.patchset.json  spec, name, base commit, start and result trees,
                        thresholds, the .files checksum, and every part's
                        file, size, checksum and path inventory checksum
    NAME.0001.patch ... the parts, in order (none for an empty change)

A set is checked whole before anything is applied: every part is present,
regular, of its recorded size and checksum and below Git's limit; no part
renames or copies; each part's paths, read back by `git apply --numstat`,
are disjoint from every other part's, and together they are exactly
NAME.files; and no other file of the directory claims the name. Applied to
its start tree, in a scratch index, the parts give exactly its result tree.

Nothing touches a real index or work tree until all of that holds, and then
only a clean checkout of the start tree: the index is the start tree, the
work tree is the index, and no untracked file stands where a set adds one.
Should Git still refuse a part on the real checkout, the index and work tree
are put back to the start tree before the refusal is reported, so a refusal
never leaves a partly applied change behind.

No network access, shell interpolation, or GitHub API calls.
"""
import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

SPEC = "lexlean/patch-set/1"
GIT_APPLY_LIMIT = 1023 * 1024 * 1024  # apply.c MAX_APPLY_SIZE: refused at or above
TARGET_BYTES = 128 * 1024 * 1024
MAX_BYTES = 768 * 1024 * 1024
MAX_PATHS = 100000
MAX_PARTS = 9999
MAX_MANIFEST_BYTES = 8 * 1024 * 1024
OBJECT = re.compile(r"\A(?:[0-9a-f]{40}|[0-9a-f]{64})\Z")
NAME = re.compile(r"\A[a-z][a-z0-9-]{0,63}\Z")
DIFF = b"diff --git "
KEYS = {"spec", "name", "base", "start_tree", "tree", "target_bytes", "max_bytes",
        "paths", "files_sha256", "parts"}
PART_KEYS = {"file", "bytes", "sha256", "paths", "paths_sha256"}


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


def patch_names(payload, source):
    """Parse git apply --numstat -z for non-rename binary/text patches."""
    if not payload:
        return []
    require(payload.endswith(b"\0"), source + ": truncated numstat output")
    found = []
    for row in payload[:-1].split(b"\0"):
        fields = row.split(b"\t", 2)
        require(len(fields) == 3 and all(
            n == b"-" or (n.isascii() and n.isdigit()) for n in fields[:2]),
            source + ": malformed binary/text numstat row")
        require(fields[2], source + ": unexpected rename/copy metadata")
        found.append(fields[2])
    paths_from_nul(b"\0".join(found) + b"\0", source)
    return found


def sha256(path):
    digest = hashlib.sha256()
    with open(path, "rb") as stream:
        for block in iter(lambda: stream.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def nul_digest(paths):
    return hashlib.sha256(b"".join(p + b"\0" for p in sorted(paths))).hexdigest()


def unique_keys(pairs):
    """A JSON object whose keys are distinct: `json` keeps the last of a
    repeated key, so a manifest could otherwise say two things at once."""
    keys = [key for key, _ in pairs]
    require(len(keys) == len(set(keys)), "repeated manifest key")
    return dict(pairs)


def regular(path, what):
    require(path.is_file() and not path.is_symlink(),
            what + " missing, directory, or symlink: " + path.name)


def toplevel(repo):
    """The checkout's top directory. Git reads the paths of `apply` relative to
    the directory it runs in, so every command runs at the top."""
    result = subprocess.run(["git", "rev-parse", "--show-toplevel"], cwd=repo,
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
    require(result.returncode == 0, "not inside a Git checkout")
    return result.stdout.decode().strip()


def git(*args, cwd=None, env=None, input=None):
    cwd = toplevel(cwd)
    result = subprocess.run(["git", *args], cwd=cwd, env=env, input=input,
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
    if result.returncode != 0:
        raise ValueError("git " + args[0] + ": " +
                         result.stderr.decode("utf-8", "replace").strip())
    return result.stdout


def object_id(text, what):
    require(isinstance(text, str) and OBJECT.match(text), what + " is not a full object ID")
    return text


def numstat(patch):
    """The paths of one patch as Git itself reads them, outside any checkout."""
    with patch.open("rb") as stream:
        require(not any(line.startswith((b"rename from ", b"rename to ",
                                         b"copy from ", b"copy to ")) for line in stream),
                patch.name + ": unexpected rename/copy patch; use --no-renames")
    try:
        with tempfile.TemporaryDirectory() as scratch:
            env = dict(os.environ)
            for key in ("GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE",
                        "GIT_PREFIX", "GIT_COMMON_DIR"):
                env.pop(key, None)
            env["GIT_CEILING_DIRECTORIES"] = str(Path(scratch).resolve().parent)
            parsed = subprocess.run(
                ["git", "apply", "--numstat", "-z", str(patch.resolve())],
                cwd=scratch, env=env, check=False, stdout=subprocess.PIPE,
                stderr=subprocess.PIPE, timeout=300,
            )
    except (OSError, subprocess.TimeoutExpired) as error:
        raise ValueError(patch.name + ": cannot inspect patch") from error
    require(parsed.returncode == 0, patch.name + ": invalid binary/text patch path inventory: " +
            parsed.stderr.decode("utf-8", "replace").strip())
    return patch_names(parsed.stdout, patch.name)


def file_diffs(stream):
    """Each whole file diff of a `git diff` stream. A line that starts a file
    diff cannot occur inside one: a text hunk line starts with ' ', '+', '-'
    or '\\', and Git's base85 alphabet has no space."""
    current = []
    for line in stream:
        if line.startswith(DIFF) and current:
            yield b"".join(current)
            current = []
        require(current or line.startswith(DIFF), "a patch must begin with a file diff")
        current.append(line)
    if current:
        yield b"".join(current)


def split(stream, name, out, target, maximum):
    """Write the parts of one patch stream; returns their file names."""
    require(0 < target <= maximum < GIT_APPLY_LIMIT and maximum <= MAX_BYTES,
            "thresholds must satisfy 0 < target <= max <= MAX_BYTES < Git's limit")
    parts, chunk, size = [], [], 0

    def flush():
        require(len(parts) < MAX_PARTS, name + ": too many parts")
        file = "%s.%04d.patch" % (name, len(parts) + 1)
        with open(out / file, "xb") as handle:
            handle.writelines(chunk)
        parts.append(file)

    for diff in file_diffs(stream):
        require(len(diff) < maximum, name + ": one file diff of " + str(len(diff)) +
                " bytes reaches the part maximum")
        if chunk and size + len(diff) > target:
            flush()
            chunk, size = [], 0
        chunk.append(diff)
        size += len(diff)
    if chunk:
        flush()
    return parts


def manifest_for(name, out, base, start_tree, tree, target, maximum, files, parts):
    declared = paths_from_nul(files, name + ".files")
    rows, seen = [], set()
    for file in parts:
        found = numstat(out / file)
        require(found and not seen & set(found), file + ": empty or overlapping part")
        seen |= set(found)
        rows.append({"file": file, "bytes": (out / file).stat().st_size,
                     "sha256": sha256(out / file), "paths": len(found),
                     "paths_sha256": nul_digest(found)})
    require(seen == set(declared), name + ": parts and declared file inventory disagree")
    return {"spec": SPEC, "name": name, "base": base, "start_tree": start_tree, "tree": tree,
            "target_bytes": target, "max_bytes": maximum, "paths": len(declared),
            "files_sha256": hashlib.sha256(files).hexdigest(), "parts": rows}


def write_manifest(out, name, manifest):
    with open(out / (name + ".patchset.json"), "x", encoding="utf-8") as handle:
        json.dump(manifest, handle, indent=2, sort_keys=True)
        handle.write("\n")


def claimed(out, name):
    return [p for p in out.iterdir() if p.name.startswith(name + ".")]


def write(name, out, base, target=TARGET_BYTES, maximum=MAX_BYTES, repo=None):
    """The staged change of the checkout at `repo`, against its HEAD, as a set."""
    require(NAME.match(name), "unsafe set name " + repr(name))
    require(0 < target <= maximum < GIT_APPLY_LIMIT and maximum <= MAX_BYTES,
            "thresholds must satisfy 0 < target <= max <= MAX_BYTES < Git's limit")
    out = Path(out)
    out.mkdir(parents=True, exist_ok=True)
    require(not claimed(out, name), name + ": the output directory already holds a set")
    base = object_id(git("rev-parse", "--verify", base + "^{commit}", cwd=repo)
                     .decode().strip(), "base")
    start_tree = git("rev-parse", "HEAD^{tree}", cwd=repo).decode().strip()
    tree = git("write-tree", cwd=repo).decode().strip()
    files = git("diff", "--cached", "--no-renames", "--name-only", "-z", cwd=repo)
    # Written whole in a scratch directory beside the output, then moved in: a
    # write that fails leaves no partial set behind.
    with tempfile.TemporaryDirectory(dir=out) as scratch:
        stage = Path(scratch)
        (stage / (name + ".files")).write_bytes(files)
        with tempfile.TemporaryFile() as whole:
            diff = subprocess.run(["git", "diff", "--cached", "--no-renames", "--binary"],
                                  cwd=toplevel(repo), stdout=whole, stderr=subprocess.PIPE,
                                  check=False)
            require(diff.returncode == 0,
                    "git diff: " + diff.stderr.decode("utf-8", "replace"))
            whole.seek(0)
            parts = split(whole, name, stage, target, maximum)
        manifest = manifest_for(name, stage, base, start_tree, tree, target, maximum,
                                files, parts)
        write_manifest(stage, name, manifest)
        for path in sorted(stage.iterdir()):
            path.replace(out / path.name)
    return manifest


def load(directory, name, base):
    """The checked manifest and path inventory of a set; applies nothing."""
    require(NAME.match(name), "unsafe set name " + repr(name))
    directory = Path(directory)
    require(directory.is_dir() and not directory.is_symlink(),
            "patch-set directory unavailable or linked")
    path = directory / (name + ".patchset.json")
    files_path = directory / (name + ".files")
    regular(path, name + " manifest")
    regular(files_path, name + " file manifest")
    require(path.stat().st_size <= MAX_MANIFEST_BYTES and
            files_path.stat().st_size <= MAX_MANIFEST_BYTES, name + ": oversized manifest")
    try:
        manifest = json.loads(path.read_text(encoding="utf-8"),
                              object_pairs_hook=unique_keys)
    except ValueError as error:
        raise ValueError(name + ": unreadable manifest") from error
    require(isinstance(manifest, dict) and set(manifest) == KEYS, name + ": manifest keys")
    require(manifest["spec"] == SPEC and manifest["name"] == name, name + ": spec or name")
    require(object_id(manifest["base"], "base") == base,
            name + ": made from " + str(manifest["base"]) + ", not " + base)
    object_id(manifest["start_tree"], "start tree")
    object_id(manifest["tree"], "tree")
    target, maximum = manifest["target_bytes"], manifest["max_bytes"]
    require(all(type(v) is int for v in (target, maximum, manifest["paths"])) and
            0 < target <= maximum <= MAX_BYTES < GIT_APPLY_LIMIT, name + ": thresholds")
    files = files_path.read_bytes()
    require(hashlib.sha256(files).hexdigest() == manifest["files_sha256"],
            name + ": file manifest checksum")
    declared = paths_from_nul(files, name + ".files")
    require(len(declared) == manifest["paths"], name + ": path count")
    parts = manifest["parts"]
    require(isinstance(parts, list) and len(parts) <= MAX_PARTS, name + ": parts")
    expected = {name + ".patchset.json", name + ".files"}
    seen = set()
    for index, row in enumerate(parts, 1):
        require(isinstance(row, dict) and set(row) == PART_KEYS, name + ": part keys")
        file = row["file"]
        require(file == "%s.%04d.patch" % (name, index), name + ": part order or name")
        expected.add(file)
        part = directory / file
        regular(part, "part")
        size = part.stat().st_size
        require(type(row["bytes"]) is int and size == row["bytes"] and 0 < size < maximum,
                file + ": size")
        require(sha256(part) == row["sha256"], file + ": checksum")
        found = numstat(part)
        require(found and type(row["paths"]) is int and len(found) == row["paths"] and
                nul_digest(found) == row["paths_sha256"], file + ": path inventory")
        require(not seen & set(found), file + ": a path another part also writes")
        seen |= set(found)
    extra = sorted(p.name for p in claimed(directory, name) if p.name not in expected)
    require(not extra, name + ": unlisted files claim the set: " + ", ".join(extra))
    require(seen == set(declared), name + ": parts and declared file inventory disagree")
    return manifest, declared


def scratch_env(scratch):
    """An environment whose Git index is a scratch file: what runs in it never
    reads or writes the checkout's index or work tree."""
    return dict(os.environ, GIT_INDEX_FILE=str(Path(scratch) / "index"))


def apply_parts(directory, manifest, repo=None, env=None, index_only=True):
    for row in manifest["parts"]:
        git("apply", "--cached" if index_only else "--index",
            str((Path(directory) / row["file"]).resolve()), cwd=repo, env=env)


def check_tree(directory, name, manifest, repo=None):
    """The parts applied to the start tree in a scratch index give the tree."""
    with tempfile.TemporaryDirectory() as scratch:
        env = scratch_env(scratch)
        git("read-tree", manifest["start_tree"], cwd=repo, env=env)
        apply_parts(directory, manifest, repo=repo, env=env)
        tree = git("write-tree", cwd=repo, env=env).decode().strip()
    require(tree == manifest["tree"], name + ": the parts give tree " + tree +
            ", not " + manifest["tree"])


def tree_entries(tree, repo=None):
    """Each path of a tree, with its mode and object."""
    out = {}
    for row in git("ls-tree", "-r", "-z", "--full-tree", tree, cwd=repo).split(b"\0"):
        if row:
            meta, path = row.split(b"\t", 1)
            mode, _, oid = meta.split(b" ")
            out[path] = (mode, oid)
    return out


def index_entries(repo=None, env=None):
    """Each path of the index, with its mode and object; a conflicted index
    is refused."""
    out = {}
    for row in git("ls-files", "-s", "-z", cwd=repo, env=env).split(b"\0"):
        if row:
            meta, path = row.split(b"\t", 1)
            mode, oid, stage = meta.split(b" ")
            require(stage == b"0", "the index holds a conflict at " + repr(path))
            out[path] = (mode, oid)
    return out


def require_clean(start, declared, repo=None):
    """The checkout is exactly the start tree, so a refusal part-way can put it
    back without losing anything: the index is the start tree, the work tree
    is the index, and nothing untracked stands where a set adds a file."""
    current = git("write-tree", cwd=repo).decode().strip()
    require(current == start, "the index holds " + current + ", not the start tree " + start)
    dirty = git("diff", "--no-renames", "--name-only", "-z", cwd=repo)
    require(not dirty, "the work tree differs from the index at " +
            ", ".join(repr(p) for p in dirty.rstrip(b"\0").split(b"\0")[:5]))
    top = Path(toplevel(repo))
    tracked = tree_entries(start, repo) if declared else {}
    stray = [p for p in declared if p not in tracked and
             os.path.lexists(top / os.fsdecode(p))]
    require(not stray, "an untracked file stands where a set adds one: " +
            ", ".join(repr(p) for p in stray[:5]))


def apply_in_place(directory, sets, start, expected, repo=None):
    """Apply prevalidated sets to the real index and work tree; afterwards
    the index must hold `expected`. Any refusal puts the start tree back."""
    try:
        for manifest in sets:
            apply_parts(directory, manifest, repo=repo, index_only=False)
        tree = git("write-tree", cwd=repo).decode().strip()
        require(tree == expected, "applied tree " + tree + ", not " + expected)
    except ValueError as error:
        try:
            git("read-tree", "--reset", "-u", start, cwd=repo)
        except ValueError as again:
            raise ValueError(str(error) + "; and the start tree could not be restored: " +
                             str(again)) from error
        raise ValueError(str(error) + "; the index and work tree were restored to the "
                         "start tree " + start) from error
    return tree


def apply(directory, name, base, repo=None):
    """Apply a checked set to the index and work tree, which must hold exactly
    its start tree; afterwards they hold exactly its result tree. The whole
    set is applied in a scratch index first, and the real checkout is touched
    only once that gives the recorded tree."""
    manifest, declared = load(directory, name, base)
    require_clean(manifest["start_tree"], declared, repo)
    check_tree(directory, name, manifest, repo=repo)
    apply_in_place(directory, [manifest], manifest["start_tree"], manifest["tree"], repo)
    return manifest, declared


def combine(directory, names, base, repo=None):
    """Apply disjoint sets that share one start tree, in turn, to an index and
    work tree holding it; afterwards each set's paths hold that set's tree and
    every other path still holds the start tree. Every check runs on a scratch
    index before the real checkout is touched."""
    loaded, owners = [], {}
    for name in names:
        manifest, declared = load(directory, name, base)
        check_tree(directory, name, manifest, repo=repo)
        for path in declared:
            require(path not in owners, "writer collision: " + repr(path) + " owned by " +
                    owners.get(path, "") + " and " + name)
            owners[path] = name
        loaded.append((name, manifest, declared))
    starts = {m["start_tree"] for _, m, _ in loaded}
    require(len(starts) <= 1, "the sets do not share one start tree")
    current = git("write-tree", cwd=repo).decode().strip()
    start = starts.pop() if starts else current
    require_clean(start, list(owners), repo)
    with tempfile.TemporaryDirectory() as scratch:
        env = scratch_env(scratch)
        git("read-tree", start, cwd=repo, env=env)
        for _, manifest, _ in loaded:
            apply_parts(directory, manifest, repo=repo, env=env)
        index = index_entries(repo, env)
        for name, manifest, declared in loaded:
            if declared:
                mine = tree_entries(manifest["tree"], repo)
                require(all(index.get(p) == mine.get(p) for p in declared),
                        name + ": combined paths differ from its tree")
        changed = paths_from_nul(git("diff", "--cached", "--no-renames", "--name-only",
                                     "-z", start, cwd=repo, env=env), "combined")
        require(set(changed) == set(owners), "the combined change is not the union of the sets")
        expected = git("write-tree", cwd=repo, env=env).decode().strip()
    tree = apply_in_place(directory, [m for _, m, _ in loaded], start, expected, repo)
    return start, tree, len(owners)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    sub = parser.add_subparsers(dest="command", required=True)
    w = sub.add_parser("write", help="the staged change as a set")
    w.add_argument("--name", required=True)
    w.add_argument("--base", required=True)
    w.add_argument("--out", required=True, type=Path)
    w.add_argument("--target-bytes", type=int, default=TARGET_BYTES)
    w.add_argument("--max-bytes", type=int, default=MAX_BYTES)
    for command in ("check", "apply"):
        c = sub.add_parser(command)
        c.add_argument("--name", required=True)
        c.add_argument("--base", required=True)
        c.add_argument("--dir", required=True, type=Path)
    c = sub.add_parser("combine", help="apply disjoint sets of one start tree in turn")
    c.add_argument("--names", required=True, help="comma-separated, in order")
    c.add_argument("--base", required=True)
    c.add_argument("--dir", required=True, type=Path)
    args = parser.parse_args(argv)
    try:
        if args.command == "write":
            m = write(args.name, args.out, args.base, args.target_bytes, args.max_bytes)
        else:
            base = object_id(git("rev-parse", "--verify", args.base + "^{commit}")
                             .decode().strip(), "base")
            if args.command == "combine":
                start, tree, count = combine(args.dir, args.names.split(","), base)
                print("combined: %d paths, %s -> %s: PASS" % (count, start[:12], tree[:12]))
                return 0
            if args.command == "check":
                m, _ = load(args.dir, args.name, base)
                check_tree(args.dir, args.name, m)
            else:
                m, _ = apply(args.dir, args.name, base)
        print("%s: %d paths in %d parts, %s -> %s: PASS" % (
            args.name, m["paths"], len(m["parts"]), m["start_tree"][:12], m["tree"][:12]))
        return 0
    except ValueError as error:
        print("HOLD: " + str(error), file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())

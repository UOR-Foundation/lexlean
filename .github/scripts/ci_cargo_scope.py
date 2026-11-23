#!/usr/bin/env python3
"""Exact local-Cargo reverse-closure for nonqualifying LexLean preview.

This source-backed development feedback mechanism is NOT a normative selective
merge qualifier. New/missing packages, changed manifests and unrecognized
source files do not authorize skipping any SPEC §9.4 obligation.
"""
import hashlib
import re
import tomllib
from pathlib import Path

# These package locations are actual repository-owned source roots, not a
# general monorepo prediction. A new member demands an explicit evaluation.
PACKAGES = {
    "repo-model": "crates/model",
    "lexlean": "crates/lexlean",
    "repo-conformance": "crates/conformance",
    "xtask": "xtask",
}
_NAME = re.compile(r"[a-z][a-z0-9_-]{0,62}\Z")


def load_graph(root):
    """Read actual TOML, reject incomplete/renamed workspace package mapping."""
    root = Path(root)
    data = bytearray()
    configs = {}
    for name, directory in sorted(PACKAGES.items()):
        file = root / directory / "Cargo.toml"
        if file.is_symlink() or not file.is_file() or file.stat().st_size > 128 * 1024:
            raise ValueError("missing, linked or oversized Cargo manifest: " + directory)
        raw = file.read_bytes()
        try:
            parsed = tomllib.loads(raw.decode("utf-8"))
        except (UnicodeDecodeError, tomllib.TOMLDecodeError) as error:
            raise ValueError("malformed Cargo manifest: " + directory) from error
        if parsed.get("package", {}).get("name") != name:
            raise ValueError("Cargo package identity drift: " + directory)
        configs[name] = parsed
        data.extend(directory.encode() + b"\0" + raw + b"\0")
    root_file = root / "Cargo.toml"
    if root_file.is_symlink() or not root_file.is_file() or root_file.stat().st_size > 128 * 1024:
        raise ValueError("workspace Cargo manifest unavailable")
    raw = root_file.read_bytes()
    try:
        workspace = tomllib.loads(raw.decode("utf-8"))
    except (UnicodeDecodeError, tomllib.TOMLDecodeError) as error:
        raise ValueError("workspace Cargo manifest malformed") from error
    # LexLean uses ["crates/*", "xtask"]. Any other member or exclusion
    # changes the accepted graph's boundary and needs fresh policy review.
    if workspace.get("workspace", {}).get("members") != ["crates/*", "xtask"]:
        raise ValueError("workspace membership changed: graph requires review")
    if set((root / "crates").glob("*/Cargo.toml")) != {
            root / d / "Cargo.toml" for d in PACKAGES.values() if d.startswith("crates/")
    }:
        raise ValueError("unmodeled Cargo workspace crate: graph requires review")
    data.extend(b"Cargo.toml\0" + raw)
    # dependency -> all direct dependents, including dev/build/target sections
    reverse = {name: set() for name in PACKAGES}
    for owner, config in configs.items():
        deps = [config.get("dependencies", {}), config.get("dev-dependencies", {}),
                config.get("build-dependencies", {})]
        for target in config.get("target", {}).values():
            deps += [target.get("dependencies", {}), target.get("dev-dependencies", {}),
                     target.get("build-dependencies", {})]
        for group in deps:
            if not isinstance(group, dict):
                raise ValueError("malformed dependency section")
            for key, definition in group.items():
                actual = definition.get("package", key) if isinstance(definition, dict) else key
                if actual in PACKAGES:
                    reverse[actual].add(owner)
    return reverse, hashlib.sha256(data).hexdigest()


def affected(root, paths):
    """Transitive reverse closure of actual Cargo *source* changes only."""
    reverse, digest = load_graph(root)
    roots = set()
    unresolved = []
    for path in paths:
        if not path.endswith(".rs"):
            continue
        matches = [name for name, prefix in PACKAGES.items()
                   if path.startswith(prefix + "/")]
        if len(matches) != 1:
            unresolved.append(path)
        else:
            roots.add(matches[0])
    closure = set(roots)
    pending = list(roots)
    while pending:
        current = pending.pop()
        for consumer in reverse[current] - closure:
            closure.add(consumer)
            pending.append(consumer)
    # No guessed local package names enter a command line.
    if any(not _NAME.fullmatch(name) for name in closure):
        raise ValueError("invalid package identity")
    return {
        "cargo_package_roots": sorted(roots),
        "cargo_check_packages": sorted(closure),
        "cargo_unresolved_rust_paths": sorted(unresolved),
        "cargo_manifest_sha256": digest,
        "cargo_feedback_only": True,
    }

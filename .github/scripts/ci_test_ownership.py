#!/usr/bin/env python3
"""A4: source-bound conformance/BDD/native-lane ownership audit.

This is READ-ONLY and NONQUALIFYING. Exact-source ownership is necessary for
invalidation, but it is not a proof of complete code/fixture/toolchain impact,
an authenticated provider receipt, or authority to skip a current-head test.
"""
import argparse
import hashlib
import json
import re
from collections import Counter
from pathlib import Path

SCHEMA = "lexlean/ci-test-ownership/1"
SOURCES = (
    "model/ids.toml",
    "crates/conformance/tests/conformance.rs",
    "crates/conformance/src/cases/mod.rs",
    "crates/conformance/src/cases/repository.rs",
    "crates/conformance/src/bdd_parts.rs",
    ".github/bdd-parts.json",
    ".github/intel-host-lanes.json",
    ".github/vv-rows.json",
)
_ID = re.compile(r"[A-Z]{2}-[0-9]{2}\Z")
_NAME = re.compile(r"[a-z0-9]+(?:-[a-z0-9]+)*\Z")
_CASE = re.compile(
    r'^#\[test\]\nfn (conformance_[a-z0-9_]+)\(\) \{\n'
    r'    repo_conformance::cases::run\("([A-Z]{2}-[0-9]{2})"\);\n\}',
    re.M,
)
_FUNCTION = re.compile(r"^fn (conformance_[a-z0-9_]+)\(\)", re.M)
_REGISTER = re.compile(r'^id = "([A-Z]{2}-[0-9]{2})"$', re.M)


class Hold(ValueError):
    """An incomplete, ambiguous, or tampered owner mapping."""


def require(condition, reason):
    if not condition:
        raise Hold(reason)


def strict_json(text):
    def no_duplicate(pairs):
        obj = {}
        for key, value in pairs:
            require(key not in obj, "duplicate JSON key: " + key)
            obj[key] = value
        return obj
    def nonfinite(_):
        raise Hold("nonfinite number")
    try:
        return json.loads(text, object_pairs_hook=no_duplicate,
                          parse_constant=nonfinite)
    except (ValueError, TypeError) as error:
        raise Hold("invalid JSON data") from error


def exact(obj, required, label):
    require(isinstance(obj, dict) and set(obj) == set(required),
            "unexpected " + label + " structure")


def sequence(values, predicate, label):
    require(isinstance(values, list) and all(predicate(x) for x in values),
            "invalid " + label)
    require(len(set(values)) == len(values), "duplicate " + label)
    return list(values)


def ownership(sources):
    require(isinstance(sources, dict) and set(sources) == set(SOURCES)
            and all(isinstance(sources[k], str) for k in SOURCES),
            "missing or unrecognized source input")
    identifiers = _REGISTER.findall(sources["model/ids.toml"])
    sections = sources["model/ids.toml"].count("[[id]]")
    require(bool(identifiers) and sections == len(identifiers) and
            len(set(identifiers)) == len(identifiers),
            "conformance ID register is incomplete or duplicated")
    generated = sources["crates/conformance/tests/conformance.rs"]
    cases = _CASE.findall(generated)
    declared = _FUNCTION.findall(generated)
    require(len(cases) == len(declared) == len(identifiers),
            "generated test wrappers do not cover the ID register")
    wrapper_ids = {}
    for function, ident in cases:
        require(function == "conformance_" + ident.lower().replace("-", "_"),
                "generated test dispatches the wrong conformance ID")
        require(ident not in wrapper_ids, "duplicate generated test ID")
        wrapper_ids[ident] = function
    require(set(wrapper_ids) == set(identifiers),
            "generated test register mismatch")

    registry = sources["crates/conformance/src/cases/mod.rs"]
    case_implementation = sources["crates/conformance/src/cases/repository.rs"]
    require("mod repository;" in registry and
            '"RP" => repository::run(id),' in registry and
            '"RP-14" => {' in case_implementation,
            "RP-14 dispatcher/source binding absent")
    selector = sources["crates/conformance/src/bdd_parts.rs"]
    for required in ('pub fn harness_filters(', 'pub fn cargo_targets(',
                     'for name in self.named_tests()', 'filters.push("--skip".to_owned());',
                     'pub fn test_name('):
        require(required in selector, "BDD selector semantic marker missing")

    bdd = strict_json(sources[".github/bdd-parts.json"])
    exact(bdd, ("remainder", "parts", "targets"), "BDD plan")
    remainder = bdd["remainder"]
    named = bdd["parts"]
    target_parts = bdd["targets"]
    require(isinstance(remainder, str) and _NAME.fullmatch(remainder) is not None,
            "invalid remainder")
    require(isinstance(named, dict) and bool(named) and
            isinstance(target_parts, dict) and bool(target_parts) and
            remainder not in named and remainder not in target_parts,
            "missing or overlapping BDD partitions")
    require(target_parts == {"honesty": ["bdd"]},
            "honesty must own complete BDD meta-gate target")
    owner = {}
    for part, members in named.items():
        require(isinstance(part, str) and _NAME.fullmatch(part) is not None,
                "invalid BDD part name")
        for ident in sequence(members, lambda x: isinstance(x, str) and
                              _ID.fullmatch(x) is not None,
                              "BDD case ID"):
            require(ident in wrapper_ids, "BDD part names unregistered case")
            require(ident not in owner, "case assigned to multiple BDD parts")
            owner[ident] = part
    require(owner.keys() < wrapper_ids.keys(), "empty or absent remainder")
    for ident in wrapper_ids:
        owner.setdefault(ident, remainder)
    selected = set(named) | set(target_parts) | {remainder}

    hosts = strict_json(sources[".github/intel-host-lanes.json"])
    exact(hosts, ("producer", "lanes"), "native lane plan")
    require(hosts["producer"] == ["honesty"], "host producer does not own honesty")
    lanes = hosts["lanes"]
    require(isinstance(lanes, dict) and set(lanes) ==
            {"lower", "print", "validate", "moves"},
            "native lanes are missing or changed")
    lane_for_part = {"honesty": "producer"}
    for lane, parts in lanes.items():
        for part in sequence(parts, lambda x: isinstance(x, str) and
                             _NAME.fullmatch(x) is not None,
                             "native partition"):
            require(part in selected and part not in lane_for_part,
                    "native coverage missing, unknown or duplicated")
            lane_for_part[part] = lane
    require(set(lane_for_part) == selected, "not all BDD partitions have host lanes")

    rows = strict_json(sources[".github/vv-rows.json"])
    require(isinstance(rows, list) and rows, "missing verification recipe rows")
    vv = {}
    for row in rows:
        require(isinstance(row, dict) and
                isinstance(row.get("part"), str) and
                isinstance(row.get("bdd_part"), str) and
                isinstance(row.get("recipes"), str),
                "malformed verification row")
        part = row["part"]
        require(part not in vv, "duplicate verification row")
        vv[part] = row
    require(set(vv) == {"examples", remainder, "honesty"} |
            {"bdd-" + key for key in named},
            "missing, extra or duplicate BDD verification recipes")
    require(vv["honesty"]["bdd_part"] == "honesty" and
            vv[remainder]["bdd_part"] == remainder and
            "bdd" in vv["honesty"]["recipes"].split() and
            "bdd" in vv[remainder]["recipes"].split(),
            "honesty/rest BDD verification removed")
    require(vv["examples"]["bdd_part"] == "",
            "examples incorrectly claims a BDD part")
    for part in named:
        row = vv["bdd-" + part]
        require(row["bdd_part"] == part and row["recipes"].split() == ["bdd"],
                "named BDD partition recipe changed")

    pinned = {name: hashlib.sha256(sources[name].encode("utf-8")).hexdigest()
              for name in SOURCES}
    rp_part = owner["RP-14"]
    rp_lane = lane_for_part[rp_part]
    require(rp_part == "rest" and rp_lane == "moves",
            "RP-14 ownership changed: re-evaluate failure analysis")
    sizes = Counter(owner.values())
    return {
        "schema": SCHEMA,
        "qualification": "NONQUALIFYING_OWNER_MAP_ONLY",
        "source_to_runtime_dependency_closure": "INCOMPLETE",
        "shared_compiled_conformance_binary": True,
        "cross_commit_reuse_authority": False,
        "source_sha256": pinned,
        "conformance_case_count": len(wrapper_ids),
        "bdd_named_parts": len(named),
        "bdd_target_parts": sorted(target_parts),
        "bdd_remainder": remainder,
        "host_execution_lanes": sorted(lanes),
        "cases_per_part": dict(sorted(sizes.items())),
        "rp14": {
            "registered_test": wrapper_ids["RP-14"],
            "bdd_part": rp_part,
            "native_lane": rp_lane,
            "ordinary_linux_row": "just vv (rest)",
            "other_host_families": ["windows", "intel"],
            "shared_source": "crates/conformance/src/cases/repository.rs",
            "compile_impact": "SHARED_CONFORMANCE_BINARY_REQUIRES_REVALIDATION",
        },
    }


def audit_repo(root):
    base = Path(root).resolve()
    data = {}
    for path in SOURCES:
        target = base / path
        require(target.is_file() and not target.is_symlink(),
                "missing or linked ownership input")
        try:
            data[path] = target.read_text(encoding="utf-8")
        except (UnicodeError, OSError) as error:
            raise Hold("unreadable ownership source") from error
    return ownership(data)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, required=True)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args(argv)
    try:
        report = audit_repo(args.repo_root)
    except (Hold, OSError) as error:
        parser.exit(2, "HOLD: " + str(error) + "\n")
    encoded = json.dumps(report, indent=2, sort_keys=True) + "\n"
    if args.output:
        args.output.write_text(encoded, encoding="utf-8")
    else:
        print(encoded, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

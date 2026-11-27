"""Advisory reproducer for the exact host_lanes check_join at 1de925f0308df16a4d67e3272a2a01de5ecb8e10.
No normative authority. Run with Python 3.9+; expected result is reproduction
of existing weaknesses, not passing qualification. The function below was
extracted directly from GitHub blob bbe3c9b8a145100456f02753209e9c2a2423d3ef.
"""
from collections import Counter
from copy import deepcopy

class Invalid(Exception):
    pass

def require(condition, message):
    if not condition:
        raise Invalid(message)

class factory:
    @staticmethod
    def test_name(i):
        return "conformance_" + i.lower().replace("-", "_")

def check_join(receipts, names, plan, bdd, here):
    """The lanes are this run's and commit's, complete, disjoint, and passed."""
    require(sorted(receipts) == sorted(names), f"lanes {sorted(names)}, receipts {sorted(receipts)}")
    for lane, receipt in receipts.items():
        for field in ("commit", "tree", "run", "attempt"):
            if field == "attempt":
                require(str(receipt[field]).isdigit() and int(receipt[field]) <= int(here[field] or 0),
                        f"lane `{lane}` ran in another attempt")
            else:
                require(receipt[field] == here[field], f"lane `{lane}` ran on another {field}")
        require(receipt.get("passed") is True, f"lane `{lane}` did not pass")
    conformance = receipts[PRODUCER]["conformance"]
    require(conformance and all(r["conformance"] == conformance for r in receipts.values()),
            "the lanes listed different conformance harnesses")
    named = {factory.test_name(i) for ids in bdd["parts"].values() for i in ids}
    require(named <= set(conformance), f"named tests absent: {sorted(named - set(conformance))}")
    whole = Counter(receipts[PRODUCER]["whole"] or [])
    ran = Counter(name for r in receipts.values() for name in r["executed"])
    require(whole, "the producer listed no workspace")
    missing, extra = whole - ran, ran - whole
    require(not missing and not extra,
            f"executed is not the host's listing: missing {sorted(missing)[:20]}, extra {sorted(extra)[:20]}")
    for lane in names:
        if lane == PRODUCER:
            continue
        require(set(receipts[lane]["executed"]) <= set(conformance),
                f"lane `{lane}` ran a test outside the conformance harness")
    print(f"host-lanes: {len(ran)} tests of {len(names)} lanes equal the host's listing, each once")
    return sum(ran.values())



BDD = {"parts": {"one": ["AA-01"], "two": ["AA-02"]}, "targets": {"honesty": ["bdd"]}, "remainder": "rest"}
PLAN = {"producer": ["honesty"], "lanes": {"a": ["one"], "b": ["two", "rest"]}}
HERE = {"commit": "c"*40, "tree": "t"*40, "run": "7", "attempt": "2"}
H = ["conformance_aa_01", "conformance_aa_02", "conformance_aa_03"]
base = {**HERE, "passed": True, "conformance": H, "whole": None}
valid = {"producer": {**base, "executed": ["meta", "unit"], "whole": ["meta", "unit", *H]},
         "a": {**base, "executed": [H[0]]}, "b": {**base, "executed": H[1:]}}

def check(name, mutate, expected):
    r = deepcopy(valid)
    mutate(r)
    try:
        result = check_join(r, list(valid), PLAN, BDD, HERE)
        actual = "ACCEPTED"
    except Invalid as exc:
        result = str(exc)
        actual = "REFUSED"
    print(name, actual, result)
    assert actual == expected

if __name__ == "__main__":
    check("valid", lambda r: None, "ACCEPTED")
    check("missing test", lambda r: r["a"]["executed"].clear(), "REFUSED")
    check("failed lane", lambda r: r["a"].update(passed=False), "REFUSED")
    check("zero attempt", lambda r: r["a"].update(attempt="0"), "ACCEPTED")
    check("swapped lane assignments", lambda r: (r["a"].update(executed=H[1:]), r["b"].update(executed=[H[0]])), "ACCEPTED")
    check("producer absorbs named conformance", lambda r: (r["a"].update(executed=[]), r["producer"]["executed"].append(H[0])), "ACCEPTED")
    check("duplicate whole inventory matched by duplicate producer output", lambda r: (r["producer"]["whole"].append("meta"), r["producer"]["executed"].append("meta")), "ACCEPTED")
    check("prior attempt with no provider lineage", lambda r: r["a"].update(attempt="1"), "ACCEPTED")

"""The cross-implementation vectors of LexLean's κ-labels (SPEC.md §21.1,
`AR-16`), written by an implementation independent of the compiler: this
script and the reference `blake3` package for Python.

    python -I tests/kappa/generate.py > tests/kappa/vectors.json

Every vector states its recipe's inputs, the exact carrier bytes in hex, and
the κ-label. A κ-label is `blake3:` and the 64 lowercase hexadecimal digits of
the BLAKE3 hash of the carrier; a leaf's carrier is its bytes, and a framed
recipe's carrier is its domain, a NUL, and its frames, each frame the label's
length as four big-endian bytes, the label, the payload's length as eight
big-endian bytes, and the payload. An identity framed into another is framed as
its 71 label bytes. The compiler, the reference `blake3` crate, and uor-addr's
`blake3` σ-axis each reproduce every carrier and label (`conformance_ar_16`).
"""
import json
import struct
import sys

import blake3

TOOLCHAIN = "leanprover/lean4:v4.32.1"


def kappa(carrier: bytes) -> str:
    return "blake3:" + blake3.blake3(carrier).hexdigest()


def frame(label: str, payload: bytes) -> bytes:
    name = label.encode("utf-8")
    return struct.pack(">I", len(name)) + name + struct.pack(">Q", len(payload)) + payload


def framed(domain: str, frames: list) -> bytes:
    return domain.encode("utf-8") + b"\x00" + b"".join(frame(label, payload) for label, payload in frames)


def vector(kind: str, recipe: str, inputs: dict, carrier: bytes) -> dict:
    return {"kind": kind, "recipe": recipe, "inputs": inputs, "carrier": carrier.hex(), "kappa": kappa(carrier)}


vectors = []

# A leaf: the address of bytes, which Hologram's store re-derives.
for text in ["", "abc", "LexLean"]:
    vectors.append(vector("leaf", "raw bytes", {"bytes": text.encode().hex()}, text.encode()))

# A tree (§11.5): rows in bytewise path order.
rows = [("entries/nat.toml", b"[entry]\nid = \"nat\"\n"), ("lexicon.toml", b"spec = \"lexlean/lexicon/1\"\n")]
vectors.append(vector(
    "tree", "lexlean-tree-v2",
    {"rows": [{"path": path, "bytes": data.hex()} for path, data in rows]},
    framed("lexlean-tree-v2", rows),
))

# A source ID (§21.3).
project = b"spec = \"lexlean/project/2\"\nname = \"demo\"\n"
lock = b"spec = \"lexlean/lock/4\"\nlanguage = \"1.2\"\n"
sources = [("src/Main.lex.tex", b"\\begin{lexlean}{Main}\n\\end{lexlean}\n")]
source_frames = [("project", project), ("lock", lock)]
for path, data in sources:
    source_frames += [("path", path.encode()), ("source", data)]
source_carrier = framed("lexlean-source-v2", source_frames)
vectors.append(vector(
    "source", "lexlean-source-v2",
    {"project": project.hex(), "lock": lock.hex(), "sources": [{"path": p, "bytes": d.hex()} for p, d in sources]},
    source_carrier,
))

# A semantic ID (§21.4): the compiler-semantics κ-label is framed as its label.
compiler_semantics = kappa(b"compiler semantics of the vector")
linked_ir = '{"modules":[]}'
closure = '{"module":"","packages":[],"spec":"lexlean/lexicon-closure/2"}'
semantic_carrier = framed("lexlean-semantic-v2", [
    ("compiler-semantics", compiler_semantics.encode()),
    ("language", b"1.2"),
    ("toolchain", TOOLCHAIN.encode()),
    ("linked-ir", linked_ir.encode()),
    ("lexicon-closure", closure.encode()),
])
vectors.append(vector(
    "semantic", "lexlean-semantic-v2",
    {"compiler_semantics": compiler_semantics, "language": "1.2", "toolchain": TOOLCHAIN, "linked_ir": linked_ir, "lexicon_closure": closure},
    semantic_carrier,
))

# A build ID (§21.5).
build_carrier = framed("lexlean-build-v2", [("source-id", kappa(source_carrier).encode()), ("semantic-id", kappa(semantic_carrier).encode())])
vectors.append(vector("build", "lexlean-build-v2", {"source_id": kappa(source_carrier), "semantic_id": kappa(semantic_carrier)}, build_carrier))

# A declaration ID (§21.9): a group of one declaration that references one
# declaration outside it, then the member's ID.
declaration = '{"kind":"definition","name":"double","parameters":[{"name":"n","type":{"kind":"nat"}}],"result":{"kind":"nat"}}'
reference_id = kappa(b"the declaration small")
group_carrier = framed("lexlean-declaration-group-v2", [
    ("compiler-semantics", compiler_semantics.encode()),
    ("language", b"1.2"),
    ("toolchain", TOOLCHAIN.encode()),
    ("declaration", declaration.encode()),
    ("reference", b"small"),
    ("reference-id", reference_id.encode()),
])
vectors.append(vector(
    "declaration-group", "lexlean-declaration-group-v2",
    {"compiler_semantics": compiler_semantics, "language": "1.2", "toolchain": TOOLCHAIN, "declarations": [declaration], "references": [{"name": "small", "id": reference_id}]},
    group_carrier,
))
member_carrier = framed("lexlean-declaration-v2", [("group", kappa(group_carrier).encode()), ("name", b"double")])
vectors.append(vector("declaration", "lexlean-declaration-v2", {"group": kappa(group_carrier), "name": "double"}, member_carrier))

# A lexicon entry's signature (§13.7): the leaf of its canonical printed form.
signature = "(fun (n : Nat) Nat)"
vectors.append(vector("entry-signature", "raw bytes", {"signature": signature}, signature.encode()))

document = {"spec": "lexlean/kappa-vectors/1", "axis": "blake3", "generator": "tests/kappa/generate.py with the reference blake3 package " + blake3.__version__, "vectors": vectors}
# Bytes, so that no host's text mode changes the line ends.
sys.stdout.buffer.write((json.dumps(document, indent=2, sort_keys=True, ensure_ascii=False) + "\n").encode("utf-8"))

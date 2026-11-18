#!/usr/bin/env bash
# The browser prover's assets for one release (SPEC.md §30.3): the runtime's
# `.olean` files built by the pinned Lean, the WASM prover's driver, and the
# `lexlean` executable for wasm32-wasip1, each labelled by the BLAKE3 κ of its
# raw bytes. Run from the repository root with the pinned Lean (elan) and Rust
# toolchains, `b3sum`, and `python3`:
#
#   .github/lean-wasm.sh VERSION OUT
#
# OUT receives `lexlean-VERSION-lean-wasm.tar.gz`, `lexlean-VERSION.wasm.gz`,
# and `lean-wasm.json`, which names every file with its κ. Each artifact is
# built twice in distinct directories and refused unless the two agree byte
# for byte, so what is published is what any host rebuilds.
set -euo pipefail

version="${1:?usage: lean-wasm.sh VERSION OUT}"
out="${2:?usage: lean-wasm.sh VERSION OUT}"
root="$(pwd)"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
mkdir -p "$out"
stage="$work/stage"
mkdir -p "$stage/runtime" "$stage/driver"

# The runtime: the Lean this compiler generates from the shipped native
# sources, compiled by Lake twice. Lake's traces and setup files record the
# build directory, so only what Lean itself writes is compared and published.
for run in a b; do
  cargo xtask runtime-package --output "$work/runtime-$run"
  (cd "$work/runtime-$run" && lake build)
done
while read -r module; do
  relative="${module//.//}"
  for extension in olean olean.server olean.private ir ilean; do
    built="lib/lean/$relative.$extension"
    a="$work/runtime-a/.lake/build/$built"
    b="$work/runtime-b/.lake/build/$built"
    if [ ! -f "$a" ]; then
      # A module has an `.olean` and its `.ilean` whatever it declares; the
      # module-system parts and the IR are written only when there are any.
      case "$extension" in olean | ilean) echo "::error::$built was not built" >&2; exit 1 ;; esac
      continue
    fi
    cmp "$a" "$b" || { echo "::error::$built differs between two builds" >&2; exit 1; }
    mkdir -p "$stage/runtime/$(dirname "$relative")"
    cp "$a" "$stage/runtime/$relative.$extension"
  done
done < "$work/runtime-a/modules.txt"

# The driver: its committed bytes, published as they are, once Lake has built
# both executables from them with the pinned Lean.
cp -R lean/wasm-prover/driver "$work/driver-build"
(cd "$work/driver-build" && lake build elab symlist)
for file in Elab.lean SymList.lean lakefile.toml lean-toolchain; do
  cp "lean/wasm-prover/driver/$file" "$stage/driver/$file"
done

# The WASI executable, without `production-data` (§8.4), from two clean target
# directories; gzip without a name or time, so its bytes are fixed too.
for run in a b; do
  CARGO_TARGET_DIR="$work/wasm-$run" cargo build --locked --release \
    --target wasm32-wasip1 -p lexlean --bin lexlean --no-default-features
done
wasm_a="$work/wasm-a/wasm32-wasip1/release/lexlean.wasm"
cmp "$wasm_a" "$work/wasm-b/wasm32-wasip1/release/lexlean.wasm" \
  || { echo "::error::lexlean.wasm differs between two clean builds" >&2; exit 1; }
cp "$wasm_a" "$work/lexlean.wasm"
gzip -9 -n -c "$work/lexlean.wasm" > "$out/lexlean-$version.wasm.gz"

# One archive of the runtime and the driver, with fixed order, owners, and
# times.
tar --sort=name --mtime=@0 --owner=0 --group=0 --numeric-owner \
  -C "$stage" -cf - runtime driver | gzip -9 -n > "$out/lexlean-$version-lean-wasm.tar.gz"

# The labels: κ of each file's raw bytes, with its SHA-256 beside it.
python3 - "$version" "$stage" "$work/lexlean.wasm" "$out" "$root" <<'PYTHON'
import hashlib, json, os, subprocess, sys

version, stage, wasm, out, root = sys.argv[1:]

def kappa(path):
    return "blake3:" + subprocess.run(["b3sum", "--no-names", path], check=True,
                                      capture_output=True, text=True).stdout.strip()

def row(asset, name, path):
    data = open(path, "rb").read()
    return {"asset": asset, "path": name, "bytes": len(data), "kappa": kappa(path),
            "sha256": hashlib.sha256(data).hexdigest()}

archive = f"lexlean-{version}-lean-wasm.tar.gz"
gz = f"lexlean-{version}.wasm.gz"
files = []
for top in ("runtime", "driver"):
    for directory, _, names in sorted(os.walk(os.path.join(stage, top))):
        for name in sorted(names):
            path = os.path.join(directory, name)
            files.append(row(archive, os.path.relpath(path, stage).replace(os.sep, "/"), path))
files.append(row(gz, "lexlean.wasm", wasm))
files.sort(key=lambda r: (r["asset"], r["path"]))
assets = [row(name, name, os.path.join(out, name)) for name in sorted([archive, gz])]
lean = subprocess.run(["lean", "--version"], check=True, capture_output=True, text=True).stdout.strip()
commit = subprocess.run(["git", "-C", root, "rev-parse", "HEAD"], check=True,
                        capture_output=True, text=True).stdout.strip()
document = {
    "spec": "lexlean/lean-wasm-assets/1",
    "lexlean_version": version,
    "lexlean_commit": commit,
    "lean_toolchain": open(os.path.join(root, "lean-toolchain")).read().strip(),
    "lean_version": lean,
    "wasm_build": "cargo build --locked --release --target wasm32-wasip1 -p lexlean --bin lexlean --no-default-features; gzip -9 -n",
    "assets": [{k: v for k, v in a.items() if k != "path"} for a in assets],
    "files": files,
}
with open(os.path.join(out, "lean-wasm.json"), "w", newline="\n") as handle:
    json.dump(document, handle, indent=2, sort_keys=True)
    handle.write("\n")
print(json.dumps(document["assets"], indent=2))
PYTHON

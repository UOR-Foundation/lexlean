# Shared-root resolution regression inputs

These are small inputs for VR-03, VR-08 and VR-15. They preserve the shape of
Rule30's reported failure: generated `Rule30UOR.GeneratedPd5e66423d6416ea1419f4591e6818cfd48e9f2db8677cd84c7fe6b91d5ae406b`
imports `Rule30UOR.NativeAcceptanceFixture`. The short names here keep the
namespace-selection issue separate from theorem synthesis.

Compile Base into an imported library root and Generated into a distinct staged
root with the exact pinned Lean 4.32.1. Neither search-path order can resolve
both objects using Lean's first-top-level-root rule. The canonical verifier must
use one confined, identity-bound view for compilation, replay and this audit.
The original library must remain byte-identical. An empty axiom set is required.

Required negative controls: missing Base, corrupt Base against its recorded
hash, duplicate full module paths (including equal bytes), substituted companion
files, and a symlink escaping either input or view. No negative may produce a
verified attestation. Retain actual process records, source/tool identities and
view inventory; a standalone successful replay is not compiler acceptance.

These inputs are not a passed test or a production fix. Canonical pipeline
integration and the pinned-tool runtime gate are separate obligations.

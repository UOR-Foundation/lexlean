//! Fixed source for a Lake-hosted invocation consumer, not execution wiring.
//!
//! Pinned API basis: Lean 4.32.1 Lean.Data.Json.{Parser,Printer,Basic} and
//! Init.System.IO. This source has not been compiled with that toolchain.
//! Before running it Rust must validate Descriptor, all tool/source/view hashes,
//! producer associations, canonical root bindings, support import closure and
//! byte/OS argument budgets. In particular Lake must not shadow the helper's
//! Init/Lean/Std imports. These checks cannot be delegated to code that has
//! already imported those modules. No cryptographic verification is claimed
//! here: hash strings are shape-checked and their authority remains in Rust.
//!
//! Protocol: --lexlean-invocation-chain-1 DESCRIPTOR_JSON ROOT_BINDINGS_JSON
//! CAPTURED_LEAN_PATH. Bindings are a canonical object with the four named roots
//! and optional library:<digest> keys, each an existing absolute canonical
//! directory. Absent/noncanonical directories refuse. Typed captured entries
//! are compared to realPath of the actual entries; empty entries remain empty.
//! Final LEAN_PATH preserves the original raw captured string, including empty
//! entries. Filesystem race protection remains the caller's private-view duty.

/// Immutable source; no user text is interpolated into Lean syntax.
pub(crate) const SOURCE: &str = r#"import Lean.Data.Json.Parser
import Lean.Data.Json.Printer
import Init.System.IO

open Lean System

namespace LexLeanInvocation

private def refuse {α : Type} : IO α :=
  throw <| IO.userError "LexLean invocation chain refused"

private def need (ok : Bool) : IO Unit := do
  unless ok do refuse

private def take {α : Type} (x : Except String α) : IO α :=
  match x with
  | .ok value => pure value
  | .error _ => refuse

private def field (j : Json) (key : String) : IO Json := take (j.getObjVal? key)
private def text (j : Json) : IO String := take j.getStr?
private def items (j : Json) : IO (Array Json) := take j.getArr?
private def flag (j : Json) : IO Bool := take j.getBool?
private def number (j : Json) : IO Nat := do
  let n ← take j.getNat?
  need (n ≤ 9223372036854775807)
  return n

private def shape (j : Json) (keys : List String) : IO Unit := do
  let obj ← take j.getObj?
  let actual := obj.foldl (init := ([] : List String)) fun acc k _ => k :: acc
  need (actual.length == keys.length && keys.all actual.contains)

private def noNul (s : String) : Bool := !s.toList.any (fun c => c.toNat == 0)
private def hash (j : Json) : IO Unit := do
  let s ← text j
  need (s.length == 64 && s.toList.all fun c =>
    ('0' ≤ c && c ≤ '9') || ('a' ≤ c && c ≤ 'f'))

private def component (s : String) : Bool :=
  !s.isEmpty && s != "." && s != ".." && s.toList.all (fun c =>
    c.toNat ≥ 32 && !(127 ≤ c.toNat && c.toNat ≤ 159) && c != '/' && c != '\\' && c != ':')

private def rootName (s : String) : IO Unit := do
  if ["toolchain", "workspace", "project", "staging"].contains s then return
  need (s.startsWith "library:")
  hash (.str (s.drop 8 |>.toString))

-- Rust canonical JSON uses short escapes for b/f/t as well as n/r.
-- Lean Json.compress uses different escapes for b/f/t, so is not substituted.
private def quoted (s : String) : String :=
  let body := s.foldl (fun out c =>
    if c == '"' then out ++ "\\\""
    else if c == '\\' then out ++ "\\\\"
    else if c.toNat == 8 then out ++ "\\b"
    else if c.toNat == 12 then out ++ "\\f"
    else if c == '\n' then out ++ "\\n"
    else if c == '\r' then out ++ "\\r"
    else if c == '\t' then out ++ "\\t"
    else if c.toNat < 32 then
      (out ++ "\\u00").push (Nat.digitChar (c.toNat / 16)) |>.push (Nat.digitChar (c.toNat % 16))
    else out.push c) ""
  "\"" ++ body ++ "\""

private partial def canonical (j : Json) : IO String := do
  match j with
  | .null => refuse
  | .bool b => return if b then "true" else "false"
  | .num _ => return toString (← number j)
  | .str s => return quoted s
  | .arr a => return "[" ++ String.intercalate "," (← a.toList.mapM canonical) ++ "]"
  | .obj obj =>
    let rows := obj.foldl (init := ([] : List (String × Json))) fun acc k v => (k,v) :: acc
    let fields ← rows.reverse.mapM fun (k,v) => do
      return quoted k ++ ":" ++ (← canonical v)
    return "{" ++ String.intercalate "," fields ++ "}"

private def parse (raw : String) : IO Json := do
  -- Same nesting bound as the Rust codec; byte admission belongs to its caller.
  let mut depth := 0
  let mut inString := false
  let mut escaped := false
  for c in raw.toList do
    if inString then
      if escaped then escaped := false
      else if c == '\\' then escaped := true
      else if c == '"' then inString := false
    else if c == '"' then inString := true
    else if c == '{' || c == '[' then
      depth := depth + 1
      need (depth ≤ 16)
    else if c == '}' || c == ']' then
      need (depth > 0)
      depth := depth - 1
  need (depth == 0 && !inString)
  let j ← take (Json.parse raw)
  -- Duplicate keys, escaped aliases, unsorted keys and noncanonical numbers
  -- cannot survive exact canonical reserialization equality.
  need ((← canonical j) == raw)
  return j

private def pathParts (j : Json) : IO (String × Array String) := do
  shape j ["root", "relative"]
  let root ← text (← field j "root")
  rootName root
  let parts ← (← items (← field j "relative")).mapM text
  need (parts.all component)
  return (root, parts)

private def bindings (j : Json) : IO Unit := do
  let obj ← take j.getObj?
  for key in ["toolchain", "workspace", "project", "staging"] do
    let _ ← field j key
  let rows := obj.foldl (init := ([] : List (String × Json))) fun acc k v => (k,v) :: acc
  for (key, value) in rows do
    rootName key
    let raw ← text value
    need (noNul raw)
    let p := FilePath.mk raw
    need p.isAbsolute
    need ((← IO.FS.realPath p).toString == raw)
    need (← p.isDir)

private def resolve (roots j : Json) : IO FilePath := do
  let (root, parts) ← pathParts j
  let base ← text (← field roots root)
  return parts.foldl (fun p c => p / c) (FilePath.mk base)

private def existing (p : FilePath) : IO Unit := do
  need ((← IO.FS.realPath p) == p)

private def tool (roots j : Json) (allowed : List String) : IO (String × FilePath) := do
  shape j ["kind", "path", "sha256"]
  let kind ← text (← field j "kind")
  need (allowed.contains kind)
  hash (← field j "sha256")
  let pj ← field j "path"
  let (root, parts) ← pathParts pj
  need (root == "toolchain" && !parts.isEmpty)
  let p ← resolve roots pj
  existing p
  return (kind, p)

private def pathRows (j : Json) : IO (Bool × Array Json) := do
  shape j ["present", "entries"]
  let present ← flag (← field j "present")
  let rows ← items (← field j "entries")
  need (present || rows.isEmpty)
  for row in rows do
    if row != Json.str "" then
      let _ ← pathParts row
  return (present, rows)

private def validate (roots d : Json) : IO Unit := do
  shape d ["spec", "context", "lake", "launcher", "source", "target",
    "arguments", "cwd", "environment", "view"]
  need ((← text (← field d "spec")) == "lexlean/invocation-chain/1")
  let context ← field d "context"
  shape context ["source_id", "semantic_id", "build_id"]
  for key in ["source_id", "semantic_id", "build_id"] do hash (← field context key)
  let _ ← tool roots (← field d "lake") ["lake"]
  let (_, launcher) ← tool roots (← field d "launcher") ["lean"]
  need ((← IO.FS.realPath (← IO.appPath)) == launcher)
  let _ ← tool roots (← field d "target") ["lean", "leanchecker"]
  let source ← field d "source"
  shape source ["path", "bytes", "sha256"]
  hash (← field source "sha256")
  need ((← number (← field source "bytes")) > 0)
  let sp ← field source "path"
  let (sr, sc) ← pathParts sp
  need (sr == "staging" && !sc.isEmpty)
  existing (← resolve roots sp)
  let cwd ← field d "cwd"
  let (cr, _) ← pathParts cwd
  need (cr == "workspace")
  need ((← IO.FS.realPath (← IO.currentDir)) == (← resolve roots cwd))
  let view ← field d "view"
  shape view ["root", "objects", "sha256"]
  hash (← field view "sha256")
  let vr ← field view "root"
  let (rk, parts) ← pathParts vr
  need (rk == "staging" && !parts.isEmpty)
  let vp ← resolve roots vr
  existing vp
  need (← vp.isDir)
  let mut last := ""
  let mut producers : List (String × Json) := []
  let mut primaries : List String := []
  for obj in (← items (← field view "objects")) do
    shape obj ["module", "relative", "bytes", "sha256", "producer"]
    let name ← text (← field obj "module")
    need (!name.isEmpty && (name.splitOn ".").all component)
    let rel ← text (← field obj "relative")
    need ((rel.splitOn "/").all component && last < rel)
    last := rel
    let stem := String.intercalate "/" (name.splitOn ".")
    need ([".olean", ".olean.private", ".olean.server", ".ir"].any fun ext => rel == stem ++ ext)
    if rel == stem ++ ".olean" then primaries := name :: primaries
    let _ ← number (← field obj "bytes")
    hash (← field obj "sha256")
    let producer ← field obj "producer"
    shape producer ["kind", "sha256"]
    need (["imported", "generated"].contains (← text (← field producer "kind")))
    hash (← field producer "sha256")
    for (oldName, oldProducer) in producers do
      if oldName == name then need (oldProducer == producer)
    producers := (name, producer) :: producers
  need (producers.all fun p => primaries.contains p.1)
  -- Rust, not this consumer, computes framed view_digest and checks file bytes.

private def environment (roots d : Json) (captured : String) : IO String := do
  need ((← IO.getEnv "LEAN_PATH") == some captured)
  let env ← field d "environment"
  shape env ["captured", "effective"]
  let (present, old) ← pathRows (← field env "captured")
  let (effectivePresent, effective) ← pathRows (← field env "effective")
  need (present && effectivePresent)
  let vr ← field (← field d "view") "root"
  need (effective == #[vr] ++ old)
  let separator := if System.Platform.isWindows then ";" else ":"
  let actual := captured.splitOn separator
  need (actual.length == old.size)
  for (entry, row) in actual.zip old.toList do
    if row == Json.str "" then need entry.isEmpty
    else
      need (!entry.isEmpty)
      let expected ← resolve roots row
      existing expected
      need ((← IO.FS.realPath (FilePath.mk entry)) == expected)
  let view ← resolve roots vr
  return view.toString ++ separator ++ captured

private def argument (roots j : Json) : IO (Bool × String) := do
  shape j ["kind", "value"]
  let kind ← text (← field j "kind")
  let value ← field j "value"
  if kind == "literal" then
    let s ← text value
    need (noNul s)
    return (false, s)
  need (kind == "path")
  let p ← resolve roots value
  -- A typed argument may name a root for -R, but never traverses out of it.
  return (true, p.toString)

private def request (roots d : Json) : IO (FilePath × Array String) := do
  let (kind, target) ← tool roots (← field d "target") ["lean", "leanchecker"]
  let argv ← (← items (← field d "arguments")).mapM (argument roots)
  if kind == "leanchecker" then
    match argv.toList with
    | [(false, name)] =>
      need (!name.startsWith "-" && !name.isEmpty &&
        (name.splitOn ".").all (fun s => component s && !s.toList.any Char.isWhitespace))
    | _ => refuse
  else
    let (root?, output?, source) ← match argv.toList with
      | [(true, source)] => pure (none, none, source)
      | [(false, "-R"), (true, root), (true, source)] => pure (some root, none, source)
      | [(false, "-R"), (true, root), (false, "-o"), (true, output), (true, source)] =>
        pure (some root, some output, source)
      | _ => refuse
    need (source.endsWith ".lean")
    existing (FilePath.mk source)
    if let some root := root? then
      existing (FilePath.mk root)
      need (← (FilePath.mk root).isDir)
    if let some output := output? then
      need (output.endsWith ".olean")
      let p := FilePath.mk output
      let some parent := p.parent | refuse
      existing parent
      if ← p.pathExists then existing p
  return (target, argv.map Prod.snd)

private def execute (args : List String) : IO UInt32 := do
  let ["--lexlean-invocation-chain-1", descriptor, rootBindings, captured] := args | refuse
  let d ← parse descriptor
  let roots ← parse rootBindings
  bindings roots
  validate roots d
  let leanPath ← environment roots d captured
  let (target, argv) ← request roots d
  let child ← IO.Process.spawn {
    cmd := target.toString, args := argv, cwd := none,
    stdin := .inherit, stdout := .inherit, stderr := .inherit,
    env := #[("LEAN_PATH", some leanPath)], inheritEnv := true, setsid := false }
  child.wait

end LexLeanInvocation

def main (args : List String) : IO UInt32 := do
  try LexLeanInvocation.execute args
  catch _ =>
    IO.eprintln "LexLean invocation chain refused"
    return 1
"#;

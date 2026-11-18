/-
Lean's own kernel over produced kernel terms (SPEC.md §18.10), with nothing but the pinned Lean.

  lean --run KernelCheck.lean <suffix.ndjson> <module>...

Imports the named modules (the project's generated modules, compiled by the pinned Lean with every olean part, on
LEAN_PATH), reads a
self-contained lean4export 3.1.0 file (one whose numbering starts empty), and adds each theorem it declares
through `Kernel.Environment.addDeclCore`, the C++ kernel, in order. Nothing in the file is believed: a theorem is
reported accepted only if the kernel adds it. For each one it prints

  accepted <name> axioms: <every axiom it rests on, sorted, comma separated>
  refused <name>: <the kernel's reason>

and exits 0 when every theorem was accepted, 1 otherwise.
-/
import Lean

open Lean

namespace KernelCheck

structure Tables where
  names : Std.HashMap Nat Name := ({} : Std.HashMap Nat Name).insert 0 .anonymous
  levels : Std.HashMap Nat Level := ({} : Std.HashMap Nat Level).insert 0 .zero
  exprs : Std.HashMap Nat Expr := {}

def nat (j : Json) (key : String) : Except String Nat := do
  (← j.getObjVal? key).getNat?

def name (t : Tables) (j : Json) (key : String) : Except String Name := do
  let id ← nat j key
  match t.names[id]? with
  | some n => pure n
  | none => throw s!"name {id} is not stated"

def level (t : Tables) (id : Nat) : Except String Level :=
  match t.levels[id]? with
  | some l => pure l
  | none => throw s!"level {id} is not stated"

def expr (t : Tables) (j : Json) (key : String) : Except String Expr := do
  let id ← nat j key
  match t.exprs[id]? with
  | some e => pure e
  | none => throw s!"expression {id} is not stated"

/-- The kernel's refusal, by kind: pretty-printing a term needs the elaborator's printer. -/
def describe : Kernel.Exception → String
  | .unknownConstant _ n => s!"unknown constant '{n}'"
  | .alreadyDeclared _ n => s!"constant has already been declared '{n}'"
  | .declTypeMismatch .. => "declaration type mismatch"
  | .declHasMVars _ n _ => s!"declaration has metavariables '{n}'"
  | .declHasFVars _ n _ => s!"declaration has free variables '{n}'"
  | .funExpected .. => "function expected"
  | .typeExpected .. => "type expected"
  | .letTypeMismatch .. => "let-declaration type mismatch"
  | .exprTypeMismatch .. => "type mismatch"
  | .appTypeMismatch .. => "application type mismatch"
  | .invalidProj .. => "invalid projection"
  | .thmTypeIsNotProp _ n _ => s!"type of theorem '{n}' is not a proposition"
  | .other msg => msg
  | .deterministicTimeout => "deterministic timeout"
  | .excessiveMemory => "excessive memory consumption detected"
  | .deepRecursion => "deep recursion detected"
  | .interrupted => "interrupted"

/-- Every axiom a constant rests on, through everything it refers to. -/
partial def axiomsOf (env : Kernel.Environment) (n : Name) : StateM NameSet NameSet := do
  if (← get).contains n then return {}
  modify (·.insert n)
  let mut found : NameSet := {}
  if let some ci := env.find? n then
    if ci matches .axiomInfo _ then found := found.insert n
    for c in ci.getUsedConstantsAsSet do
      for a in (← axiomsOf env c) do
        found := found.insert a
  return found

def step (t : Tables) (j : Json) : Except String (Tables × Option (Name × Expr × Expr)) := do
  if let .ok id := nat j "in" then
    if let .ok s := j.getObjVal? "str" then
      let pre ← name t s "pre"
      return ({ t with names := t.names.insert id (.str pre (← (← s.getObjVal? "str").getStr?)) }, none)
    let n ← j.getObjVal? "num"
    let pre ← name t n "pre"
    return ({ t with names := t.names.insert id (.num pre (← nat n "i")) }, none)
  if let .ok id := nat j "il" then
    let l ← if let .ok s := nat j "succ" then pure (Level.succ (← level t s))
      else if let .ok p := j.getObjVal? "param" then pure (Level.param (← name t (Json.mkObj [("p", p)]) "p"))
      else if let .ok m := j.getObjVal? "max" then
        pure (Level.max (← level t (← (← m.getArrVal? 0).getNat?)) (← level t (← (← m.getArrVal? 1).getNat?)))
      else if let .ok m := j.getObjVal? "imax" then
        pure (Level.imax (← level t (← (← m.getArrVal? 0).getNat?)) (← level t (← (← m.getArrVal? 1).getNat?)))
      else throw "a level other than succ, max, imax, or param"
    return ({ t with levels := t.levels.insert id l }, none)
  if let .ok id := nat j "ie" then
    let e ←
      if let .ok b := nat j "bvar" then pure (Expr.bvar b)
      else if let .ok l := nat j "sort" then pure (Expr.sort (← level t l))
      else if let .ok c := j.getObjVal? "const" then do
        let us ← (← (← c.getObjVal? "us").getArr?).toList.mapM fun u => do level t (← u.getNat?)
        pure (Expr.const (← name t c "name") us)
      else if let .ok a := j.getObjVal? "app" then pure (Expr.app (← expr t a "fn") (← expr t a "arg"))
      else if let .ok b := j.getObjVal? "lam" then
        pure (Expr.lam (← name t b "name") (← expr t b "type") (← expr t b "body") .default)
      else if let .ok b := j.getObjVal? "forallE" then
        pure (Expr.forallE (← name t b "name") (← expr t b "type") (← expr t b "body") .default)
      else if let .ok b := j.getObjVal? "letE" then
        pure (Expr.letE (← name t b "name") (← expr t b "type") (← expr t b "value") (← expr t b "body") false)
      else if let .ok v := j.getObjVal? "natVal" then pure (Expr.lit (.natVal (← v.getStr?).toNat!))
      else if let .ok v := j.getObjVal? "strVal" then pure (Expr.lit (.strVal (← v.getStr?)))
      else if let .ok p := j.getObjVal? "proj" then
        pure (Expr.proj (← name t p "typeName") (← nat p "idx") (← expr t p "struct"))
      else throw s!"an expression form this checker does not read: {j.compress}"
    return ({ t with exprs := t.exprs.insert id e }, none)
  if let .ok d := j.getObjVal? "thm" then
    return (t, some (← name t d "name", ← expr t d "type", ← expr t d "value"))
  throw s!"a line this checker does not read: {j.compress.take 120}"

end KernelCheck

open KernelCheck in
def main (args : List String) : IO UInt32 := do
  let file :: modules := args | throw <| .userError "usage: KernelCheck <suffix.ndjson> <module>..."
  -- The sysroot is the running Lean's own: `findSysroot` would ask whichever `lean` is on PATH.
  initSearchPath ((← IO.appDir) / "..")
  let env ← importModules (modules.toArray.map fun m => { module := m.toName }) {} (trustLevel := 1024)
  let mut kernel := env.toKernelEnv
  let mut tables : Tables := {}
  let mut ok := true
  for line in (← IO.FS.lines file) do
    if line.isEmpty then continue
    let json ← IO.ofExcept (Json.parse line)
    let (next, decl) ← IO.ofExcept (step tables json)
    tables := next
    if let some (n, type, value) := decl then
      let d := Declaration.thmDecl { name := n, levelParams := [], type, value, all := [n] }
      match kernel.addDeclCore 0 d (cancelTk? := none) with
      | .ok env' =>
        kernel := env'
        let (found, _) := (axiomsOf kernel n).run {}
        let names := (found.toList.map toString).toArray.qsort (· < ·)
        IO.println s!"accepted {n} axioms: {", ".intercalate names.toList}"
      | .error ex =>
        ok := false
        IO.println s!"refused {n}: {describe ex}"
  return if ok then 0 else 1

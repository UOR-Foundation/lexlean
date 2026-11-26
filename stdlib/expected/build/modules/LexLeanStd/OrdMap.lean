module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Order
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanStd.OrdMap

public inductive OrdMap (Value : Type) where
  | leaf
  | node (_ : OrdMap (Value)) (_ : ByteArray) (_ : Value) (_ : Nat) (_ : OrdMap (Value))

@[expose] public def emptyMap (Value : Type) : OrdMap (Value) := OrdMap.leaf

@[expose] public def heightOf (Value : Type) (map : OrdMap (Value)) : Nat := (match map with | OrdMap.leaf => 0 | OrdMap.node _ _ _ height _ => height)

@[expose] public def larger (a : Nat) (b : Nat) : Nat := (if (Nat.blt (a) (b)) then b else a)

@[expose] public def joinNode (Value : Type) (left : OrdMap (Value)) (key : ByteArray) (value : Value) (right : OrdMap (Value)) : OrdMap (Value) := OrdMap.node (left) (key) (value) ((larger (heightOf (Value) (left)) (heightOf (Value) (right)) + 1)) (right)

@[expose] public def rotateFromLeft (Value : Type) (left : OrdMap (Value)) (key : ByteArray) (value : Value) (right : OrdMap (Value)) : OrdMap (Value) := (match left with | OrdMap.leaf => joinNode (Value) (left) (key) (value) (right) | OrdMap.node outer lkey lvalue _ inner => (if (Nat.blt (heightOf (Value) (inner)) ((heightOf (Value) (outer) + 1))) then joinNode (Value) (outer) (lkey) (lvalue) (joinNode (Value) (inner) (key) (value) (right)) else (match inner with | OrdMap.leaf => joinNode (Value) (left) (key) (value) (right) | OrdMap.node innerLeft ikey ivalue _ innerRight => joinNode (Value) (joinNode (Value) (outer) (lkey) (lvalue) (innerLeft)) (ikey) (ivalue) (joinNode (Value) (innerRight) (key) (value) (right)))))

@[expose] public def rotateFromRight (Value : Type) (left : OrdMap (Value)) (key : ByteArray) (value : Value) (right : OrdMap (Value)) : OrdMap (Value) := (match right with | OrdMap.leaf => joinNode (Value) (left) (key) (value) (right) | OrdMap.node inner rkey rvalue _ outer => (if (Nat.blt (heightOf (Value) (inner)) ((heightOf (Value) (outer) + 1))) then joinNode (Value) (joinNode (Value) (left) (key) (value) (inner)) (rkey) (rvalue) (outer) else (match inner with | OrdMap.leaf => joinNode (Value) (left) (key) (value) (right) | OrdMap.node innerLeft ikey ivalue _ innerRight => joinNode (Value) (joinNode (Value) (left) (key) (value) (innerLeft)) (ikey) (ivalue) (joinNode (Value) (innerRight) (rkey) (rvalue) (outer)))))

@[expose] public def balanced (Value : Type) (left : OrdMap (Value)) (key : ByteArray) (value : Value) (right : OrdMap (Value)) : OrdMap (Value) := (let hl : Nat := heightOf (Value) (left); (let hr : Nat := heightOf (Value) (right); (if (Nat.blt ((hr + 1)) (hl)) then rotateFromLeft (Value) (left) (key) (value) (right) else (if (Nat.blt ((hl + 1)) (hr)) then rotateFromRight (Value) (left) (key) (value) (right) else joinNode (Value) (left) (key) (value) (right)))))

@[expose] public def mapGet (Value : Type) : (map : OrdMap (Value)) -> (key : ByteArray) -> Option (Value)
  | OrdMap.leaf, _key => Option.none
  | OrdMap.node left here value _ right, key => (if (LexLeanRuntime.equal (key) (here) : Bool) then Option.some (value) else (if LexLeanStd.Order.lessBytes (key) (here) then mapGet (Value) (left) (key) else mapGet (Value) (right) (key)))

@[expose] public def mapInsert (Value : Type) : (map : OrdMap (Value)) -> (key : ByteArray) -> (value : Value) -> OrdMap (Value)
  | OrdMap.leaf, key, value => OrdMap.node (OrdMap.leaf) (key) (value) (1) (OrdMap.leaf)
  | OrdMap.node left here held height right, key, value => (if (LexLeanRuntime.equal (key) (here) : Bool) then OrdMap.node (left) (key) (value) (height) (right) else (if LexLeanStd.Order.lessBytes (key) (here) then balanced (Value) (mapInsert (Value) (left) (key) (value)) (here) (held) (right) else balanced (Value) (left) (here) (held) (mapInsert (Value) (right) (key) (value))))

@[expose] public def takeLeast (Value : Type) : (map : OrdMap (Value)) -> (Prod (Option ((Prod (ByteArray) (Value)))) (OrdMap (Value)))
  | OrdMap.leaf => (Option.none, OrdMap.leaf)
  | OrdMap.node left key value _ right => (match left with | OrdMap.leaf => (Option.some ((key, value)), right) | OrdMap.node _ _ _ _ _ => (let taken : (Prod (Option ((Prod (ByteArray) (Value)))) (OrdMap (Value))) := takeLeast (Value) (left); ((taken).1, balanced (Value) ((taken).2) (key) (value) (right))))

@[expose] public def joinSides (Value : Type) (left : OrdMap (Value)) (right : OrdMap (Value)) : OrdMap (Value) := (let taken : (Prod (Option ((Prod (ByteArray) (Value)))) (OrdMap (Value))) := takeLeast (Value) (right); (match (taken).1 with | Option.none => left | Option.some least => balanced (Value) (left) ((least).1) ((least).2) ((taken).2)))

@[expose] public def mapRemove (Value : Type) : (map : OrdMap (Value)) -> (key : ByteArray) -> OrdMap (Value)
  | OrdMap.leaf, _key => OrdMap.leaf
  | OrdMap.node left here value _ right, key => (if (LexLeanRuntime.equal (key) (here) : Bool) then joinSides (Value) (left) (right) else (if LexLeanStd.Order.lessBytes (key) (here) then balanced (Value) (mapRemove (Value) (left) (key)) (here) (value) (right) else balanced (Value) (left) (here) (value) (mapRemove (Value) (right) (key))))

@[expose] public def entriesInto (Value : Type) : (map : OrdMap (Value)) -> (rest : List ((Prod (ByteArray) (Value)))) -> List ((Prod (ByteArray) (Value)))
  | OrdMap.leaf, rest => rest
  | OrdMap.node left key value _ right, rest => entriesInto (Value) (left) (((key, value) :: entriesInto (Value) (right) (rest)))

@[expose] public def mapEntries (Value : Type) (map : OrdMap (Value)) : List ((Prod (ByteArray) (Value))) := entriesInto (Value) (map) (([] : List ((Prod (ByteArray) (Value)))))

@[expose] public def mapSize (Value : Type) : (map : OrdMap (Value)) -> Nat
  | OrdMap.leaf => 0
  | OrdMap.node left _ _ _ right => ((mapSize (Value) (left) + mapSize (Value) (right)) + 1)

@[expose] public def mapFromList (Value : Type) (entries : List ((Prod (ByteArray) (Value)))) : OrdMap (Value) := (LexLeanCollections.listFold ((fun (map : OrdMap (Value)) (entry : (Prod (ByteArray) (Value))) => mapInsert (Value) (map) ((entry).1) ((entry).2))) (emptyMap (Value)) (entries) : OrdMap (Value))

@[expose] public def shapeOf (Value : Type) : (map : OrdMap (Value)) -> (Prod (Bool) (Nat))
  | OrdMap.leaf => (true, 0)
  | OrdMap.node left _ _ height right => (let l : (Prod (Bool) (Nat)) := shapeOf (Value) (left); (let r : (Prod (Bool) (Nat)) := shapeOf (Value) (right); (let tall : Nat := (larger ((l).2) ((r).2) + 1); ((if (l).1 then (if (r).1 then (if (LexLeanRuntime.equal (height) (tall) : Bool) then (if (Nat.blt ((l).2) (((r).2 + 2))) then (Nat.blt ((r).2) (((l).2 + 2))) else false) else false) else false) else false), tall))))

@[expose] public def ascendingKeys (Value : Type) (entries : List ((Prod (ByteArray) (Value)))) : Bool := ((LexLeanCollections.listFold ((fun (state : (Prod (Bool) (Option (ByteArray)))) (entry : (Prod (ByteArray) (Value))) => (match (state).2 with | Option.none => ((state).1, Option.some ((entry).1)) | Option.some last => ((if (state).1 then LexLeanStd.Order.lessBytes (last) ((entry).1) else false), Option.some ((entry).1))))) ((true, Option.none)) (entries) : (Prod (Bool) (Option (ByteArray))))).1

@[expose] public def isOrdMap (Value : Type) (map : OrdMap (Value)) : Bool := (if (shapeOf (Value) (map)).1 then ascendingKeys (Value) (mapEntries (Value) (map)) else false)

@[expose] public def runStep (state : (Prod (List (Option (ByteArray))) (OrdMap (ByteArray)))) (op : (Prod (Nat) ((Prod (ByteArray) (ByteArray))))) : (Prod (List (Option (ByteArray))) (OrdMap (ByteArray))) := (let map : OrdMap (ByteArray) := (state).2; (let key : ByteArray := ((op).2).1; (let before : Option (ByteArray) := mapGet (ByteArray) (map) (key); (let seen : List (Option (ByteArray)) := (before :: (state).1); (if (Nat.beq ((op).1) (0)) then (seen, mapInsert (ByteArray) (map) (key) (((op).2).2)) else (if (Nat.beq ((op).1) (1)) then (seen, mapRemove (ByteArray) (map) (key)) else (seen, map)))))))

@[expose] public def runOps (ops : List ((Prod (Nat) ((Prod (ByteArray) (ByteArray)))))) : (Prod (List (Option (ByteArray))) (OrdMap (ByteArray))) := (LexLeanCollections.listFold ((fun (state : (Prod (List (Option (ByteArray))) (OrdMap (ByteArray)))) (op : (Prod (Nat) ((Prod (ByteArray) (ByteArray))))) => runStep (state) (op))) ((([] : List (Option (ByteArray))), emptyMap (ByteArray))) (ops) : (Prod (List (Option (ByteArray))) (OrdMap (ByteArray))))

@[expose] public def ordMapRun (ops : List ((Prod (Nat) ((Prod (ByteArray) (ByteArray)))))) : (Prod (List (Option (ByteArray))) (List ((Prod (ByteArray) (ByteArray))))) := (let ran : (Prod (List (Option (ByteArray))) (OrdMap (ByteArray))) := runOps (ops); (LexLeanStd.Order.reverseList (Option (ByteArray)) ((ran).1), mapEntries (ByteArray) ((ran).2)))

@[expose] public def ordMapBalanced (ops : List ((Prod (Nat) ((Prod (ByteArray) (ByteArray)))))) : Bool := isOrdMap (ByteArray) ((runOps (ops)).2)

end LexLeanStd.OrdMap

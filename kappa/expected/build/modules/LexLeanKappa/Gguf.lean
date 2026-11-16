module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanKappa.Axis
public import LexLeanKappa.Json
public import LexLeanKappa.Label
public import LexLeanStd.Sha256
public import LexLeanStd.Text
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanKappa.Gguf

public structure Frame where
  tag : Nat
  remaining : Nat
  depth : Nat

public structure Measure where
  position : Nat
  ok : Bool
  stack : List (Frame)

public structure Section where
  position : Nat
  ok : Bool
  alignment : Nat
  entries : List ((Prod (ByteArray) (ByteArray)))

public structure Tensor where
  name : ByteArray
  head : ByteArray
  offset : Nat
  size : Nat

public structure Tensors where
  position : Nat
  ok : Bool
  entries : List (Tensor)

public structure Placing where
  done : Bool
  acc : List ((Prod (ByteArray) (ByteArray)))

public structure TensorPlacing where
  done : Bool
  acc : List (Tensor)

public structure Emit where
  ok : Bool
  offset : Nat
  out : ByteArray

@[expose] public def readLe (subject : ByteArray) (position : Nat) (width : Nat) : Nat := ((LexLeanCollections.iterate ((fun (state : (Prod (Nat) (Nat))) => (let index : Nat := (LexLeanRuntime.subtract ((state).1) (1) : Nat); (index, ((LexLeanRuntime.multiply ((state).2) (256) : Nat) + LexLeanKappa.Label.byteNat (subject) ((position + index))))))) (width) ((width, 0)) : (Prod (Nat) (Nat)))).2

@[expose] public def writeLe (value : Nat) (width : Nat) : ByteArray := ((LexLeanCollections.iterate ((fun (state : (Prod (Nat) (ByteArray))) => ((LexLeanRuntime.quotient ((state).1) (256) (0) : Nat), (LexLeanRuntime.append ((state).2) (LexLeanKappa.Json.byteOf ((LexLeanRuntime.remainder ((state).1) (256) (0) : Nat))) : ByteArray)))) (width) ((value, _root_.ByteArray.mk #[])) : (Prod (Nat) (ByteArray)))).2

@[expose] public def fits (subject : ByteArray) (position : Nat) (count : Nat) : Bool := (let total : Nat := (LexLeanRuntime.length (subject) : Nat); ((Nat.ble (position) (total)) && (Nat.ble (count) ((LexLeanRuntime.subtract (total) (position) : Nat)))))

@[expose] public def digest (message : ByteArray) : ByteArray := (let hex : ByteArray := LexLeanStd.Sha256.sha256Hex (message); ((LexLeanCollections.iterate ((fun (state : (Prod (Nat) (ByteArray))) => (let offset : Nat := (LexLeanRuntime.multiply ((state).1) (2) : Nat); (((state).1 + 1), (LexLeanRuntime.append ((state).2) (LexLeanKappa.Json.byteOf (((LexLeanRuntime.multiply (LexLeanKappa.Json.hexValue (LexLeanKappa.Label.byteNat (hex) (offset))) (16) : Nat) + LexLeanKappa.Json.hexValue (LexLeanKappa.Label.byteNat (hex) ((offset + 1)))))) : ByteArray))))) (32) ((0, _root_.ByteArray.mk #[])) : (Prod (Nat) (ByteArray)))).2)

@[expose] public def alignUp (offset : Nat) (alignment : Nat) : Nat := (let rest : Nat := (LexLeanRuntime.remainder (offset) (alignment) (0) : Nat); (if (Nat.beq (rest) (0)) then offset else (offset + (LexLeanRuntime.subtract (alignment) (rest) : Nat))))

@[expose] public def isPowerOfTwo (value : Nat) : Bool := (Nat.beq (((LexLeanCollections.iterateUntil ((fun (current : Nat) => (if ((Nat.blt (1) (current)) && (Nat.beq ((LexLeanRuntime.remainder (current) (2) (0) : Nat)) (0))) then Option.some ((LexLeanRuntime.quotient (current) (2) (0) : Nat)) else Option.none))) (64) (value) : (Prod (Nat) (Bool)))).1) (1))

@[expose] public def scalarWidth (tag : Nat) : Nat := (if ((Nat.beq (tag) (0)) || ((Nat.beq (tag) (1)) || (Nat.beq (tag) (7)))) then 1 else (if ((Nat.beq (tag) (2)) || (Nat.beq (tag) (3))) then 2 else (if ((Nat.beq (tag) (4)) || ((Nat.beq (tag) (5)) || (Nat.beq (tag) (6)))) then 4 else (if ((Nat.beq (tag) (10)) || ((Nat.beq (tag) (11)) || (Nat.beq (tag) (12)))) then 8 else 0))))

@[expose] public def measured (position : Nat) (stack : List (Frame)) : Option (Measure) := Option.some (({ position := position, ok := true, stack := stack } : Measure))

@[expose] public def refused (position : Nat) : Option (Measure) := Option.some (({ position := position, ok := false, stack := ([] : List (Frame)) } : Measure))

@[expose] public def element (subject : ByteArray) (position : Nat) (top : Frame) (stack : List (Frame)) : Option (Measure) := (let tag : Nat := (top).tag; (let width : Nat := scalarWidth (tag); (if (Nat.blt (0) (width)) then (if fits (subject) (position) (width) then measured ((position + width)) (stack) else refused (position)) else (if (Nat.beq (tag) (8)) then (if fits (subject) (position) (8) then (let size : Nat := readLe (subject) (position) (8); (if fits (subject) ((position + 8)) (size) then measured (((position + 8) + size)) (stack) else refused (position))) else refused (position)) else (if (Nat.beq (tag) (9)) then (if (Nat.ble (64) ((top).depth)) then refused (position) else (if fits (subject) (position) (12) then measured ((position + 12)) ((({ tag := readLe (subject) (position) (4), remaining := readLe (subject) ((position + 4)) (8), depth := ((top).depth + 1) } : Frame) :: stack)) else refused (position))) else refused (position))))))

@[expose] public def measureStep (subject : ByteArray) (state : Measure) : Option (Measure) := (if (state).ok then (match (state).stack with | List.nil => Option.none | List.cons top rest => (if (Nat.beq ((top).remaining) (0)) then measured ((state).position) (rest) else element (subject) ((state).position) (top) ((({ tag := (top).tag, remaining := (LexLeanRuntime.subtract ((top).remaining) (1) : Nat), depth := (top).depth } : Frame) :: rest)))) else Option.none)

@[expose] public def measure (subject : ByteArray) (position : Nat) (tag : Nat) : (Prod (Bool) (Nat)) := (let state : Measure := ((LexLeanCollections.iterateUntil ((fun (state : Measure) => measureStep (subject) (state))) (((LexLeanRuntime.multiply ((LexLeanRuntime.length (subject) : Nat)) (2) : Nat) + 4)) (({ position := position, ok := true, stack := (({ tag := tag, remaining := 1, depth := 0 } : Frame) :: ([] : List (Frame))) } : Measure)) : (Prod (Measure) (Bool)))).1; (if (state).ok then (match (state).stack with | List.nil => (true, (state).position) | List.cons _ _ => (false, 0)) else (false, 0)))

@[expose] public def valueBody (subject : ByteArray) (tag : Nat) («at» : Nat) (span : Nat) : ByteArray := (if (Nat.blt (0) (scalarWidth (tag))) then LexLeanStd.Text.sliceOr (subject) («at») (span) else (if (Nat.beq (tag) (8)) then (LexLeanRuntime.append (LexLeanStd.Text.sliceOr (subject) («at») (8)) (digest (LexLeanStd.Text.sliceOr (subject) ((«at» + 8)) ((LexLeanRuntime.subtract (span) (8) : Nat)))) : ByteArray) else (LexLeanRuntime.append (LexLeanStd.Text.sliceOr (subject) («at») (12)) (digest (LexLeanStd.Text.sliceOr (subject) ((«at» + 12)) ((LexLeanRuntime.subtract (span) (12) : Nat)))) : ByteArray)))

@[expose] public def reversePairs (items : List ((Prod (ByteArray) (ByteArray)))) : List ((Prod (ByteArray) (ByteArray))) := (LexLeanCollections.listFold ((fun (reversed : List ((Prod (ByteArray) (ByteArray)))) (item : (Prod (ByteArray) (ByteArray))) => (item :: reversed))) (([] : List ((Prod (ByteArray) (ByteArray))))) (items) : List ((Prod (ByteArray) (ByteArray))))

@[expose] public def placeEntry (key : ByteArray) (body : ByteArray) (items : List ((Prod (ByteArray) (ByteArray)))) : List ((Prod (ByteArray) (ByteArray))) := (let state : Placing := (LexLeanCollections.listFold ((fun (state : Placing) (item : (Prod (ByteArray) (ByteArray))) => (if (state).done then ({ done := true, acc := (item :: (state).acc) } : Placing) else (if LexLeanKappa.Json.lessKey (key) ((item).1) then ({ done := true, acc := (item :: ((key, body) :: (state).acc)) } : Placing) else ({ done := false, acc := (item :: (state).acc) } : Placing))))) (({ done := false, acc := ([] : List ((Prod (ByteArray) (ByteArray)))) } : Placing)) (items) : Placing); (if (state).done then reversePairs ((state).acc) else reversePairs (((key, body) :: (state).acc))))

@[expose] public def reverseTensors (items : List (Tensor)) : List (Tensor) := (LexLeanCollections.listFold ((fun (reversed : List (Tensor)) (item : Tensor) => (item :: reversed))) (([] : List (Tensor))) (items) : List (Tensor))

@[expose] public def placeTensor (tensor : Tensor) (items : List (Tensor)) : List (Tensor) := (let state : TensorPlacing := (LexLeanCollections.listFold ((fun (state : TensorPlacing) (item : Tensor) => (if (state).done then ({ done := true, acc := (item :: (state).acc) } : TensorPlacing) else (if LexLeanKappa.Json.lessKey ((tensor).name) ((item).name) then ({ done := true, acc := (item :: (tensor :: (state).acc)) } : TensorPlacing) else ({ done := false, acc := (item :: (state).acc) } : TensorPlacing))))) (({ done := false, acc := ([] : List (Tensor)) } : TensorPlacing)) (items) : TensorPlacing); (if (state).done then reverseTensors ((state).acc) else reverseTensors ((tensor :: (state).acc))))

@[expose] public def failSection (state : Section) : Section := ({ position := (state).position, ok := false, alignment := (state).alignment, entries := ([] : List ((Prod (ByteArray) (ByteArray)))) } : Section)

@[expose] public def pairStep (subject : ByteArray) (state : Section) (key : ByteArray) (tag : Nat) («at» : Nat) : Section := (let span : (Prod (Bool) (Nat)) := measure (subject) («at») (tag); (if (span).1 then (let alignment : Nat := (if ((LexLeanRuntime.equal (key) (LexLeanKappa.Label.text ("general.alignment")) : Bool) && (Nat.beq (tag) (4))) then readLe (subject) («at») (4) else (state).alignment); (if ((Nat.ble (8) (alignment)) && isPowerOfTwo (alignment)) then ({ position := (span).2, ok := true, alignment := alignment, entries := placeEntry (key) ((LexLeanRuntime.append (digest (key)) ((LexLeanRuntime.append (writeLe (tag) (4)) (valueBody (subject) (tag) («at») ((LexLeanRuntime.subtract ((span).2) («at») : Nat))) : ByteArray)) : ByteArray)) ((state).entries) } : Section) else failSection (state))) else failSection (state)))

@[expose] public def sectionStep (subject : ByteArray) (state : Section) : Section := (if (state).ok then (let position : Nat := (state).position; (if fits (subject) (position) (8) then (let size : Nat := readLe (subject) (position) (8); (if (fits (subject) ((position + 8)) (size) && fits (subject) (((position + 8) + size)) (4)) then pairStep (subject) (state) (LexLeanStd.Text.sliceOr (subject) ((position + 8)) (size)) (readLe (subject) (((position + 8) + size)) (4)) ((((position + 8) + size) + 4)) else failSection (state))) else failSection (state))) else state)

@[expose] public def blockOf (kind : Nat) : (Prod (Nat) (Nat)) := (let table : ByteArray := (let __byte0 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 0); let __byte1 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 1); let __byte2 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 2); let __byte3 : _root_.UInt8 := _root_.UInt8.ofNat (nat_lit 3); _root_.ByteArray.mk #[__byte0, (_root_.UInt8.ofNat (nat_lit 4)), __byte1, __byte0, __byte2, __byte1, __byte0, (_root_.UInt8.ofNat (nat_lit 18)), __byte2, __byte0, (_root_.UInt8.ofNat (nat_lit 20)), __byte2, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, __byte0, (_root_.UInt8.ofNat (nat_lit 22)), __byte2, __byte0, (_root_.UInt8.ofNat (nat_lit 24)), __byte2, __byte0, (_root_.UInt8.ofNat (nat_lit 34)), __byte2, __byte0, (_root_.UInt8.ofNat (nat_lit 36)), __byte2, __byte0, (_root_.UInt8.ofNat (nat_lit 84)), __byte3, __byte0, (_root_.UInt8.ofNat (nat_lit 110)), __byte3, __byte0, (_root_.UInt8.ofNat (nat_lit 144)), __byte3, __byte0, (_root_.UInt8.ofNat (nat_lit 176)), __byte3, __byte0, (_root_.UInt8.ofNat (nat_lit 210)), __byte3, __byte1, (_root_.UInt8.ofNat (nat_lit 36)), __byte3, __byte0, (_root_.UInt8.ofNat (nat_lit 66)), __byte3, __byte0, (_root_.UInt8.ofNat (nat_lit 74)), __byte3, __byte0, (_root_.UInt8.ofNat (nat_lit 98)), __byte3, __byte0, (_root_.UInt8.ofNat (nat_lit 50)), __byte3, __byte0, (_root_.UInt8.ofNat (nat_lit 18)), __byte2, __byte0, (_root_.UInt8.ofNat (nat_lit 110)), __byte3, __byte0, (_root_.UInt8.ofNat (nat_lit 82)), __byte3, __byte0, (_root_.UInt8.ofNat (nat_lit 136)), __byte3, __byte0, __byte1, __byte1, __byte0, __byte2, __byte1, __byte0, (_root_.UInt8.ofNat (nat_lit 4)), __byte1, __byte0, (_root_.UInt8.ofNat (nat_lit 8)), __byte1, __byte0, (_root_.UInt8.ofNat (nat_lit 8)), __byte1, __byte0, (_root_.UInt8.ofNat (nat_lit 56)), __byte3, __byte0, __byte2, __byte1]); (let «at» : Nat := (LexLeanRuntime.multiply (kind) (3) : Nat); (let code : Nat := (if (Nat.blt (kind) (31)) then LexLeanKappa.Label.byteNat (table) ((«at» + 2)) else 0); (((LexLeanRuntime.multiply (LexLeanKappa.Label.byteNat (table) («at»)) (256) : Nat) + LexLeanKappa.Label.byteNat (table) ((«at» + 1))), (if (Nat.beq (code) (1)) then 1 else (if (Nat.beq (code) (2)) then 32 else (if (Nat.beq (code) (3)) then 256 else 0)))))))

@[expose] public def elements (subject : ByteArray) («at» : Nat) (count : Nat) : (Prod (Bool) (Nat)) := ((LexLeanCollections.iterate ((fun (state : (Prod (Nat) ((Prod (Bool) (Nat))))) => (let dimension : Nat := readLe (subject) ((«at» + (LexLeanRuntime.multiply ((state).1) (8) : Nat))) (8); (let product : Nat := ((state).2).2; (let fitting : Bool := (((state).2).1 && ((Nat.beq (dimension) (0)) || (Nat.ble (product) ((LexLeanRuntime.quotient (18446744073709551615) (dimension) (0) : Nat))))); (((state).1 + 1), (fitting, (if fitting then (LexLeanRuntime.multiply (product) (dimension) : Nat) else 0)))))))) (count) ((0, (true, 1))) : (Prod (Nat) ((Prod (Bool) (Nat)))))).2

@[expose] public def failTensors (state : Tensors) : Tensors := ({ position := (state).position, ok := false, entries := ([] : List (Tensor)) } : Tensors)

@[expose] public def tensorInfo (subject : ByteArray) (state : Tensors) (name : ByteArray) («at» : Nat) : Tensors := (let rank : Nat := readLe (subject) («at») (4); (if ((Nat.ble (rank) (4)) && fits (subject) ((«at» + 4)) (((LexLeanRuntime.multiply (rank) (8) : Nat) + 12))) then (let count : (Prod (Bool) (Nat)) := elements (subject) ((«at» + 4)) (rank); (let kindAt : Nat := ((«at» + 4) + (LexLeanRuntime.multiply (rank) (8) : Nat)); (let layout : (Prod (Nat) (Nat)) := blockOf (readLe (subject) (kindAt) (4)); (let per : Nat := (layout).2; (if ((count).1 && ((Nat.blt (0) (per)) && (Nat.beq ((LexLeanRuntime.remainder ((count).2) (per) (0) : Nat)) (0)))) then (let blocks : Nat := (LexLeanRuntime.quotient ((count).2) (per) (0) : Nat); (if (Nat.ble (blocks) ((LexLeanRuntime.quotient (18446744073709551615) ((layout).1) (0) : Nat))) then ({ position := (kindAt + 12), ok := true, entries := placeTensor (({ name := name, head := LexLeanStd.Text.sliceOr (subject) («at») (((LexLeanRuntime.multiply (rank) (8) : Nat) + 8)), offset := readLe (subject) ((kindAt + 4)) (8), size := (LexLeanRuntime.multiply (blocks) ((layout).1) : Nat) } : Tensor)) ((state).entries) } : Tensors) else failTensors (state))) else failTensors (state)))))) else failTensors (state)))

@[expose] public def tensorStep (subject : ByteArray) (state : Tensors) : Tensors := (if (state).ok then (let position : Nat := (state).position; (if fits (subject) (position) (8) then (let size : Nat := readLe (subject) (position) (8); (if (fits (subject) ((position + 8)) (size) && fits (subject) (((position + 8) + size)) (4)) then tensorInfo (subject) (state) (LexLeanStd.Text.sliceOr (subject) ((position + 8)) (size)) (((position + 8) + size)) else failTensors (state))) else failTensors (state))) else state)

@[expose] public def emitTensor (subject : ByteArray) (start : Nat) (alignment : Nat) (state : Emit) (tensor : Tensor) : Emit := (if ((state).ok && ((Nat.ble ((tensor).offset) ((LexLeanRuntime.length (subject) : Nat))) && (Nat.ble ((state).offset) ((LexLeanRuntime.subtract (18446744073709551615) ((tensor).size) : Nat))))) then (let begin : Nat := (start + (tensor).offset); (if fits (subject) (begin) ((tensor).size) then ({ ok := true, offset := alignUp (((state).offset + (tensor).size)) (alignment), out := (LexLeanRuntime.append ((state).out) ((LexLeanRuntime.append (digest ((tensor).name)) ((LexLeanRuntime.append ((tensor).head) ((LexLeanRuntime.append (writeLe ((state).offset) (8)) (digest (LexLeanStd.Text.sliceOr (subject) (begin) ((tensor).size))) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray) } : Emit) else ({ ok := false, offset := 0, out := _root_.ByteArray.mk #[] } : Emit))) else ({ ok := false, offset := 0, out := _root_.ByteArray.mk #[] } : Emit))

@[expose] public def pairBytes (entries : List ((Prod (ByteArray) (ByteArray)))) : ByteArray := (LexLeanCollections.listFold ((fun (out : ByteArray) (entry : (Prod (ByteArray) (ByteArray))) => (LexLeanRuntime.append (out) ((entry).2) : ByteArray))) (_root_.ByteArray.mk #[]) (entries) : ByteArray)

@[expose] public def skeleton (subject : ByteArray) («section» : Section) (tensors : Tensors) : Option (ByteArray) := (let alignment : Nat := («section»).alignment; (let start : Nat := alignUp ((tensors).position) (alignment); (let emitted : Emit := (LexLeanCollections.listFold ((fun (state : Emit) (tensor : Tensor) => emitTensor (subject) (start) (alignment) (state) (tensor))) (({ ok := true, offset := 0, out := _root_.ByteArray.mk #[] } : Emit)) ((tensors).entries) : Emit); (if (emitted).ok then Option.some ((LexLeanRuntime.append (LexLeanStd.Text.sliceOr (subject) (0) (24)) ((LexLeanRuntime.append (writeLe (alignment) (8)) ((LexLeanRuntime.append (pairBytes ((«section»).entries)) ((emitted).out) : ByteArray)) : ByteArray)) : ByteArray)) else Option.none))))

@[expose] public def canonicalGguf (subject : ByteArray) : Option (ByteArray) := (let total : Nat := (LexLeanRuntime.length (subject) : Nat); (if ((Nat.ble (24) (total)) && (LexLeanRuntime.equal (LexLeanStd.Text.sliceOr (subject) (0) (8)) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 71)), (_root_.UInt8.ofNat (nat_lit 71)), (_root_.UInt8.ofNat (nat_lit 85)), (_root_.UInt8.ofNat (nat_lit 70)), (_root_.UInt8.ofNat (nat_lit 3)), (_root_.UInt8.ofNat (nat_lit 0)), (_root_.UInt8.ofNat (nat_lit 0)), (_root_.UInt8.ofNat (nat_lit 0))]) : Bool)) then (let tensorCount : Nat := readLe (subject) (8) (8); (let pairCount : Nat := readLe (subject) (16) (8); (if ((Nat.ble (pairCount) (total)) && (Nat.ble (tensorCount) (total))) then (let «section» : Section := (LexLeanCollections.iterate ((fun (state : Section) => sectionStep (subject) (state))) (pairCount) (({ position := 24, ok := true, alignment := 32, entries := ([] : List ((Prod (ByteArray) (ByteArray)))) } : Section)) : Section); (if («section»).ok then (let tensors : Tensors := (LexLeanCollections.iterate ((fun (state : Tensors) => tensorStep (subject) (state))) (tensorCount) (({ position := («section»).position, ok := true, entries := ([] : List (Tensor)) } : Tensors)) : Tensors); (if (tensors).ok then skeleton (subject) («section») (tensors) else Option.none)) else Option.none)) else Option.none))) else Option.none))

@[expose] public def ggufLabel (subject : ByteArray) : Option (ByteArray) := (match canonicalGguf (subject) with | Option.some canonical => Option.some (LexLeanKappa.Label.kappaLabel (canonical)) | Option.none => Option.none)

@[expose] public def ggufLabelOn (axis : ByteArray) (subject : ByteArray) : Option (ByteArray) := (match canonicalGguf (subject) with | Option.some canonical => LexLeanKappa.Axis.labelOn (axis) (canonical) | Option.none => Option.none)

end LexLeanKappa.Gguf

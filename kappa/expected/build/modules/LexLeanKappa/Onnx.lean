module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanKappa.Axis
public import LexLeanKappa.Gguf
public import LexLeanKappa.Json
public import LexLeanKappa.Label
public import LexLeanStd.Text
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanKappa.Onnx

public structure Field where
  number : Nat
  wire : Nat
  value : Nat
  payload : ByteArray

public structure Varint where
  position : Nat
  value : Nat
  shift : Nat
  scale : Nat
  ok : Bool
  done : Bool

public structure Scan where
  position : Nat
  ok : Bool
  done : Bool
  fields : List (Field)

public structure Decoded where
  position : Nat
  ok : Bool
  values : List (Nat)

public structure Frame where
  fields : List (Field)
  acc : ByteArray
  depth : Nat

public structure Digesting where
  stack : List (Frame)
  result : ByteArray
  done : Bool

public structure Task where
  graph : Bool
  body : ByteArray
  depth : Nat

public structure Machine where
  ok : Bool
  out : ByteArray
  tasks : List (Task)

public structure Node where
  index : Nat
  body : ByteArray
  key : ByteArray
  inputs : List (ByteArray)
  outputs : List (ByteArray)

public structure Kahn where
  ok : Bool
  emitted : List (Nat)
  order : List (Node)

public structure Choice where
  found : Bool
  best : Node

public structure FieldPlacing where
  done : Bool
  acc : List (Field)

@[expose] public def varintStep (subject : ByteArray) (state : Varint) : Option (Varint) := (if (state).done then Option.none else (let position : Nat := (state).position; (if ((Nat.ble ((LexLeanRuntime.length (subject) : Nat)) (position)) || (Nat.ble (70) ((state).shift))) then Option.some (({ position := position, value := 0, shift := 0, scale := 1, ok := false, done := true } : Varint)) else (let byte : Nat := LexLeanKappa.Label.byteNat (subject) (position); (let low : Nat := (LexLeanRuntime.remainder (byte) (128) (0) : Nat); (let term : Nat := (if (Nat.beq ((state).shift) (63)) then (LexLeanRuntime.multiply ((LexLeanRuntime.remainder (low) (2) (0) : Nat)) ((state).scale) : Nat) else (LexLeanRuntime.multiply (low) ((state).scale) : Nat)); Option.some (({ position := (position + 1), value := ((state).value + term), shift := ((state).shift + 7), scale := (if (Nat.blt ((state).shift) (63)) then (LexLeanRuntime.multiply ((state).scale) (128) : Nat) else (state).scale), ok := true, done := (Nat.blt (byte) (128)) } : Varint))))))))

@[expose] public def varint (subject : ByteArray) (position : Nat) : (Prod (Bool) ((Prod (Nat) (Nat)))) := (let state : Varint := ((LexLeanCollections.iterateUntil ((fun (state : Varint) => varintStep (subject) (state))) (12) (({ position := position, value := 0, shift := 0, scale := 1, ok := true, done := false } : Varint)) : (Prod (Varint) (Bool)))).1; (((state).ok && (state).done), ((state).value, (state).position)))

@[expose] public def failScan (state : Scan) : Option (Scan) := Option.some (({ position := (state).position, ok := false, done := true, fields := ([] : List (Field)) } : Scan))

@[expose] public def pushField (state : Scan) (stop : Nat) (number : Nat) (wire : Nat) (value : Nat) (payload : ByteArray) (next : Nat) : Option (Scan) := Option.some (({ position := next, ok := true, done := (Nat.beq (number) (stop)), fields := (({ number := number, wire := wire, value := value, payload := payload } : Field) :: (state).fields) } : Scan))

@[expose] public def fieldValue (subject : ByteArray) (stop : Nat) (state : Scan) (number : Nat) (wire : Nat) («at» : Nat) : Option (Scan) := (if (Nat.beq (wire) (0)) then (let read : (Prod (Bool) ((Prod (Nat) (Nat)))) := varint (subject) («at»); (if (read).1 then pushField (state) (stop) (number) (0) (((read).2).1) (_root_.ByteArray.mk #[]) (((read).2).2) else failScan (state))) else (if (Nat.beq (wire) (1)) then (if LexLeanKappa.Gguf.fits (subject) («at») (8) then pushField (state) (stop) (number) (1) (LexLeanKappa.Gguf.readLe (subject) («at») (8)) (_root_.ByteArray.mk #[]) ((«at» + 8)) else failScan (state)) else (if (Nat.beq (wire) (5)) then (if LexLeanKappa.Gguf.fits (subject) («at») (4) then pushField (state) (stop) (number) (5) (LexLeanKappa.Gguf.readLe (subject) («at») (4)) (_root_.ByteArray.mk #[]) ((«at» + 4)) else failScan (state)) else (if (Nat.beq (wire) (2)) then (let read : (Prod (Bool) ((Prod (Nat) (Nat)))) := varint (subject) («at»); (let size : Nat := ((read).2).1; (let start : Nat := ((read).2).2; (if ((read).1 && LexLeanKappa.Gguf.fits (subject) (start) (size)) then pushField (state) (stop) (number) (2) (0) (LexLeanStd.Text.sliceOr (subject) (start) (size)) ((start + size)) else failScan (state))))) else failScan (state)))))

@[expose] public def scanStep (subject : ByteArray) (stop : Nat) (state : Scan) : Option (Scan) := (if ((state).done || (Nat.ble ((LexLeanRuntime.length (subject) : Nat)) ((state).position))) then Option.none else (let read : (Prod (Bool) ((Prod (Nat) (Nat)))) := varint (subject) ((state).position); (let tag : Nat := ((read).2).1; (let number : Nat := (LexLeanRuntime.quotient (tag) (8) (0) : Nat); (if ((read).1 && (Nat.blt (0) (number))) then fieldValue (subject) (stop) (state) (number) ((LexLeanRuntime.remainder (tag) (8) (0) : Nat)) (((read).2).2) else failScan (state))))))

@[expose] public def scan (subject : ByteArray) (stop : Nat) : Scan := ((LexLeanCollections.iterateUntil ((fun (state : Scan) => scanStep (subject) (stop) (state))) (((LexLeanRuntime.length (subject) : Nat) + 2)) (({ position := 0, ok := true, done := false, fields := ([] : List (Field)) } : Scan)) : (Prod (Scan) (Bool)))).1

@[expose] public def reverseFields (items : List (Field)) : List (Field) := (LexLeanCollections.listFold ((fun (reversed : List (Field)) (item : Field) => (item :: reversed))) (([] : List (Field))) (items) : List (Field))

@[expose] public def parsed (subject : ByteArray) : (Prod (Bool) (List (Field))) := (let state : Scan := scan (subject) (0); ((state).ok, reverseFields ((state).fields)))

@[expose] public def emptyField : Field := ({ number := 0, wire := 0, value := 0, payload := _root_.ByteArray.mk #[] } : Field)

@[expose] public def firstIn (fields : List (Field)) (number : Nat) : Field := (LexLeanCollections.listFold ((fun (found : Field) (item : Field) => (if (Nat.blt (0) ((found).number)) then found else (if (Nat.beq ((item).number) (number)) then item else found)))) (emptyField) (fields) : Field)

@[expose] public def bytesOf (field : Field) : ByteArray := (if (Nat.beq ((field).wire) (2)) then (field).payload else _root_.ByteArray.mk #[])

@[expose] public def varintOf (field : Field) : Nat := (if ((Nat.blt (0) ((field).number)) && (Nat.beq ((field).wire) (0))) then (field).value else 0)

@[expose] public def hasVarint (field : Field) : Bool := ((Nat.blt (0) ((field).number)) && (Nat.beq ((field).wire) (0)))

@[expose] public def firstScanned (subject : ByteArray) (number : Nat) : (Prod (Bool) (Field)) := (let state : Scan := scan (subject) (number); (if (state).ok then (match (state).fields with | List.cons last _ => (true, (if (Nat.beq ((last).number) (number)) then last else emptyField)) | List.nil => (true, emptyField)) else (false, emptyField)))

@[expose] public def spansOf (fields : List (Field)) (number : Nat) : List (ByteArray) := (LexLeanCollections.listFold ((fun (out : List (ByteArray)) (item : Field) => (if ((Nat.beq ((item).number) (number)) && (Nat.beq ((item).wire) (2))) then ((item).payload :: out) else out))) (([] : List (ByteArray))) (reverseFields (fields)) : List (ByteArray))

@[expose] public def countOf (fields : List (Field)) (number : Nat) : Nat := (LexLeanCollections.listFold ((fun (count : Nat) (item : Field) => (if (Nat.beq ((item).number) (number)) then (count + 1) else count))) (0) (fields) : Nat)

@[expose] public def digestEach (items : List (ByteArray)) : ByteArray := (LexLeanCollections.listFold ((fun (out : ByteArray) (item : ByteArray) => (LexLeanRuntime.append (out) (LexLeanKappa.Gguf.digest (item)) : ByteArray))) (_root_.ByteArray.mk #[]) (items) : ByteArray)

@[expose] public def escaped (subject : ByteArray) : ByteArray := (LexLeanRuntime.append (((LexLeanCollections.iterate ((fun (state : (Prod (Nat) (ByteArray))) => (((state).1 + 1), (LexLeanRuntime.append ((state).2) ((LexLeanRuntime.append (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 1))]) (LexLeanKappa.Json.byteOf (LexLeanKappa.Label.byteNat (subject) ((state).1))) : ByteArray)) : ByteArray)))) ((LexLeanRuntime.length (subject) : Nat)) ((0, _root_.ByteArray.mk #[])) : (Prod (Nat) (ByteArray)))).2) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 0))]) : ByteArray)

@[expose] public def writeBe (value : Nat) : ByteArray := ((LexLeanCollections.iterate ((fun (state : (Prod (Nat) (ByteArray))) => ((LexLeanRuntime.quotient ((state).1) (256) (0) : Nat), (LexLeanRuntime.append (LexLeanKappa.Json.byteOf ((LexLeanRuntime.remainder ((state).1) (256) (0) : Nat))) ((state).2) : ByteArray)))) (8) ((value, _root_.ByteArray.mk #[])) : (Prod (Nat) (ByteArray)))).2

@[expose] public def decodeStep (payload : ByteArray) (state : Decoded) : Option (Decoded) := (if ((state).ok && (Nat.blt ((state).position) ((LexLeanRuntime.length (payload) : Nat)))) then (let read : (Prod (Bool) ((Prod (Nat) (Nat)))) := varint (payload) ((state).position); (if (read).1 then Option.some (({ position := ((read).2).2, ok := true, values := (((read).2).1 :: (state).values) } : Decoded)) else Option.some (({ position := 0, ok := false, values := ([] : List (Nat)) } : Decoded)))) else Option.none)

@[expose] public def decodeAll (payload : ByteArray) : (Prod (Bool) (List (Nat))) := (let state : Decoded := ((LexLeanCollections.iterateUntil ((fun (state : Decoded) => decodeStep (payload) (state))) (((LexLeanRuntime.length (payload) : Nat) + 1)) (({ position := 0, ok := true, values := ([] : List (Nat)) } : Decoded)) : (Prod (Decoded) (Bool)))).1; ((state).ok, (state).values))

@[expose] public def packed (fields : List (Field)) (number : Nat) : (Prod (Bool) (List (Nat))) := (let state : (Prod (Bool) (List (Nat))) := (LexLeanCollections.listFold ((fun (state : (Prod (Bool) (List (Nat)))) (item : Field) => (if (Nat.beq ((item).number) (number)) then (if (Nat.beq ((item).wire) (2)) then (let decoded : (Prod (Bool) (List (Nat))) := decodeAll ((item).payload); (((state).1 && (decoded).1), (LexLeanCollections.listFold ((fun (acc : List (Nat)) (value : Nat) => (value :: acc))) ((state).2) ((LexLeanCollections.listFold ((fun (acc : List (Nat)) (value : Nat) => (value :: acc))) (([] : List (Nat))) ((decoded).2) : List (Nat))) : List (Nat)))) else (if (Nat.beq ((item).wire) (0)) then ((state).1, ((item).value :: (state).2)) else state)) else state))) ((true, ([] : List (Nat)))) (fields) : (Prod (Bool) (List (Nat)))); ((state).1, (LexLeanCollections.listFold ((fun (acc : List (Nat)) (value : Nat) => (value :: acc))) (([] : List (Nat))) ((state).2) : List (Nat))))

@[expose] public def countNats (values : List (Nat)) : Nat := (LexLeanCollections.listFold ((fun (count : Nat) (_value : Nat) => (count + 1))) (0) (values) : Nat)

@[expose] public def wideEach (values : List (Nat)) (width : Nat) : ByteArray := (LexLeanCollections.listFold ((fun (out : ByteArray) (value : Nat) => (LexLeanRuntime.append (out) (LexLeanKappa.Gguf.writeLe (value) (width)) : ByteArray))) (_root_.ByteArray.mk #[]) (values) : ByteArray)

@[expose] public def sortedBy (items : List (ByteArray)) (keys : List (ByteArray)) : List (ByteArray) := (let paired : List ((Prod (ByteArray) (ByteArray))) := ((LexLeanCollections.listFold ((fun (state : (Prod (List (ByteArray)) (List ((Prod (ByteArray) (ByteArray)))))) (item : ByteArray) => (match (state).1 with | List.cons key rest => (rest, LexLeanKappa.Gguf.placeEntry (key) (item) ((state).2)) | List.nil => (([] : List (ByteArray)), LexLeanKappa.Gguf.placeEntry (_root_.ByteArray.mk #[]) (item) ((state).2))))) ((keys, ([] : List ((Prod (ByteArray) (ByteArray)))))) (items) : (Prod (List (ByteArray)) (List ((Prod (ByteArray) (ByteArray))))))).2; (LexLeanCollections.listFold ((fun (out : List (ByteArray)) (entry : (Prod (ByteArray) (ByteArray))) => ((entry).2 :: out))) (([] : List (ByteArray))) (LexLeanKappa.Gguf.reversePairs (paired)) : List (ByteArray)))

@[expose] public def scannedKeys (items : List (ByteArray)) (number : Nat) : List (ByteArray) := (LexLeanCollections.listFold ((fun (out : List (ByteArray)) (item : ByteArray) => (bytesOf ((firstScanned (item) (number)).2) :: out))) (([] : List (ByteArray))) ((LexLeanCollections.listFold ((fun (out : List (ByteArray)) (item : ByteArray) => (item :: out))) (([] : List (ByteArray))) (items) : List (ByteArray))) : List (ByteArray))

@[expose] public def lengthOf (items : List (ByteArray)) : Nat := (LexLeanCollections.listFold ((fun (count : Nat) (_item : ByteArray) => (count + 1))) (0) (items) : Nat)

@[expose] public def pairDigests (body : ByteArray) (number : Nat) : (Prod (Bool) (ByteArray)) := (let all : (Prod (Bool) (List (Field))) := parsed (body); (let entries : List (ByteArray) := spansOf ((all).2) (number); (LexLeanCollections.listFold ((fun (state : (Prod (Bool) (ByteArray))) (entry : ByteArray) => (let key : (Prod (Bool) (Field)) := firstScanned (entry) (1); (let value : (Prod (Bool) (Field)) := firstScanned (entry) (2); (((state).1 && ((key).1 && (value).1)), (LexLeanRuntime.append ((state).2) ((LexLeanRuntime.append (LexLeanKappa.Gguf.digest (bytesOf ((key).2))) (LexLeanKappa.Gguf.digest (bytesOf ((value).2))) : ByteArray)) : ByteArray)))))) (((all).1, LexLeanKappa.Gguf.writeLe (lengthOf (entries)) (4))) (sortedBy (entries) (scannedKeys (entries) (1))) : (Prod (Bool) (ByteArray)))))

@[expose] public def placeField (field : Field) (items : List (Field)) : List (Field) := (let state : FieldPlacing := (LexLeanCollections.listFold ((fun (state : FieldPlacing) (item : Field) => (if (state).done then ({ done := true, acc := (item :: (state).acc) } : FieldPlacing) else (if (Nat.blt ((field).number) ((item).number)) then ({ done := true, acc := (item :: (field :: (state).acc)) } : FieldPlacing) else ({ done := false, acc := (item :: (state).acc) } : FieldPlacing))))) (({ done := false, acc := ([] : List (Field)) } : FieldPlacing)) (items) : FieldPlacing); (if (state).done then reverseFields ((state).acc) else reverseFields ((field :: (state).acc))))

@[expose] public def byNumber (fields : List (Field)) : List (Field) := (LexLeanCollections.listFold ((fun (sorted : List (Field)) (item : Field) => placeField (item) (sorted))) (([] : List (Field))) (fields) : List (Field))

@[expose] public def fieldHead (field : Field) : ByteArray := (let wire : Nat := (field).wire; (LexLeanRuntime.append (LexLeanKappa.Gguf.writeLe ((field).number) (8)) ((LexLeanRuntime.append (LexLeanKappa.Json.byteOf (wire)) ((if ((Nat.beq (wire) (0)) || (Nat.beq (wire) (1))) then LexLeanKappa.Gguf.writeLe ((field).value) (8) else (if (Nat.beq (wire) (5)) then LexLeanKappa.Gguf.writeLe ((field).value) (4) else _root_.ByteArray.mk #[]))) : ByteArray)) : ByteArray))

@[expose] public def absorb (stack : List (Frame)) (digest : ByteArray) : Digesting := (match stack with | List.nil => ({ stack := ([] : List (Frame)), result := digest, done := true } : Digesting) | List.cons parent rest => ({ stack := (({ fields := (parent).fields, acc := (LexLeanRuntime.append ((parent).acc) (digest) : ByteArray), depth := (parent).depth } : Frame) :: rest), result := _root_.ByteArray.mk #[], done := false } : Digesting))

@[expose] public def digestStep (state : Digesting) : Option (Digesting) := (if (state).done then Option.none else (match (state).stack with | List.nil => Option.none | List.cons top rest => (match (top).fields with | List.nil => Option.some (absorb (rest) (LexLeanKappa.Gguf.digest ((top).acc))) | List.cons field later => (let acc : ByteArray := (LexLeanRuntime.append ((top).acc) (fieldHead (field)) : ByteArray); (let depth : Nat := (top).depth; (let payload : ByteArray := (field).payload; (let inner : (Prod (Bool) (List (Field))) := parsed (payload); (let nested : Bool := (((Nat.beq ((field).wire) (2)) && (Nat.blt (depth) (32))) && ((Nat.blt (0) ((LexLeanRuntime.length (payload) : Nat))) && (inner).1)); (if nested then Option.some (({ stack := (({ fields := byNumber ((inner).2), acc := _root_.ByteArray.mk #[], depth := (depth + 1) } : Frame) :: (({ fields := later, acc := acc, depth := depth } : Frame) :: rest)), result := _root_.ByteArray.mk #[], done := false } : Digesting)) else Option.some (({ stack := (({ fields := later, acc := (if (Nat.beq ((field).wire) (2)) then (LexLeanRuntime.append (acc) (LexLeanKappa.Gguf.digest (payload)) : ByteArray) else acc), depth := depth } : Frame) :: rest), result := _root_.ByteArray.mk #[], done := false } : Digesting)))))))))))

@[expose] public def protoDigest (body : ByteArray) : (Prod (Bool) (ByteArray)) := (let all : (Prod (Bool) (List (Field))) := parsed (body); (let state : Digesting := ((LexLeanCollections.iterateUntil ((fun (state : Digesting) => digestStep (state))) (((LexLeanRuntime.multiply ((LexLeanRuntime.length (body) : Nat)) (40) : Nat) + 8)) (({ stack := (({ fields := byNumber ((all).2), acc := _root_.ByteArray.mk #[], depth := 0 } : Frame) :: ([] : List (Frame))), result := _root_.ByteArray.mk #[], done := false } : Digesting)) : (Prod (Digesting) (Bool)))).1; (((all).1 && (state).done), (state).result)))

@[expose] public def dataKind (kind : Nat) : (Prod (Nat) (Nat)) := (if ((Nat.beq (kind) (1)) || (Nat.beq (kind) (14))) then (4, 0) else (if ((Nat.beq (kind) (11)) || (Nat.beq (kind) (15))) then (10, 0) else (if (Nat.beq (kind) (7)) then (7, 8) else (if (Nat.beq (kind) (13)) then (11, 8) else (if (Nat.beq (kind) (12)) then (11, 4) else (if (Nat.beq (kind) (6)) then (5, 4) else (if ((Nat.beq (kind) (4)) || ((Nat.beq (kind) (5)) || ((Nat.beq (kind) (10)) || (Nat.beq (kind) (16))))) then (5, 2) else (if (Nat.beq (kind) (8)) then (6, 0) else (5, 1)))))))))

@[expose] public def fixedPayload (fields : List (Field)) (number : Nat) : ByteArray := (LexLeanCollections.listFold ((fun (out : ByteArray) (item : Field) => (if (Nat.beq ((item).number) (number)) then (if (Nat.beq ((item).wire) (2)) then (LexLeanRuntime.append (out) ((item).payload) : ByteArray) else (if (Nat.beq ((item).wire) (5)) then (LexLeanRuntime.append (out) (LexLeanKappa.Gguf.writeLe ((item).value) (4)) : ByteArray) else (if (Nat.beq ((item).wire) (1)) then (LexLeanRuntime.append (out) (LexLeanKappa.Gguf.writeLe ((item).value) (8)) : ByteArray) else out))) else out))) (_root_.ByteArray.mk #[]) (fields) : ByteArray)

@[expose] public def dataDigest (body : ByteArray) (fields : List (Field)) (kind : Nat) : (Prod (Bool) (ByteArray)) := (if (Nat.beq (varintOf (firstIn (fields) (14))) (1)) then (let pairs : (Prod (Bool) (ByteArray)) := pairDigests (body) (13); ((pairs).1, LexLeanKappa.Gguf.digest ((LexLeanRuntime.append (LexLeanKappa.Label.text ("onnx:external-data:v1")) ((pairs).2) : ByteArray)))) else (let raw : Field := firstIn (fields) (9); (if ((Nat.beq ((raw).wire) (2)) && (Nat.blt (0) ((LexLeanRuntime.length ((raw).payload) : Nat)))) then (true, LexLeanKappa.Gguf.digest ((raw).payload)) else (let layout : (Prod (Nat) (Nat)) := dataKind (kind); (if (Nat.beq ((layout).2) (0)) then (if (Nat.beq (kind) (8)) then (true, LexLeanKappa.Gguf.digest (digestEach (spansOf (fields) (6)))) else (true, LexLeanKappa.Gguf.digest (fixedPayload (fields) ((layout).1)))) else (let values : (Prod (Bool) (List (Nat))) := packed (fields) ((layout).1); ((values).1, LexLeanKappa.Gguf.digest (wideEach ((values).2) ((layout).2)))))))))

@[expose] public def tensorBytes (body : ByteArray) : (Prod (Bool) (ByteArray)) := (let all : (Prod (Bool) (List (Field))) := parsed (body); (let fields : List (Field) := (all).2; (let kind : Nat := (LexLeanRuntime.remainder (varintOf (firstIn (fields) (2))) (4294967296) (0) : Nat); (let dims : (Prod (Bool) (List (Nat))) := packed (fields) (1); (let data : (Prod (Bool) (ByteArray)) := dataDigest (body) (fields) (kind); ((((all).1 && ((Nat.blt (0) (kind)) && (Nat.blt (kind) (24)))) && ((dims).1 && (data).1)), (LexLeanRuntime.append (LexLeanKappa.Gguf.digest (bytesOf (firstIn (fields) (8)))) ((LexLeanRuntime.append (LexLeanKappa.Gguf.writeLe (kind) (4)) ((LexLeanRuntime.append (LexLeanKappa.Gguf.writeLe (countNats ((dims).2)) (4)) ((LexLeanRuntime.append (wideEach ((dims).2) (8)) ((data).2) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)))))))

@[expose] public def joinEach (items : List (ByteArray)) (number : Nat) : (Prod (Bool) (ByteArray)) := (LexLeanCollections.listFold ((fun (state : (Prod (Bool) (ByteArray))) (item : ByteArray) => (let piece : (Prod (Bool) (ByteArray)) := (if (Nat.beq (number) (0)) then tensorBytes (item) else protoDigest (item)); (((state).1 && (piece).1), (LexLeanRuntime.append ((state).2) ((piece).2) : ByteArray))))) ((true, _root_.ByteArray.mk #[])) (items) : (Prod (Bool) (ByteArray)))

@[expose] public def tensorSection (fields : List (Field)) : (Prod (Bool) (ByteArray)) := (let tensors : List (ByteArray) := spansOf (fields) (5); (let joined : (Prod (Bool) (ByteArray)) := joinEach (sortedBy (tensors) (scannedKeys (tensors) (8))) (0); ((joined).1, (LexLeanRuntime.append (LexLeanKappa.Gguf.writeLe (lengthOf (tensors)) (4)) ((joined).2) : ByteArray))))

@[expose] public def valueInfos (fields : List (Field)) (number : Nat) : (Prod (Bool) (ByteArray)) := (let infos : List (ByteArray) := spansOf (fields) (number); (LexLeanCollections.listFold ((fun (state : (Prod (Bool) (ByteArray))) (info : ByteArray) => (let name : (Prod (Bool) (Field)) := firstScanned (info) (1); (let shape : (Prod (Bool) (Field)) := firstScanned (info) (2); (let typed : (Prod (Bool) (ByteArray)) := protoDigest (bytesOf ((shape).2)); (((state).1 && ((name).1 && ((shape).1 && (typed).1))), (LexLeanRuntime.append ((state).2) ((LexLeanRuntime.append (LexLeanKappa.Gguf.digest (bytesOf ((name).2))) ((typed).2) : ByteArray)) : ByteArray))))))) ((true, LexLeanKappa.Gguf.writeLe (lengthOf (infos)) (4))) (sortedBy (infos) (scannedKeys (infos) (1))) : (Prod (Bool) (ByteArray))))

@[expose] public def fixedFloats (fields : List (Field)) : ByteArray := (LexLeanCollections.listFold ((fun (out : ByteArray) (item : Field) => (if (Nat.beq ((item).number) (7)) then (if (Nat.beq ((item).wire) (2)) then (LexLeanRuntime.append (out) (LexLeanKappa.Gguf.digest ((item).payload)) : ByteArray) else (if (Nat.beq ((item).wire) (5)) then (LexLeanRuntime.append (out) (LexLeanKappa.Gguf.writeLe ((item).value) (4)) : ByteArray) else out)) else out))) (_root_.ByteArray.mk #[]) (fields) : ByteArray)

@[expose] public def attributeValue («attribute» : ByteArray) (kind : Nat) : (Prod (Bool) (ByteArray)) := (let all : (Prod (Bool) (List (Field))) := parsed («attribute»); (let fields : List (Field) := (all).2; (if (Nat.beq (kind) (1)) then (let found : (Prod (Bool) (Field)) := firstScanned («attribute») (2); ((found).1, (if (Nat.beq (((found).2).wire) (5)) then LexLeanKappa.Gguf.writeLe (((found).2).value) (4) else _root_.ByteArray.mk #[]))) else (if (Nat.beq (kind) (2)) then (let found : (Prod (Bool) (Field)) := firstScanned («attribute») (3); ((found).1, LexLeanKappa.Gguf.writeLe (varintOf ((found).2)) (8))) else (if (Nat.beq (kind) (3)) then (let found : (Prod (Bool) (Field)) := firstScanned («attribute») (4); ((found).1, LexLeanKappa.Gguf.digest (bytesOf ((found).2)))) else (if (Nat.beq (kind) (4)) then (let found : (Prod (Bool) (Field)) := firstScanned («attribute») (5); (let tensor : (Prod (Bool) (ByteArray)) := tensorBytes (bytesOf ((found).2)); (((found).1 && (tensor).1), (tensor).2))) else (if (Nat.beq (kind) (6)) then ((all).1, fixedFloats (fields)) else (if (Nat.beq (kind) (7)) then (let values : (Prod (Bool) (List (Nat))) := packed (fields) (8); (((all).1 && (values).1), wideEach ((values).2) (8))) else (if (Nat.beq (kind) (8)) then ((all).1, digestEach (spansOf (fields) (9))) else (if (Nat.beq (kind) (9)) then (let joined : (Prod (Bool) (ByteArray)) := joinEach (spansOf (fields) (10)) (0); (((all).1 && (joined).1), (joined).2)) else (if (Nat.beq (kind) (11)) then (let found : (Prod (Bool) (Field)) := firstScanned («attribute») (22); (let digest : (Prod (Bool) (ByteArray)) := protoDigest (bytesOf ((found).2)); (((found).1 && (digest).1), (digest).2))) else (if (Nat.beq (kind) (12)) then (let joined : (Prod (Bool) (ByteArray)) := joinEach (spansOf (fields) (23)) (1); (((all).1 && (joined).1), (joined).2)) else (if (Nat.beq (kind) (13)) then (let found : (Prod (Bool) (Field)) := firstScanned («attribute») (14); (let digest : (Prod (Bool) (ByteArray)) := protoDigest (bytesOf ((found).2)); (((found).1 && (digest).1), (digest).2))) else (if (Nat.beq (kind) (14)) then (let joined : (Prod (Bool) (ByteArray)) := joinEach (spansOf (fields) (15)) (1); (((all).1 && (joined).1), (joined).2)) else (true, _root_.ByteArray.mk #[])))))))))))))))

@[expose] public def literal (body : ByteArray) : Task := ({ graph := false, body := body, depth := 0 } : Task)

@[expose] public def reverseTasks (items : List (Task)) : List (Task) := (LexLeanCollections.listFold ((fun (reversed : List (Task)) (item : Task) => (item :: reversed))) (([] : List (Task))) (items) : List (Task))

@[expose] public def attributeTasks («attribute» : ByteArray) (depth : Nat) : (Prod (Bool) (List (Task))) := (let name : (Prod (Bool) (Field)) := firstScanned («attribute») (1); (let typed : (Prod (Bool) (Field)) := firstScanned («attribute») (20); (let kind : Nat := (LexLeanRuntime.remainder (varintOf ((typed).2)) (4294967296) (0) : Nat); (let head : Task := literal ((LexLeanRuntime.append (LexLeanKappa.Gguf.digest (bytesOf ((name).2))) (LexLeanKappa.Gguf.writeLe (kind) (4)) : ByteArray)); (let okHead : Bool := ((name).1 && (typed).1); (if (Nat.beq (kind) (5)) then (let found : (Prod (Bool) (Field)) := firstScanned («attribute») (6); ((okHead && (found).1), (({ graph := true, body := bytesOf ((found).2), depth := (depth + 1) } : Task) :: (head :: ([] : List (Task)))))) else (if (Nat.beq (kind) (10)) then (let all : (Prod (Bool) (List (Field))) := parsed («attribute»); ((okHead && (all).1), (LexLeanCollections.listFold ((fun (out : List (Task)) (graph : ByteArray) => (({ graph := true, body := graph, depth := (depth + 1) } : Task) :: out))) ((head :: ([] : List (Task)))) (spansOf ((all).2) (11)) : List (Task)))) else (let value : (Prod (Bool) (ByteArray)) := attributeValue («attribute») (kind); ((okHead && (value).1), (literal ((value).2) :: (head :: ([] : List (Task)))))))))))))

@[expose] public def nodeTasks (node : Node) (depth : Nat) : (Prod (Bool) (List (Task))) := (let fields : List (Field) := (parsed ((node).body)).2; (let attributes : List (ByteArray) := spansOf (fields) (5); (let head : ByteArray := (LexLeanRuntime.append (LexLeanKappa.Gguf.digest (bytesOf (firstIn (fields) (3)))) ((LexLeanRuntime.append (LexLeanKappa.Gguf.digest (bytesOf (firstIn (fields) (4)))) ((LexLeanRuntime.append (LexLeanKappa.Gguf.digest (bytesOf (firstIn (fields) (7)))) ((LexLeanRuntime.append (LexLeanKappa.Gguf.digest (bytesOf (firstIn (fields) (8)))) ((LexLeanRuntime.append (LexLeanKappa.Gguf.writeLe (countOf (fields) (1)) (4)) ((LexLeanRuntime.append (digestEach (spansOf (fields) (1))) ((LexLeanRuntime.append (LexLeanKappa.Gguf.writeLe (countOf (fields) (2)) (4)) ((LexLeanRuntime.append (digestEach (spansOf (fields) (2))) (LexLeanKappa.Gguf.writeLe (lengthOf (attributes)) (4)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray); (LexLeanCollections.listFold ((fun (state : (Prod (Bool) (List (Task)))) («attribute» : ByteArray) => (let tasks : (Prod (Bool) (List (Task))) := attributeTasks («attribute») (depth); (((state).1 && (tasks).1), (LexLeanCollections.listFold ((fun (out : List (Task)) (task : Task) => (task :: out))) ((state).2) (reverseTasks ((tasks).2)) : List (Task)))))) ((true, (literal (head) :: ([] : List (Task))))) (sortedBy (attributes) (scannedKeys (attributes) (1))) : (Prod (Bool) (List (Task)))))))

@[expose] public def containsBytes (items : List (ByteArray)) (wanted : ByteArray) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (item : ByteArray) => (found || (LexLeanRuntime.equal (item) (wanted) : Bool)))) (false) (items) : Bool)

@[expose] public def containsNat (items : List (Nat)) (wanted : Nat) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (item : Nat) => (found || (Nat.beq (item) (wanted))))) (false) (items) : Bool)

@[expose] public def isReady (nodes : List (Node)) (emitted : List (Nat)) (node : Node) : Bool := (LexLeanCollections.listFold ((fun (ready : Bool) (input : ByteArray) => (if (Nat.blt (0) ((LexLeanRuntime.length (input) : Nat))) then (LexLeanCollections.listFold ((fun (still : Bool) (producer : Node) => (if (containsBytes ((producer).outputs) (input) && (if containsNat (emitted) ((producer).index) then false else true)) then false else still))) (ready) (nodes) : Bool) else ready))) (true) ((node).inputs) : Bool)

@[expose] public def kahnStep (nodes : List (Node)) (state : Kahn) : Kahn := (if (state).ok then (let choice : Choice := (LexLeanCollections.listFold ((fun (choice : Choice) (node : Node) => (if (containsNat ((state).emitted) ((node).index) || (if isReady (nodes) ((state).emitted) (node) then false else true)) then choice else (if (choice).found then (if LexLeanKappa.Json.lessKey (((choice).best).key) ((node).key) then choice else ({ found := true, best := node } : Choice)) else ({ found := true, best := node } : Choice))))) (({ found := false, best := ({ index := 0, body := _root_.ByteArray.mk #[], key := _root_.ByteArray.mk #[], inputs := ([] : List (ByteArray)), outputs := ([] : List (ByteArray)) } : Node) } : Choice)) (nodes) : Choice); (if (choice).found then ({ ok := true, emitted := (((choice).best).index :: (state).emitted), order := ((choice).best :: (state).order) } : Kahn) else ({ ok := false, emitted := ([] : List (Nat)), order := ([] : List (Node)) } : Kahn))) else state)

@[expose] public def nodeOf (index : Nat) (body : ByteArray) : (Prod (Bool) (Node)) := (let all : (Prod (Bool) (List (Field))) := parsed (body); (let fields : List (Field) := (all).2; ((all).1, ({ index := index, body := body, key := (LexLeanRuntime.append (escaped (bytesOf (firstIn (fields) (3)))) ((LexLeanRuntime.append (escaped (bytesOf (firstIn (fields) (4)))) (escaped (bytesOf (firstIn (fields) (7)))) : ByteArray)) : ByteArray), inputs := spansOf (fields) (1), outputs := spansOf (fields) (2) } : Node))))

@[expose] public def reverseNodes (items : List (Node)) : List (Node) := (LexLeanCollections.listFold ((fun (reversed : List (Node)) (item : Node) => (item :: reversed))) (([] : List (Node))) (items) : List (Node))

@[expose] public def graphTasks (graph : ByteArray) (depth : Nat) : (Prod (Bool) (List (Task))) := (let all : (Prod (Bool) (List (Field))) := parsed (graph); (let fields : List (Field) := (all).2; (let bodies : List (ByteArray) := spansOf (fields) (1); (let read : (Prod (Nat) ((Prod (Bool) (List (Node))))) := (LexLeanCollections.listFold ((fun (state : (Prod (Nat) ((Prod (Bool) (List (Node)))))) (body : ByteArray) => (let node : (Prod (Bool) (Node)) := nodeOf ((state).1) (body); (((state).1 + 1), ((((state).2).1 && (node).1), ((node).2 :: ((state).2).2)))))) ((0, (true, ([] : List (Node))))) (bodies) : (Prod (Nat) ((Prod (Bool) (List (Node)))))); (let nodes : List (Node) := reverseNodes (((read).2).2); (let ordering : Kahn := (LexLeanCollections.iterate ((fun (state : Kahn) => kahnStep (nodes) (state))) ((read).1) (({ ok := true, emitted := ([] : List (Nat)), order := ([] : List (Node)) } : Kahn)) : Kahn); (let tensors : (Prod (Bool) (ByteArray)) := tensorSection (fields); (let inputs : (Prod (Bool) (ByteArray)) := valueInfos (fields) (11); (let outputs : (Prod (Bool) (ByteArray)) := valueInfos (fields) (12); (let infos : (Prod (Bool) (ByteArray)) := valueInfos (fields) (13); (let tail : Task := literal ((LexLeanRuntime.append ((tensors).2) ((LexLeanRuntime.append ((inputs).2) ((LexLeanRuntime.append ((outputs).2) ((infos).2) : ByteArray)) : ByteArray)) : ByteArray)); (let emitted : (Prod (Bool) (List (Task))) := (LexLeanCollections.listFold ((fun (state : (Prod (Bool) (List (Task)))) (node : Node) => (let tasks : (Prod (Bool) (List (Task))) := nodeTasks (node) (depth); (((state).1 && (tasks).1), (LexLeanCollections.listFold ((fun (out : List (Task)) (task : Task) => (task :: out))) ((state).2) (reverseTasks ((tasks).2)) : List (Task)))))) ((true, (literal ((LexLeanRuntime.append (LexLeanKappa.Gguf.digest (bytesOf (firstIn (fields) (2)))) (LexLeanKappa.Gguf.writeLe ((read).1) (4)) : ByteArray)) :: ([] : List (Task))))) (reverseNodes ((ordering).order)) : (Prod (Bool) (List (Task)))); ((((Nat.ble (depth) (64)) && (all).1) && ((((read).2).1 && (ordering).ok) && ((emitted).1 && ((tensors).1 && ((inputs).1 && ((outputs).1 && (infos).1)))))), reverseTasks ((tail :: (emitted).2)))))))))))))))

@[expose] public def machineStep (state : Machine) : Option (Machine) := (if (state).ok then (match (state).tasks with | List.nil => Option.none | List.cons task rest => (if (task).graph then (let expanded : (Prod (Bool) (List (Task))) := graphTasks ((task).body) ((task).depth); (if (expanded).1 then Option.some (({ ok := true, out := (state).out, tasks := (LexLeanCollections.listFold ((fun (out : List (Task)) (item : Task) => (item :: out))) (rest) (reverseTasks ((expanded).2)) : List (Task)) } : Machine)) else Option.some (({ ok := false, out := _root_.ByteArray.mk #[], tasks := ([] : List (Task)) } : Machine)))) else Option.some (({ ok := true, out := (LexLeanRuntime.append ((state).out) ((task).body) : ByteArray), tasks := rest } : Machine)))) else Option.none)

@[expose] public def graphBytes (graph : ByteArray) : (Prod (Bool) (ByteArray)) := (let state : Machine := ((LexLeanCollections.iterateUntil ((fun (state : Machine) => machineStep (state))) (((LexLeanRuntime.multiply ((LexLeanRuntime.length (graph) : Nat)) (8) : Nat) + 16)) (({ ok := true, out := _root_.ByteArray.mk #[], tasks := (({ graph := true, body := graph, depth := 0 } : Task) :: ([] : List (Task))) } : Machine)) : (Prod (Machine) (Bool)))).1; (let finished : Bool := (match (state).tasks with | List.nil => true | List.cons _ _ => false); (((state).ok && finished), (state).out)))

@[expose] public def opsetBytes (fields : List (Field)) : (Prod (Bool) (ByteArray)) := (let entries : List (ByteArray) := spansOf (fields) (8); (let tally : (Prod (Bool) (Bool)) := (LexLeanCollections.listFold ((fun (state : (Prod (Bool) (Bool))) (entry : ByteArray) => (let domain : (Prod (Bool) (Field)) := firstScanned (entry) (1); (let version : (Prod (Bool) (Field)) := firstScanned (entry) (2); (((state).1 && ((domain).1 && (version).1)), ((state).2 || ((Nat.beq ((LexLeanRuntime.length (bytesOf ((domain).2)) : Nat)) (0)) && ((Nat.blt (0) (varintOf ((version).2))) && (Nat.ble (varintOf ((version).2)) (9223372036854775807)))))))))) ((true, false)) (entries) : (Prod (Bool) (Bool))); (let keys : List (ByteArray) := (LexLeanCollections.listFold ((fun (out : List (ByteArray)) (entry : ByteArray) => ((LexLeanRuntime.append (escaped (bytesOf ((firstScanned (entry) (1)).2))) (writeBe (varintOf ((firstScanned (entry) (2)).2))) : ByteArray) :: out))) (([] : List (ByteArray))) ((LexLeanCollections.listFold ((fun (out : List (ByteArray)) (entry : ByteArray) => (entry :: out))) (([] : List (ByteArray))) (entries) : List (ByteArray))) : List (ByteArray)); (((tally).1 && ((tally).2 || (Nat.beq (lengthOf (entries)) (0)))), (LexLeanCollections.listFold ((fun (out : ByteArray) (entry : ByteArray) => (LexLeanRuntime.append (out) ((LexLeanRuntime.append (LexLeanKappa.Gguf.digest (bytesOf ((firstScanned (entry) (1)).2))) (LexLeanKappa.Gguf.writeLe (varintOf ((firstScanned (entry) (2)).2)) (8)) : ByteArray)) : ByteArray))) (_root_.ByteArray.mk #[]) (sortedBy (entries) (keys)) : ByteArray)))))

@[expose] public def canonicalOnnx (subject : ByteArray) : Option (ByteArray) := (let all : (Prod (Bool) (List (Field))) := parsed (subject); (let fields : List (Field) := (all).2; (let version : Field := firstIn (fields) (1); (let opsets : (Prod (Bool) (ByteArray)) := opsetBytes (fields); (let graph : ByteArray := bytesOf (firstIn (fields) (7)); (let emitted : (Prod (Bool) (ByteArray)) := graphBytes (graph); (let pairs : (Prod (Bool) (ByteArray)) := pairDigests (subject) (14); (if ((((all).1 && hasVarint (version)) && (((Nat.blt (0) ((version).value)) && (Nat.ble ((version).value) (13))) && (opsets).1)) && (((Nat.blt (0) ((LexLeanRuntime.length (graph) : Nat))) && (emitted).1) && (pairs).1)) then Option.some ((LexLeanRuntime.append (LexLeanKappa.Gguf.writeLe ((version).value) (8)) ((LexLeanRuntime.append ((opsets).2) ((LexLeanRuntime.append ((emitted).2) ((LexLeanRuntime.append (LexLeanKappa.Gguf.digest (bytesOf (firstIn (fields) (2)))) ((LexLeanRuntime.append (LexLeanKappa.Gguf.digest (bytesOf (firstIn (fields) (3)))) ((LexLeanRuntime.append (LexLeanKappa.Gguf.digest (bytesOf (firstIn (fields) (4)))) ((LexLeanRuntime.append (LexLeanKappa.Gguf.writeLe (varintOf (firstIn (fields) (5))) (8)) ((pairs).2) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) else Option.none))))))))

@[expose] public def onnxLabel (subject : ByteArray) : Option (ByteArray) := (match canonicalOnnx (subject) with | Option.some canonical => Option.some (LexLeanKappa.Label.kappaLabel (canonical)) | Option.none => Option.none)

@[expose] public def onnxLabelOn (axis : ByteArray) (subject : ByteArray) : Option (ByteArray) := (match canonicalOnnx (subject) with | Option.some canonical => LexLeanKappa.Axis.labelOn (axis) (canonical) | Option.none => Option.none)

end LexLeanKappa.Onnx

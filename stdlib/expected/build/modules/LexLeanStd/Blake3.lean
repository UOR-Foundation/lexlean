module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanStd.Blake3

public structure Digest where
  h0 : UInt32
  h1 : UInt32
  h2 : UInt32
  h3 : UInt32
  h4 : UInt32
  h5 : UInt32
  h6 : UInt32
  h7 : UInt32

public structure Words where
  w0 : UInt32
  w1 : UInt32
  w2 : UInt32
  w3 : UInt32
  w4 : UInt32
  w5 : UInt32
  w6 : UInt32
  w7 : UInt32
  w8 : UInt32
  w9 : UInt32
  w10 : UInt32
  w11 : UInt32
  w12 : UInt32
  w13 : UInt32
  w14 : UInt32
  w15 : UInt32

public structure Quad where
  a : UInt32
  b : UInt32
  c : UInt32
  d : UInt32

@[expose] public def widen (value : UInt32) : UInt64 := (match (LexLeanRuntime.checkedConvert (value) : Option (UInt64)) with | Option.some wide => wide | Option.none => (0 : UInt64))

@[expose] public def narrow (value : UInt64) : UInt32 := (match (LexLeanRuntime.checkedConvert ((LexLeanRuntime.bitAnd (value) ((4294967295 : UInt64)) : UInt64)) : Option (UInt32)) with | Option.some low => low | Option.none => (0 : UInt32))

@[expose] public def add (left : UInt32) (right : UInt32) : UInt32 := (match (LexLeanRuntime.checkedAdd (widen (left)) (widen (right)) : Option (UInt64)) with | Option.some sum => narrow (sum) | Option.none => (0 : UInt32))

@[expose] public def shiftRight (value : UInt32) (amount : UInt32) : UInt32 := (match (LexLeanRuntime.shiftRight (value) (amount) : Option (UInt32)) with | Option.some shifted => shifted | Option.none => (0 : UInt32))

@[expose] public def shiftLeft (value : UInt32) (amount : UInt32) : UInt32 := (match (LexLeanRuntime.shiftLeft (value) (amount) : Option (UInt32)) with | Option.some shifted => shifted | Option.none => (0 : UInt32))

@[expose] public def rotateRight (value : UInt32) (amount : UInt32) (complement : UInt32) : UInt32 := (LexLeanRuntime.bitOr (shiftRight (value) (amount)) (shiftLeft (value) (complement)) : UInt32)

@[expose] public def mix (a : UInt32) (b : UInt32) (c : UInt32) (d : UInt32) (x : UInt32) (y : UInt32) : Quad := (let a1 : UInt32 := add (add (a) (b)) (x); (let d1 : UInt32 := rotateRight ((LexLeanRuntime.bitXor (d) (a1) : UInt32)) ((16 : UInt32)) ((16 : UInt32)); (let c1 : UInt32 := add (c) (d1); (let b1 : UInt32 := rotateRight ((LexLeanRuntime.bitXor (b) (c1) : UInt32)) ((12 : UInt32)) ((20 : UInt32)); (let a2 : UInt32 := add (add (a1) (b1)) (y); (let d2 : UInt32 := rotateRight ((LexLeanRuntime.bitXor (d1) (a2) : UInt32)) ((8 : UInt32)) ((24 : UInt32)); (let c2 : UInt32 := add (c1) (d2); (let b2 : UInt32 := rotateRight ((LexLeanRuntime.bitXor (b1) (c2) : UInt32)) ((7 : UInt32)) ((25 : UInt32)); ({ a := a2, b := b2, c := c2, d := d2 } : Quad)))))))))

@[expose] public def round (state : Words) (block : Words) : Words := (let q0 : Quad := mix ((state).w0) ((state).w4) ((state).w8) ((state).w12) ((block).w0) ((block).w1); (let q1 : Quad := mix ((state).w1) ((state).w5) ((state).w9) ((state).w13) ((block).w2) ((block).w3); (let q2 : Quad := mix ((state).w2) ((state).w6) ((state).w10) ((state).w14) ((block).w4) ((block).w5); (let q3 : Quad := mix ((state).w3) ((state).w7) ((state).w11) ((state).w15) ((block).w6) ((block).w7); (let r0 : Quad := mix ((q0).a) ((q1).b) ((q2).c) ((q3).d) ((block).w8) ((block).w9); (let r1 : Quad := mix ((q1).a) ((q2).b) ((q3).c) ((q0).d) ((block).w10) ((block).w11); (let r2 : Quad := mix ((q2).a) ((q3).b) ((q0).c) ((q1).d) ((block).w12) ((block).w13); (let r3 : Quad := mix ((q3).a) ((q0).b) ((q1).c) ((q2).d) ((block).w14) ((block).w15); ({ w0 := (r0).a, w1 := (r1).a, w2 := (r2).a, w3 := (r3).a, w4 := (r3).b, w5 := (r0).b, w6 := (r1).b, w7 := (r2).b, w8 := (r2).c, w9 := (r3).c, w10 := (r0).c, w11 := (r1).c, w12 := (r1).d, w13 := (r2).d, w14 := (r3).d, w15 := (r0).d } : Words)))))))))

@[expose] public def permute (block : Words) : Words := ({ w0 := (block).w2, w1 := (block).w6, w2 := (block).w3, w3 := (block).w10, w4 := (block).w7, w5 := (block).w0, w6 := (block).w4, w7 := (block).w13, w8 := (block).w1, w9 := (block).w11, w10 := (block).w12, w11 := (block).w5, w12 := (block).w9, w13 := (block).w14, w14 := (block).w15, w15 := (block).w8 } : Words)

@[expose] public def wordOfNat (value : Nat) : UInt32 := (((LexLeanCollections.iterate ((fun (state : (Prod (Nat) ((Prod (UInt32) (UInt32))))) => (let remaining : Nat := (state).1; (let mask : UInt32 := ((state).2).1; (let bits : UInt32 := ((state).2).2; ((LexLeanRuntime.quotient (remaining) (2) (0) : Nat), (shiftLeft (mask) ((1 : UInt32)), (if (Nat.beq ((LexLeanRuntime.remainder (remaining) (2) (0) : Nat)) (1)) then (LexLeanRuntime.bitOr (bits) (mask) : UInt32) else bits)))))))) (32) ((value, ((1 : UInt32), (0 : UInt32)))) : (Prod (Nat) ((Prod (UInt32) (UInt32)))))).2).2

@[expose] public def initial : Digest := ({ h0 := (1779033703 : UInt32), h1 := (3144134277 : UInt32), h2 := (1013904242 : UInt32), h3 := (2773480762 : UInt32), h4 := (1359893119 : UInt32), h5 := (2600822924 : UInt32), h6 := (528734635 : UInt32), h7 := (1541459225 : UInt32) } : Digest)

@[expose] public def compress (chain : Digest) (block : Words) (counter : Nat) (length : UInt32) (flags : UInt32) : Digest := (let start : Words := ({ w0 := (chain).h0, w1 := (chain).h1, w2 := (chain).h2, w3 := (chain).h3, w4 := (chain).h4, w5 := (chain).h5, w6 := (chain).h6, w7 := (chain).h7, w8 := (1779033703 : UInt32), w9 := (3144134277 : UInt32), w10 := (1013904242 : UInt32), w11 := (2773480762 : UInt32), w12 := wordOfNat ((LexLeanRuntime.remainder (counter) (4294967296) (0) : Nat)), w13 := wordOfNat ((LexLeanRuntime.quotient (counter) (4294967296) (0) : Nat)), w14 := length, w15 := flags } : Words); (let final : Words := ((LexLeanCollections.iterate ((fun (state : (Prod (Words) (Words))) => (round ((state).1) ((state).2), permute ((state).2)))) (7) ((start, block)) : (Prod (Words) (Words)))).1; ({ h0 := (LexLeanRuntime.bitXor ((final).w0) ((final).w8) : UInt32), h1 := (LexLeanRuntime.bitXor ((final).w1) ((final).w9) : UInt32), h2 := (LexLeanRuntime.bitXor ((final).w2) ((final).w10) : UInt32), h3 := (LexLeanRuntime.bitXor ((final).w3) ((final).w11) : UInt32), h4 := (LexLeanRuntime.bitXor ((final).w4) ((final).w12) : UInt32), h5 := (LexLeanRuntime.bitXor ((final).w5) ((final).w13) : UInt32), h6 := (LexLeanRuntime.bitXor ((final).w6) ((final).w14) : UInt32), h7 := (LexLeanRuntime.bitXor ((final).w7) ((final).w15) : UInt32) } : Digest)))

@[expose] public def byteAt (message : ByteArray) (size : Nat) (position : Nat) : UInt32 := (if (Nat.blt (position) (size)) then (match (LexLeanRuntime.index (message) (position) : Option (UInt8)) with | Option.some byte => (match (LexLeanRuntime.checkedConvert (byte) : Option (UInt32)) with | Option.some wide => wide | Option.none => (0 : UInt32)) | Option.none => (0 : UInt32)) else (0 : UInt32))

@[expose] public def blockWord (message : ByteArray) (size : Nat) (position : Nat) : UInt32 := (LexLeanRuntime.bitOr (byteAt (message) (size) (position)) ((LexLeanRuntime.bitOr (shiftLeft (byteAt (message) (size) ((position + 1))) ((8 : UInt32))) ((LexLeanRuntime.bitOr (shiftLeft (byteAt (message) (size) ((position + 2))) ((16 : UInt32))) (shiftLeft (byteAt (message) (size) ((position + 3))) ((24 : UInt32))) : UInt32)) : UInt32)) : UInt32)

@[expose] public def load (message : ByteArray) (size : Nat) (offset : Nat) : Words := ({ w0 := blockWord (message) (size) (offset), w1 := blockWord (message) (size) ((offset + 4)), w2 := blockWord (message) (size) ((offset + 8)), w3 := blockWord (message) (size) ((offset + 12)), w4 := blockWord (message) (size) ((offset + 16)), w5 := blockWord (message) (size) ((offset + 20)), w6 := blockWord (message) (size) ((offset + 24)), w7 := blockWord (message) (size) ((offset + 28)), w8 := blockWord (message) (size) ((offset + 32)), w9 := blockWord (message) (size) ((offset + 36)), w10 := blockWord (message) (size) ((offset + 40)), w11 := blockWord (message) (size) ((offset + 44)), w12 := blockWord (message) (size) ((offset + 48)), w13 := blockWord (message) (size) ((offset + 52)), w14 := blockWord (message) (size) ((offset + 56)), w15 := blockWord (message) (size) ((offset + 60)) } : Words)

@[expose] public def chunkChain (message : ByteArray) (size : Nat) (chunk : Nat) (counter : Nat) (root : Bool) : Digest := (let start : Nat := (LexLeanRuntime.multiply (chunk) (1024) : Nat); (let remaining : Nat := (LexLeanRuntime.subtract (size) (start) : Nat); (let length : Nat := (if (Nat.blt (remaining) (1024)) then remaining else 1024); (let blocks : Nat := (if (Nat.beq (length) (0)) then 1 else (LexLeanRuntime.quotient ((length + 63)) (64) (0) : Nat)); ((LexLeanCollections.iterate ((fun (state : (Prod (Nat) (Digest))) => (let index : Nat := (state).1; (let last : Bool := (Nat.beq ((index + 1)) (blocks)); (let flags : UInt32 := (LexLeanRuntime.bitOr ((if (Nat.beq (index) (0)) then (1 : UInt32) else (0 : UInt32))) ((if last then (if root then (10 : UInt32) else (2 : UInt32)) else (0 : UInt32))) : UInt32); (let blockLength : UInt32 := (if last then wordOfNat ((LexLeanRuntime.subtract (length) ((LexLeanRuntime.multiply (index) (64) : Nat)) : Nat)) else (64 : UInt32)); ((index + 1), compress ((state).2) (load (message) (size) ((start + (LexLeanRuntime.multiply (index) (64) : Nat)))) (counter) (blockLength) (flags)))))))) (blocks) ((0, initial)) : (Prod (Nat) (Digest)))).2))))

@[expose] public def parent (left : Digest) (right : Digest) (root : Bool) : Digest := compress (initial) (({ w0 := (left).h0, w1 := (left).h1, w2 := (left).h2, w3 := (left).h3, w4 := (left).h4, w5 := (left).h5, w6 := (left).h6, w7 := (left).h7, w8 := (right).h0, w9 := (right).h1, w10 := (right).h2, w11 := (right).h3, w12 := (right).h4, w13 := (right).h5, w14 := (right).h6, w15 := (right).h7 } : Words)) (0) ((64 : UInt32)) ((if root then (12 : UInt32) else (4 : UInt32)))

@[expose] public def push (stack : List (Digest)) (chain : Digest) (total : Nat) : List (Digest) := (let merged : (Prod (Nat) ((Prod (Digest) (List (Digest))))) := (LexLeanCollections.iterate ((fun (state : (Prod (Nat) ((Prod (Digest) (List (Digest)))))) => (let count : Nat := (state).1; (let current : Digest := ((state).2).1; (let rest : List (Digest) := ((state).2).2; (if (Nat.beq ((LexLeanRuntime.remainder (count) (2) (0) : Nat)) (0)) then (match rest with | List.nil => state | List.cons top below => ((LexLeanRuntime.quotient (count) (2) (0) : Nat), (parent (top) (current) (false), below))) else state)))))) (64) ((total, (chain, stack))) : (Prod (Nat) ((Prod (Digest) (List (Digest)))))); (((merged).2).1 :: ((merged).2).2))

@[expose] public def collapse (stack : List (Digest)) (last : Digest) (root : Bool) : Digest := ((LexLeanCollections.listFold ((fun (state : (Prod (Nat) (Digest))) (top : Digest) => ((LexLeanRuntime.subtract ((state).1) (1) : Nat), parent (top) ((state).2) ((root && (Nat.beq ((state).1) (1))))))) (((LexLeanRuntime.length (stack) : Nat), last)) (stack) : (Prod (Nat) (Digest)))).2

@[expose] public def tree (message : ByteArray) (counter : Nat) (root : Bool) : Digest := (let size : Nat := (LexLeanRuntime.length (message) : Nat); (let chunks : Nat := (if (Nat.beq (size) (0)) then 1 else (LexLeanRuntime.quotient ((size + 1023)) (1024) (0) : Nat)); (let lastChunk : Nat := (LexLeanRuntime.subtract (chunks) (1) : Nat); (if (Nat.beq (chunks) (1)) then chunkChain (message) (size) (0) (counter) (root) else (let stack : List (Digest) := ((LexLeanCollections.iterate ((fun (state : (Prod (Nat) (List (Digest)))) => (let index : Nat := (state).1; ((index + 1), push ((state).2) (chunkChain (message) (size) (index) ((counter + index)) (false)) ((index + 1)))))) (lastChunk) ((0, ([] : List (Digest)))) : (Prod (Nat) (List (Digest))))).2; collapse (stack) (chunkChain (message) (size) (lastChunk) ((counter + lastChunk)) (false)) (root))))))

@[expose] public def blake3 (message : ByteArray) : Digest := tree (message) (0) (true)

@[expose] public def hexDigit (nibble : UInt32) : ByteArray := (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((8 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((4 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((2 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((1 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 48))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 49))]) else (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((1 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 50))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 51))])) else (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((2 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((1 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 52))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 53))]) else (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((1 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 54))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 55))]))) else (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((4 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((2 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((1 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 56))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 57))]) else (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((1 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 97))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 98))])) else (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((2 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((1 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 99))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 100))]) else (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (nibble) ((1 : UInt32)) : UInt32)) ((0 : UInt32)) : Bool) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 101))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 102))]))))

@[expose] public def hexWord (value : UInt32) : ByteArray := (LexLeanRuntime.append (hexDigit ((LexLeanRuntime.bitAnd (shiftRight (value) ((4 : UInt32))) ((15 : UInt32)) : UInt32))) ((LexLeanRuntime.append (hexDigit ((LexLeanRuntime.bitAnd (value) ((15 : UInt32)) : UInt32))) ((LexLeanRuntime.append (hexDigit ((LexLeanRuntime.bitAnd (shiftRight (value) ((12 : UInt32))) ((15 : UInt32)) : UInt32))) ((LexLeanRuntime.append (hexDigit ((LexLeanRuntime.bitAnd (shiftRight (value) ((8 : UInt32))) ((15 : UInt32)) : UInt32))) ((LexLeanRuntime.append (hexDigit ((LexLeanRuntime.bitAnd (shiftRight (value) ((20 : UInt32))) ((15 : UInt32)) : UInt32))) ((LexLeanRuntime.append (hexDigit ((LexLeanRuntime.bitAnd (shiftRight (value) ((16 : UInt32))) ((15 : UInt32)) : UInt32))) ((LexLeanRuntime.append (hexDigit ((LexLeanRuntime.bitAnd (shiftRight (value) ((28 : UInt32))) ((15 : UInt32)) : UInt32))) (hexDigit ((LexLeanRuntime.bitAnd (shiftRight (value) ((24 : UInt32))) ((15 : UInt32)) : UInt32))) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)

@[expose] public def hexDigest (digest : Digest) : ByteArray := (LexLeanRuntime.append (hexWord ((digest).h0)) ((LexLeanRuntime.append (hexWord ((digest).h1)) ((LexLeanRuntime.append (hexWord ((digest).h2)) ((LexLeanRuntime.append (hexWord ((digest).h3)) ((LexLeanRuntime.append (hexWord ((digest).h4)) ((LexLeanRuntime.append (hexWord ((digest).h5)) ((LexLeanRuntime.append (hexWord ((digest).h6)) (hexWord ((digest).h7)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)) : ByteArray)

@[expose] public def blake3Hex (message : ByteArray) : ByteArray := hexDigest (blake3 (message))

@[expose] public def pieceChain (piece : ByteArray) (index : Nat) (total : Nat) : Digest := tree (piece) ((LexLeanRuntime.multiply (index) (1024) : Nat)) ((Nat.beq (total) (1)))

@[expose] public def pieceRoot (chains : List (Digest)) : Digest := (let folded : (Prod (Nat) ((Prod (List (Digest)) (Option (Digest))))) := (LexLeanCollections.listFold ((fun (state : (Prod (Nat) ((Prod (List (Digest)) (Option (Digest)))))) (chain : Digest) => (let count : Nat := (state).1; (let stack : List (Digest) := ((state).2).1; (match ((state).2).2 with | Option.none => (count, (stack, Option.some (chain))) | Option.some pending => ((count + 1), (push (stack) (pending) ((count + 1)), Option.some (chain)))))))) ((0, (([] : List (Digest)), Option.none))) (chains) : (Prod (Nat) ((Prod (List (Digest)) (Option (Digest)))))); (match ((folded).2).2 with | Option.none => blake3 (_root_.ByteArray.mk #[]) | Option.some last => collapse (((folded).2).1) (last) (true)))

public theorem official_0 : (blake3 (_root_.ByteArray.mk #[]) = ({ h0 := (3108574127 : UInt32), h1 := (2795633141 : UInt32), h2 := (3930931360 : UInt32), h3 := (1237965878 : UInt32), h4 := (3374697371 : UInt32), h5 := (3071459757 : UInt32), h6 := (3398671052 : UInt32), h7 := (1647452132 : UInt32) } : Digest)) := by
  rfl

public theorem official_1 : (blake3 (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 0))]) = ({ h0 := (3755883053 : UInt32), h1 := (4049673201 : UInt32), h2 := (896436300 : UInt32), h3 := (1932959919 : UInt32), h4 := (2810695021 : UInt32), h5 := (3249874765 : UInt32), h6 := (3492086353 : UInt32), h7 := (333615861 : UInt32) } : Digest)) := by
  rfl

end LexLeanStd.Blake3

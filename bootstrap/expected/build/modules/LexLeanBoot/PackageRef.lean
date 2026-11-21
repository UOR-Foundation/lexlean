module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Numeral
public import LexLeanStd.Text
public import LexLeanBoot.LexiconLse
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.PackageRef

public structure PackageRef where
  package : ByteArray
  version : ByteArray

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def isAsciiAlpha (byte : UInt8) : Bool := (let value : Nat := (let __widen0 : UInt8 := byte; ((((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((128 : UInt8)) : UInt8)) ((128 : UInt8)) : Bool) then 128 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((64 : UInt8)) : UInt8)) ((64 : UInt8)) : Bool) then 64 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((32 : UInt8)) : UInt8)) ((32 : UInt8)) : Bool) then 32 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((16 : UInt8)) : UInt8)) ((16 : UInt8)) : Bool) then 16 else 0))) + (((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((8 : UInt8)) : UInt8)) ((8 : UInt8)) : Bool) then 8 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((4 : UInt8)) : UInt8)) ((4 : UInt8)) : Bool) then 4 else 0)) + ((if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((2 : UInt8)) : UInt8)) ((2 : UInt8)) : Bool) then 2 else 0) + (if (LexLeanRuntime.equal ((LexLeanRuntime.bitAnd (__widen0) ((1 : UInt8)) : UInt8)) ((1 : UInt8)) : Bool) then 1 else 0))))); (((Nat.ble (65) (value)) && (Nat.ble (value) (90))) || ((Nat.ble (97) (value)) && (Nat.ble (value) (122)))))

@[expose] public def isAlnumHyphen (byte : UInt8) : Bool := (if LexLeanStd.Numeral.isDecimal (byte) then true else (if isAsciiAlpha (byte) then true else (LexLeanRuntime.equal (byte) ((45 : UInt8)) : Bool)))

@[expose] public def alnumHyphenEnd (subject : ByteArray) : Nat := ((LexLeanCollections.iterateUntil ((fun (position : Nat) => (match LexLeanStd.Text.byteAt (subject) (position) with | Option.some b => (if isAlnumHyphen (b) then Option.some ((position + 1)) else Option.none) | Option.none => Option.none))) ((LexLeanRuntime.length (subject) : Nat)) (0) : (Prod (Nat) (Bool)))).1

@[expose] public def isAllDigits (subject : ByteArray) : Bool := (Nat.beq (LexLeanStd.Numeral.decimalEnd (subject)) ((LexLeanRuntime.length (subject) : Nat)))

@[expose] public def isPrereleaseSegment (segment : ByteArray) : Bool := (let n : Nat := (LexLeanRuntime.length (segment) : Nat); (if (Nat.beq (n) (0)) then false else (if (Nat.beq (alnumHyphenEnd (segment)) (n)) then (if isAllDigits (segment) then LexLeanStd.Numeral.canonicalDecimal (segment) else true) else false)))

@[expose] public def isBuildSegment (segment : ByteArray) : Bool := (let n : Nat := (LexLeanRuntime.length (segment) : Nat); (if (Nat.beq (n) (0)) then false else (Nat.beq (alnumHyphenEnd (segment)) (n))))

@[expose] public def segCount (segments : List (ByteArray)) : Nat := (LexLeanCollections.listFold ((fun (count : Nat) (_segment : ByteArray) => (count + 1))) (0) (segments) : Nat)

@[expose] public def allCanonicalDecimal (segments : List (ByteArray)) : Bool := (LexLeanCollections.listFold ((fun (ok : Bool) (segment : ByteArray) => (if ok then LexLeanStd.Numeral.canonicalDecimal (segment) else false))) (true) (segments) : Bool)

@[expose] public def allPrereleaseSegments (segments : List (ByteArray)) : Bool := (LexLeanCollections.listFold ((fun (ok : Bool) (segment : ByteArray) => (if ok then isPrereleaseSegment (segment) else false))) (true) (segments) : Bool)

@[expose] public def allBuildSegments (segments : List (ByteArray)) : Bool := (LexLeanCollections.listFold ((fun (ok : Bool) (segment : ByteArray) => (if ok then isBuildSegment (segment) else false))) (true) (segments) : Bool)

@[expose] public def isSemverCore (core : ByteArray) : Bool := (let parts : List (ByteArray) := LexLeanStd.Text.splitOn (core) ((46 : UInt8)); (if (Nat.beq (segCount (parts)) (3)) then allCanonicalDecimal (parts) else false))

@[expose] public def isExactVersion (version : ByteArray) : Bool := (match LexLeanStd.Text.find (version) (lit ("+")) with | Option.some plusAt => (let mainPart : ByteArray := LexLeanStd.Text.sliceOr (version) (0) (plusAt); (let buildPart : ByteArray := LexLeanStd.Text.sliceOr (version) ((plusAt + 1)) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (version) : Nat)) ((plusAt + 1)) : Nat)); (if allBuildSegments (LexLeanStd.Text.splitOn (buildPart) ((46 : UInt8))) then (match LexLeanStd.Text.find (mainPart) (lit ("-")) with | Option.some dashAt => (isSemverCore (LexLeanStd.Text.sliceOr (mainPart) (0) (dashAt)) && allPrereleaseSegments (LexLeanStd.Text.splitOn (LexLeanStd.Text.sliceOr (mainPart) ((dashAt + 1)) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (mainPart) : Nat)) ((dashAt + 1)) : Nat))) ((46 : UInt8)))) | Option.none => isSemverCore (mainPart)) else false))) | Option.none => (match LexLeanStd.Text.find (version) (lit ("-")) with | Option.some dashAt => (isSemverCore (LexLeanStd.Text.sliceOr (version) (0) (dashAt)) && allPrereleaseSegments (LexLeanStd.Text.splitOn (LexLeanStd.Text.sliceOr (version) ((dashAt + 1)) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (version) : Nat)) ((dashAt + 1)) : Nat))) ((46 : UInt8)))) | Option.none => isSemverCore (version)))

@[expose] public def parsePackageRef (text : ByteArray) : Option (PackageRef) := (match LexLeanStd.Text.find (text) (lit ("@")) with | Option.some «at» => (let package : ByteArray := LexLeanStd.Text.sliceOr (text) (0) («at»); (let version : ByteArray := LexLeanStd.Text.sliceOr (text) ((«at» + 1)) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (text) : Nat)) ((«at» + 1)) : Nat)); (if LexLeanBoot.LexiconLse.isPackageId (package) then (if isExactVersion (version) then Option.some (({ package := package, version := version } : PackageRef)) else Option.none) else Option.none))) | Option.none => Option.none)

end LexLeanBoot.PackageRef

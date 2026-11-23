module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Numeral
public import LexLeanStd.Order
public import LexLeanStd.Text
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.TomlRead

public inductive Value where
  | VStr (_ : ByteArray)
  | VBool (_ : Bool)
  | VNat (_ : Nat)
  | VArr (_ : List (ByteArray))

public structure Section where
  path : List (ByteArray)
  isArray : Bool
  rows : List (List ((Prod (ByteArray) (Value))))

public structure RawClose where
  path : List (ByteArray)
  isArray : Bool
  row : List ((Prod (ByteArray) (Value)))

public structure Ps where
  pos : Nat
  out : ByteArray
  mode : Nat
  closed : Bool

public structure Es where
  pos : Nat
  out : List (ByteArray)
  status : Nat

public structure Pst where
  rawClosures : List (RawClose)
  curPath : List (ByteArray)
  curArray : Bool
  curRow : List ((Prod (ByteArray) (Value)))
  pendingKey : Option (ByteArray)
  pendingVals : List (ByteArray)
  failed : Bool

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def skipWs (line : ByteArray) (position : Nat) : Nat := (let total : Nat := (LexLeanRuntime.length (line) : Nat); (let rest : Nat := (LexLeanRuntime.subtract (total) (position) : Nat); (position + LexLeanStd.Text.trimStart (LexLeanStd.Text.sliceOr (line) (position) (rest)) (rest))))

@[expose] public def psStep (line : ByteArray) (n : Nat) (st : Ps) : Option (Ps) := (if (st).closed then Option.none else (let position : Nat := (st).pos; (if (Nat.blt (position) (n)) then (let mode : Nat := (st).mode; (if (Nat.beq (mode) (1)) then (match LexLeanStd.Text.byteAt (line) (position) with | Option.some b => (if (if (LexLeanRuntime.equal (b) ((34 : UInt8)) : Bool) then true else (if (LexLeanRuntime.equal (b) ((92 : UInt8)) : Bool) then true else (if (LexLeanRuntime.equal (b) ((110 : UInt8)) : Bool) then true else (if (LexLeanRuntime.equal (b) ((116 : UInt8)) : Bool) then true else (LexLeanRuntime.equal (b) ((114 : UInt8)) : Bool))))) then Option.some (({ pos := (position + 1), out := cat ((st).out) ((if (LexLeanRuntime.equal (b) ((34 : UInt8)) : Bool) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 34))] else (if (LexLeanRuntime.equal (b) ((92 : UInt8)) : Bool) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 92))] else (if (LexLeanRuntime.equal (b) ((110 : UInt8)) : Bool) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 10))] else (if (LexLeanRuntime.equal (b) ((116 : UInt8)) : Bool) then _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 9))] else _root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 13))]))))), mode := 0, closed := false } : Ps)) else Option.none) | Option.none => Option.none) else (match LexLeanStd.Text.byteAt (line) (position) with | Option.some b => (if (LexLeanRuntime.equal (b) ((34 : UInt8)) : Bool) then Option.some (({ pos := (position + 1), out := (st).out, mode := 0, closed := true } : Ps)) else (if (LexLeanRuntime.equal (b) ((92 : UInt8)) : Bool) then Option.some (({ pos := (position + 1), out := (st).out, mode := 1, closed := false } : Ps)) else Option.some (({ pos := (position + 1), out := cat ((st).out) (LexLeanStd.Text.sliceOr (line) (position) (1)), mode := 0, closed := false } : Ps)))) | Option.none => Option.none))) else Option.none)))

@[expose] public def parseString (line : ByteArray) (start : Nat) : Option ((Prod (ByteArray) (Nat))) := (match LexLeanStd.Text.byteAt (line) (start) with | Option.some b => (if (LexLeanRuntime.equal (b) ((34 : UInt8)) : Bool) then (let n : Nat := (LexLeanRuntime.length (line) : Nat); (let final : Ps := ((LexLeanCollections.iterateUntil ((fun (st : Ps) => psStep (line) (n) (st))) ((n + 2)) (({ pos := (start + 1), out := _root_.ByteArray.mk #[], mode := 0, closed := false } : Ps)) : (Prod (Ps) (Bool)))).1; (if (final).closed then Option.some (((final).out, (final).pos)) else Option.none))) else Option.none) | Option.none => Option.none)

@[expose] public def esStep (line : ByteArray) (n : Nat) (st : Es) : Option (Es) := (if (LexLeanRuntime.equal ((st).status) (0) : Bool) then (let position : Nat := skipWs (line) ((st).pos); (if (Nat.blt (position) (n)) then (match LexLeanStd.Text.byteAt (line) (position) with | Option.some b => (if (LexLeanRuntime.equal (b) ((34 : UInt8)) : Bool) then (match parseString (line) (position) with | Option.some pair => (let afterPos : Nat := skipWs (line) ((pair).2); (if (Nat.blt (afterPos) (n)) then (match LexLeanStd.Text.byteAt (line) (afterPos) with | Option.some cb => (if (LexLeanRuntime.equal (cb) ((44 : UInt8)) : Bool) then Option.some (({ pos := skipWs (line) ((afterPos + 1)), out := ((pair).1 :: (st).out), status := 0 } : Es)) else Option.some (({ pos := afterPos, out := ((pair).1 :: (st).out), status := 2 } : Es))) | Option.none => Option.none) else Option.some (({ pos := afterPos, out := ((pair).1 :: (st).out), status := 1 } : Es)))) | Option.none => Option.some (({ pos := position, out := (st).out, status := 2 } : Es))) else Option.some (({ pos := position, out := (st).out, status := 2 } : Es))) | Option.none => Option.none) else Option.some (({ pos := position, out := (st).out, status := 1 } : Es)))) else Option.none)

@[expose] public def parseStringArray (line : ByteArray) : Option (List (ByteArray)) := (let n : Nat := (LexLeanRuntime.length (line) : Nat); (let final : Es := ((LexLeanCollections.iterateUntil ((fun (st : Es) => esStep (line) (n) (st))) ((n + 2)) (({ pos := 0, out := ([] : List (ByteArray)), status := 0 } : Es)) : (Prod (Es) (Bool)))).1; (if (LexLeanRuntime.equal ((final).status) (1) : Bool) then Option.some (LexLeanStd.Order.reverseList (ByteArray) ((final).out)) else Option.none)))

@[expose] public def isBlankOrComment (line : ByteArray) : Bool := (let t : ByteArray := LexLeanStd.Text.trim (line); (if (Nat.beq ((LexLeanRuntime.length (t) : Nat)) (0)) then true else LexLeanStd.Text.startsWith (t) (lit ("#"))))

@[expose] public def parseHeader (t : ByteArray) : Option ((Prod (List (ByteArray)) (Bool))) := (let isArr : Bool := LexLeanStd.Text.startsWith (t) (lit ("[[")); (let total : Nat := (LexLeanRuntime.length (t) : Nat); (let afterOpen : ByteArray := (if isArr then LexLeanStd.Text.sliceOr (t) (2) ((LexLeanRuntime.subtract (total) (2) : Nat)) else LexLeanStd.Text.sliceOr (t) (1) ((LexLeanRuntime.subtract (total) (1) : Nat))); (let closeOk : Bool := (if isArr then LexLeanStd.Text.endsWith (afterOpen) (lit ("]]")) else LexLeanStd.Text.endsWith (afterOpen) (lit ("]"))); (if closeOk then (let bodyLen : Nat := (LexLeanRuntime.length (afterOpen) : Nat); (let strip : Nat := (if isArr then 2 else 1); (let inner : ByteArray := LexLeanStd.Text.sliceOr (afterOpen) (0) ((LexLeanRuntime.subtract (bodyLen) (strip) : Nat)); (if (Nat.beq ((LexLeanRuntime.length (inner) : Nat)) (0)) then Option.none else Option.some ((LexLeanStd.Text.splitOn (inner) ((46 : UInt8)), isArr)))))) else Option.none)))))

@[expose] public def pathEq (a : List (ByteArray)) (b : List (ByteArray)) : Bool := (LexLeanRuntime.equal (LexLeanStd.Text.joinWith (a) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 46))])) (LexLeanStd.Text.joinWith (b) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 46))])) : Bool)

@[expose] public def rowHasKey (row : List ((Prod (ByteArray) (Value)))) (key : ByteArray) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (entry : (Prod (ByteArray) (Value))) => (if found then true else (LexLeanRuntime.equal ((entry).1) (key) : Bool)))) (false) (row) : Bool)

@[expose] public def rowGet (row : List ((Prod (ByteArray) (Value)))) (key : ByteArray) : Option (Value) := (LexLeanCollections.listFold ((fun (found : Option (Value)) (entry : (Prod (ByteArray) (Value))) => (match found with | Option.some _ => found | Option.none => (if (LexLeanRuntime.equal ((entry).1) (key) : Bool) then Option.some ((entry).2) else Option.none)))) (Option.none) (row) : Option (Value))

@[expose] public def rowKeys (row : List ((Prod (ByteArray) (Value)))) : List (ByteArray) := LexLeanStd.Order.reverseList (ByteArray) ((LexLeanCollections.listFold ((fun (acc : List (ByteArray)) (entry : (Prod (ByteArray) (Value))) => ((entry).1 :: acc))) (([] : List (ByteArray))) (row) : List (ByteArray)))

@[expose] public def sectionMatches (sec : Section) (path : List (ByteArray)) (isArr : Bool) : Bool := (pathEq ((sec).path) (path) && (LexLeanRuntime.equal ((sec).isArray) (isArr) : Bool))

@[expose] public def anyMatch (sections : List (Section)) (path : List (ByteArray)) (isArr : Bool) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (sec : Section) => (if found then true else sectionMatches (sec) (path) (isArr)))) (false) (sections) : Bool)

@[expose] public def appendRowToSection (sec : Section) (path : List (ByteArray)) (isArr : Bool) (row : List ((Prod (ByteArray) (Value)))) : Section := (if sectionMatches (sec) (path) (isArr) then ({ path := (sec).path, isArray := (sec).isArray, rows := LexLeanStd.Order.reverseList (List ((Prod (ByteArray) (Value)))) ((row :: LexLeanStd.Order.reverseList (List ((Prod (ByteArray) (Value)))) ((sec).rows))) } : Section) else sec)

@[expose] public def mergeInto (sections : List (Section)) (path : List (ByteArray)) (isArr : Bool) (row : List ((Prod (ByteArray) (Value)))) : List (Section) := LexLeanStd.Order.reverseList (Section) ((LexLeanCollections.listFold ((fun (acc : List (Section)) (sec : Section) => (appendRowToSection (sec) (path) (isArr) (row) :: acc))) (([] : List (Section))) (sections) : List (Section)))

@[expose] public def appendNew (sections : List (Section)) (path : List (ByteArray)) (isArr : Bool) (row : List ((Prod (ByteArray) (Value)))) : List (Section) := LexLeanStd.Order.reverseList (Section) ((({ path := path, isArray := isArr, rows := (row :: ([] : List (List ((Prod (ByteArray) (Value)))))) } : Section) :: LexLeanStd.Order.reverseList (Section) (sections)))

@[expose] public def groupStep (acc : (Prod (Bool) (List (Section)))) (raw : RawClose) : (Prod (Bool) (List (Section))) := (if (acc).1 then acc else (let path : List (ByteArray) := (raw).path; (let isArr : Bool := (raw).isArray; (let row : List ((Prod (ByteArray) (Value))) := (raw).row; (let found : Bool := anyMatch ((acc).2) (path) (isArr); (if found then (if isArr then (false, mergeInto ((acc).2) (path) (isArr) (row)) else (true, (acc).2)) else (false, appendNew ((acc).2) (path) (isArr) (row))))))))

@[expose] public def closeRow (rawClosures : List (RawClose)) (path : List (ByteArray)) (isArr : Bool) (row : List ((Prod (ByteArray) (Value)))) : List (RawClose) := (({ path := path, isArray := isArr, row := LexLeanStd.Order.reverseList ((Prod (ByteArray) (Value))) (row) } : RawClose) :: rawClosures)

@[expose] public def classifyAndStep (st : Pst) (key : ByteArray) (valText : ByteArray) : Pst := (if (LexLeanRuntime.equal (valText) (lit ("true")) : Bool) then ({ rawClosures := (st).rawClosures, curPath := (st).curPath, curArray := (st).curArray, curRow := ((key, Value.VBool (true)) :: (st).curRow), pendingKey := Option.none, pendingVals := ([] : List (ByteArray)), failed := false } : Pst) else (if (LexLeanRuntime.equal (valText) (lit ("false")) : Bool) then ({ rawClosures := (st).rawClosures, curPath := (st).curPath, curArray := (st).curArray, curRow := ((key, Value.VBool (false)) :: (st).curRow), pendingKey := Option.none, pendingVals := ([] : List (ByteArray)), failed := false } : Pst) else (if LexLeanStd.Text.startsWith (valText) (lit ("\"")) then (match parseString (valText) (0) with | Option.some pair => (if (Nat.beq ((pair).2) ((LexLeanRuntime.length (valText) : Nat))) then ({ rawClosures := (st).rawClosures, curPath := (st).curPath, curArray := (st).curArray, curRow := ((key, Value.VStr ((pair).1)) :: (st).curRow), pendingKey := Option.none, pendingVals := ([] : List (ByteArray)), failed := false } : Pst) else ({ rawClosures := (st).rawClosures, curPath := (st).curPath, curArray := (st).curArray, curRow := (st).curRow, pendingKey := Option.none, pendingVals := ([] : List (ByteArray)), failed := true } : Pst)) | Option.none => ({ rawClosures := (st).rawClosures, curPath := (st).curPath, curArray := (st).curArray, curRow := (st).curRow, pendingKey := Option.none, pendingVals := ([] : List (ByteArray)), failed := true } : Pst)) else (if LexLeanStd.Text.startsWith (valText) (lit ("[")) then (if (LexLeanRuntime.equal (valText) (lit ("[")) : Bool) then ({ rawClosures := (st).rawClosures, curPath := (st).curPath, curArray := (st).curArray, curRow := (st).curRow, pendingKey := Option.some (key), pendingVals := ([] : List (ByteArray)), failed := false } : Pst) else (if LexLeanStd.Text.endsWith (valText) (lit ("]")) then (match parseStringArray (LexLeanStd.Text.sliceOr (valText) (1) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (valText) : Nat)) (2) : Nat))) with | Option.some items => ({ rawClosures := (st).rawClosures, curPath := (st).curPath, curArray := (st).curArray, curRow := ((key, Value.VArr (items)) :: (st).curRow), pendingKey := Option.none, pendingVals := ([] : List (ByteArray)), failed := false } : Pst) | Option.none => ({ rawClosures := (st).rawClosures, curPath := (st).curPath, curArray := (st).curArray, curRow := (st).curRow, pendingKey := Option.none, pendingVals := ([] : List (ByteArray)), failed := true } : Pst)) else ({ rawClosures := (st).rawClosures, curPath := (st).curPath, curArray := (st).curArray, curRow := (st).curRow, pendingKey := Option.none, pendingVals := ([] : List (ByteArray)), failed := true } : Pst))) else (match LexLeanStd.Numeral.parseDecimal (valText) with | Option.some num => ({ rawClosures := (st).rawClosures, curPath := (st).curPath, curArray := (st).curArray, curRow := ((key, Value.VNat (num)) :: (st).curRow), pendingKey := Option.none, pendingVals := ([] : List (ByteArray)), failed := false } : Pst) | Option.none => ({ rawClosures := (st).rawClosures, curPath := (st).curPath, curArray := (st).curArray, curRow := (st).curRow, pendingKey := Option.none, pendingVals := ([] : List (ByteArray)), failed := true } : Pst))))))

@[expose] public def tomlLineStep (st : Pst) (line : ByteArray) : Pst := (if (st).failed then st else (match (st).pendingKey with | Option.some key => (let t : ByteArray := LexLeanStd.Text.trim (line); (if (LexLeanRuntime.equal (t) (lit ("]")) : Bool) then ({ rawClosures := (st).rawClosures, curPath := (st).curPath, curArray := (st).curArray, curRow := ((key, Value.VArr (LexLeanStd.Order.reverseList (ByteArray) ((st).pendingVals))) :: (st).curRow), pendingKey := Option.none, pendingVals := ([] : List (ByteArray)), failed := false } : Pst) else (let closing : Bool := LexLeanStd.Text.endsWith (t) (lit ("]")); (let body : ByteArray := (if closing then LexLeanStd.Text.sliceOr (t) (0) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (t) : Nat)) (1) : Nat)) else t); (match parseStringArray (body) with | Option.some more => (let grown : List (ByteArray) := (LexLeanCollections.listFold ((fun (acc : List (ByteArray)) (item : ByteArray) => (item :: acc))) ((st).pendingVals) (more) : List (ByteArray)); (if closing then ({ rawClosures := (st).rawClosures, curPath := (st).curPath, curArray := (st).curArray, curRow := ((key, Value.VArr (LexLeanStd.Order.reverseList (ByteArray) (grown))) :: (st).curRow), pendingKey := Option.none, pendingVals := ([] : List (ByteArray)), failed := false } : Pst) else ({ rawClosures := (st).rawClosures, curPath := (st).curPath, curArray := (st).curArray, curRow := (st).curRow, pendingKey := Option.some (key), pendingVals := grown, failed := false } : Pst))) | Option.none => ({ rawClosures := (st).rawClosures, curPath := (st).curPath, curArray := (st).curArray, curRow := (st).curRow, pendingKey := (st).pendingKey, pendingVals := (st).pendingVals, failed := true } : Pst)))))) | Option.none => (if isBlankOrComment (line) then st else (let t : ByteArray := LexLeanStd.Text.trim (line); (if LexLeanStd.Text.startsWith (t) (lit ("[")) then (let closed : List (RawClose) := closeRow ((st).rawClosures) ((st).curPath) ((st).curArray) ((st).curRow); (match parseHeader (t) with | Option.some pair => ({ rawClosures := closed, curPath := (pair).1, curArray := (pair).2, curRow := ([] : List ((Prod (ByteArray) (Value)))), pendingKey := Option.none, pendingVals := ([] : List (ByteArray)), failed := false } : Pst) | Option.none => ({ rawClosures := closed, curPath := ([] : List (ByteArray)), curArray := false, curRow := ([] : List ((Prod (ByteArray) (Value)))), pendingKey := Option.none, pendingVals := ([] : List (ByteArray)), failed := true } : Pst))) else (match LexLeanStd.Text.find (t) (lit ("=")) with | Option.none => ({ rawClosures := (st).rawClosures, curPath := (st).curPath, curArray := (st).curArray, curRow := (st).curRow, pendingKey := Option.none, pendingVals := ([] : List (ByteArray)), failed := true } : Pst) | Option.some cut => (let key : ByteArray := LexLeanStd.Text.trim (LexLeanStd.Text.sliceOr (t) (0) (cut)); (let valText : ByteArray := LexLeanStd.Text.trim (LexLeanStd.Text.sliceOr (t) ((cut + 1)) ((LexLeanRuntime.subtract ((LexLeanRuntime.length (t) : Nat)) ((cut + 1)) : Nat))); (if (if (Nat.beq ((LexLeanRuntime.length (key) : Nat)) (0)) then true else (match LexLeanStd.Text.find (key) (lit (".")) with | Option.some _ => true | Option.none => false)) then ({ rawClosures := (st).rawClosures, curPath := (st).curPath, curArray := (st).curArray, curRow := (st).curRow, pendingKey := Option.none, pendingVals := ([] : List (ByteArray)), failed := true } : Pst) else (if rowHasKey ((st).curRow) (key) then ({ rawClosures := (st).rawClosures, curPath := (st).curPath, curArray := (st).curArray, curRow := (st).curRow, pendingKey := Option.none, pendingVals := ([] : List (ByteArray)), failed := true } : Pst) else classifyAndStep (st) (key) (valText)))))))))))

@[expose] public def tomlParse (text : ByteArray) : Option (List (Section)) := (let final : Pst := (LexLeanCollections.listFold ((fun (st : Pst) (line : ByteArray) => tomlLineStep (st) (line))) (({ rawClosures := ([] : List (RawClose)), curPath := ([] : List (ByteArray)), curArray := false, curRow := ([] : List ((Prod (ByteArray) (Value)))), pendingKey := Option.none, pendingVals := ([] : List (ByteArray)), failed := false } : Pst)) (LexLeanStd.Text.lines (text)) : Pst); (if (final).failed then Option.none else (match (final).pendingKey with | Option.some _ => Option.none | Option.none => (let allRaw : List (RawClose) := LexLeanStd.Order.reverseList (RawClose) (closeRow ((final).rawClosures) ((final).curPath) ((final).curArray) ((final).curRow)); (let grouped : (Prod (Bool) (List (Section))) := (LexLeanCollections.listFold ((fun (acc : (Prod (Bool) (List (Section)))) (raw : RawClose) => groupStep (acc) (raw))) ((false, ([] : List (Section)))) (allRaw) : (Prod (Bool) (List (Section)))); (if (grouped).1 then Option.none else Option.some ((grouped).2)))))))

@[expose] public def docSection (doc : List (Section)) (path : List (ByteArray)) (isArr : Bool) : Option (Section) := (LexLeanCollections.listFold ((fun (found : Option (Section)) (sec : Section) => (match found with | Option.some _ => found | Option.none => (if sectionMatches (sec) (path) (isArr) then Option.some (sec) else Option.none)))) (Option.none) (doc) : Option (Section))

end LexLeanBoot.TomlRead

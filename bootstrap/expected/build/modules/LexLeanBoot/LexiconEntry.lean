module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanStd.Blake3
public import LexLeanStd.Numeral
public import LexLeanStd.Order
public import LexLeanStd.Text
public import LexLeanBoot.LexiconLre
public import LexLeanBoot.LexiconLse
public import LexLeanBoot.LexiconSchema
public import LexLeanBoot.LseScopes
public import LexLeanBoot.PrimitiveScan
public import LexLeanBoot.TomlRead
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.LexiconEntry

public structure Form where
  id : ByteArray
  channel : Nat
  surface : ByteArray
  atoms : List (LexLeanBoot.PrimitiveScan.Atom)
  canonicalSource : Bool
  features : List (ByteArray)

public inductive Denotation where
  | DCore (_ : ByteArray)
  | DLean (_ : ByteArray) (_ : ByteArray)
  | DDocument (_ : ByteArray) (_ : ByteArray)
  | DDefined (_ : LexLeanBoot.LexiconLse.Lse) (_ : ByteArray)

public structure EliminatorConstructor where
  package : ByteArray
  entry : ByteArray
  leanName : ByteArray
  fields : List (ByteArray)
  inductionHypotheses : List (ByteArray)

public structure Eliminator where
  casesLeanName : ByteArray
  inductionLeanName : ByteArray
  constructors : List (EliminatorConstructor)

public structure Entry where
  id : ByteArray
  category : Nat
  frame : Nat
  surfaceArity : Nat
  signature : Option (LexLeanBoot.LexiconLse.Lse)
  signatureCanonical : Option (ByteArray)
  signatureHash : Option (ByteArray)
  universes : List (ByteArray)
  precedence : Option (Nat)
  associativity : Option (Nat)
  denotation : Denotation
  forms : List (Form)
  renderMath : Option (LexLeanBoot.LexiconLre.Render)
  renderText : Option (LexLeanBoot.LexiconLre.Render)
  calculation : Bool
  eliminator : Option (Eliminator)

public structure RawDenotationFields where
  kind : ByteArray
  constructor : Option (ByteArray)
  module : Option (ByteArray)
  name : Option (ByteArray)
  component : Option (ByteArray)
  value : Option (ByteArray)

public structure RawForm where
  id : ByteArray
  channel : ByteArray
  surface : ByteArray
  canonicalSource : Bool
  features : List (ByteArray)

public structure RawConstructor where
  entry : ByteArray
  leanName : ByteArray
  fields : List (ByteArray)
  inductionHypotheses : List (ByteArray)

public structure RawEliminator where
  casesLeanName : ByteArray
  inductionLeanName : ByteArray
  constructors : List (RawConstructor)

public structure RawEntry where
  spec : ByteArray
  id : ByteArray
  category : ByteArray
  signature : Option (ByteArray)
  universes : Option (List (ByteArray))
  surfaceArity : Nat
  frame : ByteArray
  precedence : Option (Nat)
  associativity : Option (ByteArray)
  dKind : ByteArray
  dConstructor : Option (ByteArray)
  dModule : Option (ByteArray)
  dName : Option (ByteArray)
  dComponent : Option (ByteArray)
  dValue : Option (ByteArray)
  forms : List (RawForm)
  hasRender : Bool
  renderMath : Option (ByteArray)
  renderText : Option (ByteArray)
  hasCalculation : Bool
  calcKind : ByteArray
  eliminator : Option (RawEliminator)

@[expose] public def lit (literal : String) : ByteArray := (LexLeanRuntime.utf8Encode (literal) : ByteArray)

@[expose] public def cat (left : ByteArray) (right : ByteArray) : ByteArray := (LexLeanRuntime.append (left) (right) : ByteArray)

@[expose] public def cat3 (a : ByteArray) (b : ByteArray) (c : ByteArray) : ByteArray := cat (cat (a) (b)) (c)

@[expose] public def quoted (text : ByteArray) : ByteArray := cat3 (lit ("`")) (text) (lit ("`"))

@[expose] public def errD (message : ByteArray) : (Prod (ByteArray) (ByteArray)) := (lit ("LLR3004"), message)

@[expose] public def formUnsafeD (message : ByteArray) : (Prod (ByteArray) (ByteArray)) := (lit ("LLR3006"), message)

@[expose] public def limitD (message : ByteArray) : (Prod (ByteArray) (ByteArray)) := (lit ("LLS8002"), message)

@[expose] public def appendRev (revAcc : List ((Prod (ByteArray) (ByteArray)))) (item : Option ((Prod (ByteArray) (ByteArray)))) : List ((Prod (ByteArray) (ByteArray))) := (match item with | Option.some d => (d :: revAcc) | Option.none => revAcc)

@[expose] public def getStr (row : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value)))) (key : ByteArray) : Option (ByteArray) := (match LexLeanBoot.TomlRead.rowGet (row) (key) with | Option.some v => (match v with | LexLeanBoot.TomlRead.Value.VStr s => Option.some (s) | LexLeanBoot.TomlRead.Value.VBool _ => Option.none | LexLeanBoot.TomlRead.Value.VNat _ => Option.none | LexLeanBoot.TomlRead.Value.VArr _ => Option.none) | Option.none => Option.none)

@[expose] public def getOptStr (row : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value)))) (key : ByteArray) : Option (Option (ByteArray)) := (match LexLeanBoot.TomlRead.rowGet (row) (key) with | Option.none => Option.some (Option.none) | Option.some v => (match v with | LexLeanBoot.TomlRead.Value.VStr s => Option.some (Option.some (s)) | LexLeanBoot.TomlRead.Value.VBool _ => Option.none | LexLeanBoot.TomlRead.Value.VNat _ => Option.none | LexLeanBoot.TomlRead.Value.VArr _ => Option.none))

@[expose] public def getNat (row : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value)))) (key : ByteArray) : Option (Nat) := (match LexLeanBoot.TomlRead.rowGet (row) (key) with | Option.some v => (match v with | LexLeanBoot.TomlRead.Value.VStr _ => Option.none | LexLeanBoot.TomlRead.Value.VBool _ => Option.none | LexLeanBoot.TomlRead.Value.VNat n => Option.some (n) | LexLeanBoot.TomlRead.Value.VArr _ => Option.none) | Option.none => Option.none)

@[expose] public def getOptNat (row : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value)))) (key : ByteArray) : Option (Option (Nat)) := (match LexLeanBoot.TomlRead.rowGet (row) (key) with | Option.none => Option.some (Option.none) | Option.some v => (match v with | LexLeanBoot.TomlRead.Value.VStr _ => Option.none | LexLeanBoot.TomlRead.Value.VBool _ => Option.none | LexLeanBoot.TomlRead.Value.VNat n => Option.some (Option.some (n)) | LexLeanBoot.TomlRead.Value.VArr _ => Option.none))

@[expose] public def getBool (row : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value)))) (key : ByteArray) : Option (Bool) := (match LexLeanBoot.TomlRead.rowGet (row) (key) with | Option.some v => (match v with | LexLeanBoot.TomlRead.Value.VStr _ => Option.none | LexLeanBoot.TomlRead.Value.VBool b => Option.some (b) | LexLeanBoot.TomlRead.Value.VNat _ => Option.none | LexLeanBoot.TomlRead.Value.VArr _ => Option.none) | Option.none => Option.none)

@[expose] public def getArr (row : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value)))) (key : ByteArray) : Option (List (ByteArray)) := (match LexLeanBoot.TomlRead.rowGet (row) (key) with | Option.some v => (match v with | LexLeanBoot.TomlRead.Value.VStr _ => Option.none | LexLeanBoot.TomlRead.Value.VBool _ => Option.none | LexLeanBoot.TomlRead.Value.VNat _ => Option.none | LexLeanBoot.TomlRead.Value.VArr a => Option.some (a)) | Option.none => Option.none)

@[expose] public def getOptArr (row : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value)))) (key : ByteArray) : Option (Option (List (ByteArray))) := (match LexLeanBoot.TomlRead.rowGet (row) (key) with | Option.none => Option.some (Option.none) | Option.some v => (match v with | LexLeanBoot.TomlRead.Value.VStr _ => Option.none | LexLeanBoot.TomlRead.Value.VBool _ => Option.none | LexLeanBoot.TomlRead.Value.VNat _ => Option.none | LexLeanBoot.TomlRead.Value.VArr a => Option.some (Option.some (a))))

@[expose] public def firstRow (sec : LexLeanBoot.TomlRead.Section) : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value))) := (match (sec).rows with | List.cons row _ => row | List.nil => ([] : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value)))))

@[expose] public def topRow (doc : List (LexLeanBoot.TomlRead.Section)) : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value))) := (match LexLeanBoot.TomlRead.docSection (doc) (([] : List (ByteArray))) (false) with | Option.some sec => firstRow (sec) | Option.none => ([] : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value)))))

@[expose] public def emptyArrOrNone (row : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value)))) (key : ByteArray) : Option (List (List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value))))) := (match getArr (row) (key) with | Option.none => Option.none | Option.some arr => (match arr with | List.cons _ _ => Option.none | List.nil => Option.some (([] : List (List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value))))))))

@[expose] public def formRowsOrEmpty (doc : List (LexLeanBoot.TomlRead.Section)) (top : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value)))) : Option (List (List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value))))) := (match LexLeanBoot.TomlRead.docSection (doc) ((lit ("form") :: ([] : List (ByteArray)))) (true) with | Option.some fsec => Option.some ((fsec).rows) | Option.none => emptyArrOrNone (top) (lit ("form")))

@[expose] public def constructorRowsOrEmpty (doc : List (LexLeanBoot.TomlRead.Section)) (elimRow : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value)))) : Option (List (List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value))))) := (match LexLeanBoot.TomlRead.docSection (doc) ((lit ("eliminator") :: (lit ("constructor") :: ([] : List (ByteArray))))) (true) with | Option.some csec => Option.some ((csec).rows) | Option.none => emptyArrOrNone (elimRow) (lit ("constructor")))

@[expose] public def hasOnlyKeys (row : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value)))) (allowed : List (ByteArray)) : Bool := (LexLeanCollections.listFold ((fun (ok : Bool) (key : ByteArray) => (if ok then LexLeanBoot.LexiconSchema.isIn (allowed) (key) else false))) (true) (LexLeanBoot.TomlRead.rowKeys (row)) : Bool)

@[expose] public def rootAllowedKeys : List (ByteArray) := (lit ("spec") :: (lit ("id") :: (lit ("category") :: (lit ("signature") :: (lit ("universes") :: (lit ("surface_arity") :: (lit ("frame") :: (lit ("precedence") :: (lit ("associativity") :: (lit ("form") :: ([] : List (ByteArray))))))))))))

@[expose] public def denotationAllowedKeys : List (ByteArray) := (lit ("kind") :: (lit ("constructor") :: (lit ("module") :: (lit ("name") :: (lit ("component") :: (lit ("value") :: ([] : List (ByteArray))))))))

@[expose] public def formAllowedKeys : List (ByteArray) := (lit ("id") :: (lit ("channel") :: (lit ("surface") :: (lit ("canonical_source") :: (lit ("features") :: ([] : List (ByteArray)))))))

@[expose] public def renderAllowedKeys : List (ByteArray) := (lit ("math") :: (lit ("text") :: ([] : List (ByteArray))))

@[expose] public def calculationAllowedKeys : List (ByteArray) := (lit ("kind") :: ([] : List (ByteArray)))

@[expose] public def eliminatorAllowedKeys : List (ByteArray) := (lit ("cases_lean_name") :: (lit ("induction_lean_name") :: (lit ("constructor") :: ([] : List (ByteArray)))))

@[expose] public def eliminatorConstructorAllowedKeys : List (ByteArray) := (lit ("entry") :: (lit ("lean_name") :: (lit ("fields") :: (lit ("induction_hypotheses") :: ([] : List (ByteArray))))))

@[expose] public def extractDenotationRow (drow : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value)))) : Option (RawDenotationFields) := (if hasOnlyKeys (drow) (denotationAllowedKeys) then (match getStr (drow) (lit ("kind")) with | Option.none => Option.none | Option.some dKind => (match getOptStr (drow) (lit ("constructor")) with | Option.none => Option.none | Option.some dConstructor => (match getOptStr (drow) (lit ("module")) with | Option.none => Option.none | Option.some dModule => (match getOptStr (drow) (lit ("name")) with | Option.none => Option.none | Option.some dName => (match getOptStr (drow) (lit ("component")) with | Option.none => Option.none | Option.some dComponent => (match getOptStr (drow) (lit ("value")) with | Option.none => Option.none | Option.some dValue => Option.some (({ kind := dKind, constructor := dConstructor, module := dModule, name := dName, component := dComponent, value := dValue } : RawDenotationFields)))))))) else Option.none)

@[expose] public def extractForm (row : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value)))) : Option (RawForm) := (if hasOnlyKeys (row) (formAllowedKeys) then (match getStr (row) (lit ("id")) with | Option.none => Option.none | Option.some formId => (match getStr (row) (lit ("channel")) with | Option.none => Option.none | Option.some channel => (match getStr (row) (lit ("surface")) with | Option.none => Option.none | Option.some surface => (match getBool (row) (lit ("canonical_source")) with | Option.none => Option.none | Option.some cs => (match getArr (row) (lit ("features")) with | Option.none => Option.none | Option.some features => Option.some (({ id := formId, channel := channel, surface := surface, canonicalSource := cs, features := features } : RawForm))))))) else Option.none)

@[expose] public def extractForms (rows : List (List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value))))) : Option (List (RawForm)) := (match (LexLeanCollections.listFold ((fun (acc : Option (List (RawForm))) (row : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value)))) => (match acc with | Option.none => Option.none | Option.some soFar => (match extractForm (row) with | Option.none => Option.none | Option.some rf => Option.some ((rf :: soFar)))))) (Option.some (([] : List (RawForm)))) (rows) : Option (List (RawForm))) with | Option.some revList => Option.some (LexLeanStd.Order.reverseList (RawForm) (revList)) | Option.none => Option.none)

@[expose] public def extractConstructor (row : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value)))) : Option (RawConstructor) := (if hasOnlyKeys (row) (eliminatorConstructorAllowedKeys) then (match getStr (row) (lit ("entry")) with | Option.none => Option.none | Option.some entryId => (match getStr (row) (lit ("lean_name")) with | Option.none => Option.none | Option.some leanName => (match getArr (row) (lit ("fields")) with | Option.none => Option.none | Option.some fields => (match getArr (row) (lit ("induction_hypotheses")) with | Option.none => Option.none | Option.some ihs => Option.some (({ entry := entryId, leanName := leanName, fields := fields, inductionHypotheses := ihs } : RawConstructor)))))) else Option.none)

@[expose] public def extractConstructors (rows : List (List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value))))) : Option (List (RawConstructor)) := (match (LexLeanCollections.listFold ((fun (acc : Option (List (RawConstructor))) (row : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value)))) => (match acc with | Option.none => Option.none | Option.some soFar => (match extractConstructor (row) with | Option.none => Option.none | Option.some rc => Option.some ((rc :: soFar)))))) (Option.some (([] : List (RawConstructor)))) (rows) : Option (List (RawConstructor))) with | Option.some revList => Option.some (LexLeanStd.Order.reverseList (RawConstructor) (revList)) | Option.none => Option.none)

@[expose] public def extractEliminator (doc : List (LexLeanBoot.TomlRead.Section)) : Option (Option (RawEliminator)) := (match LexLeanBoot.TomlRead.docSection (doc) ((lit ("eliminator") :: ([] : List (ByteArray)))) (false) with | Option.none => Option.some (Option.none) | Option.some sec => (let row : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value))) := firstRow (sec); (if hasOnlyKeys (row) (eliminatorAllowedKeys) then (match getStr (row) (lit ("cases_lean_name")) with | Option.none => Option.none | Option.some cln => (match getStr (row) (lit ("induction_lean_name")) with | Option.none => Option.none | Option.some iln => (match constructorRowsOrEmpty (doc) (row) with | Option.none => Option.none | Option.some constructorRows => (match extractConstructors (constructorRows) with | Option.none => Option.none | Option.some constructors => Option.some (Option.some (({ casesLeanName := cln, inductionLeanName := iln, constructors := constructors } : RawEliminator))))))) else Option.none)))

@[expose] public def extractRaw (doc : List (LexLeanBoot.TomlRead.Section)) : Option (RawEntry) := (let top : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value))) := topRow (doc); (if hasOnlyKeys (top) (rootAllowedKeys) then (match getStr (top) (lit ("spec")) with | Option.none => Option.none | Option.some spec => (match getStr (top) (lit ("id")) with | Option.none => Option.none | Option.some entryId => (match getStr (top) (lit ("category")) with | Option.none => Option.none | Option.some category => (match getOptStr (top) (lit ("signature")) with | Option.none => Option.none | Option.some signature => (match getOptArr (top) (lit ("universes")) with | Option.none => Option.none | Option.some universes => (match getNat (top) (lit ("surface_arity")) with | Option.none => Option.none | Option.some surfaceArity => (match getStr (top) (lit ("frame")) with | Option.none => Option.none | Option.some frame => (match getOptNat (top) (lit ("precedence")) with | Option.none => Option.none | Option.some precedence => (match getOptStr (top) (lit ("associativity")) with | Option.none => Option.none | Option.some associativity => (match LexLeanBoot.TomlRead.docSection (doc) ((lit ("denotation") :: ([] : List (ByteArray)))) (false) with | Option.none => Option.none | Option.some dsec => (let drow : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value))) := firstRow (dsec); (match extractDenotationRow (drow) with | Option.none => Option.none | Option.some dfields => (let dKind : ByteArray := (dfields).kind; (let dConstructor : Option (ByteArray) := (dfields).constructor; (let dModule : Option (ByteArray) := (dfields).module; (let dName : Option (ByteArray) := (dfields).name; (let dComponent : Option (ByteArray) := (dfields).component; (let dValue : Option (ByteArray) := (dfields).value; (match formRowsOrEmpty (doc) (top) with | Option.none => Option.none | Option.some formRows => (match extractForms (formRows) with | Option.none => Option.none | Option.some forms => (let hasRender : Bool := (match LexLeanBoot.TomlRead.docSection (doc) ((lit ("render") :: ([] : List (ByteArray)))) (false) with | Option.some _ => true | Option.none => false); (let rrow : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value))) := (match LexLeanBoot.TomlRead.docSection (doc) ((lit ("render") :: ([] : List (ByteArray)))) (false) with | Option.some s => firstRow (s) | Option.none => ([] : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value))))); (match getOptStr (rrow) (lit ("math")) with | Option.none => Option.none | Option.some renderMath => (match getOptStr (rrow) (lit ("text")) with | Option.none => Option.none | Option.some renderText => (let hasCalculation : Bool := (match LexLeanBoot.TomlRead.docSection (doc) ((lit ("calculation") :: ([] : List (ByteArray)))) (false) with | Option.some _ => true | Option.none => false); (let crow : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value))) := (match LexLeanBoot.TomlRead.docSection (doc) ((lit ("calculation") :: ([] : List (ByteArray)))) (false) with | Option.some s => firstRow (s) | Option.none => ([] : List ((Prod (ByteArray) (LexLeanBoot.TomlRead.Value))))); (match (if hasCalculation then getStr (crow) (lit ("kind")) else Option.some (_root_.ByteArray.mk #[])) with | Option.none => Option.none | Option.some calcKind => (match extractEliminator (doc) with | Option.none => Option.none | Option.some eliminator => Option.some (({ spec := spec, id := entryId, category := category, signature := signature, universes := universes, surfaceArity := surfaceArity, frame := frame, precedence := precedence, associativity := associativity, dKind := dKind, dConstructor := dConstructor, dModule := dModule, dName := dName, dComponent := dComponent, dValue := dValue, forms := forms, hasRender := hasRender, renderMath := renderMath, renderText := renderText, hasCalculation := hasCalculation, calcKind := calcKind, eliminator := eliminator } : RawEntry)))))))))))))))))))))))))))))) else Option.none))

public structure HeaderOut where
  diagsRev : List ((Prod (ByteArray) (ByteArray)))
  precedence : Option (Nat)
  associativity : Option (Nat)

@[expose] public def validateHeader (entryId : ByteArray) (categoryIdx : Nat) (frameIdx : Nat) (categoryText : ByteArray) (frameText : ByteArray) (surfaceArity : Nat) (rawPrecedence : Option (Nat)) (rawAssociativity : Option (ByteArray)) (isCore : Bool) : HeaderOut := (let shape : LexLeanBoot.LexiconSchema.Shape := LexLeanBoot.LexiconSchema.shapeFacts (categoryIdx) (frameIdx) (surfaceArity); (let d1 : List ((Prod (ByteArray) (ByteArray))) := appendRev (([] : List ((Prod (ByteArray) (ByteArray))))) ((if LexLeanBoot.LexiconLse.isEntryId (entryId) then Option.none else Option.some (errD (cat (quoted (entryId)) (lit (" is not a valid entry ID")))))); (let d2 : List ((Prod (ByteArray) (ByteArray))) := appendRev (d1) ((if ((shape).coreOnly && (if isCore then false else true)) then Option.some (errD (cat3 (lit ("a non-core entry cannot use category `")) (categoryText) (lit ("`")))) else Option.none)); (let d3 : List ((Prod (ByteArray) (ByteArray))) := appendRev (d2) ((if (shape).permitted then Option.none else Option.some (errD (cat (cat (cat (cat (lit ("category `")) (categoryText)) (lit ("` does not permit frame `"))) (frameText)) (lit ("`")))))); (let d4 : List ((Prod (ByteArray) (ByteArray))) := appendRev (d3) ((match (shape).fixedArity with | Option.some fixed => (if (Nat.beq (surfaceArity) (fixed)) then Option.none else Option.some (errD (cat (cat (cat (cat (lit ("frame `")) (frameText)) (lit ("` fixes surface_arity "))) (LexLeanStd.Numeral.decimal (fixed))) (cat (lit (", found ")) (LexLeanStd.Numeral.decimal (surfaceArity)))))) | Option.none => (if (Nat.beq (surfaceArity) (0)) then Option.some (errD (lit ("a call frame needs surface_arity >= 1"))) else Option.none))); (let needsPrecedence : Bool := (if (Nat.beq (frameIdx) (2)) then true else (if (Nat.beq (frameIdx) (3)) then true else (Nat.beq (frameIdx) (4)))); (let precPair : (Prod (List ((Prod (ByteArray) (ByteArray)))) (Option (Nat))) := (match rawPrecedence with | Option.some value => (if needsPrecedence then (if (Nat.ble (value) (255)) then (d4, Option.some (value)) else (appendRev (d4) (Option.some (errD (lit ("precedence is 0..255")))), Option.none)) else (appendRev (d4) (Option.some (errD (lit ("precedence does not apply to this frame")))), Option.none)) | Option.none => (if needsPrecedence then (appendRev (d4) (Option.some (errD (lit ("this frame requires a precedence")))), Option.none) else (d4, Option.none))); (let d5 : List ((Prod (ByteArray) (ByteArray))) := (precPair).1; (let precedence : Option (Nat) := (precPair).2; (let isInfix : Bool := (Nat.beq (frameIdx) (4)); (let assocPair : (Prod (List ((Prod (ByteArray) (ByteArray)))) (Option (Nat))) := (match rawAssociativity with | Option.some text => (if isInfix then (if (LexLeanRuntime.equal (text) (lit ("left")) : Bool) then (d5, Option.some (0)) else (if (LexLeanRuntime.equal (text) (lit ("right")) : Bool) then (d5, Option.some (1)) else (if (LexLeanRuntime.equal (text) (lit ("none")) : Bool) then (d5, Option.some (2)) else (appendRev (d5) (Option.some (errD (lit ("associativity is left, right, or none")))), Option.none)))) else (appendRev (d5) (Option.some (errD (lit ("associativity applies only to infix frames")))), Option.none)) | Option.none => (if isInfix then (appendRev (d5) (Option.some (errD (lit ("an infix frame requires an associativity")))), Option.none) else (d5, Option.none))); ({ diagsRev := (assocPair).1, precedence := precedence, associativity := (assocPair).2 } : HeaderOut))))))))))))

@[expose] public def sortedUniqueStep (state : (Prod (Option (ByteArray)) (Bool))) (item : ByteArray) : (Prod (Option (ByteArray)) (Bool)) := (match (state).1 with | Option.none => (Option.some (item), (state).2) | Option.some prev => (Option.some (item), (if (state).2 then LexLeanStd.Order.lessBytes (prev) (item) else false)))

@[expose] public def isSortedUniqueStrict (items : List (ByteArray)) : Bool := ((LexLeanCollections.listFold ((fun (state : (Prod (Option (ByteArray)) (Bool))) (item : ByteArray) => sortedUniqueStep (state) (item))) ((Option.none, true)) (items) : (Prod (Option (ByteArray)) (Bool)))).2

@[expose] public def anyNotLseIdentifier (names : List (ByteArray)) : Option (ByteArray) := (LexLeanCollections.listFold ((fun (found : Option (ByteArray)) (name : ByteArray) => (match found with | Option.some _ => found | Option.none => (if LexLeanBoot.LexiconLse.isLseIdentifier (name) then Option.none else Option.some (name))))) (Option.none) (names) : Option (ByteArray))

@[expose] public def wrapLseFailure (what : ByteArray) (failure : LexLeanBoot.LexiconLse.Failure) : (Prod (ByteArray) (ByteArray)) := (if (Nat.beq ((failure).kind) (1)) then limitD (cat (cat (what) (lit (": "))) ((failure).message)) else errD (cat (cat (cat (lit ("invalid ")) (what)) (lit (" LSE: "))) ((failure).message)))

public structure SigOut where
  diagsRev : List ((Prod (ByteArray) (ByteArray)))
  universes : List (ByteArray)
  signature : Option (LexLeanBoot.LexiconLse.Lse)
  signatureCanonical : Option (ByteArray)
  signatureHash : Option (ByteArray)

@[expose] public def validateSignature (entryId : ByteArray) (categoryText : ByteArray) (rawUniverses : Option (List (ByteArray))) (rawSignature : Option (ByteArray)) (hasSignature : Bool) (surfaceArity : Nat) (maxScopeDepth : Nat) (diagsIn : List ((Prod (ByteArray) (ByteArray)))) : SigOut := (let universes : List (ByteArray) := (match rawUniverses with | Option.some us => us | Option.none => ([] : List (ByteArray))); (let d1 : List ((Prod (ByteArray) (ByteArray))) := appendRev (diagsIn) ((if isSortedUniqueStrict (universes) then Option.none else Option.some (errD (lit ("universes must be sorted and unique"))))); (let d2 : List ((Prod (ByteArray) (ByteArray))) := (match anyNotLseIdentifier (universes) with | Option.none => d1 | Option.some bad => appendRev (d1) (Option.some (errD (cat (quoted (bad)) (lit (" is not a universe name")))))); (match rawSignature with | Option.some text => (if hasSignature then (match LexLeanBoot.LexiconLse.parseLse (text) (maxScopeDepth) with | Except.ok parsed => (let bound : Nat := (LexLeanRuntime.length (text) : Nat); (let d3 : List ((Prod (ByteArray) (ByteArray))) := (match LexLeanBoot.LseScopes.checkScopes (parsed) (universes) (bound) with | Option.none => d2 | Option.some scopeErr => appendRev (d2) (Option.some (errD (cat (lit ("invalid signature: ")) (scopeErr))))); (let shape2 : (Prod (Nat) (Bool)) := LexLeanBoot.LseScopes.outerShape (parsed) (bound); (let explicit : Nat := (shape2).1; (let d4 : List ((Prod (ByteArray) (ByteArray))) := (if (Nat.beq (surfaceArity) (explicit)) then d3 else appendRev (d3) (Option.some (errD (cat3 (cat3 (quoted (entryId)) (lit (": surface_arity ")) (LexLeanStd.Numeral.decimal (surfaceArity))) (lit (" does not match the ")) (cat (LexLeanStd.Numeral.decimal (explicit)) ((if (Nat.beq (explicit) (1)) then lit (" explicit binder of the signature's outer pi") else lit (" explicit binders of the signature's outer pi")))))))); (let canonical : ByteArray := LexLeanBoot.LexiconLse.printLse (parsed) (true); (let hash : ByteArray := LexLeanStd.Blake3.blake3Label (canonical); ({ diagsRev := d4, universes := universes, signature := Option.some (parsed), signatureCanonical := Option.some (canonical), signatureHash := Option.some (hash) } : SigOut)))))))) | Except.error failure => ({ diagsRev := appendRev (d2) (Option.some (wrapLseFailure (lit ("signature")) (failure))), universes := universes, signature := Option.none, signatureCanonical := Option.none, signatureHash := Option.none } : SigOut)) else ({ diagsRev := appendRev (d2) (Option.some (errD (cat3 (lit ("category `")) (categoryText) (lit ("` does not carry a signature"))))), universes := universes, signature := Option.none, signatureCanonical := Option.none, signatureHash := Option.none } : SigOut)) | Option.none => (if hasSignature then ({ diagsRev := appendRev (d2) (Option.some (errD (lit ("every semantic entry has a complete LSE signature")))), universes := universes, signature := Option.none, signatureCanonical := Option.none, signatureHash := Option.none } : SigOut) else ({ diagsRev := appendRev (d2) ((match rawUniverses with | Option.some _ => Option.some (errD (lit ("universes apply only with a signature"))) | Option.none => Option.none)), universes := universes, signature := Option.none, signatureCanonical := Option.none, signatureHash := Option.none } : SigOut))))))

@[expose] public def isSomeBytes (o : Option (ByteArray)) : Nat := (match o with | Option.some _ => 1 | Option.none => 0)

@[expose] public def optCount (a : Option (ByteArray)) (b : Option (ByteArray)) (c : Option (ByteArray)) (d : Option (ByteArray)) (e : Option (ByteArray)) : Nat := ((((isSomeBytes (a) + isSomeBytes (b)) + isSomeBytes (c)) + isSomeBytes (d)) + isSomeBytes (e))

@[expose] public def validateCoreDenotation (isCore : Bool) (dConstructor : Option (ByteArray)) (fc : Nat) (diagsIn : List ((Prod (ByteArray) (ByteArray)))) : (Prod (List ((Prod (ByteArray) (ByteArray)))) (Denotation)) := (let d1 : List ((Prod (ByteArray) (ByteArray))) := appendRev (diagsIn) ((if isCore then Option.none else Option.some (errD (lit ("only lexlean.core may use a core denotation"))))); (let shapeOk : Bool := (match dConstructor with | Option.some c => ((Nat.beq (fc) (1)) && LexLeanBoot.LexiconLse.isEntryId (c)) | Option.none => false); (if shapeOk then (d1, Denotation.DCore ((match dConstructor with | Option.some c => c | Option.none => _root_.ByteArray.mk #[]))) else (appendRev (d1) (Option.some (errD (lit ("a core denotation has exactly `constructor`")))), Denotation.DCore (_root_.ByteArray.mk #[])))))

@[expose] public def validateLeanDenotation (dModule : Option (ByteArray)) (dName : Option (ByteArray)) (fc : Nat) (diagsIn : List ((Prod (ByteArray) (ByteArray)))) : (Prod (List ((Prod (ByteArray) (ByteArray)))) (Denotation)) := (let shapeOk : Bool := (match dModule with | Option.some m => (match dName with | Option.some n => ((Nat.beq (fc) (2)) && (LexLeanBoot.LexiconSchema.isLeanName (m) && LexLeanBoot.LexiconSchema.isLeanName (n))) | Option.none => false) | Option.none => false); (if shapeOk then (diagsIn, Denotation.DLean ((match dModule with | Option.some m => m | Option.none => _root_.ByteArray.mk #[])) ((match dName with | Option.some n => n | Option.none => _root_.ByteArray.mk #[]))) else (appendRev (diagsIn) (Option.some (errD (lit ("a lean denotation has exactly `module` and `name` in the conservative ASCII Lean-name grammar")))), Denotation.DLean (_root_.ByteArray.mk #[]) (_root_.ByteArray.mk #[]))))

@[expose] public def validateDocumentDenotation (dModule : Option (ByteArray)) (dComponent : Option (ByteArray)) (fc : Nat) (diagsIn : List ((Prod (ByteArray) (ByteArray)))) : (Prod (List ((Prod (ByteArray) (ByteArray)))) (Denotation)) := (let shapeOk : Bool := (match dModule with | Option.some _ => (match dComponent with | Option.some _ => (Nat.beq (fc) (2)) | Option.none => false) | Option.none => false); (if shapeOk then (diagsIn, Denotation.DDocument ((match dModule with | Option.some m => m | Option.none => _root_.ByteArray.mk #[])) ((match dComponent with | Option.some c => c | Option.none => _root_.ByteArray.mk #[]))) else (appendRev (diagsIn) (Option.some (errD (lit ("a document denotation has exactly `module` and `component`")))), Denotation.DDocument (_root_.ByteArray.mk #[]) (_root_.ByteArray.mk #[]))))

@[expose] public def validateDefinedDenotation (dValue : Option (ByteArray)) (fc : Nat) (maxScopeDepth : Nat) (universes : List (ByteArray)) (diagsIn : List ((Prod (ByteArray) (ByteArray)))) : (Prod (List ((Prod (ByteArray) (ByteArray)))) (Denotation)) := (match dValue with | Option.none => (appendRev (diagsIn) (Option.some (errD (lit ("a defined denotation has exactly `value`")))), Denotation.DDefined (LexLeanBoot.LexiconLse.Lse.LSortProp) (_root_.ByteArray.mk #[])) | Option.some valueText => (if (Nat.beq (fc) (1)) then (match LexLeanBoot.LexiconLse.parseLse (valueText) (maxScopeDepth) with | Except.ok value => (let bound : Nat := (LexLeanRuntime.length (valueText) : Nat); (let d2 : List ((Prod (ByteArray) (ByteArray))) := (match LexLeanBoot.LseScopes.checkScopes (value) (universes) (bound) with | Option.none => diagsIn | Option.some scopeErr => appendRev (diagsIn) (Option.some (errD (cat (lit ("invalid defined value: ")) (scopeErr))))); (d2, Denotation.DDefined (value) (LexLeanBoot.LexiconLse.printLse (value) (true))))) | Except.error failure => (appendRev (diagsIn) (Option.some (wrapLseFailure (lit ("defined value")) (failure))), Denotation.DDefined (LexLeanBoot.LexiconLse.Lse.LSortProp) (_root_.ByteArray.mk #[]))) else (appendRev (diagsIn) (Option.some (errD (lit ("a defined denotation has exactly `value`")))), Denotation.DDefined (LexLeanBoot.LexiconLse.Lse.LSortProp) (_root_.ByteArray.mk #[]))))

@[expose] public def validateDenotation (isCore : Bool) (dKind : ByteArray) (dConstructor : Option (ByteArray)) (dModule : Option (ByteArray)) (dName : Option (ByteArray)) (dComponent : Option (ByteArray)) (dValue : Option (ByteArray)) (maxScopeDepth : Nat) (universes : List (ByteArray)) (diagsIn : List ((Prod (ByteArray) (ByteArray)))) : (Prod (List ((Prod (ByteArray) (ByteArray)))) (Denotation)) := (let fc : Nat := optCount (dConstructor) (dModule) (dName) (dComponent) (dValue); (if (LexLeanRuntime.equal (dKind) (lit ("core")) : Bool) then validateCoreDenotation (isCore) (dConstructor) (fc) (diagsIn) else (if (LexLeanRuntime.equal (dKind) (lit ("lean")) : Bool) then validateLeanDenotation (dModule) (dName) (fc) (diagsIn) else (if (LexLeanRuntime.equal (dKind) (lit ("document")) : Bool) then validateDocumentDenotation (dModule) (dComponent) (fc) (diagsIn) else (if (LexLeanRuntime.equal (dKind) (lit ("defined")) : Bool) then validateDefinedDenotation (dValue) (fc) (maxScopeDepth) (universes) (diagsIn) else (appendRev (diagsIn) (Option.some (errD (cat (quoted (dKind)) (lit (" is not a denotation kind"))))), Denotation.DCore (_root_.ByteArray.mk #[])))))))

@[expose] public def isControlAtom (atom : LexLeanBoot.PrimitiveScan.Atom) : Bool := (match (atom).kind with | LexLeanBoot.PrimitiveScan.AtomClass.Control => true | LexLeanBoot.PrimitiveScan.AtomClass.Word => false | LexLeanBoot.PrimitiveScan.AtomClass.Numeral => false | LexLeanBoot.PrimitiveScan.AtomClass.Delimiter => false | LexLeanBoot.PrimitiveScan.AtomClass.AsciiSymbol => false | LexLeanBoot.PrimitiveScan.AtomClass.UnicodeSymbol => false | LexLeanBoot.PrimitiveScan.AtomClass.Whitespace => false)

@[expose] public def anyControlAtom (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (atom : LexLeanBoot.PrimitiveScan.Atom) => (if found then true else isControlAtom (atom)))) (false) (atoms) : Bool)

@[expose] public def listHasBytes (items : List (ByteArray)) (target : ByteArray) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (item : ByteArray) => (if found then true else (LexLeanRuntime.equal (item) (target) : Bool)))) (false) (items) : Bool)

@[expose] public def forbiddenAtomDiags (forbiddenControls : List (ByteArray)) (formId : ByteArray) (atoms : List (LexLeanBoot.PrimitiveScan.Atom)) (diagsIn : List ((Prod (ByteArray) (ByteArray)))) : List ((Prod (ByteArray) (ByteArray))) := (LexLeanCollections.listFold ((fun (acc : List ((Prod (ByteArray) (ByteArray)))) (atom : LexLeanBoot.PrimitiveScan.Atom) => (if isControlAtom (atom) then (if listHasBytes (forbiddenControls) ((atom).text) then (errD (cat (cat (cat (lit ("form `")) (formId)) (lit ("` declares the always-forbidden control `"))) (cat ((atom).text) (lit ("` (§12.4)")))) :: acc) else acc) else acc))) (diagsIn) (atoms) : List ((Prod (ByteArray) (ByteArray))))

@[expose] public def featureDiags (formId : ByteArray) (features : List (ByteArray)) (diagsIn : List ((Prod (ByteArray) (ByteArray)))) : List ((Prod (ByteArray) (ByteArray))) := (let d1 : List ((Prod (ByteArray) (ByteArray))) := (if isSortedUniqueStrict (features) then diagsIn else (errD (cat (cat (lit ("form `")) (formId)) (lit ("`: features must be sorted and unique"))) :: diagsIn)); (LexLeanCollections.listFold ((fun (acc : List ((Prod (ByteArray) (ByteArray)))) (feature : ByteArray) => (if LexLeanBoot.LexiconSchema.isIn (LexLeanBoot.LexiconSchema.features) (feature) then acc else (errD (cat (cat (cat (cat (lit ("form `")) (formId)) (lit ("`: `"))) (feature)) (lit ("` is not a feature"))) :: acc)))) (d1) (features) : List ((Prod (ByteArray) (ByteArray)))))

@[expose] public def surfaceWhitespaceBad (surface : ByteArray) : Bool := (if (Nat.beq ((LexLeanRuntime.length (surface) : Nat)) (0)) then true else (if LexLeanStd.Text.startsWith (surface) (lit (" ")) then true else (if LexLeanStd.Text.endsWith (surface) (lit (" ")) then true else (if (match LexLeanStd.Text.find (surface) (cat (lit (" ")) (lit (" "))) with | Option.some _ => true | Option.none => false) then true else (match LexLeanStd.Text.find (surface) (_root_.ByteArray.mk #[(_root_.UInt8.ofNat (nat_lit 10))]) with | Option.some _ => true | Option.none => false)))))

public structure FormGate where
  diagsRev : List ((Prod (ByteArray) (ByteArray)))
  channelIdx : Option (Nat)
  atoms : List (LexLeanBoot.PrimitiveScan.Atom)

@[expose] public def formGate (formId : ByteArray) (channelText : ByteArray) (surface : ByteArray) (diagsIn : List ((Prod (ByteArray) (ByteArray)))) : FormGate := (match LexLeanBoot.LexiconSchema.tokenIndex (2) (channelText) with | Option.none => ({ diagsRev := appendRev (diagsIn) (Option.some (errD (lit ("a form channel is text, math, or both")))), channelIdx := Option.none, atoms := ([] : List (LexLeanBoot.PrimitiveScan.Atom)) } : FormGate) | Option.some channelIdx => (if surfaceWhitespaceBad (surface) then ({ diagsRev := appendRev (diagsIn) (Option.some (errD (cat (cat (lit ("form `")) (formId)) (lit ("`: leading, trailing, or repeated whitespace in the surface is forbidden"))))), channelIdx := Option.none, atoms := ([] : List (LexLeanBoot.PrimitiveScan.Atom)) } : FormGate) else (match LexLeanBoot.PrimitiveScan.scanSource (lit ("<form>")) (surface) ((LexLeanRuntime.length (surface) : Nat)) with | Except.error rejection => ({ diagsRev := appendRev (diagsIn) (Option.some (errD (cat (cat (cat (lit ("form `")) (formId)) (lit ("`: surface does not scan: "))) ((rejection).message)))), channelIdx := Option.none, atoms := ([] : List (LexLeanBoot.PrimitiveScan.Atom)) } : FormGate) | Except.ok atoms => ({ diagsRev := diagsIn, channelIdx := Option.some (channelIdx), atoms := atoms } : FormGate))))

@[expose] public def validateOneForm (raw : RawForm) (isDuplicate : Bool) (isCore : Bool) (forbiddenControls : List (ByteArray)) (diagsIn : List ((Prod (ByteArray) (ByteArray)))) : (Prod (List ((Prod (ByteArray) (ByteArray)))) (Option (Form))) := (let formId : ByteArray := (raw).id; (let d1 : List ((Prod (ByteArray) (ByteArray))) := appendRev (diagsIn) ((if LexLeanBoot.LexiconLre.isFormId (formId) then Option.none else Option.some (errD (cat (quoted (formId)) (lit (" is not a form ID ([a-z][a-z0-9-]*)")))))); (let d2 : List ((Prod (ByteArray) (ByteArray))) := appendRev (d1) ((if isDuplicate then Option.some (errD (cat (lit ("duplicate form ID ")) (quoted (formId)))) else Option.none)); (let gate : FormGate := formGate (formId) ((raw).channel) ((raw).surface) (d2); (match (gate).channelIdx with | Option.none => ((gate).diagsRev, Option.none) | Option.some channelIdx => (let atoms : List (LexLeanBoot.PrimitiveScan.Atom) := (gate).atoms; (let d3 : List ((Prod (ByteArray) (ByteArray))) := forbiddenAtomDiags (forbiddenControls) (formId) (atoms) ((gate).diagsRev); (let isControlForm : Bool := anyControlAtom (atoms); (let d4 : List ((Prod (ByteArray) (ByteArray))) := appendRev (d3) ((if ((isControlForm && (raw).canonicalSource) && (if isCore then false else true)) then Option.some (errD (cat (cat (lit ("form `")) (formId)) (lit ("`: control-sequence forms are aliases only for non-core entries")))) else Option.none)); (let singleControlAlias : Bool := (((if (raw).canonicalSource then false else true) && (Nat.beq ((LexLeanRuntime.length (atoms) : Nat)) (1))) && anyControlAtom (atoms)); (let d5 : List ((Prod (ByteArray) (ByteArray))) := (if ((if isCore then false else true) && (if singleControlAlias then false else true)) then (match LexLeanBoot.LexiconSchema.surfaceSafety (atoms) (channelIdx) with | Option.some reason => appendRev (d4) (Option.some (formUnsafeD (cat (cat (cat (cat (cat (lit ("form `")) (formId)) (lit ("`: the "))) ((if (raw).canonicalSource then lit ("canonical") else lit ("alias")))) (cat (lit (" surface `")) ((raw).surface))) (cat (lit ("` is not renderer-safe: ")) (reason))))) | Option.none => d4) else d4); (let d6 : List ((Prod (ByteArray) (ByteArray))) := featureDiags (formId) ((raw).features) (d5); (d6, Option.some (({ id := formId, channel := channelIdx, surface := (raw).surface, atoms := atoms, canonicalSource := (raw).canonicalSource, features := (raw).features } : Form)))))))))))))))

public structure FormFoldState where
  diagsRev : List ((Prod (ByteArray) (ByteArray)))
  formsRev : List (Form)
  seenIds : List (ByteArray)

@[expose] public def formFoldStep (isCore : Bool) (forbiddenControls : List (ByteArray)) (state : FormFoldState) (raw : RawForm) : FormFoldState := (let isDup : Bool := listHasBytes ((state).seenIds) ((raw).id); (let stepOut : (Prod (List ((Prod (ByteArray) (ByteArray)))) (Option (Form))) := validateOneForm (raw) (isDup) (isCore) (forbiddenControls) ((state).diagsRev); ({ diagsRev := (stepOut).1, formsRev := (match (stepOut).2 with | Option.some f => (f :: (state).formsRev) | Option.none => (state).formsRev), seenIds := ((raw).id :: (state).seenIds) } : FormFoldState)))

public structure FormsOut where
  diagsRev : List ((Prod (ByteArray) (ByteArray)))
  forms : List (Form)

@[expose] public def validateForms (raws : List (RawForm)) (isCore : Bool) (forbiddenControls : List (ByteArray)) (diagsIn : List ((Prod (ByteArray) (ByteArray)))) : FormsOut := (let diags0 : List ((Prod (ByteArray) (ByteArray))) := appendRev (diagsIn) ((match raws with | List.nil => Option.some (errD (lit ("an entry declares at least one form"))) | List.cons _ _ => Option.none)); (let final : FormFoldState := (LexLeanCollections.listFold ((fun (state : FormFoldState) (raw : RawForm) => formFoldStep (isCore) (forbiddenControls) (state) (raw))) (({ diagsRev := diags0, formsRev := ([] : List (Form)), seenIds := ([] : List (ByteArray)) } : FormFoldState)) (raws) : FormFoldState); ({ diagsRev := (final).diagsRev, forms := LexLeanStd.Order.reverseList (Form) ((final).formsRev) } : FormsOut)))

@[expose] public def coversChannel (formChannel : Nat) (target : Nat) : Bool := (if (Nat.beq (formChannel) (2)) then true else (Nat.beq (formChannel) (target)))

@[expose] public def anyCanonical (forms : List (Form)) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (form : Form) => (if found then true else (form).canonicalSource))) (false) (forms) : Bool)

@[expose] public def countCanonical (forms : List (Form)) (target : Nat) : Nat := (LexLeanCollections.listFold ((fun (count : Nat) (form : Form) => (if ((form).canonicalSource && coversChannel ((form).channel) (target)) then (count + 1) else count))) (0) (forms) : Nat)

@[expose] public def listHasNat (items : List (Nat)) (target : Nat) : Bool := (LexLeanCollections.listFold ((fun (found : Bool) (item : Nat) => (if found then true else (Nat.beq (item) (target))))) (false) (items) : Bool)

@[expose] public def validateCanonicalFormsForChannel (categoryText : ByteArray) (requiredChannels : List (Nat)) (forms : List (Form)) (target : Nat) (channelName : ByteArray) (diagsIn : List ((Prod (ByteArray) (ByteArray)))) : List ((Prod (ByteArray) (ByteArray))) := (let count : Nat := countCanonical (forms) (target); (let d1 : List ((Prod (ByteArray) (ByteArray))) := appendRev (diagsIn) ((if (Nat.blt (1) (count)) then Option.some (errD (cat (lit ("more than one canonical source form for the ")) (cat (channelName) (lit (" channel"))))) else Option.none)); appendRev (d1) ((if (listHasNat (requiredChannels) (target) && (Nat.beq (count) (0))) then Option.some (errD (cat (cat (lit ("category `")) (categoryText)) (cat (lit ("` requires a canonical source form in the ")) (cat (channelName) (lit (" channel")))))) else Option.none))))

@[expose] public def validateCanonicalForms (categoryText : ByteArray) (needsSome : Bool) (requiredChannels : List (Nat)) (forms : List (Form)) (diagsIn : List ((Prod (ByteArray) (ByteArray)))) : List ((Prod (ByteArray) (ByteArray))) := (let d1 : List ((Prod (ByteArray) (ByteArray))) := appendRev (diagsIn) ((if (needsSome && (if anyCanonical (forms) then false else true)) then Option.some (errD (cat (cat (lit ("category `")) (categoryText)) (lit ("` requires a canonical source form in the text or the math channel")))) else Option.none)); (let d2 : List ((Prod (ByteArray) (ByteArray))) := validateCanonicalFormsForChannel (categoryText) (requiredChannels) (forms) (0) (lit ("text")) (d1); validateCanonicalFormsForChannel (categoryText) (requiredChannels) (forms) (1) (lit ("math")) (d2)))

@[expose] public def wrapLreFailure (what : ByteArray) (failure : LexLeanBoot.LexiconLse.Failure) : (Prod (ByteArray) (ByteArray)) := (if (Nat.beq ((failure).kind) (1)) then limitD (cat (cat (what) (lit (" render: "))) ((failure).message)) else errD (cat (cat (cat (lit ("invalid ")) (what)) (lit (" LRE: "))) ((failure).message)))

@[expose] public def insertNatSorted (items : List (Nat)) (key : Nat) : List (Nat) := (let done : (Prod (Bool) (List (Nat))) := (LexLeanCollections.listFold ((fun (state : (Prod (Bool) (List (Nat)))) (item : Nat) => (if (state).1 then (true, (item :: (state).2)) else (if (Nat.ble (key) (item)) then (true, (item :: (key :: (state).2))) else (false, (item :: (state).2)))))) ((false, ([] : List (Nat)))) (items) : (Prod (Bool) (List (Nat)))); (match (done).1 with | Bool.true => LexLeanStd.Order.reverseList (Nat) ((done).2) | Bool.false => LexLeanStd.Order.reverseList (Nat) ((key :: (done).2))))

@[expose] public def sortNats (items : List (Nat)) : List (Nat) := (LexLeanCollections.listFold ((fun (acc : List (Nat)) (item : Nat) => insertNatSorted (acc) (item))) (([] : List (Nat))) (items) : List (Nat))

@[expose] public def rangeUpTo (n : Nat) : List (Nat) := LexLeanStd.Order.reverseList (Nat) ((((LexLeanCollections.iterateUntil ((fun (st : (Prod (Nat) (List (Nat)))) => (if (Nat.blt ((st).1) (n)) then Option.some ((((st).1 + 1), ((st).1 :: (st).2))) else Option.none))) ((n + 1)) ((0, ([] : List (Nat)))) : (Prod ((Prod (Nat) (List (Nat)))) (Bool)))).1).2)

@[expose] public def natListText (items : List (Nat)) : ByteArray := (LexLeanCollections.listFold ((fun (acc : ByteArray) (item : Nat) => cat (cat (acc) (lit (","))) (LexLeanStd.Numeral.decimal (item)))) (_root_.ByteArray.mk #[]) (items) : ByteArray)

@[expose] public def findFormById (forms : List (Form)) (target : ByteArray) : Option (Form) := (LexLeanCollections.listFold ((fun (found : Option (Form)) (form : Form) => (match found with | Option.some _ => found | Option.none => (if (LexLeanRuntime.equal ((form).id) (target) : Bool) then Option.some (form) else Option.none)))) (Option.none) (forms) : Option (Form))

@[expose] public def validateOneSelfFormRef (forms : List (Form)) (targetChannel : Nat) (channelName : ByteArray) (selfForm : ByteArray) (diagsIn : List ((Prod (ByteArray) (ByteArray)))) : List ((Prod (ByteArray) (ByteArray))) := (match findFormById (forms) (selfForm) with | Option.none => (errD (cat (cat (cat (channelName) (lit (" render references unknown self form "))) (quoted (selfForm))) (_root_.ByteArray.mk #[])) :: diagsIn) | Option.some form => (let d1 : List ((Prod (ByteArray) (ByteArray))) := appendRev (diagsIn) ((if coversChannel ((form).channel) (targetChannel) then Option.none else Option.some (errD (cat (cat (cat (channelName) (lit (" render references self form "))) (quoted (selfForm))) (cat (lit (", which is not a ")) (cat (channelName) (lit (" form")))))))); (match LexLeanBoot.LexiconSchema.surfaceSafety ((form).atoms) ((form).channel) with | Option.none => d1 | Option.some reason => appendRev (d1) (Option.some (formUnsafeD (cat (cat (cat (cat (cat (channelName) (lit (" render references self form "))) (quoted (selfForm))) (lit (" whose surface `"))) ((form).surface)) (cat (lit ("` is not renderer-safe: ")) (reason))))))))

@[expose] public def validateSelfFormRefs (selfForms : List (ByteArray)) (forms : List (Form)) (targetChannel : Nat) (channelName : ByteArray) (diagsIn : List ((Prod (ByteArray) (ByteArray)))) : List ((Prod (ByteArray) (ByteArray))) := (LexLeanCollections.listFold ((fun (acc : List ((Prod (ByteArray) (ByteArray)))) (selfForm : ByteArray) => validateOneSelfFormRef (forms) (targetChannel) (channelName) (selfForm) (acc))) (diagsIn) (selfForms) : List ((Prod (ByteArray) (ByteArray))))

public structure RenderStepOut where
  diagsRev : List ((Prod (ByteArray) (ByteArray)))
  render : Option (LexLeanBoot.LexiconLre.Render)

@[expose] public def validateOneRender (channelName : ByteArray) (targetChannel : Nat) (renderSourceOpt : Option (ByteArray)) (forms : List (Form)) (maxScopeDepth : Nat) (surfaceArity : Nat) (diagsIn : List ((Prod (ByteArray) (ByteArray)))) : RenderStepOut := (match renderSourceOpt with | Option.none => ({ diagsRev := diagsIn, render := Option.none } : RenderStepOut) | Option.some src => (match LexLeanBoot.LexiconLre.parseLre (src) (maxScopeDepth) with | Except.error failure => ({ diagsRev := appendRev (diagsIn) (Option.some (wrapLreFailure (channelName) (failure))), render := Option.none } : RenderStepOut) | Except.ok render => (let nodes : Nat := ((LexLeanRuntime.length (src) : Nat) + 2); (let facts : LexLeanBoot.LexiconLre.Facts := LexLeanBoot.LexiconLre.renderFacts (render) (nodes); (let d1 : List ((Prod (ByteArray) (ByteArray))) := appendRev (diagsIn) ((if (LexLeanRuntime.equal (natListText (sortNats ((facts).slots))) (natListText (rangeUpTo (surfaceArity))) : Bool) then Option.none else Option.some (errD (cat (cat (channelName) (lit (" render must use every slot 0.."))) (cat (LexLeanStd.Numeral.decimal (surfaceArity)) (lit (" exactly once"))))))); (let d2 : List ((Prod (ByteArray) (ByteArray))) := (match (facts).scriptError with | Option.none => d1 | Option.some problem => appendRev (d1) (Option.some (errD (cat (cat (channelName) (lit (" render: "))) (problem))))); (let d3 : List ((Prod (ByteArray) (ByteArray))) := (if (Nat.beq (targetChannel) (0)) then (match (facts).mathOnly with | Option.none => d2 | Option.some construct => appendRev (d2) (Option.some (errD (cat (cat (lit ("text render uses `")) (construct)) (lit ("`, a mathematical construct with no text rendering")))))) else d2); (let d4 : List ((Prod (ByteArray) (ByteArray))) := validateSelfFormRefs ((facts).selfForms) (forms) (targetChannel) (channelName) (d3); ({ diagsRev := d4, render := Option.some (render) } : RenderStepOut)))))))))

public structure RenderBlockOut where
  diagsRev : List ((Prod (ByteArray) (ByteArray)))
  renderMath : Option (LexLeanBoot.LexiconLre.Render)
  renderText : Option (LexLeanBoot.LexiconLre.Render)

@[expose] public def isSomeBytesOpt (o : Option (ByteArray)) : Bool := (match o with | Option.some _ => true | Option.none => false)

@[expose] public def validateRenderBlock (hasRender : Bool) (categoryIdx : Nat) (categoryText : ByteArray) (needsSome : Bool) (requiredChannels : List (Nat)) (renderMathText : Option (ByteArray)) (renderTextText : Option (ByteArray)) (forms : List (Form)) (maxScopeDepth : Nat) (surfaceArity : Nat) (frameIdx : Nat) (diagsIn : List ((Prod (ByteArray) (ByteArray)))) : RenderBlockOut := (let d1 : List ((Prod (ByteArray) (ByteArray))) := (if hasRender then appendRev (diagsIn) ((if (if (Nat.beq (categoryIdx) (0)) then true else (Nat.beq (categoryIdx) (1))) then Option.some (errD (lit ("structural and grammar entries do not carry render templates"))) else Option.none)) else diagsIn); (let hasTextCanonical : Bool := (needsSome && (Nat.blt (0) (countCanonical (forms) (0)))); (let d2 : List ((Prod (ByteArray) (ByteArray))) := (if hasRender then appendRev (d1) ((if (isSomeBytesOpt (renderTextText) && ((if listHasNat (requiredChannels) (0) then false else true) && (if hasTextCanonical then false else true))) then Option.some (errD (cat (cat (lit ("category `")) (categoryText)) (lit ("` has no text canonical form, so a text render does not apply")))) else Option.none)) else d1); (let mathStep : RenderStepOut := validateOneRender (lit ("math")) (1) (renderMathText) (forms) (maxScopeDepth) (surfaceArity) (d2); (let textStep : RenderStepOut := validateOneRender (lit ("text")) (0) (renderTextText) (forms) (maxScopeDepth) (surfaceArity) ((mathStep).diagsRev); (let needsMathRender : Bool := (if (Nat.beq (frameIdx) (1)) then true else (if (Nat.beq (frameIdx) (2)) then true else (if (Nat.beq (frameIdx) (3)) then true else (Nat.beq (frameIdx) (4))))); (let d3 : List ((Prod (ByteArray) (ByteArray))) := appendRev ((textStep).diagsRev) ((if (needsMathRender && (match (mathStep).render with | Option.some _ => false | Option.none => true)) then Option.some (errD (lit ("a math frame with surface arguments requires a canonical math render"))) else Option.none)); ({ diagsRev := d3, renderMath := (mathStep).render, renderText := (textStep).render } : RenderBlockOut))))))))

@[expose] public def validateCalculation (hasCalculation : Bool) (calcKind : ByteArray) (categoryIdx : Nat) (diagsIn : List ((Prod (ByteArray) (ByteArray)))) : (Prod (List ((Prod (ByteArray) (ByteArray)))) (Bool)) := (if (if hasCalculation then false else true) then (diagsIn, false) else (let d1 : List ((Prod (ByteArray) (ByteArray))) := appendRev (diagsIn) ((if (LexLeanRuntime.equal (calcKind) (lit ("equality")) : Bool) then Option.none else Option.some (errD (lit ("language 1.0 ships exactly the equality calculation descriptor"))))); (let d2 : List ((Prod (ByteArray) (ByteArray))) := appendRev (d1) ((if (Nat.beq (categoryIdx) (15)) then Option.none else Option.some (errD (lit ("a calculation descriptor belongs to an infix relation"))))); (d2, true))))

@[expose] public def eliminatorBinderStep (state : (Prod (List ((Prod (ByteArray) (ByteArray)))) (List (ByteArray)))) (name : ByteArray) : (Prod (List ((Prod (ByteArray) (ByteArray)))) (List (ByteArray))) := (let isDup : Bool := listHasBytes ((state).2) (name); (let bad : Bool := (if LexLeanBoot.LexiconLse.isLseIdentifier (name) then isDup else true); (appendRev ((state).1) ((if bad then Option.some (errD (cat (cat (lit ("invalid or duplicate eliminator binder ")) (quoted (name))) (_root_.ByteArray.mk #[]))) else Option.none)), (name :: (state).2))))

@[expose] public def validateEliminatorBinders (names : List (ByteArray)) (diagsIn : List ((Prod (ByteArray) (ByteArray)))) : List ((Prod (ByteArray) (ByteArray))) := ((LexLeanCollections.listFold ((fun (state : (Prod (List ((Prod (ByteArray) (ByteArray)))) (List (ByteArray)))) (name : ByteArray) => eliminatorBinderStep (state) (name))) ((diagsIn, ([] : List (ByteArray)))) (names) : (Prod (List ((Prod (ByteArray) (ByteArray)))) (List (ByteArray))))).1

@[expose] public def qualifiedText (package : ByteArray) (entryName : ByteArray) : ByteArray := cat (cat (package) (lit ("::"))) (entryName)

public structure ConstructorFoldState where
  diagsRev : List ((Prod (ByteArray) (ByteArray)))
  constructorsRev : List (EliminatorConstructor)
  seenQualified : List (ByteArray)

@[expose] public def constructorFoldStep (state : ConstructorFoldState) (raw : RawConstructor) : ConstructorFoldState := (match LexLeanBoot.LexiconLse.qualifiedId ((raw).entry) with | Except.error failure => ({ diagsRev := appendRev ((state).diagsRev) (Option.some (errD ((failure).message))), constructorsRev := (state).constructorsRev, seenQualified := (state).seenQualified } : ConstructorFoldState) | Except.ok parts => (let package : ByteArray := (parts).1; (let entryName : ByteArray := (parts).2; (let qt : ByteArray := qualifiedText (package) (entryName); (let isDup : Bool := listHasBytes ((state).seenQualified) (qt); (let d1 : List ((Prod (ByteArray) (ByteArray))) := appendRev ((state).diagsRev) ((if isDup then Option.some (errD (cat (lit ("duplicate eliminator constructor ")) (quoted (qt)))) else Option.none)); (let d2 : List ((Prod (ByteArray) (ByteArray))) := appendRev (d1) ((if LexLeanBoot.LexiconSchema.isLeanName ((raw).leanName) then Option.none else Option.some (errD (cat (quoted ((raw).leanName)) (lit (" is not a Lean name")))))); (let d3 : List ((Prod (ByteArray) (ByteArray))) := validateEliminatorBinders ((raw).fields) (d2); (let d4 : List ((Prod (ByteArray) (ByteArray))) := validateEliminatorBinders ((raw).inductionHypotheses) (d3); ({ diagsRev := d4, constructorsRev := (({ package := package, entry := entryName, leanName := (raw).leanName, fields := (raw).fields, inductionHypotheses := (raw).inductionHypotheses } : EliminatorConstructor) :: (state).constructorsRev), seenQualified := (qt :: (state).seenQualified) } : ConstructorFoldState))))))))))

public structure EliminatorOut where
  diagsRev : List ((Prod (ByteArray) (ByteArray)))
  eliminator : Option (Eliminator)

@[expose] public def validateEliminator (rawElim : Option (RawEliminator)) (signatureSortValued : Bool) (entryId : ByteArray) (diagsIn : List ((Prod (ByteArray) (ByteArray)))) : EliminatorOut := (match rawElim with | Option.none => ({ diagsRev := diagsIn, eliminator := Option.none } : EliminatorOut) | Option.some re => (let d1 : List ((Prod (ByteArray) (ByteArray))) := appendRev (diagsIn) ((if signatureSortValued then Option.none else Option.some (errD (cat (cat (quoted (entryId)) (lit (": an eliminator descriptor belongs to a type entry, whose signature results in a sort"))) (_root_.ByteArray.mk #[]))))); (let d2 : List ((Prod (ByteArray) (ByteArray))) := appendRev (d1) ((if LexLeanBoot.LexiconSchema.isLeanName ((re).casesLeanName) then Option.none else Option.some (errD (cat (cat (lit ("eliminator cases_lean_name ")) (quoted ((re).casesLeanName))) (lit (" is not a Lean name")))))); (let d3 : List ((Prod (ByteArray) (ByteArray))) := appendRev (d2) ((if LexLeanBoot.LexiconSchema.isLeanName ((re).inductionLeanName) then Option.none else Option.some (errD (cat (cat (lit ("eliminator induction_lean_name ")) (quoted ((re).inductionLeanName))) (lit (" is not a Lean name")))))); (let folded : ConstructorFoldState := (LexLeanCollections.listFold ((fun (state : ConstructorFoldState) (raw : RawConstructor) => constructorFoldStep (state) (raw))) (({ diagsRev := d3, constructorsRev := ([] : List (EliminatorConstructor)), seenQualified := ([] : List (ByteArray)) } : ConstructorFoldState)) ((re).constructors) : ConstructorFoldState); (let constructors : List (EliminatorConstructor) := LexLeanStd.Order.reverseList (EliminatorConstructor) ((folded).constructorsRev); (let d4 : List ((Prod (ByteArray) (ByteArray))) := appendRev ((folded).diagsRev) ((if (Nat.beq ((LexLeanRuntime.length (constructors) : Nat)) (0)) then Option.some (errD (lit ("an eliminator needs at least one constructor"))) else Option.none)); ({ diagsRev := d4, eliminator := Option.some (({ casesLeanName := (re).casesLeanName, inductionLeanName := (re).inductionLeanName, constructors := constructors } : Eliminator)) } : EliminatorOut))))))))

@[expose] public def parseEntry (text : ByteArray) (isCore : Bool) (forbiddenControls : List (ByteArray)) (maxScopeDepth : Nat) : Except (List ((Prod (ByteArray) (ByteArray)))) (Entry) := (match LexLeanBoot.TomlRead.tomlParse (text) with | Option.none => Except.error ((errD (lit ("invalid entry file")) :: ([] : List ((Prod (ByteArray) (ByteArray)))))) | Option.some doc => (match extractRaw (doc) with | Option.none => Except.error ((errD (lit ("invalid entry file")) :: ([] : List ((Prod (ByteArray) (ByteArray)))))) | Option.some raw => (if (if (LexLeanRuntime.equal ((raw).spec) (lit ("lexlean/entry/1")) : Bool) then false else true) then Except.error (((lit ("LLC0103"), cat (cat (lit ("unsupported entry schema ")) (quoted ((raw).spec))) (_root_.ByteArray.mk #[])) :: ([] : List ((Prod (ByteArray) (ByteArray)))))) else (match LexLeanBoot.LexiconSchema.tokenIndex (0) ((raw).category) with | Option.none => Except.error ((errD (cat (quoted ((raw).category)) (lit (" is not a category"))) :: ([] : List ((Prod (ByteArray) (ByteArray)))))) | Option.some categoryIdx => (match LexLeanBoot.LexiconSchema.tokenIndex (1) ((raw).frame) with | Option.none => Except.error ((errD (cat (quoted ((raw).frame)) (lit (" is not a frame"))) :: ([] : List ((Prod (ByteArray) (ByteArray)))))) | Option.some frameIdx => (let shape : LexLeanBoot.LexiconSchema.Shape := LexLeanBoot.LexiconSchema.shapeFacts (categoryIdx) (frameIdx) ((raw).surfaceArity); (let headerOut : HeaderOut := validateHeader ((raw).id) (categoryIdx) (frameIdx) ((raw).category) ((raw).frame) ((raw).surfaceArity) ((raw).precedence) ((raw).associativity) (isCore); (let sigOut : SigOut := validateSignature ((raw).id) ((raw).category) ((raw).universes) ((raw).signature) ((shape).hasSignature) ((raw).surfaceArity) (maxScopeDepth) ((headerOut).diagsRev); (let denOut : (Prod (List ((Prod (ByteArray) (ByteArray)))) (Denotation)) := validateDenotation (isCore) ((raw).dKind) ((raw).dConstructor) ((raw).dModule) ((raw).dName) ((raw).dComponent) ((raw).dValue) (maxScopeDepth) ((sigOut).universes) ((sigOut).diagsRev); (let formsOut : FormsOut := validateForms ((raw).forms) (isCore) (forbiddenControls) ((denOut).1); (let canonOut : List ((Prod (ByteArray) (ByteArray))) := validateCanonicalForms ((raw).category) ((shape).needsSome) ((shape).requiredChannels) ((formsOut).forms) ((formsOut).diagsRev); (let renderOut : RenderBlockOut := validateRenderBlock ((raw).hasRender) (categoryIdx) ((raw).category) ((shape).needsSome) ((shape).requiredChannels) ((raw).renderMath) ((raw).renderText) ((formsOut).forms) (maxScopeDepth) ((raw).surfaceArity) (frameIdx) (canonOut); (let calcOut : (Prod (List ((Prod (ByteArray) (ByteArray)))) (Bool)) := validateCalculation ((raw).hasCalculation) ((raw).calcKind) (categoryIdx) ((renderOut).diagsRev); (let signatureSortValued : Bool := (match (sigOut).signature with | Option.none => false | Option.some sig => (LexLeanBoot.LseScopes.outerShape (sig) ((LexLeanRuntime.length (text) : Nat))).2); (let elimOut : EliminatorOut := validateEliminator ((raw).eliminator) (signatureSortValued) ((raw).id) ((calcOut).1); (match (elimOut).diagsRev with | List.cons _ _ => Except.error (LexLeanStd.Order.reverseList ((Prod (ByteArray) (ByteArray))) ((elimOut).diagsRev)) | List.nil => Except.ok (({ id := (raw).id, category := categoryIdx, frame := frameIdx, surfaceArity := (raw).surfaceArity, signature := (sigOut).signature, signatureCanonical := (sigOut).signatureCanonical, signatureHash := (sigOut).signatureHash, universes := (sigOut).universes, precedence := (headerOut).precedence, associativity := (headerOut).associativity, denotation := (denOut).2, forms := (formsOut).forms, renderMath := (renderOut).renderMath, renderText := (renderOut).renderText, calculation := (calcOut).2, eliminator := (elimOut).eliminator } : Entry))))))))))))))))))

end LexLeanBoot.LexiconEntry

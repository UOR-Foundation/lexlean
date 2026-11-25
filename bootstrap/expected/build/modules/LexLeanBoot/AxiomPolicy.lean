module
public import Init
public import LexLeanRuntime.Portable
public import LexLeanRuntime.Collections
public import LexLeanBoot.AxiomOutput
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.AxiomPolicy

public inductive Policy where
  | None
  | Allow (_ : List (ByteArray))
  | Exact (_ : List (ByteArray))

public inductive RuleVerifyPolicy where
  | Document
  | Core
  | Semantic
  | Model (_ : List (ByteArray))
  | Runtime

@[expose] public def memListOfBytesEnd (items : List (ByteArray)) (item : ByteArray) : Bool := (LexLeanCollections.listFold ((fun (acc : Bool) (head : ByteArray) => (if acc then true else (LexLeanRuntime.equal (head) (item) : Bool)))) (false) (items) : Bool)

@[expose] public def eqListOfBytesEnd (left : List (ByteArray)) (right : List (ByteArray)) : Bool := (let walked : (Prod (Bool) (List (ByteArray))) := (LexLeanCollections.listFold ((fun (acc : (Prod (Bool) (List (ByteArray)))) (lhead : ByteArray) => (if (acc).1 then (match (acc).2 with | List.nil => (false, ([] : List (ByteArray))) | List.cons rhead rtail => (if (LexLeanRuntime.equal (lhead) (rhead) : Bool) then (true, rtail) else (false, rtail))) else acc))) ((true, right)) (left) : (Prod (Bool) (List (ByteArray)))); (if (walked).1 then (match (walked).2 with | List.nil => true | List.cons _ _ => false) else false))

@[expose] public def insertsortedListOfBytesEnd (sorted : List (ByteArray)) (item : ByteArray) : List (ByteArray) := (let placed : (Prod (Bool) (List (ByteArray))) := (LexLeanCollections.listFold ((fun (acc : (Prod (Bool) (List (ByteArray)))) (head : ByteArray) => (if (acc).1 then (true, (head :: (acc).2)) else (if LexLeanBoot.AxiomOutput.lessBytes (item) (head) then (true, (head :: (item :: (acc).2))) else (false, (head :: (acc).2)))))) ((false, ([] : List (ByteArray)))) (sorted) : (Prod (Bool) (List (ByteArray)))); LexLeanBoot.AxiomOutput.reversed (ByteArray) ((if (placed).1 then (placed).2 else (item :: (placed).2))))

@[expose] public def dedupListOfBytesEnd (items : List (ByteArray)) : List (ByteArray) := LexLeanBoot.AxiomOutput.reversed (ByteArray) ((LexLeanCollections.listFold ((fun (acc : List (ByteArray)) (x : ByteArray) => (match acc with | List.nil => (x :: acc) | List.cons last _ => (if (LexLeanRuntime.equal (last) (x) : Bool) then acc else (x :: acc))))) (([] : List (ByteArray))) (items) : List (ByteArray)))

@[expose] public def policyKind (this2 : Policy) : ByteArray := (match this2 with | Policy.None => LexLeanBoot.AxiomOutput.text ("none") | Policy.Allow _ => LexLeanBoot.AxiomOutput.text ("allow") | Policy.Exact _ => LexLeanBoot.AxiomOutput.text ("exact"))

@[expose] public def policyAxioms (this2 : Policy) : List (ByteArray) := (match this2 with | Policy.None => ([] : List (ByteArray)) | Policy.Allow got3 => got3 | Policy.Exact got4 => got4)

@[expose] public def policyPermits (this2 : Policy) (observed : List (ByteArray)) : Bool := (match this2 with | Policy.None => (Nat.beq (LexLeanBoot.AxiomOutput.listLength (ByteArray) (observed)) (0)) | Policy.Allow got3 => (LexLeanCollections.listFold ((fun (acc2 : Bool) (item2 : ByteArray) => (if acc2 then memListOfBytesEnd (got3) (item2) else false))) (true) (observed) : Bool) | Policy.Exact got4 => (let sorted4 : List (ByteArray) := observed; (let sorted5 : List (ByteArray) := (LexLeanCollections.listFold ((fun (acc : List (ByteArray)) (x : ByteArray) => insertsortedListOfBytesEnd (acc) (x))) (([] : List (ByteArray))) (sorted4) : List (ByteArray)); (let sorted6 : List (ByteArray) := dedupListOfBytesEnd (sorted5); eqListOfBytesEnd (sorted6) (got4)))))

@[expose] public def ruleToken (this2 : RuleVerifyPolicy) : ByteArray := (match this2 with | RuleVerifyPolicy.Document => LexLeanBoot.AxiomOutput.text ("document") | RuleVerifyPolicy.Core => LexLeanBoot.AxiomOutput.text ("core") | RuleVerifyPolicy.Semantic => LexLeanBoot.AxiomOutput.text ("semantic") | RuleVerifyPolicy.Model _ => LexLeanBoot.AxiomOutput.text ("model") | RuleVerifyPolicy.Runtime => LexLeanBoot.AxiomOutput.text ("runtime"))

end LexLeanBoot.AxiomPolicy

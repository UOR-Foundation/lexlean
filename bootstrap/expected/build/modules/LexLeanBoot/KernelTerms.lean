module
public import Init
public import LexLeanRuntime.Portable
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
set_option linter.constructorNameAsVariable false
namespace LexLeanBoot.KernelTerms

public inductive Term where
  | bvar (_ : Nat)
  | sort (_ : Nat)
  | const (_ : Nat)
  | app (_ : Term) (_ : Term)
  | lam (_ : Term) (_ : Term)
  | pi (_ : Term) (_ : Term)

public inductive Refusal where
  | notAnEquation
  | missingCase (_ : Nat)
  | repeatedCase (_ : Nat)
  | binderCount (_ : Nat)
  | unknownCase (_ : Nat)
  | notProduced (_ : Nat)
  | unregisteredLaw (_ : Nat)

public structure Decl where
  name : Nat
  statement : Term
  value : Term

public inductive Equation where
  | equation (_ : Term) (_ : Term)
  | iff (_ : Term)
  | other

public inductive Check where
  | pass
  | fail (_ : Refusal)

public inductive Produced where
  | made (_ : Term)
  | refused (_ : Refusal)

public inductive Declared where
  | declared (_ : Decl)
  | rejected (_ : Refusal)

public structure Ctor where
  fields : Nat
  recursive : Nat

public structure Branch where
  ctor : Nat
  binders : Nat
  proof : Term

@[expose] public def apps : (fn : Term) -> (args : List (Term)) -> Term
  | fn, List.nil => fn
  | fn, List.cons head rest => apps (Term.app (fn) (head)) (rest)

@[expose] public def countCase : (index : Nat) -> (branches : List (Branch)) -> Nat
  | _index, List.nil => 0
  | index, List.cons head rest => (if (Nat.beq ((head).ctor) (index)) then (countCase (index) (rest) + 1) else countCase (index) (rest))

@[expose] public def bindersOf : (index : Nat) -> (branches : List (Branch)) -> Nat
  | _index, List.nil => 0
  | index, List.cons head rest => (if (Nat.beq ((head).ctor) (index)) then (head).binders else bindersOf (index) (rest))

@[expose] public def proofOf : (index : Nat) -> (branches : List (Branch)) -> Term
  | _index, List.nil => Term.bvar (0)
  | index, List.cons head rest => (if (Nat.beq ((head).ctor) (index)) then (head).proof else proofOf (index) (rest))

@[expose] public def strayFrom : (count : Nat) -> (branches : List (Branch)) -> Option (Nat)
  | _count, List.nil => Option.none
  | count, List.cons head rest => (if (Nat.blt ((head).ctor) (count)) then strayFrom (count) (rest) else Option.some ((head).ctor))

@[expose] public def wanted (shape : Ctor) (induct : Bool) : Nat := (if induct then ((shape).fields + (shape).recursive) else (shape).fields)

@[expose] public def checkFrom : (ctors : List (Ctor)) -> (index : Nat) -> (branches : List (Branch)) -> (induct : Bool) -> Check
  | List.nil, index, branches, _induct => (match strayFrom (index) (branches) with | Option.none => Check.pass | Option.some stray => Check.fail (Refusal.unknownCase (stray)))
  | List.cons head rest, index, branches, induct => (if (Nat.beq (countCase (index) (branches)) (0)) then Check.fail (Refusal.missingCase (index)) else (if (Nat.beq (countCase (index) (branches)) (1)) then (if (Nat.beq (bindersOf (index) (branches)) (wanted (head) (induct))) then checkFrom (rest) ((index + 1)) (branches) (induct) else Check.fail (Refusal.binderCount (index))) else Check.fail (Refusal.repeatedCase (index))))

@[expose] public def minorsFrom : (ctors : List (Ctor)) -> (index : Nat) -> (branches : List (Branch)) -> List (Term)
  | List.nil, _index, _branches => ([] : List (Term))
  | List.cons _ rest, index, branches => (proofOf (index) (branches) :: minorsFrom (rest) ((index + 1)) (branches))

@[expose] public def casesTerm (casesOn : Nat) (params : List (Term)) (motive : Term) (scrutinee : Term) (ctors : List (Ctor)) (branches : List (Branch)) : Produced := (match checkFrom (ctors) (0) (branches) (false) with | Check.pass => Produced.made (apps (Term.app (Term.app (apps (Term.const (casesOn)) (params)) (motive)) (scrutinee)) (minorsFrom (ctors) (0) (branches))) | Check.fail refusal => Produced.refused (refusal))

@[expose] public def refusalOf (result : Produced) : Check := (match result with | Produced.made _ => Check.pass | Produced.refused refusal => Check.fail (refusal))

@[expose] public def declare (name : Nat) (statement : Term) (produced : Produced) : Declared := (match produced with | Produced.made value => Declared.declared (({ name := name, statement := statement, value := value } : Decl)) | Produced.refused refusal => Declared.rejected (refusal))

@[expose] public def declaredType (declared : Declared) : Option (Term) := (match declared with | Declared.declared decl => Option.some ((decl).statement) | Declared.rejected _ => Option.none)

@[expose] public def whenOk (produced : Produced) (statement : Term) : Option (Term) := (match produced with | Produced.made _ => Option.some (statement) | Produced.refused _ => Option.none)

public theorem declare_binds_statement (name : Nat) (statement : Term) (produced : Produced) : (declaredType (declare (name) (statement) (produced)) = whenOk (produced) (statement)) := by
  cases produced with
  | made value =>
    rfl
  | refused refusal =>
    rfl

public theorem cases_refuses_exactly (casesOn : Nat) (params : List (Term)) (motive : Term) (scrutinee : Term) (ctors : List (Ctor)) (branches : List (Branch)) : (refusalOf (casesTerm (casesOn) (params) (motive) (scrutinee) (ctors) (branches)) = checkFrom (ctors) (0) (branches) (false)) := by
  set_option linter.unusedSimpArgs false in
  cases llCase : checkFrom (ctors) (0) (branches) (false) <;>
    simp (config := {decide := true}) only [casesTerm, refusalOf, llCase, ite_true, ite_false]

public theorem missing_case_refused : (refusalOf (casesTerm (7) (([] : List (Term))) (Term.bvar (0)) (Term.bvar (1)) ((({ fields := 0, recursive := 0 } : Ctor) :: (({ fields := 0, recursive := 0 } : Ctor) :: ([] : List (Ctor))))) ((({ ctor := 0, binders := 0, proof := Term.const (9) } : Branch) :: ([] : List (Branch))))) = Check.fail (Refusal.missingCase (1))) := by
  rfl

@[expose] public def fnOf (term : Term) : Term := (match term with | Term.bvar _ => Term.sort (0) | Term.sort _ => Term.sort (0) | Term.const _ => Term.sort (0) | Term.app fn _ => fn | Term.lam _ _ => Term.sort (0) | Term.pi _ _ => Term.sort (0))

@[expose] public def argOf (term : Term) : Term := (match term with | Term.bvar _ => Term.sort (0) | Term.sort _ => Term.sort (0) | Term.const _ => Term.sort (0) | Term.app _ arg => arg | Term.lam _ _ => Term.sort (0) | Term.pi _ _ => Term.sort (0))

@[expose] public def constOf (term : Term) : Nat := (match term with | Term.bvar _ => 0 | Term.sort _ => 0 | Term.const name => (name + 1) | Term.app _ _ => 0 | Term.lam _ _ => 0 | Term.pi _ _ => 0)

@[expose] public def equationOf (goal : Term) : Equation := (if (Nat.beq (constOf (fnOf (fnOf (fnOf (goal))))) (1)) then Equation.equation (argOf (fnOf (fnOf (goal)))) (argOf (fnOf (goal))) else (if (Nat.beq (constOf (fnOf (fnOf (goal)))) (3)) then Equation.iff (argOf (fnOf (goal))) else Equation.other))

@[expose] public def equationCheck (goal : Term) : Check := (match equationOf (goal) with | Equation.equation _ _ => Check.pass | Equation.iff _ => Check.pass | Equation.other => Check.fail (Refusal.notAnEquation))

@[expose] public def reflexivityTerm (goal : Term) : Produced := (match equationOf (goal) with | Equation.equation ty lhs => Produced.made (Term.app (Term.app (Term.const (1)) (ty)) (lhs)) | Equation.iff lhs => Produced.made (Term.app (Term.const (3)) (lhs)) | Equation.other => Produced.refused (Refusal.notAnEquation))

@[expose] public def decideTerm (goal : Term) («instance» : Term) : Produced := Produced.made (Term.app (Term.app (Term.app (Term.const (4)) (goal)) («instance»)) (Term.app (Term.app (Term.const (1)) (Term.const (5))) (Term.const (6))))

@[expose] public def inductionTerm (recursor : Nat) (params : List (Term)) (motive : Term) (scrutinee : Term) (generals : List (Term)) (ctors : List (Ctor)) (branches : List (Branch)) : Produced := (match checkFrom (ctors) (0) (branches) (true) with | Check.pass => Produced.made (apps (apps (Term.app (apps (Term.const (recursor)) (params)) (motive)) (minorsFrom (ctors) (0) (branches))) ((scrutinee :: generals))) | Check.fail refusal => Produced.refused (refusal))

@[expose] public def memberCheck : (names : List (Nat)) -> (name : Nat) -> Check
  | List.nil, name => Check.fail (Refusal.notProduced (name))
  | List.cons head rest, name => (if (Nat.beq (head) (name)) then Check.pass else memberCheck (rest) (name))

@[expose] public def allMembers : (required : List (Nat)) -> (names : List (Nat)) -> Check
  | List.nil, _names => Check.pass
  | List.cons head rest, names => (match memberCheck (names) (head) with | Check.pass => allMembers (rest) (names) | Check.fail refusal => Check.fail (refusal))

@[expose] public def constants : (names : List (Nat)) -> List (Term)
  | List.nil => ([] : List (Term))
  | List.cons head rest => (Term.const (head) :: constants (rest))

@[expose] public def applyTerm («theorem» : Nat) (produced : List (Nat)) (args : List (Term)) : Produced := (match memberCheck (produced) («theorem») with | Check.pass => Produced.made (apps (Term.const («theorem»)) (args)) | Check.fail refusal => Produced.refused (refusal))

@[expose] public def lawCheck (law : Nat) (registered : List (Nat)) (premises : List (Nat)) (produced : List (Nat)) : Check := (match memberCheck (registered) (law) with | Check.pass => allMembers (premises) (produced) | Check.fail _ => Check.fail (Refusal.unregisteredLaw (law)))

@[expose] public def lawTerm (law : Nat) (registered : List (Nat)) (args : List (Term)) (premises : List (Nat)) (produced : List (Nat)) : Produced := (match lawCheck (law) (registered) (premises) (produced) with | Check.pass => Produced.made (apps (apps (Term.const (law)) (args)) (constants (premises))) | Check.fail refusal => Produced.refused (refusal))

public theorem reflexivity_refuses_exactly (goal : Term) : (refusalOf (reflexivityTerm (goal)) = equationCheck (goal)) := by
  set_option linter.unusedSimpArgs false in
  cases llCase : equationOf (goal) <;>
    simp (config := {decide := true}) only [equationCheck, reflexivityTerm, refusalOf, llCase, ite_true, ite_false]

public theorem decide_never_refuses (goal : Term) («instance» : Term) : (refusalOf (decideTerm (goal) («instance»)) = Check.pass) := by
  rfl

public theorem induction_refuses_exactly (recursor : Nat) (params : List (Term)) (motive : Term) (scrutinee : Term) (generals : List (Term)) (ctors : List (Ctor)) (branches : List (Branch)) : (refusalOf (inductionTerm (recursor) (params) (motive) (scrutinee) (generals) (ctors) (branches)) = checkFrom (ctors) (0) (branches) (true)) := by
  set_option linter.unusedSimpArgs false in
  cases llCase : checkFrom (ctors) (0) (branches) (true) <;>
    simp (config := {decide := true}) only [inductionTerm, refusalOf, llCase, ite_true, ite_false]

public theorem apply_refuses_exactly («theorem» : Nat) (produced : List (Nat)) (args : List (Term)) : (refusalOf (applyTerm («theorem») (produced) (args)) = memberCheck (produced) («theorem»)) := by
  set_option linter.unusedSimpArgs false in
  cases llCase : memberCheck (produced) («theorem») <;>
    simp (config := {decide := true}) only [applyTerm, refusalOf, llCase, ite_true, ite_false]

public theorem law_refuses_exactly (law : Nat) (registered : List (Nat)) (args : List (Term)) (premises : List (Nat)) (produced : List (Nat)) : (refusalOf (lawTerm (law) (registered) (args) (premises) (produced)) = lawCheck (law) (registered) (premises) (produced)) := by
  set_option linter.unusedSimpArgs false in
  cases llCase : lawCheck (law) (registered) (premises) (produced) <;>
    simp (config := {decide := true}) only [lawTerm, refusalOf, llCase, ite_true, ite_false]

public theorem repeated_case_refused : (refusalOf (casesTerm (7) (([] : List (Term))) (Term.bvar (0)) (Term.bvar (1)) ((({ fields := 0, recursive := 0 } : Ctor) :: (({ fields := 0, recursive := 0 } : Ctor) :: ([] : List (Ctor))))) ((({ ctor := 0, binders := 0, proof := Term.const (9) } : Branch) :: (({ ctor := 0, binders := 0, proof := Term.const (9) } : Branch) :: (({ ctor := 1, binders := 0, proof := Term.const (9) } : Branch) :: ([] : List (Branch))))))) = Check.fail (Refusal.repeatedCase (0))) := by
  rfl

public theorem binder_count_refused : (refusalOf (casesTerm (7) (([] : List (Term))) (Term.bvar (0)) (Term.bvar (1)) ((({ fields := 0, recursive := 0 } : Ctor) :: (({ fields := 0, recursive := 0 } : Ctor) :: ([] : List (Ctor))))) ((({ ctor := 0, binders := 1, proof := Term.const (9) } : Branch) :: (({ ctor := 1, binders := 0, proof := Term.const (9) } : Branch) :: ([] : List (Branch)))))) = Check.fail (Refusal.binderCount (0))) := by
  rfl

public theorem unknown_case_refused : (refusalOf (casesTerm (7) (([] : List (Term))) (Term.bvar (0)) (Term.bvar (1)) ((({ fields := 0, recursive := 0 } : Ctor) :: (({ fields := 0, recursive := 0 } : Ctor) :: ([] : List (Ctor))))) ((({ ctor := 0, binders := 0, proof := Term.const (9) } : Branch) :: (({ ctor := 1, binders := 0, proof := Term.const (9) } : Branch) :: (({ ctor := 5, binders := 0, proof := Term.const (9) } : Branch) :: ([] : List (Branch))))))) = Check.fail (Refusal.unknownCase (5))) := by
  rfl

public theorem cases_selects_by_constructor : (casesTerm (7) (([] : List (Term))) (Term.bvar (0)) (Term.bvar (1)) ((({ fields := 0, recursive := 0 } : Ctor) :: (({ fields := 0, recursive := 0 } : Ctor) :: ([] : List (Ctor))))) ((({ ctor := 1, binders := 0, proof := Term.const (11) } : Branch) :: (({ ctor := 0, binders := 0, proof := Term.const (10) } : Branch) :: ([] : List (Branch))))) = Produced.made (Term.app (Term.app (Term.app (Term.app (Term.const (7)) (Term.bvar (0))) (Term.bvar (1))) (Term.const (10))) (Term.const (11)))) := by
  rfl

public theorem reflexivity_term_of_equation : (reflexivityTerm (Term.app (Term.app (Term.app (Term.const (0)) (Term.const (5))) (Term.const (6))) (Term.const (6))) = Produced.made (Term.app (Term.app (Term.const (1)) (Term.const (5))) (Term.const (6)))) := by
  rfl

public theorem not_an_equation_refused : (refusalOf (reflexivityTerm (Term.const (6))) = Check.fail (Refusal.notAnEquation)) := by
  rfl

public theorem unproduced_theorem_refused : (refusalOf (applyTerm (12) ((11 :: ([] : List (Nat)))) (([] : List (Term)))) = Check.fail (Refusal.notProduced (12))) := by
  rfl

public theorem unregistered_law_refused : (refusalOf (lawTerm (13) (([] : List (Nat))) (([] : List (Term))) (([] : List (Nat))) (([] : List (Nat)))) = Check.fail (Refusal.unregisteredLaw (13))) := by
  rfl

@[expose] public def isMade (result : Produced) : Bool := (match result with | Produced.made _ => true | Produced.refused _ => false)

@[expose] public def passes (check : Check) : Bool := (match check with | Check.pass => true | Check.fail _ => false)

public theorem reflexivity_total (goal : Term) : (isMade (reflexivityTerm (goal)) = passes (equationCheck (goal))) := by
  set_option linter.unusedSimpArgs false in
  cases llCase : equationOf (goal) <;>
    simp (config := {decide := true}) only [equationCheck, isMade, passes, reflexivityTerm, llCase, ite_true, ite_false]

public theorem decide_total (goal : Term) («instance» : Term) : (isMade (decideTerm (goal) («instance»)) = true) := by
  rfl

public theorem cases_total (casesOn : Nat) (params : List (Term)) (motive : Term) (scrutinee : Term) (ctors : List (Ctor)) (branches : List (Branch)) : (isMade (casesTerm (casesOn) (params) (motive) (scrutinee) (ctors) (branches)) = passes (checkFrom (ctors) (0) (branches) (false))) := by
  set_option linter.unusedSimpArgs false in
  cases llCase : checkFrom (ctors) (0) (branches) (false) <;>
    simp (config := {decide := true}) only [casesTerm, isMade, passes, llCase, ite_true, ite_false]

public theorem induction_total (recursor : Nat) (params : List (Term)) (motive : Term) (scrutinee : Term) (generals : List (Term)) (ctors : List (Ctor)) (branches : List (Branch)) : (isMade (inductionTerm (recursor) (params) (motive) (scrutinee) (generals) (ctors) (branches)) = passes (checkFrom (ctors) (0) (branches) (true))) := by
  set_option linter.unusedSimpArgs false in
  cases llCase : checkFrom (ctors) (0) (branches) (true) <;>
    simp (config := {decide := true}) only [inductionTerm, isMade, passes, llCase, ite_true, ite_false]

public theorem apply_total («theorem» : Nat) (produced : List (Nat)) (args : List (Term)) : (isMade (applyTerm («theorem») (produced) (args)) = passes (memberCheck (produced) («theorem»))) := by
  set_option linter.unusedSimpArgs false in
  cases llCase : memberCheck (produced) («theorem») <;>
    simp (config := {decide := true}) only [applyTerm, isMade, passes, llCase, ite_true, ite_false]

public theorem law_total (law : Nat) (registered : List (Nat)) (args : List (Term)) (premises : List (Nat)) (produced : List (Nat)) : (isMade (lawTerm (law) (registered) (args) (premises) (produced)) = passes (lawCheck (law) (registered) (premises) (produced))) := by
  set_option linter.unusedSimpArgs false in
  cases llCase : lawCheck (law) (registered) (premises) (produced) <;>
    simp (config := {decide := true}) only [isMade, lawTerm, passes, llCase, ite_true, ite_false]

@[expose] public def termAt : (terms : List (Term)) -> (position : Nat) -> Term
  | List.nil, _position => Term.sort (0)
  | List.cons head rest, position => (if (Nat.beq (position) (0)) then head else termAt (rest) ((LexLeanRuntime.subtract (position) (1) : Nat)))

@[expose] public def enumerateBranches : (ctors : List (Ctor)) -> (index : Nat) -> (proofs : List (Term)) -> List (Branch)
  | List.nil, _index, _proofs => ([] : List (Branch))
  | List.cons _ rest, index, proofs => (({ ctor := index, binders := 0, proof := termAt (proofs) (index) } : Branch) :: enumerateBranches (rest) ((index + 1)) (proofs))

public theorem enumerate_none_total (proofs : List (Term)) : (passes (checkFrom (([] : List (Ctor))) (0) (enumerateBranches (([] : List (Ctor))) (0) (proofs)) (false)) = true) := by
  rfl

public theorem enumerate_one_total (proofs : List (Term)) : (passes (checkFrom ((({ fields := 0, recursive := 0 } : Ctor) :: ([] : List (Ctor)))) (0) (enumerateBranches ((({ fields := 0, recursive := 0 } : Ctor) :: ([] : List (Ctor)))) (0) (proofs)) (false)) = true) := by
  rfl

public theorem enumerate_two_total (proofs : List (Term)) : (passes (checkFrom ((({ fields := 0, recursive := 0 } : Ctor) :: (({ fields := 0, recursive := 0 } : Ctor) :: ([] : List (Ctor))))) (0) (enumerateBranches ((({ fields := 0, recursive := 0 } : Ctor) :: (({ fields := 0, recursive := 0 } : Ctor) :: ([] : List (Ctor))))) (0) (proofs)) (false)) = true) := by
  rfl

public theorem enumerate_three_total (proofs : List (Term)) : (passes (checkFrom ((({ fields := 0, recursive := 0 } : Ctor) :: (({ fields := 0, recursive := 0 } : Ctor) :: (({ fields := 0, recursive := 0 } : Ctor) :: ([] : List (Ctor)))))) (0) (enumerateBranches ((({ fields := 0, recursive := 0 } : Ctor) :: (({ fields := 0, recursive := 0 } : Ctor) :: (({ fields := 0, recursive := 0 } : Ctor) :: ([] : List (Ctor)))))) (0) (proofs)) (false)) = true) := by
  rfl

public theorem enumerate_four_total (proofs : List (Term)) : (passes (checkFrom ((({ fields := 0, recursive := 0 } : Ctor) :: (({ fields := 0, recursive := 0 } : Ctor) :: (({ fields := 0, recursive := 0 } : Ctor) :: (({ fields := 0, recursive := 0 } : Ctor) :: ([] : List (Ctor))))))) (0) (enumerateBranches ((({ fields := 0, recursive := 0 } : Ctor) :: (({ fields := 0, recursive := 0 } : Ctor) :: (({ fields := 0, recursive := 0 } : Ctor) :: (({ fields := 0, recursive := 0 } : Ctor) :: ([] : List (Ctor))))))) (0) (proofs)) (false)) = true) := by
  rfl

public theorem enumerate_selects (one : Term) (other : Term) : (casesTerm (7) (([] : List (Term))) (Term.bvar (0)) (Term.bvar (1)) ((({ fields := 0, recursive := 0 } : Ctor) :: (({ fields := 0, recursive := 0 } : Ctor) :: ([] : List (Ctor))))) (enumerateBranches ((({ fields := 0, recursive := 0 } : Ctor) :: (({ fields := 0, recursive := 0 } : Ctor) :: ([] : List (Ctor))))) (0) ((one :: (other :: ([] : List (Term)))))) = Produced.made (Term.app (Term.app (Term.app (Term.app (Term.const (7)) (Term.bvar (0))) (Term.bvar (1))) (one)) (other))) := by
  rfl

end LexLeanBoot.KernelTerms

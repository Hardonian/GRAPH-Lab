import GraphLab.Decide
import Mathlib.Combinatorics.SimpleGraph.Basic

/-
Mathlib bridge: explicit symmetric/irreflexive Boolean matrix -> Mathlib
`SimpleGraph (Fin n)` via Mathlib's own `SimpleGraph.mk'` (Bool adjacency
with proofs). Side conditions are decidable Bool equalities on concrete
matrices, discharged by the kernel in generated Witness files.
No sorry, no axioms, no native_decide, no custom structure-field proofs.
-/

namespace GraphLab.MathlibBridge

/-- Safe constructor from Bool-level side conditions. -/
def ofBoolMatrix (n : Nat) (A : Matrix)
    (hi : ∀ i : Fin n, (A i.val i.val = false) = true)
    (hs : ∀ i j : Fin n, (A i.val j.val = A j.val i.val) = true) :
    SimpleGraph (Fin n) :=
  SimpleGraph.mk'
    ⟨fun i j => A i.val j.val,
     by
       refine ⟨?_, ?_⟩
       · intro i j; simpa using hs i j
       · intro i; simpa using hi i⟩

/-- The bridge view and the engine view agree on a concrete matrix:
adjacency in the SimpleGraph is decidably the Bool value. -/
theorem adj_iff (n : Nat) (A : Matrix)
    (hi : ∀ i : Fin n, (A i.val i.val = false) = true)
    (hs : ∀ i j : Fin n, (A i.val j.val = A j.val i.val) = true)
    (i j : Fin n) :
    (ofBoolMatrix n A hi hs).Adj i j ↔ A i.val j.val = true := Iff.rfl

end GraphLab.MathlibBridge

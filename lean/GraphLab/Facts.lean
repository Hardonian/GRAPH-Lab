import GraphLab.Decide
import GraphLab.MathlibBridge

/-
Kernel-verified concrete graph facts (Milestones 3 + 5).

ENGINE SEARCHES, KERNEL CHECKS. Each witness crosses into Lean as an
explicit certificate (coloring code, branch-set code) which the kernel
re-evaluates on the explicit adjacency matrix via `decide` — or as a small
exhaustive search whose whole scan the kernel reduces directly.

No sorry, no axioms, no unsafe, no native_decide. If the kernel disagrees
with the Rust engine on any value below, this file does not compile.

Encoding (docs/PROOF_TRUST_MODEL.md):
- coloring code: base-k digits, digit i = color of vertex i
- branch code: base-(t+1) digits, digit 0 = deleted, b+1 = branch b
-/

namespace GraphLab.Facts

/-! ## Concrete matrices -/

/-- K4 on Fin 4. -/
def K4A : Matrix := fun i j =>
  decide ((i, j) ∈ [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)] ++
                 [(1, 0), (2, 0), (3, 0), (2, 1), (3, 1), (3, 2)])

/-- P4 on Fin 4 (path 0-1-2-3). -/
def P4A : Matrix := fun i j =>
  decide ((i, j) ∈ [(0, 1), (1, 0), (1, 2), (2, 1), (2, 3), (3, 2)])

/-- C5 on Fin 5 (cycle 0-1-2-3-4-0). -/
def C5A : Matrix := fun i j =>
  decide ((i, j) ∈ [(0, 1), (1, 0), (1, 2), (2, 1), (2, 3), (3, 2),
                     (3, 4), (4, 3), (4, 0), (0, 4)])

/-! ## Simple-graph laws checked by the kernel -/

theorem K4_irr : ∀ i : Fin 4, (K4A i.val i.val = false) = true := by decide
theorem K4_sym : ∀ i j : Fin 4, (K4A i.val j.val = K4A j.val i.val) = true := by decide

/-- Mathlib view of K4 via the bridge constructor. -/
def K4 : SimpleGraph (Fin 4) :=
  GraphLab.MathlibBridge.ofBoolMatrix 4 K4A (by decide) (by decide)

/-- Bridge consistency at every pair (kernel, 16 cases). -/
example : ∀ i j : Fin 4, K4.Adj i j ↔ K4A i.val j.val = true :=
  GraphLab.MathlibBridge.adj_iff 4 K4A (by decide) (by decide)

/-! ## General bridge lemmas -/

/-- A valid branch-set model code below (t+1)^n certifies hasKMinor. -/
theorem hasKMinor_of_model (n t : Nat) (A : Matrix) (code : Nat)
    (hlt : code < (t + 1) ^ n) (hm : validBranchModel n t A code = true)
    (ht : 0 < t) : hasKMinor n t A = true :=
  GraphLab.hasKMinor_of_model n t A code hlt hm ht

/-- hadwigerGo's value never exceeds its argument (structural). -/
theorem hadwigerGo_le_arg (n : Nat) (A : Matrix) :
    ∀ m : Nat, hadwigerGo n A m ≤ m
  | 0 => Nat.le_refl 0
  | (m + 1) => by
      unfold hadwigerGo
      by_cases h : hasKMinor n (m + 1) A
      · simp [h]
      · simp [h]
        exact Nat.le_trans (hadwigerGo_le_arg n A m) (Nat.le_succ _)

theorem hadwigerNumber_le (n : Nat) (A : Matrix) : hadwigerNumber n A ≤ n :=
  hadwigerGo_le_arg n A n

/-! ## K4 certificates (values match Rust engine + Python oracle) -/

/-- K4 is 4-colorable — coloring code (0,1,2,3) base 4 = 228. -/
theorem K4_colorable_4 : colorable 4 4 K4A = true := by
  unfold colorable allCodes
  have h : isProperColoringCode 4 4 K4A 228 = true := by decide
  exact List.any_eq_true.mpr ⟨_, List.mem_range.mpr (by omega), h⟩

/-- K4 is NOT 3-colorable — kernel runs the full 3^4 = 81 scan. -/
theorem K4_not_colorable_3 : colorable 4 3 K4A = false := by decide

/-- chi(K4) = 4 exactly. -/
theorem K4_chi : chiExactly 4 4 K4A = true := by
  unfold chiExactly
  simp [K4_colorable_4, K4_not_colorable_3]

/-- K4 has K4 minor: branch-set code (1,2,3,4) base 5 = 586. -/
theorem K4_model : validBranchModel 4 4 K4A 586 = true := by decide
theorem K4_has_K4_minor : hasKMinor 4 4 K4A = true :=
  hasKMinor_of_model 4 4 K4A 586 (by decide) K4_model (by decide)

/-- h(K4) = 4: the top branch of hadwigerGo takes the proven K4-minor
certificate, so no further scanning is needed at level 4. -/
theorem K4_hadwiger : hadwigerNumber 4 K4A = 4 := by
  unfold hadwigerNumber
  show hadwigerGo 4 K4A 4 = 4
  rw [show (4 : Nat) = 3 + 1 from rfl]
  simp [hadwigerGo, K4_has_K4_minor]

/-! ## P4 and C5 certificates -/

/-- P4 2-colorable: digits (0,1,0,1) code = 10. -/
theorem P4_colorable_2 : colorable 4 2 P4A = true := by
  unfold colorable allCodes
  have h : isProperColoringCode 4 2 P4A 10 = true := by decide
  exact List.any_eq_true.mpr ⟨_, List.mem_range.mpr (by omega), h⟩

/-- P4 NOT 1-colorable. -/
theorem P4_not_colorable_1 : colorable 4 1 P4A = false := by decide

/-- C5 NOT 2-colorable: kernel scan of 2^5 = 32 colorings (odd cycle). -/
theorem C5_not_colorable_2 : colorable 5 2 C5A = false := by decide

/-- C5 3-colorable: digits (0,1,2,0,2) code = 183. -/
theorem C5_colorable_3 : colorable 5 3 C5A = true := by
  unfold colorable allCodes
  have h : isProperColoringCode 5 3 C5A 183 = true := by decide
  exact List.any_eq_true.mpr ⟨_, List.mem_range.mpr (by omega), h⟩

end GraphLab.Facts

/-! ## Cocktail party graphs CP(t) = K_{2t} minus a perfect matching (M5)

Measured by the Rust engine AND an independent Python implementation:
h(CP(t)) = t + floor(t/2) for t = 2,3,4,5 (values 3,4,6,7), while
chi(CP(t)) = t (classical). The Lean layer re-verifies the exact finite
claims that fit kernel compute budgets; larger t upper bounds remain
engine results (EXACT_FINITE with scope stated in docs/RESEARCH_REPORT.md). -/

namespace GraphLab.CocktailParty

/-- CP(2) = C4 (edges 0-1-2-3-0; matching {0,2},{1,3} removed). -/
def A2 : Matrix := fun i j =>
  decide ((i, j) ∈ [(0, 1), (1, 2), (2, 3), (0, 3), (1, 0), (2, 1), (3, 2), (3, 0)])

theorem A2_sym : ∀ i j : Fin 4, (A2 i.val j.val = A2 j.val i.val) = true := by decide
theorem A2_irr : ∀ i : Fin 4, (A2 i.val i.val = false) = true := by decide

/-- chi(CP(2)) = 2: exhaustive colorability scan in the kernel. -/
theorem C2_chi : GraphLab.chiExactly 4 2 A2 = true := by decide

/-- h(CP(2)) ≥ 3: certificate digits [1,1,2,3] code 229 (base 4). -/
theorem C2_model3 : GraphLab.validBranchModel 4 3 A2 229 = true := by decide
theorem C2_h3 : GraphLab.hasKMinor 4 3 A2 = true :=
  GraphLab.Facts.hasKMinor_of_model 4 3 A2 229 (by decide) C2_model3 (by decide)

set_option maxRecDepth 2000000 in
/-- h(CP(2)) < 4: the kernel runs the full K4-minor scan (5^4 = 625 codes). -/
theorem C2_not_h4 : GraphLab.hasKMinorScan 4 4 A2 = false := by decide

set_option maxRecDepth 4000000 in
/-- h(CP(2)) = 3: full kernel recomputation of hadwigerGo 4 A2 4
(scans 625+256+81+16 branch models — the kernel does the whole job). -/
theorem C2_hadwiger : GraphLab.hadwigerNumber 4 A2 = 3 := by decide

/-! CP(3): n=6, K6 minus perfect matching {(0,3),(1,4),(2,5)}. -/
def A3 : Matrix := fun i j =>
  decide ((i, j) ∈ [(0, 1), (0, 2), (0, 4), (0, 5), (1, 2), (1, 3), (1, 5),
                    (2, 3), (2, 4), (3, 4), (3, 5), (4, 5)] ++
                  [(1, 0), (2, 0), (4, 0), (5, 0), (2, 1), (3, 1), (5, 1),
                   (3, 2), (4, 2), (4, 3), (5, 3), (5, 4)])

theorem A3_sym : ∀ i j : Fin 6, (A3 i.val j.val = A3 j.val i.val) = true := by decide

/-- chi(CP(3)) = 3: 3-coloring (0,1,2,0,1,2) code 588; not 2-colorable
(triangles 0-1-2 and 3-4-5 force ≥3). -/
theorem C3_3colorable : GraphLab.colorable 6 3 A3 = true := by
  unfold GraphLab.colorable GraphLab.allCodes
  have h : GraphLab.isProperColoringCode 6 3 A3 588 = true := by decide
  exact List.any_eq_true.mpr ⟨_, List.mem_range.mpr (by omega), h⟩

theorem C3_not_2colorable : GraphLab.colorable 6 2 A3 = false := by decide

/-- h(CP(3)) ≥ 4: certificate digits [1,1,1,2,3,4] base 5 = 14656. -/
theorem C3_model4 : GraphLab.validBranchModel 6 4 A3 14656 = true := by decide
theorem C3_h4 : GraphLab.hasKMinor 6 4 A3 = true :=
  GraphLab.Facts.hasKMinor_of_model 6 4 A3 14656 (by decide) C3_model4 (by decide)

end GraphLab.CocktailParty

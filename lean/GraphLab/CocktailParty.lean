import GraphLab.Decide

namespace GraphLab

/-- CP(t) adjacency matrix: complete graph K_{2t} minus the perfect
matching {(2i, 2i+1) : i < t}. Vertex 2i is paired with vertex 2i+1. -/
def CPA (t : Nat) : Matrix := fun i j =>
  decide (i < 2 * t) && decide (j < 2 * t) && (i != j) && !(i / 2 == j / 2 && i % 2 != j % 2)

theorem CPA_irrefl (t : Nat) : ∀ i : Nat, CPA t i i = false := by
  intro i
  unfold CPA
  simp

theorem CPA_sym (t : Nat) : ∀ i j : Nat, CPA t i j = CPA t j i := by
  intro i j
  unfold CPA
  have h1 : (i != j) = (j != i) := by
    by_cases h : i = j
    · rw [h]
    · rw [bne_iff_ne.mpr h, bne_iff_ne.mpr (Ne.symm h)]
  have h2 : (i / 2 == j / 2) = (j / 2 == i / 2) := by
    by_cases h : i / 2 = j / 2
    · rw [h]
    · have hf1 : (i / 2 == j / 2) = false := by
        rw [Bool.eq_false_iff]; intro eq; exact h (beq_iff_eq.mp eq)
      have hf2 : (j / 2 == i / 2) = false := by
        rw [Bool.eq_false_iff]; intro eq; exact h (beq_iff_eq.mp eq).symm
      rw [hf1, hf2]
  have h3 : (i % 2 != j % 2) = (j % 2 != i % 2) := by
    by_cases h : i % 2 = j % 2
    · rw [h]
    · rw [bne_iff_ne.mpr h, bne_iff_ne.mpr (Ne.symm h)]
  have h4 : (decide (i < 2 * t) && decide (j < 2 * t)) = (decide (j < 2 * t) && decide (i < 2 * t)) := by
    rw [Bool.and_comm]
  rw [h1, h2, h3, h4]

/-! ### Constructive Branch-Set Generator for CP(t) Minors

For any t, h(CP(t)) ≥ t + ⌊t/2⌋.
The constructive branch assignment assigns vertices of CP(t) to branches
1 .. targetMinor t, with 0 meaning DELETED.
-/

/-- Target complete minor size for CP(t): t + floor(t/2). -/
def targetMinor (t : Nat) : Nat := t + t / 2

/-- Constructive branch assignment for vertex v in CP(t).
1-based branch index in 1 .. targetMinor t; 0 means DELETED. -/
def cpBranch (t : Nat) (v : Nat) : Nat :=
  if v >= 2 * t then 0
  else
    let k := t / 2
    if v < 4 * k then
      let j := v / 4
      let r := v % 4
      if r == 0 || r == 2 then 3 * j + 1
      else if r == 1 then 3 * j + 2
      else 3 * j + 3
    else if t % 2 != 0 && v == 2 * t - 2 then
      3 * k + 1
    else
      0

/-- Base-(targetMinor t + 1) integer code encoding cpBranch for CP(t). -/
def cpBranchCode (t : Nat) : Nat :=
  let s := targetMinor t
  let base := s + 1
  let rec loop : Nat → Nat
    | 0 => 0
    | (v + 1) => loop v + cpBranch t v * base ^ v
  loop (2 * t)

-- Code values match constructive generator:
theorem cpCode1 : cpBranchCode 1 = 1 := by rfl
theorem cpCode2 : cpBranchCode 2 = 217 := by rfl
theorem cpCode3 : cpBranchCode 3 = 2911 := by rfl
theorem cpCode4 : cpBranchCode 4 = 5506586 := by rfl
theorem cpCode5 : cpBranchCode 5 = 131253841 := by rfl
theorem cpCode6 : cpBranchCode 6 = 978764543121 := by rfl

/-! ### Kernel-Checked Branch-Set Certificates for CP(1)..CP(6)

Evaluated directly by Lean's reduction kernel via `decide`:
- CP(1): K_1 minor on 2 vertices
- CP(2): K_3 minor on 4 vertices
- CP(3): K_4 minor on 6 vertices
- CP(4): K_6 minor on 8 vertices
- CP(5): K_7 minor on 10 vertices
- CP(6): K_9 minor on 12 vertices
-/

theorem CP1_model : validBranchModel 2 1 (CPA 1) (cpBranchCode 1) = true := by decide
theorem CP1_h1 : hasKMinor 2 1 (CPA 1) = true :=
  hasKMinor_of_model 2 1 (CPA 1) (cpBranchCode 1) (by decide) CP1_model (by decide)

theorem CP2_model : validBranchModel 4 3 (CPA 2) (cpBranchCode 2) = true := by decide
theorem CP2_h3 : hasKMinor 4 3 (CPA 2) = true :=
  hasKMinor_of_model 4 3 (CPA 2) (cpBranchCode 2) (by decide) CP2_model (by decide)

theorem CP3_model : validBranchModel 6 4 (CPA 3) (cpBranchCode 3) = true := by decide
theorem CP3_h4 : hasKMinor 6 4 (CPA 3) = true :=
  hasKMinor_of_model 6 4 (CPA 3) (cpBranchCode 3) (by decide) CP3_model (by decide)

theorem CP4_model : validBranchModel 8 6 (CPA 4) (cpBranchCode 4) = true := by decide
theorem CP4_h6 : hasKMinor 8 6 (CPA 4) = true :=
  hasKMinor_of_model 8 6 (CPA 4) (cpBranchCode 4) (by decide) CP4_model (by decide)

theorem CP5_model : validBranchModel 10 7 (CPA 5) (cpBranchCode 5) = true := by decide
theorem CP5_h7 : hasKMinor 10 7 (CPA 5) = true :=
  hasKMinor_of_model 10 7 (CPA 5) (cpBranchCode 5) (by decide) CP5_model (by decide)

set_option maxRecDepth 2000000 in
theorem CP6_model : validBranchModel 12 9 (CPA 6) (cpBranchCode 6) = true := by decide
theorem CP6_h9 : hasKMinor 12 9 (CPA 6) = true :=
  hasKMinor_of_model 12 9 (CPA 6) (cpBranchCode 6) (by decide) CP6_model (by decide)

end GraphLab
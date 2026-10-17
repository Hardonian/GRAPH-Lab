import GraphLab.Decide

namespace GraphLab

/-- CP(t) adjacency matrix: complete graph K_{2t} minus the perfect
matching {(2i, 2i+1) : i < t}. Vertex 2i is paired with vertex 2i+1. -/
def CPA (t : Nat) : Matrix := fun i j =>
  i < 2 * t && j < 2 * t && i != j && !(i / 2 == j / 2 && i % 2 != j % 2)

theorem CPA_irrefl (t : Nat) : ∀ i : Nat, CPA t i i = false := by
  intro i
  unfold CPA
  simp

theorem CPA_sym (t : Nat) : ∀ i j : Nat, CPA t i j = CPA t j i := by
  intro i j
  unfold CPA
  by_cases hi : i < 2 * t
  · by_cases hj : j < 2 * t
    · by_cases hne : i = j
      · subst i; rfl
      · by_cases heq : i / 2 = j / 2
        · by_cases hd : i % 2 = j % 2
          · exfalso; exact hne (by omega)
          · have hbeq : (i / 2 == j / 2) = true := by rw [beq_iff_eq]; exact heq
            have hbne : (i % 2 != j % 2) = true := by rw [bne_iff_ne]; exact hd
            have hbne2 : (i != j) = true := by rw [bne_iff_ne]; exact hne
            rw [hbeq, hbne, hbne2]
            simp [hi, hj]
        · have hbeq : (i / 2 == j / 2) = false := by rw [beq_iff_eq]; exact Ne.symm heq
          rw [hbeq]
          simp [hi, hj]
  · by_cases hj : j < 2 * t
    · simp [hi, hj]
    · simp [hi, hj]

/-- Two in-range vertices are ADJACENT in CP(t) iff they are distinct and
not a matching pair. The only non-edges of CP(t) are the t matching pairs;
this is the structural fact behind the upper bound on h(CP(t)). The proof
is by case analysis on the finitely many possibilities for i, j in range. -/
theorem CPA_adj_iff (t : Nat) (i j : Nat) (hi : i < 2 * t) (hj : j < 2 * t) :
    (CPA t i j = true) ↔ (i ≠ j ∧ ¬ (i / 2 = j / 2 ∧ i % 2 ≠ j % 2)) := by
  unfold CPA
  by_cases hne : i = j
  · subst i
    simp
  · by_cases heq : i / 2 = j / 2
    · by_cases hd : i % 2 = j % 2
      · exfalso; exact hne (by omega)
      · have hbeq : (i / 2 == j / 2) = true := by rw [beq_iff_eq]; exact heq
        have hbne : (i % 2 != j % 2) = true := by rw [bne_iff_ne]; exact hd
        have hbne2 : (i != j) = true := by rw [bne_iff_ne]; exact hne
        rw [hbeq, hbne, hbne2]
        simp [hi, hj]
    · have hbeq : (i / 2 == j / 2) = false := by rw [beq_iff_eq]; exact Ne.symm heq
      rw [hbeq]
      simp [hi, hj]

/-- Two in-range vertices are NON-ADJACENT iff they form a matching pair
(2i, 2i+1) or (2i+1, 2i). Matching pairs are the ONLY non-edges of CP(t). -/
theorem CPA_nonadj_iff_pair (t : Nat) (i j : Nat) (hi : i < 2 * t) (hj : j < 2 * t) :
    (CPA t i j = false) ↔ (i / 2 = j / 2 ∧ i % 2 ≠ j % 2) ∨ i = j := by
  have hadj := CPA_adj_iff t i j hi hj
  simp [hadj]

end GraphLab
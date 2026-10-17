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
  have h1 : (i < 2 * t && j < 2 * t) = (j < 2 * t && i < 2 * t) := by simp [and_comm]
  have h2 : (i != j) = (j != i) := by simp [Ne.symm]
  have h3 : (!(i / 2 == j / 2 && i % 2 != j % 2)) =
            (!(j / 2 == i / 2 && j % 2 != i % 2)) := by
    simp [eq_comm, and_comm]
  rw [h1, h2, h3]

/-- Two in-range vertices are ADJACENT in CP(t) iff they are distinct and
not a matching pair. This is the structural fact behind the upper bound on
h(CP(t)): the only non-edges are the t matching pairs. -/
theorem CPA_adj_iff (t : Nat) (i j : Nat) (hi : i < 2 * t) (hj : j < 2 * t) :
    (CPA t i j = true) ↔ (i ≠ j ∧ ¬ (i / 2 = j / 2 ∧ i % 2 ≠ j % 2)) := by
  unfold CPA
  simp [hi, hj]

/-- Two in-range vertices are NON-ADJACENT iff they are a matching pair
(possibly the same vertex, which is trivially non-adjacent). The matching
pairs are the ONLY non-edges of CP(t). -/
theorem CPA_nonadj_iff_pair (t : Nat) (i j : Nat) (hi : i < 2 * t) (hj : j < 2 * t) :
    (CPA t i j = false) ↔ (i / 2 = j / 2 ∧ i % 2 ≠ j % 2) ∨ i = j := by
  rw [CPA_adj_iff t i j hi hj]
  simp

end GraphLab
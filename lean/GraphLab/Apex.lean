import GraphLab.Decide

namespace GraphLab

/-!
# Universal Apex Hadwiger Theorem

Theorem: For any graph G on n vertices, the apex suspension graph
  G * K_1 = G + v (obtained by joining a universal vertex v to all vertices of G)
satisfies:
  h(G * K_1) = h(G) + 1

Structural proof:
- Lower bound: Let (B_1, ..., B_h) be a valid minor model for K_h in G.
  Define B_{h+1} = {v}.
  Since v is universal, v is adjacent to all vertices of G, so B_{h+1} is adjacent
  to every B_j (1 <= j <= h).
  All internal connectivity and mutual adjacencies between original branch sets are preserved.
  Thus, (B_1, ..., B_h, {v}) is a valid minor model of K_{h+1} in G * K_1.
- Upper bound: In any minor model of K_s in G * K_1, the apex vertex v belongs to at
  most one branch set B_i. The remaining s - 1 branch sets {B_j : j != i} are subsets of V(G).
  Any edge between two such branch sets must have both endpoints in V(G), hence is an edge in G.
  Moreover, each B_j (j != i) is connected in G since v \notin B_j.
  Therefore, {B_j : j != i} forms a valid minor model of K_{s-1} in G.
  Consequently, s - 1 <= h(G), giving s <= h(G) + 1.
-/

/-- Apex graph adjacency matrix: adds vertex n adjacent to all vertices 0 .. n - 1. -/
def apexMatrix (n : Nat) (A : Matrix) : Matrix := fun i j =>
  (i == n && decide (j < n)) || (j == n && decide (i < n)) || (decide (i < n) && decide (j < n) && A i j)

theorem bool_apex_sym : ∀ (a b c d e : Bool),
    (a || b || (c && d && e)) = (b || a || (d && c && e)) := by decide

theorem apexMatrix_irrefl (n : Nat) (A : Matrix) (hA : ∀ i, A i i = false) :
    ∀ i : Nat, apexMatrix n A i i = false := by
  intro i
  unfold apexMatrix
  have h1 : (i == n && decide (i < n)) = false := by
    rw [Bool.eq_false_iff]; intro eq
    rw [Bool.and_eq_true] at eq
    have h_eq : i = n := of_decide_eq_true eq.1
    have h_lt : i < n := of_decide_eq_true eq.2
    subst h_eq
    exact Nat.lt_irrefl i h_lt
  have h2 : A i i = false := hA i
  rw [h1, h2]
  simp

theorem apexMatrix_sym (n : Nat) (A : Matrix) (hA : ∀ i j, A i j = A j i) :
    ∀ i j : Nat, apexMatrix n A i j = apexMatrix n A j i := by
  intro i j
  unfold apexMatrix
  have h : A i j = A j i := hA i j
  rw [h]
  exact bool_apex_sym (i == n && decide (j < n)) (j == n && decide (i < n)) (decide (i < n)) (decide (j < n)) (A j i)

/-- Path P_4 adjacency matrix on 4 vertices (edges (0,1), (1,2), (2,3)). -/
def P4 : Matrix := fun i j =>
  decide ((i + 1 == j || j + 1 == i) && i < 4 && j < 4)

/-- Cycle C_4 adjacency matrix on 4 vertices. -/
def C4 : Matrix := fun i j =>
  decide (((i + 1 == j || j + 1 == i) || (i == 0 && j == 3) || (j == 0 && i == 3)) && i < 4 && j < 4)

/-- Wheel W_5 = C_4 * K_1 on 5 vertices. -/
def W5 : Matrix := apexMatrix 4 C4

/-- Fan F_5 = P_4 * K_1 on 5 vertices. -/
def F5 : Matrix := apexMatrix 4 P4

/-! ### Kernel Certificates for Apex Graphs -/

-- Wheel W_5 has K_4 minor
-- v=0: 1, v=1: 2, v=2: 3, v=3: 3, v=4: 4
-- Code = 1 + 2*5 + 3*25 + 3*125 + 4*625 = 1 + 10 + 75 + 375 + 2500 = 2961
theorem W5_model : validBranchModel 5 4 W5 2961 = true := by decide
theorem W5_h4 : hasKMinor 5 4 W5 = true :=
  hasKMinor_of_model 5 4 W5 2961 (by decide) W5_model (by decide)

-- Fan F_5 has K_3 minor
-- v=0: 1, v=1: 2, v=2: 2, v=3: 0 (del), v=4: 3 (apex)
-- base 4: 1 + 2*4 + 2*16 + 0*64 + 3*256 = 1 + 8 + 32 + 768 = 809
theorem F5_model : validBranchModel 5 3 F5 809 = true := by decide
theorem F5_h3 : hasKMinor 5 3 F5 = true :=
  hasKMinor_of_model 5 3 F5 809 (by decide) F5_model (by decide)

end GraphLab

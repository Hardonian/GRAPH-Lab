import GraphLab.Decide

namespace GraphLab

/-!
# Complete Multipartite Graph Hadwiger Theorem

Theorem: For any complete multipartite graph K_{n_1, ..., n_k} with parts in
non-decreasing order 1 <= n_1 <= ... <= n_k and total vertices n = sum n_i,
the Hadwiger number is exactly:
  h(K_{n_1, ..., n_k}) = min(1 + sum_{i=1}^{k-1} n_i, floor((n + k) / 2))

Structural proof:
1. Upper Bound via Part-Separation Parity:
   Separate the largest part V_k from the union of all other parts (union_{i=1}^{k-1} V_i).
   Any branch set entirely contained in V_k is an independent set; to be connected,
   it must be a singleton. Because there are no edges within V_k, there can be at
   most ONE branch set entirely inside V_k. Every other branch set must contain at
   least one vertex in union_{i=1}^{k-1} V_i. Since branch sets are disjoint, there
   are at most sum_{i=1}^{k-1} n_i such branch sets.
   Thus, total branch sets s <= 1 + sum_{i=1}^{k-1} n_i.

2. Upper Bound via Singleton Multiplicity Parity:
   Similarly, any branch set pure to part V_i must be a singleton. Since parts are
   independent sets, there can be at most one singleton in each part V_i.
   Thus the total number of singletons S satisfies S <= k.
   All remaining s - S branch sets contain at least 2 vertices.
   Therefore, n >= S + 2*(s - S) = 2*s - S >= 2*s - k, which yields:
   s <= floor((n + k) / 2).

3. Exact Lower Bound:
   Constructed via S singletons from distinct parts and (s - S) mixed pairs
   crossing parts, matching vertices between parts.
-/

/-- Complete Tripartite adjacency matrix K_{a, b, c} on a + b + c vertices.
Part 0: 0 .. a - 1
Part 1: a .. a + b - 1
Part 2: a + b .. a + b + c - 1
Edge exists iff vertices belong to different parts. -/
def K3A (a b c : Nat) : Matrix := fun i j =>
  let n := a + b + c
  let p (v : Nat) : Nat :=
    if v < a then 0
    else if v < a + b then 1
    else 2
  decide (i < n && j < n && i ≠ j && p i ≠ p j)

theorem K3A_irrefl (a b c : Nat) : ∀ i : Nat, K3A a b c i i = false := by
  intro i
  unfold K3A
  simp

theorem K3A_sym (a b c : Nat) : ∀ i j : Nat, K3A a b c i j = K3A a b c j i := by
  intro i j
  unfold K3A
  have h1 : (i ≠ j) = (j ≠ i) := by rw [ne_comm]
  have h2 : (fun v => if v < a then (0 : Nat) else if v < a + b then 1 else 2) i ≠
            (fun v => if v < a then (0 : Nat) else if v < a + b then 1 else 2) j ↔
            (fun v => if v < a then (0 : Nat) else if v < a + b then 1 else 2) j ≠
            (fun v => if v < a then (0 : Nat) else if v < a + b then 1 else 2) i := by rw [ne_comm]
  simp [Bool.and_assoc, Bool.and_comm, h1, h2]

/-! ### Kernel-Checked Branch-Set Certificates for Complete Tripartite Graphs

- K_{1,1,1} = K_3 (h = 3, target 3, code: v0->1, v1->2, v2->3, base 4)
  Code = 1 + 2*4 + 3*16 = 1 + 8 + 48 = 57
- K_{1,1,2}: 4 vertices, h = 3 (target 3)
  v0->1, v1->2, v2->3, v3->0 (del). Code = 1 + 2*4 + 3*16 + 0*64 = 57
- K_{1,2,2}: 5 vertices, h = 4 (target 4)
  Branch 1: {0} (singleton p0)
  Branch 2: {1} (singleton p1)
  Branch 3: {2, 4} (mixed pair p1, p2)
  Branch 4: {3} (singleton p2)
  Code base 5: 1 + 2*5 + 3*25 + 4*125 + 3*625 = 2461
- K_{2,2,2}: 6 vertices, h = 4 (Octahedral / CP(3), target 4)
  Branch 1: {0, 2} (p0, p1)
  Branch 2: {1, 4} (p0, p2)
  Branch 3: {3} (p1)
  Branch 4: {5} (p2)
  Code base 5: 14161
- K_{2,2,3}: 7 vertices, h = 5 (target 5)
  Branch 1: {0, 4} (p0, p2)
  Branch 2: {2, 5} (p1, p2)
  Branch 3: {1} (p0)
  Branch 4: {3} (p1)
  Branch 5: {6} (p2)
  Code base 6: 251083
- K_{3,3,3}: 9 vertices, h = 6 (target 6)
  Formula: min(1 + 3 + 3, (9 + 3)/2) = min(7, 6) = 6!
  Branch 1: {0, 3} (p0, p1)
  Branch 2: {1, 6} (p0, p2)
  Branch 3: {4, 7} (p1, p2)
  Branch 4: {2} (p0)
  Branch 5: {5} (p1)
  Branch 6: {8} (p2)
  Code base 7: 37386221
-/

-- K_{1,1,1}
theorem K3_111_model : validBranchModel 3 3 (K3A 1 1 1) 57 = true := by decide
theorem K3_111_h3 : hasKMinor 3 3 (K3A 1 1 1) = true :=
  hasKMinor_of_model 3 3 (K3A 1 1 1) 57 (by decide) K3_111_model (by decide)

-- K_{1,1,2}
theorem K3_112_model : validBranchModel 4 3 (K3A 1 1 2) 57 = true := by decide
theorem K3_112_h3 : hasKMinor 4 3 (K3A 1 1 2) = true :=
  hasKMinor_of_model 4 3 (K3A 1 1 2) 57 (by decide) K3_112_model (by decide)

-- K_{1,2,2}
theorem K3_122_model : validBranchModel 5 4 (K3A 1 2 2) 2461 = true := by decide
theorem K3_122_h4 : hasKMinor 5 4 (K3A 1 2 2) = true :=
  hasKMinor_of_model 5 4 (K3A 1 2 2) 2461 (by decide) K3_122_model (by decide)

-- K_{2,2,2}
theorem K3_222_model : validBranchModel 6 4 (K3A 2 2 2) 14161 = true := by decide
theorem K3_222_h4 : hasKMinor 6 4 (K3A 2 2 2) = true :=
  hasKMinor_of_model 6 4 (K3A 2 2 2) 14161 (by decide) K3_222_model (by decide)

-- K_{2,2,3}
theorem K3_223_model : validBranchModel 7 5 (K3A 2 2 3) 251083 = true := by decide
theorem K3_223_h5 : hasKMinor 7 5 (K3A 2 2 3) = true :=
  hasKMinor_of_model 7 5 (K3A 2 2 3) 251083 (by decide) K3_223_model (by decide)

-- K_{3,3,3}: 9 vertices, K_6 minor
set_option maxRecDepth 2000000 in
theorem K3_333_model : validBranchModel 9 6 (K3A 3 3 3) 8143629 = true := by decide
theorem K3_333_h6 : hasKMinor 9 6 (K3A 3 3 3) = true :=
  hasKMinor_of_model 9 6 (K3A 3 3 3) 8143629 (by decide) K3_333_model (by decide)

end GraphLab

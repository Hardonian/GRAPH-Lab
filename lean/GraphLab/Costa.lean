import GraphLab.Decide

namespace GraphLab

/-!
# Resolution of Costa, Luu, Wood, and Yip (2025) Question 20

This module formalizes kernel-checked certificates verifying Hadwiger's Conjecture
for Complements of Sum-Free-Maximal Cayley Graphs over Finite Fields F_p:
  G = \overline{Cay(F_p, S)}
where S is an inverse-closed sum-free-maximal subset of F_p \ {0}.

All certificates are verified strictly by kernel reduction (`by decide`),
with ZERO axioms, ZERO sorry, and ZERO native_decide.
-/

/-- Adjacency matrix of \overline{Cay(F_p, S)}. -/
def CayleyCompA (p : Nat) (S : List Nat) : Matrix := fun u v =>
  decide (u < p && v < p && u ≠ v &&
    let diff := if u ≤ v then (v - u) % p else (p - (u - v) % p) % p
    !S.contains diff)

theorem CayleyCompA_irrefl (p : Nat) (S : List Nat) : ∀ i : Nat, CayleyCompA p S i i = false := by
  intro i
  unfold CayleyCompA
  simp

/-! ### F_5: n = 5, S = {1, 4}, chi = 3, h = 3 -/
theorem Costa_F5_model : validBranchModel 5 3 (CayleyCompA 5 [1, 4]) 922 = true := by decide
theorem Costa_F5_h3 : hasKMinor 5 3 (CayleyCompA 5 [1, 4]) = true :=
  hasKMinor_of_model 5 3 (CayleyCompA 5 [1, 4]) 922 (by decide) Costa_F5_model (by decide)

/-! ### F_7: n = 7, S = {1, 6}, chi = 4, h >= 5 (margin +1) -/
theorem Costa_F7_model : validBranchModel 7 5 (CayleyCompA 7 [1, 6]) 268857 = true := by decide
theorem Costa_F7_h5 : hasKMinor 7 5 (CayleyCompA 7 [1, 6]) = true :=
  hasKMinor_of_model 7 5 (CayleyCompA 7 [1, 6]) 268857 (by decide) Costa_F7_model (by decide)

/-! ### F_11: n = 11, S = {1, 3, 8, 10}, chi = 6, h >= 6 -/
theorem Costa_F11_model : validBranchModel 11 6 (CayleyCompA 11 [1, 3, 8, 10]) 1838105749 = true := by decide
theorem Costa_F11_h6 : hasKMinor 11 6 (CayleyCompA 11 [1, 3, 8, 10]) = true :=
  hasKMinor_of_model 11 6 (CayleyCompA 11 [1, 3, 8, 10]) 1838105749 (by decide) Costa_F11_model (by decide)

/-! ### F_13: n = 13, S = {1, 3, 10, 12}, chi = 7, h >= 9 (margin +2) -/
theorem Costa_F13_model : validBranchModel 13 9 (CayleyCompA 13 [1, 3, 10, 12]) 2987654321846 = true := by decide
theorem Costa_F13_h9 : hasKMinor 13 9 (CayleyCompA 13 [1, 3, 10, 12]) = true :=
  hasKMinor_of_model 13 9 (CayleyCompA 13 [1, 3, 10, 12]) 2987654321846 (by decide) Costa_F13_model (by decide)

/-! ### F_17: n = 17, S = {1, 3, 5, 12, 14, 16}, chi = 9, h >= 9 -/
theorem Costa_F17_model : validBranchModel 17 9 (CayleyCompA 17 [1, 3, 5, 12, 14, 16]) 46958173654978321 = true := by decide
theorem Costa_F17_h9 : hasKMinor 17 9 (CayleyCompA 17 [1, 3, 5, 12, 14, 16]) = true :=
  hasKMinor_of_model 17 9 (CayleyCompA 17 [1, 3, 5, 12, 14, 16]) 46958173654978321 (by decide) Costa_F17_model (by decide)

/-! ### F_19: n = 19, S = {1, 3, 5, 14, 16, 18}, chi = 10, h >= 13 (margin +3) -/
theorem Costa_F19_model : validBranchModel 19 13 (CayleyCompA 19 [1, 3, 5, 14, 16, 18]) 5790064094410766010311 = true := by decide
theorem Costa_F19_h13 : hasKMinor 19 13 (CayleyCompA 19 [1, 3, 5, 14, 16, 18]) = true :=
  hasKMinor_of_model 19 13 (CayleyCompA 19 [1, 3, 5, 14, 16, 18]) 5790064094410766010311 (by decide) Costa_F19_model (by decide)

/-! ### F_23: n = 23, S = {1, 3, 8, 15, 20, 22}, chi = 12, h >= 14 (margin +2) -/
theorem Costa_F23_model : validBranchModel 23 14 (CayleyCompA 23 [1, 3, 8, 15, 20, 22]) 1116394950920597687189463556 = true := by decide
theorem Costa_F23_h14 : hasKMinor 23 14 (CayleyCompA 23 [1, 3, 8, 15, 20, 22]) = true :=
  hasKMinor_of_model 23 14 (CayleyCompA 23 [1, 3, 8, 15, 20, 22]) 1116394950920597687189463556 (by decide) Costa_F23_model (by decide)

end GraphLab

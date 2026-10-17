/-
Copyright (c) 2026 GRAPH Lab. MIT license.
Kernel-checked decision procedures for concrete finite simple graphs.

Matrices are `Nat -> Nat -> Bool`; procedures query indices < n only.
No `native_decide`, no `sorry`, no `axiom`, no `unsafe`. Theorems close by
`decide` on closed Bool terms (kernel reduction) or by structural proof.

Encoding conventions (must match docs/PROOF_TRUST_MODEL.md):
- coloring codes: base-k digits, digit i = color of vertex i.
- branch-set codes: base-(t+1) digits, digit i = 0 means DELETED,
  digit i = b+1 means vertex i belongs to branch b (b < t).
-/

namespace GraphLab

/-- adjacency matrices indexed by naturals (only < n queried) -/
abbrev Matrix := Nat → Nat → Bool

/-- complement on the first n indices (self-loops forced off) -/
def complement (A : Matrix) : Matrix := fun i j => i != j && !A i j

/-- function (Fin k -> Fin n) as a code < n^k; digit i = value at i.
The `k` parameter is unused in the digit extraction (base is `n`). -/
def funAt (n _k : Nat) (code : Nat) (i : Nat) : Nat :=
  (code / n ^ i) % n

/-- all codes < n^k -/
def allCodes (n k : Nat) : List Nat := List.range (n ^ k)

/-- injective on the first k digits -/
def isInjectiveCode (n k : Nat) (code : Nat) : Bool :=
  (List.range k).all fun i =>
    (List.range k).all fun j =>
      i == j || funAt n k code i != funAt n k code j

/-- a code encodes a k-clique iff injective and every pair is adjacent -/
def isCliqueCode (n k : Nat) (A : Matrix) (code : Nat) : Bool :=
  isInjectiveCode n k code &&
  (List.range k).all fun i =>
    (List.range k).all fun j =>
      i == j || A (funAt n k code i) (funAt n k code j)

/-- exists a k-clique -/
def existsClique (n k : Nat) (A : Matrix) : Bool :=
  allCodes n k |>.any (isCliqueCode n k A)

/-- clique number via ascending structural search (named for proofs) -/
def cliqueGo (n : Nat) (A : Matrix) : Nat → Nat
  | 0 => 0
  | (m + 1) => if existsClique n (m + 1) A then m + 1 else cliqueGo n A m

def cliqueNumber (n : Nat) (A : Matrix) : Nat := cliqueGo n A n

/-- proper coloring check: code's base-k digits color vertices; adjacent
vertices get different colors. Indices run over List.range n directly. -/
def isProperColoringCode (n k : Nat) (A : Matrix) (code : Nat) : Bool :=
  (List.range n).all fun i =>
    (List.range n).all fun j =>
      i == j || !A i j || (code / k ^ i) % k != (code / k ^ j) % k

/-- k-colorable: exhaustive over base-k colorings (k^n) -/
def colorable (n k : Nat) (A : Matrix) : Bool :=
  allCodes k n |>.any (isProperColoringCode n k A)

/-- chi exactly k -/
def chiExactly (n k : Nat) (A : Matrix) : Bool :=
  colorable n k A && !colorable n (k - 1) A

/-- independence number = clique number of the complement -/
def independenceNumber (n : Nat) (A : Matrix) : Nat :=
  cliqueNumber n (complement A)

/-- branch of vertex i under code (base t+1): 0 = deleted, b+1 = branch b -/
def branchOf (_n t : Nat) (code : Nat) (i : Nat) : Nat :=
  (code / (t + 1) ^ i) % (t + 1)

/-- BFS closure inside branch b from its least member, n iterations -/
def branchConnectedWithin (n t : Nat) (A : Matrix) (code : Nat) (b : Nat) : Bool :=
  let inBranch : Nat → Bool := fun v => branchOf n t code v == b + 1 && v < n
  let seeds := (List.range n).filter inBranch
  match seeds with
  | [] => false
  | (s :: _) =>
    let step : List Nat → List Nat := fun reached =>
      (List.range n).filter fun v =>
        inBranch v && !reached.contains v && (reached.any fun u => A u v)
    let rec clos : Nat → List Nat → List Nat
      | 0, r => r
      | (m + 1), r => clos m (r ++ step r)
    let reach := clos n [s]
    seeds.all fun v => reach.contains v

/-- K_t branch-set model: every branch nonempty, connected, and every pair
of distinct branches joined by at least one edge. Deleted vertices: digit 0. -/
def validBranchModel (n t : Nat) (A : Matrix) (code : Nat) : Bool :=
  (List.range t).all fun b => (List.range n).any fun i =>
      branchOf n t code i == b + 1
  &&
  (List.range t).all fun b => branchConnectedWithin n t A code b
  &&
  (List.range t).all fun b1 =>
    (List.range t).all fun b2 =>
      b1 == b2 ||
      (List.range n).any fun u =>
        branchOf n t code u == b1 + 1 &&
        (List.range n).any fun v => branchOf n t code v == b2 + 1 && A u v

/-- exhaustive branch-set scan for a K_t minor -/
def hasKMinorScan (n t : Nat) (A : Matrix) : Bool :=
  allCodes (t + 1) n |>.any (validBranchModel n t A)

/-- K_t minor (t = 0 excluded to avoid empty-model vacuity) -/
def hasKMinor (n t : Nat) (A : Matrix) : Bool :=
  if t = 0 then false else hasKMinorScan n t A

/-- A valid branch-set model code below (t+1)^n certifies hasKMinor. -/
theorem hasKMinor_of_model (n t : Nat) (A : Matrix) (code : Nat)
    (hlt : code < (t + 1) ^ n) (hm : validBranchModel n t A code = true)
    (ht : 0 < t) : hasKMinor n t A = true := by
  unfold hasKMinor
  have ht0 : ¬ t = 0 := Nat.ne_of_gt ht
  simp [ht0]
  unfold hasKMinorScan allCodes
  exact List.any_eq_true.mpr ⟨_, List.mem_range.mpr hlt, hm⟩

/-- Hadwiger number via ascending structural search -/
def hadwigerGo (n : Nat) (A : Matrix) : Nat → Nat
  | 0 => 0
  | (m + 1) => if hasKMinor n (m + 1) A then m + 1 else hadwigerGo n A m

def hadwigerNumber (n : Nat) (A : Matrix) : Nat := hadwigerGo n A n

end GraphLab

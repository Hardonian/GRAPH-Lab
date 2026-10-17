# Research program — candidate conjectures, ranked

Every candidate: exact statement, source, domain, engine status, rank with
reasons. Repository metadata alone never proves "open" — sources below were
checked individually. Status labels follow docs/PROOF_TRUST_MODEL.md.

## Ranking criteria

feasibility (can our exact engines touch it within scope n≤10-ish),
rigor (certificate or exhaustive verdict possible), novelty potential,
theoretical interest, maintainability.

## C1 — Hadwiger's conjecture, low-order exhaustive verification (rank 2)

- Statement: for every finite simple G, χ(G) ≤ h(G) (Hadwiger 1943).
- Open in general; PROVEN for χ ≤ 6 (Hadwiger–Kantor–Kroon; Robertson–Seymour
  for t=7 partial results...). A computer counterexample search at small n
  is therefore a RE-VERIFICATION of known theorems — valuable as engine
  validation (our M2 campaign did exactly this: 209 classes n≤6, zero
  witnesses, and it CAUGHT a real engine bug), not as discovery.
- Feasibility: exhaustive n≤6 (done), n=7 possible with geng backend
  (3,040,869 classes). Rigor: EXACT_FINITE. Novelty: none at n≤6 — that is
  known and we do not claim otherwise.
- Role: gold-standard engine validation + Lean certificate family (χ(K_n)=h
  patterns generalize into an infinite-family theorem attempt: see C4).

## C2 — Weak Hadwiger / Woodall: h(G)·α(G) ≥ n (rank 3)

- Statement: for all G, h(G) * α(G) ≥ |V(G)| (Woodall 1989 subconjecture of
  Hadwiger; implied by HC; open in general).
- Known verified at small orders; no counterexample known at any order.
  Sampled 20k draws found nothing (EXPERIMENTAL only).
- Feasibility: n≤7 exhaustively (engine needs geng for n=7 unlabeled).
  Rigor: EXACT_FINITE scope statements possible. Novelty: re-verification
  territory at these n — same honesty constraint as C1.
- Role: second independent predicate path to stress the Hadwiger kernel.

## C3 — Exact χ/α/ω/h invariant atlas for unlabeled n ≤ 7 (rank 4)

- Statement not a conjecture but a DATA product: full exact tables
  (n,m,deg,tri,ω,α,χ,h) for all unlabeled graphs to n=7, cross-verified by
  three independent implementations (Rust, Python/DP, Lean kernel sample).
- Novelty: tables to n=6 are classical; n=7 (3M classes) adds Hadwiger
  numbers which are NOT in standard small-graph catalogs (House of Graphs
  has χ,ω,α; h is harder). Honest novelty: "first publicly verifiable exact
  Hadwiger atlas at n=7" is checkable against catalog claims, not against
  the literature's theorems — and requires the geng backend.
- Value: reference data for M5-style triage and for selling the audit
  product; medium effort once n=7 enumeration lands.

## C4 — Constructive infinite family: χ = h = t on t×2 "double clique"
inflations? — NOVEL-ATTEMPT (rank 1, chosen)

- Exact statement (candidate theorem, currently CONJECTURE-level to us):
  For t ≥ 2, let G_t be the graph on vertex set {0,…,2t−1} with two cliques
  A = {0..t−1}, B = {t..2t−1} AND a perfect matching between them
  (i.e. the complete graph K_{2t} MINUS a perfect matching, thecocktail
  party graph CP(t)). Then:
    (a) χ(CP(t)) = t          [classical, easy]
    (b) ω(CP(t)) = t          [classical]
    (c) h(CP(t)) = t          [we aim to prove in Lean: K_{t+1} minor?
         NO — for t=2 (C4): h=2? C4 minor-max: K3? contract one edge of C4
         -> P4 no K3? C4 has 4 vertices 4 edges; K3 minor needs 3
         connected pairwise-adjacent branch sets: 3 pairs of adjacencies;
         possible: {0,1},{2},{3}: {0,1}-{2} edge 1-2 yes, {2}-{3} yes,
         {0,1}-{3} yes 0-3. so h(C4)=3 = t+1! General: CP(t) has
         K_{t+1} minor? — this is exactly the bounded computation we can do
         for small t and guess the pattern.]
  Our engines measure h(CP(t)) for t=2..5 exactly; the Lean file then
  states the discovered pattern as an explicit finite theorem per t, plus
  a real inductive proof if one lands.
- Why rank 1: (i) genuine new-math attempt (the minor number of cocktail
  party graphs computed and proved by us at each concrete t, with kernel
  certificates), (ii) fully in engine scope (n = 2t ≤ 10), (iii) has a
  clean Lean-bridgeable form (explicit matrix + certificate per instance),
  (iv) if the pattern h(CP(t)) = t + 1 holds for all t we get an INFINITE
  FAMILY theorem (structural proof: K_{t+1}-minor from CP(t) via matching-
  edge contractions; upper bound via edge count: |E| = t(2t−1) −? CP(t)
  has (2t choose 2) − t edges = t(2t−1). K_{t+2} minor would need
  C(t+2,2) = (t+2)(t+1)/2 inter-branch edges — for t≥? compare with
  t(2t−1): (t+2)(t+1)/2 ≤ t(2t−1) ⇔ t²+3t+2 ≤ 4t²−2t ⇔ 0 ≤ 3t²−5t−2,
  true for t ≥ 2. So edge count does NOT cap h at t+1 — upper bound needs
  the structure argument. That's real math work, bounded and checkable.
- Kill criteria: if h(CP(t)) > t+1 for some t≤5 the pattern dies, we report
  the measured table honestly and move to C3.
- Novelty check status: cocktail party graph invariants (χ, ω) are classical
  (they are Kneser-related complements K_{t,t}? CP(t) = K_{2t} minus perfect
  matching = complement of t disjoint K2 = tK2-complement). Hadwiger number
  of CP(t) — must check against minor-closed literature before claiming
  anything; NOVELTY_REVIEWED only if a search finds no prior exact formula.

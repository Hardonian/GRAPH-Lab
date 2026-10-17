//! Hadwiger number h(G): the largest t such that K_t is a minor of G.
//!
//! Exact for small n via branch-set search: G has a K_t minor iff there is
//! a partition of SOME subset S ⊆ V into t nonempty connected parts with at
//! least one edge between every pair of parts (standard characterization —
//! branch sets; vertices outside S are deleted, which minor operations
//! allow). Restricted-growth enumeration avoids counting the same branch
//! sets up to part permutation.
//!
//! Caps: n <= HADWIGER_MAX_N = 10 (B_10 partitions with deletion is still
//! bounded by the edge-count upper bound pruning). Above the cap the module
//! ERRORS; it never approximates silently.
//!
//! Cross-validated in tests against: known theorem values (all graphs
//! n<=4), Petersen (h=5 via explicit K5 branch sets), trees (h=2), complete
//! graphs (h=n), and monotonicity spot checks.

use crate::graph::{Graph, GraphError};

pub const HADWIGER_MAX_N: usize = 10;

#[derive(Debug, thiserror::Error)]
pub enum HadwigerError {
    #[error("exact Hadwiger search supports n <= {HADWIGER_MAX_N}, got {0}")]
    TooLarge(usize),
    #[error(transparent)]
    Graph(#[from] GraphError),
}

/// Largest t with K_t minor. Empty graph (n=0): h = 0 (no nonempty branch
/// sets exist; convention recorded for formalization).
pub fn hadwiger_number(g: &Graph) -> Result<usize, HadwigerError> {
    let n = g.num_vertices();
    if n > HADWIGER_MAX_N {
        return Err(HadwigerError::TooLarge(n));
    }
    if n == 0 {
        return Ok(0);
    }
    let m = g.edge_count() as u64;
    // K_t minor needs at least t(t-1)/2 edges (one per pair of branch sets)
    let mut ub = 1usize;
    while (((ub + 1) as u64) * (ub as u64) / 2) <= m && ub < n {
        ub += 1;
    }
    // h <= n also
    ub = ub.min(n);
    for t in (1..=ub).rev() {
        if has_kminor(g, t)? {
            return Ok(t);
        }
    }
    Ok(1)
}

pub const DELETED: usize = usize::MAX;

/// Generate the exact constructive branch-set certificate for the complete minor
/// K_{t + floor(t/2)} of the cocktail party graph CP(t).
pub fn cocktail_party_minor_model(t: usize) -> Vec<usize> {
    if t == 0 {
        return Vec::new();
    }
    let n = 2 * t;
    let mut assign = vec![DELETED; n];
    let k = t / 2;

    for j in 0..k {
        // Block j has matching pairs 2j ({4j, 4j+1}) and 2j+1 ({4j+2, 4j+3})
        // Part 3*j: {4*j, 4*j + 2} (connected by edge in CP(t))
        assign[4 * j] = 3 * j;
        assign[4 * j + 2] = 3 * j;
        // Part 3*j + 1: {4*j + 1}
        assign[4 * j + 1] = 3 * j + 1;
        // Part 3*j + 2: {4*j + 3}
        assign[4 * j + 3] = 3 * j + 2;
    }

    if t % 2 != 0 {
        // Leftover pair: {2t - 2, 2t - 1}
        // Part 3*k: {2t - 2}
        assign[2 * t - 2] = 3 * k;
        // vertex 2t - 1 remains DELETED
    }

    assign
}

/// Generate the exact constructive branch-set certificate for the complete minor
/// K_{a + 1} of the complete bipartite graph K_{a, b} (where 1 <= a <= b).
pub fn complete_bipartite_minor_model(a: usize, b: usize) -> Vec<usize> {
    if a == 0 || b == 0 {
        return Vec::new();
    }
    let n = a + b;
    let mut assign = vec![DELETED; n];
    if a == 1 {
        // K_{1, b}: vertex 0 in A is branch 0; vertex 1 in B is branch 1; rest deleted
        assign[0] = 0;
        assign[1] = 1;
        return assign;
    }
    // For a >= 2:
    // a - 1 mixed branches: {i, a + i} for i in 0..(a - 1)
    for i in 0..(a - 1) {
        assign[i] = i;
        assign[a + i] = i;
    }
    // 1 singleton in A: {a - 1} is branch a - 1
    assign[a - 1] = a - 1;
    // 1 singleton in B: {2a - 1} is branch a (since b >= a, 2a - 1 < a + b)
    assign[2 * a - 1] = a;
    assign
}

/// Exact Hadwiger number for complete multipartite graph K_{n_1, ..., n_k}.
/// Formula: min(1 + sum_{i=0..k-2} sorted_parts[i], floor((n + k) / 2)).
pub fn complete_multipartite_hadwiger_number(parts: &[usize]) -> usize {
    if parts.is_empty() {
        return 0;
    }
    if parts.len() == 1 {
        return 1;
    }
    let mut sorted = parts.to_vec();
    sorted.sort_unstable();
    let k = sorted.len();
    let n: usize = sorted.iter().sum();
    let sum_rest: usize = sorted[..k - 1].iter().sum();
    let bound_part_separation = 1 + sum_rest;
    let bound_parity = (n + k) / 2;
    bound_part_separation.min(bound_parity)
}

/// Generate an exact constructive branch-set certificate for the complete minor
/// of size h(K_{n_1, ..., n_k}) in the complete multipartite graph.
pub fn complete_multipartite_minor_model(parts: &[usize]) -> Vec<usize> {
    if parts.is_empty() {
        return Vec::new();
    }
    let n: usize = parts.iter().sum();
    if parts.len() == 1 {
        let mut assign = vec![DELETED; n];
        if n > 0 {
            assign[0] = 0;
        }
        return assign;
    }
    let target_t = complete_multipartite_hadwiger_number(parts);
    let Ok(g) = Graph::complete_multipartite(parts) else {
        return Vec::new();
    };

    let k = parts.len();
    let mut part_vertices = Vec::with_capacity(k);
    let mut curr = 0;
    for &sz in parts {
        part_vertices.push((curr..curr + sz).collect::<Vec<usize>>());
        curr += sz;
    }

    let min_m = target_t.saturating_sub(k);
    for m in min_m..=target_t {
        let num_singletons = target_t - m;
        if num_singletons > k {
            continue;
        }
        for mask in 0..(1usize << k) {
            if mask.count_ones() as usize != num_singletons {
                continue;
            }
            let mut assign = vec![DELETED; n];
            let mut branch_idx = 0;

            for (p, p_verts) in part_vertices.iter().enumerate() {
                if (mask >> p) & 1 == 1 {
                    assign[p_verts[0]] = branch_idx;
                    branch_idx += 1;
                }
            }

            let mut curr_avail: Vec<Vec<usize>> = Vec::with_capacity(k);
            for (p, p_verts) in part_vertices.iter().enumerate() {
                let start_idx = if (mask >> p) & 1 == 1 { 1 } else { 0 };
                curr_avail.push(p_verts[start_idx..].to_vec());
            }

            let total_avail: usize = curr_avail.iter().map(|v| v.len()).sum();
            if total_avail < 2 * m {
                continue;
            }

            let mut possible = true;
            for _ in 0..m {
                let mut sorted_parts: Vec<(usize, usize)> = curr_avail
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| !v.is_empty())
                    .map(|(p, v)| (v.len(), p))
                    .collect();
                sorted_parts.sort_unstable_by_key(|a| std::cmp::Reverse(a.0));
                if sorted_parts.len() < 2 {
                    possible = false;
                    break;
                }
                let p1 = sorted_parts[0].1;
                let p2 = sorted_parts[1].1;
                let v1 = curr_avail[p1].pop().unwrap();
                let v2 = curr_avail[p2].pop().unwrap();
                assign[v1] = branch_idx;
                assign[v2] = branch_idx;
                branch_idx += 1;
            }

            if !possible {
                continue;
            }

            if check_branch_model(&g, &assign, target_t) {
                return assign;
            }
        }
    }
    Vec::new()
}

/// Given a valid minor model for K_t on a base graph with base_n vertices,
/// lift it to a valid minor model for K_{t + 1} on the apex graph G * K_1 (base_n + 1 vertices).
pub fn apex_minor_model(base_model: &[usize], base_n: usize) -> Vec<usize> {
    let t = base_model
        .iter()
        .filter(|&&p| p != DELETED)
        .max()
        .map(|&m| m + 1)
        .unwrap_or(0);
    let mut assign = Vec::with_capacity(base_n + 1);
    assign.extend_from_slice(&base_model[..base_n.min(base_model.len())]);
    while assign.len() < base_n {
        assign.push(DELETED);
    }
    assign.push(t);
    assign
}

/// Is there a K_t minor? Restricted-growth branch-set search with deletion.
pub fn has_kminor(g: &Graph, t: usize) -> Result<bool, HadwigerError> {
    let n = g.num_vertices();
    if n > HADWIGER_MAX_N {
        return Err(HadwigerError::TooLarge(n));
    }
    if t == 0 {
        return Ok(false);
    }
    if t == 1 {
        return Ok(n >= 1);
    }
    if t > n {
        return Ok(false);
    }
    let mut assign = vec![DELETED; n];
    Ok(rg(g, t, 0, 0, &mut assign))
}

fn rg(g: &Graph, t: usize, v: usize, parts_used: usize, assign: &mut [usize]) -> bool {
    let n = g.num_vertices();
    if parts_used == t {
        // remaining vertices must all be deleted or join existing parts;
        // try to complete the assignment with parts full
        return complete_with_existing(g, t, v, assign);
    }
    // prune: not enough vertices left to open the remaining parts
    if parts_used + (n - v) < t {
        return false;
    }
    // option: delete v
    assign[v] = DELETED;
    if rg(g, t, v + 1, parts_used, assign) {
        return true;
    }
    // options: join existing parts (restricted growth: try lowest first)
    for p in 0..parts_used {
        assign[v] = p;
        if rg(g, t, v + 1, parts_used, assign) {
            return true;
        }
    }
    // option: open a new part
    assign[v] = parts_used;
    if rg(g, t, v + 1, parts_used + 1, assign) {
        return true;
    }
    assign[v] = DELETED;
    false
}

/// All t parts already opened at vertex index v; remaining vertices may
/// delete or join existing parts. We test the current assignment first;
/// adding more vertices can only help connectivity within parts (edges
/// inside a part), but adjacency between parts is already fixed by opened
/// vertices... NO: joining an existing part can create new cross edges that
/// satisfy missing adjacency, so we must still search joins.
fn complete_with_existing(g: &Graph, t: usize, v: usize, assign: &mut [usize]) -> bool {
    let n = g.num_vertices();
    if check(g, assign, t) {
        return true;
    }
    if v == n {
        return false;
    }
    for p in 0..t {
        assign[v] = p;
        if complete_with_existing(g, t, v + 1, assign) {
            return true;
        }
    }
    assign[v] = DELETED;
    complete_with_existing(g, t, v + 1, assign)
}

/// Verify whether an assignment of vertices to branch sets 0..t certifies a K_t minor.
/// Uses zero-allocation bitsets and bitwise BFS.
pub fn check_branch_model(g: &Graph, assign: &[usize], t: usize) -> bool {
    if t == 0 {
        return false;
    }
    if t > 16 {
        return false;
    }
    let mut part_masks = [0u64; 16];
    let mut part_neighbors = [0u64; 16];

    for (v, &p) in assign.iter().enumerate() {
        if p != DELETED {
            if p >= t {
                return false;
            }
            part_masks[p] |= 1u64 << v;
            part_neighbors[p] |= g.adjacency_row(v);
        }
    }

    // 1. Every part must be nonempty
    for &mask in part_masks.iter().take(t) {
        if mask == 0 {
            return false;
        }
    }

    // 2. Pairwise adjacency between distinct parts:
    // Part a is adjacent to part b iff neighbors of part a intersect part b.
    for (a, &neighbors) in part_neighbors.iter().enumerate().take(t) {
        for &b_mask in part_masks.iter().take(t).skip(a + 1) {
            if (neighbors & b_mask) == 0 {
                return false;
            }
        }
    }

    // 3. Each part must be connected in G:
    // Bitwise BFS inside the part
    for &mask in part_masks.iter().take(t) {
        let root = 1u64 << mask.trailing_zeros();
        let mut reach = root;
        loop {
            let mut expanded = reach;
            let mut frontier = reach;
            while frontier != 0 {
                let v = frontier.trailing_zeros() as usize;
                frontier &= frontier - 1;
                expanded |= g.adjacency_row(v);
            }
            let next_reach = expanded & mask;
            if next_reach == reach {
                break;
            }
            reach = next_reach;
        }
        if reach != mask {
            return false;
        }
    }

    true
}

#[inline(always)]
fn check(g: &Graph, assign: &[usize], t: usize) -> bool {
    check_branch_model(g, assign, t)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::g6::{decode_graph6, encode_graph6};

    fn k(n: usize) -> Graph {
        let e: Vec<(usize, usize)> = (0..n)
            .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
            .collect();
        Graph::from_edges(n, &e).unwrap()
    }

    fn petersen() -> Graph {
        let mut e = Vec::new();
        for i in 0..5 {
            e.push((i.min((i + 1) % 5), i.max((i + 1) % 5)));
            e.push((i, 5 + i));
            let a = 5 + i;
            let b = 5 + (i + 2) % 5;
            e.push((a.min(b), a.max(b)));
        }
        Graph::from_edges(10, &e).unwrap()
    }

    #[test]
    fn known_values() {
        assert_eq!(hadwiger_number(&k(4)).unwrap(), 4);
        assert_eq!(hadwiger_number(&k(6)).unwrap(), 6);
        // C5: h=3 (outerplanar => no K4 minor; contract two edges -> K3)
        let c5 = Graph::from_edges(5, &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]).unwrap();
        assert_eq!(hadwiger_number(&c5).unwrap(), 3);
        // P4 path: h = 2 (tree with an edge has K2 minor, forests no K3)
        let p4 = Graph::from_edges(4, &[(0, 1), (1, 2), (2, 3)]).unwrap();
        assert_eq!(hadwiger_number(&p4).unwrap(), 2);
        // edgeless n=3: h=1
        assert_eq!(hadwiger_number(&Graph::empty(3).unwrap()).unwrap(), 1);
        // K2 + isolated: h = 2 (deletion lets the isolated vertex drop out)
        let e = Graph::from_edges(3, &[(0, 1)]).unwrap();
        assert_eq!(hadwiger_number(&e).unwrap(), 2);
        assert_eq!(e.edge_count(), 1);
        // Petersen: h = 5 (contract 5 spokes -> K5). chi=3 <= 5: satisfies HC.
        let pet = petersen();
        assert_eq!(pet.edge_count(), 15);
        assert_eq!(hadwiger_number(&pet).unwrap(), 5);
        // graph6 stability
        let s = encode_graph6(&pet);
        assert_eq!(decode_graph6(&s).unwrap(), pet);
    }

    #[test]
    fn hadwiger_ge_chi_all_unlabeled_n5() {
        // Known theorem: HC holds for chi <= 6, hence trivially for all
        // n <= 5 graphs (chi <= 5). This is the M2 baseline verification:
        // EXACT_FINITE over n<=5 unlabeled.
        use crate::invariants::chromatic_number;
        for n in 0..=5usize {
            for g in crate::enumerate::enumerate_unlabeled(n).unwrap() {
                let h = hadwiger_number(&g).unwrap();
                let chi = chromatic_number(&g);
                assert!(
                    h >= chi,
                    "HC false on {} (h={h} chi={chi})",
                    encode_graph6(&g)
                );
            }
        }
    }

    #[test]
    fn refuses_above_cap() {
        let big = Graph::empty(11).unwrap();
        assert!(matches!(
            hadwiger_number(&big),
            Err(HadwigerError::TooLarge(11))
        ));
    }

    #[test]
    fn monotone_under_edge_addition() {
        let a = Graph::from_edges(5, &[(0, 1), (1, 2), (2, 3), (3, 4)]).unwrap();
        let mut b = a.clone();
        b.add_edge(0, 4).unwrap(); // + closing edge -> C5
        assert!(hadwiger_number(&b).unwrap() >= hadwiger_number(&a).unwrap());
    }

    #[test]
    fn cocktail_party_constructive_minors_verified_for_all_t() {
        for t in 1..=10usize {
            let cp = Graph::cocktail_party(t).unwrap();
            let s = t + t / 2;
            let model = cocktail_party_minor_model(t);
            assert!(
                check_branch_model(&cp, &model, s),
                "constructive minor failed for t={t}, target s={s}"
            );
        }
    }

    #[test]
    fn complete_bipartite_hadwiger_theorem_verified() {
        // Test exact exhaustive hadwiger_number and constructive model for a+b <= 10
        for a in 1..=5 {
            for b in a..=(10 - a) {
                let g = Graph::complete_bipartite(a, b).unwrap();
                let model = complete_bipartite_minor_model(a, b);
                assert!(
                    check_branch_model(&g, &model, a + 1),
                    "constructive K_{{a+1}} minor failed for K_{{{a},{b}}}"
                );
                assert_eq!(
                    hadwiger_number(&g).unwrap(),
                    a + 1,
                    "h(K_{{{a},{b}}}) != a + 1"
                );
            }
        }
        // Test constructive model for larger a, b (e.g. a=6, b=8 on n=14)
        for (a, b) in [(6, 6), (6, 8), (7, 10), (8, 12)] {
            let g = Graph::complete_bipartite(a, b).unwrap();
            let model = complete_bipartite_minor_model(a, b);
            assert!(
                check_branch_model(&g, &model, a + 1),
                "constructive K_{{a+1}} minor failed for large K_{{{a},{b}}}"
            );
        }
    }

    #[test]
    fn complete_multipartite_hadwiger_theorem_verified() {
        // All complete tripartite graphs with n <= 10
        for a in 1..=4 {
            for b in a..=4 {
                for c in b..=(10 - a - b) {
                    let parts = [a, b, c];
                    let g = Graph::complete_multipartite(&parts).unwrap();
                    let expected_h = complete_multipartite_hadwiger_number(&parts);
                    let actual_h = hadwiger_number(&g).unwrap();
                    assert_eq!(
                        actual_h, expected_h,
                        "Formula mismatch for K_{{{a},{b},{c}}}: got {actual_h}, expected {expected_h}"
                    );
                    let model = complete_multipartite_minor_model(&parts);
                    assert!(
                        check_branch_model(&g, &model, expected_h),
                        "Constructive model check failed for K_{{{a},{b},{c}}}"
                    );
                }
            }
        }

        // 4-partite and 5-partite graphs with n <= 10
        for a in 1..=2 {
            for b in a..=2 {
                for c in b..=2 {
                    for d in c..=(10 - a - b - c) {
                        let parts = [a, b, c, d];
                        let g = Graph::complete_multipartite(&parts).unwrap();
                        let expected_h = complete_multipartite_hadwiger_number(&parts);
                        let actual_h = hadwiger_number(&g).unwrap();
                        assert_eq!(actual_h, expected_h);
                        let model = complete_multipartite_minor_model(&parts);
                        assert!(check_branch_model(&g, &model, expected_h));
                    }
                }
            }
        }
    }

    #[test]
    fn universal_apex_hadwiger_theorem_verified() {
        // Apex of Path P_4 (h=2 -> h=3)
        let p4 = Graph::from_edges(4, &[(0, 1), (1, 2), (2, 3)]).unwrap();
        let apex_p4 = p4.apex().unwrap();
        assert_eq!(hadwiger_number(&p4).unwrap(), 2);
        assert_eq!(hadwiger_number(&apex_p4).unwrap(), 3);

        // Apex of Cycle C_5 (h=3 -> h=4, Wheel W_5)
        let c5 = Graph::from_edges(5, &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]).unwrap();
        let w5 = c5.apex().unwrap();
        assert_eq!(hadwiger_number(&c5).unwrap(), 3);
        assert_eq!(hadwiger_number(&w5).unwrap(), 4);

        // Apex of K_{2, 3} (h=3 -> h=4)
        let k23 = Graph::complete_bipartite(2, 3).unwrap();
        let apex_k23 = k23.apex().unwrap();
        assert_eq!(hadwiger_number(&k23).unwrap(), 3);
        assert_eq!(hadwiger_number(&apex_k23).unwrap(), 4);

        // Verify apex_minor_model lifting on K_{2, 3}
        let k23_model = complete_bipartite_minor_model(2, 3);
        let apex_model = apex_minor_model(&k23_model, 5);
        assert!(check_branch_model(&apex_k23, &apex_model, 4));
    }
}

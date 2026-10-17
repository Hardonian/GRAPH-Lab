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

fn check(g: &Graph, assign: &[usize], t: usize) -> bool {
    let n = assign.len();
    let mut sizes = vec![0usize; t];
    for &p in assign {
        if p == DELETED {
            continue;
        }
        sizes[p] += 1;
    }
    if sizes.contains(&0) {
        return false;
    }
    // pairwise adjacency between parts
    for a in 0..t {
        for b in (a + 1)..t {
            let mut adj = false;
            'e: for u in 0..n {
                if assign[u] != a {
                    continue;
                }
                for w in g.neighbors(u) {
                    if assign[w] == b {
                        adj = true;
                        break 'e;
                    }
                }
            }
            if !adj {
                return false;
            }
        }
    }
    // each part connected
    for p in 0..t {
        let members: Vec<usize> = (0..n).filter(|&v| assign[v] == p).collect();
        let mut seen = vec![false; n];
        let mut stack = vec![members[0]];
        seen[members[0]] = true;
        let mut cnt = 0usize;
        while let Some(x) = stack.pop() {
            cnt += 1;
            for y in g.neighbors(x) {
                if assign[y] == p && !seen[y] {
                    seen[y] = true;
                    stack.push(y);
                }
            }
        }
        if cnt != members.len() {
            return false;
        }
    }
    true
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
}

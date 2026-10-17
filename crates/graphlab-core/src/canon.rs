//! Canonical labeling / isomorphism — two independent backends.
//!
//! 1. `fallback_canonical_bytes` — EXHAUSTIVE lexicographic maximum of the
//!    packed adjacency over all n! vertex permutations. Exact by
//!    construction; hard-capped at FALLBACK_MAX_VERTICES (refuses above).
//! 2. `nauty_canonical_bytes` — nauty/Traces `getcanon` (vendored C via
//!    nauty-pet, compiled in). Fast; trusting nauty's canonical labeling.
//!
//! `canonical_bytes` = exhaustive form within the cap, nauty above it.
//!
//! The two backends use different byte conventions (nauty's own edge order
//! vs our lex-max packing), so cross-verification is done at the level of
//! isomorphism classes (tests build the class map and assert a bijection),
//! not byte equality. Degree-sequence hashing is NEVER an isomorphism
//! certificate here.

use crate::graph::{Graph, GraphError};
use petgraph::graph::UnGraph;

/// Hard cap for the exhaustive backend.
pub const FALLBACK_MAX_VERTICES: usize = 8;

#[derive(Debug, Clone, thiserror::Error)]
pub enum CanonError {
    #[error("graph too large for exhaustive canonicalization (n={0} > {FALLBACK_MAX_VERTICES}); use the nauty backend")]
    TooLargeForFallback(usize),
    #[error("nauty backend failed: {0}")]
    NautyFailed(String),
    #[error("graph error: {0}")]
    Graph(#[from] GraphError),
}

/// Default canonical form: exhaustive within cap, else nauty.
pub fn canonical_bytes(g: &Graph) -> Result<Vec<u8>, CanonError> {
    if g.num_vertices() <= FALLBACK_MAX_VERTICES {
        fallback_canonical_bytes(g)
    } else {
        nauty_canonical_bytes(g)
    }
}

/// Isomorphism decision using `canonical_bytes`.
pub fn is_isomorphic(a: &Graph, b: &Graph) -> Result<bool, CanonError> {
    if a.num_vertices() != b.num_vertices() || a.edge_count() != b.edge_count() {
        return Ok(false);
    }
    Ok(canonical_bytes(a)? == canonical_bytes(b)?)
}

/// Canonical form via nauty/Traces getcanon.
pub fn nauty_canonical_bytes(g: &Graph) -> Result<Vec<u8>, CanonError> {
    use nauty_pet::prelude::*;
    let n = g.num_vertices();
    if n <= 1 {
        return Ok(Vec::new());
    }
    let mut pg: UnGraph<(), ()> = UnGraph::new_undirected();
    let idx: Vec<_> = (0..n).map(|_| pg.add_node(())).collect();
    for i in 0..n {
        for j in (i + 1)..n {
            if g.has_edge(i, j) {
                pg.add_edge(idx[i], idx[j], ());
            }
        }
    }
    let cl = pg
        .try_into_canon_labelling()
        .map_err(|e| CanonError::NautyFailed(format!("{e:?}")))?;
    // contract: cl.labelling[v] = canonical position of vertex v
    let mut perm: Vec<usize> = vec![usize::MAX; n];
    for (v, &pos) in cl.labelling.iter().enumerate() {
        if pos >= n {
            return Err(CanonError::NautyFailed(format!(
                "labelling position {pos} out of range for n={n}"
            )));
        }
        if perm[pos] != usize::MAX {
            return Err(CanonError::NautyFailed(
                "labelling is not a permutation (duplicate position)".into(),
            ));
        }
        perm[pos] = v;
    }
    if perm.contains(&usize::MAX) {
        return Err(CanonError::NautyFailed(
            "labelling is not surjective".into(),
        ));
    }
    Ok(apply_and_pack(g, &perm))
}

/// Exhaustive lexicographic-max canonical bytes (exact; capped).
pub fn fallback_canonical_bytes(g: &Graph) -> Result<Vec<u8>, CanonError> {
    let n = g.num_vertices();
    if n > FALLBACK_MAX_VERTICES {
        return Err(CanonError::TooLargeForFallback(n));
    }
    let mut best: Option<Vec<u8>> = None;
    let mut used = vec![false; n];
    let mut cur = Vec::with_capacity(n);
    enumerate(g, n, &mut used, &mut cur, &mut best);
    Ok(best.unwrap_or_else(|| raw_pack(g)))
}

fn enumerate(
    g: &Graph,
    n: usize,
    used: &mut [bool],
    cur: &mut Vec<usize>,
    best: &mut Option<Vec<u8>>,
) {
    if cur.len() == n {
        let packed = apply_and_pack(g, cur);
        if best.as_ref().is_none_or(|b| packed > *b) {
            *best = Some(packed);
        }
        return;
    }
    for v in 0..n {
        if !used[v] {
            used[v] = true;
            cur.push(v);
            enumerate(g, n, used, cur, best);
            cur.pop();
            used[v] = false;
        }
    }
}

/// Pack upper-triangle adjacency (i<j, i ascending, j ascending) MSB-first
/// into bytes so lexicographic byte order is a total order.
fn pack_pairs(bits: &[bool]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bits.len().div_ceil(6));
    for chunk in bits.chunks(6) {
        let mut v: u8 = 0;
        for b in chunk {
            v = (v << 1) | (*b as u8);
        }
        v <<= 6 - chunk.len();
        out.push(v);
    }
    out
}

fn raw_pack(g: &Graph) -> Vec<u8> {
    let n = g.num_vertices();
    let mut bits = Vec::with_capacity(if n < 2 { 0 } else { n * (n - 1) / 2 });
    for i in 0..n {
        for j in (i + 1)..n {
            bits.push(g.has_edge(i, j));
        }
    }
    pack_pairs(&bits)
}

/// perm[pos] = original vertex placed at pos.
fn apply_and_pack(g: &Graph, perm: &[usize]) -> Vec<u8> {
    let n = g.num_vertices();
    let mut bits = Vec::with_capacity(if n < 2 { 0 } else { n * (n - 1) / 2 });
    for i in 0..n {
        for j in (i + 1)..n {
            bits.push(g.has_edge(perm[i], perm[j]));
        }
    }
    pack_pairs(&bits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;

    fn all_labeled(n: usize) -> Vec<Graph> {
        let pairs: Vec<(usize, usize)> = (0..n)
            .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
            .collect();
        let m = pairs.len();
        (0u64..(1u64 << m))
            .map(|bits| {
                let edges: Vec<(usize, usize)> = pairs
                    .iter()
                    .enumerate()
                    .filter(|(k, _)| (bits >> k) & 1 == 1)
                    .map(|(_, &p)| p)
                    .collect();
                Graph::from_edges(n, &edges).unwrap()
            })
            .collect()
    }

    #[test]
    fn backends_agree_on_isomorphism_classes_n5() {
        // Every labeled graph on 5 vertices: exhaustive class comparison.
        let graphs = all_labeled(5);
        let mut fb: Vec<Vec<u8>> = Vec::new();
        let mut nau: Vec<Vec<u8>> = Vec::new();
        for g in &graphs {
            fb.push(fallback_canonical_bytes(g).unwrap());
            nau.push(nauty_canonical_bytes(g).unwrap());
        }
        // Bijection check: same partition induced by the two classings.
        for i in 0..graphs.len() {
            for j in i + 1..graphs.len() {
                assert_eq!(
                    fb[i] == fb[j],
                    nau[i] == nau[j],
                    "backend class disagreement between graphs {i},{j}"
                );
            }
        }
    }

    #[test]
    fn isomorphic_pairs_share_canon() {
        let a = Graph::from_edges(4, &[(0, 1), (1, 2), (2, 3)]).unwrap();
        let b = Graph::from_edges(4, &[(2, 0), (0, 3), (3, 1)]).unwrap();
        assert!(is_isomorphic(&a, &b).unwrap());
        let c = Graph::from_edges(4, &[(0, 1), (2, 3)]).unwrap();
        assert!(!is_isomorphic(&a, &c).unwrap());
    }

    #[test]
    fn invariance_under_random_relabelings_n10() {
        // n=10 > cap: exercises the NAUTY path and its relabel invariance.
        use rand::seq::SliceRandom;
        use rand::SeedableRng;
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let base = Graph::from_edges(
            10,
            &[
                (0, 1),
                (1, 2),
                (2, 0),
                (3, 4),
                (4, 5),
                (5, 6),
                (6, 3),
                (0, 6),
                (7, 8),
                (8, 9),
                (9, 7),
                (6, 7),
            ],
        )
        .unwrap();
        let canon0 = nauty_canonical_bytes(&base).unwrap();
        for _ in 0..50 {
            let mut perm: Vec<usize> = (0..10).collect();
            perm.shuffle(&mut rng);
            let mut edges = Vec::new();
            for i in 0..10 {
                for j in (i + 1)..10 {
                    if base.has_edge(i, j) {
                        edges.push((perm[i], perm[j]));
                    }
                }
            }
            let g = Graph::from_edges(10, &edges).unwrap();
            assert_eq!(nauty_canonical_bytes(&g).unwrap(), canon0);
        }
    }

    #[test]
    fn fallback_refuses_above_cap() {
        let big = Graph::from_edges(FALLBACK_MAX_VERTICES + 1, &[(0, 1), (2, 3), (4, 5), (6, 7)])
            .unwrap();
        assert!(matches!(
            fallback_canonical_bytes(&big),
            Err(CanonError::TooLargeForFallback(_))
        ));
    }

    #[test]
    fn nonisomorphic_same_degree_sequence_distinguished() {
        // C6 vs 2*C3 (both 2-regular on 6 vertices) — degree sequence lies,
        // canonical form must not.
        let c6e: Vec<(usize, usize)> = (0..6).map(|i| (i, (i + 1) % 6)).collect();
        let c6 = Graph::from_edges(6, &c6e).unwrap();
        let two_tri =
            Graph::from_edges(6, &[(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)]).unwrap();
        assert_eq!(c6.degree_sequence(), two_tri.degree_sequence());
        assert!(!is_isomorphic(&c6, &two_tri).unwrap());
    }
}

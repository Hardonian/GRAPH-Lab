//! Graph enumeration.
//!
//! `enumerate_unlabeled(n)`: all non-isomorphic simple undirected graphs on
//! n vertices, via exhaustive labeled enumeration + canonical dedup, for
//! n <= EXHAUSTIVE_MAX_N. Known counts (Harary & Palmer; OEIS A000088):
//! n=0..6 -> 1, 1, 2, 4, 11, 34, 156. Tests pin these.
//!
//! `enumerate_labeled_upto(n)`: raw graph6 strings for a generator-free
//! oracle use in tests (labeled bit enumeration order — deterministic).
//!
//! The dedup uses `canon::canonical_bytes` (exhaustive within cap); this is
//! exact, not heuristic. For n > EXHAUSTIVE_MAX_N this module refuses rather
//! than pretending coverage (future: geng via nauty backend — see STATUS).

use rayon::prelude::*;

use crate::canon;
use crate::g6::encode_graph6;
use crate::graph::Graph;

/// Inductive unlabeled enumeration capped at EXHAUSTIVE_MAX_N.
pub const EXHAUSTIVE_MAX_N: usize = 9;

#[derive(Debug, thiserror::Error)]
pub enum EnumError {
    #[error("exhaustive unlabeled enumeration supports n <= {EXHAUSTIVE_MAX_N}, got {0}")]
    TooLarge(usize),
    #[error(transparent)]
    Canon(#[from] canon::CanonError),
    #[error(transparent)]
    Graph(#[from] crate::graph::GraphError),
}

/// All non-isomorphic graphs on n vertices, deterministically ordered by
/// (edge count, canonical bytes, graph6).
pub fn enumerate_unlabeled(n: usize) -> Result<Vec<Graph>, EnumError> {
    if n > EXHAUSTIVE_MAX_N {
        return Err(EnumError::TooLarge(n));
    }
    if n == 0 {
        return Ok(vec![Graph::empty(0)?]);
    }
    if n == 1 {
        return Ok(vec![Graph::empty(1)?]);
    }

    let mut current = vec![Graph::empty(1)?];

    for k in 2..=n {
        let num_masks = 1u64 << (k - 1);
        let candidates: Vec<(Vec<u8>, Graph)> = current
            .par_iter()
            .flat_map(|g| {
                let mut local = Vec::with_capacity(num_masks as usize);
                for mask in 0..num_masks {
                    let h = g.extend_vertex(mask).unwrap();
                    let key = if k <= 6 {
                        canon::canonical_bytes(&h).unwrap()
                    } else {
                        canon::nauty_canonical_bytes(&h).unwrap()
                    };
                    local.push((key, h));
                }
                local
            })
            .collect();

        let mut seen = std::collections::HashSet::new();
        let mut next = Vec::new();
        for (key, h) in candidates {
            if seen.insert(key.clone()) {
                next.push((key, h));
            }
        }
        next.sort_by(|(ka, a), (kb, b)| {
            a.edge_count().cmp(&b.edge_count()).then_with(|| ka.cmp(kb))
        });
        current = next.into_iter().map(|(_, h)| h).collect();
    }

    Ok(current)
}

/// Deterministic labeled enumeration (graph6 strings) for oracle use.
pub fn enumerate_labeled_upto(n: usize) -> Vec<String> {
    (0..=n)
        .flat_map(|k| {
            let pairs: Vec<(usize, usize)> = (0..k)
                .flat_map(|i| (i + 1..k).map(move |j| (i, j)))
                .collect();
            let m = pairs.len();
            (0u64..(1u64 << m)).map(move |bits| {
                let edges: Vec<(usize, usize)> = pairs
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| (bits >> *i) & 1 == 1)
                    .map(|(_, &p)| p)
                    .collect();
                encode_graph6(&Graph::from_edges(k, &edges).unwrap())
            })
        })
        .collect()
}

/// Known unlabeled counts n = 0..=9 (A000088). Hard-coded reference, not
/// computed — tests compare the engine against these.
pub const KNOWN_UNLABELED_COUNTS: [usize; 10] = [1, 1, 2, 4, 11, 34, 156, 1044, 12346, 274668];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::g6::decode_graph6;

    #[test]
    fn unlabeled_counts_match_known_sequence() {
        for (n, &expected) in KNOWN_UNLABELED_COUNTS.iter().take(9).enumerate() {
            let gs = enumerate_unlabeled(n).unwrap();
            assert_eq!(
                gs.len(),
                expected,
                "n={n}: engine count {} != known {expected}",
                gs.len()
            );
        }
    }

    #[test]
    fn unlabeled_count_n9_pinned() {
        let gs = enumerate_unlabeled(9).unwrap();
        assert_eq!(gs.len(), KNOWN_UNLABELED_COUNTS[9]);
    }

    #[test]
    fn refuses_above_cap() {
        assert!(matches!(
            enumerate_unlabeled(10),
            Err(EnumError::TooLarge(10))
        ));
    }

    #[test]
    fn dedup_is_idempotent_under_reencoding() {
        // every emitted graph round-trips graph6 losslessly
        for n in 1..=5 {
            for g in enumerate_unlabeled(n).unwrap() {
                let s = encode_graph6(&g);
                let h = decode_graph6(&s).unwrap();
                assert_eq!(g, h, "graph6 round-trip failed for {s}");
            }
        }
    }

    #[test]
    fn complement_bijection_on_classes() {
        // complement maps unlabeled classes to unlabeled classes bijectively
        for n in 1..=6 {
            let gs = enumerate_unlabeled(n).unwrap();
            let mut ckeys: std::collections::HashSet<Vec<u8>> = std::collections::HashSet::new();
            for g in &gs {
                ckeys.insert(canon::canonical_bytes(&g.complement()).unwrap());
            }
            assert_eq!(
                ckeys.len(),
                gs.len(),
                "complement not a class bijection at n={n}"
            );
        }
    }
}

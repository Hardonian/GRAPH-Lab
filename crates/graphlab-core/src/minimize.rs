//! Witness minimization: shrink a violating witness while preserving the
//! violation. Every accepted transformation is logged (edge/vertex removals)
//! so the final witness is reproducible step by step from the original.
//!
//! Strategy order (each must preserve violation on recheck):
//! 1. edge deletions in deterministic order (repeat passes until stable),
//! 2. vertex deletions (isolated first, then any),
//! 3. for predicate-invariant monotone shrinkage the loop iterates to fixpoint.

use serde::{Deserialize, Serialize};
use serde_json::Map;

use crate::g6::encode_graph6;
use crate::graph::Graph;
use crate::predicates;
use crate::search::Witness;

#[derive(Debug, thiserror::Error)]
pub enum MinimizeError {
    #[error("witness rejected on entry (violated=false?): {0}")]
    Entry(String),
    #[error(transparent)]
    Recheck(#[from] crate::verify::RecheckError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShrinkStep {
    pub kind: String, // "remove-edge" | "remove-vertex"
    pub detail: String,
    pub resulting_graph6: String,
    pub n: usize,
    pub m: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct MinimizedWitness {
    pub witness: Witness,
    pub steps: Vec<ShrinkStep>,
    pub original_graph6: String,
}

/// Build a new graph with `n` vertices from an edge list helper.
fn rebuild(base: &Graph, keep: &[usize], drop_edge: Option<(usize, usize)>) -> Graph {
    let mut edges: Vec<(usize, usize)> = Vec::new();
    let n = keep.len();
    let mut pos = Map::new();
    let _ = &mut pos;
    for (i, &v) in keep.iter().enumerate() {
        for (j, &w) in keep.iter().enumerate() {
            if i < j && base.has_edge(v, w) {
                let e = (i, j);
                if Some(e) == drop_edge {
                    continue;
                }
                edges.push(e);
            }
        }
    }
    Graph::from_edges(n, &edges).expect("rebuild from valid sub-edges")
}

pub fn minimize(w: &Witness) -> Result<MinimizedWitness, MinimizeError> {
    let pred = predicates::predicate_from_str(&w.predicate)
        .map_err(|e| MinimizeError::Entry(e.to_string()))?;
    let mut current = w.clone();
    crate::verify::recheck_witness(&current)?;
    let original = current.graph6.clone();
    let mut steps = Vec::new();

    let mut changed = true;
    while changed {
        changed = false;
        // ---- edge deletions ----
        let mut guard = 0;
        loop {
            guard += 1;
            if guard > 100_000 {
                break; // safety valve; logged below via steps count
            }
            let g = crate::g6::decode_graph6(&current.graph6)
                .map_err(|e| MinimizeError::Entry(e.to_string()))?;
            let n = g.num_vertices();
            let mut found = false;
            'outer: for a in 0..n {
                for b in (a + 1)..n {
                    if !g.has_edge(a, b) {
                        continue;
                    }
                    let mut edges: Vec<(usize, usize)> = Vec::new();
                    for i in 0..n {
                        for j in (i + 1)..n {
                            if (i, j) == (a, b) {
                                continue;
                            }
                            if g.has_edge(i, j) {
                                edges.push((i, j));
                            }
                        }
                    }
                    let cand = Graph::from_edges(n, &edges).expect("valid subset");
                    let (viol, vals) = pred
                        .evaluate(&cand)
                        .map_err(|e| MinimizeError::Entry(e.to_string()))?;
                    if viol {
                        let g6 = encode_graph6(&cand);
                        steps.push(ShrinkStep {
                            kind: "remove-edge".into(),
                            detail: format!("({a},{b})"),
                            resulting_graph6: g6.clone(),
                            n,
                            m: cand.edge_count(),
                        });
                        current.graph6 = g6;
                        current.m = cand.edge_count();
                        current.n = n;
                        current.reported = vals;
                        found = true;
                        changed = true;
                        break 'outer;
                    }
                }
            }
            if !found {
                break;
            }
        }
        // ---- vertex deletions ----
        loop {
            let g = crate::g6::decode_graph6(&current.graph6)
                .map_err(|e| MinimizeError::Entry(e.to_string()))?;
            let n = g.num_vertices();
            if n <= 1 {
                break;
            }
            let mut found = false;
            for v in 0..n {
                let keep: Vec<usize> = (0..n).filter(|&x| x != v).collect();
                let cand = rebuild(&g, &keep, None);
                let res = pred.evaluate(&cand);
                if let Ok((viol, vals)) = res {
                    if viol {
                        let g6 = encode_graph6(&cand);
                        steps.push(ShrinkStep {
                            kind: "remove-vertex".into(),
                            detail: format!("{v}"),
                            resulting_graph6: g6.clone(),
                            n: cand.num_vertices(),
                            m: cand.edge_count(),
                        });
                        current.graph6 = g6;
                        current.n = cand.num_vertices();
                        current.m = cand.edge_count();
                        current.reported = vals;
                        found = true;
                        changed = true;
                        break;
                    }
                }
            }
            if !found {
                break;
            }
        }
    }

    // final independent gate: minimized witness must pass full recheck
    crate::verify::recheck_witness(&current)?;
    Ok(MinimizedWitness {
        witness: current,
        steps,
        original_graph6: original,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::campaign::render_id;
    use crate::predicates::CanaryChiLeNMinus1;
    use crate::search::Predicate;
    use crate::search::{DomainMode, DomainScope, EvidenceClass};

    fn canary_witness_on_k4() -> Witness {
        let g = Graph::from_edges(4, &[(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]).unwrap();
        let (_, vals) = CanaryChiLeNMinus1.evaluate(&g).unwrap();
        Witness {
            schema: "graphlab-witness/1".into(),
            campaign: "test".into(),
            predicate: render_id(&CanaryChiLeNMinus1.id()),
            graph6: encode_graph6(&g),
            n: 4,
            m: 6,
            reported: vals,
            domain_mode: DomainMode::ExhaustiveUnlabeled,
            domain_scope: DomainScope {
                n_min: 1,
                n_max: 4,
                exhaustive: true,
                samples: None,
            },
            evidence: EvidenceClass::ExactFinite,
            seed: None,
            run_id: "t".into(),
        }
    }

    #[test]
    fn k4_minimizes_to_k1() {
        // canary: chi > n-1 holds exactly for complete graphs; shrinking K4
        // edge-by-edge/vertex-by-vertex lands on K1 (chi=1 > 0). Every step
        // must keep violation.
        let w = canary_witness_on_k4();
        let m = minimize(&w).unwrap();
        assert_eq!(m.witness.n, 1);
        assert!(!m.steps.is_empty());
        crate::verify::recheck_witness(&m.witness).unwrap();
        // steps monotonically shrink m or n
        let mut prev_m = 6usize;
        let mut prev_n = 4usize;
        for s in &m.steps {
            assert!(
                s.m < prev_m || (s.m == prev_m && s.n < prev_n),
                "non-shrinking step {s:?}"
            );
            prev_m = s.m;
            prev_n = s.n;
        }
    }

    #[test]
    fn minimizer_refuses_nonwitness() {
        let mut w = canary_witness_on_k4();
        w.graph6 = encode_graph6(&Graph::from_edges(4, &[(0, 1)]).unwrap());
        assert!(minimize(&w).is_err());
    }
}

//! Independent witness re-checking.
//!
//! `recheck_witness` does NOT reuse the search path. It:
//! 1. re-decodes the graph6 string with the strict codec (fresh parse),
//! 2. re-derives every reported value from scratch on the decoded graph,
//! 3. asserts the predicate is STILL violated and the reported numbers match,
//! 4. asserts graph6 round-trip losslessness.
//!    A witness that disagrees with its own file is a pipeline failure.

use serde_json::{json, Value};

use crate::g6::{decode_graph6, encode_graph6};
use crate::predicates;
use crate::search::Witness;

#[derive(Debug, thiserror::Error)]
pub enum RecheckError {
    #[error("graph6 decode failed: {0}")]
    Decode(#[from] crate::graph::GraphError),
    #[error("graph6 round-trip mismatch: {0} vs {1}")]
    RoundTrip(String, String),
    #[error("predicate re-evaluation says NOT violated")]
    NotViolated,
    #[error("reported value mismatch: {0}")]
    ValueMismatch(String),
    #[error("unknown predicate: {0}")]
    UnknownPredicate(String),
}

pub fn recheck_witness(w: &Witness) -> Result<(), RecheckError> {
    let g = decode_graph6(&w.graph6)?;
    let re = encode_graph6(&g);
    if re != w.graph6 {
        return Err(RecheckError::RoundTrip(w.graph6.clone(), re));
    }
    if g.num_vertices() != w.n || g.edge_count() != w.m {
        return Err(RecheckError::ValueMismatch(format!(
            "n/m header {}/{} vs decoded {}/{}",
            w.n,
            w.m,
            g.num_vertices(),
            g.edge_count(),
        )));
    }
    let pred = predicates::predicate_from_str(&w.predicate)
        .map_err(|_| RecheckError::UnknownPredicate(w.predicate.clone()))?;
    let (viol, fresh) = pred
        .evaluate(&g)
        .map_err(|e| RecheckError::ValueMismatch(e.to_string()))?;
    if !viol {
        return Err(RecheckError::NotViolated);
    }
    for (k, v) in &w.reported {
        if k.starts_with('_') {
            continue; // metadata fields
        }
        match fresh.get(k) {
            Some(fv) if fv == v => {}
            Some(fv) => {
                return Err(RecheckError::ValueMismatch(format!(
                    "{k}: witness={} recheck={fv}",
                    v.as_i64().unwrap_or(-9999)
                )))
            }
            None => {
                return Err(RecheckError::ValueMismatch(format!(
                    "{k}: absent on recheck"
                )))
            }
        }
    }
    Ok(())
}

/// Fully independent re-check in the sense of "another implementation":
/// re-derives chi and (where n <= cap) h with fresh decode, and emits the
/// comparison as a JSON value for the evidence ledger.
pub fn independent_report(w: &Witness) -> Result<Value, RecheckError> {
    recheck_witness(w)?;
    let g = decode_graph6(&w.graph6)?;
    let chi = crate::invariants::chromatic_number(&g);
    let h = crate::hadwiger::hadwiger_number(&g).ok();
    let alpha = crate::invariants::independence_number(&g);
    Ok(json!({
        "graph6": w.graph6,
        "n": g.num_vertices(),
        "m": g.edge_count(),
        "chi": chi,
        "hadwiger": h,
        "alpha": alpha,
        "recheck": "ok",
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::campaign::render_id;
    use crate::predicates::CanaryChiLeNMinus1;
    use crate::search::Predicate;
    use crate::search::{DomainMode, DomainScope, EvidenceClass};
    use serde_json::Map;

    fn canary_witness() -> Witness {
        let g =
            crate::graph::Graph::from_edges(4, &[(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)])
                .unwrap();
        let (viol, vals) = CanaryChiLeNMinus1.evaluate(&g).unwrap();
        assert!(viol);
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
                n_min: 4,
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
    fn good_witness_passes() {
        let w = canary_witness();
        recheck_witness(&w).unwrap();
        let v = independent_report(&w).unwrap();
        assert_eq!(v["chi"], 4);
    }

    #[test]
    fn tampered_reported_value_fails() {
        let mut w = canary_witness();
        let mut m = Map::new();
        m.insert("n".into(), 4.into());
        m.insert("chi".into(), 99.into()); // lied about chi
        w.reported = m;
        assert!(matches!(
            recheck_witness(&w),
            Err(RecheckError::ValueMismatch(_))
        ));
    }

    #[test]
    fn tampered_graph6_fails() {
        let mut w = canary_witness();
        w.graph6 = "Dw".into(); // K3 instead of K4; m/n header now wrong
        assert!(recheck_witness(&w).is_err());
    }

    #[test]
    fn nonviolating_graph_fails() {
        let mut w = canary_witness();
        let p3 = crate::graph::Graph::from_edges(4, &[(0, 1), (1, 2), (2, 3)]).unwrap();
        w.graph6 = encode_graph6(&p3);
        w.n = 4;
        w.m = 3;
        let (viol, vals) = CanaryChiLeNMinus1.evaluate(&p3).unwrap();
        assert!(!viol);
        w.reported = vals;
        assert!(matches!(
            recheck_witness(&w),
            Err(RecheckError::NotViolated)
        ));
    }
}

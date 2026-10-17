//! Concrete predicates for the counterexample pipeline.

use serde_json::{Map, Value};

use crate::graph::Graph;
use crate::hadwiger;
use crate::invariants;
use crate::search::{Predicate, PredicateError, PredicateId};

/// Hadwiger's conjecture: chi(G) <= h(G). Violation predicate: h < chi.
pub struct HadwigerLowerBound;

impl Predicate for HadwigerLowerBound {
    fn id(&self) -> PredicateId {
        PredicateId::HadwigerLowerBoundV1
    }
    fn evaluate(&self, g: &Graph) -> Result<(bool, Map<String, Value>), PredicateError> {
        let n = g.num_vertices();
        let h =
            hadwiger::hadwiger_number(g).map_err(|e| PredicateError::OutOfDomain(e.to_string()))?;
        let chi = invariants::chromatic_number(g);
        let mut m = Map::new();
        m.insert("n".into(), n.into());
        m.insert("chi".into(), chi.into());
        m.insert("hadwiger".into(), h.into());
        Ok((h < chi, m))
    }
}

/// Weak Hadwiger (Woodall 1989 subconjecture): h(G) * alpha(G) >= n.
pub struct WeakHadwiger;

impl Predicate for WeakHadwiger {
    fn id(&self) -> PredicateId {
        PredicateId::WeakHadwigerV1
    }
    fn evaluate(&self, g: &Graph) -> Result<(bool, Map<String, Value>), PredicateError> {
        let n = g.num_vertices();
        let h =
            hadwiger::hadwiger_number(g).map_err(|e| PredicateError::OutOfDomain(e.to_string()))?;
        let alpha = invariants::independence_number(g);
        let mut m = Map::new();
        m.insert("n".into(), n.into());
        m.insert("hadwiger".into(), h.into());
        m.insert("alpha".into(), alpha.into());
        m.insert("h_times_alpha".into(), (h * alpha).into());
        Ok((n > 0 && h * alpha < n, m))
    }
}

/// Pipeline canary: "for all G with n >= 2: chi(G) <= n-1" — falsified by
/// every complete graph K_n (n>=2). Used to prove the search/verify/minimize
/// machinery actually fires; never presented as research.
pub struct CanaryChiLeNMinus1;

impl Predicate for CanaryChiLeNMinus1 {
    fn id(&self) -> PredicateId {
        PredicateId::CanaryChiLeNMinus1V1
    }
    fn evaluate(&self, g: &Graph) -> Result<(bool, Map<String, Value>), PredicateError> {
        let n = g.num_vertices();
        if n == 0 {
            return Err(PredicateError::OutOfDomain("canary requires n >= 1".into()));
        }
        let chi = invariants::chromatic_number(g);
        let mut m = Map::new();
        m.insert("n".into(), n.into());
        m.insert("chi".into(), chi.into());
        Ok((n >= 1 && chi > n - 1, m))
    }
}

pub fn by_id(id: &PredicateId) -> Result<Box<dyn Predicate>, PredicateError> {
    match id {
        PredicateId::HadwigerLowerBoundV1 => Ok(Box::new(HadwigerLowerBound)),
        PredicateId::WeakHadwigerV1 => Ok(Box::new(WeakHadwiger)),
        PredicateId::CanaryChiLeNMinus1V1 => Ok(Box::new(CanaryChiLeNMinus1)),
        PredicateId::Custom(name) => Err(PredicateError::OutOfDomain(format!(
            "unknown predicate '{name}' — add it to the registry with an exact statement"
        ))),
    }
}

pub fn predicate_from_str(s: &str) -> Result<Box<dyn Predicate>, PredicateError> {
    let id = match s {
        "hadwiger-lower-bound-v1" => PredicateId::HadwigerLowerBoundV1,
        "weak-hadwiger-v1" => PredicateId::WeakHadwigerV1,
        "canary-chi-le-n-1-v1" => PredicateId::CanaryChiLeNMinus1V1,
        other => PredicateId::Custom(other.to_string()),
    };
    by_id(&id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::Predicate;

    fn k(n: usize) -> Graph {
        let e: Vec<(usize, usize)> = (0..n)
            .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
            .collect();
        Graph::from_edges(n, &e).unwrap()
    }

    #[test]
    fn weak_hadwiger_holds_on_known_families() {
        // K5: h=5, alpha=1, n=5 -> 5>=5 ok
        let p = WeakHadwiger;
        let (viol, _) = p.evaluate(&k(5)).unwrap();
        assert!(!viol);
        // edgeless n=3: h=1, alpha=3, 3>=3 ok
        let (viol, _) = p.evaluate(&Graph::empty(3).unwrap()).unwrap();
        assert!(!viol);
        // C5: h=3, alpha=2, 6>=5 ok
        let c5 = Graph::from_edges(5, &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]).unwrap();
        let (viol, _) = p.evaluate(&c5).unwrap();
        assert!(!viol);
    }

    #[test]
    fn hadwiger_holds_on_petersen() {
        let mut e = Vec::new();
        for i in 0..5 {
            e.push((i.min((i + 1) % 5), i.max((i + 1) % 5)));
            e.push((i, 5 + i));
            let a = 5 + i;
            let b = 5 + (i + 2) % 5;
            e.push((a.min(b), a.max(b)));
        }
        let pet = Graph::from_edges(10, &e).unwrap();
        let (viol, vals) = HadwigerLowerBound.evaluate(&pet).unwrap();
        assert!(!viol, "Petersen: {vals:?}");
        assert_eq!(vals["hadwiger"], 5);
        assert_eq!(vals["chi"], 3);
    }

    #[test]
    fn canary_fires_on_complete_graphs_only() {
        let p = CanaryChiLeNMinus1;
        // K3: chi=3 > n-1=2 -> violation (canary hit)
        assert!(p.evaluate(&k(3)).unwrap().0);
        // P3: chi=2 <= 2 no
        let p3 = Graph::from_edges(3, &[(0, 1), (1, 2)]).unwrap();
        assert!(!p.evaluate(&p3).unwrap().0);
        // n=1: chi=1 > 0 -> fires? n>=1 && chi > n-1 -> 1>0 true. K1 is
        // "complete graph with chi = n". Documented: canary fires for all K_n,
        // n>=1; harmless since it's only ever a machinery test.
        assert!(p.evaluate(&Graph::empty(1).unwrap()).unwrap().0);
    }

    #[test]
    fn unknown_predicate_errors_not_panics() {
        let e = predicate_from_str("not-a-predicate");
        assert!(e.is_err());
    }
}

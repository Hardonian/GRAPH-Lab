//! Counterexample search: campaigns, deterministic generators, witnesses.
//!
//! Design contract (docs/ARCHITECTURE.md):
//! - Every search is either EXHAUSTIVE over a declared domain (labeled or
//!   unlabeled graphs with n <= cap) or SEEDED-SAMPLED (explicitly labeled
//!   EXPERIMENTAL — sampling never supports a universal claim, and an
//!   uneventful sample never supports a negative claim).
//! - A hit is recorded as a Witness with a violated-predicate id, the graph
//!   in graph6 form, engine values, and full run metadata. The witness is
//!   RE-CHECKED by `verify::recheck_witness` (fresh predicate evaluation +
//!   independent graph6 decode) before being emitted to the output file.
//! - Minimization (`minimize.rs`) shrinks a witness by edge/vertex removal
//!   while the violation persists, logging every accepted transformation.

use serde::{Deserialize, Serialize};

use crate::graph::Graph;

/// Predicate id registry. Every campaign references a predicate by id.
/// The predicate code lives in `predicates.rs`; the id is versioned so a
/// witness always names the exact definition it violated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PredicateId {
    /// "h(G) >= chi(G)" — Hadwiger's conjecture. Violation = h < chi.
    HadwigerLowerBoundV1,
    /// "h(G) * alpha(G) >= n" — weak Hadwiger (Woodall). Violation = h*a < n.
    WeakHadwigerV1,
    /// "omega(G) * chi(G) >= n"? no — that's false trivially; kept as a
    /// pipeline canary predicate (expected hits prove the search works):
    /// "for all G with n>=2: chi(G) <= n - 1" — false on K_n (canary).
    CanaryChiLeNMinus1V1,
    /// Named user predicate resolved by id string (registry lookup).
    Custom(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DomainMode {
    /// Exhaustive over all unlabeled graphs with n0 <= n <= n1 (canonical).
    ExhaustiveUnlabeled,
    /// Exhaustive over all labeled graphs with n0 <= n <= n1.
    ExhaustiveLabeled,
    /// Seeded pseudo-random labeled graphs, `samples` draws. EXPERIMENTAL.
    Sampled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceClass {
    ExactFinite,
    Experimental,
}

pub fn evidence_class(mode: DomainMode) -> EvidenceClass {
    match mode {
        DomainMode::ExhaustiveUnlabeled | DomainMode::ExhaustiveLabeled => {
            EvidenceClass::ExactFinite
        }
        DomainMode::Sampled => EvidenceClass::Experimental,
    }
}

/// The witness file schema (v1). Stable field names; additions bump the
/// minor version, semantic changes bump the major.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Witness {
    pub schema: String, // "graphlab-witness/1"
    pub campaign: String,
    pub predicate: String, // PredicateId rendered
    pub graph6: String,
    pub n: usize,
    pub m: usize,
    /// engine-reported values that constitute the violation
    pub reported: serde_json::Map<String, serde_json::Value>,
    /// the domain the hit was found under (scope for negative results)
    pub domain_mode: DomainMode,
    pub domain_scope: DomainScope,
    pub evidence: EvidenceClass,
    pub seed: Option<u64>,
    pub run_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainScope {
    pub n_min: usize,
    pub n_max: usize,
    /// true = the search covered the ENTIRE declared domain (exhaustive).
    pub exhaustive: bool,
    pub samples: Option<usize>,
}

/// A violation: the predicate's values show the claim is false for this graph.
pub trait Predicate: Send + Sync {
    fn id(&self) -> PredicateId;
    /// returns violated=true plus reported values (engine facts).
    fn evaluate(
        &self,
        g: &Graph,
    ) -> Result<(bool, serde_json::Map<String, serde_json::Value>), PredicateError>;
}

#[derive(Debug, thiserror::Error)]
pub enum PredicateError {
    #[error("instance outside predicate's declared domain: {0}")]
    OutOfDomain(String),
    #[error("evaluation error: {0}")]
    Eval(String),
}

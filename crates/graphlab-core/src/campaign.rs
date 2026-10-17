//! Campaign runner: executes a predicate search over a declared domain with
//! deterministic seeds, rayon parallelism bounded by config, and
//! independently-rechecked witness emission.
//!
//! Honesty contract:
//! - Exhaustive modes refuse (typed error) when the declared domain exceeds
//!   the engine's coverage caps; they never silently truncate.
//! - Sampled mode is EXPERIMENTAL; its summaries say so.
//! - Every witness is re-checked independently (fresh decode + fresh
//!   predicate evaluation) before acceptance; rejected witnesses are still
//!   written to the output file, marked `{"rejected": ...}`. No
//!   cherry-picking.

use std::io::Write;

use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::canon;
use crate::enumerate;
use crate::g6::encode_graph6;
use crate::graph::Graph;
use crate::search::{evidence_class, DomainMode, DomainScope, PredicateId, Witness};

#[derive(Debug, thiserror::Error)]
pub enum RunError {
    #[error("invalid campaign: {0}")]
    Config(String),
    #[error("engine limit: {0}")]
    Engine(String),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    pub name: String,
    pub predicate: String,
    pub mode: DomainMode,
    pub n_min: usize,
    pub n_max: usize,
    pub seed: Option<u64>,
    pub samples: Option<usize>,
    pub p: Option<f64>,
    pub threads: Option<usize>,
    /// stop after N witnesses (0 = collect all)
    pub max_witnesses: usize,
}

#[derive(Debug, Serialize)]
pub struct RunSummary {
    pub campaign: String,
    pub run_id: String,
    pub predicate: String,
    pub mode: DomainMode,
    pub domain_scope: DomainScope,
    pub evidence: String,
    pub graphs_examined: u64,
    pub witnesses: usize,
    pub witness_rechecks_ok: usize,
    pub witness_rechecks_failed: usize,
    pub eval_errors: Vec<String>,
    pub wall_ms: u128,
    pub seed: Option<u64>,
    pub threads: usize,
    pub output: String,
}

/// Unlabeled exhaustive coverage cap = engine enumeration cap (n <= 6).
fn exhaustive_unlabeled_graphs(n_min: usize, n_max: usize) -> Result<Vec<Graph>, RunError> {
    let mut out = Vec::new();
    for n in n_min..=n_max {
        match enumerate::enumerate_unlabeled(n) {
            Ok(gs) => out.extend(gs),
            Err(e) => {
                return Err(RunError::Engine(format!(
                "{e} — declared domain n up to {n_max} exceeds coverage; refusing to fake coverage"
            )))
            }
        }
    }
    Ok(out)
}

fn exhaustive_labeled_graphs(n_min: usize, n_max: usize) -> Result<Vec<Graph>, RunError> {
    let mut out = Vec::new();
    for n in n_min..=n_max {
        if n > 7 {
            return Err(RunError::Engine(format!(
                "labeled exhaustive capped at n=7 (2^21 instances); got n={n}"
            )));
        }
        let pairs: Vec<(usize, usize)> = (0..n)
            .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
            .collect();
        let m = pairs.len();
        for bits in 0u32..(1u32 << m) {
            let edges: Vec<(usize, usize)> = pairs
                .iter()
                .enumerate()
                .filter(|(k, _)| (bits >> k) & 1 == 1)
                .map(|(_, &p)| p)
                .collect();
            out.push(Graph::from_edges(n, &edges).map_err(|e| RunError::Engine(e.to_string()))?);
        }
    }
    Ok(out)
}

/// Deterministic sampled family: ChaCha8 keyed by (campaign seed, draw
/// index); (seed, draw_index) reproduces any individual sample exactly.
pub fn sampled_graph(c: &Campaign, draw: usize) -> Result<Graph, RunError> {
    let seed = c
        .seed
        .ok_or_else(|| RunError::Config("sampled mode requires seed".into()))?;
    let p =
        c.p.ok_or_else(|| RunError::Config("sampled mode requires p".into()))?;
    let mut rng =
        ChaCha8Rng::seed_from_u64(seed ^ (draw as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    let n = rng.gen_range(c.n_min..=c.n_max);
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            if rng.gen::<f64>() < p {
                edges.push((i, j));
            }
        }
    }
    Graph::from_edges(n, &edges).map_err(|e| RunError::Engine(e.to_string()))
}

pub fn run(
    c: &Campaign,
    pred: &dyn crate::search::Predicate,
    threads: usize,
    out_path: &str,
) -> Result<RunSummary, RunError> {
    let t0 = std::time::Instant::now();
    let run_id = format!("{}-{}s", c.name, epoch_secs());
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads.max(1))
        .build()
        .map_err(|e| RunError::Config(e.to_string()))?;

    if c.n_min > c.n_max || c.n_min == 0 && c.n_max == 0 {
        return Err(RunError::Config("bad n range".into()));
    }

    // materialize the candidate stream (exhaustive modes validate coverage
    // up front and fail loudly if out of scope)
    let (graphs, exhaustive): (Vec<Graph>, bool) = match c.mode {
        DomainMode::ExhaustiveUnlabeled => (exhaustive_unlabeled_graphs(c.n_min, c.n_max)?, true),
        DomainMode::ExhaustiveLabeled => (exhaustive_labeled_graphs(c.n_min, c.n_max)?, true),
        DomainMode::Sampled => {
            let ns = c
                .samples
                .ok_or_else(|| RunError::Config("sampled requires samples".into()))?;
            let gs: Result<Vec<Graph>, RunError> = (0..ns).map(|d| sampled_graph(c, d)).collect();
            (gs?, false)
        }
    };

    let scope = DomainScope {
        n_min: c.n_min,
        n_max: c.n_max,
        exhaustive,
        samples: if exhaustive { None } else { c.samples },
    };

    // parallel evaluation: collect (hit, optional eval error) pairs
    type EvalOut = (Option<(Graph, Map<String, Value>)>, Option<String>);
    let eval_out: Vec<EvalOut> = pool.install(|| {
        graphs
            .par_iter()
            .map(|g| match pred.evaluate(g) {
                Ok((true, vals)) => (Some((g.clone(), vals)), None),
                Ok((false, _)) => (None, None),
                Err(e) => (None, Some(e.to_string())),
            })
            .collect()
    });

    let hits: Vec<(Graph, Map<String, Value>)> = eval_out
        .iter()
        .filter_map(|(h, _)| h.clone())
        .take(if c.max_witnesses == 0 {
            usize::MAX
        } else {
            c.max_witnesses
        })
        .collect();
    let mut eval_errors: Vec<String> = eval_out.iter().filter_map(|(_, e)| e.clone()).collect();
    eval_errors.sort();
    eval_errors.dedup();

    let file = std::fs::File::create(out_path)?;
    let mut writer = std::io::BufWriter::new(file);

    let mut witnesses = 0usize;
    let mut ok = 0usize;
    let mut failed = 0usize;
    for (g, vals) in &hits {
        let w = Witness {
            schema: "graphlab-witness/1".into(),
            campaign: c.name.clone(),
            predicate: render_id(&pred.id()),
            graph6: encode_graph6(g),
            n: g.num_vertices(),
            m: g.edge_count(),
            reported: vals.clone(),
            domain_mode: c.mode,
            domain_scope: scope.clone(),
            evidence: evidence_class(c.mode),
            seed: c.seed,
            run_id: run_id.clone(),
        };
        match crate::verify::recheck_witness(&w) {
            Ok(()) => {
                ok += 1;
                witnesses += 1;
                writeln!(writer, "{}", serde_json::to_string(&w)?)?;
            }
            Err(e) => {
                failed += 1;
                // rejected witnesses stay in the file (no cherry-picking)
                writeln!(
                    writer,
                    "{}",
                    serde_json::to_string(&json!({"rejected": w, "reason": e.to_string()}))?
                )?;
            }
        }
    }
    // also record the manifest line
    writeln!(
        writer,
        "{}",
        serde_json::to_string(&json!({"manifest": {
            "graphs_examined": graphs.len(),
            "exhaustive": exhaustive,
            "predicate": render_id(&pred.id()),
            "run_id": run_id,
        }}))?
    )?;
    writer.flush()?;

    Ok(RunSummary {
        campaign: c.name.clone(),
        run_id,
        predicate: c.predicate.clone(),
        mode: c.mode,
        domain_scope: scope,
        evidence: format!("{:?}", evidence_class(c.mode)),
        graphs_examined: graphs.len() as u64,
        witnesses,
        witness_rechecks_ok: ok,
        witness_rechecks_failed: failed,
        eval_errors,
        wall_ms: t0.elapsed().as_millis(),
        seed: c.seed,
        threads,
        output: out_path.to_string(),
    })
}

fn epoch_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn render_id(id: &PredicateId) -> String {
    match id {
        PredicateId::HadwigerLowerBoundV1 => "hadwiger-lower-bound-v1".into(),
        PredicateId::WeakHadwigerV1 => "weak-hadwiger-v1".into(),
        PredicateId::CanaryChiLeNMinus1V1 => "canary-chi-le-n-1-v1".into(),
        PredicateId::Custom(s) => s.clone(),
    }
}

/// canonical-key dedup helper used by tests and campaigns
pub fn canonical_keys(gs: &[Graph]) -> Vec<Vec<u8>> {
    gs.iter()
        .map(|g| canon::canonical_bytes(g).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::predicates::CanaryChiLeNMinus1;
    use crate::search::Predicate;

    fn tmp_out(tag: &str) -> String {
        let mut p = std::env::temp_dir();
        p.push(format!("graphlab-test-{tag}-{}.jsonl", std::process::id()));
        p.to_string_lossy().to_string()
    }

    #[test]
    fn canary_campaign_fires_and_rechecks() {
        let c = Campaign {
            name: "canary-test".into(),
            predicate: "canary-chi-le-n-1-v1".into(),
            mode: DomainMode::ExhaustiveUnlabeled,
            n_min: 1,
            n_max: 4,
            seed: None,
            samples: None,
            p: None,
            threads: Some(4),
            max_witnesses: 100,
        };
        let out = tmp_out("canary");
        let s = run(&c, &CanaryChiLeNMinus1 as &dyn Predicate, 4, &out).unwrap();
        // K1, K2, K3, K4 among unlabeled classes n=1..4 -> 4 witnesses
        assert_eq!(s.witnesses, 4, "summary: {s:?}");
        assert_eq!(s.witness_rechecks_ok, 4);
        assert_eq!(s.witness_rechecks_failed, 0);
        assert_eq!(s.graphs_examined, 1 + 2 + 4 + 11); // unlabeled n=1..4
        let _ = std::fs::remove_file(out);
    }

    #[test]
    fn sampled_mode_is_reproducible() {
        let mut c = Campaign {
            name: "samp".into(),
            predicate: "canary-chi-le-n-1-v1".into(),
            mode: DomainMode::Sampled,
            n_min: 2,
            n_max: 6,
            seed: Some(1234),
            samples: Some(200),
            p: Some(0.5),
            threads: Some(4),
            max_witnesses: 1000,
        };
        let g1 = sampled_graph(&c, 7).unwrap();
        let g2 = sampled_graph(&c, 7).unwrap();
        assert_eq!(g1, g2, "same draw index => same graph");
        c.seed = Some(1235);
        let g3 = sampled_graph(&c, 7).unwrap();
        assert_ne!(g1, g3);
    }

    #[test]
    fn exhaustive_refuses_beyond_cap_honestly() {
        let c = Campaign {
            name: "over".into(),
            predicate: "canary-chi-le-n-1-v1".into(),
            mode: DomainMode::ExhaustiveUnlabeled,
            n_min: 1,
            n_max: crate::enumerate::EXHAUSTIVE_MAX_N + 1,
            seed: None,
            samples: None,
            p: None,
            threads: Some(1),
            max_witnesses: 0,
        };
        let r = run(
            &c,
            &CanaryChiLeNMinus1 as &dyn Predicate,
            1,
            &tmp_out("over"),
        );
        assert!(matches!(r, Err(RunError::Engine(_))), "{r:?}");
    }

    #[test]
    fn hadwiger_campaign_exhaustive_n5_has_no_witnesses() {
        // Hadwiger verified for all unlabeled graphs n<=5 (known theorem
        // territory); expect zero witnesses and zero eval errors.
        let c = Campaign {
            name: "hadw-n5".into(),
            predicate: "hadwiger-lower-bound-v1".into(),
            mode: DomainMode::ExhaustiveUnlabeled,
            n_min: 0,
            n_max: 5,
            seed: None,
            samples: None,
            p: None,
            threads: Some(8),
            max_witnesses: 0,
        };
        let out = tmp_out("hn5");
        let s = run(
            &c,
            &crate::predicates::HadwigerLowerBound as &dyn Predicate,
            8,
            &out,
        )
        .unwrap();
        assert_eq!(s.graphs_examined, 1 + 1 + 2 + 4 + 11 + 34);
        assert_eq!(s.witnesses, 0);
        assert!(s.eval_errors.is_empty(), "{:?}", s.eval_errors);
        let _ = std::fs::remove_file(out);
    }
}

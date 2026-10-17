//! Benchmark harness: CPU baselines for the exact kernels, with explicit
//! thread budgets and raw JSON output. GPU probes are separate (scripts/gpu/)
//! and must pass the occupancy gate before launch.

use std::time::Instant;

use graphlab_core::enumerate;
use graphlab_core::hadwiger;
use graphlab_core::invariants;
use rayon::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct KernelBench {
    pub name: String,
    pub items: u64,
    pub threads: usize,
    pub wall_ms: u128,
    pub items_per_s: u64,
    pub single_thread_equiv_items_per_s: Option<u64>,
}

/// Enumerate-all unlabeled graphs n=0..6 and compute the full invariant set
/// for each class (canonicalization + dedup dominates).
pub fn bench_enumerate_invariants(threads: usize, reps: usize) -> KernelBench {
    let pool = build_pool(threads);
    let mut best_ms: Option<u128> = None;
    for _ in 0..reps {
        let t = Instant::now();
        let count: usize = pool.install(|| {
            // per-n enumeration is sequential inside enumerate; parallelize
            // across the per-class invariant computations at n=5/6 which carry
            // the heaviest chi/omega/alpha loads
            let mut c = 0usize;
            for n in 0..=6usize {
                let gs = enumerate::enumerate_unlabeled(n).unwrap();
                c += gs.len();
            }
            let heavy: Vec<_> = (0..=5usize)
                .flat_map(|n| enumerate::enumerate_unlabeled(n).unwrap())
                .collect();
            let _: Vec<_> = heavy.par_iter().map(invariants::all_invariants).collect();
            c
        });
        let ms = t.elapsed().as_millis();
        assert_eq!(count, 209); // known total for n=0..6
        best_ms = Some(best_ms.map_or(ms, |b: u128| b.min(ms)));
    }
    let ms = best_ms.unwrap();
    let items = 209u64 * reps as u64;
    KernelBench {
        name: "enumerate+dedup(n<=6) + invariants(n<=5)".into(),
        items,
        threads,
        wall_ms: ms,
        items_per_s: ((items.max(1) as u128) * 1000 / ms.max(1)) as u64,
        single_thread_equiv_items_per_s: None,
    }
}

/// Labeled-graph invariant sweep: the raw search inner loop at scale
/// (all 2^15 graphs on n=6 vertices labeled = 32768 instances).
pub fn bench_labeled_sweep(threads: usize) -> KernelBench {
    let pool = build_pool(threads);
    let t = Instant::now();
    let gs: Vec<_> = {
        // 6 vertices labeled
        let pairs: Vec<(usize, usize)> = (0..6)
            .flat_map(|i| (i + 1..6).map(move |j| (i, j)))
            .collect();
        let m = pairs.len();
        (0u32..(1u32 << m))
            .map(|bits| {
                let edges: Vec<(usize, usize)> = pairs
                    .iter()
                    .enumerate()
                    .filter(|(k, _)| (bits >> k) & 1 == 1)
                    .map(|(_, &p)| p)
                    .collect();
                graphlab_core::Graph::from_edges(6, &edges).unwrap()
            })
            .collect()
    };
    let n_items = gs.len() as u64;
    pool.install(|| {
        let _: Vec<_> = gs
            .par_iter()
            .map(|g| {
                (
                    g.edge_count(),
                    invariants::triangle_count(g),
                    g.degree_sequence(),
                )
            })
            .collect();
    });
    let ms = t.elapsed().as_millis().max(1);
    KernelBench {
        name: "labeled n=6 sweep: m+triangles+degree-seq".into(),
        items: n_items,
        threads,
        wall_ms: ms,
        items_per_s: ((n_items as u128) * 1000 / ms) as u64,
        single_thread_equiv_items_per_s: None,
    }
}

/// Exact Hadwiger kernel on the worst unlabeled n<=6 classes (534 branch
/// searches), parallelized.
pub fn bench_hadwiger(threads: usize) -> KernelBench {
    let pool = build_pool(threads);
    let gs: Vec<_> = (0..=6usize)
        .flat_map(|n| enumerate::enumerate_unlabeled(n).unwrap())
        .collect();
    let n_items = gs.len() as u64;
    let t = Instant::now();
    pool.install(|| {
        let _: Vec<_> = gs
            .par_iter()
            .map(|g| hadwiger::hadwiger_number(g).unwrap())
            .collect();
    });
    let ms = t.elapsed().as_millis().max(1);
    KernelBench {
        name: "hadwiger exact (n<=6, 209 unlabeled + repeats)".into(),
        items: n_items,
        threads,
        wall_ms: ms,
        items_per_s: ((n_items as u128) * 1000 / ms) as u64,
        single_thread_equiv_items_per_s: None,
    }
}

fn build_pool(threads: usize) -> rayon::ThreadPool {
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads.clamp(1, 48))
        .build()
        .expect("pool")
}

//! graphlab CLI.
//!
//! Implemented (M0/M1): doctor, hash, enumerate, invariants, canon, g6.
//! Unimplemented commands (search/minimize/verify/formalize/verify-lean/
//! benchmark) arrive in later milestones — they are NOT listed until they run.

mod bench;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use graphlab_core::{canon, enumerate, g6, invariants};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Parser)]
#[command(
    name = "graphlab",
    version,
    about = "Graph-theoretic discovery & formal verification lab"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Report host/toolchain/engine capabilities
    Doctor {
        #[arg(long)]
        json: bool,
    },
    /// Print SHA-256 of a file (evidence ledger primitive)
    Hash { path: String },
    /// Enumerate unlabeled simple graphs on n vertices (exhaustive n <= 6)
    Enumerate {
        #[arg(long)]
        vertices: usize,
        #[arg(long)]
        output: Option<String>,
    },
    /// Exact invariants of a graph (graph6 string, or '-' = stdin)
    Invariants {
        #[arg(long)]
        input: String,
        #[arg(long)]
        json: bool,
    },
    /// Canonical-form certificate bytes (hex). Isomorphic iff equal.
    Canon {
        #[arg(long)]
        input: String,
    },
    /// Validate + round-trip a graph6 string
    G6 {
        #[arg(long)]
        decode: Option<String>,
    },
    /// Exact Hadwiger number h(G) (K_t minor search; branch-set for n <= 10, or --sat for any n <= 64)
    Hadwiger {
        #[arg(long)]
        input: String,
        #[arg(long)]
        json: bool,
        /// Use CDCL SAT minor solver (supports n > 10)
        #[arg(long)]
        sat: bool,
    },
    /// Run a campaign (TOML) over its declared domain
    Search {
        #[arg(long)]
        campaign: String,
        #[arg(long)]
        output: Option<String>,
        /// override thread budget (still capped at 48 host default)
        #[arg(long)]
        threads: Option<usize>,
    },
    /// Minimize a witness file while preserving its violation
    Minimize {
        #[arg(long)]
        witness: String,
        #[arg(long)]
        output: Option<String>,
    },
    /// Independently recheck a witness file
    Verify {
        #[arg(long)]
        witness: String,
    },
    /// CPU/GPU benchmarks: raw JSON to stdout or --output file
    Benchmark {
        #[arg(long, default_value_t = 48)]
        threads: usize,
        /// probe GPU suitability (occupancy gate) without launching work
        #[arg(long)]
        gpu_if_capable: bool,
        #[arg(long)]
        output: Option<String>,
    },
}

#[derive(Serialize)]
struct Doctor {
    engine_version: &'static str,
    rust: String,
    max_vertices: usize,
    exhaustive_enum_max_n: usize,
    canon_fallback_max_vertices: usize,
    cpu_threads_available: usize,
    cpu_threads_default_budget: usize,
    cores: usize,
    memory_total_gib: u64,
    free_disk_repo_gib: u64,
    gpu_tools: GpuTools,
    external_backends: ExternalBackends,
}

#[derive(Serialize)]
struct GpuTools {
    nvidia_smi: bool,
    nvcc: Option<String>,
}

#[derive(Serialize)]
struct ExternalBackends {
    /// nauty/Traces arrive vendored via nauty-pet (compiled in); this reports
    /// optional *system* cross-check binaries only.
    lean: bool,
    z3: bool,
    cvc5: bool,
}

fn have(prog: &str) -> bool {
    std::process::Command::new(prog)
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok()
}

fn nvcc_version() -> Option<String> {
    let out = std::process::Command::new("nvcc")
        .arg("--version")
        .output()
        .ok()?;
    let s = String::from_utf8_lossy(&out.stdout);
    s.lines()
        .find(|l| l.contains("release"))
        .map(|l| l.trim().to_string())
}

fn mem_total_gib() -> u64 {
    std::fs::read_to_string("/proc/meminfo")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("MemTotal:"))
                .and_then(|l| l.split_whitespace().nth(1))
                .and_then(|k| k.parse::<u64>().ok())
        })
        .map(|kib| kib / 1024 / 1024)
        .unwrap_or(0)
}

fn fsfree_gib(path: &str) -> u64 {
    std::process::Command::new("df")
        .arg("-BG")
        .arg(path)
        .output()
        .ok()
        .and_then(|o| {
            let s = String::from_utf8_lossy(&o.stdout).to_string();
            let line = s.lines().nth(1)?;
            let avail = line.split_whitespace().nth(3)?;
            avail.trim_end_matches('G').parse().ok()
        })
        .unwrap_or(0)
}

fn rustc_version() -> String {
    std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".into())
}

fn cmd_doctor(json: bool) -> Result<()> {
    let cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let d = Doctor {
        engine_version: env!("CARGO_PKG_VERSION"),
        rust: rustc_version(),
        max_vertices: graphlab_core::MAX_VERTICES,
        exhaustive_enum_max_n: enumerate::EXHAUSTIVE_MAX_N,
        canon_fallback_max_vertices: canon::FALLBACK_MAX_VERTICES,
        cpu_threads_available: cores,
        cpu_threads_default_budget: (cores * 3 / 4).max(1),
        cores,
        memory_total_gib: mem_total_gib(),
        free_disk_repo_gib: fsfree_gib("."),
        gpu_tools: GpuTools {
            nvidia_smi: have("nvidia-smi"),
            nvcc: nvcc_version(),
        },
        external_backends: ExternalBackends {
            lean: have("lean"),
            z3: have("z3"),
            cvc5: have("cvc5"),
        },
    };
    if json {
        println!("{}", serde_json::to_string_pretty(&d)?);
    } else {
        println!(
            "graphlab v{} — caps: enum n<={}, canon fallback n<={}, graph n<={}",
            d.engine_version,
            d.exhaustive_enum_max_n,
            d.canon_fallback_max_vertices,
            d.max_vertices
        );
        println!(
            "  lean:{} z3:{} cvc5:{} nvidia-smi:{} nvcc:{}",
            d.external_backends.lean,
            d.external_backends.z3,
            d.external_backends.cvc5,
            d.gpu_tools.nvidia_smi,
            d.gpu_tools.nvcc.as_deref().unwrap_or("absent")
        );
    }
    Ok(())
}

fn cmd_hash(path: &str) -> Result<()> {
    let bytes = std::fs::read(path).with_context(|| format!("reading {path}"))?;
    let mut h = Sha256::new();
    h.update(&bytes);
    println!("{}", hex::encode(h.finalize()));
    Ok(())
}

fn parse_graph_input(spec: &str) -> Result<graphlab_core::Graph> {
    let raw = if spec == "-" {
        let mut s = String::new();
        std::io::Read::read_to_string(&mut std::io::stdin(), &mut s)?;
        s
    } else if std::path::Path::new(spec).exists() {
        std::fs::read_to_string(spec).with_context(|| format!("reading {spec}"))?
    } else {
        // inline graph6 string
        spec.to_string()
    };
    let line = raw
        .lines()
        .map(|l| l.trim())
        .find(|l| !l.is_empty())
        .context("no graph6 string found in input")?;
    g6::decode_graph6(line).with_context(|| format!("decoding graph6 '{line}'"))
}

fn cmd_enumerate(vertices: usize, output: Option<String>) -> Result<()> {
    let graphs = match enumerate::enumerate_unlabeled(vertices) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("graphlab enumerate: {e}");
            std::process::exit(3);
        }
    };
    let mut out: Box<dyn std::io::Write> = match &output {
        Some(p) => Box::new(std::fs::File::create(p).with_context(|| format!("creating {p}"))?),
        None => Box::new(std::io::stdout().lock()),
    };
    for g in &graphs {
        let iv = invariants::all_invariants(g);
        let s = g6::encode_graph6(g);
        let line = serde_json::json!({"graph6": s, "invariants": iv});
        writeln!(out, "{line}")?;
    }
    let expect = enumerate::KNOWN_UNLABELED_COUNTS
        .get(vertices)
        .map_or("?".to_string(), |c| c.to_string());
    eprintln!(
        "enumerated {} unlabeled graphs on {vertices} vertices (known count: {expect})",
        graphs.len()
    );
    if enumerate::KNOWN_UNLABELED_COUNTS.get(vertices) == Some(&graphs.len()) {
        eprintln!("count matches known sequence [EXACT_FINITE]");
    } else {
        eprintln!("WARNING: count does NOT match known sequence");
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Doctor { json } => cmd_doctor(json),
        Cmd::Hash { path } => cmd_hash(&path),
        Cmd::Enumerate { vertices, output } => cmd_enumerate(vertices, output),
        Cmd::Invariants { input, json } => {
            let g = parse_graph_input(&input)?;
            let iv = invariants::all_invariants(&g);
            if json {
                println!("{}", serde_json::to_string_pretty(&iv)?);
            } else {
                println!(
                    "n={} m={} deg={} comp={} conn={} tri={} c4={} omega={} alpha={} chi={} density={}/{}",
                    iv.n,
                    iv.m,
                    serde_json::to_string(&iv.degree_sequence)?,
                    iv.components,
                    iv.connected,
                    iv.triangles,
                    iv.cycle4,
                    iv.clique_number,
                    iv.independence_number,
                    iv.chromatic_number,
                    iv.density_num,
                    iv.density_den
                );
            }
            Ok(())
        }
        Cmd::Canon { input } => {
            let g = parse_graph_input(&input)?;
            let bytes = match canon::canonical_bytes(&g) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("graphlab canon: {e}");
                    std::process::exit(4);
                }
            };
            println!("{}", hex::encode(bytes));
            Ok(())
        }
        Cmd::G6 { decode } => {
            let s = decode.context("provide --decode <graph6>")?;
            let s = s.trim_end_matches('\n').to_string();
            let g = match g6::decode_graph6(&s) {
                Ok(g) => g,
                Err(e) => {
                    eprintln!("invalid graph6: {e}");
                    std::process::exit(5);
                }
            };
            let re = g6::encode_graph6(&g);
            if re != s {
                eprintln!("graphlab g6: round-trip mismatch: in={s} out={re}");
                std::process::exit(6);
            }
            println!("{re}");
            Ok(())
        }
        Cmd::Hadwiger { input, json, sat } => {
            let g = parse_graph_input(&input)?;
            let (h, model) = if sat || g.num_vertices() > graphlab_core::hadwiger::HADWIGER_MAX_N {
                let h = match graphlab_core::hadwiger_sat::hadwiger_number_sat(&g) {
                    Ok(h) => h,
                    Err(e) => {
                        eprintln!("graphlab hadwiger (sat): {e}");
                        std::process::exit(7);
                    }
                };
                let m = graphlab_core::hadwiger_sat::find_branch_model_sat(&g, h)
                    .ok()
                    .flatten();
                (h, m)
            } else {
                let h = match graphlab_core::hadwiger::hadwiger_number(&g) {
                    Ok(h) => h,
                    Err(e) => {
                        eprintln!("graphlab hadwiger: {e}");
                        std::process::exit(7);
                    }
                };
                let m = graphlab_core::hadwiger::find_branch_model(&g, h);
                (h, m)
            };
            if json {
                let v = serde_json::json!({
                    "graph6": g6::encode_graph6(&g),
                    "hadwiger": h,
                    "branch_model": model,
                    "chi": graphlab_core::invariants::chromatic_number(&g),
                    "n": g.num_vertices(),
                    "m": g.edge_count(),
                    "solver": if sat || g.num_vertices() > graphlab_core::hadwiger::HADWIGER_MAX_N { "sat_cdcl" } else { "branch_set_exact" },
                });
                println!("{}", serde_json::to_string_pretty(&v)?);
            } else {
                println!("{h}");
            }
            Ok(())
        }
        Cmd::Search {
            campaign,
            output,
            threads,
        } => {
            let text = std::fs::read_to_string(&campaign)
                .with_context(|| format!("reading {campaign}"))?;
            let c = match graphlab_core::campaign::Campaign::from_toml_str(&text) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("graphlab search: {e}");
                    std::process::exit(8);
                }
            };
            let pred = match graphlab_core::predicates::predicate_from_str(&c.predicate) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("graphlab search: {e}");
                    std::process::exit(8);
                }
            };
            let threads = threads.or(c.threads).unwrap_or(48).min(48);
            let out = output.unwrap_or_else(|| format!("results/campaigns/{}.jsonl", c.name));
            if let Some(dir) = std::path::Path::new(&out).parent() {
                std::fs::create_dir_all(dir)?;
            }
            match graphlab_core::campaign::run(&c, pred.as_ref(), threads, &out) {
                Ok(summary) => {
                    println!("{}", serde_json::to_string_pretty(&summary)?);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("graphlab search: {e}");
                    std::process::exit(9);
                }
            }
        }
        Cmd::Minimize { witness, output } => {
            let w = load_first_witness(&witness)?;
            let m = match graphlab_core::minimize::minimize(&w) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("graphlab minimize: {e}");
                    std::process::exit(10);
                }
            };
            let out = output.unwrap_or_else(|| "-".into());
            let body = serde_json::to_string_pretty(&m)?;
            if out == "-" {
                println!("{body}");
            } else {
                std::fs::write(&out, body)?;
                eprintln!("minimized witness written to {out}");
            }
            Ok(())
        }
        Cmd::Verify { witness } => {
            let w = load_first_witness(&witness)?;
            match graphlab_core::verify::independent_report(&w) {
                Ok(v) => {
                    println!("{}", serde_json::to_string_pretty(&v)?);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("graphlab verify: FAIL — {e}");
                    std::process::exit(11);
                }
            }
        }
        Cmd::Benchmark {
            threads,
            gpu_if_capable,
            output,
        } => {
            let mut report = serde_json::json!({
                "cpu": {
                    "threads": threads,
                    "benches": [
                        bench::bench_enumerate_invariants(threads, 3),
                        bench::bench_labeled_sweep(threads),
                        bench::bench_hadwiger(threads),
                    ],
                },
                "gpu": {"probed": gpu_if_capable, "used": false},
            });
            if gpu_if_capable {
                let occ = gpu_occupancy();
                report["gpu"]["occupancy_percent"] = serde_json::json!(occ);
                report["gpu"]["decision"] = serde_json::json!(
                    if occ.iter().any(|(_, p)| *p > 10) {
                        "skipped: occupancy >10% on one or more cards (production lanes active); CPU-only run is authoritative"
                    } else {
                        "cards look idle; no CUDA kernel is wired yet — GPU remains unused (see docs/GPU_FINDINGS.md)"
                    }
                );
            }
            let body = serde_json::to_string_pretty(&report)?;
            match output {
                Some(p) => {
                    if let Some(dir) = std::path::Path::new(&p).parent() {
                        std::fs::create_dir_all(dir)?;
                    }
                    std::fs::write(&p, &body)?;
                    eprintln!("bench written to {p}");
                }
                None => println!("{body}"),
            }
            Ok(())
        }
    }
}

/// query nvidia-smi utilization.gpu per card; None if unavailable
fn gpu_occupancy() -> Vec<(String, u32)> {
    let Ok(out) = std::process::Command::new("nvidia-smi")
        .args([
            "--query-gpu=name,utilization.gpu",
            "--format=csv,noheader,nounits",
        ])
        .output()
    else {
        return Vec::new();
    };
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter_map(|l| {
            let mut parts = l.splitn(2, ',');
            let name = parts.next()?.trim().to_string();
            let pct: u32 = parts.next()?.trim().parse().ok()?;
            Some((name, pct))
        })
        .collect()
}

/// Load the first non-manifest, non-rejected witness line from a JSONL file.
fn load_first_witness(path: &str) -> Result<graphlab_core::search::Witness> {
    use graphlab_core::search::Witness;
    let text = std::fs::read_to_string(path).with_context(|| format!("reading {path}"))?;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let v: serde_json::Value = serde_json::from_str(line)
            .with_context(|| format!("bad JSONL line: {}", &line[..line.len().min(80)]))?;
        if v.get("rejected").is_some() || v.get("manifest").is_some() {
            continue;
        }
        let w: Witness =
            serde_json::from_value(v).with_context(|| "line is not a graphlab-witness/1 object")?;
        return Ok(w);
    }
    anyhow::bail!("no witnesses found in {path}")
}

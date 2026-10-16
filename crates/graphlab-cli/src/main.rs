//! graphlab CLI.
//!
//! Commands implemented today (M0): `doctor`.
//! Planned commands return a clean `not implemented` error with exit code 3
//! — they are advertised in help only once they actually work.

use clap::{Parser, Subcommand};
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
    /// Report host/toolchain/engine capabilities as JSON
    Doctor {
        #[arg(long)]
        json: bool,
    },
    /// Print SHA-256 of a file (used by the evidence ledger)
    Hash { path: String },
}

#[derive(Serialize)]
struct Doctor {
    engine_version: &'static str,
    rust: String,
    max_vertices: usize,
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
    /// nauty/Traces arrive with the nauty-pet crate (compiled in), so this
    /// reports the *system* binaries only (optional cross-check).
    system_nauty_drg: bool,
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
        || std::process::Command::new(prog)
            .arg("-version")
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

fn cmd_doctor(json: bool) -> anyhow::Result<()> {
    let cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let budget = (cores * 3 / 4).max(1);
    let disk = fsfree_gib(".");
    let d = Doctor {
        engine_version: env!("CARGO_PKG_VERSION"),
        rust: rustc_version(),
        max_vertices: graphlab_core::MAX_VERTICES,
        cpu_threads_available: cores,
        cpu_threads_default_budget: budget,
        cores,
        memory_total_gib: mem_total_gib(),
        free_disk_repo_gib: disk,
        gpu_tools: GpuTools {
            nvidia_smi: have("nvidia-smi"),
            nvcc: nvcc_version(),
        },
        external_backends: ExternalBackends {
            system_nauty_drg: have("drg"),
            lean: have("lean"),
            z3: have("z3"),
            cvc5: have("cvc5"),
        },
    };
    if json {
        println!("{}", serde_json::to_string_pretty(&d)?);
    } else {
        println!("graphlab doctor");
        println!(
            "  engine          : v{} (rust {})",
            d.engine_version, d.rust
        );
        println!("  max vertices    : {}", d.max_vertices);
        println!(
            "  cpu threads     : {} (budget {})",
            d.cpu_threads_available, d.cpu_threads_default_budget
        );
        println!("  memory          : {} GiB", d.memory_total_gib);
        println!("  free disk (repo): {} GiB", d.free_disk_repo_gib);
        println!("  nvidia-smi      : {}", d.gpu_tools.nvidia_smi);
        println!(
            "  nvcc            : {}",
            d.gpu_tools.nvcc.as_deref().unwrap_or("absent")
        );
        println!("  lean            : {}", d.external_backends.lean);
        println!("  z3              : {}", d.external_backends.z3);
        println!("  cvc5            : {}", d.external_backends.cvc5);
    }
    Ok(())
}

fn rustc_version() -> String {
    std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".into())
}

fn fsfree_gib(path: &str) -> u64 {
    // statvfs via libc-free approach: parse `df` once.
    let out = std::process::Command::new("df")
        .arg("-BG")
        .arg(path)
        .output()
        .ok();
    out.and_then(|o| {
        let s = String::from_utf8_lossy(&o.stdout).to_string();
        let line = s.lines().nth(1)?;
        let avail = line.split_whitespace().nth(3)?;
        avail.trim_end_matches('G').parse().ok()
    })
    .unwrap_or(0)
}

fn cmd_hash(path: &str) -> anyhow::Result<()> {
    let bytes = std::fs::read(path)?;
    let mut h = Sha256::new();
    h.update(&bytes);
    println!("{}", hex::encode(h.finalize()));
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Doctor { json } => cmd_doctor(json),
        Cmd::Hash { path } => cmd_hash(&path),
    }
}

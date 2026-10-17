//! CLI integration tests: every advertised command must run with trustworthy
//! exit codes, including the failure modes the commission requires.

use std::process::Command;

fn bin() -> String {
    let mut p = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    p.push("graphlab");
    p.to_string_lossy().to_string()
}

fn args(a: &[&str]) -> std::process::Output {
    Command::new(bin()).args(a).output().expect("cli runs")
}

#[test]
fn doctor_json_parses() {
    let o = args(&["doctor", "--json"]);
    assert!(o.status.success(), "{:?}", o);
    let v: serde_json::Value = serde_json::from_slice(&o.stdout).unwrap();
    assert_eq!(v["exhaustive_enum_max_n"], 8);
}

#[test]
fn enumerate_all_levels_succeed_and_n7_is_1044_lines() {
    for n in 0..=7usize {
        let o = args(&["enumerate", "--vertices", &n.to_string()]);
        assert!(o.status.success(), "n={n}: {:?}", o);
        let lines = String::from_utf8_lossy(&o.stdout).lines().count();
        let expect = [1usize, 1, 2, 4, 11, 34, 156, 1044][n];
        assert_eq!(lines, expect, "n={n}");
    }
}

#[test]
fn enumerate_n9_fails_loudly_exit3() {
    let o = args(&["enumerate", "--vertices", "9"]);
    assert_eq!(o.status.code(), Some(3), "{:?}", o);
    let e = String::from_utf8_lossy(&o.stderr);
    assert!(
        e.contains("n <=") || e.contains("exceeds"),
        "stderr should name the limit: {e}"
    );
}

#[test]
fn g6_invalid_exits_5() {
    let o = args(&["g6", "--decode", "!@#"]);
    assert_eq!(o.status.code(), Some(5), "{:?}", o);
}

#[test]
fn invariants_k4_values() {
    let o = args(&["invariants", "--input", "C~", "--json"]);
    assert!(o.status.success(), "{:?}", o);
    let v: serde_json::Value = serde_json::from_slice(&o.stdout).unwrap();
    assert_eq!(v["chromatic_number"], 4);
    assert_eq!(v["triangles"], 4);
}

#[test]
fn hadwiger_petersen_is_5() {
    // petersen graph edges -> build via python-free route: encode by hand
    let o = args(&["hadwiger", "--input", "IheA@GUAo"]); // Petersen graph6
                                                         // if my hand string is wrong this exits 5 (invalid g6) or 7; assert success:
    assert!(
        o.status.success(),
        "graph6 wrong? {:?}",
        String::from_utf8_lossy(&o.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&o.stdout).trim(), "5");
}

#[test]
fn search_baseline_produces_replayable_witnesses() {
    let dir = std::env::temp_dir();
    let out = dir.join("cli-test-baseline.jsonl");
    let _ = std::fs::remove_file(&out);
    let toml = concat!(env!("CARGO_MANIFEST_DIR"), "/../../campaigns/baseline.toml");
    let o = args(&[
        "search",
        "--campaign",
        toml,
        "--output",
        out.to_str().unwrap(),
    ]);
    assert!(
        o.status.success(),
        "{:?}",
        String::from_utf8_lossy(&o.stderr)
    );
    let text = std::fs::read_to_string(&out).unwrap();
    assert_eq!(
        text.lines()
            .filter(|l| l.starts_with(r#"{"schema""#))
            .count(),
        4
    );
    // verify command accepts the first witness
    let o = args(&["verify", "--witness", out.to_str().unwrap()]);
    assert!(o.status.success(), "verify must pass: {:?}", o);
    // minimize the first witness (K1 for canary)
    let o = args(&[
        "minimize",
        "--witness",
        out.to_str().unwrap(),
        "--output",
        "-",
    ]);
    assert!(o.status.success(), "minimize: {:?}", o);
    std::fs::remove_file(&out).ok();
}

#[test]
fn verify_tampered_witness_exits_11() {
    let dir = std::env::temp_dir();
    let f = dir.join("cli-tamper.jsonl");
    std::fs::write(
        &f,
        r#"{"schema":"graphlab-witness/1","campaign":"x","predicate":"canary-chi-le-n-1-v1","graph6":"B_","n":1,"m":0,"reported":{"chi":4,"n":1},"domain_mode":"ExhaustiveUnlabeled","domain_scope":{"n_min":1,"n_max":1,"exhaustive":true,"samples":null},"evidence":"ExactFinite","seed":null,"run_id":"t"}"#,
    )
    .unwrap();
    let o = args(&["verify", "--witness", f.to_str().unwrap()]);
    assert_eq!(
        o.status.code(),
        Some(11),
        "tampered witness must fail verify"
    );
    std::fs::remove_file(&f).ok();
}

#[test]
fn empty_witness_file_is_a_clean_failure_not_a_panic() {
    let dir = std::env::temp_dir();
    let f = dir.join("cli-empty.jsonl");
    std::fs::write(&f, "").unwrap();
    let o = args(&["verify", "--witness", f.to_str().unwrap()]);
    assert!(!o.status.success());
    let code = o.status.code().unwrap();
    assert!(code != 0 && code < 128, "clean typed exit, got {code}");
    std::fs::remove_file(&f).ok();
}

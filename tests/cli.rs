use std::io::Write;
use std::process::{Command, Stdio};

fn run(json: &str, cache_dir: &str, config_dir: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_ccstatus"))
        .env("XDG_CACHE_HOME", cache_dir)
        .env("XDG_CONFIG_HOME", config_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn ccstatus");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(json.as_bytes())
        .unwrap();
    let out = child.wait_with_output().expect("wait");
    assert!(out.status.success(), "exit code should be 0");
    String::from_utf8(out.stdout).unwrap()
}

fn tmp(name: &str) -> String {
    let p = std::env::temp_dir().join(format!("ccstatus_it_{name}"));
    let _ = std::fs::remove_dir_all(&p);
    std::fs::create_dir_all(&p).unwrap();
    p.to_string_lossy().into_owned()
}

const FULL: &str = r#"{
    "model": {"display_name": "Opus 4.8"},
    "context_window": {"used_percentage": 0, "context_window_size": 1000000, "total_input_tokens": 280000, "total_output_tokens": 60000},
    "rate_limits": {
        "five_hour": {"used_percentage": 42, "resets_at": 4102444800},
        "seven_day": {"used_percentage": 18, "resets_at": 4102444800}
    }
}"#;

#[test]
fn renders_header_and_three_rows() {
    let cache = tmp("live_cache");
    let config = tmp("live_config");
    let out = run(FULL, &cache, &config);
    assert_eq!(out.lines().count(), 4); // model header + 3 gauge rows
    assert!(out.contains("Opus 4.8")); // header line
    assert!(out.contains("Context"));
    assert!(out.contains("5h"));
    assert!(out.contains("7d"));
    // Rate-limit rows show used percentage + reset countdown.
    assert!(out.contains("42%"));
    assert!(out.contains("18%"));
    assert!(out.contains("↑280k ↓60k / 1.0m"), "context token detail in: {out}");
    assert!(out.contains("resets in"), "reset countdown in: {out}");
}

#[test]
fn cold_start_uses_cache() {
    let cache = tmp("cold_cache");
    let config = tmp("cold_config");
    // First run seeds the cache from live data (42% / 18% used).
    run(FULL, &cache, &config);
    // Second run has no rate_limits — must fall back to cache.
    let no_limits = r#"{"model":{"display_name":"Opus 4.8"},"context_window":{"context_window_size":1000000}}"#;
    let out = run(no_limits, &cache, &config);
    assert!(out.contains("42%"), "expected cached 42% in: {out}");
    assert!(out.contains("18%"), "expected cached 18% in: {out}");
}

#[test]
fn cold_start_no_cache_shows_dashes() {
    let cache = tmp("empty_cache");
    let config = tmp("empty_config");
    let no_limits = r#"{"model":{"display_name":"Opus 4.8"},"context_window":{"context_window_size":1000000}}"#;
    let out = run(no_limits, &cache, &config);
    // 5h row (line index 2) has no data at all → "--".
    let five = out.lines().nth(2).expect("5h row");
    assert!(five.contains("--"), "expected dashes in 5h row: {five}");
}

#[test]
fn malformed_stdin_exits_zero() {
    let cache = tmp("bad_cache");
    let config = tmp("bad_config");
    let out = run("total garbage", &cache, &config);
    // Header still renders with the default model name.
    assert!(out.contains("Claude"));
}

#[test]
fn persist_preserves_absent_window() {
    let cache = tmp("preserve_cache");
    let config = tmp("preserve_config");
    // Run 1: both windows live (42 / 18) seed the cache.
    run(FULL, &cache, &config);
    // Run 2: only five_hour live (50). seven_day must NOT be wiped.
    let only_five = r#"{"model":{"display_name":"Opus 4.8"},"context_window":{"context_window_size":1000000},"rate_limits":{"five_hour":{"used_percentage":50,"resets_at":4102444800}}}"#;
    run(only_five, &cache, &config);
    // Run 3: neither live — weekly must still show the preserved 18%.
    let no_limits = r#"{"model":{"display_name":"Opus 4.8"},"context_window":{"context_window_size":1000000}}"#;
    let out = run(no_limits, &cache, &config);
    assert!(out.contains("18%"), "weekly cached value must survive a single-window persist: {out}");
    assert!(out.contains("50%"), "current should reflect the updated cached value: {out}");
}

#[test]
fn rolled_over_cached_window_shows_zero() {
    let cache = tmp("rollover_cache");
    let config = tmp("rollover_config");
    // Run 1: live data whose reset is already in the past (2001).
    let past = r#"{"model":{"display_name":"Opus 4.8"},"context_window":{"context_window_size":1000000},"rate_limits":{"five_hour":{"used_percentage":42,"resets_at":1000000000},"seven_day":{"used_percentage":18,"resets_at":1000000000}}}"#;
    run(past, &cache, &config);
    // Run 2: no live data — cached windows have past resets_at, so pct → 0.
    let no_limits = r#"{"model":{"display_name":"Opus 4.8"},"context_window":{"context_window_size":1000000}}"#;
    let out = run(no_limits, &cache, &config);
    // Rolled over -> used pct forced to 0; stale 42% gone.
    let five = out.lines().nth(2).expect("5h row");
    assert!(five.contains("0%"), "rolled-over 5h window should render 0%: {five}");
    assert!(!out.contains("42%"), "stale 42% must not show after rollover: {out}");
}

#[test]
fn null_live_percentage_renders_dashes_not_zero() {
    let cache = tmp("nullpct_cache");
    let config = tmp("nullpct_config");
    // five_hour present but percentage is null; no cache seeded → 5h row must show --, not 0%.
    let json = r#"{"model":{"display_name":"Opus 4.8"},"context_window":{"used_percentage":5,"context_window_size":1000000},"rate_limits":{"five_hour":{"used_percentage":null,"resets_at":4102444800}}}"#;
    let out = run(json, &cache, &config);
    // lines: 0 header, 1 Context, 2 = 5h row.
    let five = out.lines().nth(2).expect("5h row");
    assert!(five.contains("--"), "null-pct window should render -- in 5h row: {five}");
    assert!(!five.contains('%'), "null-pct window must not show a percentage: {five}");
}

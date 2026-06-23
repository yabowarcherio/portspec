//! Black-box tests for the `portspec` binary.

use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_portspec"))
}

#[test]
fn expands_ports_by_default() {
    let out = bin().arg("22,8000-8002").output().unwrap();
    assert!(out.status.success());
    let s = String::from_utf8(out.stdout).unwrap();
    assert_eq!(
        s.lines().collect::<Vec<_>>(),
        vec!["22", "8000", "8001", "8002"]
    );
}

#[test]
fn count_flag_prints_number() {
    let out = bin().args(["--count", "1-1024"]).output().unwrap();
    let s = String::from_utf8(out.stdout).unwrap();
    assert_eq!(s.trim(), "1024");
}

#[test]
fn ranges_flag_prints_normalized() {
    let out = bin()
        .args(["--ranges", "80,1-10,11-20,80"])
        .output()
        .unwrap();
    let s = String::from_utf8(out.stdout).unwrap();
    assert_eq!(s.trim(), "1-20,80");
}

#[test]
fn union_of_multiple_specs() {
    let out = bin().args(["--ranges", "1-10", "20-30"]).output().unwrap();
    let s = String::from_utf8(out.stdout).unwrap();
    assert_eq!(s.trim(), "1-10,20-30");
}

#[test]
fn intersect_flag() {
    let out = bin()
        .args(["--ranges", "--intersect", "50-150", "1-100"])
        .output()
        .unwrap();
    let s = String::from_utf8(out.stdout).unwrap();
    assert_eq!(s.trim(), "50-100");
}

#[test]
fn contains_hit_exits_zero() {
    let out = bin()
        .args(["--contains", "443", "1-1024"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
}

#[test]
fn contains_miss_exits_one() {
    let out = bin().args(["--contains", "70", "22,80"]).output().unwrap();
    assert_eq!(out.status.code(), Some(1));
}

#[test]
fn reverse_and_limit() {
    let out = bin()
        .args(["-R", "--limit", "3", "1-100"])
        .output()
        .unwrap();
    let s = String::from_utf8(out.stdout).unwrap();
    assert_eq!(s.lines().collect::<Vec<_>>(), vec!["100", "99", "98"]);
}

#[test]
fn bad_spec_exits_two() {
    let out = bin().arg("99999").output().unwrap();
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn difference_flag() {
    let out = bin()
        .args(["--ranges", "--difference", "20-30", "1-100"])
        .output()
        .unwrap();
    let s = String::from_utf8(out.stdout).unwrap();
    assert_eq!(s.trim(), "1-19,31-100");
}

#[test]
fn invert_flag() {
    let out = bin()
        .args(["--ranges", "--invert", "1-65535"])
        .output()
        .unwrap();
    let s = String::from_utf8(out.stdout).unwrap();
    assert_eq!(s.trim(), "0");
}

#[test]
fn tagged_flag_emits_proto_prefixed_lines() {
    let out = bin().args(["--tagged", "T:22,80,U:53,123"]).output().unwrap();
    assert!(out.status.success());
    let s = String::from_utf8(out.stdout).unwrap();
    let lines: Vec<&str> = s.lines().collect();
    assert_eq!(lines, ["tcp 22", "tcp 80", "udp 53", "udp 123"]);
}

#[test]
fn tagged_flag_default_proto_is_tcp() {
    let out = bin().args(["--tagged", "22,80,U:53"]).output().unwrap();
    let s = String::from_utf8(out.stdout).unwrap();
    let lines: Vec<&str> = s.lines().collect();
    assert_eq!(lines, ["tcp 22", "tcp 80", "udp 53"]);
}

#[test]
fn tagged_flag_bad_spec_exits_two() {
    let out = bin().args(["--tagged", "T:notaport"]).output().unwrap();
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn resolve_flag_appends_service_names() {
    let out = bin().args(["--resolve", "22,80,443,9999"]).output().unwrap();
    assert!(out.status.success());
    let s = String::from_utf8(out.stdout).unwrap();
    let lines: Vec<&str> = s.lines().collect();
    assert_eq!(lines[0], "22\tssh");
    assert_eq!(lines[1], "80\thttp");
    assert_eq!(lines[2], "443\thttps");
    // 9999 is not in the built-in table; the name column is empty (just TAB).
    assert_eq!(lines[3], "9999\t");
}

#[test]
fn preset_top_100_replaces_input() {
    let out = bin().args(["--preset", "top-100", "--count"]).output().unwrap();
    assert!(out.status.success());
    let n: u32 = String::from_utf8(out.stdout).unwrap().trim().parse().unwrap();
    assert!((90..=170).contains(&n), "preset top-100 count out of band: {n}");
}

#[test]
fn preset_top_1000_is_larger_than_top_100() {
    let small = bin()
        .args(["--preset", "top-100", "--count"])
        .output()
        .unwrap();
    let big = bin()
        .args(["--preset", "top-1000", "--count"])
        .output()
        .unwrap();
    let s: u32 = String::from_utf8(small.stdout).unwrap().trim().parse().unwrap();
    let b: u32 = String::from_utf8(big.stdout).unwrap().trim().parse().unwrap();
    assert!(b > s);
}

#[test]
fn preset_unknown_exits_two() {
    let out = bin().args(["--preset", "nope"]).output().unwrap();
    assert_eq!(out.status.code(), Some(2));
    let err = String::from_utf8(out.stderr).unwrap();
    assert!(err.contains("unknown preset"), "stderr: {err}");
}

#[test]
fn json_flag_emits_summary() {
    let out = bin().args(["--json", "80,1-10,11-20"]).output().unwrap();
    assert!(out.status.success());
    let s = String::from_utf8(out.stdout).unwrap();
    let v: serde_json::Value = serde_json::from_str(&s).unwrap();
    assert_eq!(v["spec"], "1-20,80");
    assert_eq!(v["count"], 21);
    assert_eq!(v["ranges"][0][0], 1);
    assert_eq!(v["ranges"][0][1], 20);
}

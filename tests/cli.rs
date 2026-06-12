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

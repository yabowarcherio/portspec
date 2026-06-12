//! Integration tests for `PortRange`.

use portspec::{ParseError, PortRange};

#[test]
fn parses_single_and_range() {
    let single: PortRange = "80".parse().unwrap();
    assert_eq!(single, PortRange::single(80));
    assert_eq!(single.count(), 1);

    let range: PortRange = "1000-2000".parse().unwrap();
    assert_eq!(range.start(), 1000);
    assert_eq!(range.end(), 2000);
    assert_eq!(range.count(), 1001);
}

#[test]
fn parses_open_ended_forms() {
    assert_eq!(
        "1024-".parse::<PortRange>().unwrap(),
        PortRange::new(1024, 65535).unwrap()
    );
    assert_eq!(
        "-1024".parse::<PortRange>().unwrap(),
        PortRange::new(0, 1024).unwrap()
    );
    assert_eq!("-".parse::<PortRange>().unwrap(), PortRange::FULL);
}

#[test]
fn rejects_bad_input() {
    assert_eq!("".parse::<PortRange>().unwrap_err(), ParseError::Empty);
    assert!(matches!(
        "70000".parse::<PortRange>().unwrap_err(),
        ParseError::BadPort(_)
    ));
    assert_eq!(
        "200-100".parse::<PortRange>().unwrap_err(),
        ParseError::StartAfterEnd
    );
}

#[test]
fn contains_and_overlaps() {
    let r: PortRange = "100-200".parse().unwrap();
    assert!(r.contains(100) && r.contains(200) && r.contains(150));
    assert!(!r.contains(99) && !r.contains(201));

    let nested: PortRange = "120-180".parse().unwrap();
    let disjoint: PortRange = "300-400".parse().unwrap();
    assert!(r.overlaps(&nested) && r.contains_range(&nested));
    assert!(!r.overlaps(&disjoint));
}

#[test]
fn merge_adjacent_and_overlapping() {
    let a: PortRange = "1-10".parse().unwrap();
    let b: PortRange = "11-20".parse().unwrap(); // adjacent
    let c: PortRange = "5-15".parse().unwrap(); // overlapping
    let d: PortRange = "30-40".parse().unwrap(); // disjoint
    assert_eq!(a.merge(&b), Some(PortRange::new(1, 20).unwrap()));
    assert_eq!(a.merge(&c), Some(PortRange::new(1, 15).unwrap()));
    assert_eq!(a.merge(&d), None);
}

#[test]
fn iterates_both_directions() {
    let r: PortRange = "65533-65535".parse().unwrap();
    let fwd: Vec<u16> = r.iter().collect();
    assert_eq!(fwd, [65533, 65534, 65535]);
    let rev: Vec<u16> = r.iter().rev().collect();
    assert_eq!(rev, [65535, 65534, 65533]);
    assert_eq!(r.iter().len(), 3);
}

#[test]
fn display_round_trips() {
    for s in ["80", "1000-2000"] {
        let r: PortRange = s.parse().unwrap();
        assert_eq!(r.to_string(), s);
    }
}

//! Integration tests for `PortSpec`.

use portspec::{ParseError, PortRange, PortSpec};

#[test]
fn parses_and_normalizes() {
    let spec: PortSpec = "80,22,443".parse().unwrap();
    // Sorted on parse.
    assert_eq!(
        spec.ranges(),
        &[
            PortRange::single(22),
            PortRange::single(80),
            PortRange::single(443),
        ]
    );
    assert_eq!(spec.count(), 3);
}

#[test]
fn merges_overlapping_and_adjacent() {
    let spec: PortSpec = "1-10,11-20,5-15,100".parse().unwrap();
    // 1-10, 11-20 and 5-15 all collapse into 1-20.
    assert_eq!(
        spec.ranges(),
        &[PortRange::new(1, 20).unwrap(), PortRange::single(100),]
    );
    assert_eq!(spec.count(), 21);
}

#[test]
fn dedupes_repeated_ports() {
    let spec: PortSpec = "80,80,80".parse().unwrap();
    assert_eq!(spec, "80".parse().unwrap());
    assert_eq!(spec.count(), 1);
}

#[test]
fn skips_empty_items() {
    let spec: PortSpec = "22, ,80,".parse().unwrap();
    assert_eq!(spec.count(), 2);
}

#[test]
fn rejects_empty() {
    assert_eq!("".parse::<PortSpec>().unwrap_err(), ParseError::Empty);
    assert_eq!(",,".parse::<PortSpec>().unwrap_err(), ParseError::Empty);
}

#[test]
fn contains_uses_ranges() {
    let spec: PortSpec = "20-25,80,8000-8100".parse().unwrap();
    assert!(spec.contains(22) && spec.contains(80) && spec.contains(8050));
    assert!(!spec.contains(26) && !spec.contains(79) && !spec.contains(9000));
}

#[test]
fn iter_is_sorted_and_unique() {
    let spec: PortSpec = "8000-8002,22,22".parse().unwrap();
    let ports: Vec<u16> = spec.iter().collect();
    assert_eq!(ports, [22, 8000, 8001, 8002]);
}

#[test]
fn union_and_intersection() {
    let a: PortSpec = "1-100".parse().unwrap();
    let b: PortSpec = "50-150".parse().unwrap();
    assert_eq!(a.union(&b), "1-150".parse().unwrap());
    assert_eq!(a.intersection(&b), "50-100".parse().unwrap());

    let disjoint: PortSpec = "200-300".parse().unwrap();
    assert!(a.intersection(&disjoint).is_empty());
}

#[test]
fn display_round_trips_normalized() {
    let spec: PortSpec = "443,1-10,11-20".parse().unwrap();
    assert_eq!(spec.to_string(), "1-20,443");
}

#[test]
fn difference_subtracts_ports() {
    let a: PortSpec = "1-100".parse().unwrap();
    let b: PortSpec = "20-30,50".parse().unwrap();
    assert_eq!(a.difference(&b), "1-19,31-49,51-100".parse().unwrap());

    // Removing everything yields empty.
    assert!(a.difference(&"1-100".parse().unwrap()).is_empty());
    // Removing a disjoint spec is a no-op.
    assert_eq!(a.difference(&"200-300".parse().unwrap()), a);
}

#[test]
fn difference_across_max_port() {
    let a: PortSpec = "65530-65535".parse().unwrap();
    let b: PortSpec = "65535".parse().unwrap();
    assert_eq!(a.difference(&b), "65530-65534".parse().unwrap());
}

#[test]
fn overlaps_subset_contains() {
    let big: PortSpec = "1-1000".parse().unwrap();
    let small: PortSpec = "100-200".parse().unwrap();
    let partial: PortSpec = "900-1100".parse().unwrap();
    let disjoint: PortSpec = "2000-3000".parse().unwrap();

    assert!(big.overlaps(&small) && big.overlaps(&partial));
    assert!(!big.overlaps(&disjoint));
    assert!(small.is_subset_of(&big) && big.contains_spec(&small));
    assert!(!partial.is_subset_of(&big));
    assert!(big.is_subset_of(&big));
}

#[test]
fn insert_and_remove() {
    let mut spec: PortSpec = "1-10".parse().unwrap();
    spec.insert(PortRange::new(11, 20).unwrap());
    assert_eq!(spec, "1-20".parse().unwrap()); // merged

    spec.insert(PortRange::single(100));
    assert_eq!(spec.count(), 21);

    spec.remove(PortRange::new(5, 15).unwrap());
    assert_eq!(spec, "1-4,16-20,100".parse().unwrap());
}

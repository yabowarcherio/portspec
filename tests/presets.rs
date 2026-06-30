//! Tests for the built-in port presets.

use portspec::{top_1000_tcp, top_100_tcp, TOP_1000_TCP_LIST, TOP_100_TCP_LIST};

#[test]
fn presets_parse_cleanly() {
    let _ = top_100_tcp();
    let _ = top_1000_tcp();
}

#[test]
fn list_constants_round_trip_through_parse() {
    use portspec::PortSpec;
    let small: PortSpec = TOP_100_TCP_LIST.parse().unwrap();
    assert_eq!(small, top_100_tcp());
    let big: PortSpec = TOP_1000_TCP_LIST.parse().unwrap();
    assert_eq!(big, top_1000_tcp());
}

#[test]
fn top_1000_is_a_superset_of_top_100() {
    let small = top_100_tcp();
    let big = top_1000_tcp();
    assert!(small.is_subset_of(&big));
    // And it's strictly larger.
    assert!(big.count() > small.count());
}

#[test]
fn presets_normalize_and_dedup() {
    // Whatever the lists look like in source, the PortSpec invariant
    // forbids overlapping ranges — so every range must be disjoint.
    let spec = top_1000_tcp();
    let ranges = spec.ranges();
    for win in ranges.windows(2) {
        assert!(
            win[0].end() < win[1].start(),
            "ranges should be disjoint and sorted: {:?} vs {:?}",
            win[0],
            win[1],
        );
    }
}

#[test]
fn preset_lookup_accepts_case_and_dash_variants() {
    use portspec::{preset, top_1000_tcp, top_100_tcp};
    assert_eq!(preset("top-100").unwrap(), top_100_tcp());
    assert_eq!(preset("TOP100").unwrap(), top_100_tcp());
    assert_eq!(preset("Top-1000").unwrap(), top_1000_tcp());
    assert_eq!(preset("top1000").unwrap(), top_1000_tcp());
}

#[test]
fn preset_lookup_unknown_name_is_error() {
    use portspec::{preset, ParseError};
    let err = preset("nope").unwrap_err();
    assert!(matches!(err, ParseError::Malformed(_)));
}

#[test]
fn common_services_are_covered_by_both_presets() {
    let small = top_100_tcp();
    let big = top_1000_tcp();
    for p in [22, 25, 53, 80, 443] {
        assert!(small.contains(p), "small missing tcp/{p}");
        assert!(big.contains(p), "big missing tcp/{p}");
    }
}

//! Tests for [`TaggedSpec`] — split TCP/UDP port specs.

use portspec::{Proto, TaggedSpec};

#[test]
fn parses_basic_t_u_split() {
    let t: TaggedSpec = "T:22,80,U:53,123".parse().unwrap();
    assert_eq!(t.tcp.count(), 2);
    assert_eq!(t.udp.count(), 2);
    assert!(t.contains(Proto::Tcp, 22));
    assert!(t.contains(Proto::Tcp, 80));
    assert!(t.contains(Proto::Udp, 53));
    assert!(t.contains(Proto::Udp, 123));
    // Cross-transport: 80 is TCP, not UDP.
    assert!(!t.contains(Proto::Udp, 80));
}

#[test]
fn default_prefix_is_tcp() {
    // No leading T:, so the first ports are TCP.
    let t: TaggedSpec = "22,80,U:53".parse().unwrap();
    assert_eq!(t.tcp.count(), 2);
    assert_eq!(t.udp.count(), 1);
    assert!(t.contains(Proto::Tcp, 22));
    assert!(t.contains(Proto::Udp, 53));
}

#[test]
fn prefix_switches_carry_forward() {
    // Once T: or U: appears, subsequent entries inherit it until another prefix
    let t: TaggedSpec = "U:53,123,T:80,443".parse().unwrap();
    assert_eq!(t.udp.count(), 2);
    assert_eq!(t.tcp.count(), 2);
    assert!(t.contains(Proto::Udp, 123));
    assert!(t.contains(Proto::Tcp, 443));
}

#[test]
fn parses_ranges_per_transport() {
    let t: TaggedSpec = "T:8000-8002,U:5000-5001".parse().unwrap();
    assert_eq!(t.tcp.count(), 3);
    assert_eq!(t.udp.count(), 2);
}

#[test]
fn empty_input_is_error() {
    let err = "".parse::<TaggedSpec>().unwrap_err();
    use portspec::ParseError;
    assert!(matches!(err, ParseError::Empty));
}

#[test]
fn count_sums_both_transports() {
    let t: TaggedSpec = "T:22,U:22".parse().unwrap();
    // Same port number, but tcp/22 and udp/22 are distinct entries.
    assert_eq!(t.count(), 2);
}

#[test]
fn display_round_trips_through_parse() {
    let t: TaggedSpec = "T:22,80,U:53".parse().unwrap();
    let s = t.to_string();
    let back: TaggedSpec = s.parse().unwrap();
    assert_eq!(t, back);
}

#[test]
fn empty_protocol_section_is_handled() {
    // Bare `T:` switches the default but emits nothing.
    let t: TaggedSpec = "T:,U:53".parse().unwrap();
    assert!(t.tcp.is_empty());
    assert_eq!(t.udp.count(), 1);
}

#[test]
fn for_proto_returns_matching_spec() {
    let t: TaggedSpec = "T:22,U:53".parse().unwrap();
    assert_eq!(t.for_proto(Proto::Tcp).count(), 1);
    assert_eq!(t.for_proto(Proto::Udp).count(), 1);
}

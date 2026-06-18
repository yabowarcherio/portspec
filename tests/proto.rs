//! Tests for the [`Proto`] enum.

use portspec::{ParseError, Proto};

#[test]
fn parses_canonical_names() {
    assert_eq!("tcp".parse::<Proto>().unwrap(), Proto::Tcp);
    assert_eq!("udp".parse::<Proto>().unwrap(), Proto::Udp);
}

#[test]
fn parses_case_variants_and_single_letter() {
    for s in ["TCP", "Tcp", "T", "t"] {
        assert_eq!(s.parse::<Proto>().unwrap(), Proto::Tcp, "input: {s}");
    }
    for s in ["UDP", "Udp", "U", "u"] {
        assert_eq!(s.parse::<Proto>().unwrap(), Proto::Udp, "input: {s}");
    }
}

#[test]
fn parses_with_surrounding_whitespace() {
    assert_eq!("  tcp  ".parse::<Proto>().unwrap(), Proto::Tcp);
    assert_eq!("\tudp\n".parse::<Proto>().unwrap(), Proto::Udp);
}

#[test]
fn rejects_unknown_names() {
    let err = "icmp".parse::<Proto>().unwrap_err();
    assert!(matches!(err, ParseError::Malformed(_)));
}

#[test]
fn as_str_and_letter() {
    assert_eq!(Proto::Tcp.as_str(), "tcp");
    assert_eq!(Proto::Udp.as_str(), "udp");
    assert_eq!(Proto::Tcp.letter(), 'T');
    assert_eq!(Proto::Udp.letter(), 'U');
}

#[test]
fn display_matches_as_str() {
    assert_eq!(format!("{}", Proto::Tcp), "tcp");
    assert_eq!(format!("{}", Proto::Udp), "udp");
}

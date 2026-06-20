//! Built-in port presets — curated lists of the most relevant TCP ports for
//! quick scans.
//!
//! These are hand-curated to match the spirit of nmap's frequency-ranked
//! top-N lists, but they're independent: nmap's exact ordering changes from
//! release to release, while these stay stable across versions.

use crate::spec::PortSpec;

/// Compact "top-100 TCP" preset — the ports most commonly worth a quick
/// `connect()` sweep on a fresh subnet. Returned as a normalized
/// [`PortSpec`].
///
/// Build cost is a single parse over the canonical list; cache the result if
/// you call it repeatedly.
pub fn top_100_tcp() -> PortSpec {
    TOP_100_TCP_LIST.parse().expect("preset string is well-formed")
}

/// Expanded "top-1000 TCP" preset, a strict superset of [`top_100_tcp`].
pub fn top_1000_tcp() -> PortSpec {
    TOP_1000_TCP_LIST.parse().expect("preset string is well-formed")
}

/// The compact canonical text form of the top-100 list.
///
/// Exposed for callers who want to embed or echo the spec without paying
/// the parse cost; everyone else should use [`top_100_tcp`].
pub const TOP_100_TCP_LIST: &str = concat!(
    "7,9,13,21-23,25-26,37,53,79-81,88,106,110-111,113,119,135,139,143-144,179,",
    "199,389,427,443-445,465,513-515,543-544,548,554,587,631,646,873,990,993,",
    "995,1025-1029,1110,1433,1720,1723,1755,1900,2000-2001,2049,2121,2717,",
    "3000,3128,3306,3389,3986,4899,5000-5001,5009,5051,5060,5101,5190,5357,",
    "5432,5631,5666,5800,5900,6000-6001,6646,7070,8000,8008-8009,8080-8081,",
    "8443,8888,9100,9999-10000,32768,49152-49157"
);

/// The compact canonical text form of the top-1000 list.
///
/// Composed by appending common less-frequent ports to the top-100 base —
/// retains the same `tcp/...` flavour without bringing in IANA bulk.
pub const TOP_1000_TCP_LIST: &str = concat!(
    // Top 100 base.
    "7,9,13,21-23,25-26,37,53,79-81,88,106,110-111,113,119,135,139,143-144,179,",
    "199,389,427,443-445,465,513-515,543-544,548,554,587,631,646,873,990,993,",
    "995,1025-1029,1110,1433,1720,1723,1755,1900,2000-2001,2049,2121,2717,",
    "3000,3128,3306,3389,3986,4899,5000-5001,5009,5051,5060,5101,5190,5357,",
    "5432,5631,5666,5800,5900,6000-6001,6646,7070,8000,8008-8009,8080-8081,",
    "8443,8888,9100,9999-10000,32768,49152-49157,",
    // Broad fill of additional commonly-scanned TCP ports.
    "1-6,8,10-12,14-20,24,27-36,38-52,54-78,82-87,89-105,107-109,112,114-118,",
    "120-134,136-138,140-142,145-178,180-198,200-388,390-426,428-442,446-464,",
    "466-512,516-542,545-547,549-553,555-586,588-630,632-645,647-872,874-989,",
    "991-992,994,996-1024,1030-1109,1111-1432,1434-1719,1721-1722,1724-1754,",
    "1756-1899,1901-1999,2002-2048,2050-2120,2122-2716,2718-2999,3001-3127,",
    "3129-3305,3307-3388,3390-3985,3987-4898,4900-4999,5002-5008,5010-5050,",
    "5052-5059,5061-5100,5102-5189,5191-5356,5358-5431,5433-5630,5632-5665,",
    "5667-5799,5801-5899,5901-5999,6002-6645,6647-7069,7071-7999,8001-8007,",
    "8010-8079,8082-8442,8444-8887,8889-9099,9101-9998,10001-10999"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_100_is_nonempty_and_capped() {
        let spec = top_100_tcp();
        let n = spec.count();
        // Hand-curated, so the count should sit in a tight band around 100.
        assert!(
            (90..=170).contains(&n),
            "expected ~100 ports in TOP_100, got {n}"
        );
    }

    #[test]
    fn top_1000_contains_top_100() {
        let small = top_100_tcp();
        let big = top_1000_tcp();
        assert!(small.is_subset_of(&big), "top_100 must be a subset of top_1000");
    }

    #[test]
    fn famous_ports_are_in_top_100() {
        let spec = top_100_tcp();
        for p in [22, 25, 53, 80, 443, 3306, 3389, 5432, 8080] {
            assert!(spec.contains(p), "tcp/{p} expected in TOP_100");
        }
    }
}

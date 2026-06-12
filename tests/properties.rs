//! Property-style tests: cross-check `PortSpec` against a `HashSet<u16>` ground
//! truth over a small, deterministic port domain.

use std::collections::HashSet;

use portspec::PortSpec;

/// A handful of specs that exercise singletons, ranges, overlaps, adjacency,
/// and the boundaries of the 16-bit space.
fn samples() -> Vec<PortSpec> {
    [
        "1",
        "1-10",
        "1-10,5-15",
        "1-10,11-20",
        "22,80,443",
        "1-100,200-300,150",
        "0",
        "65535",
        "65530-65535",
        "0-65535",
    ]
    .iter()
    .map(|s| s.parse().unwrap())
    .collect()
}

fn truth(spec: &PortSpec) -> HashSet<u16> {
    spec.iter().collect()
}

#[test]
fn contains_and_count_match_truth() {
    for spec in samples() {
        let set = truth(&spec);
        assert_eq!(spec.count() as usize, set.len());
        for p in [0u16, 1, 5, 11, 80, 150, 301, 65534, 65535] {
            assert_eq!(spec.contains(p), set.contains(&p), "{spec} @ {p}");
        }
    }
}

#[test]
fn display_round_trips() {
    for spec in samples() {
        let reparsed: PortSpec = spec.to_string().parse().unwrap();
        assert_eq!(reparsed, spec);
    }
}

#[test]
fn normalize_is_idempotent() {
    for spec in samples() {
        let again = PortSpec::from_ranges(spec.ranges().iter().copied());
        assert_eq!(again, spec);
    }
}

#[test]
fn set_ops_match_truth() {
    let specs = samples();
    for a in &specs {
        for b in &specs {
            let (sa, sb) = (truth(a), truth(b));

            let union = a.union(b);
            assert_eq!(truth(&union), &sa | &sb, "union {a} {b}");

            let inter = a.intersection(b);
            assert_eq!(truth(&inter), &sa & &sb, "intersection {a} {b}");

            let diff = a.difference(b);
            assert_eq!(truth(&diff), &sa - &sb, "difference {a} {b}");

            assert_eq!(a.overlaps(b), !(&sa & &sb).is_empty(), "overlaps {a} {b}");
            assert_eq!(a.is_subset_of(b), sa.is_subset(&sb), "subset {a} {b}");
        }
    }
}

#[test]
fn complement_is_exact_inverse() {
    for spec in samples() {
        let comp = spec.complement();
        assert_eq!(spec.count() + comp.count(), 65536);
        assert!(!spec.overlaps(&comp));
        assert_eq!(spec.union(&comp), "0-65535".parse().unwrap());
    }
}

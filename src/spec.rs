//! [`PortSpec`] — a normalized set of port ranges parsed from a spec string.

use std::fmt;
use std::str::FromStr;

use crate::error::ParseError;
use crate::range::PortRange;

/// A set of ports described by a specification string such as
/// `"22,80,443,1000-2000"`.
///
/// The internal ranges are always kept **normalized**: sorted by start and
/// merged so that no two ranges overlap or touch. Equal specs therefore compare
/// equal regardless of how they were written (`"80,80,80"` == `"80"`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PortSpec {
    ranges: Vec<PortRange>,
}

impl PortSpec {
    /// An empty spec covering no ports.
    pub const fn new() -> Self {
        PortSpec { ranges: Vec::new() }
    }

    /// Build a normalized spec from a collection of ranges.
    pub fn from_ranges<I: IntoIterator<Item = PortRange>>(ranges: I) -> Self {
        let mut spec = PortSpec {
            ranges: ranges.into_iter().collect(),
        };
        spec.normalize();
        spec
    }

    /// Sort the ranges by start and merge every pair that overlaps or touches.
    fn normalize(&mut self) {
        if self.ranges.len() < 2 {
            return;
        }
        self.ranges.sort_unstable();
        let mut merged: Vec<PortRange> = Vec::with_capacity(self.ranges.len());
        for r in self.ranges.drain(..) {
            match merged.last_mut() {
                Some(last) => match last.merge(&r) {
                    Some(combined) => *last = combined,
                    None => merged.push(r),
                },
                None => merged.push(r),
            }
        }
        self.ranges = merged;
    }

    /// The normalized, non-overlapping ranges, in ascending order.
    pub fn ranges(&self) -> &[PortRange] {
        &self.ranges
    }

    /// Add a range to the spec, re-normalizing so the invariant holds.
    pub fn insert(&mut self, range: PortRange) {
        self.ranges.push(range);
        self.normalize();
    }

    /// Remove every port in `range` from the spec.
    pub fn remove(&mut self, range: PortRange) {
        *self = self.difference(&PortSpec::from(range));
    }

    /// `true` if the spec covers no ports.
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    /// The total number of distinct ports covered.
    pub fn count(&self) -> u32 {
        self.ranges.iter().map(|r| r.count()).sum()
    }

    /// Returns `true` if `port` is covered by the spec.
    ///
    /// Runs a binary search over the normalized ranges, so it is `O(log n)` in
    /// the number of ranges.
    pub fn contains(&self, port: u16) -> bool {
        self.ranges
            .binary_search_by(|r| {
                if port < r.start() {
                    std::cmp::Ordering::Greater
                } else if port > r.end() {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            })
            .is_ok()
    }

    /// Iterate over every port in the spec, ascending, without duplicates.
    pub fn iter(&self) -> impl Iterator<Item = u16> + '_ {
        self.ranges.iter().flat_map(|r| r.iter())
    }

    /// Returns `true` if the two specs share at least one port.
    pub fn overlaps(&self, other: &PortSpec) -> bool {
        self.ranges
            .iter()
            .any(|a| other.ranges.iter().any(|b| a.overlaps(b)))
    }

    /// Returns `true` if every port in `self` is also in `other`.
    pub fn is_subset_of(&self, other: &PortSpec) -> bool {
        self.difference(other).is_empty()
    }

    /// Returns `true` if every port in `other` is also in `self`.
    pub fn contains_spec(&self, other: &PortSpec) -> bool {
        other.is_subset_of(self)
    }

    /// The union of two specs: every port in either.
    pub fn union(&self, other: &PortSpec) -> PortSpec {
        let mut ranges = self.ranges.clone();
        ranges.extend_from_slice(&other.ranges);
        PortSpec::from_ranges(ranges)
    }

    /// The complement of the spec over the whole port space (`0..=65535`) —
    /// every port *not* covered.
    pub fn complement(&self) -> PortSpec {
        self.complement_within(PortRange::FULL)
    }

    /// The complement of the spec restricted to `bound` — the ports of `bound`
    /// that the spec does not cover.
    pub fn complement_within(&self, bound: PortRange) -> PortSpec {
        PortSpec::from(bound).difference(self)
    }

    /// The difference of two specs: ports in `self` that are not in `other`.
    pub fn difference(&self, other: &PortSpec) -> PortSpec {
        let mut out = Vec::new();
        for r in &self.ranges {
            // `cur` walks across `r` in u32 so the `+1` past 65535 never wraps.
            let mut cur = u32::from(r.start());
            let end = u32::from(r.end());
            for o in &other.ranges {
                let (os, oe) = (u32::from(o.start()), u32::from(o.end()));
                if oe < cur || os > end {
                    continue;
                }
                if os > cur {
                    out.push(PortRange::new(cur as u16, (os - 1) as u16).expect("cur < os"));
                }
                cur = cur.max(oe + 1);
                if cur > end {
                    break;
                }
            }
            if cur <= end {
                out.push(PortRange::new(cur as u16, end as u16).expect("cur <= end"));
            }
        }
        PortSpec::from_ranges(out)
    }

    /// The intersection of two specs: only ports present in both.
    pub fn intersection(&self, other: &PortSpec) -> PortSpec {
        let mut out = Vec::new();
        for a in &self.ranges {
            for b in &other.ranges {
                if a.overlaps(b) {
                    let start = a.start().max(b.start());
                    let end = a.end().min(b.end());
                    out.push(PortRange::new(start, end).expect("overlap implies start <= end"));
                }
            }
        }
        PortSpec::from_ranges(out)
    }
}

impl fmt::Display for PortSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for r in &self.ranges {
            if !first {
                f.write_str(",")?;
            }
            write!(f, "{r}")?;
            first = false;
        }
        Ok(())
    }
}

impl FromStr for PortSpec {
    type Err = ParseError;

    /// Parse a comma-separated list of ports and ranges. Whitespace around items
    /// is ignored; empty items (e.g. a trailing comma) are skipped.
    fn from_str(s: &str) -> Result<Self, ParseError> {
        let s = s.trim();
        if s.is_empty() {
            return Err(ParseError::Empty);
        }
        let mut ranges = Vec::new();
        for item in s.split(',') {
            let item = item.trim();
            if item.is_empty() {
                continue;
            }
            ranges.push(item.parse::<PortRange>()?);
        }
        if ranges.is_empty() {
            return Err(ParseError::Empty);
        }
        Ok(PortSpec::from_ranges(ranges))
    }
}

impl From<PortRange> for PortSpec {
    fn from(r: PortRange) -> Self {
        PortSpec { ranges: vec![r] }
    }
}

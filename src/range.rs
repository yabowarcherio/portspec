//! A single inclusive range of port numbers.

use std::fmt;
use std::str::FromStr;

use crate::error::ParseError;

/// An inclusive range of TCP/UDP port numbers, `start..=end`.
///
/// Both endpoints are valid 16-bit port numbers (`0..=65535`); the range is
/// always non-empty since it is inclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PortRange {
    // Field order matters for the derived ordering: by start, then end.
    start: u16,
    end: u16,
}

impl PortRange {
    /// Build a range from its inclusive endpoints.
    ///
    /// # Errors
    ///
    /// Returns [`ParseError::StartAfterEnd`] if `start > end`.
    pub fn new(start: u16, end: u16) -> Result<Self, ParseError> {
        if start > end {
            return Err(ParseError::StartAfterEnd);
        }
        Ok(Self { start, end })
    }

    /// A range covering a single port.
    pub const fn single(port: u16) -> Self {
        Self {
            start: port,
            end: port,
        }
    }

    /// The full port space, `0..=65535`.
    pub const FULL: PortRange = PortRange {
        start: 0,
        end: u16::MAX,
    };

    /// The first (lowest) port in the range.
    #[inline]
    pub const fn start(&self) -> u16 {
        self.start
    }

    /// The last (highest) port in the range.
    #[inline]
    pub const fn end(&self) -> u16 {
        self.end
    }

    /// The number of ports in the range.
    #[inline]
    pub const fn count(&self) -> u32 {
        (self.end as u32) - (self.start as u32) + 1
    }

    /// Returns `true` if `port` falls within the range, inclusive.
    #[inline]
    pub const fn contains(&self, port: u16) -> bool {
        self.start <= port && port <= self.end
    }

    /// Returns `true` if the two ranges share at least one port.
    pub const fn overlaps(&self, other: &PortRange) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    /// Returns `true` if `other` is entirely within this range.
    pub const fn contains_range(&self, other: &PortRange) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    /// Returns `true` if the two ranges touch or overlap and can therefore be
    /// combined into one contiguous range.
    pub const fn is_mergeable(&self, other: &PortRange) -> bool {
        // Overlapping, or directly adjacent (e.g. 1-10 and 11-20).
        let lo = if self.start <= other.start {
            self
        } else {
            other
        };
        let hi = if self.start <= other.start {
            other
        } else {
            self
        };
        // hi.start <= lo.end + 1, guarding the u16 overflow at 65535.
        (hi.start as u32) <= (lo.end as u32) + 1
    }

    /// Merge two touching or overlapping ranges into one, or `None` if they are
    /// disjoint with a gap between them.
    pub fn merge(&self, other: &PortRange) -> Option<PortRange> {
        if self.is_mergeable(other) {
            Some(PortRange {
                start: self.start.min(other.start),
                end: self.end.max(other.end),
            })
        } else {
            None
        }
    }

    /// Iterate over every port in the range, lowest to highest.
    pub const fn iter(&self) -> PortRangeIter {
        PortRangeIter {
            front: self.start as u32,
            back: self.end as u32,
            done: false,
        }
    }
}

impl fmt::Display for PortRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start == self.end {
            write!(f, "{}", self.start)
        } else {
            write!(f, "{}-{}", self.start, self.end)
        }
    }
}

/// Parse a single port number, rejecting anything outside `0..=65535`.
fn parse_port(s: &str) -> Result<u16, ParseError> {
    s.trim()
        .parse::<u16>()
        .map_err(|_| ParseError::BadPort(s.to_string()))
}

impl FromStr for PortRange {
    type Err = ParseError;

    /// Parse `N`, `N-M`, `N-` (to 65535), `-M` (from 0), or `-` (the full
    /// range).
    fn from_str(s: &str) -> Result<Self, ParseError> {
        let s = s.trim();
        if s.is_empty() {
            return Err(ParseError::Empty);
        }
        match s.split_once('-') {
            None => {
                let p = parse_port(s)?;
                Ok(PortRange::single(p))
            }
            Some((lo, hi)) => {
                let start = if lo.trim().is_empty() {
                    0
                } else {
                    parse_port(lo)?
                };
                let end = if hi.trim().is_empty() {
                    u16::MAX
                } else {
                    parse_port(hi)?
                };
                PortRange::new(start, end)
            }
        }
    }
}

impl IntoIterator for PortRange {
    type Item = u16;
    type IntoIter = PortRangeIter;
    fn into_iter(self) -> PortRangeIter {
        self.iter()
    }
}

/// Iterator over the ports of a [`PortRange`], lowest to highest.
///
/// Backed by a `u32` cursor so the inclusive end at `65535` is handled without
/// overflow. Implements [`DoubleEndedIterator`].
#[derive(Debug, Clone)]
pub struct PortRangeIter {
    front: u32,
    back: u32,
    done: bool,
}

impl Iterator for PortRangeIter {
    type Item = u16;

    fn next(&mut self) -> Option<u16> {
        if self.done {
            return None;
        }
        let cur = self.front;
        if cur == self.back {
            self.done = true;
        } else {
            self.front = cur + 1;
        }
        Some(cur as u16)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.done {
            return (0, Some(0));
        }
        let n = (self.back - self.front + 1) as usize;
        (n, Some(n))
    }
}

impl DoubleEndedIterator for PortRangeIter {
    fn next_back(&mut self) -> Option<u16> {
        if self.done {
            return None;
        }
        let cur = self.back;
        if cur == self.front {
            self.done = true;
        } else {
            self.back = cur - 1;
        }
        Some(cur as u16)
    }
}

impl ExactSizeIterator for PortRangeIter {}

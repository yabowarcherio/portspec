//! Error type for parsing port ranges and specifications.

use std::fmt;

/// An error produced while parsing a port, range, or specification.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseError {
    /// The input was empty or contained only whitespace.
    Empty,
    /// A port number was missing, non-numeric, or out of the 0..=65535 range.
    BadPort(String),
    /// A range's start port sorts after its end port.
    StartAfterEnd,
    /// The input did not look like a port or range, or had stray parts.
    Malformed(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Empty => write!(f, "empty input"),
            ParseError::BadPort(s) => write!(f, "invalid port number: {s:?}"),
            ParseError::StartAfterEnd => {
                write!(f, "range start is greater than range end")
            }
            ParseError::Malformed(s) => write!(f, "malformed input: {s:?}"),
        }
    }
}

impl std::error::Error for ParseError {}

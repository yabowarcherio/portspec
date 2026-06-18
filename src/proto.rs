//! [`Proto`] — the transport-layer protocol a port spec is scoped to.

use std::fmt;
use std::str::FromStr;

use crate::error::ParseError;

/// A transport-layer protocol — TCP or UDP.
///
/// Lets callers distinguish between a service that listens on `tcp/53` (DNS
/// zone transfers) versus `udp/53` (regular DNS queries), without having two
/// separate types for what is otherwise the same port arithmetic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Proto {
    /// TCP — connection-oriented stream transport (RFC 9293).
    Tcp,
    /// UDP — datagram transport (RFC 768).
    Udp,
}

impl Proto {
    /// The canonical lower-case label (`"tcp"` / `"udp"`).
    pub const fn as_str(self) -> &'static str {
        match self {
            Proto::Tcp => "tcp",
            Proto::Udp => "udp",
        }
    }

    /// The single-letter shorthand (`'T'` for TCP, `'U'` for UDP) used by
    /// `nmap`-style spec strings like `T:80,U:53`.
    pub const fn letter(self) -> char {
        match self {
            Proto::Tcp => 'T',
            Proto::Udp => 'U',
        }
    }
}

impl fmt::Display for Proto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Proto {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        match s.trim() {
            "tcp" | "TCP" | "Tcp" | "t" | "T" => Ok(Proto::Tcp),
            "udp" | "UDP" | "Udp" | "u" | "U" => Ok(Proto::Udp),
            other => Err(ParseError::Malformed(other.to_string())),
        }
    }
}

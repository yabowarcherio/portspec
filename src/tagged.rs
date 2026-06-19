//! [`TaggedSpec`] — a port specification split across TCP and UDP.
//!
//! Many scanners (`nmap -p T:80,U:53`) accept a single spec string that mixes
//! ports for both transports. `TaggedSpec` captures that shape exactly: two
//! independent [`PortSpec`]s, one per [`Proto`].

use std::fmt;
use std::str::FromStr;

use crate::error::ParseError;
use crate::proto::Proto;
use crate::spec::PortSpec;

/// A spec carrying separate TCP and UDP port sets.
///
/// Parses an `nmap`-style spec like `T:80,443,U:53,123` — entries are split
/// across the two transports based on the most recent `T:`/`U:` prefix
/// (`T:` is the implicit default for un-prefixed entries at the front).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TaggedSpec {
    /// The TCP-side port set.
    pub tcp: PortSpec,
    /// The UDP-side port set.
    pub udp: PortSpec,
}

impl TaggedSpec {
    /// An empty spec covering no ports on either transport.
    pub const fn new() -> Self {
        TaggedSpec {
            tcp: PortSpec::new(),
            udp: PortSpec::new(),
        }
    }

    /// The [`PortSpec`] associated with a given protocol.
    pub fn for_proto(&self, proto: Proto) -> &PortSpec {
        match proto {
            Proto::Tcp => &self.tcp,
            Proto::Udp => &self.udp,
        }
    }

    /// `true` if both transports are empty.
    pub fn is_empty(&self) -> bool {
        self.tcp.is_empty() && self.udp.is_empty()
    }

    /// The total number of ports across both transports — a `tcp/80` and a
    /// `udp/80` count as two distinct entries.
    pub fn count(&self) -> u32 {
        self.tcp.count() + self.udp.count()
    }

    /// `true` if `port` is in the named protocol's spec.
    pub fn contains(&self, proto: Proto, port: u16) -> bool {
        self.for_proto(proto).contains(port)
    }
}

impl FromStr for TaggedSpec {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        let t = s.trim();
        if t.is_empty() {
            return Err(ParseError::Empty);
        }
        // Walk the spec splitting on commas, but treat any `T:`/`U:` prefix
        // as a switch of the "current" protocol. The default before the
        // first prefix is TCP, matching nmap.
        let mut current = Proto::Tcp;
        let mut tcp_parts: Vec<String> = Vec::new();
        let mut udp_parts: Vec<String> = Vec::new();

        for raw in t.split(',') {
            let chunk = raw.trim();
            if chunk.is_empty() {
                continue;
            }
            let (proto, body) = match chunk.split_once(':') {
                Some((p, b)) if p.len() <= 4 => match p.parse::<Proto>() {
                    Ok(proto) => {
                        current = proto;
                        (proto, b.trim())
                    }
                    Err(_) => (current, chunk),
                },
                _ => (current, chunk),
            };
            if body.is_empty() {
                // A bare `T:` switches the default but emits nothing.
                continue;
            }
            match proto {
                Proto::Tcp => tcp_parts.push(body.to_string()),
                Proto::Udp => udp_parts.push(body.to_string()),
            }
        }

        let tcp = if tcp_parts.is_empty() {
            PortSpec::new()
        } else {
            tcp_parts.join(",").parse::<PortSpec>()?
        };
        let udp = if udp_parts.is_empty() {
            PortSpec::new()
        } else {
            udp_parts.join(",").parse::<PortSpec>()?
        };
        Ok(TaggedSpec { tcp, udp })
    }
}

impl fmt::Display for TaggedSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut wrote = false;
        if !self.tcp.is_empty() {
            write!(f, "T:{}", self.tcp)?;
            wrote = true;
        }
        if !self.udp.is_empty() {
            if wrote {
                f.write_str(",")?;
            }
            write!(f, "U:{}", self.udp)?;
        }
        Ok(())
    }
}

//! # portspec
//!
//! Parse and manipulate TCP/UDP **port specifications** — the comma-separated
//! lists of ports and ranges familiar from tools like `nmap -p`
//! (`"22,80,443,1000-2000"`). Pure integer logic over `u16` port numbers: no
//! sockets, no DNS, no embedded data.
//!
//! ```
//! use portspec::PortSpec;
//!
//! let spec: PortSpec = "22,80,443,8000-8002".parse().unwrap();
//! assert_eq!(spec.count(), 6);
//! assert!(spec.contains(8001));
//! let ports: Vec<u16> = spec.iter().collect();
//! assert_eq!(ports, [22, 80, 443, 8000, 8001, 8002]);
//! ```
//!
//! ## Features
//!
//! - `cli` *(default)* enables the `portspec` binary; disable it
//!   (`default-features = false`) for a slim library dependency.
//! - `serde` derives `Serialize`/`Deserialize` on the public types.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod error;
mod range;
mod services;
mod spec;

pub use error::ParseError;
pub use range::{PortRange, PortRangeIter};
pub use services::{port_for, service_for, Service, SERVICES, SERVICES_COUNT};
pub use spec::{PortSpec, PortSpecIter};

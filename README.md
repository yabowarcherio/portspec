# portspec

[![CI](https://github.com/yabowarcherio/portspec/actions/workflows/ci.yml/badge.svg)](https://github.com/yabowarcherio/portspec/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

Parse and manipulate TCP/UDP **port specifications** — the comma-separated lists
of ports and ranges familiar from tools like `nmap -p` (`"22,80,443,1000-2000"`).
Pure integer logic over `u16` port numbers: no sockets, no DNS, no embedded
data. Library **and** CLI.

- **Ranges** — `1000-2000`, with open-ended forms `1024-`, `-1024`, `-`
- **Specs** — `22,80,443,8000-8002`, kept normalized (sorted, merged, deduped)
- Containment, counts, union and intersection
- Forward/reverse iteration over the covered ports

## Install

```sh
cargo install portspec     # CLI
cargo add portspec          # library
```

For a slim library-only dependency without the CLI stack:

```toml
[dependencies]
portspec = { version = "0.1", default-features = false }
```

## Usage (CLI)

```text
portspec [OPTIONS] <SPEC>...

Arguments:
  <SPEC>...  Port specs to combine (union). Use `-` to read from stdin.

Options:
  -c, --count            Print the number of ports instead of listing them
  -r, --ranges           Print the normalized range form (e.g. 1-20,443)
      --intersect <SPEC> Keep only ports also present in this spec
      --contains <PORT>  Exit 0 if PORT is covered, 1 otherwise
  -l, --limit <N>        Stop after listing N ports (0 = no limit)
  -R, --reverse          List ports from highest to lowest
  -h, --help             Print help
  -V, --version
```

```sh
$ portspec 22,80,8000-8002
22
80
8000
8001
8002

$ portspec --ranges 80,1-10,11-20,80      # normalize + merge
1-20,80

$ portspec --count 1-1024                 # 1024
$ portspec --intersect 50-150 --ranges 1-100   # 50-100
$ portspec --contains 443 1-1024 && echo open
```

**Exit codes:** `0` success · `1` `--contains` did not match · `2` a spec failed
to parse.

## Usage (library)

```rust
use portspec::PortSpec;

let spec: PortSpec = "22,80,443,8000-8002".parse().unwrap();
assert_eq!(spec.count(), 6);
assert!(spec.contains(8001));

// Normalized, merged, deduped — equal specs compare equal.
let a: PortSpec = "1-10,11-20,5-15".parse().unwrap();
assert_eq!(a.to_string(), "1-20");

// Set algebra.
let b: PortSpec = "15-30".parse().unwrap();
assert_eq!(a.union(&b).to_string(), "1-30");
assert_eq!(a.intersection(&b).to_string(), "15-20");
```

More set algebra and conversions:

```rust
use portspec::{PortRange, PortSpec};

let spec: PortSpec = "1-100".parse().unwrap();

// Difference, complement, and predicates.
assert_eq!(spec.difference(&"20-30".parse().unwrap()).to_string(), "1-19,31-100");
assert!(spec.is_subset_of(&"1-200".parse().unwrap()));
assert!(spec.complement().contains(0) && !spec.complement().contains(50));

// Build from individual ports (adjacent ones merge) and the IANA ranges.
assert_eq!(PortSpec::from_ports([80, 81, 82]).to_string(), "80-82");
assert_eq!(PortRange::WELL_KNOWN.to_string(), "0-1023");
```

The `PortRange` type exposes the single-range surface (parsing, `contains`,
`overlaps`, `intersection`, `merge`, double-ended iteration). Enable the `serde`
feature to derive `Serialize`/`Deserialize` on both types.

## Design notes

- **No networking.** This crate is pure port arithmetic; it never opens a socket
  or resolves a name. Safe in build scripts and hot loops.
- **Always normalized.** A `PortSpec` keeps its ranges sorted, merged, and
  non-overlapping, so containment is a binary search and equality is structural.
- **`#![forbid(unsafe_code)]`.**

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at
your option.

# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `Proto` enum (`Tcp` / `Udp`) with `as_str()` / `letter()` accessors and
  `FromStr` accepting `"tcp"`/`"udp"`/`"t"`/`"u"` plus case variants. Lets
  callers tag specs with their transport instead of carrying the protocol
  out-of-band.
- `TaggedSpec` parses nmap-style spec strings (`T:22,80,U:53,123`), splitting
  ports across two independent `PortSpec`s. The default proto is TCP, and
  `T:`/`U:` prefixes switch which proto subsequent comma-separated entries
  apply to. Round-trips through `Display`/`FromStr`.

## [0.2.0]

### Added

- Built-in services table (`SERVICES`, `port_for`, `service_for`,
  `SERVICES_COUNT`) — well-known TCP/UDP names like `ssh`, `http`, `https`,
  `dns`, `ldap`, `postgres`, `rdp`, …
- `PortSpec` parser accepts service names (`"ssh,http,8000-8002"`).
- `PortSpec::iter_named` pairs each port with its service name (if any).

## [0.1.0]

Initial release.

### Added

- `PortRange`: an inclusive range of port numbers with parsing (`N`, `N-M`,
  `N-`, `-M`, `-`), `contains`, `overlaps`, `contains_range`, `intersection`,
  `merge`, counts, and double-ended iteration. IANA `WELL_KNOWN` / `REGISTERED`
  / `DYNAMIC` / `FULL` constants.
- `PortSpec`: a normalized (sorted, merged, deduped) set of ranges with
  `contains` (binary search), `count`, `iter`, `insert`, `remove`, `union`,
  `intersection`, `difference`, `complement` / `complement_within`, and the
  `overlaps` / `is_subset_of` / `contains_spec` predicates.
- Conversions: `from_ports`, `FromIterator<PortRange>` / `FromIterator<u16>`,
  and an owning `IntoIterator` (`PortSpecIter`).
- `ParseError` covering empty, malformed, bad-port, and start-after-end inputs.
- `portspec` CLI: expand to ports, `--count`, `--ranges`, `--json`,
  `--intersect`, `--difference`, `--invert`, `--contains`, `--limit`,
  `--reverse`, and stdin input via `-`.
- Optional `serde` feature deriving `Serialize`/`Deserialize`.
- Criterion benchmarks and `HashSet`-cross-checked property tests.

[Unreleased]: https://github.com/yabowarcherio/portspec/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/yabowarcherio/portspec/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/yabowarcherio/portspec/releases/tag/v0.1.0

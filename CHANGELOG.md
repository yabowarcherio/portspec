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
- Curated `top_100_tcp()` and `top_1000_tcp()` preset specs plus the raw
  `TOP_100_TCP_LIST` / `TOP_1000_TCP_LIST` text constants for callers who
  want to echo the list without paying the parse cost.
- `PortSpec::nth_port(index)` returns the `index`th port in ascending order
  without iterating to it. `first()` / `last()` expose the spec's extremes.
- CLI `--resolve` appends each port's service name from the built-in table
  (empty when unknown) in default listing mode.
- CLI `--preset top-100` / `--preset top-1000` replaces input specs with a
  curated preset; combines with `--intersect`/`--difference`/`--invert`.
- CLI `--tagged` interprets the input spec as an nmap-style `T:80,U:53`
  string and emits `tcp PORT` / `udp PORT` lines (one per port, ascending).
- Expanded the built-in services table to 49 entries (added databases,
  queues, infra: cassandra, couchdb, elasticsearch, etcd, grpc, kafka,
  memcached, mongodb, mqtt, nats, prometheus, zookeeper, bgp, irc, git,
  amqp).
- `services_for(port)` iterates every name registered to a port, for callers
  who want all aliases rather than the first alphabetical match.
- `TaggedSpec::union` / `intersection` / `difference` — per-protocol set
  algebra. A port that's TCP-only in one operand and UDP-only in the other
  never crosses transports.

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

# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0]

Initial release.

### Added

- `PortRange`: an inclusive range of port numbers with parsing (`N`, `N-M`,
  `N-`, `-M`, `-`), `contains`, `overlaps`, `contains_range`, `merge`, counts,
  and double-ended iteration.
- `PortSpec`: a normalized (sorted, merged, deduped) set of ranges parsed from a
  comma-separated spec, with `contains` (binary search), `count`, `iter`,
  `union`, and `intersection`.
- `ParseError` covering empty, malformed, bad-port, and start-after-end inputs.
- `portspec` CLI: expand to ports, `--count`, `--ranges`, `--intersect`,
  `--contains`, `--limit`, `--reverse`, and stdin input via `-`.
- Optional `serde` feature deriving `Serialize`/`Deserialize`.

[Unreleased]: https://github.com/yabowarcherio/portspec/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yabowarcherio/portspec/releases/tag/v0.1.0

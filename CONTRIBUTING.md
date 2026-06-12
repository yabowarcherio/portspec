# Contributing to portspec

Thanks for taking the time to contribute! This is a small, focused crate, so
the bar is mostly "keep it pure, correct, and well-tested."

## Getting started

```sh
git clone https://github.com/yabowarcherio/portspec
cd portspec
cargo test
```

You need a recent stable Rust toolchain (see `rust-version` in
[`Cargo.toml`](Cargo.toml) for the minimum supported version, MSRV).

## Before you open a PR

Please make sure the following all pass locally — CI runs the same checks:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo build --no-default-features        # library-only must still build
cargo deny check                         # licenses & advisories (if installed)
```

## Guidelines

- **No networking, ever.** This crate is pure port arithmetic. Do not add a
  dependency or code path that opens a socket or resolves a name.
- **No `unsafe`.** The crate sets `#![forbid(unsafe_code)]`; keep it that way.
- **Keep specs normalized.** Any new `PortSpec` constructor or combinator must
  return sorted, merged, non-overlapping ranges.
- **Add a test** for any behavior change. Edge cases worth covering: the full
  range `-`, adjacent merges across `65535`, empty/spurious items, and
  union/intersection on disjoint specs.
- **Document public items.** `missing_docs` is a warning; keep the API
  documented.

## Code of Conduct

Be kind and constructive. We follow the spirit of the
[Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

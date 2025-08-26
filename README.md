# toolchest - Essential Utility Collection for Rust

[![Crates.io](https://img.shields.io/crates/v/toolchest.svg)](https://crates.io/crates/toolchest)
[![Documentation](https://docs.rs/toolchest/badge.svg)](https://docs.rs/toolchest)
[![CI](https://github.com/danjloveless/toolchest/workflows/CI/badge.svg)](https://github.com/danjloveless/toolchest/actions)

Your essential collection of Rust utilities for strings, math, time, types, collections, and more.

## Why toolchest?

Every craftsperson needs a well-organized toolchest. This crate provides a focused set of utilities
that cover common tasks across domains without pulling in heavy dependencies.

## Quick Start

```rust
use toolchest::prelude::*;
use std::time::Duration;

// String manipulation
let snake = strings::to_snake_case("HelloWorld");  // "hello_world"

// Math utilities
let rounded = math::round(3.14159, 2);  // 3.14
let clamped = math::clamp(15, 0, 10);   // 10

// Deep operations (example)
// let merged = deep::merge(&default_config, &user_config);

// Function combinators (example)
// let search = functions::debounce(expensive_search, Duration::from_millis(300));

// Type utilities
let is_empty = types::is_empty::<Vec<i32>>(&vec![]);
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
toolchest = "0.1.0"
```

MSRV: 1.81.0

Optional features:
- `json` – serde/serde_json helpers
- `fs` – filesystem utilities (walkdir)

## Modules at a Glance

For full API details, see the docs: https://docs.rs/toolchest

### Strings
- case conversion/manipulation/escape/words/slug/validators and more

### Math
- rounding/stats/ranges/numeric helpers/primes/vectors/distances

### Deep
- clone/merge/equal/path and JSON path (feature: `json`)

### Functions
- debounce/throttle/memoize/retry/backoff/compose/rate-limiters/timeout

### Types
- checking/conversions/non-empty and helpers

### Collections
- chunk/uniq/set ops/grouping/windows/cartesian/transpose/sort/find

### Time
- humanize/parse/stopwatch/backoff/cron-lite

### Random
- ranges/choices/uuid/bytes

### Hash
- djb2/fnv1a/murmur3/consistent hash

### IO (feature: `fs`)
- read/write/dirs/find files

### Validation
- Luhn, ASCII/UTF-8, IBAN, E.164 phone, US SSN

### Encoding
- hex, rot13/caesar


## Performance

- Zero runtime dependencies by default (feature-gated extras)
- Zero-cost abstractions
- Optimized for common cases
- Partial `no_std` support

## Comparison with other crates

| Crate | What it is | Overlap with `toolchest` | When to choose it |
| --- | --- | --- | --- |
| [`itertools`](https://crates.io/crates/itertools) | Iterator adaptors and utilities | Some collection helpers overlap conceptually (chunking, grouping, cartesian, windows) | You need advanced iterator combinators and zero-allocation streaming transforms |
| [`heck`](https://crates.io/crates/heck) | String case conversions | Overlaps with `strings::case` | You only need case conversion and prefer a focused crate |
| [`convert_case`](https://crates.io/crates/convert_case) | String case conversions | Overlaps with `strings::case` | Same as above; pick one of these if you only need this |
| [`rand`](https://crates.io/crates/rand) | RNGs and distributions | `random` provides quick, non-crypto helpers only | You need configurable RNGs, distributions, or crypto-secure randomness |
| [`time`](https://crates.io/crates/time) / [`chrono`](https://crates.io/crates/chrono) | Date/time types and parsing | `time` module has humanize/parse/stopwatch/backoff; not a full datetime stack | You need full-featured datetime, time zones, formatting/parsing |
| [`regex`](https://crates.io/crates/regex) | Regular expressions | Minimal overlap; `strings`/`validation` provide common checks | You need general-purpose pattern matching |
| [`statrs`](https://crates.io/crates/statrs) | Statistics and distributions | `math` has common rounding/stats helpers | You need rich statistical distributions and tests |
| [`serde`/`serde_json`](https://serde.rs/) | Serialization and JSON | Optional `json` feature provides helpers around them | You are doing full serialization/deserialization work

## Why not just use X?

- If you only need advanced iterator adaptors, use **`itertools`**.
- If you need cryptographically secure randomness, distributions, or a configurable RNG, use **`rand`** (and friends like `rand_chacha`).
- If you need full datetime handling (time zones, formatting, parsing), use **`time`** or **`chrono`**.
- If you need regex-powered text processing, use **`regex`**.
- If you only need string case conversion, use **`heck`** or **`convert_case`**.
- If you need heavy statistics and probability distributions, use **`statrs`**.

`toolchest` aims to cover a broad set of day-to-day utilities with:
- Small, focused APIs you can use in minutes
- Zero default runtime dependencies and feature-gated extras
- Pragmatic, non-crypto defaults where appropriate

It complements the crates above; use them when you need depth, use `toolchest` when you want breadth without pulling in many dependencies.

## Migration from Common Crates

Replacing multiple utility crates with `toolchest`:

```toml
# Before
[dependencies]
convert_case = "0.6"
heck = "0.4"
stringcase = "0.2"

# After
[dependencies]
toolchest = "0.1.0"
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

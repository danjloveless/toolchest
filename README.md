# toolchest - Essential Utility Collection for Rust

[![Crates.io](https://img.shields.io/crates/v/toolchest.svg)](https://crates.io/crates/toolchest)
[![Documentation](https://docs.rs/toolchest/badge.svg)](https://docs.rs/toolchest)
[![CI](https://github.com/danjloveless/toolchest/workflows/CI/badge.svg)](https://github.com/danjloveless/toolchest/actions)

Your essential collection of Rust utilities – everything `itertools` doesn't do.

## Why toolchest?

Every craftsperson needs a well-organized toolchest. In Rust, you need two utility crates:
- `itertools` - for collection/iterator operations
- `toolchest` - for everything else

Together, they provide a complete utility toolkit without scattered dependencies.

## Quick Start

```rust
use itertools::Itertools;     // For collection/iterator helpers
use toolchest::prelude::*;    // For everything else
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

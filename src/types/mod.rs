//! Type utilities module.
//!
//! Helpers for type checking, conversions, and ergonomic wrappers.
//!
//! Examples:
//! ```rust
//! use toolchest::types::{is_empty, parse_or_default, to_string_safe, NonEmptyVec};
//! assert!(is_empty::<Vec<i32>>(&vec![]));
//! let n: i32 = parse_or_default("not a number");
//! assert_eq!(n, 0);
//! assert_eq!(to_string_safe("hello"), "hello");
//!
//! let nev = NonEmptyVec::from_vec(vec![1,2,3]).unwrap();
//! assert_eq!(nev.len(), 3);
//! ```

pub mod checking;
pub mod conversion;
pub mod extras;

pub use checking::{is_empty, IsEmpty};
pub use conversion::{default_to, parse_or, parse_or_default, to_string_safe};
pub use extras::{map_ok_or, map_some_or, NonEmptyVec};

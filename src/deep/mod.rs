//! Deep operations module.
//!
//! Helpers for deep cloning, deep equality, merging nested structures, and
//! path-based get/set access. Optional JSON-path helpers are available behind
//! the `json` feature.
//!
//! Examples:
//! ```rust
//! use toolchest::deep::{deep_equal, merge};
//! use std::collections::HashMap;
//! let a = HashMap::from([("x", 1), ("y", 2)]);
//! let b = HashMap::from([("y", 3)]);
//! let merged = merge(&a, &b);
//! assert!(deep_equal(&merged, &HashMap::from([("x", 1), ("y", 3)])));
//! ```

pub mod clone;
pub mod equal;
#[cfg(feature = "json")]
pub mod json_path;
pub mod merge;
pub mod path;

pub use clone::{clone as deep_clone, DeepClone};
pub use equal::{deep_equal, deep_equal_slice};
#[cfg(feature = "json")]
pub use json_path::{json_get, json_has, json_set};
pub use merge::{merge, merge_all, DeepMerge};
pub use path::{get, has, set, PathAccess};

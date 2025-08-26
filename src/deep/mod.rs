//! Deep operations module

pub mod clone;
pub mod merge;
pub mod path;
pub mod equal;
#[cfg(feature = "json")]
pub mod json_path;

pub use clone::{clone as deep_clone, DeepClone};
pub use merge::{merge, merge_all, DeepMerge};
pub use path::{get, has, set, PathAccess};
pub use equal::{deep_equal, deep_equal_slice};
#[cfg(feature = "json")]
pub use json_path::{json_get, json_has, json_set};



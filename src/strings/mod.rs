//! String utilities module.
//!
//! A grab bag of ergonomic string helpers for casing, trimming, padding,
//! path/url handling, word utilities, and more.
//!
//! Highlights:
//! - Casing: [`to_snake_case`], [`to_camel_case`], [`to_kebab_case`], [`to_title_case`]
//! - Manipulation: [`capitalize`], [`uncapitalize`], [`pad_start`], [`pad_end`], [`trim`], [`truncate`]
//! - Paths/URLs: [`join_paths`], [`normalize_path`], [`url_encode`], [`url_decode`]
//! - Extras: [`slugify`], inflection helpers
//!
//! Examples:
//! ```rust
//! use toolchest::strings::{to_snake_case, capitalize, pad_end, url_encode};
//! assert_eq!(to_snake_case("HelloWorld"), "hello_world");
//! assert_eq!(capitalize("rust"), "Rust");
//! assert_eq!(pad_end("hi", 4, '.'), "hi..");
//! assert_eq!(url_encode("a b"), "a%20b");
//! ```

pub mod case;
pub mod escape;
pub mod extra;
pub mod manipulation;
pub mod path;
pub mod url;
pub mod words;

pub use case::{to_camel_case, to_kebab_case, to_pascal_case, to_snake_case, to_title_case};
pub use extra::{levenshtein_distance, pluralize, singularize, slugify};
pub use manipulation::{
    capitalize, pad_end, pad_start, trim, truncate, truncate_with, uncapitalize,
};
pub use path::{join_paths, normalize_path};
pub use url::{url_decode, url_encode};

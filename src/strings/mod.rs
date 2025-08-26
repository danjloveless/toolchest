//! Strings utilities module

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

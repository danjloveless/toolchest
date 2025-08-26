//! Strings utilities module

pub mod case;
pub mod manipulation;
pub mod escape;
pub mod words;
pub mod extra;
pub mod url;
pub mod path;

pub use case::{to_camel_case, to_kebab_case, to_pascal_case, to_snake_case, to_title_case};
pub use manipulation::{capitalize, pad_end, pad_start, trim, truncate, truncate_with, uncapitalize};
pub use extra::{levenshtein_distance, pluralize, singularize, slugify};
pub use url::{url_decode, url_encode};
pub use path::{join_paths, normalize_path};



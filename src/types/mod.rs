//! Type utilities module

pub mod checking;
pub mod conversion;
pub mod extras;

pub use checking::{is_empty, IsEmpty};
pub use conversion::{default_to, parse_or, parse_or_default, to_string_safe};
pub use extras::{map_ok_or, map_some_or, NonEmptyVec};



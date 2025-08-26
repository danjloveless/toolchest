//! Type conversion utilities

use std::fmt::Display;
use std::str::FromStr;

/// Return value or default if None
///
/// Example:
/// ```rust
/// use toolchest::types::default_to;
/// assert_eq!(default_to(Some(5), 0), 5);
/// assert_eq!(default_to(None, 7), 7);
/// ```
pub fn default_to<T>(value: Option<T>, default: T) -> T {
    value.unwrap_or(default)
}

/// Convert any Display type to String safely
///
/// Example:
/// ```rust
/// use toolchest::types::to_string_safe;
/// assert_eq!(to_string_safe(42), "42");
/// ```
pub fn to_string_safe<T: Display>(value: T) -> String {
    value.to_string()
}

/// Parse string or return default value
///
/// Example:
/// ```rust
/// use toolchest::types::parse_or_default;
/// let x: i32 = parse_or_default("not a number");
/// assert_eq!(x, 0);
/// let y: i32 = parse_or_default("12");
/// assert_eq!(y, 12);
/// ```
pub fn parse_or_default<T>(s: &str) -> T
where
    T: FromStr + Default,
{
    s.parse().unwrap_or_default()
}

/// Parse string or return provided default
///
/// Example:
/// ```rust
/// use toolchest::types::parse_or;
/// let x: i32 = parse_or("oops", 5);
/// assert_eq!(x, 5);
/// let y: i32 = parse_or("10", 0);
/// assert_eq!(y, 10);
/// ```
pub fn parse_or<T>(s: &str, default: T) -> T
where
    T: FromStr,
{
    s.parse().unwrap_or(default)
}

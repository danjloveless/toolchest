//! Type conversion utilities

use std::fmt::Display;
use std::str::FromStr;

/// Return value or default if None
pub fn default_to<T>(value: Option<T>, default: T) -> T {
    value.unwrap_or(default)
}

/// Convert any Display type to String safely
pub fn to_string_safe<T: Display>(value: T) -> String {
    value.to_string()
}

/// Parse string or return default value
pub fn parse_or_default<T>(s: &str) -> T
where
    T: FromStr + Default,
{
    s.parse().unwrap_or_default()
}

/// Parse string or return provided default
pub fn parse_or<T>(s: &str, default: T) -> T
where
    T: FromStr,
{
    s.parse().unwrap_or(default)
}



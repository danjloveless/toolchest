//! Rounding utilities with precision

/// Round a float to n decimal places
pub fn round(value: f64, precision: u32) -> f64 {
    let multiplier = 10_f64.powi(precision as i32);
    (value * multiplier).round() / multiplier
}

/// Floor a float to n decimal places
pub fn floor(value: f64, precision: u32) -> f64 {
    let multiplier = 10_f64.powi(precision as i32);
    (value * multiplier).floor() / multiplier
}

/// Ceil a float to n decimal places
pub fn ceil(value: f64, precision: u32) -> f64 {
    let multiplier = 10_f64.powi(precision as i32);
    (value * multiplier).ceil() / multiplier
}

/// Clamp a value between min and max
#[inline]
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Check if value is in range
pub fn in_range<T: PartialOrd>(value: T, range: core::ops::Range<T>) -> bool {
    value >= range.start && value < range.end
}

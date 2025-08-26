//! Math utilities module.
//!
//! Numeric helpers covering rounding, clamping, statistics, and more.
//!
//! Examples:
//! ```rust
//! use toolchest::math::{clamp, mean, median, round, in_range};
//! assert_eq!(clamp(15, 0, 10), 10);
//! assert_eq!(round(3.14159, 2), 3.14);
//! assert_eq!(in_range(5, 0..10), true);
//! assert_eq!(mean(&[1.0, 2.0, 3.0]), 2.0);
//! let mut vals = vec![1.0, 3.0, 2.0];
//! assert_eq!(median(&mut vals), 2.0);
//! ```

pub mod numeric;
pub mod rounding;
pub mod statistics;

pub use numeric::{approx_eq, gcd_u64, lcm_u64, signum_zero, sum_i64_saturating};
pub use rounding::{ceil, clamp, floor, in_range, round};
pub use statistics::{max_by, mean, median, min_by, percentile, std_dev, sum, variance};

//! Math utilities module

pub mod numeric;
pub mod rounding;
pub mod statistics;

pub use numeric::{approx_eq, gcd_u64, lcm_u64, signum_zero, sum_i64_saturating};
pub use rounding::{ceil, clamp, floor, in_range, round};
pub use statistics::{max_by, mean, median, min_by, percentile, std_dev, sum, variance};

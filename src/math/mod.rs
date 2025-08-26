//! Math utilities module

pub mod rounding;
pub mod statistics;
pub mod numeric;

pub use rounding::{ceil, clamp, floor, in_range, round};
pub use statistics::{max_by, mean, median, min_by, percentile, std_dev, sum, variance};
pub use numeric::{approx_eq, gcd_u64, lcm_u64, signum_zero, sum_i64_saturating};



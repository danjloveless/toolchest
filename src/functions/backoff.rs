//! Retry with exponential backoff.
//!
//! Provides [`retry_with_backoff`] which retries an operation up to a maximum
//! number of attempts, sleeping between attempts with an exponentially
//! increasing delay (doubling each time) starting from `base_delay`.
//!
//! Notes:
//! - The final error from the last attempt is returned on exhaustion.
//! - Sleep uses `std::thread::sleep`, making this a blocking API.
//! - Duration multiplication uses `saturating_mul(2)` to avoid overflow.
//!
//! Basic example:
//! ```rust
//! use toolchest::functions::retry_with_backoff;
//! use std::time::Duration;
//!
//! let mut tries = 0;
//! let result: Result<&'static str, &'static str> = retry_with_backoff(3, Duration::from_millis(5), || {
//!     tries += 1;
//!     if tries < 3 { Err("not yet") } else { Ok("ok") }
//! });
//! assert_eq!(result.unwrap(), "ok");
//! ```

use std::thread;
use std::time::Duration;

/// Retry with exponential backoff starting at `base_delay`.
///
/// - `attempts`: maximum number of times to try `op` (must be ≥ 1).
/// - `base_delay`: initial delay before the second attempt; doubles each retry.
/// - `op`: operation returning `Result<T, E>`.
///
/// Returns `Ok(T)` on the first successful attempt, or `Err(E)` from the last
/// attempt when attempts are exhausted.
///
/// Example that always fails and returns the last error:
/// ```rust
/// use toolchest::functions::retry_with_backoff;
/// use std::time::Duration;
/// let res: Result<(), &str> = retry_with_backoff(2, Duration::from_millis(1), || Err("oops"));
/// assert_eq!(res.unwrap_err(), "oops");
/// ```
pub fn retry_with_backoff<F, T, E>(
    mut attempts: u32,
    base_delay: Duration,
    mut op: F,
) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    let mut delay = base_delay;
    loop {
        match op() {
            Ok(v) => return Ok(v),
            Err(e) => {
                attempts = attempts.saturating_sub(1);
                if attempts == 0 {
                    return Err(e);
                }
                thread::sleep(delay);
                delay = delay.saturating_mul(2);
            }
        }
    }
}

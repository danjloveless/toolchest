//! Retry with fixed attempts and optional delay between tries.
//!
//! Provides [`retry`] for retrying an operation up to a fixed number of
//! attempts with an optional constant delay between attempts. Unlike
//! [`retry_with_backoff`](crate::functions::retry_with_backoff), the delay does
//! not increase.
//!
//! Basic example:
//! ```rust
//! use toolchest::functions::retry;
//! use std::time::Duration;
//!
//! let mut n = 0u32;
//! let res: Result<u32, &str> = retry(3, Some(Duration::from_millis(1)), || {
//!     n += 1;
//!     if n < 3 { Err("fail") } else { Ok(n) }
//! });
//! assert_eq!(res.unwrap(), 3);
//! ```

use std::thread;
use std::time::Duration;

/// Retry an operation up to `attempts` with optional fixed delay.
///
/// - `attempts`: maximum number of tries (must be ≥ 1)
/// - `delay`: fixed `Duration` to sleep after each failed attempt
/// - `op`: closure returning `Result<T, E>`
///
/// Returns `Ok(T)` on the first successful attempt. On exhaustion, returns the
/// last error from `op`.
///
/// Example that fails without delay:
/// ```rust
/// use toolchest::functions::retry;
/// let res: Result<(), &str> = retry(2, None, || Err("nope"));
/// assert_eq!(res.unwrap_err(), "nope");
/// ```
pub fn retry<F, T, E>(mut attempts: u32, delay: Option<Duration>, mut op: F) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    loop {
        match op() {
            Ok(v) => return Ok(v),
            Err(e) => {
                attempts = attempts.saturating_sub(1);
                if attempts == 0 {
                    return Err(e);
                }
                if let Some(d) = delay {
                    thread::sleep(d);
                }
            }
        }
    }
}

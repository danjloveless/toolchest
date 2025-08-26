//! Retry with exponential backoff

use std::thread;
use std::time::Duration;

/// Retry with exponential backoff starting at `base_delay`
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

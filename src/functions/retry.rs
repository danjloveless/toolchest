//! Retry with fixed attempts and optional delay between tries

use std::thread;
use std::time::Duration;

/// Retry an operation up to `attempts` with optional fixed delay
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



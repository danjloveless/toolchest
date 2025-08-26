//! Token-bucket rate limiting utilities.
//!
//! This module provides a lightweight, thread-safe token-bucket [`RateLimiter`]
//! implementation. A token bucket accumulates tokens over time at a fixed rate
//! up to a maximum capacity. Each permitted action consumes one token. If the
//! bucket is empty, the action is not allowed.
//!
//! - Useful for protecting external services, APIs, or any resource from being
//!   overwhelmed.
//! - Non-blocking: [`RateLimiter::allow`] returns immediately with a boolean.
//! - Thread-safe: internal state is protected by `Mutex`es wrapped in `Arc`s so
//!   a single limiter can be shared across threads.
//!
//! Basic example:
//!
//! ```rust
//! use toolchest::functions::RateLimiter;
//! use std::time::Duration;
//! use std::thread::sleep;
//!
//! // Capacity: 2 tokens; Refill rate: 5 tokens/second
//! let limiter = RateLimiter::new(2, 5);
//!
//! assert!(limiter.allow()); // consume 1
//! assert!(limiter.allow()); // consume 2 (bucket now empty)
//! assert!(!limiter.allow()); // no tokens available
//!
//! // After ~300ms at 5 tokens/sec, ~1.5 tokens accumulate (clamped to capacity)
//! sleep(Duration::from_millis(300));
//! assert!(limiter.allow()); // now allowed again
//! ```
//!
//! Sharing across threads:
//!
//! ```rust
//! use toolchest::functions::RateLimiter;
//! use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
//! use std::thread;
//! use std::time::Duration;
//!
//! let limiter = Arc::new(RateLimiter::new(1, 2)); // 1 token capacity, 2/sec refill
//! let hits = Arc::new(AtomicUsize::new(0));
//!
//! let mut handles = Vec::new();
//! for _ in 0..3 {
//!     let limiter_cloned = Arc::clone(&limiter);
//!     let hits_cloned = Arc::clone(&hits);
//!     handles.push(thread::spawn(move || {
//!         if limiter_cloned.allow() {
//!             hits_cloned.fetch_add(1, Ordering::SeqCst);
//!         }
//!     }));
//! }
//! for h in handles { h.join().unwrap(); }
//!
//! // At most 1 immediate hit (capacity 1). After a short wait, more would be allowed.
//! assert!(hits.load(Ordering::SeqCst) <= 1);
//! ```

use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Token-bucket rate limiter.
///
/// - `capacity`: maximum number of tokens the bucket can hold
/// - `refill_per_sec`: number of tokens replenished per second
///
/// Tokens are tracked with fractional precision, so refills are smooth over
/// time. Methods are thread-safe and can be called from multiple threads.
pub struct RateLimiter {
    capacity: u32,
    tokens: Arc<Mutex<f64>>, // allow fractional refill
    refill_per_sec: f64,
    last_refill: Arc<Mutex<Instant>>,
}

impl RateLimiter {
    /// Create a token-bucket limiter.
    ///
    /// - `capacity`: maximum number of tokens held at any time (must be ≥ 1).
    /// - `refill_per_second`: how many tokens are added per second.
    ///
    /// The bucket starts full at `capacity`.
    ///
    /// Example:
    ///
    /// ```rust
    /// use toolchest::functions::RateLimiter;
    /// let limiter = RateLimiter::new(10, 20); // capacity:10, refill:20 tokens/sec
    /// assert!(limiter.allow());
    /// ```
    pub fn new(capacity: u32, refill_per_second: u32) -> Self {
        Self {
            capacity,
            tokens: Arc::new(Mutex::new(capacity as f64)),
            refill_per_sec: refill_per_second as f64,
            last_refill: Arc::new(Mutex::new(Instant::now())),
        }
    }

    fn refill(&self) {
        let mut last = self.last_refill.lock().unwrap();
        let elapsed = last.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            let mut tk = self.tokens.lock().unwrap();
            *tk = (*tk + elapsed * self.refill_per_sec).min(self.capacity as f64);
            *last = Instant::now();
        }
    }

    /// Attempt to consume a single token.
    ///
    /// Returns `true` if a token was available and consumed, `false` otherwise.
    /// This method does not block; if you need to wait until a token is
    /// available, implement a retry with sleep using your desired backoff.
    ///
    /// Example:
    ///
    /// ```rust
    /// use toolchest::functions::RateLimiter;
    /// use std::thread::sleep;
    /// use std::time::Duration;
    ///
    /// let limiter = RateLimiter::new(2, 5);
    /// assert!(limiter.allow());
    /// assert!(limiter.allow());
    /// assert!(!limiter.allow());
    /// sleep(Duration::from_millis(250));
    /// assert!(limiter.allow());
    /// ```
    pub fn allow(&self) -> bool {
        self.refill();
        let mut tk = self.tokens.lock().unwrap();
        if *tk >= 1.0 {
            *tk -= 1.0;
            true
        } else {
            false
        }
    }
}

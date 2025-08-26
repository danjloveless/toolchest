//! Throttling utilities.
//!
//! Throttling ensures a function is not invoked more often than a specified
//! time interval. This is helpful when reacting to bursty events (e.g., UI
//! inputs, filesystem notifications, or rapid network callbacks) where only
//! occasional execution is desired.
//!
//! This module exposes:
//! - [`Throttled`] — a wrapper storing the underlying function and throttle state.
//! - [`throttle`] — a convenience constructor returning a [`Throttled`] instance.
//!
//! Behavior:
//! - The first `call` executes immediately.
//! - Subsequent `call`s within the configured `delay` are ignored.
//! - Once `delay` has elapsed since the last successful execution, the next
//!   `call` will execute again.
//!
//! Basic example:
//!
//! ```rust
//! use toolchest::functions::throttle;
//! use std::time::Duration;
//! use std::sync::atomic::{AtomicUsize, Ordering};
//!
//! let counter = AtomicUsize::new(0);
//! let throttled = throttle(|| { counter.fetch_add(1, Ordering::SeqCst); }, Duration::from_millis(200));
//!
//! throttled.call(); // executes, counter = 1
//! throttled.call(); // ignored (within 200ms)
//! assert_eq!(counter.load(Ordering::SeqCst), 1);
//! ```
//!
//! With sleeping to demonstrate subsequent execution:
//!
//! ```rust
//! use toolchest::functions::throttle;
//! use std::time::Duration;
//! use std::thread::sleep;
//! use std::sync::atomic::{AtomicUsize, Ordering};
//!
//! let counter = AtomicUsize::new(0);
//! let throttled = throttle(|| { counter.fetch_add(1, Ordering::SeqCst); }, Duration::from_millis(50));
//! throttled.call(); // executes (1)
//! throttled.call(); // ignored
//! sleep(Duration::from_millis(60));
//! throttled.call(); // executes (2)
//! assert_eq!(counter.load(Ordering::SeqCst), 2);
//! ```

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// A throttled function wrapper.
///
/// Stores the function and timing state used to ensure calls do not execute
/// more frequently than the configured delay interval.
pub struct Throttled<F> {
    pub(crate) func: F,
    pub(crate) delay: Duration,
    pub(crate) last_call: Arc<Mutex<Option<Instant>>>,
}

impl<F> Throttled<F>
where
    F: Fn(),
{
    /// Invoke the throttled function.
    ///
    /// Executes the wrapped function immediately on the first call. On
    /// subsequent calls, execution occurs only if at least `delay` time has
    /// passed since the last successful execution; otherwise the call is
    /// ignored.
    ///
    /// Example:
    /// ```rust
    /// use toolchest::functions::throttle;
    /// use std::time::Duration;
    /// use std::sync::atomic::{AtomicUsize, Ordering};
    /// let x = AtomicUsize::new(0);
    /// let t = throttle(|| { x.fetch_add(1, Ordering::SeqCst); }, Duration::from_millis(100));
    /// t.call(); // executes
    /// t.call(); // ignored
    /// assert_eq!(x.load(Ordering::SeqCst), 1);
    /// ```
    pub fn call(&self) {
        let should_execute = {
            let mut last = self.last_call.lock().unwrap();
            match *last {
                None => {
                    *last = Some(Instant::now());
                    true
                }
                Some(last_instant) => {
                    if last_instant.elapsed() >= self.delay {
                        *last = Some(Instant::now());
                        true
                    } else {
                        false
                    }
                }
            }
        };

        if should_execute {
            (self.func)();
        }
    }
}

/// Create a throttled wrapper around `func` with the provided `delay`.
///
/// The returned [`Throttled`] instance can be reused and shared across
/// threads if needed (the closure `F` must be `Sync` to share across threads).
///
/// Example:
/// ```rust
/// use toolchest::functions::throttle;
/// use std::time::Duration;
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// let c = AtomicUsize::new(0);
/// let t = throttle(|| { c.fetch_add(1, Ordering::SeqCst); }, Duration::from_millis(10));
/// t.call();
/// assert_eq!(c.load(Ordering::SeqCst), 1);
/// ```
pub fn throttle<F>(func: F, delay: Duration) -> Throttled<F>
where
    F: Fn(),
{
    Throttled {
        func,
        delay,
        last_call: Arc::new(Mutex::new(None)),
    }
}

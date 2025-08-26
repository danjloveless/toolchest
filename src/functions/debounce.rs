//! Debouncing utilities.
//!
//! Debouncing delays the execution of a function until a quiet period has
//! elapsed. Repeated calls reset the timer; only the last call in a burst will
//! execute after the delay.
//!
//! This module exposes:
//! - [`Debounced`] — a handle that schedules execution based on calls.
//! - [`debounce`] — constructor producing a [`Debounced`] instance.
//!
//! Behavior:
//! - Each `call` schedules execution at `now + delay` and cancels any previously
//!   scheduled run that has not yet executed.
//! - The first call starts a background worker thread to manage timing.
//! - The wrapped function executes on that worker thread.
//!
//! Basic example:
//! ```rust
//! use toolchest::functions::debounce;
//! use std::time::Duration;
//! use std::thread::sleep;
//! use std::sync::atomic::{AtomicUsize, Ordering};
//! use std::sync::Arc;
//!
//! let counter = Arc::new(AtomicUsize::new(0));
//! let c = Arc::clone(&counter);
//! let d = debounce(move || { c.fetch_add(1, Ordering::SeqCst); }, Duration::from_millis(20));
//! d.call();
//! d.call();
//! d.call();
//! // Only one execution after the quiet period
//! sleep(Duration::from_millis(40));
//! assert_eq!(counter.load(Ordering::SeqCst), 1);
//! ```

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

/// A debounced function wrapper.
///
/// Maintains a deadline that is pushed forward on each `call`. When the
/// deadline elapses without further calls, the wrapped function is executed on
/// a background thread exactly once for that burst.
pub struct Debounced<F> {
    func: Arc<Mutex<F>>,
    delay: Duration,
    // Background worker coordination
    deadline: Arc<(Mutex<Option<Instant>>, Condvar)>,
    started: Arc<AtomicBool>,
}

impl<F> Debounced<F>
where
    F: Fn() + Send + 'static,
{
    /// Invoke the debounced function; schedules execution after the delay if no newer call occurs.
    ///
    /// The function will execute on a background worker thread when the quiet
    /// period elapses. Multiple rapid `call`s collapse into a single execution.
    ///
    /// Example:
    /// ```rust
    /// use toolchest::functions::debounce;
    /// use std::time::Duration;
    /// use std::thread::sleep;
    /// use std::sync::atomic::{AtomicUsize, Ordering};
    /// use std::sync::Arc;
    /// let x = Arc::new(AtomicUsize::new(0));
    /// let xc = Arc::clone(&x);
    /// let d = debounce(move || { xc.fetch_add(1, Ordering::SeqCst); }, Duration::from_millis(10));
    /// d.call();
    /// d.call();
    /// sleep(Duration::from_millis(50));
    /// assert_eq!(x.load(Ordering::SeqCst), 1);
    /// ```
    pub fn call(&self) {
        // Update deadline to now + delay and notify worker
        {
            let (lock, cvar) = &*self.deadline;
            let mut dl = lock.lock().unwrap();
            *dl = Some(Instant::now() + self.delay);
            cvar.notify_one();
        }

        // Start worker once
        if !self.started.swap(true, Ordering::SeqCst) {
            let func = Arc::clone(&self.func);
            let deadline = Arc::clone(&self.deadline);
            thread::spawn(move || loop {
                let (lock, cvar) = &*deadline;
                // Wait for a deadline to be set
                let mut dl = lock.lock().unwrap();
                while dl.is_none() {
                    dl = cvar.wait(dl).unwrap();
                }
                // Wait until the current deadline elapses, but extend if updated
                while let Some(target) = *dl {
                    let now = Instant::now();
                    if now >= target {
                        break;
                    }
                    let dur = target.saturating_duration_since(now);
                    let (new_dl, _timeout_res) = cvar.wait_timeout(dl, dur).unwrap();
                    dl = new_dl;
                    // If deadline was updated during wait, loop continues
                }
                // Clear deadline so next burst sets a new one
                *dl = None;
                drop(dl);
                // Execute debounced function once
                let f = func.lock().unwrap();
                (*f)();
            });
        }
    }
}

/// Create a debounced version of a function.
///
/// Returns a [`Debounced`] handle that schedules the provided function to run
/// after a quiet period of `delay` following the last `call`.
///
/// Example:
/// ```rust
/// use toolchest::functions::debounce;
/// use std::time::Duration;
/// let d = debounce(move || println!("run once after quiet period"), Duration::from_millis(5));
/// d.call();
/// ```
pub fn debounce<F>(func: F, delay: Duration) -> Debounced<F>
where
    F: Fn() + Send + 'static,
{
    Debounced {
        func: Arc::new(Mutex::new(func)),
        delay,
        deadline: Arc::new((Mutex::new(None), Condvar::new())),
        started: Arc::new(AtomicBool::new(false)),
    }
}

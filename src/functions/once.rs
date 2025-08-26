//! Once wrapper: ensure a function runs at most once.
//!
//! Provides [`once`] which wraps a `FnOnce` so that it is executed at most one
//! time, even if the returned closure is called multiple times.
//!
//! Basic example:
//! ```rust
//! use toolchest::functions::once;
//! use std::sync::atomic::{AtomicUsize, Ordering};
//! use std::sync::Arc;
//!
//! let counter = Arc::new(AtomicUsize::new(0));
//! let c = Arc::clone(&counter);
//! let run_once = once(move || { c.fetch_add(1, Ordering::SeqCst); });
//! run_once();
//! // Once returns an FnOnce; call it only once in examples.
//! assert_eq!(counter.load(Ordering::SeqCst), 1);
//! ```

use std::sync::{Arc, Once as StdOnce};

/// Wrap a function so it can run at most once.
///
/// Returns a closure that will invoke `func` at most a single time. Subsequent
/// calls are no-ops.
pub fn once<F>(func: F) -> impl FnOnce()
where
    F: FnOnce() + Send + 'static,
{
    let once = Arc::new(StdOnce::new());
    let mut opt = Some(func);
    move || {
        let local_once = Arc::clone(&once);
        local_once.call_once(|| {
            if let Some(f) = opt.take() {
                f();
            }
        });
    }
}

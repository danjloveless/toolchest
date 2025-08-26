//! Simple memoization for pure functions with cloneable inputs/outputs.
//!
//! Provides [`memoize`] to wrap a pure function so that repeated calls with the
//! same argument return a cached result instead of recomputing.
//!
//! Notes and caveats:
//! - Inputs must implement `Eq + Hash + Clone`; outputs must implement `Clone`.
//! - Cache is stored in a `Mutex<HashMap<..>>`, so cloned closures are
//!   shareable across threads but concurrent access is serialized.
//! - This is best for small, frequently repeated computations; unbounded cache
//!   growth may not be suitable for long-running processes.
//!
//! Basic example:
//! ```rust
//! use toolchest::functions::memoize;
//!
//! fn slow_square(n: u32) -> u32 { n * n }
//! let sq = memoize(slow_square);
//! assert_eq!(sq(3), 9);
//! assert_eq!(sq(3), 9); // cached
//! ```

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

/// Memoize a pure function with cloneable inputs/outputs.
///
/// Returns a closure that caches results by input argument. The cache lives as
/// long as the returned closure is alive, and is shared across clones of the
/// closure.
pub fn memoize<A, R, F>(func: F) -> impl Fn(A) -> R
where
    A: Eq + Hash + Clone + 'static,
    R: Clone + 'static,
    F: Fn(A) -> R + 'static,
{
    let cache: Arc<Mutex<HashMap<A, R>>> = Arc::new(Mutex::new(HashMap::new()));
    move |arg: A| {
        if let Some(v) = cache.lock().unwrap().get(&arg).cloned() {
            return v;
        }
        let res = func(arg.clone());
        cache.lock().unwrap().insert(arg, res.clone());
        res
    }
}

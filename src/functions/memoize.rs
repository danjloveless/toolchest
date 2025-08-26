//! Simple memoization for pure functions with Cloneable inputs/outputs

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

/// Memoize a pure function with Cloneable inputs/outputs
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



//! Once wrapper: ensure a function runs at most once

use std::sync::{Arc, Once as StdOnce};

/// Wrap a function so it can run at most once
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



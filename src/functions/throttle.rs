//! Throttle implementation

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// A throttled function wrapper
pub struct Throttled<F> {
    pub(crate) func: F,
    pub(crate) delay: Duration,
    pub(crate) last_call: Arc<Mutex<Option<Instant>>>,
}

impl<F> Throttled<F>
where
    F: Fn(),
{
    /// Invoke the throttled function; executes only if the delay has passed since last execution
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

/// Create a throttled version of a function
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

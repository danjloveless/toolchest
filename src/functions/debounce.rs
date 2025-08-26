//! Debounce implementation

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// A debounced function wrapper
pub struct Debounced<F> {
    func: Arc<Mutex<F>>,
    delay: Duration,
    call_seq: Arc<AtomicU64>,
}

impl<F> Debounced<F>
where
    F: Fn() + Send + 'static,
{
    /// Invoke the debounced function; schedules execution after the delay if no newer call occurs
    pub fn call(&self) {
        let func = Arc::clone(&self.func);
        let delay = self.delay;
        let call_seq = Arc::clone(&self.call_seq);
        // Assign a unique id to this call; only the latest id may execute after delay
        let id = call_seq.fetch_add(1, Ordering::SeqCst) + 1;

        thread::spawn(move || {
            thread::sleep(delay);
            // Only execute if no newer call occurred during the delay
            if call_seq.load(Ordering::SeqCst) == id {
                let func = func.lock().unwrap();
                (*func)();
            }
        });
    }
}

/// Create a debounced version of a function
pub fn debounce<F>(func: F, delay: Duration) -> Debounced<F>
where
    F: Fn() + Send + 'static,
{
    Debounced {
        func: Arc::new(Mutex::new(func)),
        delay,
        call_seq: Arc::new(AtomicU64::new(0)),
    }
}

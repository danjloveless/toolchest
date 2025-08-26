//! Debounce implementation

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

/// A debounced function wrapper
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
    /// Invoke the debounced function; schedules execution after the delay if no newer call occurs
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

/// Create a debounced version of a function
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

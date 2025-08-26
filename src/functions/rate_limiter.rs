//! Simple token-bucket rate limiter

use std::time::Instant;
use std::sync::{Arc, Mutex};

/// Token-bucket rate limiter
pub struct RateLimiter {
    capacity: u32,
    tokens: Arc<Mutex<f64>>, // allow fractional refill
    refill_per_sec: f64,
    last_refill: Arc<Mutex<Instant>>,
}

impl RateLimiter {
    /// Create a limiter with capacity and refill rate per second
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

    /// Try to consume one token; returns true if allowed
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



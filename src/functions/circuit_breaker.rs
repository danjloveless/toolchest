//! Basic circuit breaker

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Circuit breaker state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BreakerState {
    /// Closed: calls pass, failures counted
    Closed,
    /// Open: calls rejected until cooldown elapses
    Open,
    /// HalfOpen: probing state after cooldown
    HalfOpen,
}

/// Simple circuit breaker with failure threshold and cooldown
pub struct CircuitBreaker {
    state: Arc<Mutex<BreakerState>>,
    failures: Arc<Mutex<u32>>,
    threshold: u32,
    open_until: Arc<Mutex<Option<Instant>>>,
    cooldown: Duration,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    pub fn new(threshold: u32, cooldown: Duration) -> Self {
        Self {
            state: Arc::new(Mutex::new(BreakerState::Closed)),
            failures: Arc::new(Mutex::new(0)),
            threshold,
            open_until: Arc::new(Mutex::new(None)),
            cooldown,
        }
    }

    /// Get current state
    pub fn state(&self) -> BreakerState {
        *self.state.lock().unwrap()
    }

    /// Call an operation guarded by the breaker
    pub fn call<F, T, E>(&self, mut op: F) -> Result<T, E>
    where
        F: FnMut() -> Result<T, E>,
    {
        self.maybe_transition();
        match self.state() {
            BreakerState::Open => Err(op_err()),
            BreakerState::HalfOpen | BreakerState::Closed => match op() {
                Ok(v) => {
                    self.record_success();
                    Ok(v)
                }
                Err(e) => {
                    self.record_failure();
                    Err(e)
                }
            },
        }
    }

    fn maybe_transition(&self) {
        let mut state = self.state.lock().unwrap();
        if *state == BreakerState::Open {
            if let Some(until) = *self.open_until.lock().unwrap() {
                if Instant::now() >= until {
                    *state = BreakerState::HalfOpen;
                }
            }
        }
    }

    fn record_success(&self) {
        let mut state = self.state.lock().unwrap();
        *self.failures.lock().unwrap() = 0;
        if *state == BreakerState::HalfOpen {
            *state = BreakerState::Closed;
        }
    }

    fn record_failure(&self) {
        let mut f = self.failures.lock().unwrap();
        *f += 1;
        if *f >= self.threshold {
            *self.state.lock().unwrap() = BreakerState::Open;
            *self.open_until.lock().unwrap() = Some(Instant::now() + self.cooldown);
        }
    }
}

fn op_err<T>() -> T {
    panic!("circuit open")
}

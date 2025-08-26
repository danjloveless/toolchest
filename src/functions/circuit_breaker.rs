//! Basic circuit breaker.
//!
//! A circuit breaker guards an operation from repeated failures by opening the
//! circuit after a threshold of errors, then allowing a cooldown period before
//! probing (`HalfOpen`) and potentially closing again on success.
//!
//! States:
//! - [`BreakerState::Closed`] — calls pass through, failures are counted
//! - [`BreakerState::Open`] — calls are rejected until cooldown elapses
//! - [`BreakerState::HalfOpen`] — a probing state; success closes, failure reopens
//!
//! Basic example:
//! ```rust
//! use toolchest::functions::{CircuitBreaker, BreakerState};
//! use std::time::Duration;
//!
//! let cb = CircuitBreaker::new(1, Duration::from_millis(5));
//! // First failure opens the circuit
//! let _ = cb.call::<_, (), &str>(|| Err("boom"));
//! assert_eq!(cb.state(), BreakerState::Open);
//! ```

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::{error::Error, fmt};

/// Circuit breaker state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BreakerState {
    /// Closed: calls pass, failures counted
    Closed,
    /// Open: calls rejected until cooldown elapses
    Open,
    /// HalfOpen: probing state after cooldown
    HalfOpen,
}

/// Simple circuit breaker with failure threshold and cooldown.
///
/// - `threshold`: consecutive failure count that trips the breaker
/// - `cooldown`: duration to stay open before entering `HalfOpen`
pub struct CircuitBreaker {
    state: Arc<Mutex<BreakerState>>,
    failures: Arc<Mutex<u32>>,
    threshold: u32,
    open_until: Arc<Mutex<Option<Instant>>>,
    cooldown: Duration,
}

/// Error returned by `CircuitBreaker::call`.
#[derive(Debug)]
pub enum CircuitBreakerError<E> {
    /// The circuit is open and rejecting calls
    Open,
    /// The wrapped operation returned an error
    OperationError(E),
}

impl<E: fmt::Display> fmt::Display for CircuitBreakerError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CircuitBreakerError::Open => write!(f, "circuit open"),
            CircuitBreakerError::OperationError(e) => write!(f, "operation error: {e}"),
        }
    }
}

impl<E: fmt::Debug + fmt::Display> Error for CircuitBreakerError<E> {}

impl CircuitBreaker {
    /// Create a new circuit breaker.
    pub fn new(threshold: u32, cooldown: Duration) -> Self {
        Self {
            state: Arc::new(Mutex::new(BreakerState::Closed)),
            failures: Arc::new(Mutex::new(0)),
            threshold,
            open_until: Arc::new(Mutex::new(None)),
            cooldown,
        }
    }

    /// Get current state.
    pub fn state(&self) -> BreakerState {
        *self.state.lock().unwrap()
    }

    /// Call an operation guarded by the breaker.
    ///
    /// - On `Open`, immediately returns `Err(CircuitBreakerError::Open)`.
    /// - On `Closed`/`HalfOpen`, executes the operation; success resets failure
    ///   count (and closes if probing), failure increments count and may open.
    ///
    /// Example success path:
    /// ```rust
    /// use toolchest::functions::CircuitBreaker;
    /// use std::time::Duration;
    /// let cb = CircuitBreaker::new(2, Duration::from_millis(10));
    /// let res: Result<i32, _> = cb.call::<_, i32, &str>(|| Ok(7));
    /// assert_eq!(res.unwrap(), 7);
    /// ```
    ///
    /// Example open behavior:
    /// ```rust
    /// use toolchest::functions::{CircuitBreaker, BreakerState, CircuitBreakerError};
    /// use std::time::Duration;
    /// let cb = CircuitBreaker::new(1, Duration::from_millis(50));
    /// let _: Result<(), CircuitBreakerError<&str>> = cb.call(|| Err("boom"));
    /// assert_eq!(cb.state(), BreakerState::Open);
    /// let r: Result<(), CircuitBreakerError<&str>> = cb.call(|| Ok(()));
    /// assert!(matches!(r, Err(CircuitBreakerError::Open)));
    /// ```
    pub fn call<F, T, E>(&self, mut op: F) -> Result<T, CircuitBreakerError<E>>
    where
        F: FnMut() -> Result<T, E>,
    {
        self.maybe_transition();
        match self.state() {
            BreakerState::Open => Err(CircuitBreakerError::Open),
            BreakerState::HalfOpen | BreakerState::Closed => match op() {
                Ok(v) => {
                    self.record_success();
                    Ok(v)
                }
                Err(e) => {
                    self.record_failure();
                    Err(CircuitBreakerError::OperationError(e))
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

//! Function combinators module

pub mod debounce;
pub mod throttle;
pub mod memoize;
pub mod retry;
pub mod once;
pub mod backoff;
pub mod rate_limiter;
pub mod circuit_breaker;
pub mod timeout;

pub use debounce::{debounce, Debounced};
pub use throttle::{throttle, Throttled};
pub use memoize::memoize;
pub use retry::retry;
pub use once::once;
pub use backoff::retry_with_backoff;
pub use rate_limiter::RateLimiter;
pub use circuit_breaker::{CircuitBreaker, BreakerState};
pub use compose::{compose, pipe, tap};
pub use timeout::with_timeout;
pub mod compose;



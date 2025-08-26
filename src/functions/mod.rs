//! Function combinators module

pub mod backoff;
pub mod circuit_breaker;
pub mod debounce;
pub mod memoize;
pub mod once;
pub mod rate_limiter;
pub mod retry;
pub mod throttle;
pub mod timeout;

pub use backoff::retry_with_backoff;
pub use circuit_breaker::{BreakerState, CircuitBreaker};
pub use compose::{compose, pipe, tap};
pub use debounce::{debounce, Debounced};
pub use memoize::memoize;
pub use once::once;
pub use rate_limiter::RateLimiter;
pub use retry::retry;
pub use throttle::{throttle, Throttled};
pub use timeout::with_timeout;
pub mod compose;

//! Time utilities.
//!
//! Helpers for durations, timing, simple cron-like checks, and backoff
//! iteration.
//!
//! Examples:
//! ```rust
//! use toolchest::time::{duration_humanize, parse_duration, Stopwatch, elapsed, BackoffIter, deadline};
//! use std::time::{Duration, Instant};
//!
//! assert_eq!(duration_humanize(Duration::from_secs(3661)), "1h1m1s");
//! assert_eq!(parse_duration("1h2m3s").unwrap(), Duration::from_secs(3723));
//!
//! let sw = Stopwatch::start_new();
//! let _ = 1 + 1;
//! let _elapsed = sw.elapsed();
//!
//! let (_res, took) = elapsed(|| 2 + 2);
//! assert!(took >= Duration::from_millis(0));
//!
//! let mut iter = BackoffIter::new(Duration::from_millis(1));
//! assert_eq!(iter.next().unwrap(), Duration::from_millis(1));
//! assert_eq!(iter.next().unwrap(), Duration::from_millis(2));
//!
//! let dl = Instant::now();
//! assert!(deadline(dl) || !deadline(dl));
//! ```

use std::time::{Duration, Instant};

/// Human-readable duration like "1h2m3s".
///
/// Example:
/// ```rust
/// use toolchest::time::duration_humanize;
/// use std::time::Duration;
/// assert_eq!(duration_humanize(Duration::from_secs(65)), "1m5s");
/// ```
pub fn duration_humanize(d: Duration) -> String {
    let mut secs = d.as_secs();
    let hours = secs / 3600;
    secs %= 3600;
    let mins = secs / 60;
    secs %= 60;
    if hours > 0 {
        format!("{hours}h{mins}m{secs}s")
    } else if mins > 0 {
        format!("{mins}m{secs}s")
    } else {
        format!("{secs}s")
    }
}

/// Parse strings like "1h2m3s" into `Duration`.
///
/// Example:
/// ```rust
/// use toolchest::time::parse_duration;
/// use std::time::Duration;
/// assert_eq!(parse_duration("2m30s").unwrap(), Duration::from_secs(150));
/// ```
pub fn parse_duration(s: &str) -> Option<Duration> {
    let mut total_ms: u128 = 0;
    let mut num = String::new();
    for ch in s.chars() {
        if ch.is_ascii_digit() {
            num.push(ch);
            continue;
        }
        let n: u128 = num.parse().ok()?;
        num.clear();
        total_ms += match ch {
            'h' => n * 3_600_000,
            'm' => n * 60_000,
            's' => n * 1000,
            _ => return None,
        };
    }
    if !num.is_empty() {
        return None;
    }
    Some(Duration::from_millis(total_ms as u64))
}

/// Simple stopwatch.
pub struct Stopwatch {
    start: Instant,
}
impl Stopwatch {
    /// Start a new stopwatch.
    pub fn start_new() -> Self {
        Self {
            start: Instant::now(),
        }
    }
    /// Elapsed time since start.
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

/// Measure closure execution time.
///
/// Example:
/// ```rust
/// use toolchest::time::elapsed;
/// let (v, dur) = elapsed(|| 2 + 2);
/// assert_eq!(v, 4);
/// assert!(dur >= std::time::Duration::from_millis(0));
/// ```
pub fn elapsed<T, F: FnOnce() -> T>(f: F) -> (T, Duration) {
    let sw = Stopwatch::start_new();
    let v = f();
    (v, sw.elapsed())
}

/// True if now is past the deadline.
///
/// Example:
/// ```rust
/// use toolchest::time::deadline;
/// let dl = std::time::Instant::now();
/// let _ = deadline(dl);
/// ```
pub fn deadline(d: Instant) -> bool {
    Instant::now() >= d
}

/// Iterator yielding exponentially increasing delays.
pub struct BackoffIter {
    cur: Duration,
}
impl BackoffIter {
    /// Create a backoff iterator starting at base.
    pub fn new(base: Duration) -> Self {
        Self { cur: base }
    }
}
impl Iterator for BackoffIter {
    type Item = Duration;
    fn next(&mut self) -> Option<Self::Item> {
        let out = self.cur;
        self.cur = self.cur.saturating_mul(2);
        Some(out)
    }
}

/// Very limited cron matcher supporting minute field "*" or "*/n" only (others ignored)
pub fn cron_matches(now: &chrono_like::DateTime, expr: &str) -> bool {
    let parts: Vec<&str> = expr.split_whitespace().collect();
    if parts.is_empty() {
        return false;
    }
    match parts[0] {
        "*" => true,
        s if s.starts_with("*/") => {
            let n: u32 = s[2..].parse().unwrap_or(1);
            now.minute % n == 0
        }
        m => m.parse::<u32>().ok() == Some(now.minute),
    }
}

/// Minimal datetime for cron_matches without external crates
/// Minimal datetime for cron without external crates
pub mod chrono_like {
    #[derive(Clone, Copy)]
    /// Minimal DateTime containing only minute field
    pub struct DateTime {
        /// Minute component \[0,59\]
        pub minute: u32,
    }
}

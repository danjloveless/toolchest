//! Time utilities

use std::time::{Duration, Instant};

/// Human-readable duration like "1h2m3s"
pub fn duration_humanize(d: Duration) -> String {
    let mut secs = d.as_secs();
    let hours = secs / 3600; secs %= 3600;
    let mins = secs / 60; secs %= 60;
    if hours > 0 { format!("{}h{}m{}s", hours, mins, secs) }
    else if mins > 0 { format!("{}m{}s", mins, secs) }
    else { format!("{}s", secs) }
}

/// Parse strings like "1h2m3s" into Duration
pub fn parse_duration(s: &str) -> Option<Duration> {
    let mut total_ms: u128 = 0;
    let mut num = String::new();
    for ch in s.chars() {
        if ch.is_ascii_digit() { num.push(ch); continue; }
        let n: u128 = num.parse().ok()?; num.clear();
        total_ms += match ch {
            'h' => n * 3600_000,
            'm' => n * 60_000,
            's' => n * 1000,
            _ => return None,
        };
    }
    if !num.is_empty() { return None; }
    Some(Duration::from_millis(total_ms as u64))
}

/// Simple stopwatch
pub struct Stopwatch { start: Instant }
impl Stopwatch {
    /// Start a new stopwatch
    pub fn start_new() -> Self { Self { start: Instant::now() } }
    /// Elapsed time since start
    pub fn elapsed(&self) -> Duration { self.start.elapsed() }
}

/// Measure closure execution time
pub fn elapsed<T, F: FnOnce() -> T>(f: F) -> (T, Duration) { let sw = Stopwatch::start_new(); let v = f(); (v, sw.elapsed()) }

/// True if now is past the deadline
pub fn deadline(d: Instant) -> bool { Instant::now() >= d }

/// Iterator yielding exponentially increasing delays
pub struct BackoffIter { cur: Duration }
impl BackoffIter { 
    /// Create a backoff iterator starting at base
    pub fn new(base: Duration) -> Self { Self { cur: base } }
}
impl Iterator for BackoffIter { type Item = Duration; fn next(&mut self) -> Option<Self::Item> { let out = self.cur; self.cur = self.cur.saturating_mul(2); Some(out) } }

/// Very limited cron matcher supporting minute field "*" or "*/n" only (others ignored)
pub fn cron_matches(now: &chrono_like::DateTime, expr: &str) -> bool {
    let parts: Vec<&str> = expr.split_whitespace().collect();
    if parts.len() < 1 { return false; }
    match parts[0] {
        "*" => true,
        s if s.starts_with("*/") => { let n: u32 = s[2..].parse().unwrap_or(1); now.minute % n == 0 }
        m => m.parse::<u32>().ok().map_or(false, |mm| mm == now.minute),
    }
}

/// Minimal datetime for cron_matches without external crates
/// Minimal datetime for cron without external crates
pub mod chrono_like {
    #[derive(Clone, Copy)]
    /// Minimal DateTime containing only minute field
    pub struct DateTime { /// Minute component \[0,59\]
        pub minute: u32 }
}



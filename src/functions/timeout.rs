//! Execute a closure with timeout (thread-based).
//!
//! Spawns a worker thread to run the provided closure and waits up to the
//! specified duration for a result. If the timeout elapses first, returns
//! `None`; otherwise returns `Some(T)`.
//!
//! Notes:
//! - Uses a channel to communicate the result. The worker thread is not
//!   cancelled on timeout; it simply continues running to completion in the
//!   background.
//! - `T` must be `Send + 'static` because it crosses the thread boundary.
//! - This is a simple building block; for cancellation, consider additional
//!   coordination.
//!
//! Basic example:
//! ```rust
//! use toolchest::functions::with_timeout;
//! use std::time::Duration;
//! use std::thread::sleep;
//!
//! let res = with_timeout(Duration::from_millis(10), || 42);
//! assert_eq!(res, Some(42));
//!
//! let slow = with_timeout(Duration::from_millis(1), || {
//!     sleep(Duration::from_millis(50));
//!     5
//! });
//! assert_eq!(slow, None);
//! ```

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Run closure `f` with a timeout; returns `None` on timeout.
///
/// Spawns a thread to execute `f` and waits up to `dur` for a result. If the
/// timeout elapses first, `None` is returned; otherwise `Some(T)`.
pub fn with_timeout<T, F>(dur: Duration, f: F) -> Option<T>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let res = f();
        let _ = tx.send(res);
    });
    rx.recv_timeout(dur).ok()
}

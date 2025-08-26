//! Execute a closure with timeout (thread-based)

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Run closure `f` with a timeout; returns None on timeout
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

use std::sync::{Arc, Mutex};
use std::time::Duration;
use toolchest::functions::*;

#[test]
fn test_debounce_basic() {
    let counter = Arc::new(Mutex::new(0u32));
    let c2 = Arc::clone(&counter);
    let debounced = debounce(
        move || {
            let mut v = c2.lock().unwrap();
            *v += 1;
        },
        Duration::from_millis(50),
    );

    debounced.call();
    debounced.call();
    std::thread::sleep(Duration::from_millis(80));

    assert_eq!(*counter.lock().unwrap(), 1);
}

#[test]
fn test_rate_limiter() {
    let rl = RateLimiter::new(2, 10);
    assert!(rl.allow());
    assert!(rl.allow());
    assert!(!rl.allow());
}

#[test]
fn test_circuit_breaker_opens() {
    let cb = CircuitBreaker::new(1, Duration::from_millis(10));
    let _: Result<(), CircuitBreakerError<()>> = cb.call::<_, (), ()>(|| Err(()));
    // Next call returns Open because circuit is open
    let res: Result<(), CircuitBreakerError<()>> = cb.call::<_, (), ()>(|| Ok(()));
    match res {
        Err(CircuitBreakerError::Open) => {}
        _ => panic!("expected CircuitBreakerError::Open"),
    }
}
#[test]
fn test_memoize_basic() {
    use std::sync::atomic::{AtomicU32, Ordering};
    static CALLS: AtomicU32 = AtomicU32::new(0);
    let f = memoize(|x: u32| {
        CALLS.fetch_add(1, Ordering::SeqCst);
        x * 2
    });
    assert_eq!(f(2), 4);
    assert_eq!(f(2), 4);
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
}

#[test]
fn test_compose_pipe_tap() {
    let f = |x: i32| x + 1;
    let g = |y: i32| y * 2;
    let h = compose(g, f);
    assert_eq!(h(3), 8);
    let v = pipe(3, f, g);
    assert_eq!(v, 8);
    use std::cell::Cell;
    let seen = Cell::new(0);
    let v = tap(5, |r| {
        seen.set(*r);
    });
    assert_eq!(v, 5);
    assert_eq!(seen.get(), 5);
}

#[test]
fn test_with_timeout() {
    let res = with_timeout(Duration::from_millis(50), || 42);
    assert_eq!(res, Some(42));
    let res = with_timeout(Duration::from_millis(10), || {
        std::thread::sleep(Duration::from_millis(50));
        1
    });
    assert_eq!(res, None);
}

#[test]
fn test_retry_with_backoff() {
    let mut attempts = 0u32;
    let res = retry_with_backoff(3, Duration::from_millis(1), || {
        attempts += 1;
        if attempts < 2 {
            Err(())
        } else {
            Ok(7)
        }
    });
    assert_eq!(res, Ok(7));
}

#[test]
fn test_retry() {
    let mut attempts = 0u32;
    let res = retry(3, None, || {
        attempts += 1;
        if attempts < 3 {
            Err(())
        } else {
            Ok(42)
        }
    });
    assert_eq!(res, Ok(42));
}

#[test]
fn test_throttle_basic() {
    let counter = Arc::new(Mutex::new(0u32));
    let c2 = Arc::clone(&counter);
    let throttled = throttle(
        move || {
            let mut v = c2.lock().unwrap();
            *v += 1;
        },
        Duration::from_millis(50),
    );

    throttled.call();
    throttled.call();
    assert_eq!(*counter.lock().unwrap(), 1);

    std::thread::sleep(Duration::from_millis(60));
    throttled.call();
    assert_eq!(*counter.lock().unwrap(), 2);
}

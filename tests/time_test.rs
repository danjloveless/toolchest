use std::time::{Duration, Instant};
use toolchest::time::*;

#[test]
fn test_duration_parse_humanize() {
    assert_eq!(parse_duration("1h30m").unwrap(), Duration::from_secs(5400));
    let s = duration_humanize(Duration::from_secs(3661));
    assert!(s.contains("1h"));
}

#[test]
fn test_elapsed_and_deadline() {
    let (v, d) = elapsed(|| 42);
    assert_eq!(v, 42);
    assert!(d <= Duration::from_secs(1));
    assert_eq!(deadline(Instant::now() - Duration::from_secs(1)), true);
}

#[test]
fn test_backoff_iter() {
    let mut it = BackoffIter::new(Duration::from_millis(10));
    assert_eq!(it.next().unwrap(), Duration::from_millis(10));
    assert_eq!(it.next().unwrap(), Duration::from_millis(20));
}

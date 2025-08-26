use toolchest::math::*;

#[test]
fn test_rounding() {
    assert_eq!(round(3.14159, 2), 3.14);
    assert_eq!(floor(3.149, 2), 3.14);
    assert_eq!(ceil(3.141, 2), 3.15);
}

#[test]
fn test_clamp_in_range() {
    assert_eq!(clamp(5, 0, 10), 5);
    assert_eq!(clamp(-1, 0, 10), 0);
    assert_eq!(clamp(11, 0, 10), 10);
}

#[test]
fn test_stats() {
    let mut data = vec![3.0, 1.0, 2.0];
    assert_eq!(sum(&data), 6.0);
    assert_eq!(mean(&data), 2.0);
    assert_eq!(median(&mut data), 2.0);
    assert!((variance(&[1.0, 2.0, 3.0]) - 2.0 / 3.0).abs() < 1e-9);
    assert!((std_dev(&[1.0, 2.0, 3.0]) - (2.0_f64 / 3.0).sqrt()).abs() < 1e-9);
}

#[test]
fn test_percentile_and_numeric() {
    let mut data = vec![10.0, 20.0, 30.0, 40.0];
    assert_eq!(percentile(&mut data, 50.0), 30.0);
    assert!(approx_eq(0.3 + 0.6, 0.9, 1e-9));
    assert_eq!(signum_zero(0i32), 0);
    assert_eq!(signum_zero(5i32), 1);
    assert_eq!(signum_zero(-7i32), -1);
    assert_eq!(sum_i64_saturating(&[i64::MAX, 1]), i64::MAX);
    assert_eq!(gcd_u64(12, 18), 6);
    assert_eq!(lcm_u64(12, 18), 36);
}



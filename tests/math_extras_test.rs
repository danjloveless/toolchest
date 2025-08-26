use toolchest::math::*;

#[test]
fn test_math_extras() {
    assert_eq!(numeric::lerp(0.0, 10.0, 0.5), 5.0);
    assert_eq!(numeric::map_range(5.0, 0.0, 10.0, 0.0, 100.0), 50.0);
    assert_eq!(numeric::factorial(5), 120);
    assert!(numeric::is_prime(97));
    assert_eq!(numeric::next_prime(100), 101);
    assert_eq!(numeric::prev_prime(100), Some(97));
    assert_eq!(numeric::fibonacci(10), 55);
    assert!(numeric::is_even(4));
    assert!(numeric::is_odd(5));
    assert_eq!(numeric::degrees_to_radians(180.0), std::f64::consts::PI);
    assert_eq!(numeric::radians_to_degrees(std::f64::consts::PI), 180.0);
    assert!((numeric::sigmoid(0.0) - 0.5).abs() < 1e-12);
    assert_eq!(numeric::normalize(5.0, 0.0, 10.0), 0.5);
    assert_eq!(numeric::euclidean_distance(&[0.0, 0.0], &[3.0, 4.0]), 5.0);
    assert_eq!(numeric::manhattan_distance(&[0.0, 0.0], &[3.0, 4.0]), 7.0);
    assert_eq!(
        numeric::dot_product(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]),
        32.0
    );
    assert_eq!(
        numeric::cross_product([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        [0.0, 0.0, 1.0]
    );
}

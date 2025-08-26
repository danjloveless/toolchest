//! Numeric helper utilities

/// Approximately equal for f64 within epsilon
pub fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() <= eps
}

/// Signum with zero for integers
pub fn signum_zero<T>(value: T) -> i8
where
    T: PartialOrd + From<i8>,
{
    if value > T::from(0) {
        1
    } else if value < T::from(0) {
        -1
    } else {
        0
    }
}

/// Saturating sum for i64 slice
pub fn sum_i64_saturating(values: &[i64]) -> i64 {
    values.iter().fold(0i64, |acc, &x| acc.saturating_add(x))
}

/// Greatest common divisor (Euclidean algorithm)
pub fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Least common multiple
pub fn lcm_u64(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        a / gcd_u64(a, b) * b
    }
}

/// Linear interpolation between a and b by t in \[0,1\]
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Map value from one range to another
pub fn map_range(x: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    if in_max == in_min {
        return out_min;
    }
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

/// Factorial for n up to 20 (fits in u128)
pub fn factorial(n: u32) -> u128 {
    (1..=n as u128).product()
}

/// Check primality by trial division
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut d = 3u64;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

/// Next prime greater than or equal to n
pub fn next_prime(mut n: u64) -> u64 {
    if n <= 2 {
        return 2;
    }
    if n % 2 == 0 {
        n += 1;
    }
    while !is_prime(n) {
        n += 2;
    }
    n
}
/// Previous prime strictly less than n
pub fn prev_prime(mut n: u64) -> Option<u64> {
    if n <= 2 {
        return None;
    }
    if n % 2 == 0 {
        n -= 1;
    }
    while n >= 2 && !is_prime(n) {
        n = n.saturating_sub(2);
    }
    if n >= 2 {
        Some(n)
    } else {
        None
    }
}

/// n-th Fibonacci number (F(0)=0)
pub fn fibonacci(n: u32) -> u128 {
    let (mut a, mut b) = (0u128, 1u128);
    for _ in 0..n {
        let t = a + b;
        a = b;
        b = t;
    }
    a
}

/// True if n is even
pub fn is_even(n: i64) -> bool {
    n % 2 == 0
}
/// True if n is odd
pub fn is_odd(n: i64) -> bool {
    n % 2 != 0
}

/// Convert degrees to radians
pub fn degrees_to_radians(deg: f64) -> f64 {
    deg.to_radians()
}
/// Convert radians to degrees
pub fn radians_to_degrees(rad: f64) -> f64 {
    rad.to_degrees()
}

/// Logistic sigmoid function
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}
/// Normalize x to \[0,1\] given min and max
pub fn normalize(x: f64, min: f64, max: f64) -> f64 {
    if max == min {
        0.0
    } else {
        (x - min) / (max - min)
    }
}

/// Euclidean distance between equal-length vectors
pub fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b)
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}
/// Manhattan distance between equal-length vectors
pub fn manhattan_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| (x - y).abs()).sum()
}
/// Dot product of two vectors
pub fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}
/// 3D cross product
pub fn cross_product(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

//! Statistical utilities

/// Calculate sum of numeric slice
pub fn sum<T>(values: &[T]) -> T
where
    T: Default + Copy + core::ops::Add<Output = T>,
{
    values.iter().fold(T::default(), |acc, &x| acc + x)
}

/// Calculate mean of numeric slice
pub fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    sum(values) / values.len() as f64
}

/// Calculate population variance (uses mean of squares minus square of mean)
pub fn variance(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let n = values.len() as f64;
    let sum_x = sum(values);
    let sum_x2: f64 = values.iter().map(|v| v * v).sum();
    (sum_x2 / n) - (sum_x / n).powi(2)
}

/// Calculate population standard deviation
pub fn std_dev(values: &[f64]) -> f64 {
    variance(values).sqrt()
}

/// Calculate median (requires mutable for sorting)
pub fn median(values: &mut [f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }

    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = values.len() / 2;

    if values.len() % 2 == 0 {
        (values[mid - 1] + values[mid]) / 2.0
    } else {
        values[mid]
    }
}

/// Find minimum by key function
pub fn min_by<T, F, K>(items: &[T], f: F) -> Option<&T>
where
    F: Fn(&T) -> K,
    K: Ord,
{
    items.iter().min_by_key(|item| f(item))
}

/// Find maximum by key function
pub fn max_by<T, F, K>(items: &[T], f: F) -> Option<&T>
where
    F: Fn(&T) -> K,
    K: Ord,
{
    items.iter().max_by_key(|item| f(item))
}

/// Percentile (0.0..=100.0). Uses nearest-rank method.
pub fn percentile(values: &mut [f64], p: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let p = p.clamp(0.0, 100.0);
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if values.len() == 1 {
        return values[0];
    }
    let pos = (p / 100.0) * ((values.len() - 1) as f64);
    let idx = pos.round() as usize;
    values[idx]
}

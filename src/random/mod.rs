//! Random utilities (non-cryptographic).
//!
//! Pseudo-random helpers for quick sampling, choices, and UUID-like IDs. These
//! are not cryptographically secure and should not be used for security-
//! sensitive purposes.
//!
//! Examples:
//! ```rust
//! use toolchest::random::{random_range, random_bool, random_choice, random_choices, uuid_v4, random_bytes};
//!
//! let r = random_range(5, 10);
//! assert!(r >= 5 && r < 10);
//! let _ = random_bool(0.5);
//! let v = vec![1,2,3];
//! let _ = random_choice(&v);
//! let xs = random_choices(&v, 5);
//! assert_eq!(xs.len(), 5);
//! let id = uuid_v4();
//! assert_eq!(id.len(), 36);
//! let bytes = random_bytes(4);
//! assert_eq!(bytes.len(), 4);
//! ```

use std::time::Instant;

fn next_u64(state: &mut u128) -> u64 {
    *state = state.wrapping_mul(1664525).wrapping_add(1013904223);
    (*state >> 32) as u64
}

/// Random integer in `[min, max)`.
///
/// Panics if `max <= min` due to modulo by zero.
///
/// Example:
/// ```rust
/// use toolchest::random::random_range;
/// let n = random_range(0, 3);
/// assert!(n >= 0 && n < 3);
/// ```
pub fn random_range(min: i64, max: i64) -> i64 {
    let mut s = Instant::now().elapsed().as_nanos();
    min + (next_u64(&mut s) as i64).rem_euclid(max - min)
}
/// Bernoulli trial with probability `p_true`.
///
/// `p_true` should be in `[0.0, 1.0]`.
///
/// Example:
/// ```rust
/// use toolchest::random::random_bool;
/// let _ = random_bool(0.25);
/// ```
pub fn random_bool(p_true: f64) -> bool {
    let mut s = Instant::now().elapsed().as_nanos();
    ((next_u64(&mut s) as f64) / (u64::MAX as f64)) < p_true
}
/// Choose a random element from slice.
///
/// Returns `None` if the slice is empty.
///
/// Example:
/// ```rust
/// use toolchest::random::random_choice;
/// let v = vec![1,2,3];
/// let _ = random_choice(&v);
/// ```
pub fn random_choice<T>(v: &[T]) -> Option<&T> {
    if v.is_empty() {
        None
    } else {
        let mut s = Instant::now().elapsed().as_nanos();
        v.get((next_u64(&mut s) as usize) % v.len())
    }
}
/// Sample `n` elements with replacement.
///
/// Example:
/// ```rust
/// use toolchest::random::random_choices;
/// let v = vec![1,2,3];
/// let xs = random_choices(&v, 5);
/// assert_eq!(xs.len(), 5);
/// ```
pub fn random_choices<T: Clone>(v: &[T], n: usize) -> Vec<T> {
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(x) = random_choice(v) {
            out.push(x.clone());
        }
    }
    out
}
/// Weighted random choice.
///
/// Returns an item with probability proportional to its weight. Returns `None`
/// on length mismatch or empty input.
///
/// Example:
/// ```rust
/// use toolchest::random::weighted_choice;
/// let v = ["a", "b", "c"]; let w = [0.1, 0.3, 0.6];
/// let _ = weighted_choice(&v, &w);
/// ```
pub fn weighted_choice<'a, T>(v: &'a [T], weights: &[f64]) -> Option<&'a T> {
    if v.is_empty() || v.len() != weights.len() {
        return None;
    }
    let total: f64 = weights.iter().sum();
    let mut s = Instant::now().elapsed().as_nanos();
    let mut r = ((next_u64(&mut s) as f64) / (u64::MAX as f64)) * total;
    for (item, &w) in v.iter().zip(weights.iter()) {
        if r < w {
            return Some(item);
        }
        r -= w;
    }
    v.last()
}
/// Generate a random UUID v4 (non-crypto).
///
/// Example:
/// ```rust
/// use toolchest::random::uuid_v4;
/// let id = uuid_v4();
/// assert_eq!(id.len(), 36);
/// ```
pub fn uuid_v4() -> String {
    let mut s = Instant::now().elapsed().as_nanos();
    let mut bytes = [0u8; 16];
    for b in &mut bytes {
        *b = (next_u64(&mut s) & 0xFF) as u8;
    }
    bytes[6] = (bytes[6] & 0x0F) | 0x40; // version 4
    bytes[8] = (bytes[8] & 0x3F) | 0x80; // variant
    format!("{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0],bytes[1],bytes[2],bytes[3],bytes[4],bytes[5],bytes[6],bytes[7],bytes[8],bytes[9],bytes[10],bytes[11],bytes[12],bytes[13],bytes[14],bytes[15])
}
/// Generate `n` random bytes (non-crypto).
///
/// Example:
/// ```rust
/// use toolchest::random::random_bytes;
/// let b = random_bytes(4);
/// assert_eq!(b.len(), 4);
/// ```
pub fn random_bytes(n: usize) -> Vec<u8> {
    let mut s = Instant::now().elapsed().as_nanos();
    (0..n).map(|_| (next_u64(&mut s) & 0xFF) as u8).collect()
}

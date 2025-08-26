//! Random utilities (non-cryptographic)

use std::time::Instant;

fn next_u64(state: &mut u128) -> u64 {
    *state = state.wrapping_mul(1664525).wrapping_add(1013904223);
    (*state >> 32) as u64
}

/// Random integer in [min, max)
pub fn random_range(min: i64, max: i64) -> i64 {
    let mut s = Instant::now().elapsed().as_nanos() as u128;
    min + (next_u64(&mut s) as i64).rem_euclid(max - min)
}
/// Bernoulli trial with probability p_true
pub fn random_bool(p_true: f64) -> bool {
    let mut s = Instant::now().elapsed().as_nanos() as u128;
    ((next_u64(&mut s) as f64) / (u64::MAX as f64)) < p_true
}
/// Choose a random element from slice
pub fn random_choice<T>(v: &[T]) -> Option<&T> {
    if v.is_empty() {
        None
    } else {
        let mut s = Instant::now().elapsed().as_nanos() as u128;
        v.get((next_u64(&mut s) as usize) % v.len())
    }
}
/// Sample n elements with replacement
pub fn random_choices<T: Clone>(v: &[T], n: usize) -> Vec<T> {
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(x) = random_choice(v) {
            out.push(x.clone());
        }
    }
    out
}
/// Weighted random choice
pub fn weighted_choice<'a, T>(v: &'a [T], weights: &[f64]) -> Option<&'a T> {
    if v.is_empty() || v.len() != weights.len() {
        return None;
    }
    let total: f64 = weights.iter().sum();
    let mut s = Instant::now().elapsed().as_nanos() as u128;
    let mut r = ((next_u64(&mut s) as f64) / (u64::MAX as f64)) * total;
    for (item, &w) in v.iter().zip(weights.iter()) {
        if r < w {
            return Some(item);
        }
        r -= w;
    }
    v.last()
}
/// Generate a random UUID v4 (non-crypto)
pub fn uuid_v4() -> String {
    let mut s = Instant::now().elapsed().as_nanos() as u128;
    let mut bytes = [0u8; 16];
    for b in &mut bytes {
        *b = (next_u64(&mut s) & 0xFF) as u8;
    }
    bytes[6] = (bytes[6] & 0x0F) | 0x40; // version 4
    bytes[8] = (bytes[8] & 0x3F) | 0x80; // variant
    format!("{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0],bytes[1],bytes[2],bytes[3],bytes[4],bytes[5],bytes[6],bytes[7],bytes[8],bytes[9],bytes[10],bytes[11],bytes[12],bytes[13],bytes[14],bytes[15])
}
/// Generate n random bytes (non-crypto)
pub fn random_bytes(n: usize) -> Vec<u8> {
    let mut s = Instant::now().elapsed().as_nanos() as u128;
    (0..n).map(|_| (next_u64(&mut s) & 0xFF) as u8).collect()
}

//! Non-crypto hash helpers.
//!
//! Convenient hashing utilities for quick IDs, bucket selection, and more.
//! Includes simple algorithms like djb2 and FNV-1a, alongside MurmurHash3
//! (x86 32-bit variant) and a `consistent_hash` helper for bucketing.
//!
//! Examples:
//! ```rust
//! use toolchest::hash::{hash_code, djb2, fnv1a, murmur3_32, consistent_hash};
//!
//! assert_eq!(hash_code("abc"), djb2(b"abc"));
//! let _ = fnv1a(b"hello");
//! let m = murmur3_32(b"key", 123);
//! let bucket = consistent_hash("user42", 10);
//! assert!(bucket < 10);
//! ```

/// Convenience hash for strings using djb2.
///
/// Example:
/// ```rust
/// use toolchest::hash::{hash_code, djb2};
/// assert_eq!(hash_code("abc"), djb2(b"abc"));
/// ```
pub fn hash_code(s: &str) -> u64 {
    djb2(s.as_bytes())
}

/// djb2 hash (64-bit variant).
///
/// Example:
/// ```rust
/// use toolchest::hash::djb2;
/// assert_eq!(djb2(b"a"), 177670);
/// ```
pub fn djb2(bytes: &[u8]) -> u64 {
    let mut h: u64 = 5381;
    for &b in bytes {
        h = ((h << 5) + h) + (b as u64);
    }
    h
}
/// FNV-1a 64-bit hash.
///
/// Example:
/// ```rust
/// use toolchest::hash::fnv1a;
/// let _h = fnv1a(b"hello");
/// ```
pub fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// MurmurHash3 x86 32-bit.
///
/// Example:
/// ```rust
/// use toolchest::hash::murmur3_32;
/// let h = murmur3_32(b"key", 0);
/// let _ = h;
/// ```
pub fn murmur3_32(bytes: &[u8], seed: u32) -> u32 {
    let mut h = seed;
    let c1 = 0xcc9e2d51u32;
    let c2 = 0x1b873593u32;
    let mut chunks = bytes.chunks_exact(4);
    for chunk in &mut chunks {
        let mut k = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        k = k.wrapping_mul(c1);
        k = k.rotate_left(15);
        k = k.wrapping_mul(c2);
        h ^= k;
        h = h.rotate_left(13);
        h = h.wrapping_mul(5).wrapping_add(0xe6546b64);
    }
    let rem = chunks.remainder();
    let mut k = 0u32;
    match rem.len() {
        3 => {
            k ^= (rem[2] as u32) << 16;
            k ^= (rem[1] as u32) << 8;
            k ^= rem[0] as u32;
        }
        2 => {
            k ^= (rem[1] as u32) << 8;
            k ^= rem[0] as u32;
        }
        1 => {
            k ^= rem[0] as u32;
        }
        _ => {}
    }
    if !rem.is_empty() {
        k = k.wrapping_mul(c1);
        k = k.rotate_left(15);
        k = k.wrapping_mul(c2);
        h ^= k;
    }
    h ^= bytes.len() as u32;
    h ^= h >> 16;
    h = h.wrapping_mul(0x85ebca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2ae35);
    h ^= h >> 16;
    h
}

/// Consistent hashing to bucket index `[0, buckets)`.
///
/// Returns `0` when `buckets` is `0`.
///
/// Example:
/// ```rust
/// use toolchest::hash::consistent_hash;
/// let b = consistent_hash("user42", 10);
/// assert!(b < 10);
/// ```
pub fn consistent_hash(key: &str, buckets: u32) -> u32 {
    if buckets == 0 {
        0
    } else {
        murmur3_32(key.as_bytes(), 0x9747b28c) % buckets
    }
}

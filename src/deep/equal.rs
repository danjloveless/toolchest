//! Deep equality utilities

use std::collections::HashMap;

/// Deep equality for HashMap values using PartialEq on values
pub fn deep_equal<K, V>(a: &HashMap<K, V>, b: &HashMap<K, V>) -> bool
where
    K: Eq + std::hash::Hash,
    V: PartialEq,
{
    if a.len() != b.len() {
        return false;
    }
    a.iter().all(|(k, v)| b.get(k).map_or(false, |vb| vb == v))
}

/// Deep equality for slices using PartialEq
pub fn deep_equal_slice<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    a == b
}



//! Deep merging utilities

use std::collections::HashMap;

/// Trait for types that can be deeply merged
pub trait DeepMerge {
    /// Merge `other` into `self`
    fn deep_merge(&mut self, other: &Self);
}

// Overwrite semantics for common leaf values
impl DeepMerge for i32 {
    fn deep_merge(&mut self, other: &Self) {
        *self = *other;
    }
}

impl<K, V> DeepMerge for HashMap<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone + DeepMerge,
{
    fn deep_merge(&mut self, other: &Self) {
        for (key, value) in other.iter() {
            match self.get_mut(key) {
                Some(existing) => existing.deep_merge(value),
                None => {
                    self.insert(key.clone(), value.clone());
                }
            }
        }
    }
}

/// Merge two values, with right overwriting left
pub fn merge<T: DeepMerge + Clone>(left: &T, right: &T) -> T {
    let mut result = left.clone();
    result.deep_merge(right);
    result
}

/// Merge multiple values in sequence
pub fn merge_all<T: DeepMerge + Clone>(values: &[&T]) -> Option<T> {
    if values.is_empty() {
        return None;
    }

    let mut result = values[0].clone();
    for value in &values[1..] {
        result.deep_merge(value);
    }
    Some(result)
}

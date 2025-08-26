//! Deep cloning utilities

/// Trait for types that support deep cloning
pub trait DeepClone {
    /// Return a deep clone of `self`
    fn deep_clone(&self) -> Self;
}

// Blanket implementation for types that implement Clone
impl<T: Clone> DeepClone for T {
    fn deep_clone(&self) -> Self {
        self.clone()
    }
}

/// Convenience function for deep cloning
pub fn clone<T: DeepClone>(value: &T) -> T {
    value.deep_clone()
}



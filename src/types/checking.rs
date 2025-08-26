//! Type checking utilities

/// Trait for checking if a value is empty
pub trait IsEmpty {
    /// Returns true if the value is considered empty
    fn is_empty(&self) -> bool;
}

impl IsEmpty for str {
    fn is_empty(&self) -> bool {
        <str>::is_empty(self)
    }
}

impl IsEmpty for String {
    fn is_empty(&self) -> bool {
        String::is_empty(self)
    }
}

impl<T> IsEmpty for Vec<T> {
    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }
}

impl<T> IsEmpty for &[T] {
    fn is_empty(&self) -> bool {
        <[T]>::is_empty(self)
    }
}

impl<K, V> IsEmpty for std::collections::HashMap<K, V> {
    fn is_empty(&self) -> bool {
        std::collections::HashMap::is_empty(self)
    }
}

/// Check if a value is empty
pub fn is_empty<T: IsEmpty>(value: &T) -> bool {
    value.is_empty()
}

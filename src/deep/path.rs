//! Path-based access to nested structures

/// Trait for types that support path-based access
pub trait PathAccess {
    /// Associated value type accessed by path
    type Value;
    
    /// Get a reference to the value at `path`
    fn get_path<'a>(&'a self, path: &str) -> Option<&'a Self::Value>;
    /// Set the value at `path`, returning true if set
    fn set_path(&mut self, path: &str, value: Self::Value) -> bool;
    /// True if a value exists at `path`
    fn has_path(&self, path: &str) -> bool {
        self.get_path(path).is_some()
    }
}

/// Generic get function
pub fn get<'a, T: PathAccess>(container: &'a T, path: &str) -> Option<&'a T::Value> {
    container.get_path(path)
}

/// Generic set function
pub fn set<T: PathAccess>(container: &mut T, path: &str, value: T::Value) -> bool {
    container.set_path(path, value)
}

/// Generic has function
pub fn has<T: PathAccess>(container: &T, path: &str) -> bool {
    container.has_path(path)
}



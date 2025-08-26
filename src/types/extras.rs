//! Additional type helpers

/// Map Some(v) via f or return provided default
pub fn map_some_or<T, U, F>(opt: Option<T>, default: U, f: F) -> U
where
    F: FnOnce(T) -> U,
{
    match opt {
        Some(v) => f(v),
        None => default,
    }
}

/// Map Ok(v) via f or return provided default for Err
pub fn map_ok_or<T, E, U, F>(res: Result<T, E>, default: U, f: F) -> U
where
    F: FnOnce(T) -> U,
{
    match res {
        Ok(v) => f(v),
        Err(_) => default,
    }
}

/// A non-empty vector wrapper
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NonEmptyVec<T> {
    head: T,
    tail: Vec<T>,
}

impl<T> NonEmptyVec<T> {
    /// Construct from head and tail
    pub fn new(head: T, tail: Vec<T>) -> Self {
        Self { head, tail }
    }
    /// Convert a Vec into NonEmptyVec if not empty
    pub fn from_vec(mut v: Vec<T>) -> Option<Self> {
        if v.is_empty() {
            None
        } else {
            let head = v.remove(0);
            Some(Self { head, tail: v })
        }
    }
    /// Length of the collection
    pub fn len(&self) -> usize {
        1 + self.tail.len()
    }
    /// True if collection is empty (always false)
    pub fn is_empty(&self) -> bool {
        false
    }
    /// Iterator over elements
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        std::iter::once(&self.head).chain(self.tail.iter())
    }
}

/// Placeholder cast function (not implemented without Any)
pub fn try_cast<T: 'static, U: 'static + Clone>(_t: &T) -> Result<U, &'static str> {
    Err("unsupported without Any")
}

/// Clamp x to [min, max]
pub fn coerce(x: i64, min: i64, max: i64) -> i64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

/// Option helper: predicate on Some
pub fn is_some_and<T, F: FnOnce(&T) -> bool>(opt: Option<T>, f: F) -> bool {
    opt.as_ref().is_some_and(f)
}
/// Result helper: predicate on Ok
pub fn is_ok_and<T, E, F: FnOnce(&T) -> bool>(res: Result<T, E>, f: F) -> bool {
    res.as_ref().is_ok_and(|v| f(v))
}
/// Return reference or default
pub fn as_ref_or<'a, T>(opt: Option<&'a T>, default: &'a T) -> &'a T {
    opt.unwrap_or(default)
}
/// Take Option value if predicate holds
pub fn take_if<T, F: FnOnce(&T) -> bool>(opt: &mut Option<T>, f: F) -> Option<T> {
    if opt.as_ref().is_some_and(|v| f(v)) {
        opt.take()
    } else {
        None
    }
}
/// Replace value and return old
pub fn replace_with<T>(slot: &mut T, new_val: T) -> T {
    std::mem::replace(slot, new_val)
}
/// Get the short type name without module path
pub fn type_name_short<T>() -> &'static str {
    std::any::type_name::<T>().rsplit("::").next().unwrap_or("")
}

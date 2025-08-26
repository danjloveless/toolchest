//! Additional type helpers

/// Map Some(v) via f or return provided default
///
/// Example:
/// ```rust
/// use toolchest::types::map_some_or;
/// assert_eq!(map_some_or(Some(2), 0, |v| v*2), 4);
/// assert_eq!(map_some_or::<i32,_,_>(None, 7, |v| v*2), 7);
/// ```
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
///
/// Example:
/// ```rust
/// use toolchest::types::map_ok_or;
/// let r: Result<i32, &str> = Ok(2);
/// assert_eq!(map_ok_or(r, 0, |v| v*3), 6);
/// ```
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
    ///
    /// Example:
    /// ```rust
    /// use toolchest::types::NonEmptyVec;
    /// let nev = NonEmptyVec::new(1, vec![2,3]);
    /// assert_eq!(nev.len(), 3);
    /// ```
    pub fn new(head: T, tail: Vec<T>) -> Self {
        Self { head, tail }
    }
    /// Convert a Vec into NonEmptyVec if not empty
    ///
    /// Example:
    /// ```rust
    /// use toolchest::types::NonEmptyVec;
    /// let nev = NonEmptyVec::from_vec(vec![1,2,3]).unwrap();
    /// assert_eq!(nev.len(), 3);
    /// ```
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
    ///
    /// Example:
    /// ```rust
    /// use toolchest::types::NonEmptyVec;
    /// let nev = NonEmptyVec::new(1, vec![2,3]);
    /// assert_eq!(nev.iter().cloned().collect::<Vec<_>>(), vec![1,2,3]);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        std::iter::once(&self.head).chain(self.tail.iter())
    }
}

/// Attempt to downcast a reference `&T` to a cloned value of type `U`.
///
/// Returns `Ok(U)` when `T` is the same concrete type as `U` (or a type alias),
/// otherwise returns `Err`.
pub fn try_cast<T: 'static, U: 'static + Clone>(t: &T) -> Result<U, &'static str> {
    use core::any::Any;
    let any_ref = t as &dyn Any;
    if let Some(u_ref) = any_ref.downcast_ref::<U>() {
        Ok(u_ref.clone())
    } else {
        Err("downcast failed")
    }
}

/// Clamp x to [min, max]
///
/// Example:
/// ```rust
/// use toolchest::types::extras::coerce;
/// assert_eq!(coerce(5, 0, 3), 3);
/// assert_eq!(coerce(-1, 0, 3), 0);
/// assert_eq!(coerce(2, 0, 3), 2);
/// ```
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
///
/// Example:
/// ```rust
/// use toolchest::types::extras::is_some_and;
/// assert!(is_some_and(Some(2), |v| *v == 2));
/// ```
pub fn is_some_and<T, F: FnOnce(&T) -> bool>(opt: Option<T>, f: F) -> bool {
    opt.as_ref().is_some_and(f)
}
/// Result helper: predicate on Ok
///
/// Example:
/// ```rust
/// use toolchest::types::extras::is_ok_and;
/// let r: Result<i32, &str> = Ok(5);
/// assert!(is_ok_and(r, |v| *v == 5));
/// ```
pub fn is_ok_and<T, E, F: FnOnce(&T) -> bool>(res: Result<T, E>, f: F) -> bool {
    res.as_ref().is_ok_and(f)
}
/// Return reference or default
///
/// Example:
/// ```rust
/// use toolchest::types::extras::as_ref_or;
/// let x = 1;
/// assert_eq!(as_ref_or(Some(&x), &0), &1);
/// ```
pub fn as_ref_or<'a, T>(opt: Option<&'a T>, default: &'a T) -> &'a T {
    opt.unwrap_or(default)
}
/// Take Option value if predicate holds
///
/// Example:
/// ```rust
/// use toolchest::types::extras::take_if;
/// let mut opt = Some(2);
/// let v = take_if(&mut opt, |v| *v == 2);
/// assert_eq!(v, Some(2));
/// assert!(opt.is_none());
/// ```
pub fn take_if<T, F: FnOnce(&T) -> bool>(opt: &mut Option<T>, f: F) -> Option<T> {
    if opt.as_ref().is_some_and(f) {
        opt.take()
    } else {
        None
    }
}
/// Replace value and return old
///
/// Example:
/// ```rust
/// use toolchest::types::extras::replace_with;
/// let mut x = 1;
/// let old = replace_with(&mut x, 2);
/// assert_eq!(old, 1);
/// assert_eq!(x, 2);
/// ```
pub fn replace_with<T>(slot: &mut T, new_val: T) -> T {
    std::mem::replace(slot, new_val)
}
/// Get the short type name without module path
///
/// Example:
/// ```rust
/// use toolchest::types::extras::type_name_short;
/// let name = type_name_short::<Option<i32>>();
/// assert!(name.contains("Option"));
/// ```
pub fn type_name_short<T>() -> &'static str {
    std::any::type_name::<T>().rsplit("::").next().unwrap_or("")
}

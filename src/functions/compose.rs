//! Function composition helpers.
//!
//! Utilities for composing and transforming functions and values:
//! - [`compose`] to build `g ∘ f`.
//! - [`pipe`] to pass a value through two functions.
//! - [`tap`] to perform a side-effect without changing the value.
//! - [`identity`] returns its input unchanged.
//! - [`constant`] returns a closure that always yields the same value.
//! - [`noop`] does nothing.
//! - [`negate`] flips a predicate's boolean result.
//! - [`flip`] swaps the first two arguments of a binary function.
//! - [`partial`] captures one argument for later invocation.
//! - [`times`] calls a closure for indices `0..n`.
//! - [`until`] repeatedly applies a step until a predicate holds.
//!
//! Basic examples:
//! ```rust
//! use toolchest::functions::{compose, pipe, tap};
//! use toolchest::functions::compose::{identity, negate, flip, partial, times, until};
//!
//! let double = |x: i32| x * 2;
//! let add1 = |x: i32| x + 1;
//! let h = compose(double, add1); // h(x) = double(add1(x))
//! assert_eq!(h(3), 8);
//!
//! let val = pipe(3, add1, double); // double(add1(3))
//! assert_eq!(val, 8);
//!
//! use std::sync::atomic::{AtomicUsize, Ordering};
//! let seen = AtomicUsize::new(0);
//! let value = tap(10, |_| { seen.fetch_add(1, Ordering::SeqCst); });
//! assert_eq!(value, 10);
//! assert_eq!(seen.load(Ordering::SeqCst), 1);
//!
//! assert_eq!(identity(7), 7);
//! assert_eq!(negate(|x: i32| x > 0)(-1), true);
//!
//! let sub = |a: i32, b: i32| a - b;
//! assert_eq!(flip(sub)(2, 5), 3); // computes sub(5, 2)
//!
//! let add5 = partial(|x: i32| x + 5, 5);
//! assert_eq!(add5(), 10);
//!
//! let mut acc = 0;
//! times(3, |i| acc += i as i32);
//! assert_eq!(acc, 0 + 1 + 2);
//!
//! let res = until(0, |&x| x >= 5, |x| x + 2);
//! assert_eq!(res, 6);
//! ```

/// Compose two functions `g ∘ f`.
///
/// Returns a new function that applies `f` then `g`.
pub fn compose<A, B, C, F, G>(g: G, f: F) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |a| g(f(a))
}

/// Pipe a value through `f` then `g`.
pub fn pipe<A, B, C, F, G>(a: A, f: F, g: G) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    g(f(a))
}

/// Run a side-effect on `value` and return it unchanged.
pub fn tap<T, F>(value: T, f: F) -> T
where
    F: Fn(&T),
{
    f(&value);
    value
}

/// Identity function.
pub fn identity<T>(x: T) -> T {
    x
}
/// Return a closure that always returns a clone of `x`.
pub fn constant<T: Clone>(x: T) -> impl Fn() -> T {
    move || x.clone()
}
/// Do nothing.
pub fn noop() {}

/// Logical negation of a predicate.
pub fn negate<P, F>(pred: F) -> impl Fn(P) -> bool
where
    F: Fn(P) -> bool,
{
    move |p| !pred(p)
}

/// Flip the first two arguments of a function.
pub fn flip<A, B, R, F>(f: F) -> impl Fn(B, A) -> R
where
    F: Fn(A, B) -> R,
{
    move |b, a| f(a, b)
}

/// Partially apply a single argument.
pub fn partial<A: Clone, R, F>(f: F, a: A) -> impl Fn() -> R
where
    F: Fn(A) -> R + Clone,
{
    move || f(a.clone())
}

/// Call `f` for indices `0..n`.
pub fn times<F: FnMut(usize)>(n: usize, mut f: F) {
    for i in 0..n {
        f(i);
    }
}

/// Repeatedly apply `step` until `pred` is true.
pub fn until<T, P, F>(mut value: T, pred: P, mut step: F) -> T
where
    P: Fn(&T) -> bool,
    F: FnMut(T) -> T,
{
    while !pred(&value) {
        value = step(value);
    }
    value
}

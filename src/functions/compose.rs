//! Function composition helpers

/// Compose two functions g∘f
pub fn compose<A, B, C, F, G>(g: G, f: F) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |a| g(f(a))
}

/// Pipe a value through f then g
pub fn pipe<A, B, C, F, G>(a: A, f: F, g: G) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    g(f(a))
}

/// Run a side-effect on value and return it unchanged
pub fn tap<T, F>(value: T, f: F) -> T
where
    F: Fn(&T),
{
    f(&value);
    value
}

/// Identity function
pub fn identity<T>(x: T) -> T { x }
/// Return a closure that always returns a clone of x
pub fn constant<T: Clone>(x: T) -> impl Fn() -> T { move || x.clone() }
/// Do nothing
pub fn noop() {}

/// Logical negation of a predicate
pub fn negate<P, F>(pred: F) -> impl Fn(P) -> bool
where F: Fn(P) -> bool { move |p| !pred(p) }

/// Flip the first two arguments of a function
pub fn flip<A, B, R, F>(f: F) -> impl Fn(B, A) -> R
where F: Fn(A, B) -> R { move |b, a| f(a, b) }

/// Partially apply a single argument
pub fn partial<A: Clone, R, F>(f: F, a: A) -> impl Fn() -> R
where F: Fn(A) -> R + Clone { move || f(a.clone()) }

/// Call f for indices 0..n
pub fn times<F: FnMut(usize)>(n: usize, mut f: F) { for i in 0..n { f(i); } }

/// Repeatedly apply step until pred is true
pub fn until<T, P, F>(mut value: T, pred: P, mut step: F) -> T
where P: Fn(&T) -> bool, F: FnMut(T) -> T { while !pred(&value) { value = step(value); } value }




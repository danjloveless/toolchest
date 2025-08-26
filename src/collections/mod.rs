//! Collection utilities similar to lodash (non-iterator focused).
//!
//! This module provides convenient helpers for working with slices and vectors
//! without requiring iterator chains. Functions include chunking, set-like
//! operations, grouping, windowing, and more.
//!
//! Highlights:
//! - Chunking: [`chunk`]
//! - De-duplication: [`uniq`]
//! - Set ops: [`difference`], [`intersection`], [`union`]
//! - Grouping: [`group_by`], [`key_by`], [`count_by`]
//! - Windows: [`sliding_window`]
//! - Sampling: [`sample`], [`shuffle_in_place`]
//!
//! Basic examples:
//! ```rust
//! use toolchest::collections::{chunk, uniq, difference, sliding_window};
//!
//! assert_eq!(chunk(&[1,2,3,4,5], 2), vec![vec![1,2], vec![3,4], vec![5]]);
//! assert_eq!(uniq(&[1,1,2,3,3]), vec![1,2,3]);
//! assert_eq!(difference(&[1,2,3], &[2,4]), vec![1,3]);
//! assert_eq!(sliding_window(&[1,2,3,4], 2, 1), vec![vec![1,2], vec![2,3], vec![3,4]]);
//! ```

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Split a slice into chunks of size `size`.
///
/// Returns an empty vector when `size == 0`.
///
/// Example:
/// ```rust
/// use toolchest::collections::chunk;
/// assert_eq!(chunk(&[1,2,3,4,5], 2), vec![vec![1,2], vec![3,4], vec![5]]);
/// ```
pub fn chunk<T: Clone>(slice: &[T], size: usize) -> Vec<Vec<T>> {
    if size == 0 {
        return vec![];
    }
    slice.chunks(size).map(|c| c.to_vec()).collect()
}

/// Remove `None` values from a slice of `Option<T>`.
///
/// Example:
/// ```rust
/// use toolchest::collections::compact;
/// assert_eq!(compact(&[Some(1), None, Some(2)]), vec![1,2]);
/// ```
pub fn compact<T: Clone>(slice: &[Option<T>]) -> Vec<T> {
    slice.iter().filter_map(|o| o.clone()).collect()
}

/// Remove duplicate elements while preserving order.
///
/// Example:
/// ```rust
/// use toolchest::collections::uniq;
/// assert_eq!(uniq(&[1,1,2,3,3]), vec![1,2,3]);
/// ```
pub fn uniq<T: Eq + Hash + Clone>(slice: &[T]) -> Vec<T> {
    let mut set = HashSet::new();
    let mut out = Vec::with_capacity(slice.len());
    for v in slice {
        if set.insert(v) {
            out.push(v.clone());
        }
    }
    out
}

/// Elements in `a` that are not in `b`.
///
/// Example:
/// ```rust
/// use toolchest::collections::difference;
/// assert_eq!(difference(&[1,2,3], &[2,4]), vec![1,3]);
/// ```
pub fn difference<T: Eq + Hash + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let set_b: HashSet<&T> = b.iter().collect();
    a.iter().filter(|x| !set_b.contains(x)).cloned().collect()
}

/// Elements common to `a` and `b`.
///
/// Example:
/// ```rust
/// use toolchest::collections::intersection;
/// assert_eq!(intersection(&[1,2,3], &[2,3,4]), vec![2,3]);
/// ```
pub fn intersection<T: Eq + Hash + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let set_b: HashSet<&T> = b.iter().collect();
    a.iter().filter(|x| set_b.contains(x)).cloned().collect()
}

/// Union of two slices (unique elements, order preserved).
///
/// Example:
/// ```rust
/// use toolchest::collections::union;
/// assert_eq!(union(&[1,2], &[2,3]), vec![1,2,3]);
/// ```
pub fn union<T: Eq + Hash + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    out.extend_from_slice(a);
    out.extend_from_slice(b);
    uniq(&out)
}

/// Flatten one level of nested vectors.
///
/// Example:
/// ```rust
/// use toolchest::collections::flatten;
/// assert_eq!(flatten(&[vec![1,2], vec![3]]), vec![1,2,3]);
/// ```
pub fn flatten<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
    let total: usize = nested.iter().map(|v| v.len()).sum();
    let mut out = Vec::with_capacity(total);
    for v in nested {
        out.extend_from_slice(v);
    }
    out
}

/// Group elements by a key function.
///
/// Example:
/// ```rust
/// use toolchest::collections::group_by;
/// let map = group_by(&["a", "bb", "c"], |s: &&str| s.len());
/// assert_eq!(map.get(&1).unwrap().len(), 2);
/// ```
pub fn group_by<T, K, F>(slice: &[T], f: F) -> HashMap<K, Vec<&T>>
where
    K: Eq + Hash,
    F: Fn(&T) -> K,
{
    let mut map: HashMap<K, Vec<&T>> = HashMap::new();
    for item in slice {
        map.entry(f(item)).or_default().push(item);
    }
    map
}

/// Map elements by a key function.
///
/// Example:
/// ```rust
/// use toolchest::collections::key_by;
/// let map = key_by(&["x", "yy"], |s: &&str| s.len());
/// assert_eq!(map.get(&2).unwrap(), &&"yy");
/// ```
pub fn key_by<T, K, F>(slice: &[T], f: F) -> HashMap<K, &T>
where
    K: Eq + Hash,
    F: Fn(&T) -> K,
{
    let mut map: HashMap<K, &T> = HashMap::new();
    for item in slice {
        map.insert(f(item), item);
    }
    map
}

/// Count elements by a key function.
///
/// Example:
/// ```rust
/// use toolchest::collections::count_by;
/// let map = count_by(&["a", "bb", "c"], |s: &&str| s.len());
/// assert_eq!(map.get(&1), Some(&2));
/// ```
pub fn count_by<T, K, F>(slice: &[T], f: F) -> HashMap<K, usize>
where
    K: Eq + Hash,
    F: Fn(&T) -> K,
{
    let mut map: HashMap<K, usize> = HashMap::new();
    for item in slice {
        *map.entry(f(item)).or_insert(0) += 1;
    }
    map
}

/// Partition elements into `(true, false)` by predicate.
///
/// Example:
/// ```rust
/// use toolchest::collections::partition;
/// let (t, f) = partition(&[1,2,3], |x| *x % 2 == 0);
/// assert_eq!(t.len(), 1);
/// assert_eq!(f.len(), 2);
/// ```
pub fn partition<T, F>(slice: &[T], f: F) -> (Vec<&T>, Vec<&T>)
where
    F: Fn(&T) -> bool,
{
    let mut t = Vec::new();
    let mut fvec = Vec::new();
    for item in slice {
        if f(item) {
            t.push(item);
        } else {
            fvec.push(item);
        }
    }
    (t, fvec)
}

// Simple PRNG (LCG) for shuffle/sample
struct Lcg {
    state: u128,
}
impl Lcg {
    fn new(seed: u128) -> Self {
        Self { state: seed }
    }
    fn next_u64(&mut self) -> u64 {
        // Constants from Numerical Recipes
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        (self.state >> 32) as u64
    }
}

/// Shuffle elements in place.
///
/// Example:
/// ```rust
/// use toolchest::collections::shuffle_in_place;
/// let mut v = vec![1,2,3];
/// shuffle_in_place(&mut v);
/// assert_eq!(v.len(), 3);
/// ```
pub fn shuffle_in_place<T>(slice: &mut [T]) {
    let seed = std::time::Instant::now().elapsed().as_nanos();
    let mut rng = Lcg::new(seed);
    let mut i = slice.len();
    while i > 1 {
        i -= 1;
        let j = (rng.next_u64() as usize) % (i + 1);
        slice.swap(i, j);
    }
}

/// Sample a random element.
///
/// Returns `None` if the slice is empty.
///
/// Example:
/// ```rust
/// use toolchest::collections::sample;
/// let v = vec![1,2,3];
/// let _ = sample(&v);
/// ```
pub fn sample<T>(slice: &[T]) -> Option<&T> {
    if slice.is_empty() {
        return None;
    }
    let seed = std::time::Instant::now().elapsed().as_nanos();
    let mut rng = Lcg::new(seed);
    let idx = (rng.next_u64() as usize) % slice.len();
    slice.get(idx)
}

/// Zip keys and values into a `HashMap`.
///
/// Example:
/// ```rust
/// use toolchest::collections::zip_object;
/// let map = zip_object(&["a", "b"], &[1,2]);
/// assert_eq!(map.get(&"b"), Some(&2));
/// ```
pub fn zip_object<K: Eq + Hash + Clone, V: Clone>(keys: &[K], values: &[V]) -> HashMap<K, V> {
    let mut map = HashMap::new();
    for (k, v) in keys.iter().cloned().zip(values.iter().cloned()) {
        map.insert(k, v);
    }
    map
}

/// Split a slice of pairs into two vectors.
///
/// Example:
/// ```rust
/// use toolchest::collections::unzip;
/// let (ks, vs) = unzip(&[("a", 1), ("b", 2)]);
/// assert_eq!(ks, vec!["a", "b"]);
/// assert_eq!(vs, vec![1, 2]);
/// ```
pub fn unzip<K: Clone, V: Clone>(pairs: &[(K, V)]) -> (Vec<K>, Vec<V>) {
    let mut ks = Vec::with_capacity(pairs.len());
    let mut vs = Vec::with_capacity(pairs.len());
    for (k, v) in pairs {
        ks.push(k.clone());
        vs.push(v.clone());
    }
    (ks, vs)
}

/// First index of item.
///
/// Example:
/// ```rust
/// use toolchest::collections::index_of;
/// assert_eq!(index_of(&[1,2,3], &2), Some(1));
/// ```
pub fn index_of<T: PartialEq>(slice: &[T], item: &T) -> Option<usize> {
    slice.iter().position(|x| x == item)
}

/// Last index of item.
///
/// Example:
/// ```rust
/// use toolchest::collections::last_index_of;
/// assert_eq!(last_index_of(&[1,2,1], &1), Some(2));
/// ```
pub fn last_index_of<T: PartialEq>(slice: &[T], item: &T) -> Option<usize> {
    slice.iter().rposition(|x| x == item)
}

/// Take first `n` elements.
///
/// Example:
/// ```rust
/// use toolchest::collections::take;
/// assert_eq!(take(&[1,2,3], 2), vec![1,2]);
/// ```
pub fn take<T: Clone>(slice: &[T], n: usize) -> Vec<T> {
    slice.iter().take(n).cloned().collect()
}
/// Drop first `n` elements.
///
/// Example:
/// ```rust
/// use toolchest::collections::drop;
/// assert_eq!(drop(&[1,2,3], 1), vec![2,3]);
/// ```
pub fn drop<T: Clone>(slice: &[T], n: usize) -> Vec<T> {
    slice.iter().skip(n).cloned().collect()
}
/// Take last `n` elements.
///
/// Example:
/// ```rust
/// use toolchest::collections::take_right;
/// assert_eq!(take_right(&[1,2,3], 2), vec![2,3]);
/// ```
pub fn take_right<T: Clone>(slice: &[T], n: usize) -> Vec<T> {
    slice
        .iter()
        .rev()
        .take(n)
        .cloned()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}
/// Drop last `n` elements.
///
/// Example:
/// ```rust
/// use toolchest::collections::drop_right;
/// assert_eq!(drop_right(&[1,2,3], 1), vec![1,2]);
/// ```
pub fn drop_right<T: Clone>(slice: &[T], n: usize) -> Vec<T> {
    let len = slice.len().saturating_sub(n);
    slice[..len].to_vec()
}

/// Take while predicate holds.
///
/// Example:
/// ```rust
/// use toolchest::collections::take_while;
/// assert_eq!(take_while(&[1,2,0,3], |x| *x > 0), vec![1,2]);
/// ```
pub fn take_while<T: Clone, F: Fn(&T) -> bool>(slice: &[T], f: F) -> Vec<T> {
    slice.iter().take_while(|x| f(x)).cloned().collect()
}
/// Drop while predicate holds, then keep rest.
///
/// Example:
/// ```rust
/// use toolchest::collections::drop_while;
/// assert_eq!(drop_while(&[0,0,1,2], |x| *x == 0), vec![1,2]);
/// ```
pub fn drop_while<T: Clone, F: Fn(&T) -> bool>(slice: &[T], f: F) -> Vec<T> {
    let mut started = false;
    slice
        .iter()
        .filter(|x| {
            started |= !f(x);
            started
        })
        .cloned()
        .collect()
}

/// Rotate left by `n`.
///
/// Example:
/// ```rust
/// use toolchest::collections::rotate_left;
/// let mut v = vec![1,2,3,4];
/// rotate_left(&mut v, 1);
/// assert_eq!(v, vec![2,3,4,1]);
/// ```
pub fn rotate_left<T: Clone>(v: &mut [T], n: usize) {
    v.rotate_left(n % v.len().max(1))
}
/// Rotate right by `n`.
///
/// Example:
/// ```rust
/// use toolchest::collections::rotate_right;
/// let mut v = vec![1,2,3,4];
/// rotate_right(&mut v, 1);
/// assert_eq!(v, vec![4,1,2,3]);
/// ```
pub fn rotate_right<T: Clone>(v: &mut [T], n: usize) {
    v.rotate_right(n % v.len().max(1))
}

/// Insert separator between elements.
///
/// Example:
/// ```rust
/// use toolchest::collections::intersperse;
/// assert_eq!(intersperse(&[1,2,3], 0), vec![1,0,2,0,3]);
/// ```
pub fn intersperse<T: Clone>(slice: &[T], sep: T) -> Vec<T> {
    if slice.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(slice.len() * 2 - 1);
    for (i, item) in slice.iter().cloned().enumerate() {
        if i > 0 {
            out.push(sep.clone());
        }
        out.push(item);
    }
    out
}

/// Sliding windows of given size/step.
///
/// Example:
/// ```rust
/// use toolchest::collections::sliding_window;
/// assert_eq!(sliding_window(&[1,2,3,4], 2, 2), vec![vec![1,2], vec![3,4]]);
/// ```
pub fn sliding_window<T: Clone>(slice: &[T], size: usize, step: usize) -> Vec<Vec<T>> {
    if size == 0 || step == 0 {
        return vec![];
    }
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + size <= slice.len() {
        out.push(slice[i..i + size].to_vec());
        i += step;
    }
    out
}

/// Cartesian product of two slices.
///
/// Example:
/// ```rust
/// use toolchest::collections::cartesian_product;
/// assert_eq!(cartesian_product(&[1,2], &['a','b']), vec![(1,'a'),(1,'b'),(2,'a'),(2,'b')]);
/// ```
pub fn cartesian_product<A: Clone, B: Clone>(a: &[A], b: &[B]) -> Vec<(A, B)> {
    let mut out = Vec::with_capacity(a.len() * b.len());
    for x in a {
        for y in b {
            out.push((x.clone(), y.clone()));
        }
    }
    out
}

/// Transpose a rectangular matrix.
///
/// Example:
/// ```rust
/// use toolchest::collections::transpose;
/// assert_eq!(transpose(&[vec![1,2,3], vec![4,5,6]]), vec![vec![1,4], vec![2,5], vec![3,6]]);
/// ```
pub fn transpose<T: Clone>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    if matrix.is_empty() {
        return vec![];
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut out = vec![vec![matrix[0][0].clone(); rows]; cols];
    for (r, row) in matrix.iter().enumerate().take(rows) {
        for (c, cell) in row.iter().enumerate().take(cols) {
            out[c][r] = cell.clone();
        }
    }
    out
}

/// Move item from index to index.
///
/// Returns `false` if either index is out of bounds.
///
/// Example:
/// ```rust
/// use toolchest::collections::move_item;
/// let mut v = vec![10, 20, 30];
/// assert!(move_item(&mut v, 0, 2));
/// assert_eq!(v, vec![20, 30, 10]);
/// ```
pub fn move_item<T: Clone>(v: &mut Vec<T>, from: usize, to: usize) -> bool {
    if from >= v.len() || to >= v.len() {
        return false;
    }
    let item = v.remove(from);
    v.insert(to, item);
    true
}

/// Swap two indices if valid.
///
/// Returns `false` if either index is out of bounds.
///
/// Example:
/// ```rust
/// use toolchest::collections::swap;
/// let mut v = vec![1,2,3];
/// assert!(swap(&mut v, 0, 2));
/// assert_eq!(v, vec![3,2,1]);
/// ```
pub fn swap<T>(v: &mut [T], i: usize, j: usize) -> bool {
    if i >= v.len() || j >= v.len() {
        false
    } else {
        v.swap(i, j);
        true
    }
}

/// Binary search using a comparator to `Ordering`.
///
/// Returns `Some(index)` if an element compares equal, otherwise `None`.
///
/// Example:
/// ```rust
/// use toolchest::collections::binary_search_by;
/// let v = [1,3,5,7];
/// assert_eq!(binary_search_by(&v, |x| x.cmp(&5)), Some(2));
/// ```
pub fn binary_search_by<T, F: Fn(&T) -> std::cmp::Ordering>(v: &[T], cmp: F) -> Option<usize> {
    let mut low = 0usize;
    let mut high = v.len();
    while low < high {
        let mid = (low + high) / 2;
        match cmp(&v[mid]) {
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }
    None
}

/// Find duplicate elements (unique list).
///
/// Example:
/// ```rust
/// use toolchest::collections::find_duplicates;
/// let d = find_duplicates(&[1,2,1,3,2]);
/// assert_eq!(d.len(), 2);
/// ```
pub fn find_duplicates<T: Eq + Hash + Clone>(slice: &[T]) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut dups = HashSet::new();
    for x in slice {
        if !seen.insert(x) {
            dups.insert(x.clone());
        }
    }
    dups.into_iter().collect()
}

/// True if slice is non-decreasing.
///
/// Example:
/// ```rust
/// use toolchest::collections::is_sorted;
/// assert!(is_sorted(&[1,2,2,3]));
/// assert!(!is_sorted(&[2,1]));
/// ```
pub fn is_sorted<T: Ord>(slice: &[T]) -> bool {
    slice.windows(2).all(|w| w[0] <= w[1])
}
/// Stable sort by comparator.
///
/// Example:
/// ```rust
/// use toolchest::collections::stable_sort_by;
/// let mut v = vec!["bb", "a", "ccc"];
/// stable_sort_by(&mut v, |a, b| a.len().cmp(&b.len()));
/// assert_eq!(v, vec!["a", "bb", "ccc"]);
/// ```
pub fn stable_sort_by<T, F: FnMut(&T, &T) -> std::cmp::Ordering>(v: &mut [T], mut f: F) {
    v.sort_by(|a, b| f(a, b))
}

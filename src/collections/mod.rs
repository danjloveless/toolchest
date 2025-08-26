//! Collection utilities similar to lodash (non-iterator focused)

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Split a slice into chunks of size `size`
pub fn chunk<T: Clone>(slice: &[T], size: usize) -> Vec<Vec<T>> {
    if size == 0 {
        return vec![];
    }
    slice.chunks(size).map(|c| c.to_vec()).collect()
}

/// Remove None values from a slice of Options
pub fn compact<T: Clone>(slice: &[Option<T>]) -> Vec<T> {
    slice.iter().filter_map(|o| o.clone()).collect()
}

/// Remove duplicate elements while preserving order
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

/// Elements in a that are not in b
pub fn difference<T: Eq + Hash + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let set_b: HashSet<&T> = b.iter().collect();
    a.iter().filter(|x| !set_b.contains(x)).cloned().collect()
}

/// Elements common to a and b
pub fn intersection<T: Eq + Hash + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let set_b: HashSet<&T> = b.iter().collect();
    a.iter().filter(|x| set_b.contains(x)).cloned().collect()
}

/// Union of two slices (unique elements, order preserved)
pub fn union<T: Eq + Hash + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    out.extend_from_slice(a);
    out.extend_from_slice(b);
    uniq(&out)
}

/// Flatten one level of nested vectors
pub fn flatten<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
    let total: usize = nested.iter().map(|v| v.len()).sum();
    let mut out = Vec::with_capacity(total);
    for v in nested {
        out.extend_from_slice(v);
    }
    out
}

/// Group elements by a key function
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

/// Map elements by a key function
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

/// Count elements by a key function
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

/// Partition elements into (true, false) by predicate
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

/// Shuffle elements in place
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

/// Sample a random element
pub fn sample<T>(slice: &[T]) -> Option<&T> {
    if slice.is_empty() {
        return None;
    }
    let seed = std::time::Instant::now().elapsed().as_nanos();
    let mut rng = Lcg::new(seed);
    let idx = (rng.next_u64() as usize) % slice.len();
    slice.get(idx)
}

/// Zip keys and values into a HashMap
pub fn zip_object<K: Eq + Hash + Clone, V: Clone>(keys: &[K], values: &[V]) -> HashMap<K, V> {
    let mut map = HashMap::new();
    for (k, v) in keys.iter().cloned().zip(values.iter().cloned()) {
        map.insert(k, v);
    }
    map
}

/// Split a slice of pairs into two vectors
pub fn unzip<K: Clone, V: Clone>(pairs: &[(K, V)]) -> (Vec<K>, Vec<V>) {
    let mut ks = Vec::with_capacity(pairs.len());
    let mut vs = Vec::with_capacity(pairs.len());
    for (k, v) in pairs {
        ks.push(k.clone());
        vs.push(v.clone());
    }
    (ks, vs)
}

/// First index of item
pub fn index_of<T: PartialEq>(slice: &[T], item: &T) -> Option<usize> {
    slice.iter().position(|x| x == item)
}

/// Last index of item
pub fn last_index_of<T: PartialEq>(slice: &[T], item: &T) -> Option<usize> {
    slice.iter().rposition(|x| x == item)
}

/// Take first n elements
pub fn take<T: Clone>(slice: &[T], n: usize) -> Vec<T> {
    slice.iter().take(n).cloned().collect()
}
/// Drop first n elements
pub fn drop<T: Clone>(slice: &[T], n: usize) -> Vec<T> {
    slice.iter().skip(n).cloned().collect()
}
/// Take last n elements
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
/// Drop last n elements
pub fn drop_right<T: Clone>(slice: &[T], n: usize) -> Vec<T> {
    let len = slice.len().saturating_sub(n);
    slice[..len].to_vec()
}

/// Take while predicate holds
pub fn take_while<T: Clone, F: Fn(&T) -> bool>(slice: &[T], f: F) -> Vec<T> {
    slice.iter().take_while(|x| f(x)).cloned().collect()
}
/// Drop while predicate holds, then keep rest
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

/// Rotate left by n
pub fn rotate_left<T: Clone>(v: &mut [T], n: usize) {
    v.rotate_left(n % v.len().max(1))
}
/// Rotate right by n
pub fn rotate_right<T: Clone>(v: &mut [T], n: usize) {
    v.rotate_right(n % v.len().max(1))
}

/// Insert separator between elements
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

/// Sliding windows of given size/step
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

/// Cartesian product of two slices
pub fn cartesian_product<A: Clone, B: Clone>(a: &[A], b: &[B]) -> Vec<(A, B)> {
    let mut out = Vec::with_capacity(a.len() * b.len());
    for x in a {
        for y in b {
            out.push((x.clone(), y.clone()));
        }
    }
    out
}

/// Transpose a rectangular matrix
pub fn transpose<T: Clone>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    if matrix.is_empty() {
        return vec![];
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut out = vec![vec![matrix[0][0].clone(); rows]; cols];
    for r in 0..rows {
        for c in 0..cols {
            out[c][r] = matrix[r][c].clone();
        }
    }
    out
}

/// Move item from index to index
pub fn move_item<T: Clone>(v: &mut Vec<T>, from: usize, to: usize) -> bool {
    if from >= v.len() || to >= v.len() {
        return false;
    }
    let item = v.remove(from);
    v.insert(to, item);
    true
}

/// Swap two indices if valid
pub fn swap<T>(v: &mut [T], i: usize, j: usize) -> bool {
    if i >= v.len() || j >= v.len() {
        false
    } else {
        v.swap(i, j);
        true
    }
}

/// Binary search using a comparator to Ordering
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

/// Find duplicate elements (unique list)
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

/// True if slice is non-decreasing
pub fn is_sorted<T: Ord>(slice: &[T]) -> bool {
    slice.windows(2).all(|w| w[0] <= w[1])
}
/// Stable sort by comparator
pub fn stable_sort_by<T, F: FnMut(&T, &T) -> std::cmp::Ordering>(v: &mut [T], mut f: F) {
    v.sort_by(|a, b| f(a, b))
}

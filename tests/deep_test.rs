use std::collections::HashMap;
use toolchest::deep::*;

#[test]
fn test_deep_clone_for_primitives() {
    let x = 5u32;
    let y = deep_clone(&x);
    assert_eq!(x, y);
}

#[test]
fn test_deep_equal_hashmap() {
    let mut a: HashMap<String, i32> = HashMap::new();
    a.insert("x".into(), 1);
    let mut b: HashMap<String, i32> = HashMap::new();
    b.insert("x".into(), 1);
    assert!(deep_equal(&a, &b));
}

#[test]
fn test_deep_equal_slice() {
    let a = [1, 2, 3];
    let b = [1, 2, 3];
    assert!(toolchest::deep::deep_equal_slice(&a, &b));
}

#[test]
fn test_deep_merge_hashmaps() {
    let mut a: HashMap<String, i32> = HashMap::new();
    a.insert("a".into(), 1);
    let mut b: HashMap<String, i32> = HashMap::new();
    b.insert("a".into(), 2);
    b.insert("b".into(), 3);
    
    let merged = merge(&a, &b);
    assert_eq!(merged.get("a"), Some(&2));
    assert_eq!(merged.get("b"), Some(&3));
}



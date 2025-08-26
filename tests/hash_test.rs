use toolchest::hash::*;

#[test]
fn test_hashes_basic() {
    assert_eq!(djb2(b"a"), 177670);
    let f_a = fnv1a(b"a");
    let f_a2 = fnv1a(b"a");
    let f_b = fnv1a(b"b");
    assert_ne!(f_a, 0);
    assert_eq!(f_a, f_a2);
    assert_ne!(f_a, f_b);
    let m = murmur3_32(b"hello", 0);
    assert_ne!(m, 0);
    assert_eq!(consistent_hash("key", 10) < 10, true);
}



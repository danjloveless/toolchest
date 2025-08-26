use toolchest::collections::*;

#[test]
fn test_chunk_and_uniq() {
    assert_eq!(chunk(&[1,2,3,4,5], 2), vec![vec![1,2], vec![3,4], vec![5]]);
    assert_eq!(uniq(&[1,1,2,3,3]), vec![1,2,3]);
}

#[test]
fn test_set_ops_and_group() {
    assert_eq!(difference(&[1,2,3], &[2]), vec![1,3]);
    assert_eq!(intersection(&[1,2,3], &[2,4]), vec![2]);
    assert_eq!(union(&[1,2], &[2,3]), vec![1,2,3]);
    let groups = group_by(&["a","ab","c"], |s| s.len());
    assert_eq!(groups.get(&1).unwrap().len(), 2);
}

#[test]
fn test_shuffle_and_sample() {
    let mut v = vec![1,2,3,4];
    shuffle_in_place(&mut v);
    assert_eq!(v.len(), 4);
    assert!(sample(&v).is_some());
}

#[test]
fn test_zip_unzip_index() {
    let keys = vec!["a", "b"];
    let vals = vec![1, 2];
    let m = zip_object(&keys, &vals);
    assert_eq!(m.get("a"), Some(&1));
    let pairs = vec![("x", 9), ("y", 8)];
    let (ks, vs) = unzip(&pairs);
    assert_eq!(ks, vec!["x", "y"]);
    assert_eq!(vs, vec![9, 8]);
    assert_eq!(index_of(&[1,2,3], &2), Some(1));
    assert_eq!(last_index_of(&[1,2,3,2], &2), Some(3));
}



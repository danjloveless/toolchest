use std::collections::HashMap;
use toolchest::types::*;

#[test]
fn test_is_empty() {
    assert!(is_empty::<String>(&"".to_string()));
    assert!(is_empty::<Vec<i32>>(&vec![]));
    let slice: &[i32] = &[];
    assert!(is_empty::<&[i32]>(&slice));
    let map: HashMap<i32, i32> = HashMap::new();
    assert!(is_empty::<HashMap<i32, i32>>(&map));
}

#[test]
fn test_types_extras() {
    assert_eq!(map_some_or(Some(2), 0, |x| x * 3), 6);
    assert_eq!(map_some_or::<i32, i32, _>(None, 5, |x| x * 3), 5);
    assert_eq!(map_ok_or::<_, (), _, _>(Ok(3), 0, |x| x + 1), 4);
    assert_eq!(map_ok_or::<i32, (), i32, _>(Err(()), 9, |x| x + 1), 9);

    let nev = NonEmptyVec::from_vec(vec![1, 2, 3]).unwrap();
    assert_eq!(nev.len(), 3);
    assert_eq!(nev.iter().copied().collect::<Vec<_>>(), vec![1, 2, 3]);
}
#[test]
fn test_parse_and_default() {
    assert_eq!(parse_or_default::<i32>("42"), 42);
    assert_eq!(parse_or_default::<i32>("x"), 0);
    assert_eq!(parse_or::<i32>("x", 7), 7);
}



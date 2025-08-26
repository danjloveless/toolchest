use toolchest::random::*;

#[test]
fn test_random_basics() {
    let x = random_range(0, 100);
    assert!(0 <= x && x < 100);
    let b = random_bool(0.5);
    assert!(b == true || b == false);
    let c = random_choice(&[1, 2, 3]).copied();
    assert!(c.is_some());
    let u = uuid_v4();
    assert_eq!(u.len(), 36);
}

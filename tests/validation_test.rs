use toolchest::validation::*;

#[test]
fn test_validation_basics() {
    // 4111111111111111 is a common test Visa (Luhn passes)
    assert!(validate_credit_card("4111111111111111"));
    assert!(is_ascii("hello"));
    assert!(is_utf8(b"hello"));
}



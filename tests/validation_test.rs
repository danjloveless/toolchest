use toolchest::validation::*;

#[test]
fn test_validation_basics() {
    // 4111111111111111 is a common test Visa (Luhn passes)
    assert!(validate_credit_card("4111111111111111"));
    assert!(is_ascii("hello"));
    assert!(is_utf8(b"hello"));
}

#[test]
fn test_validate_iban() {
    // Known valid IBANs (from public examples)
    assert!(validate_iban("GB82 WEST 1234 5698 7654 32"));
    assert!(validate_iban("DE89 3704 0044 0532 0130 00"));
    assert!(validate_iban("FR14 2004 1010 0505 0001 3M02 606"));
    assert!(validate_iban("GR16 0110 1250 0000 0001 2300 695"));

    // Invalid: wrong checksum / length / characters
    assert!(!validate_iban("GB82 TEST 1234 5698 7654 32")); // invalid bank code
    assert!(!validate_iban("DE89 3704 0044 0532 0130 0")); // too short
    assert!(!validate_iban("DE89 3704 0044 0532 0130 00!")); // invalid char
}

#[test]
fn test_validate_phone_e164() {
    // Valid E.164 examples
    assert!(validate_phone("+12025550123"));
    assert!(validate_phone("+442071838750"));
    assert!(validate_phone("+918527001234"));

    // Invalid: missing '+', too long, leading zero, non-digits
    assert!(!validate_phone("12025550123"));
    assert!(!validate_phone("+0123456789"));
    assert!(!validate_phone("+1234567890123456")); // >15 digits
    assert!(!validate_phone("+12345abc"));
}

#[test]
fn test_validate_ssn_us() {
    // Valid formatted and compact
    assert!(validate_ssn("123-45-6789"));
    assert!(validate_ssn("123456789"));

    // Invalid: forbidden area values, group/serial zeros, wrong length
    assert!(!validate_ssn("000-12-3456")); // area 000
    assert!(!validate_ssn("666-12-3456")); // area 666
    assert!(!validate_ssn("900-12-3456")); // area 900-999
    assert!(!validate_ssn("123-00-6789")); // group 00
    assert!(!validate_ssn("123-45-0000")); // serial 0000
    assert!(!validate_ssn("123-45-678")); // too short
}

//! Validation helpers.
//!
//! Lightweight validators and stubs for common formats. Includes a Luhn
//! implementation for credit card numbers and simple ASCII/UTF-8 checks.
//! Placeholders are provided for IBAN/phone/SSN with clear stubs.
//!
//! Examples:
//! ```rust
//! use toolchest::validation::{validate_credit_card, is_ascii, is_utf8};
//! assert!(validate_credit_card("4242424242424242"));
//! assert!(is_ascii("hello"));
//! assert!(is_utf8("ok".as_bytes()));
//! ```

/// Validate credit card number using Luhn algorithm
pub fn validate_credit_card(num: &str) -> bool {
    luhn(num)
}
fn luhn(num: &str) -> bool {
    let digits: Vec<u32> = num.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.is_empty() {
        return false;
    }
    let mut sum = 0u32;
    let mut dbl = false;
    for d in digits.iter().rev() {
        let mut v = *d;
        if dbl {
            v *= 2;
            if v > 9 {
                v -= 9;
            }
        }
        sum += v;
        dbl = !dbl;
    }
    sum % 10 == 0
}

/// Stub: Validate IBAN (not implemented)
pub fn validate_iban(_iban: &str) -> bool {
    unimplemented!("IBAN validation not yet implemented")
}
/// Stub: Validate phone number (not implemented)
pub fn validate_phone(_phone: &str) -> bool {
    unimplemented!("Phone validation not yet implemented")
}
/// Stub: Validate SSN (not implemented)
pub fn validate_ssn(_ssn: &str) -> bool {
    unimplemented!("SSN validation not yet implemented")
}
/// True if string is ASCII
pub fn is_ascii(s: &str) -> bool {
    s.is_ascii()
}
/// True if bytes are valid UTF-8
pub fn is_utf8(bytes: &[u8]) -> bool {
    std::str::from_utf8(bytes).is_ok()
}

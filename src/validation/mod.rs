//! Validation helpers

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
    false
}
/// Stub: Validate phone number (not implemented)
pub fn validate_phone(_phone: &str) -> bool {
    false
}
/// Stub: Validate SSN (not implemented)
pub fn validate_ssn(_ssn: &str) -> bool {
    false
}
/// True if string is ASCII
pub fn is_ascii(s: &str) -> bool {
    s.is_ascii()
}
/// True if bytes are valid UTF-8
pub fn is_utf8(bytes: &[u8]) -> bool {
    std::str::from_utf8(bytes).is_ok()
}

//! Validation helpers.
//!
//! Lightweight validators for common formats. Includes a Luhn
//! implementation for credit card numbers, IBAN/phone/SSN validation, and
//! simple ASCII/UTF-8 checks.
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

/// Validate IBAN using the ISO 13616 algorithm (mod-97 == 1)
///
/// Rules implemented:
/// - Strip spaces, uppercase; must be 15..=34 alphanumeric characters
/// - Move first 4 chars to the end
/// - Replace letters A..Z with 10..35 and compute the remainder modulo 97
/// - Valid when remainder equals 1
///
/// Example:
/// ```rust
/// use toolchest::validation::validate_iban;
/// assert!(validate_iban("DE89 3704 0044 0532 0130 00"));
/// assert!(!validate_iban("DE89 3704 0044 0532 0130 0"));
/// ```
pub fn validate_iban(iban: &str) -> bool {
    // Normalize: remove spaces, uppercase
    let mut s: String = iban.chars().filter(|c| !c.is_whitespace()).collect();
    s.make_ascii_uppercase();
    // Basic length and charset check
    if s.len() < 15 || s.len() > 34 || !s.chars().all(|c| c.is_ascii_alphanumeric()) {
        return false;
    }
    // Rearrange: move first 4 chars to the end. Safe due to ASCII-only check
    let rearranged = format!("{}{}", &s[4..], &s[..4]);
    // Compute mod-97 incrementally to avoid big integers
    let mut rem: u32 = 0;
    for ch in rearranged.chars() {
        let mapped = if ch.is_ascii_alphabetic() {
            // A=10 .. Z=35
            ((ch as u8 - b'A') as u32 + 10).to_string()
        } else {
            ch.to_string()
        };
        for d in mapped.bytes() {
            let digit = (d - b'0') as u32;
            rem = (rem * 10 + digit) % 97;
        }
    }
    rem == 1
}

/// Validate a phone number in E.164 format.
///
/// Requirements:
/// - Must start with '+'
/// - Must contain 1..=15 digits after '+'
/// - First digit after '+' must be 1..=9 (no leading zero)
///
/// Example:
/// ```rust
/// use toolchest::validation::validate_phone;
/// assert!(validate_phone("+12025550123"));
/// assert!(!validate_phone("12025550123"));
/// ```
pub fn validate_phone(phone: &str) -> bool {
    let mut chars = phone.chars();
    match chars.next() {
        Some('+') => {}
        _ => return false,
    }
    let digits: String = chars.collect();
    if digits.is_empty() || digits.len() > 15 {
        return false;
    }
    let mut iter = digits.chars();
    match iter.next() {
        Some(c) if c.is_ascii_digit() && c != '0' => {}
        _ => return false,
    }
    if !iter.all(|c| c.is_ascii_digit()) {
        return false;
    }
    true
}

/// Validate a US Social Security Number (SSN).
///
/// Accepts either 'AAA-GG-SSSS' or 'AAAGGSSSS' with the following rules:
/// - Area (AAA) cannot be 000, 666, or 900..=999
/// - Group (GG) cannot be 00
/// - Serial (SSSS) cannot be 0000
///
/// Example:
/// ```rust
/// use toolchest::validation::validate_ssn;
/// assert!(validate_ssn("123-45-6789"));
/// assert!(!validate_ssn("000-12-3456"));
/// ```
pub fn validate_ssn(ssn: &str) -> bool {
    let digits: String = ssn.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() != 9 {
        return false;
    }
    let area: u32 = digits[0..3].parse().unwrap_or(0);
    let group: u32 = digits[3..5].parse().unwrap_or(0);
    let serial: u32 = digits[5..9].parse().unwrap_or(0);

    if area == 0 || area == 666 || (900..=999).contains(&area) {
        return false;
    }
    if group == 0 || serial == 0 {
        return false;
    }
    true
}
/// True if string is ASCII
pub fn is_ascii(s: &str) -> bool {
    s.is_ascii()
}
/// True if bytes are valid UTF-8
pub fn is_utf8(bytes: &[u8]) -> bool {
    std::str::from_utf8(bytes).is_ok()
}

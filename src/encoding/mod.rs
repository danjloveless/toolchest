//! Encoding helpers.
//!
//! Lightweight string/byte encoding utilities including hex, ROT13, Caesar
//! cipher, and Base32 (RFC 4648 without padding).
//!
//! Examples:
//! ```rust
//! use toolchest::encoding::{hex_encode, hex_decode, rot13, caesar_cipher, base32_encode, base32_decode};
//!
//! assert_eq!(hex_encode(&[0x0f, 0xaa]), "0faa");
//! assert_eq!(hex_decode("0faa").unwrap(), vec![0x0f, 0xaa]);
//!
//! assert_eq!(rot13("Hello"), "Uryyb");
//! assert_eq!(caesar_cipher("abcXYZ", 2), "cdeZAB");
//!
//! let b32 = base32_encode(b"hi");
//! assert_eq!(base32_decode(&b32).unwrap(), b"hi");
//! ```

/// Hex-encode bytes to lowercase string
pub fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
/// Decode lowercase/uppercase hex string into bytes.
///
/// Returns `None` if the input length is odd or contains non-hex characters.
///
/// Example:
/// ```rust
/// use toolchest::encoding::{hex_encode, hex_decode};
/// let s = hex_encode(&[0xde, 0xad, 0xbe, 0xef]);
/// assert_eq!(s, "deadbeef");
/// assert_eq!(hex_decode(&s).unwrap(), vec![0xde, 0xad, 0xbe, 0xef]);
/// ```
pub fn hex_decode(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 {
        return None;
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
        .collect()
}

/// ROT13 transformation for ASCII letters.
///
/// Non-ASCII letters are left unchanged.
///
/// Example:
/// ```rust
/// use toolchest::encoding::rot13;
/// assert_eq!(rot13("Hello, World!"), "Uryyb, Jbeyq!");
/// ```
pub fn rot13(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'a'..='z' => (((c as u8 - b'a' + 13) % 26) + b'a') as char,
            'A'..='Z' => (((c as u8 - b'A' + 13) % 26) + b'A') as char,
            _ => c,
        })
        .collect()
}

/// Caesar cipher for ASCII letters.
///
/// Shifts alphabetic characters by `shift` (wrapping), preserves case, and
/// leaves other characters unchanged. Negative shifts are supported.
///
/// Example:
/// ```rust
/// use toolchest::encoding::caesar_cipher;
/// assert_eq!(caesar_cipher("abc XYZ", 2), "cde ZAB");
/// assert_eq!(caesar_cipher("cde ZAB", -2), "abc XYZ");
/// ```
pub fn caesar_cipher(s: &str, shift: i8) -> String {
    let sh = shift.rem_euclid(26) as u8;
    s.chars()
        .map(|c| match c {
            'a'..='z' => (((c as u8 - b'a' + sh) % 26) + b'a') as char,
            'A'..='Z' => (((c as u8 - b'A' + sh) % 26) + b'A') as char,
            _ => c,
        })
        .collect()
}

/// Base32 encode (RFC 4648, no padding).
///
/// Example:
/// ```rust
/// use toolchest::encoding::{base32_encode, base32_decode};
/// let enc = base32_encode(b"foo");
/// assert_eq!(base32_decode(&enc).unwrap(), b"foo");
/// ```
pub fn base32_encode(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut out = String::new();
    let mut buffer: u64 = 0;
    let mut bits: u8 = 0;
    for &b in bytes {
        buffer = (buffer << 8) | b as u64;
        bits += 8;
        while bits >= 5 {
            let idx = ((buffer >> (bits - 5)) & 0x1F) as usize;
            out.push(ALPHABET[idx] as char);
            bits -= 5;
        }
    }
    if bits > 0 {
        let idx = ((buffer << (5 - bits)) & 0x1F) as usize;
        out.push(ALPHABET[idx] as char);
    }
    out
}
/// Base32 decode (RFC 4648, no padding).
///
/// Non-alphabet characters are ignored. Returns decoded bytes if successful.
///
/// Example:
/// ```rust
/// use toolchest::encoding::{base32_encode, base32_decode};
/// let enc = base32_encode(b"test");
/// assert_eq!(base32_decode(&enc).unwrap(), b"test");
/// ```
pub fn base32_decode(s: &str) -> Option<Vec<u8>> {
    fn val(c: u8) -> Option<u8> {
        match c {
            b'A'..=b'Z' => Some(c - b'A'),
            b'2'..=b'7' => Some(26 + (c - b'2')),
            b'a'..=b'z' => Some(c - b'a'),
            _ => None,
        }
    }
    let mut out = Vec::new();
    let mut buffer: u64 = 0;
    let mut bits: u8 = 0;
    for &ch in s.as_bytes() {
        let v = match val(ch) {
            Some(v) => v,
            None => continue,
        } as u64;
        buffer = (buffer << 5) | v;
        bits += 5;
        while bits >= 8 {
            let byte = ((buffer >> (bits - 8)) & 0xFF) as u8;
            out.push(byte);
            bits -= 8;
        }
    }
    Some(out)
}

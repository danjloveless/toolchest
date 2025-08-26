//! Encoding helpers

/// Hex-encode bytes to lowercase string
pub fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
/// Decode lowercase/uppercase hex string into bytes
pub fn hex_decode(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 {
        return None;
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
        .collect()
}

/// ROT13 transformation for ASCII letters
pub fn rot13(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'a'..='z' => (((c as u8 - b'a' + 13) % 26) + b'a') as char,
            'A'..='Z' => (((c as u8 - b'A' + 13) % 26) + b'A') as char,
            _ => c,
        })
        .collect()
}

/// Caesar cipher for ASCII letters
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

/// Base32 encode (RFC 4648, no padding)
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
/// Base32 decode (RFC 4648, no padding)
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

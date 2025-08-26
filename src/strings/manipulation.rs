//! String manipulation utilities

/// Safely take a prefix by byte length, not splitting UTF-8 characters
fn prefix_by_bytes(s: &str, max_bytes: usize) -> &str {
    if max_bytes >= s.len() {
        return s;
    }
    // Find the last char boundary within max_bytes
    let mut end = 0usize;
    for (idx, ch) in s.char_indices() {
        let next = idx + ch.len_utf8();
        if next > max_bytes {
            break;
        }
        end = next;
    }
    &s[..end]
}

/// Truncate a string to a maximum byte length with ellipsis
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len < 3 {
        prefix_by_bytes(s, max_len).to_string()
    } else {
        let available = max_len - 3;
        format!("{}...", prefix_by_bytes(s, available))
    }
}

/// Truncate with custom suffix
pub fn truncate_with(s: &str, max_len: usize, suffix: &str) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        let available = max_len.saturating_sub(suffix.len());
        format!("{}{}", prefix_by_bytes(s, available), suffix)
    }
}

/// Pad string start to reach target length
pub fn pad_start(s: &str, target_len: usize, pad_char: char) -> String {
    let current_len = s.chars().count();
    if current_len >= target_len {
        s.to_string()
    } else {
        let pad_count = target_len - current_len;
        let padding: String = pad_char.to_string().repeat(pad_count);
        format!("{}{}", padding, s)
    }
}

/// Pad string end to reach target length  
pub fn pad_end(s: &str, target_len: usize, pad_char: char) -> String {
    let current_len = s.chars().count();
    if current_len >= target_len {
        s.to_string()
    } else {
        let pad_count = target_len - current_len;
        let padding: String = pad_char.to_string().repeat(pad_count);
        format!("{}{}", s, padding)
    }
}

/// Capitalize first character
pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Uncapitalize first character
pub fn uncapitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
    }
}

/// Trim whitespace from string
#[inline]
pub fn trim(s: &str) -> String {
    s.trim().to_string()
}



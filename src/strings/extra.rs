//! Additional string utilities: slugify, pluralize, singularize, levenshtein

/// Create a URL-friendly slug from a string
pub fn slugify(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut last_dash = false;
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            last_dash = false;
        } else if (ch.is_whitespace() || ch == '-' || ch == '_') && !last_dash && !out.is_empty() {
            out.push('-');
            last_dash = true;
        }
    }
    if out.ends_with('-') {
        out.pop();
    }
    out
}

/// Simple template interpolation: replaces {{key}} using provider
pub fn template<F>(input: &str, mut provider: F) -> String
where
    F: FnMut(&str) -> Option<String>,
{
    let mut out = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if i + 3 < bytes.len() && &bytes[i..i + 2] == b"{{" {
            if let Some(end) = input[i + 2..].find("}}").map(|e| i + 2 + e) {
                let key = &input[i + 2..end].trim();
                if let Some(val) = provider(key) {
                    out.push_str(&val);
                } else {
                    out.push_str(&input[i..end + 2]);
                }
                i = end + 2;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

/// Case-insensitive contains
pub fn contains_ci(haystack: &str, needle: &str) -> bool {
    haystack.to_lowercase().contains(&needle.to_lowercase())
}

/// Case-insensitive starts_with
pub fn starts_with_ci(haystack: &str, prefix: &str) -> bool {
    haystack.to_lowercase().starts_with(&prefix.to_lowercase())
}

/// Case-insensitive ends_with
pub fn ends_with_ci(haystack: &str, suffix: &str) -> bool {
    haystack.to_lowercase().ends_with(&suffix.to_lowercase())
}

/// Strip prefix if present
pub fn strip_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {
    s.strip_prefix(prefix).unwrap_or(s)
}

/// Strip suffix if present
pub fn strip_suffix<'a>(s: &'a str, suffix: &str) -> &'a str {
    s.strip_suffix(suffix).unwrap_or(s)
}

/// Ensure string has the prefix, adding if missing
pub fn ensure_prefix(s: &str, prefix: &str) -> String {
    if s.starts_with(prefix) {
        s.to_string()
    } else {
        format!("{prefix}{s}")
    }
}

/// Ensure string has the suffix, adding if missing
pub fn ensure_suffix(s: &str, suffix: &str) -> String {
    if s.ends_with(suffix) {
        s.to_string()
    } else {
        format!("{s}{suffix}")
    }
}

/// Very simple pluralize for common English nouns
pub fn pluralize(word: &str) -> String {
    if word.ends_with("y") && !matches!(word.chars().nth_back(1), Some('a' | 'e' | 'i' | 'o' | 'u'))
    {
        let mut s = word.to_string();
        s.pop();
        s.push_str("ies");
        s
    } else if word.ends_with('s')
        || word.ends_with("x")
        || word.ends_with("ch")
        || word.ends_with("sh")
    {
        format!("{word}es")
    } else {
        format!("{word}s")
    }
}

/// Very simple singularize matching the above pluralize
pub fn singularize(word: &str) -> String {
    if let Some(base) = word.strip_suffix("ies") {
        format!("{base}y")
    } else if let Some(base) = word.strip_suffix("es") {
        base.to_string()
    } else if let Some(base) = word.strip_suffix('s') {
        base.to_string()
    } else {
        word.to_string()
    }
}

/// Levenshtein distance between two strings
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let (a_len, b_len) = (a.chars().count(), b.chars().count());
    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }
    let mut prev: Vec<usize> = (0..=b_len).collect();
    let mut curr = vec![0usize; b_len + 1];
    let b_chars: Vec<char> = b.chars().collect();
    for (i, ca) in a.chars().enumerate() {
        curr[0] = i + 1;
        for (j, &cb) in b_chars.iter().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            curr[j + 1] = (prev[j + 1] + 1).min(curr[j] + 1).min(prev[j] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[b_len]
}

/// Damerau-Levenshtein distance (allows transposition)
pub fn damerau_levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let (m, n) = (a_chars.len(), b_chars.len());
    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }
    let mut d = vec![vec![0usize; n + 1]; m + 1];
    for (i, row) in d.iter_mut().enumerate().take(m + 1) {
        row[0] = i;
    }
    if let Some(first_row) = d.get_mut(0) {
        for (j, cell) in first_row.iter_mut().enumerate().take(n + 1) {
            *cell = j;
        }
    }
    for i in 1..=m {
        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            d[i][j] = (d[i - 1][j] + 1)
                .min(d[i][j - 1] + 1)
                .min(d[i - 1][j - 1] + cost);
            if i > 1
                && j > 1
                && a_chars[i - 1] == b_chars[j - 2]
                && a_chars[i - 2] == b_chars[j - 1]
            {
                d[i][j] = d[i][j].min(d[i - 2][j - 2] + 1);
            }
        }
    }
    d[m][n]
}

/// Reverse characters of a string
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

/// Check if string is a palindrome (ignores case and non-alphanumerics)
pub fn is_palindrome(s: &str) -> bool {
    let filtered: String = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    filtered.chars().eq(filtered.chars().rev())
}

/// Count non-overlapping occurrences of a substring
pub fn count_occurrences(haystack: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    haystack.match_indices(needle).count()
}

/// Find all start indices of a substring
pub fn find_all_indices(haystack: &str, needle: &str) -> Vec<usize> {
    if needle.is_empty() {
        return vec![];
    }
    haystack.match_indices(needle).map(|(i, _)| i).collect()
}

/// Word wrap text at width, breaking on whitespace
pub fn wrap(text: &str, width: usize) -> String {
    if width == 0 {
        return text.to_string();
    }
    let mut out = String::new();
    let mut line_len = 0usize;
    for word in text.split_whitespace() {
        let wlen = word.len();
        if line_len == 0 {
            out.push_str(word);
            line_len = wlen;
        } else if line_len + 1 + wlen <= width {
            out.push(' ');
            out.push_str(word);
            line_len += 1 + wlen;
        } else {
            out.push('\n');
            out.push_str(word);
            line_len = wlen;
        }
    }
    out
}

/// Add indentation with given prefix to each non-empty line
pub fn indent(text: &str, prefix: &str) -> String {
    text.lines()
        .map(|l| {
            if l.is_empty() {
                "".to_string()
            } else {
                format!("{prefix}{l}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Remove up to n leading spaces or a given prefix from each line
pub fn dedent(text: &str, n: usize) -> String {
    text.lines()
        .map(|l| {
            let mut count = 0usize;
            let mut idx = 0usize;
            for ch in l.chars() {
                if ch == ' ' && count < n {
                    count += 1;
                    idx += 1;
                } else {
                    break;
                }
            }
            &l[idx..]
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Check if string looks like an email
pub fn is_email(s: &str) -> bool {
    s.contains('@')
        && s.split('@').count() == 2
        && s.split('@').nth(1).is_some_and(|d| d.contains('.'))
}

/// Check if string looks like a URL (very basic)
pub fn is_url(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://")
}

/// Check if string is UUID v4 format
pub fn is_uuid(s: &str) -> bool {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 5 {
        return false;
    }
    let lens = [8, 4, 4, 4, 12];
    if !parts
        .iter()
        .zip(lens.iter())
        .all(|(p, &len)| p.len() == len && p.chars().all(|c| c.is_ascii_hexdigit()))
    {
        return false;
    }
    parts[2].starts_with('4')
}

/// Check if string is numeric (digits only)
pub fn is_numeric(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

/// Repeat string n times
pub fn repeat(s: &str, n: usize) -> String {
    s.repeat(n)
}

/// Replace all occurrences with callback providing replacement per match
pub fn replace_all<F>(s: &str, needle: &str, mut f: F) -> String
where
    F: FnMut(&str) -> String,
{
    if needle.is_empty() {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len());
    let mut last = 0usize;
    for (idx, m) in s.match_indices(needle) {
        out.push_str(&s[last..idx]);
        out.push_str(&f(m));
        last = idx + needle.len();
    }
    out.push_str(&s[last..]);
    out
}

/// Smart word splitting: splits by whitespace, underscores, hyphens, and camelCase boundaries
pub fn split_words(s: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    for ch in s.chars() {
        if ch.is_whitespace() || ch == '_' || ch == '-' {
            if !current.is_empty() {
                words.push(current.clone());
                current.clear();
            }
            continue;
        }
        if ch.is_ascii_uppercase()
            && !current.is_empty()
            && current
                .chars()
                .last()
                .is_some_and(|c| c.is_ascii_lowercase())
        {
            words.push(current.clone());
            current.clear();
        }
        current.push(ch);
    }
    if !current.is_empty() {
        words.push(current);
    }
    words
}

/// Longest common prefix of two strings
pub fn longest_common_prefix(a: &str, b: &str) -> String {
    let mut out = String::new();
    for (ca, cb) in a.chars().zip(b.chars()) {
        if ca == cb {
            out.push(ca);
        } else {
            break;
        }
    }
    out
}

/// Longest common suffix of two strings
pub fn longest_common_suffix(a: &str, b: &str) -> String {
    let mut out = String::new();
    for (ca, cb) in a.chars().rev().zip(b.chars().rev()) {
        if ca == cb {
            out.insert(0, ca);
        } else {
            break;
        }
    }
    out
}

/// Shell-escape a string by wrapping in single quotes and escaping inner single quotes
pub fn escape_shell(s: &str) -> String {
    format!("'{}'", s.replace("'", "'\\''"))
}

/// Random ASCII string from given charset
pub fn random_string(len: usize) -> String {
    let charset = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut state = std::time::Instant::now().elapsed().as_nanos();
    let mut next_u64 = || {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        (state >> 32) as u64
    };
    let mut out = String::with_capacity(len);
    for _ in 0..len {
        let idx = (next_u64() as usize) % charset.len();
        out.push(charset[idx] as char);
    }
    out
}

/// Mask part of a string, leaving prefix and suffix visible
pub fn mask(s: &str, prefix: usize, suffix: usize, mask_char: char) -> String {
    if s.len() <= prefix + suffix {
        return s.to_string();
    }
    let mut out = String::new();
    out.push_str(&s[..prefix]);
    out.push_str(&mask_char.to_string().repeat(s.len() - prefix - suffix));
    out.push_str(&s[s.len() - suffix..]);
    out
}

/// Truncate the middle with ellipsis if longer than max_len
pub fn ellipsis_middle(s: &str, max_len: usize) -> String {
    if s.len() <= max_len || max_len < 3 {
        return s.to_string();
    }
    let side = (max_len - 3) / 2;
    format!("{}...{}", &s[..side], &s[s.len() - side..])
}

/// Collapse consecutive whitespace to single spaces and trim ends
pub fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

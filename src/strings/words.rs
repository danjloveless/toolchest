//! Word operations utilities

/// Split a string into words (alphanumeric sequences)
pub fn words(input: &str) -> Vec<&str> {
    let mut res = Vec::new();
    let mut start: Option<usize> = None;
    for (i, ch) in input.char_indices() {
        if ch.is_alphanumeric() {
            if start.is_none() {
                start = Some(i);
            }
        } else if let Some(s) = start.take() {
            res.push(&input[s..i]);
        }
    }
    if let Some(s) = start {
        res.push(&input[s..]);
    }
    res
}

/// Count words in a string
pub fn word_count(input: &str) -> usize {
    words(input).len()
}

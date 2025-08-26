//! Simple path helpers (platform-agnostic logical normalization)

/// Normalize a path by removing `.` and resolving `..` segments
pub fn normalize_path(p: &str) -> String {
    let mut parts = Vec::new();
    for part in p.split(['/', '\\']) {
        if part.is_empty() || part == "." {
            continue;
        }
        if part == ".." {
            parts.pop();
        } else {
            parts.push(part);
        }
    }
    parts.join("/")
}

/// Join two path segments and then normalize the result
pub fn join_paths(a: &str, b: &str) -> String {
    if a.is_empty() {
        return normalize_path(b);
    }
    if b.is_empty() {
        return normalize_path(a);
    }
    normalize_path(&format!(
        "{}/{}",
        a.trim_end_matches(['/', '\\']),
        b.trim_start_matches(['/', '\\'])
    ))
}

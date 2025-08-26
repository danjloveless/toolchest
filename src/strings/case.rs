//! Case conversion utilities

/// Convert a string to snake_case
/// 
/// # Examples
/// ```
/// use toolchest::strings::to_snake_case;
/// assert_eq!(to_snake_case("HelloWorld"), "hello_world");
/// assert_eq!(to_snake_case("hello-world"), "hello_world");
/// assert_eq!(to_snake_case("HELLO_WORLD"), "hello_world");
/// ```
#[inline]
pub fn to_snake_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + s.len() / 4);
    let mut prev_is_upper = false;
    let mut first = true;
    let mut last_was_sep = false;
    
    for ch in s.chars() {
        if ch.is_ascii_uppercase() {
            if !first && !prev_is_upper && !last_was_sep {
                result.push('_');
            }
            result.push(ch.to_ascii_lowercase());
            prev_is_upper = true;
            last_was_sep = false;
        } else if ch == '-' || ch == ' ' || ch == '_' {
            if !result.is_empty() && !result.ends_with('_') {
                result.push('_');
            }
            prev_is_upper = false;
            last_was_sep = true;
        } else {
            result.push(ch.to_ascii_lowercase());
            prev_is_upper = false;
            last_was_sep = false;
        }
        first = false;
    }
    
    result
}

/// Convert to camelCase
#[inline]
pub fn to_camel_case(s: &str) -> String {
    let snake = to_snake_case(s);
    let mut result = String::with_capacity(snake.len());
    let mut capitalize_next = false;
    
    for (i, ch) in snake.chars().enumerate() {
        if ch == '_' {
            capitalize_next = true;
        } else if capitalize_next || i == 0 {
            result.push(ch.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(ch);
        }
    }
    
    // First character should be lowercase for camelCase
    if let Some(first) = result.chars().next() {
        result = format!("{}{}", first.to_lowercase(), &result[1..]);
    }
    
    result
}

/// Convert to PascalCase  
#[inline]
pub fn to_pascal_case(s: &str) -> String {
    let mut camel = to_camel_case(s);
    if let Some(first) = camel.chars().next() {
        camel = format!("{}{}", first.to_uppercase(), &camel[1..]);
    }
    camel
}

/// Convert to kebab-case
#[inline]
pub fn to_kebab_case(s: &str) -> String {
    to_snake_case(s).replace('_', "-")
}

/// Convert to Title Case
pub fn to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}



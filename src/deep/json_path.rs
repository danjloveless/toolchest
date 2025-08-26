//! JSON path helpers for serde_json::Value

#[cfg(feature = "json")]
use serde_json::Value;

/// Get a reference to the JSON value at dot-separated `path`
#[cfg(feature = "json")]
pub fn json_get<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = value;
    for part in path.split('.') {
        match current {
            Value::Object(map) => {
                current = map.get(part)?;
            }
            Value::Array(arr) => {
                let idx: usize = part.parse().ok()?;
                current = arr.get(idx)?;
            }
            _ => return None,
        }
    }
    Some(current)
}

/// Set the JSON value at `path`, creating objects along the way
#[cfg(feature = "json")]
pub fn json_set(value: &mut Value, path: &str, new_value: Value) -> bool {
    let mut parts = path.split('.').collect::<Vec<_>>();
    if parts.is_empty() { return false; }
    let last = parts.pop().unwrap();
    let mut current = value;
    for part in parts {
        match current {
            Value::Object(map) => {
                current = map.entry(part).or_insert(Value::Object(Default::default()));
            }
            Value::Array(arr) => {
                if let Ok(idx) = part.parse::<usize>() {
                    if idx >= arr.len() { return false; }
                    current = &mut arr[idx];
                } else { return false; }
            }
            _ => return false,
        }
    }
    match current {
        Value::Object(map) => { map.insert(last.to_string(), new_value); true }
        Value::Array(arr) => {
            if let Ok(idx) = last.parse::<usize>() {
                if idx >= arr.len() { return false; }
                arr[idx] = new_value; true
            } else { false }
        }
        _ => false,
    }
}

/// True if a value exists at `path`
#[cfg(feature = "json")]
pub fn json_has(value: &Value, path: &str) -> bool { json_get(value, path).is_some() }

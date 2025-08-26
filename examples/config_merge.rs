use std::collections::HashMap;

fn main() {
    let mut a: HashMap<String, i32> = HashMap::new();
    a.insert("retries".into(), 1);

    let mut b: HashMap<String, i32> = HashMap::new();
    b.insert("retries".into(), 3);
    b.insert("timeout".into(), 30);

    let merged = toolchest::deep::merge(&a, &b);
    println!("merged: {:?}", merged);
}

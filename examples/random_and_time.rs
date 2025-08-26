use toolchest::{random, time};
use std::time::Duration;

fn main() {
    let choice = random::random_choice(&["red", "green", "blue"]).unwrap();
    println!("choice: {}", choice);

    let (val, took) = time::elapsed(|| {
        std::thread::sleep(Duration::from_millis(20));
        42
    });
    println!("val={}, elapsed={}", val, time::duration_humanize(took));
}


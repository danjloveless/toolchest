use std::time::Duration;

fn main() {
    let debounced = toolchest::functions::debounce(
        || {
            println!("executed");
        },
        Duration::from_millis(100),
    );

    debounced.call();
    debounced.call();
    std::thread::sleep(Duration::from_millis(150));
}

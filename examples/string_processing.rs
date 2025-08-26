fn main() {
    let input = "HelloWorld";
    let snake = toolchest::strings::to_snake_case(input);
    println!("{} -> {}", input, snake);
}

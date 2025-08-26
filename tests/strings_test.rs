use toolchest::strings::*;

#[test]
fn test_snake_case_variants() {
    assert_eq!(to_snake_case("HelloWorld"), "hello_world");
    assert_eq!(to_snake_case("helloWorld"), "hello_world");
    assert_eq!(to_snake_case("hello-world"), "hello_world");
    assert_eq!(to_snake_case("hello world"), "hello_world");
    assert_eq!(to_snake_case("HELLO_WORLD"), "hello_world");
    assert_eq!(to_snake_case(""), "");
    assert_eq!(to_snake_case("a"), "a");
}

#[test]
fn test_escape_and_words() {
    assert_eq!(escape::escape_html("<a>&\"'"), "&lt;a&gt;&amp;&quot;&#39;");
    assert_eq!(escape::escape_regex("a+b?c"), r"a\+b\?c");
    let w = words::words("Hello, world! 42");
    assert_eq!(w, vec!["Hello", "world", "42"]);
    assert_eq!(words::word_count("one two three"), 3);
}

#[test]
fn test_extra_strings() {
    assert_eq!(slugify("Hello, World!"), "hello-world");
    assert_eq!(pluralize("box"), "boxes");
    assert_eq!(singularize("boxes"), "box");
    assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
    assert_eq!(extra::damerau_levenshtein("ca", "ac"), 1);
    let tpl = extra::template("Hello {{ name }}!", |k| if k=="name" { Some("Rust".into()) } else { None });
    assert_eq!(tpl, "Hello Rust!");
}

#[test]
fn test_strings_helpers_more() {
    assert!(extra::contains_ci("Hello", "he"));
    assert!(extra::starts_with_ci("Hello", "he"));
    assert!(extra::ends_with_ci("Hello", "LO"));
    assert_eq!(extra::strip_prefix("/path", "/"), "path");
    assert_eq!(extra::strip_suffix("file.txt", ".txt"), "file");
    assert_eq!(extra::ensure_prefix("path", "/"), "/path");
    assert_eq!(extra::ensure_suffix("file", ".txt"), "file.txt");
}

#[test]
fn test_url_and_path() {
    assert_eq!(url::url_encode("a b"), "a%20b");
    assert_eq!(url::url_decode("a%20b"), "a b");
    assert_eq!(path::normalize_path("a/./b/../c"), "a/c");
    assert_eq!(path::join_paths("a/b", "c/d"), "a/b/c/d");
}
#[test]
fn test_truncate() {
    assert_eq!(truncate("Hello World", 5), "He...");
    assert_eq!(truncate("Hi", 5), "Hi");
    assert_eq!(truncate("Hello", 5), "Hello");
    assert_eq!(truncate("Testing", 7), "Testing");
}

use proptest::prelude::*;

proptest! {
    #[test]
    fn truncate_never_exceeds_max_length(s in "\\PC*", max_len in 0..100usize) {
        let result = truncate(&s, max_len);
        assert!(result.len() <= max_len.max(3));
    }
}



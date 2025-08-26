use toolchest::encoding::*;

#[test]
fn test_hex_rot13_caesar() {
    let bytes = b"abc";
    let h = hex_encode(bytes);
    assert_eq!(h, "616263");
    assert_eq!(hex_decode(&h).unwrap(), bytes);
    assert_eq!(rot13("uryyb"), "hello");
    assert_eq!(caesar_cipher("abc", 3), "def");
}

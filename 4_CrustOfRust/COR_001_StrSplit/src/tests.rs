use crate::StrSplit;

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let hastack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(hastack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}

#[test]
fn multi_character_delimiter() {
    println!("TEST: MultiChar");
    let haystack = "abcXXdefXXghix";
    let letters: Vec<_> = StrSplit::new(haystack, "XX").collect();
    assert_eq!(letters, vec!["abc", "def", "ghix"]);
}

#[test]
fn unicode_test() {
    let s = "こんにちは 世界";
    let result: Vec<_> = StrSplit::new(s, ' ').collect();
    assert_eq!(result, vec!["こんにちは", "世界"]);
}

#[test]
fn emoji_test() {
    let s = "hello 😀 world";
    let result: Vec<_> = StrSplit::new(s, ' ').collect();
    assert_eq!(result, vec!["hello", "😀", "world"]);
}
#[test]
fn char_indices_demo() {
    let s = "a😀世";
    for (i, c) in s.char_indices() {
        println!("index: {}, char: {}", i, c);
    }
}

#[test]
#[should_panic]
fn break_utf8() {
    // INTENTIONAL PANIC.
    //
    // "न" is 3 bytes.
    //
    // slicing 0..1 breaks UTF-8 boundary.
    // Rust protects us at runtime.

    let s = "こ";
    let _x = &s[0..1];
}

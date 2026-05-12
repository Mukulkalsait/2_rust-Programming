use crate::strsplit::StrSplit;

/// Helper Function Instead of writing:
/// StrSplit::new(s, c).next() every time,
/// we made helper API: until_char(s, c) This returns ONLY first split piece.
pub fn until_char(s: &str, c: char) -> &'_ str {
    StrSplit::new(s, c).next().expect("StrSplit always gives alteast 1 resualt")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell")
}

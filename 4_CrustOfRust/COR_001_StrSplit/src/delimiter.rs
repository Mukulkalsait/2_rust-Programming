/// Trait:
/// Delimiter have functin find_next
/// so Delimiter knows how to find next one.
///
/// sending in &self ans s: &str, and geting
/// - Starting -> usize
/// - Ending   -> usize
///
/// return:
/// Option<(usize, usize)>
/// => Some((start, end))
/// => None
pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

/// Implimentation for &str
impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        // s.find(self)
        // Example:
        // "a b c".find(" ") => Some(1)
        //
        // map() here is Option::map()
        // NOT Iterator::map()
        //
        // Some(1)
        // => map(|start| (start, start + self.len()))
        // => Some((1,2))

        s.find(self).map(|start| (start, start + self.len()))
    }
}

/// Implimentation for char
impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        // char_indices()
        // returns:
        // (byte_index, char)
        //
        // Example:
        // "नम" =>
        // (0, 'न')
        // (3, 'म')
        //
        // UTF-8 chars can take multiple bytes.
        // Thats why normal indexing is dangerous.

        s.char_indices()
            .find(|(i, c)| {
                println!("indices:{}", i);
                c == self
            })
            .map(|(start, _)| {
                // len_utf8() because char can be multiple bytes.
                // 'o' => 1 byte
                // 'न' => 3 bytes
                (start, start + self.len_utf8())
            })
    }
}

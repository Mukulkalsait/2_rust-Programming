use crate::delimiter::Delimiter;

/// NOTE: TO TRACK haystacks lifetime we used  # 'hay_s
#[derive(Debug)]
/// STRUCT: concept: we are building .split() function which rust already have "a b c".split(" ").
/// -------------------------------------
///   Haystack => Ghas ka dher.
///   Needle => Delimiter.
///   Ghas ke dher me suei dhundna.
/// -------------------------------------
///
/// 1. struct with lifetiem with long name <'a> and <'haystack> are same
/// - String Slice live atleast as long aas 'haystack2'
/// 2. D -> Generic D => Delimiter
/// - can be => &str, char or anyting implimenting Delimiter.
///
/// Impl:
/// # Iterator Implimentation.
///
/// ```ignore
/// impl<'hay_s, D> Iterator for StrSplit<'hay_s, D>
/// where
///     D: Delimiter,
/// ```
/// => this litrally meen , D is a Generic type so
/// if D is Delimiter => then => StrSplit can use Iterator the way we define.
/// ## HOW THIS WORKS:
/// if we do
/// ```ignore
/// impl Delimiter for char {...}
/// ```
/// IMP:
/// ***"THIS IS HOW THE D type Becomes DELIMITER"***
/// ***"which mean char can impliment trait Delimiter"***
/// TODO:
/// - impliment rsplit()
/// - print pointers with {:p}
/// - try intentionally breaking UTF-8 slicing
/// - understand closures deeply
/// - compare with std::str::Split
///
pub struct StrSplit<'hay_s, D> {
    remainder: Option<&'hay_s str>, // this can be None
    delimiter: D,
}

impl<'hay_s, D> StrSplit<'hay_s, D> {
    /// Struct Constructor Function:
    /// - create the struct from data.
    /// - this data can be driectly used in Iterator.
    /// - create state for Iterator.
    pub fn new(haystack: &'hay_s str, delimiter: D) -> Self {
        // reminder can be none hence Some()
        Self { remainder: Some(haystack), delimiter }
    }
}

impl<'hay_s, D> Iterator for StrSplit<'hay_s, D>
where
    D: Delimiter,
{
    type Item = &'hay_s str; // Associated Type Defnie

    /// Next ==> return Self::Item => &'hay_s str.
    /// remainder => find delimiter => return before delimiter => update remainder => repeat
    fn next(&mut self) -> Option<Self::Item> {
        println!("Next called ------------------");

        // self.remainder is: Option<&str>
        // as_mut() converts it roughly into: Option<&mut &str>
        // ? means: if None => return None immediately
        let remainder = self.remainder.as_mut()?;
        println!("Current remainder: {:?}, PointerAddr : {:?}", remainder, remainder.as_ptr()); // pointer print to observe slicing memory movement

        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            println!("Delimiters[start:{}, end: {}]", delim_start, delim_end);

            let until_delimiter = &remainder[..delim_start];
            println!("Returning until_delimiter : {}", until_delimiter);
            *remainder = &remainder[delim_end..]; // MAGIC LINE: THIS moves iterator forward.
            println!("New remainder: {},PointerAddr : {:?}", remainder, remainder.as_ptr());
            Some(until_delimiter) // can also return empty string.
        } else {
            //NO DELIMITER FOUND. BUT final piece left.         Y: 1.
            // Example: "a b c" after processing: remainder = "c"
            // find_next() => None we still need to return "c"
            println!("No Delimiter found. **Taking final Reminder** ");
            self.remainder.take()
        }
    }
}

///  FUN_2: Explanation of scaller data type UNICODE CHAR. */
pub fn scaller_characters() {
    println!( "---------------------------------------------------\nA.4: Function: scaller_characters() === > \n
     IMP:  4. Characters
     ===========================
     |        Characters:       |
     |--------------------------|
     |    unicode characters    |
     | allways in SINGLE COTE ''|
     | ' ' is allways = UNICODE |
     |==========================| ");

    println!(
        "a length of CHAR in RUST is allways = {} Bytes, or 32bit.",
        std::mem::size_of::<char>()
    );

    let k = 'z';
    let l = 'Z'; //captial "Z"
    let heart_eyed_cat = '😻'; // Default UTF8 characters
    println!( "Deu to the Unicode Nature of characters. The 4 bytes can contian anything that comes under unincodes. including emojis. see k: {} l: {} heart_eyed_cat: {} ", k, l, heart_eyed_cat)
}

const TWO: u32 = 1 + 1;

fn main() {
    println!("{TWO}");
    /* Y: 2 main data types:
     * A. Scaller Datatypes.
     * B. Compound Datatypes.
     */

    /* G: A.Scaller
     *  ---------------
     *  Intiger
     *  Floatpoint number
     *  Boolean
     *  Charecter
     *
     */

    /* B:
     * INT:
     * -----------------------------------
     * \           \          \         \\
     * \  length   \  signed  \ Unsined \\
     * \           \          \         \\
     * -----------------------------------
     * \\  8-bit   \   i8     \   u8    \\
     * \\  16-bit  \   i16    \   u16   \\
     * \\  32-bit  \   i32    \   u32   \\
     * \\  64-bit  \   i64    \   u64   \\
     * \\  128-bit \   i128   \   u128  \\
     * \\   ARCH   \   i16    \   u16   \\
     * ===================================
     *
     * */

    let a = 98_324;
    let b = 0xff;
    let c = 0o77;
    let d = 0b111_0000;
    let e = b'A';
    let f = 255;

    println!("{ a }", "{ b }", "{ c }", "{ d }", " { e }", "{ f }");
}

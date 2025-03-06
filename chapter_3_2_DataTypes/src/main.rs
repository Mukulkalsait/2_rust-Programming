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
     * \           \   +/-    \    +    \\
     * -----------------------------------
     * \\  8-bit   \   i8     \   u8    \\
     * \\  16-bit  \   i16    \   u16   \\
     * \\  32-bit  \   i32    \   u32   \\
     * \\  64-bit  \   i64    \   u64   \\
     * \\  128-bit \   i128   \   u128  \\
     * \\----------\--------------------\\
     * \\   ARCH   \architecture depend \\
     * ===================================
     *
     * */

    // B: ARCHITECTURE = mostyle (32/64)

    let a = 98_324; // Decimal
    let b = 0xfffa; // Hex
    let c = 0o7735; // Octal
    let d = 0b1111; // Binery
    let e = b'A'; // Bite U8 only.
    let f: u8 = 255; // G: intiger oferflow 👇

    /* B: Intiger Overflow:
     *  \--------------------------------------------------------\
     *  \f:u8 = 255 is ( 8 bit unsigned intiger) maxvalue = 255  \
     *  \--------------------------------------------------------\
     *  \                                                        \
     *  \       assigingin it higher value:                      \
     *  \    A. while DEBUG: rust will panic---                  \
     *  \    B. while RELEASE: rust will do wrapping....         \
     *  \        if biger then   255 then                        \
     *  \                                                        \
     *  \--------------------------------------------------------\
     * */

    let g = 2.0; //  G: float default f64
    let h: f32 = 3.0; //  forcefully assgined f32

    // println!("{ a }", "{ b }", "{ c }", "{ d }", " { e }", "{ f }");  // dont know if it will run or not.
}

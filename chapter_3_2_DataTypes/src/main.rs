use colored::*;
use std::{io, u32};

fn main() {
    println!("We have 2 main data types .... \n\n\n");
    println!(
        "------------------------------------------------------\n
        1. Scaller DT   => BIFC
        2. Coumpound DT => TAF

        |====================|
        |  😻 Scaller Types. |
        |--------------------|
        |  Intiger           |
        |  Floatpoint number |
        |  Boolean           |
        |  Charecter         |
        |====================|
        |  😻 Compound Types.|
        |--------------------|
        |  Tupil             |
        |  Array             |
        |  Function          |
        |____________________|

        "
    );

    /* IMP:
     * 1. INT:
     * ---------------------------------------------------
     * \           \          \         \\               \
     * \  length   \  signed  \ Unsined \\     Float     \
     * \           \   +/-    \    +    \\               \
     * ---------------------------------------------------
     * \\  8-bit   \   i8     \   u8    \\    i8         \
     * \\  16-bit  \   i16    \   u16   \\    i16        \
     * \\  32-bit  \   i32    \   u32   \\    i32        \
     * \\  64-bit  \   i64    \   u64   \\    i64        \
     * \\  128-bit \   i128   \   u128  \\    i128       \
     * \\----------\--------------------\\---------------\
     * \\   ARCH   \           architecture depend       \
     * ===================================================
     *
     * */
    // B: ARCHITECTURE = mostyle (32/64)

    let a = 98_324; // Decimal
    let b = 0xfffa; // Hex
    let c = 0o7735; // Octal
    let d = 0b1111; // Binery
    let e = b'A'; // Bite U8 only.
    let f: u8 = 255; // G: intiger oferflow 👇
                     //
    println!(
        " so here 
        values of the following : \n

        INt: \n
        |==========================|
            A:{}|=>Decimal
            B:{}|=>HeX
            C:{}|=>Octal
            D:{}|=>Binery
            E:{}|=>Bite U8
            F:{}|=>intiger Overflow
        ",
        a, b, c, d, e, f
    );

    /* B: Intiger Overflow:
     *  \--------------------------------------------------------\
     *  \f:u8 = 255 is ( 8 bit unsigned intiger) maxvalue = 255  \
     *  \--------------------------------------------------------\
     *  \                                                        \
     *  \       assigingin it higher value:                      \
     *  \    A. while DEBUG: rust will panic---                  \
     *  \    B. while RELEASE: rust will do wrapping....         \
     *  \        if biger then num/255 reminder= assign.         \
     *  \                                                        \
     *  \--------------------------------------------------------\
     * */

    // IMP:  2. FLOAT:
    let g = 2.0; //  G: float default f64 IMP"f32= float32"
    let h: f32 = 3.0; //  forcefully assgined f32

    println!(
        "
        Float: \n
        |==========================|
            G:{}|=>Default Float
            H:{}|=>force Aggisned
             ",
        g, h
    );

    // IMP:  3. Boolean:
    // Can be set or unset.

    let i = true; // un-set
    let j: bool = false; // B:set we forcerully set a bull here

    println!(
        " the bulls are BULLS in RUST.
        I : {}
        J : {}
        ",
        i, j
    );

    /* IMP:  4. Characters
     * ===========================
     * \        Characters:       \
     * \--------------------------\
     * \    unicode characters    \
     * \ allways in SINGLE COTE ''\
     * \==========================\
     */

    let k = 'z';
    let l = 'Z'; //captial "Z"
    let heart_eyed_cat = '😻';

    println!(
        "Charactors are specials. 
So 

 1. K: {}
 2. L: {}
 3. HearteEyeCat: {}
",
        k, l, heart_eyed_cat
    );
}

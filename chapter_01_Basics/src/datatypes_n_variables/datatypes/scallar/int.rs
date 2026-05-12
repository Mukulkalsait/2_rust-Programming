///  FUN_2: Explanation of scaller data type INT. */
pub fn scaller_int() {
    println!( "---------------------------------------------------\nA.1: Function: scaller_int() === > \n 
      1. INT:
      ----------------------------------------------------
      ||          |          |         ||                |
      ||  length  |  signed  | Unsined ||     Float      |
      ||          |   +/-    |    +    || allways singed |
      ||-------------------------------------------------|
      ||  8-bit   |   i8     |   u8    ||    ---         |
      ||  16-bit  |   i16    |   u16   ||    ---         |
      ||  32-bit  |   i32    |   u32   ||    f32         |
      ||  64-bit  |   i64    |   u64   ||    f64         |
      ||  128-bit |   i128   |   u128  ||    f128        |
      ||----------|--------------------||----------------|
      ||   ARCH   |           architecture depend        |
      ===================================================
     
      B: ARCHITECTURE = mostyle (32/64) ");

    let a = 98_324; // Decimal
    let b = 0xfffa; // Hex
    let c = 0o7735; // Octal
    let _d = 0b1111; // Binery
    let _e = b'A'; // Bite U8 only.
    let f: u8 = 255; // G: intiger oferflow 👇

    println!(
        "a:{}\nb:{}\nc:{}\n=>( The valuse of d and e are not printed cos we used _d and _e to tell compailer that we are not using it and we are doing it on purpose.) \nf:{}",
        a, b, c, f
    );

    println!(
        "
     B: signed vs unisgned :
     bits   = X 0 1 0 1 0 => here X is signe
     signed = _ _ _ _ _ _
     Y:
     therefore : signed variables are from -128 to 127 ( one place for sign.)
     BUT IN ANY CASE => total avialable numbers will be same
     eg. u6 => 64 values vs i6 => (-32 to 31): 64 values 
     hence no matter if i or u total values is same just if we
     use :  Signed => range shift to nigitive to positive.
            uSigned => range start from 0 onwards.


     unsigned has Intiger OverFlow
     B: Intiger Overflow:
      |--------------------------------------------------------|
      |f:u8 = 255 is ( 8 bit unsigned intiger) maxvalue = 255  |
      |--------------------------------------------------------|
      |                                                        |
      |       assigingin it higher value:                      |
      |    A. while DEBUG: rust will panic---                  |
      |    B. while RELEASE: rust will do wrapping....         |
      |       if biger then num/256 reminder= assign.          |
      |       256=length here.                                 |
      |--------------------------------------------------------| "
    )
}

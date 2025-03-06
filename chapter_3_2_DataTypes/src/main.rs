const TWO: u32 = 1 + 1;

fn main() {
    println!("{TWO}");
    /* Y: 2 main data types:
     * A. Scaller Datatypes.
     * B. Compound Datatypes.
     */

    //DX:======== A Scaller ==========================================================================================================================================

    /* G: Scaller Types.
     *  ---------------
     *  Intiger
     *  Floatpoint number
     *  Boolean
     *  Charecter
     *
     */

    /* IMP:
     * 1. INT:
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

    // IMP:  2. FLOAT:
    let g = 2.0; //  G: float default f64
    let h: f32 = 3.0; //  forcefully assgined f32

    // IMP:  3. Boolean:
    // cna be set or unset.

    let i = true; // un-set
    let j: bool = false; // set

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

    //DX:=========== B. Compuoun Data Type =======================================================================================================================================

    /* Y: B. Compound DT
     *  ->
     *  type that represent a group of values  is "Compound data type".
     *   ----------------------------------------
     *    A. tup ( tupil)
     *   B. array
     *    C. fu
     *
     *
     *
     * */

    /* IMP: TUPIL
     *  1. 1+values = tupil.
     *  2. automatic type known.
     *  3. can be difined.
     *
     *
     */
    let tup1 = ("Lets get Rusty !!!", 1000_000);
    let tup2 = ("Lets get Rusty !!!", 1000_000, "i am third so am i Triple");
    let tup3 = ("Lets get Rusty !!!", 1000_000, "i am third so am i Triple");

    let (chanel_name, subcriber_count) = tup1; // B: 1 ===>
    let (a, b, c) = tup2; // this totally worked. + automatic assign.

    let subcount1 = tup3.0; // B: 2 ===>
    let subcount2 = tup3.1;
    let subcount3 = tup3.2;

    /*B:
     * 1 --  tupil De-stracturing
     * 2 --  tupil Dot-Notatioon.
     * */


    /* IMP: Array: 
    *===============================
    *  1. fixed Length (otherWise use VECTOR)
    *  let arrName = [ a , 'b', 23 , javan]  == array. 
    *  let arrayNm = [0;8] in this line we are making array of "8" 
    *  value whereas all 8 numbers are  "0";
    * */


    let error_code_array = [200, 404, 500];
    let nof_found = error_code_array[1] // = 404
    //



 // my_first_ever_rust_fun(); // this will run the function.
}



fn my_first_ever_rust_fun(){ 
println!(" the function si working.")
    
    /* B: fun can have arguments -> multyple
    * */

    /* B:  fun cna be a statemet either STATEMENT  or EXPRESSTION.
    *   a statement = no value return.
    *   b excersize =>a return value.
    *
    * */
}

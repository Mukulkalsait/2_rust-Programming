const TWO: u32 = 1 + 1;

fn main() {
    println!("{TWO}");

    println!(" ===== Data Types 2(S-C)=====");
    println!("-----> A. Scaller DT  BIFC \n");
    println!("-----> A. Compound DT TAF\n");

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

    // IMP:  3. Boolean:
    // Can be set or unset.

    let i = true; // un-set
    let j: bool = false; // B:set we forcerully set a bull here

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

    /* Y: B. Compound DT -> TAF
     *  ->
     *  type that represent a group of values  is "Compound data type".
     *   ----------------------------------------
     *    A. tup ( tupil)
     *    B. array
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
    let tup3 = (
        "Lets get Rusty !!!",
        1000_000,
        true,
        "so am  i a quadrapil now ?",
    );

    /*B: geting values from -> TUPIL
     *   --------------------------------
     *  1: tupil De-stracturing
     *    we are going to take all the valuse
     *    in a single go with variable for every
     *    value in tupil
     *  ---------------------------------
     *  2: tupil Dot-Notatioon.
     *    we will only take the value we
     *    needed and use it.
     * */

    let (chanel_name, subcriber_count) = tup1; // B: 1: De-stracturing
    let (a, b, c) = tup2; // this totally worked. + automatic assign.

    let subcount1 = tup3.0; // B: 2: Dot-Notatioon
    let subcount2 = tup3.1;
    let subcount3 = tup3.2;

    /* IMP: Array:
     *===============================
     *  1. fixed Length (otherWise use VECTOR)
     *  let arrName = [ a , 'b', 23 , javan]  ==> array.
     *  let arrayNm = [0;8] in this line we are making array of "8"
     *  value whereas all 8 numbers are  "0";
     * */

    let error_code_array = [200, 404, 500];
    let nof_found = error_code_array[1]; // = 404
    let array_of_8_zeros = [0, 8];

    // my_first_ever_rust_fun(); // this will run the function.

}

fn my_first_ever_rust_fun() {
    println!(" the function is not working.");

    /* B: FN
     *   1. Can have arguments -> multyple just like normal funciotns in any languages
     *   2. Same num of arguments must be passed while calling.
     *
     * G: any code in RUST is --> STATEMENT or EXPRESSTION.
     *   a. statement => perform some action but do not return a value.
     *   b. expression => perform some action + return a value.
     *   EG.
     * */

    println!("a statement. "); // Y: is a statement
    let sum = 3 + 9; // Y: expression
    return sum; // this is giving error  because we didnt set return type to the funtion.
                // IMP: the proper Way is
                // ||----------------------------\\
                // || fn function_name() -> i32{ \\
                // || return sum                 \\
                // || }                          \\
                // ||----------------------------\\
}

fn the_real_return_function(x: i32, y: i32) -> i32 {
    let sum = x * y;
    return sum;
}
fn another_way_to_return(x: i32, y: i32) -> i32 {
    let sum = x * y;
    // Y: no need of ';' semicolon or return billow.
    sum
}


//Y: The billow line should be inside the main fucntion or any another funciton. "👇👇👇👇👇"
let sum_no_23 = yet_another_way_to_return(23, 23);

fn yet_another_way_to_return(x: i32, y: i32) -> i32 {

    x * y    // Y: no need of ';' semicolon or return billow.
    
}


fn if_else_understanding(){ 
    /**
    *  Y:: if-elseif-else
    *  1. conduction must be EXPLICETYL "BULLEAN".  
    *  2.hence -> if (number) {...}  // will not work (number is not bull)
    */


    let number =23;
if number < 50 {
    println!("num is small");
}else if number = 50 { 
        println!("number is equal !!");
}else {
    println!("Number is alleready greter.");
}

    /**
    *  Y:: if-elseif-else = EXPLICETYL "BULLEAN".  
    *  eg.
    */

    let conduction = true;
    let number = if conduction{ 5 } else { 6 };

}


fn loops_in_rust() {

    // Y: 1. a loop without stop.
    loop {
        println!("again ! ctrl + c to stop.");
            break;
    }


    // Y: 2. A Propper loop.
    let mut counter = 10; 

    let return_value = loop {
        println!("This is the loop You are waiting for {}",counter);
        if(counter <= 0){break counter;} // G: here we are RETURNING  "counter";
        counter -=1;
    };

    // Y: 3. While loop.

    let mut number = 3;

    while number !=0  {
        println!("{}!", number);
       number -=1; 
    };
    println!(" This line will only print if the loop is ended.");



}

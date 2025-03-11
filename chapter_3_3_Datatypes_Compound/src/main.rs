use std::array;

fn main() {
    tupil_creation(); //  G: 1.Tupil
    array_cretion(); //  G: 2.Array
    let return_of_main_funciton = function_1(); // G: fun1
    println!("{}", return_of_main_funciton);
    loops_in_rust();
}

/* INFO:  This Function is giving information on "TUPIL".
* |------------------------------------------------------|
*/
fn tupil_creation() {
    println!(
        " 
                   Compound DT -> TAF
           ---------------------------------------
           |  It represents a group of values.   |
           ---------------------------------------
             A. tup ( tupil)
             B. array
             C. fu
           |-------------------------------------|
        "
    );

    // IMP: TUPIL >

    let power_tupil = (
        "Lets get Rusty !!!",
        1_000_000,
        true,
        "so am i a quadrapil now ?",
        "who the fuck cares 👇",
        true,
        "dont trust anyone.",
        32.42342,
    );

    /*B: geting values from -> TUPIL
     *  ---------------------------------
     *  1: tupil Dot-Notatioon.
     *    we will only take the value we
     *    needed and use it.
     *   --------------------------------
     *  2: tupil De-stracturing
     *    we are going to take all the valuse
     *    in a single go with variable for every
     *    value in tupil
     * */

    println!(
        " 
                1. Dot-Notatioon: 🩷 power_tupil :

                |==========================|
                      power_tupil[0]:{}|
                      power_tupil[1]:{}|
                      power_tupil[2]:{}|
                      power_tupil[3]:{}|
                      power_tupil[4]:{}|
                      power_tupil[5]:{}|
                      power_tupil[6]:{}|
                      power_tupil[7]:{}|
            ",
        power_tupil.0,
        power_tupil.1,
        power_tupil.2,
        power_tupil.3,
        power_tupil.4,
        power_tupil.5,
        power_tupil.6,
        power_tupil.7,
    );

    let (a0, a1, a2, a3, a4, a5, a6, a7) = power_tupil; //NOTE: The number of variables must match the number of tupil valulse.

    println!(
        " 
                2. De-stracturing : 💙 power_tupil :

                |==========================|
                      a0:{}|
                      a1:{}|
                      a2:{}|
                      a3:{}|
                      a4:{}|
                      a5:{}|
                      a6:{}|
                      a7:{}|
            ",
        a0, a1, a2, a3, a4, a5, a6, a7
    );
}

/* INFO:  This Function is giving information on "ARRAY".
* |------------------------------------------------------|
*/
fn array_cretion() {
    /* IMP: Array:
     *===============================
     *  1. fixed Length (otherWise use VECTOR)
     *  let arrName = [ a , 'b', 23 , javan]  ==> array.
     *  let arrayNm = [0;8] in this line we are making array of "8"
     *  value whereas all 8 numbers are  "0";
     * */

    println!(
            " 
                1. ARRAY ===> Fixed Length 
                    (otherWise use VECTOR)

                2. Creation: 
                        a. let arrName = [ a , 'b', 23 , javan] 
                        
                        b. let arrayNm = [0;8] 
                            (in this line we are making array of \"8\" value whereas all 8 numbers are  \"0\")
                
                3. data types must match in array otherwise ERROR.
            ");

    let error_code_array = [200, 404, 500];
    let nof_found = error_code_array[1]; // = 404
    let array_of_8_zeros = [0, 8];

    println!(
        "  The Useual Way:
        error_code_array[0]: {}
        error_code_array[1]: {} 
        error_code_array[2]: {}",
        error_code_array[0], error_code_array[1], error_code_array[2],
    );
}

/* INFO:  This Function is giving information on "FUNCTION".
* |------------------------------------------------------|
*/
fn function_1() -> String {
    /* B: FN
     *   1. Can have arguments -> multyple just like normal funciotns in any languages
     *   2. Same num of arguments must be passed while calling.
     *
     * G: any code in RUST is --> STATEMENT or EXPRESSTION.
     *   a. statement => perform some action but do not return a value.
     *   b. expression => perform some action + return a value.
     *   EG.
     * */
    println!(" ");

    /* Y:  IN order to CONCATE Strings
     *      --------------------------
     *   atleast 1 string should be created from
     *       String::from("...");
     *   otherwise concate will not work.
     **/

    let steatement1 = String::from("This is the oupout of main function."); // IMP: String::from()
    let steatement2 = "We are returningi this to insure Function Part is End.";
    let steatement3 =
        "This is only to conform if we need Strfing::from() multyple time or just once.";
    let return_of_fun = steatement1 + steatement2 + steatement3;

    /* IMP: This is giving error  because we didnt set return type to the funtion.
     * the proper Way is
     * ||----------------------------\\
     * || fn function_name() -> i32{ \\
     * || return sum                 \\
     * || }                          \\
     * ||----------------------------\\
     */
    let sum_array = [23, 34, 45, 56, 67, 78];
    let resualt1 = return_function1(sum_array[0], sum_array[1]);
    let resualt2 = short_return2(sum_array[2], sum_array[3]);
    let resualt3 = another_function(sum_array[4], sum_array[5]);

    println!(
        " R1 : {} 
          R2 : {} ",
        resualt1, resualt2,
    );
    if_else_fun1(sum_array[3]);
    // return return_of_fun; // this will tell you G: Unneeded 'return' steatement ... | hence direct ->
    // Y: no need of ';' semicolon or return billow.
    return_of_fun
}

/*This type of return will give you worning.
* |------------------------------------------------------|
*      fn return_function1(x: i32, y: i32) -> i32 {
*          let sum = x * y;
*          sum
*      }
*      herce
*      👇👇👇👇👇👇👇👇👇👇👇👇👇👇
*/

/* Just a return fucntion = Multiply
*/
fn return_function1(x: u32, y: u32) -> u32 {
    x * y
}

/* Just a return fucntion = Addition
*/
fn short_return2(a: u32, b: u32) -> u32 {
    a + b
}

/** Array rutruning function
*   we are returningn + * / % and - or 2 u32 vars
*    in thi function in form of Array.
*   return type [u32; 5]
*/
fn another_function(a: u32, b: u32) -> [u32; 5] {
    [a + b, a * b, a / b, a % b, b - a] // here b is bigger than a so b-a
}

/** if-elseif-else
 *  1. conduction must be EXPLICETYL "BULLEAN".  
 *  2. hence -> if (number) {...}  // will not work (number is not bull)
 */
fn if_else_fun1(a: u32) {
    if a < 50 {
        println!("num is small");
    } else if a == 50 {
        println!("a : {} is equal !!", a);
    } else {
        println!("a : {} is alleready greter.", a);
    }

    let conduction = true;
    let num = if conduction { 5 } else { 6 };
    println!("Number = {}", num);
}

/* INFO: loops_in_rust
*
* */

fn loops_in_rust() {
    // B: 1.Normal loop
    let mut flag: u32 = 0;
    loop {
        println!(
            "WARNING: if we dont use BREAK; the loop will not stop ::: use -> ctrl + c to stop.(in case)"
        );
        if flag >= 5 {
            break;
        }
        flag += 1;
    }

    // B: 2. Loop.
    let mut counter = 10;
    let return_value = loop {
        println!("This is the loop You are waiting for {}", counter);
        if (counter <= 0) {
            break counter;
        } // G: here we are RETURNING  "counter";
        counter -= 1;
    };
    println!("Loop run till {}", return_value);

    // B: 3. While loop.
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!(" This line will only print if the loop is ended.");

    // Y: 4. For loop / For in loop.
    let arra_x = [12, 23, 34, 45, 56, 67, 78, 89, 90];
    for element in arra_x.iter() {
        println!("The value is {}", element);
    }

    // Y: 5. Special For loop.
    for number_x in 1..34 {
        // IMP: here only 1 to 33 numbers will be printed because last is not printed.
        println!("The value is {}", number_x);
    }
}

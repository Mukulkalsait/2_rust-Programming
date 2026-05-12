/// FUN_2: Explanation of COMPOUND types FUNCTIONS. */
///  B: FN
///  1. Can have arguments -> multyple just like normal funciotns in any languages
///  2. Same num of arguments must be passed while calling.                        
///
///  G: any code in RUST is --> STATEMENT or EXPRESSTION.            
///    a. statement => perform some action but do not return a value.
///    b. expression => perform some action + return a value.        
///    EG.                                                           
pub fn compound_function() {
    let multiply = returning_function(2, 268); // Y: EXPRESSTION

    println!(
        "The multiply of returning_function_two(x:i32,y:i23)->i32 is = {}",
        multiply
    ); // Y: is a statement
}

/// FUN_3: Explanation of function types. = 2
///  IMP: the proper Way is          
///  ||----------------------------||
///  || fn function_name() -> i32{ ||
///  || return sum                 ||
///  || }                          ||
///  ||----------------------------||
///  Y:
///  1. The function will give ERROR if [ -> i32 ] return type is not specified.             
///  2. No need of ';' semicolon or "return" in last sentence. ( but you can still use them.)
pub fn returning_function(x: i32, y: i32) -> i32 {
    x * y
}

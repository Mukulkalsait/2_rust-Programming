fn main() {
    let mut x = 4; //IMP: i can see variable with number is allready showing i32.
    println!("Val of x is : {}", x); // Y:This type of printing is STRING LITRAL.
    x = 5;
    println!("Val of x is : {}", x);

    const FIXED_NUMBER: u64 = 9156_123_645; // G: Yes we can seperate numbers with '_' Underscore.
    println!("This is the FIXED_NUMBER right here :{}", FIXED_NUMBER);

    // B:  CONST:
    // I have to tell const the u64 (here 64 because number is bigg
    // also const NAME SHOULD BE CAPITAL ALL.
    // also WE have to assign a value to constant when we create it we cannot assign a const 'RUNTIME Value'.

    let shadowing_of_variable = 3;
    println!(
        "we can shadow 're-declear' variables in RUST = {}",
        shadowing_of_variable
    ); // Y:This type of printing is STRING LITRAL.
    let shadowing_of_variable = "three";

    println!(
        "This time we can see the same var is pringting string value: {}",
        shadowing_of_variable
    ); // Y:This type of printing is STRING LITRAL.
    println!("BOTH time the variable 'shadowing_of_variable' was IMMUTABLE.");
}

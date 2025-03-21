fn main() {
    let mut x = 4; //IMP: i can see variable with number is allready showing i32.
    println!("Val of x Before is : {}", x); // Y:This type of printing is STRING LITRAL.
    x = 5;
    println!("Val of x After is : {}", x);
    println!("{x}");

    let shadowing_of_variable = 3;
    println!(
        "we can shadow 're-declear' variables in RUST = {}",
        shadowing_of_variable
    );

    /* DX: SHADOWING:
     *  -
     * We must use "LET" again in order to use shadowing. and the most imp thing is both vars will
     * still be IM-Mutable.
     *  Y: HERE WE SEE " SHADOWING DONT NEED SAME TYPE WE CAN CHANGE TYPE." ∴ it can be anythig.
     */

    let shadowing_of_variable = "three";

    println!(
        "This time we can see the same var is pringting string value: {}",
        shadowing_of_variable
    );
    println!("BOTH time the variable 'shadowing_of_variable' was IMMUTABLE.");

    let shadowing_of_variable = 23;
    /* INFO:
     *   Everytime we want to change it we are shadowing it "intentionally "
     *   ∴ if we dont want to let it change it will not change in any case...
     *   because we wont use "let".
     */
    let shadowing_of_variable = shadowing_of_variable * 23 * 23;
    println!(
        "-Here we did it 3rd time with little experiment .... so 'shadowing_of_variable' : {} \n-which is same as the cube of 23 : {}",
        shadowing_of_variable,
        23 * 23 * 23
    );
    /* B:  CONST: ======================================================================
     *  -
     *  I have to tell const the u64 (here 64 because number is big.
     *  also const NAME SHOULD BE CAPITAL ALL.
     *  also WE have to assign a value to constant when we create it we cannot assign a const 'RUNTIME Value'.
     *  const will be used throught program but let is only inside FUNCTION.
     */

    const FIXED_NUMBER: u64 = 9156_123_645; // G: Yes we can seperate numbers with '_' Underscore.
    println!("This is the FIXED_NUMBER right here :{}", FIXED_NUMBER);

    /* Y: const can be assgin with things like that but we cant add a value that is begin
     *  calcualted at the RUNTIME.
     */
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}"); // we can also print like this.
}

fn main() {
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
        1000_000,
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

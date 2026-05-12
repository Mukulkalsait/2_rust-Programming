pub mod datatypes;
use datatypes::{scallar, compound}; 

pub fn variables_basic_1_main(){
    println!("const can be access BEFORE DECLERATION:{TWO}"); // Y: const can be access BEFORE DECLERATION.

    println!(
        " ===== Data Types 2(S-C)=====
     _________________________________________
    | 2 Main: Scaller and Compound Datatypes. |
    |_________________________________________|

    ======== A Scaller ========
     Types.
     ---------------
     Intiger
     Floatpoint number
     Boolean
     Charecter

    ======== B COMPOUND ========
     Type that represent a group of values  is \"Compound data type\".
     ---------------
     A. tup ( tupil)
     B. array
     C. fu "
    );
    scallar::scaller();
    compound::compound_main();
}

const TWO: u32 = 1 + 1;



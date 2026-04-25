
pub mod com_tupil;
pub mod com_array;
pub mod com_function;

///  FUN_1: Explanation of COMPOUND data types started. */
/// Y: ======== A Compound ========                                  
///  type that represent a group of values  is "Compound data type". 
///  ----------------------------------------                       
///   A. tup ( tupil)                                               
///   B. array                                                      
///   C. fu                                                         
pub fn compound_main() {
    println!(" ===== COMPOUND (TAF)=====\n A. tup ( tupil)\n B. array\n C. function ");

    com_tupil::compound_tupil();
    com_array::compound_array();
    com_function::compound_function();
}





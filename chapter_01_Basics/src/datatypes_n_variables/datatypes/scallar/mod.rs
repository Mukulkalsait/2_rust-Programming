pub mod int;
pub mod float;
pub mod booleans;
pub mod char_unicode;


///  FUN_1: Explanation of scaller data type INT.
///   ======== A Scaller (IFBC) ======== 
///   Scaller Types. IFBC             
///   ---------------                 
///   Intiger                         
///   Floatpoint number               
///   Boolean                         
///   Charecter                       
pub fn scaller() {

    int::scaller_int(); 
    float::scaller_float();
    booleans::scaller_boolean();
    char_unicode::scaller_characters();
}


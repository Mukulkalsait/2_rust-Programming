

/// FUN_2: Explanation of COMPOUND types ARRAY. */
/// IMP: Array:                                                  
/// 1. fixed Length (otherWise use VECTOR)                      
///
/// let arrName = [ a , 'b', 23 , javan]  ==> array.            
/// let arrayNm = [0;8] in this line we are making array of "8" 
/// value whereas all 8 numbers are  "0";                       
pub fn compound_array() {
    println!(
        "---------------------------------------------------\nA-3 Function: Compound_Array() === > \n"
    );

    // Y: Creation Array:
    let error_code_array: [i32; 3] = [200, 404, 500];
    let array_two = [3; 5]; // Y: generated array of [3,3,3,3,3]

    // Y: Accessiong Array:
    println!(
        "error_code_array.indexValus.1 = {}\narray_two.index.3 = {}",
        error_code_array[1], array_two[3]
    );
}

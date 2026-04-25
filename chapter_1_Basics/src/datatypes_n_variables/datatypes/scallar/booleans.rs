

///  FUN_2: Explanation of scaller data type BOOLEAN. */
pub fn scaller_boolean() {
    println!(
        "---------------------------------------------------
     A.3: Function: scaller_boolean() === >

     IMP:  3. Boolean:
     - Can be unset or Forcefully-set.
     - bool allways takes up 1 BYTE ( 8 bit ) Space. 

     IMP : we are INTENTIONALLY not USING i and j so i = _i and j = _j. to stop warning. "
    );

    let _i = true; // un-set
    let _j: bool = false; // B:set we forcerully set a bull here
    let bool_a : bool = false;
    println!("bool_a value:{}",bool_a);
}

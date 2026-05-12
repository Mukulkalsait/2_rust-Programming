/// FUN_2: Explanation of COMPOUND types TUPIL. */
///
/// IMP: TUPIL                                             
///  1. 1 + Values = tupil.                                
///  2. Type can be EXPLICIT or UNSPECIFIED and DIFFERENT.
///
/// B: geting values from -> TUPIL                         
///   --------------------------------                     
///### 1: tupil De-stracturing                               
///    we are going to take all the valuse                 
///    in a single go with variable for every              
///    value in tupil                                      
///  ---------------------------------                     
///### 2: tupil Dot-Notatioon.                               
///    we will only take the value we                      
///    needed and use it.                                  
pub fn compound_tupil() {
    println!(
        "---------------------------------------------------\nA-2 .4: Function: Compound_tupil() === > \n"
    );

    let tup1: (&'static str, u32) = ("Lets get Rusty !!!", 1000_000); // Y: digits can be writter with "_" between Underscores
    let tup2: (&'static str, u32, &'static str) =
        ("Lets get Rusty !!!", 1000000, "i am third so am i Triple ?");
    let tup3: (String, u32, bool, &'static str) = (
        String::from("Lets get Rusty !!!"),
        1000000,
        true,
        "so am  i a quadrapil now ?",
    ); // G: we can use String::from("string")

    geting_valuse_from_tupils(tup1, tup2, tup3);
}

/// FUN_3: Tupils extension. = 1 */
fn geting_valuse_from_tupils(
    tup1: (&'static str, u32),
    tup2: (&'static str, u32, &'static str),
    tup3: (String, u32, bool, &'static str),
) {
    println!("-------------------------\n fn geting_valuse_from_tupils() === > \n");

    // B: 1: De-stracturing
    let (chanel_name, subcriber_count) = tup1;
    println!(
        "Chanel Name ={}\n Subcribers Count ={}",
        chanel_name, subcriber_count
    );
    let (a, b, c) = tup2; // this totally worked. + automatic assign.
    println!("values for tup2 are = a:{}\n b:{}\n c:{}", a, b, c);

    // B: 2: Dot-Notatioon
    let subcount1 = tup3.0;
    let subcount2 = tup3.1;
    let subcount3 = tup3.2;

    println!(
        "subcribers 1:{}\n2:{}\n3:{}",
        subcount1, subcount2, subcount3
    );
}

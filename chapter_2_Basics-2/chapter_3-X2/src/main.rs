fn main() {
    println!(
        "---------------------------------------------------\n MAIN() :Start\n---------------------------------------------------"
    );
    if_else_understanding();
    array_understandign_and_usages();
}

/* FUN_1:*/
fn if_else_understanding() {
    println!(
        "---------------------------------------------------\n A. Fu if_else_understanding() => | if-elseif-else |  \n"
    );
    /* comment
     *  Y:: if-elseif-else
     *  1. Conduction must be EXPLICETYL "BULLEAN".
     *  2. hence -> if (number) {...}  // will not work (number is not bull)
     *  3. "()" are not necessary unless too complex expression.
     */

    // Y: 1. EXPLICETYL BULLEAN:
    let conduction = true;
    let number: u32 = if conduction { 32 } else { 64 }; // Y:  U32 will make sure its NonNegagive.

    // Y: 2 & 3 :
    if number < 50 {
        println!("num is small");
    } else if number == 50 {
        println!("number is equal !!");
    } else {
        println!("Number is alleready greter.");
    }
}

/* FUN_1:*/
fn array_understandign_and_usages() {
    println!(
        "---------------------------------------------------\n B. Fu array_understandign_and_usages() => | All About Arrays. |  \n"
    );

    /* IMP: Array:
     *===============================
     *  1. fixed Length (otherWise use VECTOR)
     *  let arrName = [ a , 'b', 23 , javan]  ==> array.
     *  let arrayNm = [0;8] in this line we are making array of "8"
     *  value whereas all 8 numbers are  "0";
     * */
    let all_arrays = creation_of_arrays();
    accessgin_array(all_arrays);
}

/* FUN_2:*/
fn creation_of_arrays() -> [[&'static str; 3]; 3] {
    println!("-------\n B. Fu creation_of_arrays() => | create 3 arrays and return |  \n");
    /* DX: ------------------------ VERY VERY IMPORTANT: ------------------------
     * G:
     * 1. ALL ELEMENTS of array must be of SAME TYPE.
     * 2. All Elements of ALL ARRAYS in Multy Dimentional Array must be of SAME TYPE.
     * 3. NUM of ELEMENTS in All Arraysy in Multy Dimentional Arrays should be SAME.
     * Y: -----------------
     * 4. return type in multy dimentional array =>
     *    a. [num,5] = array of 5 numbers.
     *    b. [[&'static str; 3]; 3]  = 3 arrays, each of 3 strings.
     */

    let array_one = ["Dell Inspiron", "intel pentium 3rd gen", "Intigerated gpu"];
    let array_two = [
        "Acer Predator Helios Neo 16 2024",
        "Intel i5 13500hx 13th gen",
        "Nvidia RTX 4050 6gb gddr v6",
    ];
    let array_auto = ["abcd"; 3];

    /* IMP: RETURNING an ARRAY-OF-ARRAYS without VAR
     * DX: instead:
     * ----------------------------
     * let array_of_all = [array_one, array_two, array_auto];
     * array_of_all
     * ----------------------------
     * Y: just
     * [array_one, array_two, array_auto]
     */

    // B: return =>
    [array_one, array_two, array_auto]
}

/* FUN_2:*/
fn accessgin_array(all_arrays: [[&'static str; 3]; 3]) {
    println!("-------\n B. Fu accessgin_array() => | access array and do something. |  \n");

    /* Y: Ways to Assess Arrays:
     * --------------------------------
     *  1. Access By Index Value.
     *   2. .get() [safe access + Error Handling].
     */

    indexing_array(all_arrays);
    indexing_array_with_if_else(all_arrays);
    get_method_n_error_handling(all_arrays);
    for_loop_in_array(all_arrays);
    iter_method_for_array(all_arrays);
    index_n_value_with_iter_n_enumerate(all_arrays);
    new_mutable_array_and_accessing(); // Y: array inside.
}

/* P1a: */
fn indexing_array(all_arrays: [[&'static str; 3]; 3]) {
    println!(
        "------------------------
IndexingDynamicArrya : 
1. {} -> {}
2. {} -> {}
",
        all_arrays[0][0], all_arrays[0][1], all_arrays[1][0], all_arrays[1][1]
    );
    // here we are printing {computer name } -> { computer processor}
    // which are {array value 0} -> { array value 1}
}

/* P1a: */
fn indexing_array_with_if_else(all_arrays: [[&'static str; 3]; 3]) {
    if let Some(mno_array) = all_arrays.get(1) {
        if let Some(pqr_array) = mno_array.get(0) {
            println!("Value is : {}", pqr_array);
        }
    }
    if let Some(abce_array) = all_arrays.get(2) {
        if let Some(efgh_array) = abce_array.get(2) {
            println!("the value of this array is : {}", efgh_array);
        }
    }
    if let Some(randome_array) = all_arrays.get(0) {
        for n in randome_array {
            println!("n - {:?}", n)
        }
    }
}

/* P1a: */
fn get_method_n_error_handling(all_arrays: [[&'static str; 3]; 3]) {
    match all_arrays.get(1) {
        Some(value) => println!("Values of index 1 are {:?}", value),
        None => println!("Some Error Occurs :"),
    }
}

/* P1a: */
fn for_loop_in_array(all_arrays: [[&'static str; 3]; 3]) {
    println!(
        "-------\n  Fu for_loop_in_array() => | For Loop In Arrays. 2D array is pringing... |  \n"
    );
    for n in all_arrays {
        println!("X - {:?}", n);
    }
}

/* P1a: */
fn iter_method_for_array(all_arrays: [[&'static str; 3]; 3]) {
    for n in all_arrays.iter() {
        println!("v.iter() = {:?}", n);
    }
}

/* P1a: */
fn index_n_value_with_iter_n_enumerate(all_arrays: [[&'static str; 3]; 3]) {
    for (index, value) in all_arrays.iter().enumerate() {
        println!("Index = {:?} -> Value = {:?}", index, value);
    }
}

/* P1a: */
fn new_mutable_array_and_accessing() {
    let mut nums = [1, 2, 3];
    nums[0] = 10; // B: replace
    nums[1] += 5; // B: 2 + 5 added = 7
    println!("{:?}", nums); // [10, 7, 3]
}

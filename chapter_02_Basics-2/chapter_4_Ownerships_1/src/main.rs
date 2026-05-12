fn main() {
    println!(
        " 🖳🖳🖳🖳🖳🖳🖳🖳   chapter_4_Ownership :    🖳🖳🖳🖳🖳🖳🖳🖳
=============================================================================================== "
    );

    memory_management(); //               INFO: Memory Management in different Languages.
    memory_info(); //                      IMP: VERY IMP MEMORY WORKAEOUND IN PROGRAMS
    ownership_transfer_vs_copy_trait(); //   INFO: Real EXample 1:
    golden_rules(); //                     IMP: VERY IMP INFO ON HEAP AND STACK
    ownership_working_methods(); //        IMP: This contain some examples to show owherships.
    reference_and_uses(); //               IMP: This is how we use References.
}

/* FUN: Memoryo Management in Java/C/C++/Rust
 **/
fn memory_management() {
    println!( "\n\n\n
Memory management types: 
------------------------------------------------------------------------------------------------
1. Garbage Collection => No Err + Easy/Fast Write | Large + Slow run + No Control.
   java | c# and all modern lagn.
  NOTE: garbage collectore automatically cleans its memory afer some time. everyting loads again for 1st time.

2. Memory management  => too many Error + Hard/Slow Write | Small + Fast run + Full Controll.
    C | C++ 

3. Ownership Model    => No Err* + someWhat Hard and Slowest write | Small + Fast run + vary much Controll (memory safe so its automatic).
    Rust = compail time memory check feature. (with 'unsafe' we can overwrite this.) Borrow checker.
=============================================================================================== \n\n\n
" );
}

/* FUN: Use of memory while Runnign "Programs".
* Stack vs Heap + explanations stracture.*/
fn memory_info() {
    println!( "
    /*  INFO: Memroy IN PROGRAM
     *
     *     B: 1. Heap                // FREE PULL
     *     G: 2. Stack               // Fixed Sized Memory + Functions ( Variables, Arguments, Returns and Returniung Places)
     *     Y: 3. Static / Global     // Global Variables
     *     R: 4. Code                // Code Writen
     *
     *  DX:
     *  while program is compaliling  2,3,4 are calcualted.
     *  while program is runnigng the stack data is changing {{  not Size  }} while code and static/global stays.
     *  while Fucntion is running its stack frame is changing after fun die  stack frame die.
     *  Y:
     *  1. Stack = Call by Value. = value is in stack.
     *  2. Heap = Call by reference = valuse is in Heap but pointer is in stack.
     *  3. In C/C++ we allocate the memory from Heap with 'maloc/new' and dealocate with 'free/delete'
     *  4. Dealocation is very very important in HEAP. because it is under our controll so unless we
     *     clean it it will not get cleaned itself.
     *  5. In stack the functions are arranged in order of their exicuaiotn Main is at lowest.
     *  6. After main load it starts exicution of instide it.
     *  7. As soon as a function is called inside the main it loads above Main , and if there si
     *     another funciton inside that function it loads above that funcito. and soo on.
     *  8. Allways the topmost function is running while the billow are waiting for the exicution
     *     and resualt returning.
     *  9. As a Fn ends its stack is cleared, with stack every Var, Arguments, and returntyps are
     *     cleared inside the 'STACK FRAME'.
     *
     *  A: STACK :----------------------------------------
     *  10. Unlike HEAP 'STACK' is:
     *           1.FiXED in size,
     *           2.Consistently Running while protram is running,
     *           3.Makes and Cleans its Own Shit.
     *  11. If in any case the Memory requirement of stack goes beyond the memory it has,
     *      ===> its ' STACK OFERFLOW '  {{ Program Crash. }}
     *
     *  B: HEAP :-----------------------------------------
     *   1. We Tell the compailar to assign a value to heap in C/C++/ and rust.
     *   2. when it happens Stack stores
     *        Pointer  = Ptr
     *        Length   = Len
     *        Capacity = Cp
     *      inside the 'stack' => this is what help us find the Heap we stored.
     *   3. If we store a Array in heap it Stored in the order we gave from billow to upwords.
     *    eg. arr = [a, b, c, d, e]
     *    in heap it wil be
     *  Y:
     *    |^^^^^^^^^^^^|
     *    | arr[4] = e | 🔝 ⮙
     *    | arr[3] = d | 🔝 |
     *    | arr[2] = c | 🔝 |
     *    | arr[1] = b | 🔝 |
     *    | arr[0] = a | 🔝 |
     *    |____________|
     *  and   arr is pointer stored in
     *   G:  Stack.
     *   DX: HENSE WE USE ARRAYS LIKE  =>  Arr[n].
     *   ⭐ to access the array the var we decleared will have CPL of heap inside Stack so CPL will
     *      be taken from stack and then heap will be searched. => heap is slow
     * */

");

    /*  INFO: Memroy IN PROGRAM
     *
     *     B: 1. Heap                // FREE PULL
     *     G: 2. Stack               // Fixed Sized Memory + Functions ( Variables, Arguments, Returns and Returniung Places)
     *     Y: 3. Static / Global     // Global Variables
     *     R: 4. Code                // Code Writen
     *
     *  DX:
     *  while protram is compaliling  2,3,4 are calcualted.
     *  while proram is runnigng the stack data is changing { not Size } while code and static/global stays.
     *  while Fucntion is running its stack frame is changing after fun die  stack frame die.
     *  Y:
     *  1. Stack = Call by Value. = value is in stack.
     *  2. Heap = Call by reference = valuse is in Heap but pointer is in stack.
     *  3. In C/C++ we allocate the memory from Heap with 'maloc/new' and dealocate with 'free/delete'
     *  4. Dealocation is very very important in HEAP. because it is under our controll so unless we
     *     clean it it will not get cleaned itself.
     *  5. In stack the functions are arranged in order of their exicuaiotn Main is at lowest.
     *  6. After main load it starts exicution of instide it.
     *  7. As soon as a function is called inside the main it loads above Main , and if there si
     *     another funciton inside that function it loads above that funcito. and soo on.
     *  8. Allways the topmost function is running while the billow are waiting for the exicution
     *     and resualt returning.
     *  9. As a Fn ends its stack is cleared, with stack every Var, Arguments, and returntyps are
     *     cleared inside the 'STACK FRAME'.
     *
     * G: A1: STACK :----------------------------------------
     *  10. Unlike HEAP 'STACK' is:
     *           1.FiXED in size,
     *           2.Consistently Running while protram is running,
     *           3.Makes and Cleans its Own Shit.
     *  11. If in any case the Memory requirement of stack goes beyond the memory it has,
     *      ===> its ' STACK OFERFLOW '  {{ Program Crash. }}
     *
     * G: A2: HEAP :-----------------------------------------
     *   1. We Tell the compailar to assign a value to heap in C/C++/ and rust.
     *   2. when it happens Stack stores
     *        Pointer  = Ptr
     *        Length   = Len
     *        Capacity = Cp
     *      inside the 'stack' => this is what help us find the Heap we stored.
     *   3. if we store a Array in heap it Stored in the order we gave from billow to upwords.
     *    eg. arr = [a, b, c, d, e]
     *    in heap it wil be
     *  Y:
     *    |^^^^^^^^^^^^|
     *    | arr[4] = e | 🔝 ⮙
     *    | arr[3] = d | 🔝 |
     *    | arr[2] = c | 🔝 |
     *    | arr[1] = b | 🔝 |
     *    | arr[0] = a | 🔝 |
     *    |____________|
     *  and   arr is pointer stored in
     *   G:  Stack.
     *   DX: HENSE WE USE ARRAYS LIKE  =>  Arr[n].
     *    --------------------------------------------------
     *   ⭐ to access the array the var we decleared will have CPL of heap inside Stack so CPL will
     *      be taken from stack and then heap will be searched. => heap is slow
     * */

    println!(
        " 
Memory types in rust:
===============================================================================================
 A. stack: 
  1. Stack Frame created For Every Function.
  2. Fixed sized.
  3. Size Calculated at compaillation.
     
  |~~~~~~~~~~~~~~~| => Stack For Functions
  |               |
  |               |
  |               |
  |               |
  |               |
  |               |
  |               |
  |               |
  |               |
  |               |
  |               |
  ||^^^^^^^^^^^^^||
  || stack frame || => frame for every Function.
  ||_____________||
  |_______________|  
  4. when a stack die all the variables, Arguments, Return positions die,when a stack frame die its variables die.
=============================================================================================== "
    );
}

/* FUN: copy of stack and ownership of heap. and Ownership. */
fn ownership_transfer_vs_copy_trait() {
    let x = 5;
    let y = x; // Copy

    println!("{}", x);
    println!("{}", y);

    let string1 = String::from("Hello.");
    let string2 = string1;

    // DX:  This will give error UNCOMMENT.
    // println!("{}", string1);
    println!("{}", string2);
    println!(" the Ownershipo of string1 is transfered ot string2 so s1 is dead.");

    // G: but we can still do that in rust

    let string3 = string2.clone();
    println!(" with this string2 == string3 == {}", string3);

    println!(
        " 
===============================================================================================
  eg  fn a(){{                    
            let x: &str = 'hello';
            let y: i32 = 22;      
            b();                  
            }}                    
        b(){{ 
            let x: String = String::from('world');
            }}

 |^^^^^^^^|
 |        |
 | b()  x | => stack frame for fn b()  
 | a() x,y| => Stack frame for fn a() 
 |________|    
***********************************************************************************************  
here  in 1.fn a() 
            A.botrh x and y are fixed variables so they are sored in Stack.
         2.fn b()
            A. X is string from ('somethign') which can be of different size so X is stored in 
            HEAP but its pointer is stored in STACK whith the help of heap. heap allocate memory 
            to the X
===============================================================================================

a. pushing in stack is faster > allocating memory in heap.
b. accessing data in stack is faster > on Heap ( because pointer is stored in stack and it points to words the Heap)
");
}

/* FUN: HEAP IMPORTANCE + golden_rules of ONWERSHIP */
fn golden_rules() {
    println!(
        " 
===============================================================================================
 B. Heap: 

    1. Less Organised.
    2. Data => can be Large , Dynamic , and lifetime of data can be controlled.
    3. Grow-Shrink => at Runtime. 
    4. Dangerous if not controlled. // Must be controlled.
===============================================================================================
"
    );
    /* G:  NOW 3 (ABC) GOLDEN RULES: Tattow them.
     * --------------------------------------
     *   A-1. Each value ==> must have OWNER.
     *   B-2. 1 VAR = 1 OWNER( NO MORE NO LESS)
     *   C-3. When Owner is Out Of SCOPE = value Die.
     *     4. when we assign value to another owner the value is MOVED SO OWNER Dies => "Inviladated".
     *     5. when we assign a variable to anoteher it ether copy(Stack) or Moved (Heap)
     *   DX:
     *     6. when we assign a Heap variable to the functions ARGUMENT its like Aggisgining a Heap
     *        value to Another Owner ∴ the orignal owner DIES.
     *
     * B: eg.  STACK
     *=================================================
     *     {
     *       sum = INVALD  DX: not decleared
     * G:
     *      let sum: &str =  'I am sum.';
     *        println!( sum );
     *      // sum is valid => do stuffs with Sum
     *     }
     *     //DX: sum dead.
     *=================================================
     *
     * B: eg.  HEAP
     *     {
     *       sum = INVALD  DX: not decleared
     * G:
     *      let sum: String = String::from('I am sum.');
     *      Y: '👆' this time into HEAP
     *     G:   println!( sum );
     *      // sum is valid => do stuffs with Sum
     *     }
     *     //Y: Sum Dies
     *     IMP:{ RUST ALLOCATE AND DEALOCATE THE MEMORY IN HEAP AUTOMATICALLY} <<<<<<<<<<<<<<<<<<<<<<
     *=================================================*/

    /* Y: the copy of Stack Data is done from above example.
     *   ∴ simple things get Copyed in RUST. ==> Copy Trate.
     * */
}

/* FUN: Ownership transfer even with ARGUMENTS in Function. */
fn ownership_working_methods() {
    /* FUN_2: Copy Trait. */
    fn copy_from_stack(abcd: u32) {
        println!("this is op of function copy_from_stack:{}", abcd);
        println!("just changed the repo name.");
    }

    /* FUN_2: Ownership Transfer. */
    fn ownership_transfer_into_attribute(abcd: String) {
        println!("This is the OP of ownership_transfer_into_attribute():{}. And it takes ownership of s so s is died.", abcd);
    }

    /* FUN_2: Returning a heap value will kill the returning variable. */
    fn gives_ownership() -> String {
        let returning_value = String::from("Hey i am geting Return... 옜");
        returning_value
    }

    /* FUN_2: Takes Ownership in Atribute and return Ownership in Return Value.*/
    fn takes_and_gives_ownersip(taken: String) -> String {
        let given = taken;
        println!("Here string is taken by function: {}", given);
        given
    }

    /* Y:
     *   => uncomment the code to understand
     *   how variables stored inside HEAP is
     *   Transfer ownership even while usign
     *   functions Attribute
     *   => whereas variabnles stored inside
     *   stack are copyed instead.
     */

    // P1a:   A
    let a: u32 = 23;
    copy_from_stack(a);
    println!(
        "this is op of the orignal a: which is  passed to function copy_from_stack{}",
        a
    );

    // P1a:   B
    let s = String::from("mukul");
    ownership_transfer_into_attribute(s);
    // println!("{}", s); //IMP: UNCOMMENT

    // P1a:   C
    let ownership_taken = gives_ownership();
    println!("Now this is just a String::From created in function gives_ownership which return this value:{ownership_taken}.
    we didnt gave any parameters in this function at all. but the ownership of the stinrg inside function must have died.");

    /* Y: both the functions A and B  are same,
     *    but just because one variable is stored in heap
     *    we cannot treate it same. */

    //P2c:  NOW we will do something greate --->
    let tripple_return_string_1 = String::from("I am going to run 3 times...  ⴻⴻⴻⴻⴻ");
    let tripple_return_string_2 = takes_and_gives_ownersip(tripple_return_string_1);
    println!(
        "The same function return the string here inside:{}",
        tripple_return_string_2
    );
}

/* FUN: Reference with "&" and  */
fn reference_and_uses() {
    /* FUN_2:  lets carete a FUNCTION that calcualtes the LENGHT of string */
    fn calculate_string_lenght1(s: String) -> (String, usize) {
        let length = s.len();
        (s, length)
    }
    let reference_string_1 = String::from("Yes i am the reference string !");
    let (new_s, length) = calculate_string_lenght1(reference_string_1);
    println!("String:'{new_s}' and its Length:'{length}'");

    /* Y: in above example the "REFERENCE_STRING_1" => DIED;
     * hence we use different approch this time
     * FUN_2: lets create another function that will only take string as reference and return the size.*/
    fn calculate_string_lenght_2(s: &str) -> usize {
        s.len()
        /* Y: return direct what we want.
         *   Hear instead of String we can use => &String
         *   but we user &str => which is different and is known as "Slice" */
    }

    let reference_string_2 = String::from("Yes i am the 2nd reference string !");
    let length_of_the_reference_string_2 = calculate_string_lenght_2(&reference_string_2);
    println!(
        "orignal String:{}, and its length {}",
        reference_string_2, length_of_the_reference_string_2
    );

    /* B: In reference fn calculate_string_lenght_2(reference)
     *  we are boworing the value so we
     *       1.cannot change it.
     *       2.cannot pass it to other function.
     *       3.cannot give it to other owner.
     */

    /* Y:  MUTABLE REFERENCE
     *   1.the mutable_recreance_taker_() will take mutable value
     *   and add the line "added line extra."
     */
    /* FUN_2: Thsi function takes the string as mutable and add extra string in it. and then print.
     *  AS the string is taken as reference => the scope of the reference is the function ∴ we can
     *  only print or process valus inisde fun, and cannot return it.*/
    fn mutable_recreance_taker_(string: &mut String) {
        string.push_str("added line extra."); //  with this we add the exciting files.
        println!("the orignal string + added line. = {}", string);
        println!("AS the string is taken as reference => the scope of the reference is the function ∴ we can only print or process valus inisde fun, and cannot return it !");
    }

    let mut mutable_reference_string1 = String::from("Yes I am an mutable refernce.");
    mutable_recreance_taker_(&mut mutable_reference_string1);

    /*
     * IMP:  RESTECTIONS :::::
     *   |------------------------------------------------
     *   1. Only  one mutable reference for a piece inside a SCOPE.
     *
     * G: " With this rust can prevent "DATA-RACES".
     *    if one data has many more than one pointers, while running a program
     *    if a data is being processed and othe pinter try to change or read data
     *    that will make program go corupted."
     *
     *
     * */
    let mut no_multy_mutable_reference_eg =
        String::from("1 val in a scope = one Mut reference ONLY.");

    let reference_of_one = &mut no_multy_mutable_reference_eg;
    // let reference_of_two = &mut no_multy_mutable_reference_eg;

    println!(
        "only one can take the &mut => mutable reference ∴ more than one will => ERROR. (hence reference_of_two is commented).
         so the reference_of_one is => {}",
         reference_of_one

    );

    /*
     * Y: |==============================================
     *    but this can only happen when the  references are MUTABLE, if references
     *    are "IMMUTABLE" we can use myltyple references without a problem
     *    because myltyple pointers can read the data without a problem. i guess.
     *    ∴ you can have multyple refeerences.
     */

    let non_mutable_references_one = String::from("i am a non mutable reference.");

    let reference_of_non_mut_1 = &non_mutable_references_one;
    let reference_of_non_mut_2 = &non_mutable_references_one;
    // let reference_of_non_mut_3_which_try_to_be_mut = &mut non_mutable_references_one;
    // Y: this will give error [IMMUTABLE can not be Mutate.]
    println!("{reference_of_non_mut_1} {reference_of_non_mut_2}");

    let mut a_mutable_but_used_as_both_mut_and_immutalbe_refernece = String::from("Yes thats me.");
    let reference_for_the_above_1 = &a_mutable_but_used_as_both_mut_and_immutalbe_refernece;
    let reference_for_the_above_2 = &a_mutable_but_used_as_both_mut_and_immutalbe_refernece;
    let mut reference_for_the_above_with_mutability_added =
        &a_mutable_but_used_as_both_mut_and_immutalbe_refernece;

    println!("{reference_for_the_above_1}, {reference_for_the_above_2}, {reference_for_the_above_with_mutability_added} all three are printing.");

    let mut reference_for_the_above_with_mutability_added_2 =
        &a_mutable_but_used_as_both_mut_and_immutalbe_refernece;
    println!("{reference_for_the_above_with_mutability_added_2}: Added it billow the scope ended of the others.");

    // Y: ===============Commented======================================================|
    /* FUN_2: a function whicn tryes to retnr value of Reference.*/
    // fn reference_returner_nil_fun() -> &String {
    //     let s = String::from(" a rererence value.");
    //     &s
    //
    //     /* DX:
    //      *   ERR 1. Cannot return reference to local var => data own by current fun.
    //      *   ERR 2. The function return type contain "Borrowed Val" , but the value
    //      *          isenti borrowed from anythere.
    //      */
    // }
    // Y: ===============Commented======================================================|

    println!(
        "



        RULES of REFERENCES: 
        ====================================================================| 
          1. at a given time we can have Either :
                        a. any number of referneces.
                    or  b. 1 mut reverence.
          2. references must be valid. => 
                mean: The data they poniting should be valid. {{not out of SCOPE.}} "
    );
}

fn main() {
    println!(
        " 🖳🖳🖳🖳🖳🖳🖳🖳   chapter_4_Ownership :    🖳🖳🖳🖳🖳🖳🖳🖳
=============================================================================================== "
    );

    memory_management(); //          INFO: Memory Management in different Languages.
    memory_info(); //                 IMP: VERY IMP MEMORY WORKAEOUND IN PROGRAMS
    example_one(); //                INFO: Real EXample 1:
    golden_rules(); //                IMP: VERY IMP INFO ON HEAP AND STACK
    ownership_working_methods(); //   IMP: This contain some examples to show owherships.
    reference_and_uses(); //          IMP: Thsi is how we use References.
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
fn example_one() {
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
    /* Y:  NOW 3 (ABC) GOLDEN RULES: Tattow them.
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

/* FUN:  Ownership transfer even with ARGUMENTS in Function. */
fn ownership_working_methods() {
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
    println!("{}", a);

    // P1a:   B
    let s = String::from("mukul");
    ownership_transfer_into_attribute(s);
    // println!("{}", s); //IMP: UNCOMMENT

    // P1a:   C
    let ownership_taken = gives_ownership();
    println!("{ownership_taken}");

    /* Y: both the functions A and B  are same,
     *    but just because one variable is stored in heap
     *    we cannot treate it same. */

    //P2c:  NOW we will do something greate --->
    let tripple_return_string_1 = String::from("I am going to run 3 times...  ⴻⴻⴻⴻⴻ");
    let tripple_return_string_2 = takes_and_gives_ownersip(tripple_return_string_1);
    println!("{}", tripple_return_string_2);
}

/* FUN_2: Copy Trait. */
fn copy_from_stack(abcd: u32) {
    println!("{}", abcd);
    // println!("just changed the repo name.");
}

/* FUN_2: Ownership Transfer. */
fn ownership_transfer_into_attribute(abcd: String) {
    println!("{}", abcd);
}

/* FUN_2: Returning a heap value will kill the returning variable. */
fn gives_ownership() -> String {
    let returning_value = String::from("Hey i am geting Return... 옜");
    returning_value
}

/* FUN_2: Takes Ownership in Atribute and return Ownership in Return Value.*/
fn takes_and_gives_ownersip(taken: String) -> String {
    let given = taken;
    given
}

fn reference_and_uses() {
    println!("==============");
}

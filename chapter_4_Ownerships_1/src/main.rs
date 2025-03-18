fn main() {
    println!( "\n\n\n


   🖳🖳🖳🖳🖳🖳🖳🖳   chapter_4_Ownership :    🖳🖳🖳🖳🖳🖳🖳🖳
===============================================================================================
Memory management types: 

1. Garbage Collection => No Err + Easy/Fast Write | Large + Slow run + No Control.
   java | c# and all modern lagn.
  NOTE: garbage collectore automatically cleans its memory afer some time. everyting loads again for 1st time.

2. Memory management  => too many Error + Hard/Slow Write | Small + Fast run + Full Controll.
    C | C++ 

3. Ownership Model    => No Err* + someWhat Hard and Slowest write | Small + Fast run + vary much Controll (memory safe so its automatic).
    Rust = compail time memory check feature. (with 'unsafe' we can overwrite this.) Borrow checker.
=============================================================================================== \n\n\n
" );

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
     *     cleared inside the "STACK FRAME".
     *  10. Unlike HEAP 'STACK' is:
     *           1.FiXED in size,
     *           2.Consistently Running while protram is running,
     *           3.Makes and Cleans its Own Shit.
     *   11. If in any case the Memory requirement of stack goes beyond the memory it has,
     *      ===> its " STACK OFERFLOW "  { Program Crash. }
     *
     *  B: HEAP :
     *   1. We Tell the compailar to assign a value to heap in C/C++/ and rust.
     *   2. when it happens Stack stores
     *        Pointer  = Ptr
     *        Length   = Len
     *        Capacity = Cp
     *       inside the "stack" => this is what help us find the Heap we stored.
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
     *
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

    let x = 5;
    let y = x; // Copy

    println!("{}", x);
    println!("{}", y);

    /* Y: the copy of Stack Data is done from above example.
     *   ∴ simple things get Copyed in RUST. ==> Copy Trate.
     * */

    let string1 = String::from("Hello.");
    let string2 = string1;

    // DX:  This will give error UNCOMMENT.
    // println!("{}", string1);
    println!("{}", string2);
    println!(" the Ownershipo of string1 is transfered ot string2 so s1 is dead.");

    // G: but we can still do that in rust

    let string3 = string2.clone();
    println!(" with this string2 == string3 == {}", string3);
}

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

    /*   INFO: Memroy IN PROGRAM
     *
     *       B: 1. Heap                // FREE PULL
     *       G: 2. Stack               // Fixed Sized Memory + Functions ( Variables, Arguments, Returns and Returniung Places)
     *       Y: 3. Static / Global     // Global Variables
     *       R: 4. Code                // Code Writen
     *
     * IMP:
     *  while protram is compaliling  2,3,4 are calcualted.
     *  while proram is runnigng the stack is changing while code and static/global stays.
     *  while Fucntion is running its stack frame is changing after fun die  stack frame die.
     * DX:
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
     *  9. As one Fn ends the stack is cleared, with stack every Var, Arguments, and returntyps are
     *     cleared inside the "STACK FRAME".
     *  10.Unlike heap stack is 1.FiXED ins size, 2.Running while protram is running, 3.Cleans its
     *     own stuffs.
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
  4. when a stack die all the variables die,when a stack frame die its variables die.
=============================================================================================== "
    );
    println!(
        " 
===============================================================================================
 B. Heap: 

    1. Less Organised.
    2. Data => can be Large , Dynamic , and lifetime of data can be controlled.
    3. Grow-Shrink => at Runtime.
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

    /* Y:  NOW 3 GOLDEN RULES: Tattow them.
     * --------------------------------------
     *     1. Each value ==> must have OWNER.
     *     2. 1 VAR = 1 OWNER( NO MORE NO LESS)
     *     3. When Owner is Out Of SCOPE = value Die.
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
     *     //DX: sum dead.
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

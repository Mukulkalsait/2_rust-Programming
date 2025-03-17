fn main() {
    println!( "\n\n\n


   🖳🖳🖳🖳🖳🖳🖳🖳   chapter_4_Ownership :    🖳🖳🖳🖳🖳🖳🖳🖳
===============================================================================================
Memory management types: 

1. Garbage Collection => No Err + Easy/Fast Write | Large + Slow run + No Control.
   java | c# and all modern lagn.
  NOTE: garbage collectore automatically cleans its memory afer some time. everyting loads again for 1st time.

2. Memory management  => too many Error + Hard/Slow Write | Small + Fast run + Very Much Controll(memory safe so its automatic).
    C | C++ 

3. Ownership Model    => No Err* + someWhat Hard and Slowest write | Small + Fast run + Full Controll.
    Rust = compail time memory check feature. (with 'unsafe' we can overwrite this.) Borrow checker.
=============================================================================================== \n\n\n
" );

    println!(
        " 
Memory types in rust:
===============================================================================================
stack: 
  1. Created For Every Function.
  2. Fixed sized.
  3. Size Calculated at compaillation.
     
  |~~~~~~~~~~~~~~~| => Stack For Function.
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
  || stack frame || => frame contain a variable.
  ||_____________||
  |_______________|  
     
  eg  fn a(){{                    
            let x: &str = 'hello';
            let y: i32 = 22;      
            b();                  
            }}                    

 |^^^^^^^^| STACK      
 |        | For        
 | b()  x | The        
 | a() x,y| Function
 |________| Above '👆'   

  4. when a stack die all the variables die,when a stack frame die its variables die.
=============================================================================================== "
    )
}

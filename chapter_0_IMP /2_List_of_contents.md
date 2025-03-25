0_imp:

- 1_all_methods
- 2_List

<!-- Y: chapter_1: -->

- how prontln!() = works
a. "{}{}", a, b
b. "{a}"
c. '' = unicode_char
d. OP.red() // use colored::\*; // library for colored OP
e. OP can contain "👆" emojis.
<!--DX: =============================================================================================== -->

<!-- Y: chapter_2_guessing_game -->

- Bunch of LIBRARY's
- and guessing game.
- LIBRARY :
  A. use std::io;

         io::stdin()
             //Y: the lines billow are for reading of value and then returing resualt which is enum!!!
             .read_line(&mut guess)
             .expect("Faleur to read line !!");

  B. use std::cmp::Ordering;

         match guess.cmp(&secret_number) {
             Ordering::Less => println!("{}", "Too Small 🔽".red()),
             Ordering::Greater => println!("{}", "Too Big 🔼".red()),
             Ordering::Equal => {break;}
             }

  <!--DX: =============================================================================================== -->

<!-- Y: chapter_3_1_variable_mutabliity -->

- Mutablility = mut
- defautl Non-Mutable
∴ Shadowing with "let" again.
Shadowing With = different datatypes = 🆗
Shadowing With = different sizes = 🆗
Shadowing With = Calculation /or mathimetical equation = 🆗
-- Remember Ownership.
<!--DX: =============================================================================================== -->

<!-- Y: chapter_3_2_data_types_scaller  -->

@ IF-BC

- int = signed/unsigned 8 to 128
- float = 32/64 :
- bull = un-set/ forcefully Set.
- char = unicode and normal.
<!--DX: =============================================================================================== -->

<!-- Y: chapter_3_3_datatypes_compound -->

# Tupil

- tupil creation.
- access
  => dot method
  => deconstruction with let(a,b,c)// total variables = tupil.values

# Array

- array creation // normal
- @rules
  1.  fixed size. or {use => VECTOR }
  2.  same data sizes.
- access in nomral way.

# Fn

- function creation.
- return type fucntion creation.
- Array return type.
- if else:
  -- if condition{}
  -- else if condition{}
  -- else {}
- turnory operator
  -- let variable = if condition {} else{};
- loops in rust. // special
  -- loop {
  -- if condition{ break;}
  -- }

<!--DX: =============================================================================================== -->

<!-- Y: chapter_3_4_ownership_1 -->

# Fu

    1. memory_management()
       a. Garbage
       b. Memory management
       c. Ownerhip modules.

    2. memory_info()
       a. Heap(reference)/Stack(value)/Static(global)/code => parts of memory
       b. Heap vs Stack and explanations.
       c. Heap storage with stack help.
       d. Stack and Stack Frame Stracture,

    3.

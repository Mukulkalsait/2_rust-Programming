0_imp:

- 1_all_methods
- 2_List

chapter_1:

- how prontln!() = works
  a. "{}{}", a, b
  b. "{a}"
  c. '' = unicode_char
  d. OP.red() // use colored::\*; // library for colored OP
  e. OP can contain "👆" emojis.

chapter_2_guessing_game

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

chapter_3_1_variable_mutabliity

- Mutablility = mut
- defautl Non-Mutable
  ∴ Shadowing with "let" again.
  Shadowing With = different datatypes = 🆗
  Shadowing With = different sizes = 🆗
  Shadowing With = Calculation /or mathimetical equation = 🆗
  -- Remember Ownership.

chapter_3_2_data_types_scaller --> IF-BC

- int = signed/unsigned 8 to 128
- float = 32/64 :
- bull = un-set/ forcefully Set.
- char = unicode and normal.

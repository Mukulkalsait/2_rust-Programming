use colored::*;
use rand::Rng;
use std::cmp::Ordering;
// use std::{io, u32}; // prefiusly usign this but u32 was giving HINTS.
use std::io;

fn main() {
    println!(
        "|--------------------------------- Guessing Game !---------------------------------|"
    );
    let secret_number = rand::thread_rng().gen_range(1, 1001);

    // println!("The secret number is : {}", secret_number);

    loop {
        println!("😎 Please input your Guess in the range of 1 to 1000:");
        let mut guess = String::new();
        io::stdin()
            //Y: the lines billow are for reading of value and then returing resualt which is enum!!!
            .read_line(&mut guess)
            .expect("Faleur to read line !!");
        // B: explination:
        // 1 .read_line( ) will put everyting we passed on in as string + return (resualt)
        // R: resualt is an enum (enumeration=that can be in multyple states( variant )) just like time varient.
        //  resualt will return: (ok) or (Err)
        // Y: after read_line() 󰿄
        // we added -> expect() =>
        //        A. if resualt == Err { expect will -> crash program + display given
        //           message}
        //        B. if resualt == ok { expect will -> take the return value
        //           that "ok" is holding} and return only value (not message).
        //
        // G: without expect the program will give warning while compailing.

        // let steatment = ("{}", "Your Guess :".red());
        // println!(steatment "{}", guess);
        println!("Your Guess: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("🚫 Please Input Numbers Only !");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small 🔽".red()),
            Ordering::Greater => println!("{}", "Too Big 🔼".red()),
            Ordering::Equal => {
                println!("{}", "You Won !!! 💛💙🩶🩷🤍💖💛💙🩶🩷🤍💖".green());
                additional_functionality_additioon();
                break;
            }
        }
    }
}

fn additional_functionality_additioon() {
    println!("next functionallitye is to add :
========================================================================================================
1. everyt time uer inout we are allready checking if it big or small , now we have to check how close the users guess is.
2. we will be needing 2 different mutable variables, a. for closest smaller guess and b. closest biger guess.
3. each time the guess is compaired with the exciting one guess if the guess is closer the newer will replace the older one.
4. and with the output we will be showing the result to choose in the newer 'shringked' range  with the help of a and b.
5. with this method we can increase the guess range drastically and add levels like easy, modaret, hard, and assested game or unassisted.
")
}

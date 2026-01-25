use colored::*;
use rand::Rnag;
use std::{cmp::Ordering, io};

fn main() {
    println!(
        "|--------------------------------- Guessing Game !---------------------------------|"
    );
    let secret_number = rand::thread_rng().gen_range(1, 1001);

    println!("The secret number is : {}", secret_number); // DX: turn off when done.

    loop {
        println!("😎 Please input your Guess in the range of 1 to 1000:");
        let mut guess = String::new();
        let mut cn: u32 = 3333; // closest numb
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
            Ordering::Less => smaller(guess, cn),
            Ordering::Greater => greater(guess, cn),
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
6. add what every you want to add just do it fast.
")
}

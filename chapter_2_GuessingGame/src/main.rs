use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::{io, u32};

fn main() {
    println!(
        "|--------------------------------- Guessing Game !---------------------------------|"
    );
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is : {}", secret_number);

    loop {
        println!("😎 Please input your Guess:");
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
                break;
            }
        }
    }
}

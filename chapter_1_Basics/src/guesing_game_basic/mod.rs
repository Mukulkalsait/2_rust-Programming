use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io; // Y: Previously used only u32 but got suggestoins.

mod upgrade_features;

pub fn run_the_game() {
    println!("|---------------------------------| Guessing Game |---------------------------------|");

    let randome_number = rand::thread_rng().gen_range(1, 1001);

    loop {
        println!("😎 Please input your Guess in the range of 1 to 1000:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Falure to read the line. !!!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("🚫 Please Input Numbers Only !");
                continue;
            }
        };

        match guess.cmp(&randome_number) {
            Ordering::Less => {
                println!("Too Small ⏬ \n Your number is {} smaller than actuall number.", stringify!(less_number).red())
            }
            Ordering::Greater => {
                println!("Too Big ⏫ \n Your number is {} Bigger than actuall number.", stringify!(more_number).red())
            }
            Ordering::Equal => {
                println!("{}", "You Won !!! 💛💙🩶🩷🤍💖💛💙🩶🩷🤍💖".green());
                upgrade_features::extra_instructions();
                break;
            }
        }
    }
}

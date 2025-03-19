use colored::*;
use rand::{Rna, Rng};
use std::cmp::Ordering;
use std::{io, u32};

fn main() {
    println!("I have came Rust !!!");

    println!("WHAT I LEARN TILL NOW : ");
    println!("install && update carggo and rustup ");
    println!("open local docs in ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/*");
    println!("build & run program using cargo ");

    println!("cargo init");
    println!("cargo build");
    println!("cargo run");
    println!("cargo check");
    println!("cargo bulid --release");
    println!("cargo-watch (tailwind like --watch command )");

    //R: printing values;
    let x = 4;
    let y = 12;

    println!("X = {x} and y + 2 = {} and z = {}", y + 2, 23);

let anotherSecretNumber:u32 = rand::thread_rng().gen_range(1,1001);
    
    println!("💖 Please fasten your belts to start the huge guessing Game from 1 to 1000 numbers this time ....");

    loop {
        println!("👇 Make a Guess ....");

        let mut guess:  =  String::new();
        io::stdin().read_line(&mut guess).expect("Falure to read line !");
       
        println!("You Guessed : {}" , guess);
        println!("\n\n\n");
         
        let guess: u32 = match guess.trim().parse() {
            Ok(num)=> num,
            Err(_)=>{
                println!("🚫 Please Input Numbers Only !");
                continue;
            }
            
        };

        match  guess.cmp(&anotherSecretNumber) {
            Ordering::Less => println!("{}", "Too Small 🔽".red()),
            Ordering::Greater => println!("{}", "Too Big ╆ ".red()),
            Ordering::Equal => {
                println!("{}", "You Won !!! 💛💙🩶🩷🤍💖💛💙🩶🩷🤍💖".green());
                break;
            }
        }

    }
}

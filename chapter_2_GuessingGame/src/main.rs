use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing Game!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is : {}", secret_number);
    println!("Please input your Guess:");

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

    println!("your guess: {}", guess);

    let guess: u32 = guess.trim().parse().expect("Please type a Number ...🙏🏼");

    //BASH: ther error that we purpusfully produced to crash if input is not readable.
    //╰─ cargo run
    //    Compiling chapter_2 v0.1.0 (/home/mukuldk/1Home/1Projects/class/rust-Programming/chapter_2_GuessingGame)
    // warning: unused import: `string`
    //  --> src/main.rs:1:15
    //   |
    // 1 | use std::{io, string};
    //   |               ^^^^^^
    //   |
    //   = note: `#[warn(unused_imports)]` on by default
    //
    // warning: `chapter_2` (bin "chapter_2") generated 1 warning (run `cargo fix --bin "chapter_2"` to apply 1 suggestion)
    //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
    //      Running `target/debug/chapter_2`
    // Guessing Game!
    // Please input your Guess:
    // 12
    // your guess: 12

    // R: printing values;
    // let x = 4;
    // let y = 12;
    //
    // println!("X = {x} and y + 2 = {}", y+2);
    //
    // OP = X = 4 and y + 2 = 14.
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => println!("You Won !!!"),
    }
}

use std::clone;

fn fib(num: u32) -> u32 {
    let mut first: u32 = 0;
    let mut second: u32 = 1;

    if num == 0 {
        return first;
    }
    if num == 1 {
        return 1;
    }

    for _ in 1..num - 1 {
        let temp = second;
        second += first;
        first = temp;
        println!(
            " temp:{}\n seocnd: {}\n first: {}\n|=============|",
            temp, second, first
        );
    }
    second
}

fn len_return(s: &str) -> usize {
    s.len()
}

fn len_return_string(s: String) -> usize {
    s.len()
}

fn main() {
    let num = 10;
    println!("{}", fib(num));

    let string_1 = "Mukul is here";
    println!(
        "{} is the length of &str {}.",
        len_return(string_1),
        &string_1
    );

    let string_2 = String::from("Mukul is here");
    println!(
        "The lenght of String {} is : {}",
        &string_2,
        len_return_string(string_2.clone()),
        // DX: if we dont do clone the orignal value of string_2 will be change and hence reverence
        // &string_2 will die.
    );
}

/*
* Y:  fibonaki searise
*   ___________________________________________________
*  | Num=> | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8  | 9  | 10 |
*  | ans=> | 0 | 1 | 1 | 2 | 3 | 5 | 8 | 13 | 21 | 34 |
*  |==================================================|
* */

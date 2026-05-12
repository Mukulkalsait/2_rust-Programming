use std::iter::Sum;

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn fizzbuzz(n: u32) -> String {
    if n % 15 == 0 {
        String::from("FizzBuzz")
    } else if n % 3 == 0 {
        String::from("Fizz")
    } else if n % 5 == 0 {
        String::from("Buzz")
    } else {
        n.to_string()
    }
}

fn main() {
    let x = 10;
    let y = 20;
    let sum = add_numbers(x, y);

    let mut numbers: Vec<i32> = Vec::new();
    for i in 0..5 {
        numbers.push(i * 2);
    }
    println!("{:?}", numbers);

    // --- Exercise 3: fizzbuzz loop ---
    let results: Vec<String> = (1..=15).map(fizzbuzz).collect();
    println!("{:?}", results);
}

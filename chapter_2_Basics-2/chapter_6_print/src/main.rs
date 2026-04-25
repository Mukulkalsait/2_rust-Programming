fn main() {
    print_1();
    print_2();
    print_3();
    print_4();
    print_5();
    /* Y:
     * format!: write formatted text to String
     * print!: same as format! but the text is printed to the console (io::stdout).
     * println!: same as print! but a newline is appended.
     * eprint!: same as print! but the text is printed to the standard error (io::stderr).
     * eprintln!: same as eprint! but a newline is appende
     */
}

// G: We can treat arguments like its an Array and print them with index 0,1,2,3 & ..n .
fn print_1() {
    println!("_________________________");
    println!(
        "a: {0}, b: {1} anather a: {0} and last one this time {2}",
        "1_first", "2_second", "3_third"
    );
}

// G: We can Name arguments.
fn print_2() {
    println!("  _________________________");
    println!(
        " |sub:{subject}\n |verb:{verb}\n |obj:{object}\n |int: {other_type}\n |char_type: {char_type}",
        object = "The object section.",
        subject = "The subject section.",
        verb = "The verb section.",
        other_type = 24,
        char_type = '😎'
    );
    println!("My name is {0}, {1} {0}", "Bond", "James");
}

// G: A.use of  std::fmt "{}" properly.
fn print_3() {
    // Y: Converting Numbers.
    println!("_________________________");
    println!("Base 10:               {}", 69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("this is binary of number 13: {:b}", 13);
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("string binary is giving Error"); // R:  Will error.
}

// G2: B.use of  std::fmt "{}" properly.
fn print_4() {
    println!("_________________________");
    println!("||||||||||");
    println!("{number: >5}", number = 1); // ____5
    println!("{number:x<5}", number = 1); // 5xxxx
    println!("{number:0>5}", number = 1); // 00001
    println!("{number:0<5}", number = 1); // 10000
    println!("{number:*>width$}", number = 1, width = 5); // same as above but taking it as width.
    println!("{number:*>x$}", number = 1, x = 12);

    // B: Rust 1.58+ Direct capture ARG from a surrounding variable.
    let number: f64 = 1.0;
    let width: usize = 5; // IMP: width is total width ∴ min is 2 in this case.
    println!("{number:#<width$}");
}

// G: Dead Code and formtating.
fn print_5() {
    println!("_________________________");
    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // `Structure` does not implement fmt::Display so the billow line will Error.
    // println!("This struct `{}` won't print...", Structure(3));
}

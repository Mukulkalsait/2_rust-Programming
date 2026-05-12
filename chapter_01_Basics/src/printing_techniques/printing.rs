/// Cargo Functionalities print basics:
pub fn print_cargo_basics() {
    println!("I have came Rust !!!");
    println!("WHAT I LEARN TILL NOW : ");
    println!("install && update carggo and rustup ");
    println!("open local docs in ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/*");
    println!("build & run program using cargo ");
    println!("cargo init = inintialise rust project");
    println!("cargo build = will build a binary ./target/debug/filename (debug mode)");
    println!("cargo run = will run the binary ");
    println!("cargo check = check for errors in file");
    println!("cargo bulid --release = build as release mode ./target/release/filename");
    println!("cargo run = build & run in single command");
    println!("cargo run --release = build & run as release mode ./target/release/filename");
    println!("cargo -watch (tailwind like --watch command )");
}

pub fn values_printing_methods() {
    //R: printing values;
    let x = 4;
    let y = 12;
    let ghonchu = 422;
    println!("{}", ghonchu);
    println!(
        "here, X = {x}, Y + 3 = {}, and newly created Z = {}",
        y + 3,
        50
    );
}

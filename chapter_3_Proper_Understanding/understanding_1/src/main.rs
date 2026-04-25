// Y:  cargo new <project_name> => cargo crate::
// CREAT => Cargo Rust ExcueATble

// mod math;
mod models;
mod services;
mod util;

use crate::models::user::User;
use crate::services::auth::login;
use crate::util::logger::log;

// modles
struct U {
    pub name: String,
}
impl U {
    pub fn abcd(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
// services
fn xyz(mnk: &U) {
    println!("U : {} working i dont know what.", mnk.name)
}
//utils::loger
fn logx(msg: &str) {
    println!("[LOG] {}", msg)
}

fn main() {
    // demo
    let userx = U::abcd("ame");
    logx("Starting Custome Process");
    xyz(&userx);

    // math::add(2,3);

    let user = User::new("Mukul");

    log("Starting Login Process");
    login(&user);
}

// R: RUST RESOLVER ::>
//
// # Now the Important Part
//
// # How the Rust Compiler Resolves Code
//
// When the compiler sees:
//
// ```
// crate::services::auth::login
// ```
//
// It walks the **module tree step by step**.
//
// ```
// crate
//  └ services
//      └ auth
//          └ login()
// ```
//
// Each `::` means **go deeper into the module**.
//
// ---
//
// # Rule to Remember
//
// Rust resolves paths like a **filesystem**.
//
// Example:
//
// ```
// crate::models::user::User
// ```
//
// Meaning:
//
// ```
// crate
//  └ models
//      └ user
//          └ User struct
// ```
//
// ---
//
// # Absolute vs Relative Paths
//
// ### Absolute Path
//
// Starts from crate root.
//
// ```
// crate::services::auth::login
// ```
//
// ---
//
// ### Relative Path
//
// Starts from current module.
//
// Example inside `services`:
//
// ```rust
// use crate::models::user::User;
// ```
//
// ---
//
// # What `use` Actually Does
//
// This line:
//
// ```rust
// use crate::services::auth::login;
// ```
//
// creates a shortcut.
//
// Instead of writing:
//
// ```rust
// crate::services::auth::login(&user);
// ```
//
// You can write:
//
// ```rust
// login(&user);
// ```
//
// ---
//
// # Why `pub` Exists
//
// By default **everything is private**.
//
// Example:
//
// ```
// services
//  └ auth
//      └ login()
// ```
//
// Without `pub`, `login()` cannot be accessed outside `auth`.
//
// So we write:
//
// ```rust
// pub fn login(...)
// ```
//
// Now other modules can use it.
//
// ---
//
// # The Visibility Model
//
// Rust visibility flows **downwards**.
//
// ```
// crate
//  ├ models
//  │   └ user
//  └ services
// ```
//
// Children can access parents, but not siblings unless **pub is used**.
//
// ---
//
// # How Real Libraries Look
//
// Example:
//
// ```
// crate
//  ├ parser
//  │   └ tokenizer
//  ├ compiler
//  │   └ optimizer
//  └ utils
// ```
//
// Access:
//
// ```
// crate::parser::tokenizer::parse()
// ```
//
// Once you understand this, **Rust documentation becomes easy**.
//
// Example when you see:
//
// ```
// std::io::stdin
// ```
//
// You now read it as:
//
// ```
// crate std
//  └ io
//      └ stdin()
// ```
//
// ---
//
// # Final Mental Model
//
// Rust code is always resolved like this:
//
// ```
// crate
//  └ module
//      └ submodule
//          └ item
// ```
//
// Example:
//
// ```
// crate::module::submodule::function
// crate::module::Struct
// crate::module::Enum
// ```
//
// ---
//
// # Tiny Exercise (Important)
//
// Mentally resolve this:
//
// ```
// crate::utils::logger::log
// ```
//
// Which file?
//
// Answer:
//
// ```
// src/utils/logger.rs
// ```
//
// Function:
//
// ```
// pub fn log()
// ```
//
// ---
//
// # What You Achieved Today
//
// You now understand:
//
// ```
// crate
// module
// mod
// pub
// use
// module tree
// path resolution
// ```
//
// This is **how the Rust compiler sees your project**.
//
// Most beginners never get this clear.
//
// ---
//
// # Tomorrow's Best Next Topic
//
// The next thing that will **massively improve your Rust skill** is:
//
// ### Iterators (Rust's most powerful abstraction)
//
// Things like:
//
// ```
// map
// filter
// collect
// fold
// ```
//
// This is where Rust starts feeling **very elegant**.
//
// If you want, tomorrow I can also show you something fascinating:
//
// **Why Rust iterators are faster than loops in many cases.**

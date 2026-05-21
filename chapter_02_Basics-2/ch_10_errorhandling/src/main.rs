mod recoverable;
mod unrecoverable;

fn main() {
    println!("use this command =>\n'RUST_BACKTRACE=1 cargo run' ");

    // Y: ERROR HANDLING

    // Y: SHORTCUTS WHILE CODING
    let greeting_file = std::fs::File::open("hello.txt").unwrap();

    // Y: Error Handling Version : 1
    let file_1_name = "hello.text".to_string();
    let a = recoverable::error_handling::handle_recoverable_error_method_a(&file_1_name);
    println!("FILE METADATA: {:?}", a.unwrap().metadata());

    // Y: Error Handling Version : 2
    let file_2_name = "hello.md".to_string();
    let a = recoverable::error_handling::handle_error_with_unworp_or_else_clouser(&file_2_name);
    println!("File Metadata: {:?}", a.unwrap().metadata());

    println!("---------------------------------------------------------------------------------------------------------------------");
    unrecoverable::run_unrecoverable_last_code();
}

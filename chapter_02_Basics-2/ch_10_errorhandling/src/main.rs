mod recoverable;
mod unrecoverable;

fn main() {
    println!("use this command =>\n'RUST_BACKTRACE=1 cargo run' ");

    let file_1_name = "hello.text".to_string();

    let a = recoverable::error_handling::handle_recoverable_error_method_a(&file_1_name);
    println!("FILE METADATA: {:?}", a.unwrap().metadata());

    let file_2_name = "hello.md".to_string();
    let a = recoverable::error_handling::handle_error_with_unworp_or_else_clouser(&file_2_name);
    println!("File Metadata: {:?}", a.unwrap().metadata());

    // R: unrecoverable errors are not to be handled with anythin
    unrecoverable::unrecoverable_error_triggring();
    match unrecoverable::unrecoverable_error_auto_treggring() {
        Ok(res) => {
            println!("Successed: {}", res)
        }
        Err(e) => {
            println!("Error: {:?}", e)
        }
    }
}

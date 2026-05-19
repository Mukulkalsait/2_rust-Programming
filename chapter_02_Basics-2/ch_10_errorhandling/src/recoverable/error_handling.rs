use std::{error::Error, fs::File, io::Result};

pub fn get_file_or_handle_error(file_name: &String) -> Result<File> {
    std::fs::File::open(file_name)
}

pub fn handle_recoverable_error_method_a(file_1_name: &String) -> Result<File> {
    println!("File get Error Handling Version : 1");
    let x = get_file_or_handle_error(&file_1_name);
    let a = match x {
        Ok(file) => file,
        Err(er) => match er.kind() {
            std::io::ErrorKind::NotFound => match std::fs::File::create(&file_1_name) {
                Ok(file) => file,
                Err(e) => panic!("Faild to create file: {e:?}"),
            },
            _ => {
                panic!("Problem Opening the file: {er:?}")
            }
        },
    };
    Ok(a)
}

pub fn handle_error_with_unworp_or_else_clouser(file_name: &String) -> Result<File> {
    println!("File get Error Handling Version : 2");
    let a = get_file_or_handle_error(file_name).unwrap_or_else(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            std::fs::File::create(file_name).unwrap_or_else(|err| panic!("Faild to create file: {err:?}"))
        } else {
            panic!("Faild to read file {e:?}")
        }
    });
    Ok(a)
}

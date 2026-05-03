mod api;
mod client;
mod models;
mod utils;

use api::{auth, search, upload};
use client::build_client;

fn main() {
    let client = build_client();

    match auth::login_user(&client) {
        Ok(data) => {
            println!("Login:\n{:#?}", data);
        }
        Err(err) => {
            println!("Login Error:\n{:#?}", err);
        }
    }

    match search::search_request(&client) {
        Ok(data) => {
            println!("Search:\n{:#?}", data);
        }
        Err(err) => {
            println!("Search Error:\n{:#?}", err);
        }
    }
    match upload::upload_file(&client) {
        Ok(data) => {
            println!("Upload:\n{:#?}", data);
        }
        Err(err) => {
            println!("Upload Error:\n{:#?}", err);
        }
    }
    match search::search_request2(&client) {
        Ok(data) => {
            println!("Search2:\n{:#?}", data);
        }
        Err(err) => {
            println!("Search2 Error:\n{:#?}", err);
        }
    }

    match upload::upload_file2(&client) {
        Ok(data) => {
            println!("Upload:\n{:#?}", data);
        }
        Err(err) => {
            println!("Upload Error:\n{:#?}", err);
        }
    }

    // tests_reqwest::testing_rewest_func();
    // real_usage::main_usage();
}

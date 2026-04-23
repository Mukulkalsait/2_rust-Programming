use reqwest::blocking::multipart::{self, Form}; // file accessing
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::redirect::Policy; // builder policies

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    email: String,
    passwd: String,
}

#[derive(Debug, Deserialize)]
pub struct LoignResualt {
    token: String,
}

/// send jsondata.
fn login_user(client: &Client) {
    let resualt_1 = client
        .post("http://localhost:3000/login")
        .json(&LoginRequest { email: "mukul@gmail.com".to_string(), passwd: "qwer123".into() })
        .send();
    error_handling_printer(resualt_1);
}

// basic query function
fn client_request(client: &Client) {
    let client_request = client.get("http://localhost:3000/search").query(&[("q", "rust"), ("page", "1")]).send();
    error_handling_printer(client_request);
}

/// basic auth funciton
fn auth_request(client: &Client) {
    let auth_request = client.get("http://localhost:3000/profile").bearer_auth("TOKEN").send();
    error_handling_printer(auth_request);
}

/// file uploading funciton
fn file_uploading(client: &Client) {
    let form = multipart::Form::new().file("file", "cat.png").unwrap();
    // Y:  unwrap here gives the T inside Result<T,E> part
    error_handling_printer(client.post("http://localhost:3000/file").multipart(form).send());
}

#[tokio::main]
/// Inside Tokio, .await is available only for async functions/futures. Your client is still blocking: reqwest::blocking::Client so .send() returns a Result immediately.
async fn tokio_request_function(client: &Client) {
    let body = client.get("https://api.github.com").send();
    error_handling_printer(body);
}

/// Unversal function for handling errors
fn error_handling_printer(res: Result<reqwest::blocking::Response, reqwest::Error>) {
    match res {
        Ok(response) => match &response.status().as_u16() {
            200 => {
                println!("------------OK------------");
                let data: LoignResualt = response.json().unwrap();
                println!("{:#?}", data);
            }
            404 => println!("------------NOT FOUND------------"),
            _ => println!("------------OTHER------------"),
        },
        Err(err) => {
            println!("{:#?}", err)
        }
    }
}

/// building the costume client with ClientBuilder
fn client_builder_config() -> Client {
    let timeout_duration = Duration::from_secs(5);
    let redirection_policy_limit = Policy::limited(5);
    let _cookie_store_status: bool = true;

    ClientBuilder::new()
        // .cookie_store(cookie_store_status) // Giving error:not method named 'cookie_sotre' found on struct
        .timeout(timeout_duration)
        .redirect(redirection_policy_limit)
        .build()
        .ok()
        .unwrap()
}

/// main of this file
pub fn main_usage() {
    let client_builder_var = client_builder_config();

    login_user(&client_builder_var);
    client_request(&client_builder_var);
    auth_request(&client_builder_var);
    file_uploading(&client_builder_var);
}

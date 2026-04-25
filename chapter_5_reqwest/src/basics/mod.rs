use reqwest::Response;
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
    token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    results: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    results: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProfileReponse {
    results: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UploadResponse {
    results: Vec<String>,
}

/// send jsondata.
fn login_user(client: &Client) -> Result<LoignResualt, reqwest::Error> {
    let login_request_data: LoginRequest = LoginRequest { email: "mukul@gmail.com".into(), passwd: "qwerty123".into() };
    client.post("http://localhost:3000/login").json(&login_request_data).send()?.json() //  Y:  we used the "?" here
}

// basic query function
fn search_request(client: &Client) {
    let res = client.get("http://localhost:3000/search").query(&[("q", "rust"), ("page", "1")]).send().unwrap();
    let data: SearchResponse = res.json().unwrap();
    println!("SearchResponse:\n{:#?}", data);
}

/// basic auth funciton
fn auth_request(client: &Client) {
    let res = client.get("http://localhost:3000/profile").bearer_auth("TOKEN").send().unwrap();
    let data: AuthResponse = res.json().unwrap();
    println!("AuthResponse:\n{:#?}", data)
}

/// file uploading funciton
fn file_uploading(client: &Client) {
    let form = multipart::Form::new().file("file", "cat.png").unwrap();
    let res = client.post("http://localhost:3000/file").multipart(form).send().unwrap(); // Y:  unwrap here gives the T inside Result<T,E> part
    let data: UploadResponse = res.json().unwrap();
    println!("UploadResponse:\n{:#?}", data);
}

#[tokio::main]
/// Inside Tokio, .await is available only for async functions/futures. Your client is still blocking: reqwest::blocking::Client so .send() returns a Result immediately.
async fn tokio_request_function(client: &Client) {
    let res = client.get("https://api.github.com").send().unwrap();
    let data: SearchResponse = res.json().unwrap();
    println!("BODY:\n{:#?}", data);
}

/// Unversal function for handling errors
fn error_handling_printer(res: Result<Response, reqwest::Error>) -> Option<Response> {
    match res {
        Ok(response) => {
            println!("Status: {}", response.status());
            Some(response)
        }
        Err(err) => {
            println!("{:#?}", err);
            None
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

    println!("|  LoginRequest  |");
    match login_user(&client_builder_var) {
        Ok(data) => println!("✅ Success:\n {:#?}", data),
        Err(err) => println!("❌ Faild:\n {:#?}", err),
    }

    search_request(&client_builder_var);
    auth_request(&client_builder_var);
    file_uploading(&client_builder_var);
}

use reqwest::blocking::Client;

use crate::models::{LoginRequest, LoginResponse};
use crate::utils::get_json;

pub fn login_user(client: &Client) -> Result<LoginResponse, Box<dyn std::error::Error>> {
    let json_data: LoginRequest = LoginRequest { email: "mukul@gmail.com".into(), passwd: "qwer123".into() };
    let res = client.post("http://localhost:3000/login").json(&json_data).send()?;
    get_json(res)
}

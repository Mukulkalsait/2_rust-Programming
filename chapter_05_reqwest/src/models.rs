use serde::{Deserialize, Serialize};

// Y: REQUESTS --------------------------------------------------------------------------------------------------------------------
// here we sent request.
// THEREFORE:
//  1. data from struct => json [SERIALIZATION]
//
//  SerializationList
//  1.LoginRequest

#[derive(Serialize, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub passwd: String,
}

#[derive(Serialize, Debug)]
pub struct SearchQuery {
    pub q: String,
    pub page: u16,
}

// R: RESPONSES -------------------------------------------------------------------------------------------------------------------
// here we reciece the data.
//  THEREFORE:
//  1. json => Struct [DESERALIZATION1]
//
//  DeserializationList
//  1. LoginResponse ->Option<String>
//  2. SearchResponse ->Vec<String>
//  3. UploadResponse ->String

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub token: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub results: Vec<String>,
    // Y:  vec because we don tknow how many resualts we might gee.
}

#[derive(Deserialize, Debug)]
pub struct UploadResponse {
    pub status: String,
}

use serde::{Deserialize, Serialize};

// REQUESTS --------------------------------------------------------------------------------------------------------------------
#[derive(Serialize, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub passwd: String,
}

// RESPONSES -------------------------------------------------------------------------------------------------------------------
#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub token: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub results: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct UploadResponse {
    pub status: String,
}

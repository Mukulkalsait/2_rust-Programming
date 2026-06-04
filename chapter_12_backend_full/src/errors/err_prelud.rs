// FILE: src/errors/err_prelud.rs

pub use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
pub use serde::{Deserialize, Serialize};
pub use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
/// # ===== Response Shape =====
/// - status : Fail / Error
/// - message : actual message
/// - code : status code
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
    pub code: u16,
}

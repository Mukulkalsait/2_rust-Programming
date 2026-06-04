// FILE: ./src/errors/apierrors.rs

use axum;
use serde::{Deserialize, Serialize};
use thiserror;
// EXT:

#[derive(serde::Serialize)]
/// ErrorResponse with serde:Serialize.
/// * This gives Json Like SINGLE ERROR.
/// ```json
/// { error : "Message" }
/// ```
/// > ⚡SINGLE ERROR PER RESPONSE.
pub struct ErrorResponse {
    error: String,
}

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
/// Closed Set Of Error APIs.
/// Varients => DIrect Http Mappings.
/// ThisError => AutoDisplay.
/// ``` rs
/// # Implimentation of axum::response::IntoResponse{
///  fn providing(self) -> returning axum::response::Response
///  let (status,message) =  match self
///   ApiError {all 5 options } =>
///     returning
///     axum::http::StatusCode::same_5_optiosn, ( self/message ).to_string.
///
///  let body = axum::Json(struct ErrorResponse{...});
///
///  (stats,body).into_response => Returning function.
/// }
/// ```
///
pub enum ApiError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("resource not found")]
    NotFound,
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("internal server error")]
    Internal,
}

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::Unauthorized => (axum::http::StatusCode::UNAUTHORIZED, self.to_string()),
            ApiError::Forbidden => (axum::http::StatusCode::FORBIDDEN, self.to_string()),
            ApiError::NotFound => (axum::http::StatusCode::NOT_FOUND, self.to_string()),
            ApiError::BadRequest(msg) => (axum::http::StatusCode::BAD_REQUEST, msg),
            ApiError::Internal => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()),
        };
        let body = axum::Json(ErrorResponse { error: message });
        (status, body).into_response()
    }
}

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum DatabaseOperationsErrors {
    #[error("Password is Empty:")]
    EmptyPassword,
    #[error("Max Password Size Limit {0} Reached.")] // Message Implimentation
    ExcededMaxPasswordLimit(String),
    #[error("Hash Format Invalid")]
    InvalidHashFormat,
    #[error("Hashing Error:{0}")]
    HashingError(String),
    #[error("Token is invalid.")]
    InvalidToken,
    #[error("Internal Database Error")]
    DbError,
    #[error("Please check you credentials")]
    WrongCredentialError,
    #[error("Email allready Exists, Please use different email.")]
    EmailExists,
    #[error("The User no longer Exists.")]
    UserNoLongerExists,
    #[error("Please Provide the Token")]
    TokenNotProvided,
    #[error("Permission Denide. Please contact Admin.")]
    PermissionDenied,
    #[error("User Not Authenticated, Please Authenticate User First.")]
    UserNotAuthenticated,
}

impl axum::response::IntoResponse for DatabaseOperationsErrors {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            DatabaseOperationsErrors::EmptyPassword => (axum::http::StatusCode::UNAUTHORIZED, self.to_string()),
            DatabaseOperationsErrors::ExcededMaxPasswordLimit(msg) => (axum::http::StatusCode::UNAUTHORIZED, msg), // same message here
            DatabaseOperationsErrors::InvalidHashFormat => (axum::http::StatusCode::UNAUTHORIZED, self.to_string()),
            DatabaseOperationsErrors::HashingError(msg) => (axum::http::StatusCode::UNAUTHORIZED, msg),
            DatabaseOperationsErrors::InvalidToken => (axum::http::StatusCode::UNAUTHORIZED, self.to_string()),
            DatabaseOperationsErrors::DbError => (axum::http::StatusCode::SERVICE_UNAVAILABLE, self.to_string()),
            DatabaseOperationsErrors::WrongCredentialError => (axum::http::StatusCode::UNAUTHORIZED, self.to_string()),
            DatabaseOperationsErrors::EmailExists => (axum::http::StatusCode::CONFLICT, self.to_string()),
            DatabaseOperationsErrors::UserNoLongerExists => (axum::http::StatusCode::NOT_FOUND, self.to_string()),
            DatabaseOperationsErrors::TokenNotProvided => (axum::http::StatusCode::NOT_ACCEPTABLE, self.to_string()),
            DatabaseOperationsErrors::PermissionDenied => (axum::http::StatusCode::UNAUTHORIZED, self.to_string()),
            DatabaseOperationsErrors::UserNotAuthenticated => (axum::http::StatusCode::UNAUTHORIZED, self.to_string()),
        };
        let body = axum::Json(ErrorResponse { error: message });
        (status, body).into_response()
    }
}
//------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

use axum::{http::StatusCode, response::IntoResponse, response::Response, Json};
use serde::{Deserialize, Serialize};
use std::fmt::{self, format};

#[derive(Debug, Deserialize, Serialize)]
/// what error response will look like.
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
    pub code: u16,
}

/// implimentation fo Displya (Mostly autojmatic via LSP)
impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", serde_json::to_string(&self).unwrap()) }
}

#[derive(Debug, PartialEq)]
pub enum AuthError {
    InvalidHashFormat,
    HashingError,
    InvalidToken,
    ServerError,
    TokenNotProvided,
}

impl ToString for AuthError {
    fn to_string(&self) -> String { self.to_str().to_owned() }
}

impl AuthError {
    fn to_str(&self) -> String {
        match self {
            AuthError::InvalidHashFormat => "Invalid Password Hash Format.".to_string(),
            AuthError::HashingError => "Error While Hashing Password.".to_string(),
            AuthError::InvalidToken => "Please Provide a Valid Token.".to_string(),
            AuthError::ServerError => "Internal Server Error".to_string(),
            AuthError::TokenNotProvided => "Please Provide a tocken via jS ".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum UserError {
    PermissionDenied,
    WrongCred,
    ExceedMaxPassLen(usize),
    EmptyPass,
    WeekPassword,
    UserNotFound,
    UserNoLongerExists,
    UserNotAuthenticated,
    EmailExists,
}
impl ToString for UserError {
    fn to_string(&self) -> String { self.to_string().to_owned() }
}
impl UserError {
    fn to_str(&self) -> String {
        match self {
            UserError::PermissionDenied => "You are not allowed to perform thss action.".to_string(),
            UserError::WrongCred => "Email or Password is not correct".to_string(),
            UserError::ExceedMaxPassLen(max_length) => format!("Password must not be more thatn {} charestors", max_length),
            UserError::EmptyPass => "Password cannot be empty".to_string(),
            UserError::WeekPassword => "Needs Stronger Password".to_string(),
            UserError::UserNotFound => "User not found".to_string(),
            UserError::UserNoLongerExists => "User bellonging to this token no longer esixts".to_string(),
            UserError::UserNotAuthenticated => "Authentication required, Please login again.".to_string(),
            UserError::EmailExists => "A user with this email allreayd esists.".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DBError {
    ConnectionError,
    QuerryError,
    UniqueValidation,
}

impl ToString for DBError {
    fn to_string(&self) -> String { self.to_string().to_owned() }
}
impl DBError {
    fn to_str(&self) -> String {
        match self {
            DBError::ConnectionError => "Database connection faild.".to_string(),
            DBError::QuerryError => "Query Faild".to_string(),
            DBError::UniqueValidation => "Unique constrain violation on {field}".to_string(),
        }
    }
}

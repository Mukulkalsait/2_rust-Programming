// FILE: src/errors/mod.rs

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error; // Use thiserror = better than manual ToString

#[derive(Debug, Serialize, Deserialize)]
/// # ===== Response Shape =====
/// - status : Fail / Error
/// - message : actual message
/// - code : status code
pub struct ErrorResponse {
    pub status: String, // "fail" or "error"
    pub message: String,
    pub code: u16,
}

#[derive(Debug, Error)]
/// ===== Auth Errors =====
pub enum AuthError {
    #[error("Token not provided")]
    TokenNotProvided,

    #[error("Invalid password hash format")]
    InvalidHashFormat,

    #[error("Internal server error")]
    ServerError,

    #[error("Invalid token: {reason}")] // creates automatic display for both
    InvalidToken { reason: String },

    #[error("Password hashing failed: {err_source}")]
    HashingError { err_source: String },
}

// ===== IntoResponse Implementations =====
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::InvalidToken { reason } => (StatusCode::UNAUTHORIZED, format!("Invalid token: {}", reason)),
            AuthError::TokenNotProvided => (StatusCode::UNAUTHORIZED, "Authorization token required".to_string()),
            AuthError::ServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
            AuthError::HashingError { err_source } => (StatusCode::INTERNAL_SERVER_ERROR, format!("Hashing error: {}", err_source)),
            AuthError::InvalidHashFormat => (StatusCode::BAD_REQUEST, "Invalid hash format".to_string()),
        };

        (status, Json(ErrorResponse { status: "fail".to_string(), message, code: status.as_u16() })).into_response()
    }
}

// ===== User Errors =====
#[derive(Debug, Error)]
pub enum UserError {
    #[error("Permission denied: {action}")]
    PermissionDenied { action: String },

    #[error("Invalid credentials")]
    WrongCredentials,

    #[error("Password exceeds max length: {max}")]
    ExceededMaxPasswordLength { max: usize, actual: usize },

    #[error("Password cannot be empty")]
    EmptyPassword,

    #[error("Password too weak: {reason}")]
    WeakPassword { reason: String },

    #[error("User not found")]
    UserNotFound,

    #[error("User associated with this token no longer exists")]
    UserNoLongerExists,

    #[error("Authentication required")]
    UserNotAuthenticated,

    #[error("Email already exists: {email}")]
    EmailExists { email: String },
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            UserError::PermissionDenied { action } => (StatusCode::FORBIDDEN, format!("Cannot {}", action)),
            UserError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Invalid email or password".to_string()),
            UserError::ExceededMaxPasswordLength { max, actual } => {
                (StatusCode::BAD_REQUEST, format!("Password too long: max {} chars, got {}", max, actual))
            }
            UserError::EmptyPassword => (StatusCode::BAD_REQUEST, "Password cannot be empty".to_string()),
            UserError::WeakPassword { reason } => (StatusCode::BAD_REQUEST, format!("Weak password: {}", reason)),
            UserError::UserNotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
            UserError::UserNoLongerExists => (StatusCode::NOT_FOUND, "User no longer exists".to_string()),
            UserError::UserNotAuthenticated => (StatusCode::UNAUTHORIZED, "Please login to continue".to_string()),
            UserError::EmailExists { email } => (StatusCode::CONFLICT, format!("User with email '{}' already exists", email)),
        };

        (status, Json(ErrorResponse { status: "fail".to_string(), message, code: status.as_u16() })).into_response()
    }
}

// ===== DB Errors =====
#[derive(Debug, Error)]
pub enum DbError {
    #[error("Database connection failed: {err_source}")]
    ConnectionError { err_source: String },

    #[error("Query failed: {err_source}")]
    QueryError { err_source: String },

    #[error("Unique constraint violation on '{field}'")]
    UniqueViolation { field: String },
}

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            DbError::ConnectionError { err_source } => {
                eprintln!("DB Connection error: {}", err_source); // Log internally
                (StatusCode::SERVICE_UNAVAILABLE, "Database temporarily unavailable".to_string())
            }
            DbError::QueryError { err_source } => {
                eprintln!("DB Query error: {}", err_source);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error occurred".to_string())
            }
            DbError::UniqueViolation { field } => (StatusCode::CONFLICT, format!("{} already exists", field)),
        };

        (status, Json(ErrorResponse { status: "fail".to_string(), message, code: status.as_u16() })).into_response()
    }
}

// ===== Combine multiple errors for handlers =====
#[derive(Debug)]
pub enum ApiError {
    Auth(AuthError),
    User(UserError),
    Db(DbError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Auth(err) => err.into_response(),
            ApiError::User(err) => err.into_response(),
            ApiError::Db(err) => err.into_response(),
        }
    }
}

// Auto-conversion from specific errors
impl From<AuthError> for ApiError {
    fn from(err: AuthError) -> Self { ApiError::Auth(err) }
}

impl From<UserError> for ApiError {
    fn from(err: UserError) -> Self { ApiError::User(err) }
}

impl From<DbError> for ApiError {
    fn from(err: DbError) -> Self { ApiError::Db(err) }
}

use crate::errors::err_prelud::*;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Token not provided")]
    TokenNotProvided,
    #[error("Invalid Password Hash Format")]
    InvalidHashFormat,
    #[error("Internal Server Error")]
    ServerError,

    #[error("Invalid Token: {err_source}")]
    InvalidToken { err_source: String },
    #[error("Password Hashing Faild: {err_source}")]
    HashingError { err_source: String },
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::TokenNotProvided => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::InvalidHashFormat => (StatusCode::BAD_REQUEST, self.to_string()),
            AuthError::ServerError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),

            AuthError::InvalidToken { .. } => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::HashingError { .. } => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, Json(ErrorResponse { status: "fail".to_string(), message, code: status.as_u16() })).into_response()
    }
}

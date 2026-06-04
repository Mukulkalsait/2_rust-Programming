use crate::errors::err_prelud::{self, *};

#[derive(Debug, Deserialize, Serialize, Error)]
pub enum UserError {
    #[error("Invalid Credentilas!")]
    WrongCredentials,
    #[error("Password cannot be empty!")]
    EmptyPassword,
    #[error("User not found!")]
    UserNotFound,
    #[error("User associated with this Token no longer exists!")]
    UserNoLongerExists,
    #[error("Authentication required, Please login again.")]
    UserNotAuthenticated,

    #[error("Permission denide! : {action}")]
    PermissionDenied { action: String },
    #[error("Password is too week {action}")]
    WeakPassword { action: String },
    #[error("Email already exists: {action}")]
    EmailExists { action: String },
    #[error("Password exceeds max length: {max}, password lenght : {actual}")]
    ExceededMaxPassLen { max: usize, actual: usize },
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            UserError::WrongCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            UserError::EmptyPassword => (StatusCode::BAD_REQUEST, self.to_string()),
            UserError::UserNotFound => (StatusCode::BAD_REQUEST, self.to_string()),
            UserError::UserNoLongerExists => (StatusCode::NOT_FOUND, self.to_string()),
            UserError::UserNotAuthenticated => (StatusCode::UNAUTHORIZED, self.to_string()),

            UserError::PermissionDenied { .. } => (StatusCode::FORBIDDEN, self.to_string()),
            UserError::WeakPassword { .. } => (StatusCode::BAD_REQUEST, self.to_string()),
            UserError::EmailExists { .. } => (StatusCode::CONFLICT, self.to_string()),
            UserError::ExceededMaxPassLen { .. } => (StatusCode::BAD_REQUEST, self.to_string()),
        };
        (status, Json(ErrorResponse { status: "Fail".to_string(), message, code: status.as_u16() })).into_response()
    }
}

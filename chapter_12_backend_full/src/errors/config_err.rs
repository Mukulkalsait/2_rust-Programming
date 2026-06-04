use crate::errors::err_prelud::*;

#[derive(Debug, Deserialize, Error)]
pub enum SelfConfigError {
    #[error("Missing Envornment Variable: {var}")]
    MissingEnvVar { var: String },
    #[error("Invalid Key value pair Key:{key} | Value: {val}")]
    InvalidValue { key: String, val: String },
}

impl IntoResponse for SelfConfigError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            SelfConfigError::MissingEnvVar { .. } => (StatusCode::NOT_FOUND, self.to_string()),
            SelfConfigError::InvalidValue { .. } => (StatusCode::NOT_ACCEPTABLE, self.to_string()),
        };
        (status, Json(ErrorResponse { status: "Fail".to_string(), message, code: status.as_u16() })).into_response()
    }
}

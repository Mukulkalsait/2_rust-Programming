use crate::errors::err_prelud;

#[derive(Debug, err_prelud::Serialize, err_prelud::Deserialize, err_prelud::Error)]
pub enum DbError {
    #[error("Database Connection Faild: {err_source}")]
    ConnectionError { err_source: String },
    #[error("Query Faild: {err_source}")]
    QueryError { err_source: String },
    #[error("Unique Constrain Viloation on string: {err_source}")]
    UniqueViolation { err_source: String },
}

impl err_prelud::IntoResponse for DbError {
    fn into_response(self) -> err_prelud::Response {
        let (status, message) = match self {
            DbError::ConnectionError { .. } => (err_prelud::StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            DbError::QueryError { .. } => (err_prelud::StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            DbError::UniqueViolation { .. } => (err_prelud::StatusCode::CONFLICT, self.to_string()),
            //  DbError::UniqueViolation { err_source } => (err_prelud::StatusCode::CONFLICT, format!("Unique Constrain Viloation on {}", err_source)), Y: Previously.
        };
        (status, err_prelud::Json(err_prelud::ErrorResponse { status: "Fail".to_string(), message, code: status.as_u16() })).into_response()
    }
}

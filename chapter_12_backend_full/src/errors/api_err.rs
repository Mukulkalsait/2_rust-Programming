// FILe : /src/errors/api_err.rs
use crate::errors::{auth_err::AuthError, config_err::SelfConfigError, db_err::DbError, err_prelud::*, payment_err::PaymentError, user_err::UserError};

#[derive(Debug)]
pub enum ApiError {
    Auth(AuthError),
    Db(DbError),
    Payment(PaymentError),
    SelfConfig(SelfConfigError),
    User(UserError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Auth(err) => err.into_response(),
            ApiError::Db(err) => err.into_response(),
            ApiError::Payment(err) => err.into_response(),
            ApiError::SelfConfig(err) => err.into_response(),
            ApiError::User(err) => err.into_response(),
        }
    }
}

impl From<AuthError> for ApiError {
    fn from(err: AuthError) -> Self { ApiError::Auth(err) }
}

impl From<DbError> for ApiError {
    fn from(err: DbError) -> Self { ApiError::Db(err) }
}

impl From<PaymentError> for ApiError {
    fn from(err: PaymentError) -> Self { ApiError::Payment(err) }
}
impl From<SelfConfigError> for ApiError {
    fn from(err: SelfConfigError) -> Self { ApiError::SelfConfig(err) }
}

impl From<UserError> for ApiError {
    fn from(err: UserError) -> Self { ApiError::User(err) }
}

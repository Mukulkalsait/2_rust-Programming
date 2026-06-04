// file: /src/errors/mod.rs

// Y: Prelude with dependencies and Struct.
pub mod err_prelud;

// Y: Error Structs + Imples
pub mod auth_err;
pub mod config_err;
pub mod db_err;
pub mod payment_err;
pub mod user_err;

// Y: FINAL API ERROR.
pub mod api_err;

// G: Usage:
pub use api_err::ApiError;
pub use auth_err::AuthError;
pub use config_err::SelfConfigError;
pub use db_err::DbError;
pub use payment_err::PaymentError;
pub use user_err::UserError;

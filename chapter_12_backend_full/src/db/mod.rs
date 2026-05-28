// FILE: /src/db/mod.rs
pub mod connection;
pub use connection::DBClient;

pub mod db_macros;
pub use db_macros::use_of_macros;

pub mod users;
pub use users::user_modle::{User, UserRole};

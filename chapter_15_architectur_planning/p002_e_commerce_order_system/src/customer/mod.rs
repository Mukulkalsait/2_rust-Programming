// ===== src/customer/mod.rs =====

pub mod errors;
pub mod model;
pub mod service;

pub use model::{Address, AddressID, Customer, CustomerID, CustomerStatus, LoyaltyTier};

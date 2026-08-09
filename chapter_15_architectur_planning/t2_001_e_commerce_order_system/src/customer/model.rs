// ===== src/customer/model.rs =====

use chrono::{DateTime, Utc};

//----------------------------------------------------------------------------
pub type AddressID = uuid::Uuid;

#[derive(Debug, Clone)]
/// Struct:
/// address for all (users, busnesses, wareshouses)
pub struct Address {
    pub id: AddressID,
    pub is_default: bool,

    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Enum:
/// loyality of user
pub enum LoyaltyTier {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

pub type CustomerID = uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Enum:
///
pub enum CustomerStatus {
    Active,
    Suspended,
    Inactive,
}

#[derive(Debug, Clone)]
/// Struct:
/// user struct
pub struct Customer {
    pub id: CustomerID,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub hashed_pass: String, // bcrypt hash

    pub email_verified: bool,
    pub phone_verified: bool,

    pub addresses: Vec<Address>,
    pub loyality_tear: LoyaltyTier,
    pub status: CustomerStatus,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

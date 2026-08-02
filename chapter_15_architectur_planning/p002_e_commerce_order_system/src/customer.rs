// customer.rs

use chrono::{DateTime, Utc};

//----------------------------------------------------------------------------
pub type CustomerID = uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Enum:
/// loyality of user
pub enum LoyaltyTire {
    Bronz,
    Silver,
    Gold,
    Platinum,
}

#[derive(Debug, Clone)]
/// Struct:
/// user struct
pub struct Customer {
    pub id: CustomerID,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub hashed_pass: String,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub address: Vec<Address>,
    pub loyality_tear: LoyaltyTire,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
/// Struct:
/// address for all (users, busnesses, wareshouses)
pub struct Address {
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub is_default: bool,
}

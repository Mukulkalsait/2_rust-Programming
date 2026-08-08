use chrono::Utc;
use uuid::Uuid;

use super::{Address, Customer, CustomerID, CustomerStatus, LoyaltyTier};

impl Customer {
    pub fn new(name: String, email: String, phone: String, hashed_pass: String) -> Self {
        Customer {
            id: Uuid::new_v4(),
            name,
            email,
            phone,
            hashed_pass,

            email_verified: false,
            phone_verified: false,

            addresses: Vec::new(),
            loyality_tear: LoyaltyTier::Bronze,
            status: CustomerStatus::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

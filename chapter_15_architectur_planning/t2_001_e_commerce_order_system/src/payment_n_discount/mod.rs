// payment_n_discount/mod.rs

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use crate::order_n_cart::OrderID;
//----------------------------------------------------------------------------
//
//----------------------------------------------------------------------------
pub type PaymentID = uuid::Uuid;
pub type DiscountID = uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaymentMethod {
    CreditCard { last_four: String, band: String },
    UPI { upi_id: String, provider: String },
    NetBanking { bank: String },
    Wallet { wallet_id: String, provider: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentStatus {
    Pending,
    Success,
    Fail,
    Refund,
}

#[derive(Debug, Clone)]
/// Struct:
pub struct PaymentTransation {
    pub id: PaymentID,
    pub order_id: OrderID,
    pub method: PaymentMethod,
    pub amount: Decimal,
    pub status: PaymentStatus,
    pub transation_reference: String,
    pub processing_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy)]

/// Struct:
///| DiscountType | DiscountApplication | Example |
///| :--- | :--- | :--- |
///| `Percentage { percentage: 10, max_discount: Some(100) }` | `Global` | "SAVE10" → 10% off entire order, max ₹100 |
///| `Flat { amount: 50 }` | `ProductSpecific { product_id }` | "SHOE50" → ₹50 off shoes only |
///| `FreeShipping` | `Global` | "FREESHIP" → Free delivery |
pub enum DiscountType {
    Percentage { percentage: f64, max_discount: Option<Decimal> },
    Flat { amount: Decimal },
    FreeShipping,
}

#[derive(Debug, Clone, Copy)]
pub enum DiscountApplication {
    Global,
    ProductSpecific { product_id: PaymentID },
}

#[derive(Debug, Clone)]
pub struct Discount {
    pub id: DiscountID,
    pub code: String,
    pub discount_type: DiscountType,
    pub application: DiscountApplication,
    pub valid_from: DateTime<Utc>,
    pub valid_till: DateTime<Utc>,
    pub max_uses: Option<u32>,
    pub used_count: u32,
    pub stackable: bool,
    pub priority: u8,
}

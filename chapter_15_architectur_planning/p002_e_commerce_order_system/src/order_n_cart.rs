// order.rs

use chrono::{DateTime, Utc};
use rust_decimal::{Decimal, prelude::FromPrimitive};

use crate::{CustomerID, DiscountType, payment_n_discount::Discount, product::ProductID};

//----------------------------------------------------------------------------
// Why Separate CartItem and OrderItem?
// Aspect	CartItem	OrderItem
// Purpose	Temporary selection	Permanent record
// Price	Uses current product price	Snapshot of price at purchase
// Quantity	Can be changed	Fixed
// Stock	Reserved (temporary)	Deducted (permanent)
// Discount	Applied later	Applied at checkout
//----------------------------------------------------------------------------

pub type OrderID = uuid::Uuid;
pub type OrderItemID = uuid::Uuid;
pub type CartID = uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Paid,
    Shipped,
    Delivered,
    Canceled,
    Refunded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderCancelationReason {
    CustomerRequest,
    PaymentFaild,
    ProductDiscontinued,
    OutOfStock,
}

#[derive(Debug, Clone)]
/// Struct:
///
///
///
pub struct Order {
    id: OrderID,
    customer_id: CustomerID,
    status: OrderStatus,
    applied_discount: Vec<Discount>,
    total_amount: Decimal,
    final_amount: Decimal,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
/// Struct:
pub struct OrderItem {
    id: OrderItemID,
    order_id: OrderID,
    product_id: ProductID,
    quantity: u32,
    price_at_time: Decimal,
}

impl Order {
    fn calculate_final_amount(&mut self) -> Decimal {
        let mut amount = self.total_amount;
        let mut sorted_discount = self.applied_discount.clone();
        sorted_discount.sort_by_key(|d| d.priority);

        for discount in sorted_discount {
            if !discount.stackable && self.applied_discount.len() > 1 {
                continue;
            }
            amount = match discount.discount_type {
                DiscountType::Percentage { percentage, max_discount } => {
                    let discount_amount = amount * Decimal::from_f64_retain(percentage / 100.0).unwrap();
                    let final_discount = max_discount.map_or(discount_amount, |max| discount_amount.min(max));
                    amount - final_discount
                }
                DiscountType::Flat { amount: flat_amount } => amount - flat_amount,
                DiscountType::FreeShipping => amount,
            };
        }
        self.final_amount = amount;
        amount
    }
}
// B: ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
///# Struct: Cart
///  1. Cart is TEMPERORY, you can create, use distroy.
///  2. unlike order it created once it will remain forever.
///  3. NO payment it attached to it.
///  4. No shipping address attached to it.
///
/// > Cart + CartItems
pub struct Cart {
    id: CartID,
    customer_id: CustomerID,
    items: Vec<CartItem>,
    reserverd_util: DateTime<Utc>,
}

#[derive(Debug, Clone)]
/// Struct:
pub struct CartItem {
    product_id: ProductID,
    quantity: u32,
    reserver: u32,
}

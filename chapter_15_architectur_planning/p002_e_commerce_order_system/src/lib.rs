// lib.rs

pub mod customer;
pub mod errors;
pub mod order_n_cart;
pub mod payment_n_discount;
pub mod product;

use crate::customer::*;
use crate::order_n_cart::*;
use crate::payment_n_discount::*;
use crate::product::*;

#[derive(Debug, Clone)]
/// Struct:
pub struct Inoventry {
    product: Vec<Product>,
}

#[derive(Debug, Clone)]
/// Struct:
pub struct OrderManager {
    orders: Vec<Order>,
    order_items: Vec<OrderItem>,
}

#[derive(Debug, Clone)]
/// Struct:
pub struct Store {
    inovernty: Inoventry,
    order_manager: OrderManager,
    customer: Vec<Customer>,
    payments: Vec<PaymentTransation>,
}

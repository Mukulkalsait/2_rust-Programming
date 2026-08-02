use std::fmt::write;

use serde::de::Error;

use crate::{
    order_n_cart::OrderID,
    product::{ProductID, ProductVarient},
};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Enum: Product Specific Errors.
pub enum ProductErrors {
    NotFound,
    InsufficientStock { mess: String },
    OutOfStock { mess: String, id: Option<ProductID> },
    Discoutinued { mess: String, id: Option<ProductID> },
    VarientNotFound { mess: String, id: Option<ProductID>, var: Option<ProductVarient> },
    VarientAlreadyExists { mess: String, id: Option<ProductID>, var: Option<ProductVarient> },
    InvalidPrice { mess: String, order_id: Option<OrderID> },
    InvalidQuantity { mess: String, order_id: Option<OrderID> },
    ProductNotAvailable { mess: String, product: Option<ProductID> },
}

impl std::fmt::Display for ProductErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductErrors::NotFound => write!(f, "Not Found"),
            ProductErrors::InsufficientStock { mess } => write!(f, "InsufficientStock: {}", mess),
            ProductErrors::OutOfStock { mess, id } => {
                if !id.is_none() {
                    write!(f, "product {} out of Stock: {}", id.unwrap(), mess)
                } else {
                    write!(f, "out of Stock: {}", mess)
                }
            }
            ProductErrors::Discoutinued { mess, id } => {
                if !id.is_none() {
                    write!(f, "product {} discontinued: {}", id.unwrap(), mess)
                } else {
                    write!(f, "discontinued: {}", mess)
                }
            }
            ProductErrors::VarientNotFound { mess, id, var } => {
                if !id.is_none() && !var.is_none() {
                    let x = var.as_ref().unwrap();
                    write!(f, "product {} varient not found: {} | {}", id.unwrap(), x, mess)
                } else if !id.is_none() && var.is_none() {
                    write!(f, "product {} varient not found: {}", id.unwrap(), mess)
                } else if id.is_none() && !var.is_none() {
                    let x = var.as_ref().unwrap();
                    write!(f, "varient not found: {} | {}", x, mess)
                } else {
                    write!(f, "varient not found: {} ", mess)
                }
            } // ProductErrors::VarientAlreadyExists { var }
            ProductErrors::VarientAlreadyExists { mess, id, var } => {
                if !id.is_none() && !var.is_none() {
                    let x = var.as_ref().unwrap();
                    write!(f, "product {} varient already exists: {} | {}", id.unwrap(), x, mess)
                } else if !id.is_none() && var.is_none() {
                    write!(f, "product {} varient already exists: {}", id.unwrap(), mess)
                } else if id.is_none() && !var.is_none() {
                    let x = var.as_ref().unwrap();
                    write!(f, "varient already exists: {} | {}", x, mess)
                } else {
                    write!(f, "varient already exists: {} ", mess)
                }
            }

            ProductErrors::InvalidPrice { mess, order_id } => {
                if !order_id.is_none() {
                    write!(f, "order: {}, has invalid price: {}", order_id.unwrap(), mess)
                } else {
                    write!(f, "has invalid price: {}", mess)
                }
            }

            ProductErrors::InvalidQuantity { mess, order_id } => {
                if !order_id.is_none() {
                    write!(f, "order: {}, has invalid quantity: {}", order_id.unwrap(), mess)
                } else {
                    write!(f, "has invalid quantity: {}", mess)
                }
            }

            ProductErrors::ProductNotAvailable { mess, product } => {
                if !product.is_none() {
                    write!(f, "Product {},  not available: {}", product.unwrap(), mess)
                } else {
                    write!(f, " not available: {}", mess)
                }
            }
        }
    }
}

impl std::error::Error for ProductErrors {}
pub type Result<T> = std::result::Result<T, ProductErrors>;

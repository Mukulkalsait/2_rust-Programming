// ===== src/products/errors.rs =====
use crate::order_n_cart::OrderID;
use crate::products::model::{ProductID, VariantID};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Enum: Product Specific Errors.
pub enum ProductErrors {
    UnAvailalbe,
    NotFound,
    OutOfStock { id: Option<ProductID> },
    Discoutinued { id: Option<ProductID> },
    VariantAlreadyExists { var: Option<VariantID> },
    InvalidPrice { mess: String, price: Option<i32> },
    VariantNotFound { mess: String, id: Option<ProductID> },
    InsufficientStock { requested: u32, avialable: u32 },
    InvalidQuantity { mess: String, order_id: Option<OrderID> },
    ProductNotAvailable { mess: String, product: Option<ProductID> },
}

impl std::fmt::Display for ProductErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductErrors::UnAvailalbe => write!(f, "UnAvailalbe"),
            ProductErrors::NotFound => write!(f, "Not Found"),
            ProductErrors::InsufficientStock { requested, avialable } => {
                write!(f, "Not Enough Stock avialable. Requirement:{}, Avialable:{}", requested, avialable)
            },
            ProductErrors::OutOfStock { id } => {
                if !id.is_none() {
                    write!(f, "product {} out of Stock.", id.unwrap())
                } else {
                    write!(f, "Product out of stock")
                }
            },
            ProductErrors::Discoutinued { id } => {
                if !id.is_none() {
                    write!(f, "Product {} Discontinued.", id.unwrap())
                } else {
                    write!(f, "Product Discontinued.")
                }
            },
            ProductErrors::VariantNotFound { mess, id } => {
                if !id.is_none() {
                    write!(f, "product {} variant not found: {}", id.unwrap(), mess)
                } else {
                    write!(f, "variant not found: {} ", mess)
                }
            }, // ProductErrors::VariantAlreadyExists { var }
            ProductErrors::VariantAlreadyExists { var } => {
                if !var.is_none() {
                    write!(f, "product {} variant already exists.", var.unwrap())
                } else {
                    write!(f, "variant already exists.")
                }
            },

            ProductErrors::InvalidPrice { mess, price } => {
                write!(f, "order: {}, has invalid price: {}", mess, price.unwrap())
            },

            ProductErrors::InvalidQuantity { mess, order_id } => {
                if !order_id.is_none() {
                    write!(f, "order: {}, has invalid quantity: {}", order_id.unwrap(), mess)
                } else {
                    write!(f, "has invalid quantity: {}", mess)
                }
            },

            ProductErrors::ProductNotAvailable { mess, product } => {
                if !product.is_none() {
                    write!(f, "Product {},  not available: {}", product.unwrap(), mess)
                } else {
                    write!(f, " not available: {}", mess)
                }
            },
        }
    }
}

impl std::error::Error for ProductErrors {}

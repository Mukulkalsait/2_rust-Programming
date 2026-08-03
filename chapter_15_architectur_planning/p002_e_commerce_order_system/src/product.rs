// product.rs

use crate::errors::{GeneralErrRes, ProductErrRes, ProductErrors};
use chrono::{DateTime, Utc, format::Item::Error};
use rust_decimal::{Decimal, prelude::Zero};

//----------------------------------------------------------------------------
pub type ProductID = uuid::Uuid;
pub type VarientID = uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductCategory {
    Electronics,
    Grocery,
    Clothing,
    Books,
    Others,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductStatus {
    Available,
    OutOfStock,
    InTransit,
    Discontinued,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DialSize {
    _32Nm,
    _38Nm,
    _42Nm,
    _44Nm,
    _48Nm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sizes {
    XS,
    S,
    M,
    L,
    XL,
    XXL,
    Shoe(u8),
    WatchDial(DialSize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Color {
    Red,
    Blue,
    Green,
    Black,
    Yellow,
    Orange,
    Costume(String),
}

#[derive(Debug, Clone)]
/// Struct:
///
pub struct Product {
    pub id: ProductID,
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub category: ProductCategory,
    pub status: ProductStatus,
    pub varient: Vec<ProductVarient>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Product {
    pub fn new(self, name: String, description: String, price: Decimal) -> Self {
        Product {
            id: uuid::Uuid::new_v4(),
            name,
            description,
            price,
            category: ProductCategory::Electronics,
            status: ProductStatus::Available,
            varient: vec![],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
    pub fn update_price(&mut self, price: Decimal) -> GeneralErrRes<()> {
        self.price = price;
        Ok(())
    }
    pub fn update_stock(&mut self, stock_quantity: u32, varient_id: VarientID) -> ProductErrRes<()> {
        let res = self.varient.iter_mut().find(|v| v.id == varient_id).ok_or_else(|| ProductErrors::VarientNotFound {
            mess: "Varient Not avialable".to_string(),
            id: Some(self.id),
            var: Some(varient_id),
        })?;
        res.stock_quantity = stock_quantity;
        self.updated_at = chrono::Utc::now();
        Ok(())
    }

    pub fn update_status(&mut self, status: ProductStatus) -> GeneralErrRes<()> {
        self.status = status;
        Ok(())
    }
    pub fn get_varient(&mut self, varient_id: VarientID)
    pub fn is_available(&self, varient_id: VarientID)-> bool{
        self.
    }
    // pub fn is_available(&self) -> GeneralErrRes<(bool, Option<u32>)> {
    //     if !self.stock_quantity.is_zero() { Ok((true, Some(self.stock_quantity))) } else { Ok((false, None)) }
    // }
    // pub fn can_fulfil(&self, stock_quantity: u32, requirement: u32) -> ProductErrRes<bool> {
    //     if !self.stock_quantity.is_zero() && stock_quantity >= requirement {
    //         Ok(true)
    //     } else if !self.stock_quantity.is_zero() && stock_quantity < requirement {
    //         Ok(false)
    //     } else {
    //         Err(ProductErrors::InsufficientStock { mess: "Not Sufficient Stock".to_string() })
    //     }
    // }
}

// B: ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductAttributes {
    name: String,
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// ## Struct: Product Attributes & Variants
/// ### Why ProductAttribute and ProductVariant?
/// **Problem**: Products come in multiple variations (size, color, material).
/// ```
/// T-Shirt (Product)
/// ├── Variant: Red XL (SKU: TSHIRT-RED-XL)
/// ├── Variant: Red M  (SKU: TSHIRT-RED-M)
/// └── Variant: Blue XL (SKU: TSHIRT-BLUE-XL)
/// ```
pub struct ProductVarient {
    id: VarientID,
    product_id: ProductID,
    socket_keeing_unit: String,
    attributes: Vec<ProductAttributes>,
    size: Option<Sizes>,
    color: Option<Color>,
    price_adjuctement: Decimal,
    stock_quantity: u32,
    recoder_threshold: u32,
}

impl std::fmt::Display for ProductVarient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VarientID:{},\nProductID:{},\nSUK:{},\nSize:{},\nColor:{},\nPriceAdjuctment:{},\nStock:{}",
            self.id,
            self.product_id,
            self.socket_keeing_unit,
            self.size.as_ref().map_or_else(|| "N/A".to_string(), |s| format!("{:?}", s)),
            self.color.as_ref().map_or_else(|| "N/A".to_string(), |s| format!("{:?}", s)),
            self.price_adjuctement,
            self.stock_quantity,
        )
    }
}

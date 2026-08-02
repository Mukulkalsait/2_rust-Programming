// product.rs

use rust_decimal::Decimal;

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

#[derive(Debug, Clone)]
/// Struct:
///
pub struct Product {
    id: ProductID,
    name: String,
    description: String,
    price: Decimal,
    category: ProductCategory,
    status: ProductStatus,
    stock_quantity: u32,
    recoder_threshold: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductAttributes {
    name: String,
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductVarient {
    id: VarientID,
    product_id: ProductID,
    socket_keeing_unit: String,
    attributes: Vec<ProductAttributes>,
    size: Option<Sizes>,
    color: Option<Color>,
    price_adjuctement: Decimal,
    stock_quantity: u32,
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

// sepcific type -------------------------------------------------------------------

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

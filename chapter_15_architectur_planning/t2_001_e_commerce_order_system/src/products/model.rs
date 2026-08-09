// ===== src/products/model.rs =====
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

//----------------------------------------------------------------------------
pub type ProductID = uuid::Uuid;
pub type VariantID = uuid::Uuid;

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
    _32mm,
    _38mm,
    _42mm,
    _44mm,
    _48mm,
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
    // base price
    pub base_price: Decimal,
    pub description: String,
    pub category: ProductCategory,
    pub status: ProductStatus,
    pub variant: Vec<ProductVariant>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// B: ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductAttributes {
    pub name: String,
    pub value: String,
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
pub struct ProductVariant {
    pub id: VariantID,
    pub product_id: ProductID,
    // socket keeping usint
    pub sku: String,
    pub attributes: Vec<ProductAttributes>,
    pub size: Option<Sizes>,
    pub color: Option<Color>,
    // specific price for variant extra
    pub price_adjustment: Decimal,
    pub stock_quantity: u32,
    pub recoder_threshold: u32,
}

impl std::fmt::Display for ProductVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VariantID:{},\nProductID:{},\nSUK:{},\nSize:{},\nColor:{},\nPriceAdjuctment:{},\nStock:{}",
            self.id,
            self.product_id,
            self.sku,
            self.size.as_ref().map_or_else(|| "N/A".to_string(), |s| format!("{:?}", s)),
            self.color.as_ref().map_or_else(|| "N/A".to_string(), |s| format!("{:?}", s)),
            self.price_adjustment,
            self.stock_quantity,
        )
    }
}

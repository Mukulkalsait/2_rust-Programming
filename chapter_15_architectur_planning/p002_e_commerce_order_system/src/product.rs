// product.rs

use std::vec;

use crate::errors::{GeneralErrRes, ProductErrRes, ProductErrors};
use chrono::{DateTime, Utc, format::Item::Error};
use rust_decimal::{
    Decimal,
    prelude::{ToPrimitive, Zero},
};
use uuid::Uuid;

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
    pub description: String,
    pub category: ProductCategory,
    pub status: ProductStatus,
    pub varient: Vec<ProductVarient>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Product {
    // Y: CREATE: ----------------------------------------------------------------
    pub fn new(self, name: String, description: String, category: ProductCategory, price: Decimal) -> Self {
        Product {
            id: uuid::Uuid::new_v4(),
            name,
            description,
            category,
            status: ProductStatus::Available,
            varient: vec![],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
    pub fn add_varient(&mut self, varient: ProductVarient) -> ProductErrRes<()> {
        let res = self.search_varient(varient.id);

        if let Ok(varient) = res {
            Err(ProductErrors::VarientAlreadyExists {
                mess: "The provided varient allready exists".to_string(),
                id: Some(res.unwrap().id),
                var: Some(varient.id),
            })
        } else {
            self.varient.push(varient);
            self.updated_at = chrono::Utc::now();
            Ok(())
        }
    }

    // Y: get , Find & Verify: --------------------------------------------------------------------
    //

    /// all stack is geting sum.
    pub fn get_total_stock(&self) -> Option<u32> { Some(self.varient.iter().map(|v| v.stock_quantity).sum()) }

    /// avaialbllity checking.
    pub fn is_available(&self) -> bool { matches!(self.status, ProductStatus::Available) && self.get_total_stock().unwrap() > 0 }

    pub fn search_varient(&self, varient_id: VarientID) -> ProductErrRes<&ProductVarient> {
        self.varient.iter().find(|v| v.id == varient_id).ok_or_else(|| ProductErrors::VarientNotFound {
            mess: "Varient Id not matched".to_string(),
            id: Some(varient_id),
            var: None,
        })
    }
    pub fn search_varient_mut(&mut self, varient_id: VarientID) -> ProductErrRes<&mut ProductVarient> {
        self.varient.iter_mut().find(|v| v.id == varient_id).ok_or_else(|| ProductErrors::VarientNotFound {
            mess: "Varient Id not matched".to_string(),
            id: Some(varient_id),
            var: None,
        })
    }

    pub fn get_varient(&mut self, varient_id: Option<VarientID>) -> ProductErrRes<&mut ProductVarient> { self.search_varient_mut(varient_id.unwrap()) }

    pub fn get_varient_by_attributes(&mut self, attribute: &[ProductAttributes]) -> ProductErrRes<&mut ProductVarient> {
        self.varient.iter_mut().find(|v| v.matches_attributes(attribute)).ok_or_else(|| ProductErrors::NotFound)
    }

    pub fn can_order_fulfil(&self, varient_id: VarientID, requirements: i32) -> ProductErrRes<bool> {
        match self.status {
            ProductStatus::OutOfStock | ProductStatus::InTransit => {
                Err(ProductErrors::OutOfStock { mess: "Product Out of stock".to_string(), id: Some(self.id) })
            }
            ProductStatus::Discontinued => Err(ProductErrors::Discoutinued { mess: "Product is Discontinued".to_string(), id: Some(self.id) }),
            ProductStatus::Available => {
                let res = self.search_varient(varient_id)?;
                if let Some(quantity) = res.stock_quantity.to_u32() {
                    if quantity >= requirements as u32 {
                        return Ok(true);
                    } else {
                        return Err(ProductErrors::InsufficientStock { requested: requirements, avialable: quantity });
                    }
                }
                Ok(false)
            }
        }
    }

    // Y: UPDATE: ------------------------------------------------------------------------------------

    pub fn update_price(&mut self, varient_id: VarientID, price: Decimal) -> ProductErrRes<()> {
        if matches!(self.get_varient(Some(varient_id)).unwrap().price, Decimal::ZERO) {
            return Err(ProductErrors::InvalidPrice { mess: "Provided Price is invalid".to_string(), price: price.to_i32().unwrap() });
        };
        let res = self.search_varient_mut(varient_id)?;
        res.price = price;
        self.updated_at = chrono::Utc::now();
        Ok(())
    }

    pub fn update_status(&mut self, new_status: ProductStatus) -> ProductErrRes<()> {
        match (self.status, new_status) {
            (ProductStatus::Discontinued, _) => return Err(ProductErrors::Discoutinued { mess: "Product is discontinued ".to_string(), id: None }),
            (ProductStatus::InTransit, ProductStatus::OutOfStock) => self.status = new_status,
            _ => self.status = new_status,
        }
        self.updated_at = chrono::Utc::now();
        Ok(())
    }

    pub fn update_varient_stock(&mut self, varient_id: VarientID, stock_change: i32) -> ProductErrRes<()> {
        let varient = self.search_varient_mut(varient_id)?;

        let final_stock = varient.stock_quantity as i32 + stock_change;
        if final_stock < 0 {
            return Err(ProductErrors::InsufficientStock { requested: stock_change, avialable: varient.stock_quantity });
        }

        varient.stock_quantity = final_stock as u32;
        self.updated_at = chrono::Utc::now();
        Ok(())
    }

    // pub fn update_stock(&mut self, varient_map:  ){
    // dont konw how to take a hasnmap of varientid and stokc quantity into attribute,
    // so that i can run u pdate_varient_stock function on each one of them as well as update the
    // overall stock of product stakc + use updte_status() funciton.
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
    pub id: VarientID,
    pub product_id: ProductID,
    pub price: Decimal,
    pub socket_keeing_unit: String,
    pub attributes: Vec<ProductAttributes>,
    pub size: Option<Sizes>,
    pub color: Option<Color>,
    pub price_adjuctement: Decimal,
    pub stock_quantity: u32,
    pub recoder_threshold: u32,
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

impl ProductVarient {
    pub fn new(self, product_id: ProductID, price: Decimal) -> Self {
        ProductVarient {
            id: Uuid::new_v4(),
            product_id,
            price,
            socket_keeing_unit: "test".to_string(),
            attributes: Vec::new(),
            size: None,
            color: None,
            price_adjuctement: Decimal::new(10, 1),
            stock_quantity: 1,
            recoder_threshold: 1,
        }
    }
    pub fn matches_attributes(&self, all_attribtes: &[ProductAttributes]) -> bool {
        all_attribtes.iter().all(|atter| self.attributes.iter().any(|v| v.name == atter.name))
    }
}

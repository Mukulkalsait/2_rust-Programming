// ===== src/products/service.rs =====
use chrono::Utc;
use rust_decimal::{Decimal, prelude::ToPrimitive};
use uuid::Uuid;

use super::ProductErrors;
use crate::products::model::*;

// B: ------------------------------------------------------------------

impl Product {
    // Y: Product: ----------------------------------------------------------------
    pub fn new(name: String, description: String, category: ProductCategory, price: Decimal) -> Self {
        Product {
            id: uuid::Uuid::new_v4(),
            name,
            base_price: price,
            description,
            category,
            status: ProductStatus::Available,
            variant: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    // Y: Variant: --------------------------------------------------------------------

    pub fn add_variant(&mut self, variant: ProductVariant) -> Result<(), ProductErrors> {
        if let Ok(var) = self.get_variant(variant.id) {
            Err(ProductErrors::VariantAlreadyExists { var: Some(var.id) })
        } else {
            self.variant.push(variant);
            self.updated_at = Utc::now();
            Ok(())
        }
    }
    pub fn get_variant(&self, variant_id: VariantID) -> Result<&ProductVariant, ProductErrors> {
        self.variant
            .iter()
            .find(|v| v.id == variant_id)
            .ok_or_else(|| ProductErrors::VariantNotFound { mess: "Variant Id not matched".to_string(), id: Some(variant_id) })
    }
    pub fn get_variant_mut(&mut self, variant_id: VariantID) -> Result<&mut ProductVariant, ProductErrors> {
        self.variant
            .iter_mut()
            .find(|v| v.id == variant_id)
            .ok_or_else(|| ProductErrors::VariantNotFound { mess: "Variant Id not matched".to_string(), id: Some(variant_id) })
    }
    pub fn get_variant_by_attributes(&self, attribute: &[ProductAttributes]) -> Result<&ProductVariant, ProductErrors> {
        self.variant.iter().find(|v| v.matches_attributes(attribute)).ok_or_else(|| ProductErrors::NotFound)
    }

    pub fn get_variant_by_attributes_mut(&mut self, attribute: &[ProductAttributes]) -> Result<&mut ProductVariant, ProductErrors> {
        self.variant.iter_mut().find(|v| v.matches_attributes(attribute)).ok_or_else(|| ProductErrors::NotFound)
    }

    // Y: Price: ------------------------------------------------------------------------------------

    pub fn validate_price(&self, price: Decimal) -> Result<(), ProductErrors> {
        if price <= Decimal::ZERO {
            return Err(ProductErrors::InvalidPrice { mess: "Price cannot be negative".to_string(), price: price.to_i32() });
        }
        Ok(())
    }
    pub fn update_variant_price_internally(&mut self, variant_id: VariantID, new_price: Decimal) -> Result<(), ProductErrors> {
        let base_price = self.base_price; // storing base price BorrowNused: self

        let varient = self.get_variant_mut(variant_id)?; // Borrow: self
        varient.price_adjustment = new_price - base_price; // Dead: self
        Ok(())
    }
    pub fn update_price(&mut self, variant_id: VariantID, new_price: Decimal) -> Result<(), ProductErrors> {
        self.validate_price(new_price)?;
        self.update_variant_price_internally(variant_id, new_price)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    // Y: Status: ------------------------------------------------------------------------------------

    pub fn update_status(&mut self, new_status: ProductStatus) -> Result<(), ProductErrors> {
        match (self.status, new_status) {
            (ProductStatus::Discontinued, _) => return Err(ProductErrors::Discoutinued { id: None }),
            (ProductStatus::InTransit, ProductStatus::OutOfStock) => self.status = new_status,
            _ => self.status = new_status,
        }
        self.updated_at = Utc::now();
        Ok(())
    }

    // Y: Stock: ------------------------------------------------------------------------------------

    /// all stack is geting sum.
    pub fn get_total_stock(&self) -> u32 { self.variant.iter().map(|v| v.stock_quantity).sum() }

    /// avaialbllity checking.
    pub fn is_available(&self) -> bool { matches!(self.status, ProductStatus::Available) && self.get_total_stock() > 0 }

    /// Stock change can be stock add , stock remove hence i32
    pub fn update_variant_stock(&mut self, variant_id: VariantID, stock_change: i32) -> Result<(), ProductErrors> {
        let variant = self.get_variant_mut(variant_id)?;

        let final_stock = variant.stock_quantity as i32 + stock_change;
        if final_stock < 0 {
            return Err(ProductErrors::InsufficientStock { requested: stock_change as u32, avialable: variant.stock_quantity });
        }

        variant.stock_quantity = final_stock as u32;
        self.updated_at = Utc::now();
        Ok(())
    }

    // Y: Order: ------------------------------------------------------------------------------------

    // only checking if order can be fulfiled, so u32 and not i32 (negitive for remvoal)
    pub fn can_order_fulfil(&self, variant_id: VariantID, requirements: u32) -> Result<bool, ProductErrors> {
        match self.status {
            ProductStatus::OutOfStock | ProductStatus::InTransit => Err(ProductErrors::OutOfStock { id: Some(self.id) }),
            ProductStatus::Discontinued => Err(ProductErrors::Discoutinued { id: Some(self.id) }),
            ProductStatus::Available => {
                let res = self.get_variant(variant_id)?;
                if let Some(quantity) = res.stock_quantity.to_u32() {
                    if quantity >= requirements {
                        return Ok(true);
                    } else {
                        return Err(ProductErrors::InsufficientStock { requested: requirements, avialable: quantity });
                    }
                }
                Ok(false)
            }
        }
    }
}

impl ProductVariant {
    pub fn new(product_id: ProductID, sku: String, attributes: Vec<ProductAttributes>, price_adjustment: Decimal, stock_quantity: u32) -> Self {
        ProductVariant { id: Uuid::new_v4(), product_id, sku, attributes, size: None, color: None, price_adjustment, stock_quantity, recoder_threshold: 1 }
    }
    /// both name and value verification
    pub fn matches_attributes(&self, all_attribtes: &[ProductAttributes]) -> bool {
        all_attribtes.iter().all(|atter| self.attributes.iter().any(|v| v.name == atter.name && v.value == atter.value))
    }
}

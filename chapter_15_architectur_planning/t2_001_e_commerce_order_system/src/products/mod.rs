// ===== src/products/mod.rs =====
mod errors;
mod model;
mod service;

// Re-export what's needed
pub use errors::ProductErrors;
pub use model::{Color, Product, ProductAttributes, ProductCategory, ProductID, ProductStatus, ProductVariant, Sizes, VariantID};

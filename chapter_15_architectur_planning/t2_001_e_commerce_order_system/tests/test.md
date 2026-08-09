#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_product() {
        let product = Product::new(
            "T-Shirt".to_string(),
            "100% Cotton".to_string(),
            ProductCategory::Clothing,
            Decimal::from(500),
        );
        assert_eq!(product.name, "T-Shirt");
        assert!(matches!(product.status, ProductStatus::Available));
    }

    #[test]
    fn test_add_variant() {
        let mut product = Product::new(
            "T-Shirt".to_string(),
            "100% Cotton".to_string(),
            ProductCategory::Clothing,
            Decimal::from(500),
        );

        let variant = ProductVariant::new(
            product.id,
            "TSHIRT-RED-M".to_string(),
            vec![
                ProductAttribute {
                    name: "Size".to_string(),
                    value: "M".to_string(),
                },
                ProductAttribute {
                    name: "Color".to_string(),
                    value: "Red".to_string(),
                },
            ],
            Decimal::ZERO,
            10,
        );

        product.add_variant(variant).unwrap();
        assert_eq!(product.variants.len(), 1);
    }

    #[test]
    fn test_can_fulfill() {
        let mut product = Product::new(
            "T-Shirt".to_string(),
            "100% Cotton".to_string(),
            ProductCategory::Clothing,
            Decimal::from(500),
        );

        let variant = ProductVariant::new(
            product.id,
            "TSHIRT-RED-M".to_string(),
            vec![],
            Decimal::ZERO,
            10,
        );

        product.add_variant(variant).unwrap();
        let variant_id = product.variants[0].id;

        // Should be able to fulfill 5 units
        assert!(product.can_fulfill(variant_id, 5).is_ok());

        // Should NOT be able to fulfill 15 units
        assert!(product.can_fulfill(variant_id, 15).is_err());
    }

    #[test]
    fn test_update_stock() {
        let mut product = Product::new(
            "T-Shirt".to_string(),
            "100% Cotton".to_string(),
            ProductCategory::Clothing,
            Decimal::from(500),
        );

        let variant = ProductVariant::new(
            product.id,
            "TSHIRT-RED-M".to_string(),
            vec![],
            Decimal::ZERO,
            10,
        );

        product.add_variant(variant).unwrap();
        let variant_id = product.variants[0].id;

        // Reduce stock
        product.update_variant_stock(variant_id, -3).unwrap();
        assert_eq!(product.variants[0].stock_quantity, 7);

        // Try to reduce too much
        let result = product.update_variant_stock(variant_id, -10);
        assert!(result.is_err());
    }
}

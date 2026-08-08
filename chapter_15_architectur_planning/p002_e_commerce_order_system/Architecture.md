ENTITIES:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

PRODUCT
├── id (unique)
├── name (string)
├── description (string)
├── base_price (decimal)
├── category (Electronics | Clothing | Books | Grocery | Others)
├── status (Available | OutOfStock | InTransit | Discontinued)
├── variants (list of ProductVariant)
├── created_at (date)
└── updated_at (date)

PRODUCT_VARIANT (SKU level)
├── id (unique)
├── product_id (reference to Product)
├── sku (unique identifier)
├── attributes (list of key-value pairs)
├── size (XS | S | M | L | XL | XXL | Shoe | WatchDial)
├── color (Red | Blue | Green | ...)
├── price_adjustment (decimal)
├── stock_quantity (number)
└── reorder_threshold (number)

CUSTOMER
├── id (unique)
├── name (string)
├── email (string)
├── phone (string)
├── hashed_password (bcrypt)
├── addresses (list of Address)
├── loyalty_tier (Bronze | Silver | Gold | Platinum)
├── status (Active | Suspended | Inactive)
├── email_verified (boolean)
├── phone_verified (boolean)
├── created_at (date)
└── updated_at (date)

ADDRESS
├── id (unique)
├── line1 (string)
├── line2 (optional)
├── city (string)
├── state (string)
├── country (string)
├── postal_code (string)
└── is_default (boolean)

CART (Temporary)
├── id (unique)
├── customer_id (reference)
├── items (list of CartItem)
├── reserved_until (datetime)
├── created_at (date)
└── updated_at (date)

CART_ITEM
├── product_id (reference)
├── quantity (number)
├── reserved_stock (number)
└── status (InCart | SavedForLater | Removed)

ORDER (Permanent)
├── id (unique)
├── customer_id (reference)
├── items (list of OrderItem)
├── status (Pending | Paid | Shipped | Delivered | Cancelled | Refunded)
├── applied_discounts (list of Discount)
├── total_amount (decimal - before discounts)
├── final_amount (decimal - after discounts)
├── shipping_address (Address)
├── created_at (date)
└── updated_at (date)

ORDER_ITEM (Snapshot)
├── id (unique)
├── order_id (reference)
├── product_id (reference)
├── quantity (number)
├── price_at_time (decimal - historical snapshot)
└── total (decimal)

PAYMENT_TRANSACTION
├── id (unique)
├── order_id (reference)
├── method (CreditCard | UPI | NetBanking | Wallet)
├── amount (decimal)
├── status (Pending | Success | Failed | Refunded)
├── transaction_reference (string)
└── processed_at (date)

DISCOUNT
├── id (unique)
├── code (string)
├── type (Percentage | Flat | FreeShipping)
├── application (Global | ProductSpecific)
├── valid_from (date)
├── valid_till (date)
├── max_uses (optional)
├── used_count (number)
├── stackable (boolean)
└── priority (number)

RELATIONSHIPS:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Product 1→M ProductVariant
Customer 1→M Address
Customer 1→M Order
Order 1→M OrderItem
Order 1→M PaymentTransaction
Order M→M Discount (through applied_discounts)
Cart 1→M CartItem
CartItem → ProductVariant

KEY INSIGHTS:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
- Cart is TEMPORARY, mutable, no payment/shipping
- Order is PERMANENT, immutable (except status)
- OrderItem stores price_at_time (historical snapshot)
- Price can change, but order item price is frozen at purchase
- CartItem stores product_id, OrderItem stores product_id + price
- ProductVariant is SKU-level (size/color specific)
- Inventory tracks stock per variant
- Store coordinates all subsystems (Inventory, OrderManager, Customers, Payments)

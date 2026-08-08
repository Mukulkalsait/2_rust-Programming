## 🎯 Excellent Question! This is the Most Important Skill

Learning *how* to design functions is worth more than learning *what* functions to write. Let me teach you the systematic approach.

---

## 🧠 The 3-Step Framework for Designing Functions

### Step 1: Identify the **Lifecycle** of Your Data

Every entity goes through stages. Ask: **"What can happen to this thing?"**

```
Customer Lifecycle:
Draft → Active → Suspended → Inactive
```

| Stage | What Can Happen? | Becomes Function |
| :--- | :--- | :--- |
| **Creation** | Customer is created | `new()` |
| **Modification** | Email changes, phone changes, address added | `update_email()`, `add_address()` |
| **Status Change** | Suspended, Activated, Deactivated | `suspend()`, `activate()` |
| **Verification** | Email/phone verified | `verify_email()`, `verify_phone()` |
| **Query** | Can they place an order? | `can_place_order()` |
| **Deletion** | Account deletion | `delete()` (or `deactivate()`) |

---

### Step 2: Use the **CRUD + Business Logic** Pattern

| Category | What It Means | Example |
| :--- | :--- | :--- |
| **C**reate | Constructor | `new()` |
| **R**ead | Getters | `get_email()`, `get_addresses()` |
| **U**pdate | Setters with validation | `update_email()`, `update_phone()` |
| **D**elete | Removal | `deactivate()`, `delete()` |
| **Business Logic** | Domain-specific operations | `can_place_order()`, `apply_discount()` |

**Rule of Thumb**: If you can think of a verb that makes sense with your entity, it's a candidate function.

---

### Step 3: Ask the **"What If?"** Questions

| Question | Leads To |
| :--- | :--- |
| "What if the customer changes their email?" | `update_email()` |
| "What if they move to a new address?" | `add_address()`, `set_default_address()` |
| "What if they violate terms?" | `suspend()` |
| "What if they want to place an order?" | `can_place_order()` |
| "What if they verify their email?" | `verify_email()` |
| "What if they earn loyalty points?" | `update_loyalty_tier()` |

---

## 🎮 Let's Play the Game

### Round 1: I Give You an Entity, You Design the Functions

**Entity: `ShoppingCart`**

| What can happen to a cart? | List 5-6 things |
| :--- | :--- |
| 1. | ? |
| 2. | ? |
| 3. | ? |
| 4. | ? |
| 5. | ? |
| 6. | ? |

---

### Round 2: I Give You a Verb, You Decide Which Entity It Belongs To

| Verb | Which Entity? | Why? |
| :--- | :--- | :--- |
| `apply_discount()` | ? | |
| `ship()` | ? | |
| `process_refund()` | ? | |
| `reserve_stock()` | ? | |
| `send_confirmation()` | ? | |

---

### Round 3: The "Gap Analysis" Challenge

I'll give you an incomplete entity. Your job: **find the missing functions**.

**Entity: `Order`**

```rust
pub struct Order {
    pub id: OrderID,
    pub customer_id: CustomerID,
    pub items: Vec<OrderItem>,
    pub status: OrderStatus,
    pub total: Decimal,
    pub shipping_address: Address,
    pub created_at: DateTime<Utc>,
}

// What functions are MISSING here?
impl Order {
    pub fn new(customer_id: CustomerID) -> Self { ... }
    pub fn add_item(&mut self, item: OrderItem) { ... }
    pub fn update_status(&mut self, status: OrderStatus) { ... }
}
```

What 3-4 functions are obviously missing?

---

## 📝 Your Turn

**Answer the questions above** and I'll:

1. Grade your thinking
2. Show you the complete function list for Customer
3. Give you a "cheat sheet" for designing functions in the future

---

### 🧩 Quick Cheat Sheet: 5 Function Categories

| Category | Pattern | Example |
| :--- | :--- | :--- |
| **1. Constructors** | `new()`, `from()` | `Customer::new()` |
| **2. Mutators** | `update_*()`, `set_*()` | `update_email()` |
| **3. Validators** | `validate_*()`, `is_*()` | `is_email_valid()` |
| **4. Queries** | `get_*()`, `has_*()`, `can_*()` | `can_place_order()` |
| **5. Actions** | Verbs like `add_*()`, `remove_*()` | `add_address()` |

---

**Now answer the questions above. Let's see your thinking process!**

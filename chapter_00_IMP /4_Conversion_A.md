# Rust Ownership & Option Mental Model

## 0. First Principle

Whenever touching a value in Rust, ask:

```text
Do I want:
1. Ownership?
2. Read-only access?
3. Mutable access?
4. To transform the container?
```

Everything (`unwrap`, `?`, `as_ref`, `take`, etc.) exists to answer this.

---

# 1. Core Types

```text
&T        -> borrow
&mut T    -> mutable borrow
T         -> ownership / move
Option<T> -> container
```

```text
Option<T> = Some(T) | None
```

---

# 2. Access Decision Table

| Situation                       | Tool               | Meaning                    |
| --------------------------------| ------------------ | -------------------------- |
| Need value, panic if missing    | `unwrap()`         | Extract + panic            |
| Need better panic message       | `expect("msg")`    | Extract + custom panic     |
| Handle both cases manually      | `match`            | Full control               |
| Handle only success case        | `if let`           | Cleaner partial handling   |
| Propagate failure upward        | `?`                | Early return               |

---

# 3. Borrow vs Move

| You Have    | You Want         | Use         |
| ------------| ---------------- | ----------- |
| `Option<T>` | `Option<&T>`     | `.as_ref()` |
| `Option<T>` | `Option<&mut T>` | `.as_mut()` |
| `Option<T>` | ownership of `T` | `.take()`   |

---

# 4. `&` vs `.as_ref()`

## `&`

Wraps OUTSIDE.

```rust
&x
```

Example:

```rust
&Option<Box<Node>>
```

Reference to whole structure.

---

## `.as_ref()`

Transforms INSIDE container.

```rust
x.as_ref()
```

Example:

```rust
Option<Box<Node>>
-> Option<&Box<Node>>
```

### One-liner

```text
& wraps outside
as_ref() transforms inside
```

---

# 5. `unwrap()`

## Transformation

```text
Option<T> -> T
```

## Behavior

```text
Some(T) -> gives T
None    -> panic
```

## Meaning

```text
"I am 100% sure this exists"
```

## Common Uses

- tests
- prototypes
- guaranteed-valid cases

---

## Important

```rust
user.unwrap().name
```

VALID.

Why?

```text
unwrap() gives User
then .name accesses field
```

---

## Ownership Effect

```rust
user.unwrap()
```

consumes `user`.

After:

```rust
user.unwrap();
user.unwrap(); // ❌ moved already
```

---

# 6. `?` Operator

## Purpose

```text
Propagate failure + return early
```

## Example

```rust
let x = option?;
```

Equivalent to:

```rust
match option {
    Some(v) => v,
    None => return None,
}
```

---

## `unwrap()` vs `?`

| Tool       | Failure Behavior |
| -----------| ---------------- |
| `unwrap()` | panic            |
| `?`        | early return     |

---

## Important

`?` can ALSO move ownership.

Example:

```rust
let user = maybe_user?;
```

If:

```rust
maybe_user: Option<User>
```

Then:

```rust
user: User
```

owned value extracted.

---

# 7. `.as_ref().unwrap()`

This is VERY important.

```rust
user: Option<User>
```

Step 1:

```rust
user.as_ref()
```

becomes:

```text
Option<&User>
```

Step 2:

```rust
.unwrap()
```

becomes:

```text
&User
```

NOT `User`.

---

## Result

```rust
let x = user.as_ref().unwrap();
```

```text
x: &User
```

Only borrowed.

Original `user` still usable.

---

# 8. Ownership Flow in Chains

This is valid:

```rust
user.unwrap().name.len()
```

Because ownership can continue flowing through chain.

Move does NOT stop chaining.

---

# 9. `Some(...)`

Use ONLY when creating a new optional value.

Example:

```rust
Some(new_node)
```

Avoid unnecessary wrapping:

```rust
Some(existing_value) // often unnecessary
```

---

# 10. Pattern Matching

Main Rust weapon.

## Partial handling

```rust
if let Some(x) = value {
    // use x
}
```

## Full handling

```rust
match value {
    Some(x) => ...
    None => ...
}
```

---

# 11. Linked List Rules

## Traversal / Reading

Use:

```rust
.as_ref()
```

Reason:

```text
Need borrowing, not ownership
```

---

## Modification

Use:

```rust
&mut + .as_mut()
```

Reason:

```text
Need mutable access
```

---

## Removing Nodes

Use:

```rust
.take()
```

Reason:

```text
Need ownership to move node out
```

---

# 12. Mental Models

```text
Option<T> is a box
```

| Tool         | Mental Model                     |
| -------------| -------------------------------- |
| `.unwrap()`  | open box                         |
| `.as_ref()`  | peek inside box                  |
| `.take()`    | take item out, leave empty box   |
| `?`          | if empty, leave function early   |

---

# 13. Golden Rule

Always ask:

```text
"What exact type exists RIGHT NOW?"
```

Examples:

```text
Option<User>
-> unwrap()
-> User
```

```text
Option<User>
-> as_ref()
-> Option<&User>
-> unwrap()
-> &User
```

---

# 14. Biggest Lesson

Borrowing vs moving depends on:

```text
WHAT TYPE you currently have
```

NOT the function name alone.

---

# 15. Practical Coding Flow

While coding, constantly ask:

```text
Am I:
- reading?
- modifying?
- moving?
- borrowing?
```

Then choose tool accordingly.

---

# 16. Current Rust Progress

```text
syntax ✔
concepts ✔
type intuition ❌ -> currently building
```

This is the exact phase before Rust starts "clicking".

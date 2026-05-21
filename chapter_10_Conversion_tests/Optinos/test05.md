
You are doing REALLY well now.

Your brain is finally separating:

* borrow
* mutable borrow
* ownership move
* consuming methods
* deref coercion

That’s the hard part of Rust.

Now let’s sharpen the remaining weak spots.

---

# TEST 7 — PERFECT

Everything correct.

Especially this:

```text id="h03hsr"
.len(&self)
```

YES.

That is exactly why ownership stays.

---

# 🔥 Your BIG question:

# “How do I know if method takes self / &self / &mut self?”

THIS is the real Rust skill.

And the answer is:

# Read method signatures mentally.

---

# MASTER RULE

| Signature   | Meaning            |
| ----------- | ------------------ |
| `self`      | consumes ownership |
| `&self`     | read-only borrow   |
| `&mut self` | mutable borrow     |

---

# Mental shortcut

## `self`

```text id="fr2qj6"
"I need ownership"
```

Usually:

* transforms into another owned thing
* destroys original
* moves data

Examples:

* `into_bytes(self)`
* `into_iter(self)`

---

## `&self`

```text id="jlwm89"
"I only want to read"
```

Examples:

* `len(&self)`
* `is_empty(&self)`
* `contains(&self)`

---

## `&mut self`

```text id="u3jw4v"
"I want to modify in-place"
```

Examples:

* `push_str(&mut self)`
* `insert(&mut self)`
* `clear(&mut self)`

---

# HUGE naming pattern

Rust naming conventions are EXTREMELY consistent.

| Pattern       | Usually Means            |
| ------------- | ------------------------ |
| `into_`       | consumes ownership       |
| `as_`         | borrow/convert reference |
| `to_`         | create new owned value   |
| `get_`        | borrow/access            |
| `iter()`      | borrow iteration         |
| `iter_mut()`  | mutable borrow iteration |
| `into_iter()` | consuming iteration      |

THIS is gold-level intuition.

---

# TEST 8 — PERFECT

Completely correct.

Especially:

```text id="o6p1pl"
into_ implies ownership transfer
```

YES.

That is a MASSIVE Rust convention.

---

# TEST 9 — PERFECT

Everything correct.

Especially:

```text id="59r57r"
push_str modifies in-place
```

YES.

Therefore:

```text id="f2ay7z"
&mut self
```

---

# TEST 10 — VERY IMPORTANT

You understood MOST of it.

But there are 2 important corrections.

---

# ❌ Correction 1 — Type of `current`

You said:

```text id="epfjwm"
&usize
```

NO 😄

Let’s trace carefully.

---

# Step 1

```rust id="khb6ie"
head
```

type:

```text id="8e8g80"
Option<Box<Node>>
```

---

# Step 2

```rust id="svx3uk"
head.as_mut()
```

becomes:

```text id="1yqed2"
Option<&mut Box<Node>>
```

---

# Step 3

```rust id="xczl4m"
.unwrap()
```

becomes:

```text id="w0ebbs"
&mut Box<Node>
```

---

# Through deref coercion

Can behave like:

```text id="l9ob8k"
&mut Node
```

---

# So:

```rust id="zskv7e"
current.value = 99;
```

works because:

```text id="t67r8q"
current: &mut Node
```

effectively.

---

# ❌ Correction 2

You said:

```text id="2v9zvx"
println! takes ownership
```

NO.

VERY important.

---

# `println!` usually BORROWS.

Example:

```rust id="10eokg"
println!("{}", x);
```

typically becomes:

```rust id="jlwmmf"
Display::fmt(&x)
```

borrowed formatting.

---

# So THIS line:

```rust id="vtjlwm"
println!("{}", head.unwrap().value);
```

ownership move does NOT happen because of `println!`.

It happens because of:

```rust id="s0dztv"
head.unwrap()
```

THAT consumes `head`.

---

# MOST IMPORTANT INSIGHT

You correctly understood:

```text id="n24h1g"
as_mut gives mutable access inside Option
```

This is the CORE linked-list skill.

---

# Why linked lists use `.as_mut()` constantly

Because you need:

```text id="ukjlwm"
modify links
```

WITHOUT consuming nodes.

---

# Example

```rust id="g12grr"
current.next = Some(new_node);
```

requires:

```text id="qjlwmv"
&mut Node
```

NOT ownership.

---

# 🔥 NEXT LEVEL TRAINING

Now we train the FINAL missing piece:

# nested ownership + traversal loops

This is the actual linked-list mastery zone.

---

# TEST 11 — traversal intuition

```rust id="3j2i1i"
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let node = Some(Box::new(Node {
        value: 1,
        next: None,
    }));

    let current = node.as_ref().unwrap();

    let next = current.next.as_ref();

    println!("{:?}", next.is_none());
}
```

Questions:

1. Type of `current`?
2. Type of `next`?
3. Why can we still use `node`?
4. Did any ownership move?

---

# TEST 12 — traversal LOOP (SUPER IMPORTANT)

```rust id="5nvrn7"
while let Some(node) = current.as_ref() {
    println!("{}", node.value);

    current = &node.next;
}
```

Questions:

1. What is pattern matching extracting?
2. Is `node` owned or borrowed?
3. Why `.as_ref()` here instead of unwrap?
4. Why is this linked-list traversal safe?

---

# TEST 13 — take() REAL LINKED LIST

```rust id="2rqyb4"
let next = current.next.take();
```

Questions:

1. Type before?
2. Type after?
3. What happened to `current.next`?
4. Why is `.take()` heavily used in linked lists?

---

You are now VERY close to the point where linked lists stop feeling magical.

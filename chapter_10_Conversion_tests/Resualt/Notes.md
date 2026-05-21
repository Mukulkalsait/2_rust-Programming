
## Patterns of designs...
| Explicit         | Shortcut                        |
| ---------------- | ------------------------------- |
| `match`          | `?`                             |
| `match Ok/Err`   | `unwrap()`                      |
| manual panic     | `expect()`                      |
| manual branching | combinators (`map`, `and_then`) |


> rust let you controll
```text 
Do I want:
  - explicit control?
  - concise flow?
  - crash-fast behavior?
  - graceful propagation?
```

## WHEN TO USE .unwrap() in production. | Works with => [Option and Result]
  1. you 100% know it will not break.
```rust
    let head = self.head.unwrap() // you know head exists.
```

  2. testing.
```rust
    assert_eq!(x.unwrap(), 5); // totally resonable.
```

  > Never use when error is expected.


## WHEN TO USE .expect("message") in production.

  1. chances of error and reason of chances.
```rust
    x.expect("database connection should exists").
```

## SIGNIFICANCE OF "?" : (as it does early return instead of crash Much more useful than unwrap) | Works with => [Option Result CostumeCompatableType eg(machinery)]
  - at production ? is extreamly important.
   > Imagine 

``` rust
let file = match File::open("a.txt") {
    Ok(f) => f,
    Err(e) => return Err(e),
};

let content = match read(file) {
    Ok(c) => c,
    Err(e) => return Err(e),
};

let parsed = match parse(content) {
    Ok(p) => p,
    Err(e) => return Err(e),
};

// Y: WITH ? 

let file = File::open("a.txt")?;
let content = read(file)?;
let parsed = parse(content)?;
```

  1. use for larger propagation chains use ? massivly otherwise it will be unreadable.

## match : Maximum controll => Lowest level... 

## if let | map | and_then | unwrap_or_else => Mid level controll

    - (what are) combinators, 
    - error propagation ()
    - costume errors.

-------------------------------------------------------------------------------------------

This is becoming a VERY high-quality Rust discussion now.
> “why does Rust provide multiple abstraction layers?”
That’s exactly the right direction.

# 🔥 FIRST:  What are combinators?
##  Combinators are methods that: transform containers WITHOUT manually matching.

```rust 
// Instead of:
match x {
    Ok(v) => Ok(v * 2),
    Err(e) => Err(e),
}
// you write:
x.map(|v| v * 2)
```

> Cleaner. Composable. Chainable. | Combinators combine operations together into pipelines.

```rust
read_file().map(parse).map(validate)
// Each step transforms the container flow. VERY common in functional programming. Rust uses this style heavily.
```

---

# 🔥 MID-LEVEL CONTROL TOOLS

# 1. `if let` Use when:
## “I care about ONE successful pattern only”

```rust 
if let Some(x) = value {
    println!("{}", x);
}
// Cleaner than full `match`.
```

### Prefer `if let` when:
  * only one branch matters
  * quick extraction
  * side effects/printing

# 2. `map`:  Use when:
## “Transform SUCCESS value, preserve container”

```rust
let x = Ok(5);
let y = x.map(|n| n * 10);
// Y : Container preserved.
// map transforms inner success value only
```

### Works with:
  * `Option`
  * `Result`
  * iterators

### Prefer `map` when:
  * operation cannot fail
  * just transforming value. eg.
    * length
    * uppercase
    * multiply
    * formatting

# 3. `and_then` Use when: [VERY important.]
## “My transformation ALSO returns container”

```rust
// double : Result<i32,String>
fn double(x: i32) -> Result<i32, String> {
    Ok(x * 2)
}
let y:Result<i32,String> = Ok(5);

// here y is also Result and Double is also Result
let z = y.and_then(double);
```
#### WHY NOT `map` ? Because:
```rust 
map()
//would create:
Result<Result<i32,String>,String>
// R: as Y is also Result and Double is also Result...
```
> **NESTED CONTAINER NIGHTMARE.**

#### `and_then` automatically FLATTENS container.

##### Mental model

```
## `map`
T -> U

## `and_then`
T -> Result<U,E>

##or:
T -> Option<U>
```


### Prefer `and_then` when:
  * operation may fail again
  * chaining fallible operations
> SUPER common in backend Rust.

# 4. `unwrap_or_else` Use when:
## “If failure happens, compute fallback dynamically”

```rust
let x = None;
let y = x.unwrap_or_else(|| 42); // fallback value 42 added.
```
> Difference:
```rust
// ## `unwrap_or`
unwrap_or(42) // simple fallback value.

// ## `unwrap_or_else`
unwrap_or_else(|| expensive_computation()) // fallback computed **ONLY if neededl.**
// G:  Efficient.
```

# 🔥 RULE OF THUMB

| Tool             | Use when                      |
| ---------------- | ----------------------------- |
| `if let`         | one branch matters            |
| `map`            | transform success value       |
| `and_then`       | transform that may fail again |
| `unwrap_or_else` | dynamic fallback              |

---

The `itertools` crate (by the official Rust project) extends Rust’s standard `Iterator` trait with **over 60 additional methods**.

While the standard library provides core primitives like `map`, `filter`, and `fold`, `itertools` fills in the gaps for advanced operations: **multi-element manipulation, combinations, zip variants, and chunking/grouping**.

Here is a breakdown of the most useful functions `itertools` offers, categorized by their primary use cases.

---

### 1. Grouping & Chunking

These are used when you need to split an iterator into segments, windows, or groups based on conditions or fixed sizes.

* **`chunk_by(predicate)`**: Groups consecutive elements together as long as the predicate function returns `true` (e.g., grouping consecutive identical characters or numbers).
* **`chunks(n)`**: Splits the iterator into non-overlapping chunks of fixed size `n`.
* **`tuples()`**: Yields fixed-size tuples of items (e.g., `.tuples::<(i32, i32)>()` yields pairs `(a, b)`).
* **`sliding_window(n)`**: Yields a sliding window of length `n` moving through the iterator one element at a time.

```rust
use itertools::Itertools;

// Grouping consecutive items
let data = vec![1, 1, 2, 3, 3, 3];
for (key, group) in &data.into_iter().chunk_by(|&x| x) {
    println!("Value {}: count = {}", key, group.count());
}

// Fixed-size chunking
let numbers = vec![1, 2, 3, 4, 5, 6];
for chunk in &numbers.into_iter().chunks(2) {
    let pair: Vec<i32> = chunk.collect(); // [1, 2], [3, 4], [5, 6]
}

```

---

### 2. Combinatorics (Permutations & Power Sets)

Generate mathematical combinations or selections across one or multiple iterators.

* **`permutations(k)`**: Yields all $k$-length permutations of the items.
* **`combinations(k)`**: Yields all $k$-length combinations (order does not matter).
* **`combinations_with_replacement(k)`**: Yields combinations allowing elements to be repeated.
* **`powerset()`**: Yields all possible subsets of the iterator.

```rust
let items = vec![1, 2, 3];

// All pairs of combinations (order-independent)
for combo in items.iter().combinations(2) {
    println!("{:?}", combo); // [1, 2], [1, 3], [2, 3]
}

```

---

### 3. Joining & Formatting

Construct formatted strings directly from an iterator without needing manual string allocation loops.

* **`join(separator)`**: Concatenates all elements into a single `String`, separated by the given delimiter (calls `Display` on each item).
* **`format(separator)`**: Returns a lightweight formatting object that formats items lazily (zero allocation).

```rust
let names = vec!["Alice", "Bob", "Charlie"];
let result = names.into_iter().join(", ");
println!("{}", result); // "Alice, Bob, Charlie"

```

---

### 4. Advanced Zipping & Combining

Go beyond standard `.zip()` which stops as soon as the shorter iterator runs out.

* **`zip_eq(other)`**: Like `.zip()`, but panics if the two iterators do not have the exact same length.
* **`zip_longest(other)`**: Zips two iterators together until **both** are exhausted, yielding an `EitherOrBoth<A, B>` enum (`Both(a, b)`, `Left(a)`, or `Right(b)`).
* **`cartesian_product(other)`**: Computes the full Cartesian product (nested cross-product) of two iterators.
* **`interleave(other)`**: Alternates yielding items from two iterators until both are finished.
* **`interleave_shortest(other)`**: Alternates items until the shorter iterator runs out.

```rust
use itertools::{EitherOrBoth, Itertools};

let a = vec![1, 2];
let b = vec!["x", "y", "z"];

for item in a.into_iter().zip_longest(b) {
    match item {
        EitherOrBoth::Both(n, s) => println!("Both: {}, {}", n, s),
        EitherOrBoth::Right(s)   => println!("Only right: {}", s),
        EitherOrBoth::Left(n)    => println!("Only left: {}", n),
    }
}

```

---

### 5. Sorting & Unique Filtering

Perform set operations and sorting inline on the iterator without explicitly allocating a `Vec` first.

* **`sorted()`**: Collects items into a vector, sorts them, and returns an iterator over the sorted elements.
* **`sorted_by_key(f)`**: Sorts items based on a key function.
* **`unique()`**: Filters out duplicate elements, keeping only unique ones (requires `Hash + Eq`).
* **`unique_by(f)`**: Keeps items that produce unique keys via a mapping function `f`.
* **`k_smallest(k)` / `k_largest(k)**`: Finds the top or bottom $k$ elements without fully sorting the entire collection.

```rust
let numbers = vec![5, 2, 8, 2, 1, 5];

// Deduplicate and sort inline
let unique_sorted: Vec<i32> = numbers.into_iter().unique().sorted().collect();
println!("{:?}", unique_sorted); // [1, 2, 5, 8]

```

---

### 6. Multi-Item Peeking & Processing

Standard Rust only gives you 1-item lookahead with `.peekable()`.

* **`multipeek()`**: Allows you to look arbitrary steps ahead (`.peek()`, `.peek()`, `.peek()`) without advancing the iterator.
* **`peek_nth(n)`**: Looks ahead directly to the $n$-th element.
* **`tuple_windows()`**: Yields a sliding window of fixed-size tuples (e.g., `(a, b)`, then `(b, c)`, then `(c, d)`).

```rust
let nums = vec![10, 20, 30, 40];

for (a, b) in nums.into_iter().tuple_windows() {
    println!("Pair: {}, {}", a, b); // (10, 20), (20, 30), (30, 40)
}

```

---

### Quick Summary Matrix

| Problem Category | Key `itertools` Functions |
| --- | --- |
| **String Output** | `join`, `format` |
| **Splitting Data** | `chunk_by`, `chunks`, `tuples`, `sliding_window` |
| **Combinatorics** | `permutations`, `combinations`, `powerset` |
| **Merging Iterators** | `zip_longest`, `cartesian_product`, `interleave` |
| **Ordering & Sets** | `sorted`, `unique`, `unique_by`, `k_smallest` |
| **Advanced Lookahead** | `multipeek`, `peek_nth`, `tuple_windows` |

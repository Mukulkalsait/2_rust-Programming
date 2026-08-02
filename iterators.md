# Rust Iterator Reference Guide

Methods on the `std::iter::Iterator` trait are split into **Iterator Adapters** (lazy transformations that return a new iterator) and **Consuming Adapters** (terminal operations that execute the pipeline and return a final value).

---

## 1. Iterator Adapters (Lazy)
Adapters take an iterator, transform its behavior, and return a new iterator structure. They do nothing until a consumer is called.

### Trimming & Slicing (Subsets)
*   **`take(n)`**: Yields only the first `n` elements, then stops.
*   **`take_while(predicate)`**: Yields elements as long as the condition evaluates to `true`, then stops completely.
*   **`skip(n)`**: Ignores the first `n` elements and yields all remaining elements.
*   **`skip_while(predicate)`**: Ignores elements until the condition evaluates to `false`, then yields all remaining elements without checking the condition again.

### Filtering & Mapping (Transformers)
*   **`filter(predicate)`**: Keeps only the elements that satisfy a boolean condition.
*   **`map(f)`**: Transforms every individual element using a closure.
*   **`filter_map(f)`**: Transforms and filters simultaneously; discards `None` results and unwraps `Some(v)` values.
*   **`flat_map(f)`**: Maps each element to an iterator, then flattens all nested iterators into a single linear sequence.
*   **`flatten()`**: Converts a nested iterator (an iterator of iterators or options) into a flat, one-dimensional iterator.

### Reversing, Peeking & Pacing
*   **`rev()`**: Reverses the direction of iteration (requires a `DoubleEndedIterator`).
*   **`peekable()`**: Creates an adapter allowing you to look ahead at the `.peek()` element without advancing the iterator.
*   **`step_by(n)`**: Adjusts the iteration interval to fetch every `n`-th element, skipping intermediate elements.
*   **`cycle()`**: Constantly repeats the entire sequence infinitely when it reaches the end (requires `Clone`).

### Combining & Sequencing
*   **`chain(other)`**: Concatenates two iterators sequentially, passing to the second once the first is empty.
*   **`zip(other)`**: Combines two separate iterators into a single iterator yielding pairs `(A, B)`.
*   **`enumerate()`**: Yields a tuple pair containing the current loop count index and the element `(index, value)`.

### Interception, References & State Management
*   **`inspect(f)`**: Runs a closure on a reference to each item without modifying it; typically used for debugging or logging.
*   **`scan(initial_state, f)`**: Maintains internal mutable state across iterations using a closure to yield items.
*   **`by_ref()`**: Borrows an iterator mutably rather than consuming it, letting you use a subset before continuing.
*   **`cloned()`**: Converts an iterator of references `&T` into an iterator of owned values `T` via cloning.
*   **`copied()`**: Converts an iterator of references `&T` into an iterator of owned values `T` via copying (requires `Copy`).
*   **`fuse()`**: Guarantees that once the iterator yields `None`, every subsequent call to `.next()` will also yield `None`.

---

## 2. Consuming Adapters (Terminal Operations)
Consumers actively evaluate the iterator chain, execute the iteration pipeline, and return a final result value or structural container.

### Evaluation & Collection
*   **`collect()`**: Gathers all values into a targeted collection container (e.g., `Vec`, `HashMap`, `String`).
*   **`unzip()`**: Splits an iterator of pairs `(A, B)` cleanly apart into two separate collections.
*   **`partition(predicate)`**: Consumes the iterator to split elements into two target collections based on a boolean condition.

### Accumulation, Counting & Math
*   **`fold(init, f)`**: Accumulates all elements into a single value using a running accumulator value.
*   **`reduce(f)`**: Similar to `fold`, but skips an initial parameter, tracking the first element as its base accumulator state.
*   **`sum()`**: Computes the numeric mathematical sum total of all values.
*   **`product()`**: Computes the numeric mathematical product total of all values.
*   **`count()`**: Iterates fully through the chain to count and return the total number of items.

### Searching & Indexing
*   **`find(predicate)`**: Searches sequentially for the first item matching a condition, returning it wrapped as an `Option`.
*   **`position(predicate)`**: Searches for the first item matching a condition and returns its index position wrapped as an `Option`.
*   **`rposition(predicate)`**: Searches backwards from the end for the first matching item, returning its true index position.

### Extremes & Extremums
*   **`min()`**: Returns the absolute smallest element according to standard ordering constraints.
*   **`max()`**: Returns the absolute largest element according to standard ordering constraints.
*   **`min_by(f)` / `max_by(f)`**: Returns the absolute smallest or largest element evaluated against a custom comparison closure.
*   **`min_by_key(f)` / `max_by_key(f)`**: Returns the smallest or largest element evaluated against a transformed key value.

### Quantification & Side Effects
*   **`all(predicate)`**: Returns `true` if every single element matches a given predicate condition (short-circuits on first `false`).
*   **`any(predicate)`**: Returns `true` if at least one element matches a given predicate condition (short-circuits on first `true`).
*   **`for_each(f)`**: Executes an explicit closure side effect over every single element (similar to a standard `for` loop).

### Order & Structural Lookups
*   **`nth(n)`**: Selects and extracts the element located at index position `n` (zero-indexed).
*   **`last()`**: Runs through the iterator chain completely to isolate and extract only the final element.
*   **`cmp(other)` / `partial_cmp(other)`**: Lexicographically compares the elements of two iterators.
*   **`eq(other)` / `ne(other)`**: Lexicographically tests two separate iterators for element-wise equality or inequality.
*   **`lt(other)` / `le(other)` / `gt(other)` / `ge(other)`**: Lexicographically compares two iterators for relative sequencing orders.

# Rust Methods

## 1. **String Methods**

These methods are used for manipulating and working with `String` types.

<!-- G: Easy to understand.===========================================-->

- `contains()` = Checks if a `String` contains a given substring.
- `find()` = Searches for the first occurrence of a substring.
- `is_empty()` = Checks if the `String` has a length of zero.
<!-- Y:checkers="👆"=============================================-->

- `push_str()` = Appends a string slice to the end of the `String`.
- `push()` = Adds a single character to the end of a `String`.
- `remove()` = Removes a character at a specific index.
- `replace()` = Replaces a substring with another substring.
- `replace_range()` = Replaces a range of characters with a string slice.
- `trim()` = Removes leading and trailing white space.

- `clear()` = Clears the content of a `String`, making it empty.
- `split_whitespace()` = Splits a string by whitespace.
- `split()` = Splits a `String` by a delimiter into an iterator.

- `to_uppercase()` = Converts all characters in the string to uppercase.
- `to_lowercase()` = Converts all characters in the string to lowercase.
<!-- Y:processors="👆"============================================= -->

- `parse()` = Tries to parse a `String` into a given type.
- `len()` = Returns the number of bytes in the `String`.
- `format!()` = Creates a formatted string (similar to `sprintf`).
<!-- Y:converters="👆"============================================= -->

---

## 2. **Vec (Vector) Methods**

These methods are for manipulating and working with vectors (dynamic arrays).

- `contains()` = Checks if an element exists in the `Vec`.
- `is_empty()` = Checks if the `Vec` is empty.

- `insert()` = Inserts an element at a specified index.
- `push()` = Adds an element to the end of the `Vec`.
- `pop()` = Removes the last element of the `Vec`.
- `remove()` = Removes an element at a specified index.
- `slice()` = Returns a slice of the `Vec` from a range of indices.

- `sort()` = Sorts the elements of the `Vec`.
- `reverse()` = Reverses the order of elements in the `Vec`.

 <!-- DX:No=underatanding============================================-->

- `clear()` = Clears all elements from the `Vec`.
- `len()` = Returns the number of elements in the `Vec`.
- `retain()` = Retains elements in the `Vec` that match a condition.
- `iter()` = Returns an iterator over the elements in the `Vec`.
- `extend()` = Extends the `Vec` with the elements of another iterator or collection.
- `deref()` = Provides access to the elements of a `Vec` through dereferencing.

---

## 3. **Option Methods**

`Option` is an enum that represents an optional value, either `Some(T)` or `None`.

- `is_some()` = Returns true if the `Option` is `Some`.
- `is_none()` = Returns true if the `Option` is `None`.
- `map()` = Maps a value of type `T` to type `U` if the `Option` is `Some`, or returns `None`.
- `and_then()` = Applies a function to the inner value if the `Option` is `Some`, or returns `None`.
- `unwrap()` = Returns the value inside the `Option` if it is `Some`, panics if it is `None`.
- `unwrap_or()` = Returns the value inside the `Option` if it is `Some`, or a default value if `None`.
- `ok_or()` = Converts an `Option` into a `Result`.
- `filter()` = Returns an `Option` that is `Some` if the value matches a condition, otherwise `None`.
- `flatten()` = Converts an `Option<Option<T>>` into `Option<T>`.
- `map_or()` = Returns the mapped value if `Some`, or a default value if `None`.

---

## 4. **Result Methods**

`Result` is an enum used for error handling. It represents either a success (`Ok(T)`) or an error (`Err(E)`).

- `is_ok()` = Returns true if the `Result` is `Ok`.
- `is_err()` = Returns true if the `Result` is `Err`.
- `map()` = Maps the `Ok` value to a new value if `Ok`, or returns `Err` unchanged.
- `map_err()` = Maps the `Err` value to a new value if `Err`, or returns `Ok` unchanged.
- `and_then()` = Applies a function to the `Ok` value if it is `Ok`, or returns the `Err` unchanged.
- `unwrap()` = Returns the `Ok` value if the `Result` is `Ok`, panics if it is `Err`.
- `unwrap_or()` = Returns the `Ok` value if the `Result` is `Ok`, or a default value if `Err`.
- `unwrap_or_else()` = Returns the `Ok` value if the `Result` is `Ok`, or evaluates a closure and returns its result if `Err`.
- `expect()` = Returns the `Ok` value if the `Result` is `Ok`, or panics with a custom message if `Err`.
- `ok()` = Converts a `Result<T, E>` to an `Option<T>`, returning `Some(T)` if `Ok`, or `None` if `Err`.
- `err()` = Converts a `Result<T, E>` to an `Option<E>`, returning `Some(E)` if `Err`, or `None` if `Ok`.
- `is_ok_and()` = Returns `true` if the result is `Ok` and the value matches a condition.

---

## 5. **Iterator Methods**

These methods are used for manipulating and working with iterators in Rust.

- `next()` = Returns the next element of an iterator or `None` if the iterator is exhausted.
- `map()` = Transforms each element of an iterator using a closure.
- `filter()` = Returns an iterator that only yields elements matching a condition.
- `collect()` = Consumes the iterator and collects the elements into a collection (e.g., `Vec` or `HashMap`).
- `fold()` = Combines the elements of an iterator using an accumulator function.
- `sum()` = Consumes the iterator and returns the sum of the elements.
- `count()` = Returns the number of elements in the iterator.
- `any()` = Returns `true` if any element in the iterator matches a condition.
- `all()` = Returns `true` if all elements in the iterator match a condition.
- `position()` = Returns the index of the first element matching a condition.
- `find()` = Returns the first element matching a condition.
- `skip()` = Skips the first `n` elements of the iterator.
- `take()` = Takes the first `n` elements of the iterator.
- `chain()` = Chains multiple iterators together into one iterator.
- `zip()` = Combines two iterators into an iterator of pairs.

---

## 6. **HashMap Methods**

These methods are for manipulating and working with `HashMap`s, a collection type for key-value pairs.

- `insert()` = Inserts a key-value pair into the `HashMap`.
- `get()` = Retrieves the value associated with a key.
- `contains_key()` = Checks if a `HashMap` contains a specific key.
- `remove()` = Removes the key-value pair associated with a key.
- `len()` = Returns the number of key-value pairs in the `HashMap`.
- `is_empty()` = Checks if the `HashMap` is empty.
- `keys()` = Returns an iterator over the keys of the `HashMap`.
- `values()` = Returns an iterator over the values of the `HashMap`.
- `entry()` = Returns an entry for the given key that can be used to insert or modify the value.
- `get_mut()` = Retrieves a mutable reference to the value associated with a key.
- `extend()` = Extends the `HashMap` with another collection of key-value pairs.
- `clear()` = Removes all elements from the `HashMap`.

---

## 7. **File I/O Methods**

These methods are used for reading from and writing to files.

- `open()` = Opens a file in read or write mode.
- `create()` = Creates a file for writing, truncating it if it already exists.
- `write()` = Writes data to a file.
- `read_to_string()` = Reads the entire file contents into a `String`.
- `read_line()` = Reads a single line from a file.
- `flush()` = Flushes the buffered writer, ensuring all data is written to the file.
- `set_len()` = Sets the length of a file (used for resizing).
- `metadata()` = Returns metadata about the file, such as its size.
- `remove_file()` = Removes a file from the filesystem.
- `rename()` = Renames or moves a file to a new location.

---

## 8. **Threading Methods**

Rust provides threading primitives for concurrent programming.

- `spawn()` = Spawns a new thread and returns a handle to the thread.
- `join()` = Waits for a thread to finish its execution.
- `sleep()` = Pauses the current thread for a specified duration.
- `yield_now()` = Yields the current thread to the scheduler to allow other threads to execute.

---

## 9. **Error Handling Methods**

These are used for handling errors, typically working with `Result` and `Option`.

- `expect()` = Returns the `Ok` value or panics with a message if the result is `Err`.
- `unwrap()` = Returns the `Some` value or panics if the option is `None`.
- `unwrap_or_else()` = Returns a default value or evaluates a closure to handle the `Err` or `None`.
- `map_err()` = Maps an `Err` variant to a different error type.
- `from_err()` = Converts a `Result` into a different `Result` type.

---

## 10. **Box Methods**

`Box` is a heap-allocated, non-cloneable smart pointer.

- `into_raw()` = Converts a `Box` into a raw pointer.
- `from_raw()` = Creates a `Box` from a raw pointer.
- `leak()` = Converts a `Box` into a reference that lives for the `'static` lifetime.

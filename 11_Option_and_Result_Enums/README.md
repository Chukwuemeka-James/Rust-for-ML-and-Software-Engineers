# Rust Option and Result Enums

This module introduces two of Rust’s most important enums: `Option` and `Result`. These types are fundamental in handling **absence of values** and **error propagation**, enabling safer and more expressive code without relying on null values or exceptions.

Each file in the `option_and_result_enums` directory explores different aspects of `Option` and `Result` enums through real-world examples and idiomatic Rust practices.

## Project Structure

```
11_Option_and_Result_Enums/
├── option_and_result_enums/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   └── bin/
│   │       ├── building_option_from_scratch.rs
│   │       ├── nuances_of_unwrap_method.rs
│   │       ├── real_example_of_option_enum.rs
│   │       ├── real_example_of_result_enum.rs
│   │       ├── result_methods.rs
│   │       ├── returning_a_result_enum_from_a_function.rs
│   │       ├── returning_an_option_enum_from_a_function.rs
│   │       ├── the_match_keyword_with_option_enum.rs
│   │       ├── the_option_enum.rs
│   │       ├── the_result_enum.rs
│   │       ├── the_unwrap_and_except_methods.rs
│   │       ├── the_unwrap_or_method.rs
│           ├── the_while_let_construct.rs.rs
│           └── top_level_option_variants.rs
│
├── project/  # A project to solidify your learning
    ├── Cargo.toml
    └── src/
        └── main.rs
```


## What You Will Learn

### 1. The `Option` Enum

* **File**: `the_option_enum.rs`
* Introduces `Option<T>` with examples of `Some` and `None` values.

### 2. The `Result` Enum

* **File**: `the_result_enum.rs`
* Demonstrates `Result<T, E>` with success (`Ok`) and error (`Err`) values.

### 3. Match with `Option`

* **File**: `the_match_keyword_with_option_enum.rs`
* Uses `match` to safely handle `Option` values.

---

### 4. Building a Custom `Option` Enum

* **File**: `building_option_from_scratch.rs`
* Manually creates an `Option`-like enum and implements methods such as `.unwrap()` and `.unwrap_or()`.

### 5. Real-World Use of `Option`

* **File**: `real_example_of_option_enum.rs`
* Fetching elements from a list using `.get()` which returns an `Option`.

### 6. Real-World Use of `Result`

* **File**: `real_example_of_result_enum.rs`
* Parsing strings to numbers using `.parse()` which returns a `Result`.

### 7. Returning `Option` from Functions

* **File**: `returning_an_option_enum_from_a_function.rs`
* Demonstrates how to return an `Option` from a custom function to represent missing data.

### 8. Returning `Result` from Functions

* **File**: `returning_a_result_enum_from_a_function.rs`
* Error handling using `Result` in function return types.

### 9. `unwrap()` and `expect()` Methods

* **File**: `the_unwrap_and_except_methods.rs`
* Shows how `.unwrap()` and `.expect()` can be used (and misused).

### 10. `unwrap_or()` Method

* **File**: `the_unwrap_or_method.rs`
* Provides default values when using `Option`.

### 11. Result Methods: `is_ok()`, `is_err()`

* **File**: `result_methods.rs`
* Checks `Result` status using built-in methods.

### 12. Nuances of `unwrap` with `Result`

* **File**: `nuances_of_unwrap_method.rs`
* Shows potential pitfalls of calling `.unwrap()` on an `Err`.

### 13. Top-Level Option Matching

* **File**: `top_level_option_variants.rs`
* Matches top-level `Option` values from function return results.

### 14. `while let` Construct

* **File**: `the_while_let_construct.rs`
* Iteratively pops values from a vector using `while let Some(val)`.

---

## Running the Examples

Navigate to the `option_and_result_enums` directory and run any example using:

```bash
cd 11_Option_and_Result_Enums/option_and_result_enums
cargo run --bin <filename_without_extension>
```

### Example:

```bash
cargo run --bin returning_a_result_enum_from_a_function
```

---
## Project


### Problem Statement:

Design a **Mini Library Inventory System** that allows users to add books, view all books with their availability status, borrow books, return books, and exit the system. The system should use enums to define different library operations and execute the chosen action using match statements.

---

## Key Points & Guidance:

* Create a `Book` struct with fields like `title: String`, `author: String`, `year: u16`, and `status: BookStatus`.
* Define an enum `BookStatus` with variants:

  * `Available`
  * `Borrowed`
* Define an enum `LibraryAction` with variants:

  * `AddBook`
  * `ListBooks`
  * `BorrowBook(String)` (book title)
  * `ReturnBook(String)` (book title)
  * `Exit`
* Implement **functions** to perform each library action.
* Use **match** on the enum to process different actions.
* Use **control flow** (`loop`, `match`, `if`) to create an interactive menu.
* Show how **ownership** works by passing `&mut self` or mutable references to modify book status or library collection.
* Use **string types** (`String`, `&str`) to manage book data and user input effectively.


### You are expected to try the projects before checking the solutions. The solutions are available at:

    ```
    11_Option_and_Result_Enums/project/src/main.rs
    ```

**To run any of them**:

   ```bash
   cd 11_Option_and_Result_Enums/project
   cargo run
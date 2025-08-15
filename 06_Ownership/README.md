# Rust Ownership

This module introduces the concept of **ownership** in Rust, focusing on how values are managed in memory, including ownership, borrowing, references, and the rules that govern them. The files in the `ownership` directory contain practical examples to help you grasp these concepts with concise explanations.

## Project Structure

```
06_Ownership/
├── ownership/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   └── bin/
│   │       ├── dangling_references.rs
│   │       ├── immutable_and_mutable_reference_parameters.rs
│   │       ├── moves_and_ownership.rs
│   │       ├── multiple_immutable_references.rs
│   │       ├── mutable_parameters.rs
│   │       ├── mutable_reference_restrictions.rs
│   │       ├── ownership_and_function_parameters.rs
│   │       ├── ownership_with_arrays_and_tuples.rs
│   │       ├── ownership_with_immutable_and_mutable_references.rs
│   │       ├── references_and_borrowing.rs
│   │       ├── return_values_I.rs
│   │       ├── return_values_II.rs
│   │       ├── scope_and_ownership.rs
│   │       ├── String_&String_str_and_&str.rs
│   │       ├── the_clone_method.rs
│   │       ├── the_copy_trait_with_references.rs
│   │       ├── the_copy_trait.rs
│   │       ├── the_dereference_operator.rs
│   │       ├── the_push_str_method_on_a_string_type.rs
│           └── the_string_type.rs
├── project/  # A project to solidify your learning
    ├── Cargo.toml
    └── src/
        ├── main.rs
        └── bin/
            ├── todo_app.rs
            ├── bank_app.rs
            └── quiz_app.rs
```

---

## What You Will Learn

### 1. **Dangling References**

* **File**: `dangling_references.rs`
* Demonstrates the issue of using references after the ownership has moved, leading to potential memory safety concerns.

### 2. **Immutable and Mutable Reference Parameters**

* **File**: `immutable_and_mutable_reference_parameters.rs`
* Learn how to work with immutable and mutable references as function parameters to allow modification or borrowing.

### 3. **Moves and Ownership**

* **File**: `moves_and_ownership.rs`
* Understand the ownership model where ownership is moved when passed to a new variable, invalidating the original.

### 4. **Multiple Immutable References**

* **File**: `multiple_immutable_references.rs`
* Explore how Rust allows multiple immutable references to a value at the same time but prevents mutable and immutable references from coexisting.

### 5. **Mutable Parameters**

* **File**: `mutable_parameters.rs`
* Demonstrates how mutable parameters allow you to modify the values passed to functions.

### 6. **Mutable Reference Restrictions**

* **File**: `mutable_reference_restrictions.rs`
* Understand the restriction on having more than one mutable reference to the same data at the same time.

### 7. **Ownership and Function Parameters**

* **File**: `ownership_and_function_parameters.rs`
* Learn how ownership is transferred when passing a value to a function and the implications on the original data.

### 8. **Ownership with Arrays and Tuples**

* **File**: `ownership_with_arrays_and_tuples.rs`
* See how ownership is handled when using arrays and tuples, with examples of both owned and borrowed values.

### 9. **Ownership with Immutable and Mutable References**

* **File**: `ownership_with_immutable_and_mutable_references.rs`
* Demonstrates how mutable and immutable references interact with ownership.

### 10. **References and Borrowing**

* **File**: `references_and_borrowing.rs`
* Explains borrowing references, both mutable and immutable, and the rules governing their usage.

### 11. **Return Values I**

* **File**: `return_values_I.rs`
* Shows how a function can return a value by ownership transfer, resulting in the original value being no longer accessible.

### 12. **Return Values II**

* **File**: `return_values_II.rs`
* Further explores returning values from functions, with an example that shows how ownership and modifications are passed back.

### 13. **Scope and Ownership**

* **File**: `scope_and_ownership.rs`
* Demonstrates how variables and references have specific lifetimes, and how they are scoped within functions.

### 14. **String, `&String`, `str`, and `&str`**

* **File**: `String_&String_str_and_&str.rs`
* Understand the differences between owned `String`, borrowed `&String`, the fixed-size `str`, and references to strings (`&str`).

### 15. **The Clone Method**

* **File**: `the_clone_method.rs`
* Learn about the `clone` method, which creates a deep copy of data to avoid moving ownership.

### 16. **The Copy Trait with References**

* **File**: `the_copy_trait_with_references.rs`
* Explains how the `Copy` trait allows simple, shallow copying of values, particularly for primitive types.

### 17. **The Copy Trait**

* **File**: `the_copy_trait.rs`
* Learn how types that implement the `Copy` trait can be copied rather than moved when passed around in functions.

### 18. **The Dereference Operator**

* **File**: `the_dereference_operator.rs`
* Shows how the dereference operator (`*`) is used to access the value behind a reference.

### 19. **The Push\_str Method on a String Type**

* **File**: `the_push_str_method_on_a_string_type.rs`
* Demonstrates how to modify a `String` using the `push_str` method to append more text.

### 20. **The String Type**

* **File**: `the_string_type.rs`
* Introduces the `String` type, a growable, heap-allocated string used to store dynamic text.

---

## Running the Examples

Navigate to the `ownership` directory and run an example with:

```bash
cd 06_Ownership/ownership
cargo run --bin <filename_without_extension>
```

### Example:

```bash
cargo run --bin dangling_references
```


# Project

### **Problem Statement: Build a Simple Bank Simulation Without Structs**

**Objective:**
Create a command-line banking program where a user can deposit money, withdraw money, and view their account balance. The account should be represented using simple variables (not structs), and the program should maintain the correct balance throughout the session.

**Requirements:**

* Store the account holder’s name in a `String` variable and the balance in a `f64` variable.
* Implement `deposit`, `withdraw`, and `display` as separate functions that take the necessary parameters (e.g., references for updating balance).
* Use a `loop` to continuously display a menu until the user chooses to exit.
* Implement menu selection using `match` or `if-else`.
* Validate user inputs for amounts (must be numbers and non-negative for deposits).
* Prevent withdrawals that exceed the available balance.
* Ensure proper use of references (`&` and `&mut`) to update the balance without structs.

### You are expected to try the projects before checking the solutions. The solutions are available at:

    ```
    06_Ownership/project/src/main.rs
    ```

4. **To run any of them**:

```bash
cd 06_Ownership/project
cargo run
```

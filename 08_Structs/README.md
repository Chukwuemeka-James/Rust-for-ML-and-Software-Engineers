# Rust Structs

This module introduces **structs** in Rust—how to define, initialize, update, and implement methods on them. It also covers best practices like method chaining and patterns like the builder pattern. Each file in the `structs` directory is designed to teach you one key concept about working with structs.

## Project Structure

```
08_Structs/
├── structs/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   └── bin/
│   │       ├── associated_functions.rs
│   │       ├── builder_pattern.rs
│   │       ├── calling_methods_from_other_methods.rs
│   │       ├── create_structs_in_a_function.rs
│   │       ├── define_a_struct.rs
│   │       ├── deriving_debug_trait_for_struct.rs
│   │       ├── methods_with_multiple_parameters.rs
│   │       ├── multiple_impl_blocks.rs
│   │       ├── overwrite_struct_fields.rs
│   │       ├── passing_structs_into_a_function.rs
│   │       ├── self_parameter_as_immutable_and_mutable_references_to_struct_instance.rs
│   │       ├── self_parameter_as_mutable_struct_instance.rs
│   │       ├── struct_field_initialization_shorthand_syntax.rs
│   │       ├── struct_update_syntax.rs
│   │       ├── tuple_structs.rs
│   │       └── unit_like_structs.rs
├── project/  # A project to solidify your learning
    ├── Cargo.toml
    └── src/
        ├── main.rs
        └── bin/
            ├── Coffee_Shop_Order_Manager.rs
            ├── Library_Book_Management_System.rs
            └── Simple_Task_Tracker.rs
```

---

## What You Will Learn

### 1. Define a Struct

* **File**: `define_a_struct.rs`
* Learn how to define a custom struct and create instances.

### 2. Create Structs in a Function

* **File**: `create_structs_in_a_function.rs`
* Build struct instances dynamically inside functions.

### 3. Struct Field Initialization Shorthand

* **File**: `struct_field_initialization_shorthand_syntax.rs`
* Use shorthand syntax when variable and field names match.

### 4. Struct Update Syntax

* **File**: `struct_update_syntax.rs`
* Copy fields from other struct instances using `..` syntax.

### 5. Tuple Structs

* **File**: `tuple_structs.rs`
* Learn how to use structs that behave like tuples.

### 6. Unit-like Structs

* **File**: `unit_like_structs.rs`
* Understand structs with no fields, useful for traits.

### 7. Associated Functions

* **File**: `associated_functions.rs`
* Implement constructor-like functions using `Self`.

### 8. Builder Pattern

* **File**: `builder_pattern.rs`
* Chain methods to configure and build a struct instance.

### 9. Deriving Debug Trait

* **File**: `deriving_debug_trait_for_struct.rs`
* Automatically derive the `Debug` trait for struct formatting.

### 10. Methods with Multiple Parameters

* **File**: `methods_with_multiple_parameters.rs`
* Define methods that take additional arguments.

### 11. Calling Methods from Other Methods

* **File**: `calling_methods_from_other_methods.rs`
* Use one method within another to promote reuse and clarity.

### 12. Multiple `impl` Blocks

* **File**: `multiple_impl_blocks.rs`
* Split method definitions across multiple `impl` blocks.

### 13. Overwriting Struct Fields

* **File**: `overwrite_struct_fields.rs`
* Update individual fields in a struct.

### 14. Passing Structs into Functions

* **File**: `passing_structs_into_a_function.rs`
* Learn how to pass structs by value and reference.

### 15. `self` Parameter (Immutable and Mutable)

* **File**:

  * `self_parameter_as_immutable_and_mutable_references_to_struct_instance.rs`
  * `self_parameter_as_mutable_struct_instance.rs`
* Understand how `&self`, `&mut self`, and `self` control ownership and mutability inside methods.

---

## Running the Examples

To run an example, navigate to the `structs` directory and use:

```bash
cd 08_Structs/structs
cargo run --bin <filename_without_extension>
```

### Example:

```bash
cargo run --bin builder_pattern
```

---

# Project

## 1. **Coffee Shop Order Manager**

### Problem Statement:

Build a CLI program that allows customers to create and manage coffee orders. Each order should capture the coffee type, size, extras (like milk or sugar), and temperature preference (hot/cold).

### Key Points & Guidance:

* **Define a `CoffeeOrder` struct** with fields for `coffee_type` (String), `size` (enum or String), `extras` (Vec<String>), and `is_hot` (bool).
* Implement methods to **create** a new order and **display** the order details nicely.
* Use **control flow (`if`, `match`)** to validate user input (e.g., only allow certain sizes, extras).
* Provide a simple **menu loop** where users can:

  * Create a new order
  * View existing orders
  * Edit or cancel orders
* Encourage use of **method calls** on the struct to encapsulate functionality.
* Bonus: Use **builder pattern** for flexible order creation.

---

## 2. **Simple Task Tracker**

### Problem Statement:

Create a task management app that lets users add tasks, mark them done, and list all or filtered tasks (e.g., by priority). It should run in the terminal and accept commands interactively.

### Key Points & Guidance:

* Define a `Task` struct with fields like `title` (String), `done` (bool), and `priority` (enum or integer).
* Use an **in-memory list** (Vec<Task>) to store all tasks.
* Implement methods to:

  * Add new tasks
  * Mark tasks as done
  * List tasks, optionally filtering by priority or done status
* Use **control flow (`match`, `if`)** to parse and respond to user commands.
* Build a **loop** that continuously prompts users for input until they exit.
* Show how **method calls** can update the state of each task.
* Encourage using **struct update syntax** when modifying tasks.

---

## 3. **Library Book Management System**

### Problem Statement:

Design a system that models a library’s book catalog and supports borrowing, returning, and searching books. The system should maintain a collection of books and allow users to interact with it through a menu.

### Key Points & Guidance:

* Define a `Book` struct with fields: `title`, `author`, `is_borrowed` (bool), and maybe `id`.
* Create a `Library` struct that holds a `Vec<Book>`.
* Implement methods on `Library` to:

  * Add new books
  * Borrow a book by marking it as borrowed
  * Return a book
  * Search for books by title or author
* Use **loops and match statements** to implement the interactive menu.
* Use **conditionals** to prevent borrowing an already borrowed book.
* Teach **ownership and borrowing concepts** when passing book references.
* Optional: Track overdue books with additional fields.

---

### You are expected to try the projects before checking the solutions. The solutions are available at:

    ```
    08_Structs/project/src/bin
    ```

**To run any of them**:

   ```bash
   cd 08_Structs/project
   cargo run --bin Coffee_Shop_Order_Manager
   cargo run --bin Library_Book_Management_System
   cargo run --bin Simple_Task_Tracker
   ```
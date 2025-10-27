# Rust Generics

This module introduces **generics** in Rust, covering the fundamentals and advanced usage of generic types in structs, enums, functions, and impl blocks. It explains how generics allow writing flexible, reusable code that works with multiple data types while maintaining type safety.

## Project Structure

```
10_Generics/
├── generics/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   └── bin/
│   │       ├── generics_and_impl_blocks_I.rs
│   │       ├── generics_and_impl_blocks_II.rs
│   │       ├── generics_in_enums.rs
│   │       ├── generics_in_structs.rs
│   │       ├── intro_to_generics.rs
│   │       ├── multiple_generics.rs
│   │       └── the_turbofish_operator.rs
│
├── project/  # A project to solidify your learning
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs 
│       └── bin/
│           ├── Inventory_Management_System_CLI.rs
            └── Task_Management_CLI_Tool.rs
```

---

## What You Will Learn

### 1. Introduction to Generics

* **File**: `intro_to_generics.rs`
* Understand the concept of generics with a simple identity function that works with any type.

### 2. Generics in Structs

* **File**: `generics_in_structs.rs`
* Learn how to define structs that hold generic data types, making the struct flexible and reusable.

### 3. Generics in Enums

* **File**: `generics_in_enums.rs`
* See how enums can also be generic, allowing enum variants to hold different types of data.

### 4. Multiple Generics

* **File**: `multiple_generics.rs`
* Explore functions that take multiple generic type parameters, creating tuples from heterogeneous types.

### 5. The Turbofish Operator

* **File**: `the_turbofish_operator.rs`
* Understand the turbofish syntax (`::<Type>`) to explicitly specify generic types when calling functions.

### 6. Generics and Impl Blocks (Part I)

* **File**: `generics_and_impl_blocks_I.rs`
* Learn how to implement methods on generic structs specialized for certain types.

### 7. Generics and Impl Blocks (Part II)

* **File**: `generics_and_impl_blocks_II.rs`
* Dive deeper with impl blocks combining generic and specialized implementations for more functionality.

---

## Running the Examples

Navigate to the `generics` directory and run an example by specifying the file name (without `.rs`) after `--bin`:

```bash
cd 10_Generics/generics
cargo run --bin <filename_without_extension>
```

### Example:

```bash
cargo run --bin intro_to_generics
```
---

## 1. **Task Management CLI Tool**

### Problem Statement:

Design a **Task Management CLI Tool** that allows users to **create**, **list**, **mark as complete**, and **delete** tasks. Each task should include a description and a status (`Pending` or `Completed`). The system should use enums to model task states and allow users to interact with it via terminal input.

### Key Points & Guidance:

* Define a `TaskStatus` enum with variants: `Pending`, `Completed`.
* Create a `Task` struct containing fields like `id`, `description`, and `status`.
* Use a vector (`Vec<Task>`) to store a collection of tasks.
* Implement functions/methods to:

  * Add a task
  * List all tasks
  * Mark a task as complete
  * Delete a task
* Use **pattern matching** (`match`) to determine the status of each task when displaying.
* Use a **loop** to display a menu in the CLI (e.g., using `std::io::stdin`) and process user choices.
* Optionally use **file I/O** to persist tasks across runs using a crate like `serde_json`.

### Example Interaction
```
=== Task Management CLI ===
1. Add a task
2. List all tasks
3. Mark task as complete
4. Delete a task
5. Exit
Choose an option: 1
Enter task description: Learn Rust enums
✅ Task added with ID: 1

Choose an option: 2
Current Tasks:
[1] Learn Rust enums - Pending

Choose an option: 3
Enter task ID to mark complete: 1
✅ Task 1 marked as complete.

Choose an option: 2
Current Tasks:
[1] Learn Rust enums - ✅ Completed
```

---

## 2. **Inventory Management System**

### Problem Statement:

Build an **Inventory Management System** that tracks a list of items in a store or warehouse. Each item should have a name, quantity, and category. Categories should be modeled using an enum, and operations like **adding**, **listing**, and **updating inventory** should be supported.

### Key Points & Guidance:

* Define an `ItemCategory` enum with variants like `Electronics`, `Groceries`, `Clothing`, etc.
* Create a generic `Item<T>` struct with fields like `name`, `quantity`, `category`, and optionally `metadata: T`.
* Store the inventory in a `Vec<Item<T>>` or use a `HashMap<String, Item<T>>` for easier lookup.
* Implement key inventory operations:

  * Add a new item
  * Update item quantity
  * List all items grouped by category
  * Search for an item by name
* Use **pattern matching** with the `ItemCategory` enum to display or sort items by category.
* Use **generics** to allow flexibility in how item metadata (like price, supplier info, etc.) is stored.
* Optionally, persist the inventory data using file I/O and `serde` for serialization.


## Sample Output

```
=== Inventory Management ===
1. Add Product
2. View Inventory
3. Update Stock
4. Delete Product
5. Exit
Choose an option: 1
Enter product name: Bluetooth Headphones
Enter quantity: 20
Enter price: 99.99
Enter category (Electronics/Groceries/Clothing/Other): Electronics
✅ Product added with ID: 1

Choose an option: 2
Current Inventory:
[1] Bluetooth Headphones | Qty: 20 | $99.99 | Category: Electronics

Choose an option: 3
Enter product ID to update: 1
Enter quantity to add/remove (use negative for removal): -5
✅ Updated stock. New quantity: 15
```

### You are expected to try the projects before checking the solutions. The solutions are available at:

    ```
    10_Generics/project/src/bin
    ```

**To run any of them**:

   ```bash
   cd 10_Generics/project
   cargo run --bin Inventory_Management_System_CLI
   cargo run --bin Task_Management_CLI_Tool
   ```
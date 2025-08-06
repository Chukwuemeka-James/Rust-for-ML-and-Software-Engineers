# Rust Functions

This module is designed to introduce **functions** in Rust, covering topics such as defining functions, return values, parameters and arguments, and different function styles. Each file in the `functions` directory focuses on a specific aspect of functions in Rust, with practical examples and clear explanations.

## Project Structure

```
04_Functions/
├── functions/
│   ├── Cargo.toml
│   ├── src/
│       ├── main.rs
│       └── bin/
│           ├── blocks_in_functions.rs
│           ├── explicit_return_values.rs
│           ├── implicit_return_values.rs
│           ├── intro_to_functions.rs
│           ├── parameters_and_arguments.rs
│   
├── project/  # A project to solidify your learning
```

## What You Will Learn

### 1. **Blocks in Functions**

* **File**: `blocks_in_functions.rs`
* Learn how to use blocks within functions, which can contain expressions and perform calculations before returning a value. This demonstrates the concept of scope and calculation within a function.

### 2. **Explicit Return Values**

* **File**: `explicit_return_values.rs`
* Understand how to explicitly return a value from a function using the `return` keyword. This file demonstrates how to declare functions that return a value.

### 3. **Implicit Return Values**

* **File**: `implicit_return_values.rs`
* Functions in Rust can also implicitly return values, without the need for the `return` keyword. This file shows how Rust allows functions to return values directly from expressions.

### 4. **Introduction to Functions**

* **File**: `intro_to_functions.rs`
* An introduction to defining and calling functions in Rust. Learn how to create functions that perform different actions, like printing output to the console.

### 5. **Parameters and Arguments**

* **File**: `parameters_and_arguments.rs`
* Explore how functions can accept parameters and arguments, making them more versatile. This example demonstrates how to pass values to functions and use them in calculations or actions.

### 6. **The Unit Type as a Return Type**

* **File**: `the_unit_as_a_return_type.rs`
* Learn about the unit type (`()`), which is used when a function doesn’t need to return any meaningful value. This is the default return type for functions that don’t explicitly return something.

---

## Getting Started

To run any of the example files:

```bash
cd 04_Functions/functions
cargo run --bin <filename_without_.rs>
```

For example:

```bash
cargo run --bin blocks_in_functions
```

---

## Mini Project
### **Problem Statement: Simple Banking System Simulation**

In this project, you will build a simple banking system that allows a user to perform basic banking operations such as depositing money, withdrawing money, and checking the balance. The program will help reinforce your knowledge of variables, mutability, data types, and functions in Rust.

### **Requirements**:

* Create a mutable `account_balance` variable to hold the balance of a bank account.
* Implement functions for deposit, withdrawal, and checking the balance.
* The `deposit` function should accept the current balance and the deposit amount, then return the updated balance.
* The `check_balance` function should display the current balance.
* The `withdraw` function should check if there are sufficient funds and return the updated balance, or print an error if there are insufficient funds.


*You can find the implementation inside:*

```
04_Functions/project/src/main.rs
```

To run the project:

```bash
cd 04_Functions/project
cargo run
```


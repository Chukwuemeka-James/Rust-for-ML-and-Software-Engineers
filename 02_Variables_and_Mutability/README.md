# Rust: Variables and Mutability

This project provides an introduction to core Rust concepts, including variables, mutability, scope, and compiler directives. Through various examples, it demonstrates how variable handling works in Rust and introduces some best practices for managing variables in the language.
## Keywords Overview

| Keyword | Description                                                                                 | Example                       |
| ------- | ------------------------------------------------------------------------------------------- | ----------------------------- |
| `let`   | Declares a variable binding. Variables are immutable by default.                            | `let x = 5;`                  |
| `mut`   | Makes a variable mutable, allowing its value to be changed after declaration.               | `let mut counter = 0;`        |
| `const` | Declares a compile-time constant that must have a type annotation and cannot be reassigned. | `const MAX_USERS: u32 = 100;` |
| `type`  | Creates a type alias, improving readability and semantic meaning.                           | `type Kilometers = i32;`      |


## Code Structure

```
02_Variables_and_Mutability/
├── variables_and_mutability/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   └── bin/
│   │       ├── constants.rs
│   │       ├── immutable_and_mutable_variables.rs
│   │       ├── intro_to_compiler_directives.rs
│   │       ├── intro_to_variables.rs
│   │       ├── positional_arguments_to_println.rs
│   │       ├── scopes.rs
│   │       ├── type_aliases.rs
│   │       ├── underscore_with_variables.rs
│   │       └── variable_shadowing.rs
├── project/  # A project to solidify your learning
```

Each file within the `bin/` folder covers a different aspect of Rust's variable handling system, and the main project file provides a real-world application for you to practice these concepts.

## Concepts Covered

### 1. **Constants**

* **File:** `constants.rs`
* Constants in Rust are declared using `const` and must have a type annotation.
* Constants are immutable and evaluated at compile time.

### 2. **Immutable and Mutable Variables**

* **File:** `immutable_and_mutable_variables.rs`
* In Rust, variables are immutable by default.
* To make a variable mutable, use the `mut` keyword. This allows its value to change after it is initially bound.

### 3. **Compiler Directives**

* **File:** `intro_to_compiler_directives.rs`
* The `#![allow(unused_variables)]` directive allows the program to compile even if some variables are not used, which can be useful during development and testing.

### 4. **Variable Declaration**

* **File:** `intro_to_variables.rs`
* Demonstrates basic variable declarations and arithmetic operations.
* Rust’s type inference makes it unnecessary to always specify types.

### 5. **Positional Arguments in `println!`**

* **File:** `positional_arguments_to_println.rs`
* Shows how to reuse values in `println!` using positional arguments (e.g., `{0}`, `{1}`).
* This method allows for cleaner and more dynamic print formatting.

### 6. **Variable Scopes**

* **File:** `scopes.rs`
* Rust supports block-level scoping. A variable declared inside a block `{}` will shadow any variable with the same name in the outer scope.
* After the block, the outer variable becomes accessible again.

### 7. **Type Aliases**

* **File:** `type_aliases.rs`
* Introduces type aliases using the `type` keyword to make code more readable and semantically meaningful (e.g., `type Meters = i32`).

### 8. **Underscore with Variables**

* **File:** `underscore_with_variables.rs`
* Prefixing a variable name with an underscore (e.g., `_fruits`) tells the compiler that the variable will not be used, avoiding unnecessary warnings.

### 9. **Variable Shadowing**

* **File:** `variable_shadowing.rs`
* Rust allows variables to be shadowed, meaning a variable can be redeclared with the same name and a different type.
* This helps in transforming variables while still maintaining immutability.

---

## How to Run the Examples

To run any of the examples in the `bin/` folder, use the `cargo run --bin <filename>` command. For example:

```bash
cargo run --bin constants
```

Ensure you're in the root directory of the project when executing the above command.

To rerun the executable run:

```bash
./target/debug/constants
```

---

# Project: **Static Budget Report Generator**

## Project Overview

This beginner-friendly project will help you apply Rust concepts related to variables and mutability. You will create a static monthly budget report that uses constants, variables, shadowing, scopes, and formatted output.

## Problem Statement

Create a simple Rust program to simulate a monthly budget. The program should:

* Use a constant for the monthly income.
* Define immutable variables for various expenses (e.g., rent, groceries, transport).
* Calculate the total expenses and the remaining balance.
* Demonstrate variable shadowing and block-level scoping.
* Format and display the results using `println!` macro.

## Instructions

1. Define a constant for the income and an emergency fund.
2. Create variables for the expenses.
3. Compute the total expenses and the remaining balance.
4. Use a nested scope to demonstrate variable shadowing (e.g., deduct the emergency fund).
5. Print the results using formatted strings and positional arguments.

**Note**: Try solving the problem yourself before checking the solution.

## Solution Location

Once you've attempted the task, you can compare your solution with the reference implementation in:

```
02_Variables_and_Mutability/project/src/main.rs
```
To run the Rust code located in `02_Variables_and_Mutability/project/src/main.rs`, follow these steps:

### 1. **Navigate to the Project Directory**
Open a terminal and go to the directory containing the `Cargo.toml` file (the root of the Rust project). In your case, it should be:
```sh
cd 02_Variables_and_Mutability/project/
```

### 2. **Build and Run the Project**
Use `cargo run` to compile and execute the program:
```sh
cargo run
```

This README is structured to guide beginners through understanding and applying Rust's core concepts in variables and mutability. Follow along with the provided examples, and don’t hesitate to try the project to solidify your learning.
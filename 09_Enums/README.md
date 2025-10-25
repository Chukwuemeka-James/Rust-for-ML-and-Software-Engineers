# Rust Enums

This module explores **Enums** in Rust, demonstrating how they can be used to model data with different forms and behaviors. The examples walk through foundational and advanced enum patterns, including associated values, nested enums, method implementations, and control flow integration with `match`, `if let`, and `let else`.

## Project Structure

```
09_Enums/
├── enums/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   └── bin/
│   │       ├── a_brief_discussion_on_enum_memory.rs
│   │       ├── defining_methods_on_enums.rs
│   │       ├── enum_with_associated_values_I.rs
│   │       ├── enum_with_associated_values_II.rs
│   │       ├── intro_to_enums.rs
│   │       ├── nesting_enums_in_enums.rs
│   │       ├── struct_variants.rs
│   │       ├── the_if_let_construct.rs
│   │       ├── the_let_else_construct.rs
│   │       ├── the_match_keyword_I.rs
│   │       ├── the_match_keyword_II.rs
│   │       ├── the_match_keyword_III.rs
│   │       ├── the_match_keyword_IV.rs
│   │       └── the_match_keyword_V.rs
├── project/  # A project to solidify your learning
    ├── Cargo.toml
    └── src/
        ├── main.rs
        └── bin/
            ├── Digital_Wallet_System.rs
            └── Smart_Traffic_Light_Controller.rs
```

---

## What You Will Learn

### 1. Introduction to Enums

* **File**: `intro_to_enums.rs`
* Introduces basic enum definitions and usage patterns like matching and reassignment.

### 2. Enums with Associated Values (Part I & II)

* **Files**:

  * `enum_with_associated_values_I.rs`
  * `enum_with_associated_values_II.rs`
* Demonstrates how enums can carry additional data within their variants.

### 3. Enum Memory Behavior

* **File**: `a_brief_discussion_on_enum_memory.rs`
* Illustrates how enums store and switch between associated values in memory.

### 4. Struct Variants

* **File**: `struct_variants.rs`
* Combines enums with struct-like fields for richer, more structured data.

### 5. Defining Methods on Enums

* **File**: `defining_methods_on_enums.rs`
* Implements custom methods for enums using `impl`.

### 6. Nesting Enums

* **File**: `nesting_enums_in_enums.rs`
* Shows how enums can contain other enums, useful for complex data modeling.

### 7. Pattern Matching with `match`

* **Files**:

  * `the_match_keyword_I.rs`
  * `the_match_keyword_II.rs`
  * `the_match_keyword_III.rs`
  * `the_match_keyword_IV.rs`
  * `the_match_keyword_V.rs`
* Deep dive into matching patterns, guards, and destructuring enum variants.

### 8. The `if let` Construct

* **File**: `the_if_let_construct.rs`
* Provides a concise alternative to `match` for single-pattern matching.

### 9. The `let else` Construct

* **File**: `the_let_else_construct.rs`
* Newer control flow construct to destructure and handle failure inline.

---

## Running the Examples

Navigate to the `enums` directory and run an example with:

```bash
cd 09_Enums/enums
cargo run --bin <filename_without_extension>
```

### Example:

```bash
cargo run --bin defining_methods_on_enums
```

---

## Project

### 1. Problem Statement:

Design a **Smart Traffic Light Controller** that simulates the behavior of a traffic light system. The system should model the light states (`Red`, `Green`, `Yellow`) and transition between them in a timed loop. It should print the current state and wait for a few seconds before transitioning to the next.

### Key Points & Guidance:

* Define a `TrafficLight` enum with variants: `Red`, `Green`, and `Yellow`.
* Implement a method `next()` on `TrafficLight` to return the next state in the cycle.
* Use the `std::thread::sleep` function to simulate waiting time for each state.
* Use a `loop` or `for` loop to run the light transitions a fixed number of cycles or infinitely.
* Demonstrate **pattern matching** using `match` on the enum to:

  * Print the current light
  * Determine how long to stay in that state
* Use **enums** and **methods** effectively to encapsulate logic.


### 2. Problem Statement:

Design a **Digital Wallet System** that allows users to check their balance, add funds, withdraw funds, and view recent activity. The system should use enums to define different wallet operations and execute the chosen action using match statements.


## Key Points & Guidance:

* Create a `Wallet` struct with fields like `balance: f64` and `history: Vec<String>`.
* Define an enum `WalletAction` with variants:

  * `CheckBalance`
  * `AddFunds(f64)`
  * `WithdrawFunds(f64)`
  * `ViewHistory`
  * `Exit`
* Implement **functions** to perform each wallet action.
* Use **match** on the enum to process different actions.
* Use **control flow** (`loop`, `match`, `if`) to create an interactive menu.
* Show how **ownership** works by passing `&mut self` to modify wallet state.
* Use **slices** and **string types** (`String`, `&str`) to manage user input and activity history.

---

### You are expected to try the projects before checking the solutions. The solutions are available at:

    ```
    09_Enums/project/src/bin
    ```

**To run any of them**:

   ```bash
   cd 09_Enums/project
   cargo run --bin Digital_Wallet_System
   cargo run --bin Smart_Traffic_Light_Controller
   ```
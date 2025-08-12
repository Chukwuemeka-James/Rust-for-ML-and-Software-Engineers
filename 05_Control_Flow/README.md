# Rust Control Flow

This module introduces **control flow** in Rust, covering essential concepts such as `if` statements, `match` expressions, loops (`loop`, `while`, `for`), `break`, `continue`, and recursion. Each file in the `control_flow` directory focuses on a specific aspect of control flow, complete with practical examples and concise explanations.

## Project Structure

```
05_Control_Flow/
├── control_flow/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   └── bin/
│   │       ├── assigning_result_of_if_statement_to_variable.rs
│   │       ├── intro_to_conditionals.rs
│   │       ├── recursion.rs
│   │       ├── the_continue_keyword.rs
│   │       ├── the_loop_and_break_keywords.rs
│   │       ├── the_match_statement_2.rs
│   │       ├── the_match_statement.rs
│   │       ├── the_match_statement_3.rs
│   │       └── while_loop.rs
├── project/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs  ← To-Do List Project
```

---

## What You Will Learn

### 1. `if` Statement

* **File**: `the_if_statement.rs`
* Introduction to conditional execution using `if`.

### 2. `else` Statement

* **File**: `the_else_statement.rs`
* Demonstrates how to provide a fallback path using `else`.

### 3. `else if` Statement

* **File**: `the_else_if_statement.rs`
* Adds multiple conditional branches using `else if`.

### 4. Assigning Result of `if` Statement

* **File**: `assigning_result_of_if_statement_to_variable.rs`
* Shows how `if` statements can return values directly for concise assignment.

### 5. `match` Statement

* **File**: `the_match_statement.rs`
* Understands pattern matching and value binding.

### 6. `match` with Conditions & Multiple Values

* **File**: `the_match_statement_with_multiple_values_and_conditionals.rs`
* Advanced use-cases including guards and grouped patterns.

### 7. Underscore in `match`

* **File**: `underscore_in_a_match_arm.rs`
* Catches unmatched values with `_` wildcard.

### 8. Infinite Loops and `break`

* **File**: `the_loop_and_break_keywords.rs`
* Demonstrates `loop` and how to exit with `break`.

### 9. `continue` in Loops

* **File**: `the_continue_keyword.rs`
* Skips remaining iteration using `continue`.

### 10. `while` Loop

* **File**: `while_loop.rs`
* Executes while a condition remains true.

### 11. Recursion

* **File**: `recursion.rs`
* Implements a recursive countdown function.

---

## Running the Examples

Navigate to the `control_flow` directory and run an example with:

```bash
cd 05_Control_Flow/control_flow
cargo run --bin <filename_without_extension>
```

### Example:

```bash
cargo run --bin recursion
```

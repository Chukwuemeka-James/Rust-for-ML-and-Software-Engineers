# Rust: Data Types and Operations

This module introduces essential Rust concepts such as primitive types, operators, type casting, formatting, and common standard library methods. Each file is designed to help beginners understand Rust through focused examples and hands-on practice.

## Code Structure

```
03_Data_Type/
├── data_types/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       └── bin/
│           └── *.rs      ← Concept-focused examples
├── project/
│   └── main.rs           ← Mini project to consolidate learning
```

Each file in the `bin/` folder focuses on a specific Rust concept related to data types or operators.

## Concepts Covered

### 1. **Booleans**

* **File**: `intro_to_booleans.rs`
* Learn how Rust represents truth values using `bool`.
* Explore simple conditions and positivity/negativity checks.

### 2. **Integers and Floats**

* **Files**: `integers.rs`, `floating_point_types.rs`, `formatting_floats_with_format_specifiers.rs`
* Work with signed/unsigned integers (`i32`, `u8`, etc.) and floats (`f32`, `f64`).
* Use format specifiers to control float precision in output.

### 3. **Characters and Strings**

* **Files**: `the_character_type.rs`, `strings_and_raw_strings.rs`
* Understand how Rust handles single characters (`char`) and string literals, including raw strings and escape sequences.

### 4. **Tuples and Arrays**

* **Files**: `the_tuple_type.rs`, `the_array_type.rs`, `reading_and_writing_array_elements.rs`
* Learn to group values using tuples and fixed-length sequences using arrays.
* Read from and write to array elements safely.

### 5. **Operators**

* **Files**:

  * `and_with_ampersand_operators.rs` – Logical AND
  * `or_with_vertical_pipes.rs` – Logical OR
  * `boolean_inversion_with_exclamation_mark.rs` – Logical NOT
  * `equality_and_inequality_operators.rs` – Comparison (`==`, `!=`)
  * `augmented_assignment_operator.rs` – Compound assignment (`+=`, `-=`)
  * `math_operations.rs` – Arithmetic (`+`, `-`, `*`, `/`, `%`)

### 6. **Type Casting**

* **File**: `casting_types_with_the_as_keyword.rs`
* Learn how to safely convert between numeric types using the `as` keyword.

### 7. **Ranges and Iteration**

* **File**: `ranges_and_range_iteration.rs`
* Define numeric or character ranges and iterate over them using `for` loops.

### 8. **Debugging and Printing**

* **Files**:

  * `the_dbg_macro.rs` – Debug with `dbg!` macro.
  * `the_debug_trait.rs` – Format output with `{:?}` and `{:#?}`.
  * `the_display_trait.rs` – Use `Display` trait for user-friendly output.

### 9. **Miscellaneous Types and Methods**

* **Files**:

  * `intro_to_methods.rs` – Built-in numeric/string methods like `.abs()`, `.pow()`, `.trim()`.
  * `intro_to_generics.rs` – Range expressions with generics.
  * `the_usize_and_isize_types.rs` – Pointer-sized integers.
  * `using_underscore_as_visual_separator_for_numbers.rs` – Improve readability in large numbers (e.g., `1_000_000`).

---

## How to Run the Examples

Navigate to the `data_types` directory and run any example with the following command:

```bash
cd 03_Data_Type/data_types
cargo run --bin <filename_without_.rs>
```

Example:

```bash
cargo run --bin the_array_type
```

---

## Mini Project: **Health Advisor with Predefined BMI Data**

### Project Goal

Create a simple health advisor that computes and classifies person's BMI (Body Mass Index) using predefined personal data.

### Input Data (Hardcoded)

* Name
* Weight (in kilograms)
* Height (in meters)

### BMI Formula

```
BMI = Weight / (Height ^ 2)
```

### Classification Logic

| BMI Range | Category    |
| --------- | ----------- |
| < 18.5    | Underweight |
| 18.5–24.9 | Normal      |
| 25.0–29.9 | Overweight  |
| ≥ 30.0    | Obese       |


You can find the implementation inside:

```
03_Data_Type/project/src/main.rs
```

To run the project:

```bash
cd 03_Data_Type/project
cargo run
```

---

## Summary

By the end of this module, you will:

* Understand how Rust handles data types and operations.
* Write clear, safe, and idiomatic Rust code using types, formatting, and methods.
* Be able to build small projects to demonstrate core understanding.
# Rust Error Handling

This module explores **error handling** in Rust, focusing on techniques for writing robust programs that can gracefully handle unexpected situations. You’ll learn about the `Result` and `Option` types, the `?` operator, error propagation, and how to exit programs when things go wrong. Each file in the `error_handling` directory demonstrates a specific aspect with clear, concise examples.

## Project Structure

```
15_Error_Handling/
├── error_handling/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       └── bin/
│           ├── asking_the_user_for_input.rs
│           ├── opening_a_file.rs
│           ├── propagating_errors.rs
│           ├── reading_the_files_contents.rs
│           ├── standard_error.rs
│           ├── the_panic_macro.rs
│           ├── the_process_module_and_the_exit_function.rs
│           ├── the_question_mark_operator.rs
│           ├── the_read_to_string_associated_function.rs
│           ├── understanding_error_type_redeclaration.rs
│           └── using_question_mark_with_option.rs
└── project/  # A practical application to reinforce the concepts
    ├── Cargo.toml
    ├── src/
    │   ├── main.rs
    │   └── bin/
    │       └── *.rs
    ├── sample_unauthorized.txt
    ├── sample_repeated_words.txt
    └── sample_clean.txt
```

---

## What You Will Learn

### 1. Asking the User for Input

* **File**: `asking_the_user_for_input.rs`
* Handles user input from the terminal and uses `match` and `process::exit()` for error reporting.

### 2. Opening a File

* **File**: `opening_a_file.rs`
* Demonstrates basic file opening with graceful error handling using `match`.

### 3. Reading the File's Contents

* **File**: `reading_the_files_contents.rs`
* Combines file I/O with user input and error management using `match`.

### 4. Propagating Errors

* **File**: `propagating_errors.rs`
* Shows how to use `Result<T, E>` to return and propagate errors from functions.

### 5. Standard Error Output

* **File**: `standard_error.rs`
* Introduces `eprintln!()` for writing errors to stderr.

### 6. The `panic!` Macro

* **File**: `the_panic_macro.rs`
* Demonstrates how to crash a program intentionally with `panic!`.

### 7. Exiting with the `process` Module

* **File**: `the_process_module_and_the_exit_function.rs`
* Uses `process::exit()` to end the program early.

### 8. The `?` Operator

* **File**: `the_question_mark_operator.rs`
* Shows concise error propagation using the `?` operator.

### 9. `fs::read_to_string`

* **File**: `the_read_to_string_associated_function.rs`
* Reads a file’s content in a single call using `fs::read_to_string`.

### 10. Understanding Error Type Redeclaration

* **File**: `understanding_error_type_redeclaration.rs`
* Reinforces correct handling of similar I/O error flows without naming conflicts.

### 11. Using `?` with `Option`

* **File**: `using_question_mark_with_option.rs`
* Applies the `?` operator in the context of `Option`, returning `None` early when appropriate.

---

## Running the Examples

Navigate to the `error_handling` directory and run an example with:

```bash
cd 15_Error_Handling/error_handling
cargo run --bin <filename_without_extension>
```

### Example:

```bash
cargo run --bin propagating_errors
```

---

## Practice Project: File Reader with Word Count, Frequency Analysis, and Word Filter

**Objective:**
Build a command-line tool in Rust that reads a text file, counts its content metrics (lines, words, characters), identifies the most frequent word, and checks for unauthorized words using Rust’s `HashMap`, `HashSet`, and robust file handling techniques.

**Background:**
Analyzing text files is a common task in many applications. This project introduces learners to file I/O, string processing, error handling, and collection-based data analysis in Rust. By enhancing the tool with features like frequency analysis and blacklist word detection, learners will practice using `HashMap` for frequency counting and `HashSet` for filtering.

**Requirements:**

* Prompt the user to enter a filename.
* Open and read the file content into memory using `std::fs::read_to_string`.
* Perform the following actions for each file:

  1. **Count Lines:** Number of lines in the file.
  2. **Count Words:** Total number of words.
  3. **Count Characters:** Total number of characters.
  4. **Most Frequent Word:** Determine the most frequently occurring word using a `HashMap<String, usize>`.
  5. **Unauthorized Word Check:** Use a `HashSet<String>` of banned words. If any are present in the file, display a warning and skip further analysis for that file.

**Constraints:**

* Words are case-insensitive for analysis and filtering.
* File must exist; otherwise, display a user-friendly error.
* Unauthorized words must be matched exactly (no partial match).
* Provide an option to exit the program or continue with another file.

**How the Project Should Work:**

* The program starts with a loop offering the user two options:

  * Enter a filename for analysis
  * Exit the program gracefully
* If a file is entered:

  * The program reads its contents and checks for file read errors.
  * If no unauthorized words are found:

    * Count and display lines, words, characters.
    * Compute and display the most frequent word (excluding common stopwords if desired).
  * If unauthorized words are found:

    * Show a message like: `Unauthorized words found: ["classified", "secret"]`
    * Skip the rest of the analysis for that file.
* Continue looping until the user chooses to exit.


### You are expected to try the projects before checking the solutions. The solutions are available at:

```
14_HashMaps/project/src/main.rs
```

**To run any of them**:

```bash
cd 13_Strings/project
cargo run
```

## Example Usage Guide: File Reader CLI App

#### 1. Build and Run the App

Open your terminal, navigate to your project folder, and run:

```bash
cargo run
```

#### 2. Try the App

* The program will prompt:

  ```
  Enter the file name (or type 'exit' to quit):
  ```

* Enter a file name like:

  ```
  sample_repeated_words.txt
  ```

* If the file exists, you’ll see output like:

  ```
  File 'test1.txt' loaded successfully.
  Stats:
    - Lines: 12
    - Words: 84
    - Characters: 512
    - Most Frequent Word: "data" (6 times)

  Warning: File contains unauthorized word(s): ["hack", "malware"]
  ```

* After this, you can enter another file or type `exit` to quit the program.

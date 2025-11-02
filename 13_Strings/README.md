# Rust Strings

This module explores **Strings in Rust**, covering key concepts such as `String` vs `&str`, user input, common string methods, concatenation, and the `format!` macro. Each file in the `strings` directory demonstrates a specific concept with hands-on examples.

## Project Structure

```
13_Strings/
├── strings/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       └── bin/
│           ├── collecting_user_input_with_read_line_method.rs
│           ├── common_string_methods.rs
│           ├── concatenation.rs
│           ├── review_of_strings.rs
│           └── the_format_macro.rs
└── project/
    ├── Cargo.toml
    └── src/
        ├── main.rs
        └── bin/
            └── *.rs  ← Final practice projects
```

## What You Will Learn

### 1. Collecting User Input

* **File**: `collecting_user_input_with_read_line_method.rs`
* Learn how to accept user input and handle potential errors using the `read_line` method from `std::io`.

### 2. Common String Methods

* **File**: `common_string_methods.rs`
* Explore trimming whitespace, case conversion, replacing characters, and splitting strings into vectors.

### 3. String Concatenation

* **File**: `concatenation.rs`
* Demonstrates combining strings using the `+` operator and shows how ownership affects string usage.

### 4. Review of Strings

* **File**: `review_of_strings.rs`
* Reviews the differences between `String` and `&str`, conversions, slicing, and immutability.

### 5. The `format!` Macro

* **File**: `the_format_macro.rs`
* Learn how to format strings using the powerful `format!` macro, including indexed placeholders.

---

## Running the Examples

Navigate to the `strings` directory and run an example with:

```bash
cd 13_Strings/strings
cargo run --bin <filename_without_extension>
```

### Example:

```bash
cargo run --bin common_string_methods
```

---

# Practice Project

## Mini Information Retrieval System with Enhanced Features

### Objective:

Build a mini search engine that allows users to query a list of stored text items and return **ranked and highlighted results** based on their query. The system should support **multi-word queries**, **partial matching**, and **keyword highlighting** to simulate core behaviors of modern retrieval systems.

---

### Background:

Simple search functions often return results that just *contain* the query. Real-world retrieval systems (like Google or DuckDuckGo) go further:

* **Rank results** based on relevance.
* **Highlight matches** to show users why something was returned.
* **Handle complex queries**, including multi-word or partially typed terms.

This project bridges the gap between basic filtering and intelligent retrieval.

---

### Functional Requirements:

1. **Input Corpus**:
   A vector of strings simulating documents, sentences, or titles.

2. **User Input**:
   Accept a search query via terminal input.

3. **Partial & Multi-word Matching**:

   * Match **any word** from the query to the items.
   * Handle **case-insensitive** comparisons.
   * Handle **partial words** (e.g., "prog" should match "Programming").

4. **Scoring System**:

   * Assign a **score** to each item based on:

     * The number of matching keywords.
     * The presence of partial matches.
   * Sort results by score (highest first).

5. **Keyword Highlighting**:

   * Display matched keywords in **bold**, **uppercase**, or use brackets to emphasize (e.g., `Rust is [fast] and [safe]`).

6. **User Output**:

   * Display ranked, highlighted results.
   * If no results, display a helpful message.

---

### You are expected to try the projects before checking the solutions. The solutions are available at:

    ```
    13_Strings/project/src/main.rs
    ```

**To run any of them**:

```bash
cd 13_Strings/project
cargo run
```

---

## Example Usage: Mini Search Engine

### Terminal Run

#### Prompt:

```
Enter your search query (you can use multiple words):
```

#### User Input:

```
rust data
```

### Output:

```
Search Results:
[2 match(es)] [Rust] is fast and safe
[2 match(es)] Data analysis with [Rust] is powerful
[1 match(es)] [Rust] ownership model prevents bugs
[1 match(es)] Python is great for [data] science
```

---

### Explanation:

* The query `"rust data"` is split into two keywords: `["rust", "data"]`
* The engine searches all documents for partial, case-insensitive matches.
* Each document is scored based on **how many keyword matches** it contains.
* Matching words are highlighted using brackets `[ ]`.
* Results are sorted by score in descending order.

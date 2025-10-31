# Rust Vectors

This module introduces **vectors** in Rust, covering core concepts such as creating vectors, accessing and modifying elements, managing ownership, handling out-of-bound access safely, and understanding how vector capacity works internally. Each file in the `vectors` directory provides a focused example to help you grasp vector manipulation effectively.

## Project Structure

```
12_Vectors/
├── vectors/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       └── bin/
│           ├── adding_and_removing_elements.rs
│           ├── create_a_vector.rs
│           ├── ownership_with_vectors.rs
│           ├── reading_vector_elements.rs
│           ├── the_get_method.rs
│           ├── vector_capacity_behind_the_scenes.rs
│           └── writing_vector_elements.rs
│
├── project/  # A project to solidify your learning
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       └── bin/
│           └── *.rs
```

---

## What You Will Learn

### 1. Creating a Vector

* **File**: `create_a_vector.rs`
* Learn to create vectors with type annotations and different data types.

### 2. Adding and Removing Elements

* **File**: `adding_and_removing_elements.rs`
* Use `.push()`, `.insert()`, `.pop()`, and `.remove()` to manage vector items.

### 3. Ownership with Vectors

* **File**: `ownership_with_vectors.rs`
* Understand how ownership works when storing `String`s in a vector.

### 4. Reading Vector Elements

* **File**: `reading_vector_elements.rs`
* Learn to access single or multiple elements using indexing and slicing.

### 5. Safe Access with `.get()`

* **File**: `the_get_method.rs`
* Handle out-of-bound access gracefully using `.get()` with `Option`.

### 6. Writing to Vector Elements

* **File**: `writing_vector_elements.rs`
* Modify existing values by indexing or using mutable references.

### 7. Vector Capacity Internals

* **File**: `vector_capacity_behind_the_scenes.rs`
* Explore how capacity changes behind the scenes as you add elements.

---

## Running the Examples

Navigate to the `vectors` directory and run any example using:

```bash
cd 12_Vectors/vectors
cargo run --bin <filename_without_extension>
```

### Example:

```bash
cargo run --bin create_a_vector
```

---

## Practice Project

## Mini Bank Account Simulator (with User Accounts)

### Problem Statement

> Create a **Mini Bank Account Simulator** where users can interact with their personal simulated bank account through the command line. The program first authenticates the user by requesting their username and password. If the user does not have an account, they can register by providing their full name, date of birth, and password. The system maintains a simple customer database initialized with five customers. Once logged in, users can deposit, withdraw, check their balance, and view their transaction history. All transactions are stored in a vector associated with each user. The session ends when the user chooses to exit.


### Requirements

1. Maintain a customer database with at least five initial users (username, date of birth, password, balance, and transaction history).
2. On program start, prompt the user to enter their username.

   * If username exists, request a password and allow up to three attempts.
   * If authentication fails, exit the program.
   * If username does not exist, ask if the user wants to register.

     * If yes, collect full name, date of birth, and password to create a new account with zero balance and empty transaction history.
     * If no, exit the program.
3. After successful login, show the user a menu:

   * `1. Deposit`
   * `2. Withdraw`
   * `3. Check Balance`
   * `4. View Transaction History`
   * `5. Exit`
4. Based on the user’s input:

   * **Deposit**: Ask for an amount, increase the user’s balance, and store `"Deposited X"` in their transaction history.
   * **Withdraw**: Ask for an amount. If the balance is sufficient, subtract it and store `"Withdrew X"`; otherwise, show `"Insufficient balance."`
   * **Check Balance**: Show the current balance.
   * **View Transaction History**: Print all entries from the user’s transaction history.
   * **Exit**: Quit the program with a goodbye message.
5. Loop the menu until the user chooses to exit.

---

### You are expected to try the projects before checking the solutions. The solutions are available at:

    ```
    12_Vectors/project/src/main.rs
    ```

**To run any of them**:

```bash
cd 12_Vectors/project
cargo run
```
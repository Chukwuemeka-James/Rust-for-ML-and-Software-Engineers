# Rust HashMaps and HashSets

This module introduces the **HashMap** and **HashSet** collections in Rust. You'll learn how to create, access, modify, and perform operations on hash-based data structures. The examples focus on ownership, mutation, and common operations like insertion, removal, unions, differences, and lookups.

## Project Structure

```
14_HashMaps/
├── hash_maps/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       └── bin/
│           ├── access_a_value_by_key.rs
│           ├── create_a_hashmap_with_new_function.rs
│           ├── hashmaps_and_ownership.rs
│           ├── hashset_operations.rs
│           ├── overwriting_a_value_with_an_existing_key.rs
│           ├── the_entry_method.rs
│           ├── the_hashset.rs
│           └── the_remove_method.rs
└── project/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

---

## What You Will Learn

### 1. Create a HashMap Using `.new()`

* **File**: `create_a_hashmap_with_new_function.rs`
* Demonstrates how to initialize and populate a `HashMap` with various key-value pairs.

### 2. Accessing a Value by Key

* **File**: `access_a_value_by_key.rs`
* Shows how to retrieve a value using `.get()` and safely unwrap using `.unwrap_or()`.

### 3. Overwriting a Value for an Existing Key

* **File**: `overwriting_a_value_with_an_existing_key.rs`
* Illustrates how inserting a value with an existing key replaces the old value.

### 4. The `entry` Method

* **File**: `the_entry_method.rs`
* Demonstrates conditional insertion with `.entry().or_insert()`.

### 5. HashMap Ownership Rules

* **File**: `hashmaps_and_ownership.rs`
* Explains how references and ownership behave when inserting data into a `HashMap`.

### 6. Removing Entries from a HashMap

* **File**: `the_remove_method.rs`
* Shows how to remove keys using `.remove()` and handle the returned `Option`.

### 7. Working with HashSets

* **File**: `the_hashset.rs`
* Introduces `HashSet`, including operations like `.insert()`, `.remove()`, `.contains()`, and `.get()`.

### 8. Set Operations with HashSets

* **File**: `hashset_operations.rs`
* Demonstrates advanced operations:

  * `union`
  * `difference`
  * `symmetric_difference`
  * `is_disjoint`
  * `is_subset`
  * `is_superset`

## Running the Examples

Navigate to the `hash_maps` directory and run an example with:

```bash
cd 14_HashMaps/hash_maps
cargo run --bin <filename_without_extension>
```

### Example:

```bash
cargo run --bin access_a_value_by_key
```

---

## Practice Project Social Network Friends List

**Objective:**
Build a simple social network friends management system using Rust’s `HashSet` and `HashMap` collections.

**Background:**
In social networks, each user maintains a list of friends. This project models such a network by storing users and their friends efficiently. The goal is to practice working with nested hash-based collections and set operations to manage and analyze friend relationships.

**Requirements:**

* Represent each user by a unique name (string).
* Store each user’s friends as a `HashSet` of user names to avoid duplicates and allow fast lookups.
* Implement the following features:

  1. **Add Friend:** Add a friendship link between two users.
  2. **Remove Friend:** Remove a friendship link between two users.
  3. **List Friends:** Retrieve the list of friends for a specific user.
  4. **Find Mutual Friends:** Given two users, find the intersection of their friend lists.
  5. **Suggest New Friends:** For a given user, suggest new friends based on their friends’ friends who are not already their friends.

**Constraints:**

* A user cannot be friends with themselves.
* All data must be stored using Rust’s `HashMap` and `HashSet` collections.
* Operations should be efficient, leveraging set operations like union, intersection, and difference where appropriate.


**How The Project should Work:**

* `SocialNetwork` struct holds a `HashMap<String, HashSet<String>>` — user to friends.
* `add_friend` adds users if they don't exist, then adds friendship both ways.
* `remove_friend` removes the link from both users’ friend sets.
* `list_friends` returns a reference to the friend set for a user.
* `mutual_friends` returns intersection of two friend sets.
* `suggest_friends` looks at friends-of-friends and suggests those who are not already friends or the user themself.


### You are expected to try the projects before checking the solutions. The solutions are available at:

```
14_HashMaps/project/src/main.rs
```

**To run any of them**:

```bash
cd 13_Strings/project
cargo run
```
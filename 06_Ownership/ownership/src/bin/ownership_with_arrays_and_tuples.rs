fn main() {
    // Example 1: Tuple with simple `bool` values
    let registrations = (true, false, true);

    // Copying the first element: `bool` implements `Copy`, so no ownership issues
    let first = registrations.0;
    println!("{first} and {registrations:?}");

    // Example 2: Tuple with heap-allocated `String` values
    let languages = (String::from("Rust"), String::from("JavaScript"));

    // Borrowing the first element by reference to avoid moving ownership
    let first = &languages.0;
    println!("{first} and {languages:?}");
}

/*
Learning points

1. **Tuples can store different types** – You can mix primitive types and heap-allocated types like `String` in the same tuple.
2. **Copy vs. Move in tuples** – For `Copy` types (e.g., `bool`, `i32`), accessing elements creates a copy. For non-`Copy` types (e.g., `String`), direct access would move ownership.
3. **Borrowing to avoid moves** – Using a reference (`&languages.0`) lets you access the data without transferring ownership, so the whole tuple remains valid.
4. **Debug printing** – The `:?` format specifier prints tuples and other types that implement the `Debug` trait.
*/
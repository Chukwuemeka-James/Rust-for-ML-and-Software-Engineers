fn main() {
    // Create a mutable String on the heap
    let mut name = String::from("Boris");
    println!("{name}"); // Prints: Boris

    // Append more text to the String
    name.push_str(" Pask");
    println!("{name}"); // Prints: Boris Pask

    // Append more text again
    name.push_str("haver");
    // No println! here, so final result ("Boris Paskhaver") is not shown
}

/*
Key points

1. `mut` is required when you want to change the value of a variable.
2. `String::from("Boris")` creates a heap-allocated string.
3. `.push_str("...")` appends to the end of the existing string.
4. Each mutation updates the same memory rather than creating a copy.
5. Even if you mutate the value, the **ownership** stays with the same variable.
*/
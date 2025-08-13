fn main() {
    // Create a mutable String
    let mut coffee = String::from("Mocha");

    // Create a mutable reference to `coffee`
    let a = &mut coffee;
    println!("{a}"); // Prints the value through the mutable reference

    // Move the mutable reference `a` into `b`
    let b = a;
    println!("{b}"); // Prints the value through `b`
}

/*
**Learning points for students:**

1. **Mutable reference creation** – `&mut coffee` allows you to modify or read `coffee` while borrowing it mutably.
2. **Only one mutable reference at a time** – Rust enforces exclusive access to prevent data races.
3. **Moving references** – Just like with values, references themselves can be moved. After `let b = a;`, `a` is no longer valid, and `b` is now the owner of that mutable reference.
4. **Scope of borrow** – The borrow ends when `b` goes out of scope, at which point `coffee` can be used again.
*/
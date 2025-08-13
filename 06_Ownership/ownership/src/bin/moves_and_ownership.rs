fn main() {
    // Create a String stored on the heap and bind it to variable `person`.
    let person = String::from("Boris");
    println!("My name is {person}");

    // Ownership of the String is moved from `person` to `genius`.
    let genius = person;

    // This line would cause a compile-time error because `person` no longer owns the String.
    // println!("My name is {person}");
}

/* 
Learning points:

1. **Move semantics in Rust** – When assigning a `String` (or any non-Copy type) to another variable, ownership is **moved**, not duplicated.
2. **Invalidated binding** – After the move, the original variable (`person`) can no longer be used; attempting to do so will cause a compiler error.
3. **Heap data ownership** – `String` owns its heap data, and ownership can only belong to **one variable** at a time.
*/
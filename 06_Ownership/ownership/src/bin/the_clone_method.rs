fn main() {
    // Create a String (heap-allocated data)
    let person = String::from("Boris");

    // Clone the String: deep copy of heap data
    // Both 'person' and 'genius' now have their own separate copies of the String data in memory
    let genius = person.clone();

    // Since we cloned, 'person' is still valid and can be used
    println!("This is {person}.");
}

/*
Learning points

1. **Heap allocation** – `String` stores its data on the heap, with only a pointer, length, and capacity on the stack.
2. **Move vs clone** – Normally, assigning `person` to `genius` would move ownership, making `person` invalid. Using `.clone()` instead performs a **deep copy** so both variables are valid and independent.
3. **Memory impact** – Cloning allocates new memory on the heap, which can be more expensive than a move.
4. **Ownership safety** – Cloning is useful when you truly need separate ownership of the same data.
5. **Printing** – Since `person` remains valid, we can still use it after cloning.
*/
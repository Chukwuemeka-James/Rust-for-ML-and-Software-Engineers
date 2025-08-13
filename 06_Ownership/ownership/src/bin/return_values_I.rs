fn main() {
    // Call bake_cake, which returns ownership of a String to the caller
    let cake = bake_cake();
    println!("I now have a {cake} cake!");
}

fn bake_cake() -> String {
    // Creates a new heap-allocated String and transfers ownership to the caller
    String::from("Chocolate Mousse")
}

/*
**Learning points

1. **Returning ownership** – The function `bake_cake` creates a `String` and *returns ownership* of it to the caller.
2. **Heap allocation** – The `String` value itself is stored on the heap, but its pointer, length, and capacity live on the stack.
3. **Move semantics** – When `bake_cake` returns, Rust moves the `String` into `cake` without copying the heap data.
4. **No borrowing here** – The data isn’t borrowed, so `main` fully owns `cake` after the call.
5. **Automatic cleanup** – When `cake` goes out of scope, Rust automatically frees the heap memory.

*/
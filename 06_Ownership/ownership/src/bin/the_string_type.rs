fn main() {
    // Creates a new, empty String on the heap
    let text: String = String::new(); 
    // At this point, text = "" (length 0, capacity 0)

    // Creates a new String with initial content "KitKat"
    let candy = String::from("KitKat");
    // At this point, candy = "KitKat"
}

/*
Key points

1. Both variables are **heap-allocated `String`** types (not string slices).
2. `String::new()` → creates an empty, growable string.
3. `String::from("...")` → creates a string with predefined content.
4. Even though `text` is empty now, you can later add data using `.push_str()` or `.push()`.
5. Type annotation `: String` is optional here — Rust would infer it.
*/
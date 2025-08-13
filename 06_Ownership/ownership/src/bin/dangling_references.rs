fn main() {
    // Here, we call a function that RETURNS a String.
    // The returned String's ownership is transferred to `city`.
    let city = create_city();

    // We now own `city`, so we can use it.
    println!("{city}");
}

// This function creates a new String and returns it.
// Returning the value transfers ownership to the caller.
fn create_city() -> String {
    String::from("New York")
}



/*
Learning points:

1. **Returning ownership** – In Rust, when a function returns a value, it transfers ownership of that value to the calling scope.
2. **No borrowing here** – We’re not passing references, so the original variable created inside `create_city()` no longer exists after the function ends.
3. **No `Copy` trait here** – Since `String` does not implement `Copy`, ownership must be explicitly transferred (either by move or by return).

*/
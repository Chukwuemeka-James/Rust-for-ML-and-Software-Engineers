fn main() {
    // Declare an integer variable stored on the stack
    let age = 33;

    // Declare a boolean variable stored on the stack
    let is_handsome = true;

    // Print both variables
    println!("{age}");
    println!("{is_handsome}");

    // The variables age and is_handsome are still in scope here
} // <-- is_handsome goes out of scope first, then age also goes out of scope


/*
Learning points

1. **Stack-allocated variables** – Both `age` and `is_handsome` are simple data types stored entirely on the stack.
2. **Scope** – Variables exist from the point they are declared until the end of the enclosing block (`{}`).
3. **Drop order** – When the function ends, variables go out of scope in reverse order of declaration (last in, first out).
4. **No ownership complexity here** – Since these are `Copy` types, there’s no move or borrowing concept needed; copies are made automatically if used elsewhere.
5. **Why comments about scope matter** – Understanding where a variable exists in memory is crucial for resource management in Rust.
*/
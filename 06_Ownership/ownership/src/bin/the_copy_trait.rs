fn main() {
    // 'time' is an integer (i32 by default), stored entirely on the stack
    let time = 2025;

    // Copy the value of 'time' into 'years'
    // Integers implement the Copy trait, so this is a simple, cheap stack copy
    let years = time;

    // Both 'time' and 'years' are valid and independent values
    println!("The time is {time}. It is the year {years}.");
}

/*
Learning points

1. **Primitive types like integers** are stored entirely on the stack.
2. Rust automatically copies these values when assigned to another variable because they implement the `Copy` trait.
3. **No ownership move happens**—both `time` and `years` remain valid.
4. **Copy vs Move** – Unlike heap-allocated types (e.g., `String`), primitives don’t require ownership transfer or borrowing.
*/
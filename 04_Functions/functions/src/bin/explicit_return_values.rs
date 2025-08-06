// At the end of this lesson, you will learn:
// - How to use the `return` keyword to explicitly return a value from a function.
// - The difference between implicit and explicit returns in Rust.
// - That using `return` requires a semicolon, while implicit returns must omit it.

fn main() {
    let result = square(5);
    println!("The square of 5 is {}", result);

    let result = square(13);
    println!("The square of 13 is {}", result);
}

// This function explicitly returns the square of a number.
// The 'return' keyword is used, and the statement ends with a semicolon.
fn square(number: i32) -> i32 {
    return number * number;
}
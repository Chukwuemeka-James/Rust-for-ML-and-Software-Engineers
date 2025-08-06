// At the end of this lesson, you will learn:
// - How Rust functions can return values implicitly.
// - That the last expression (without a semicolon) is returned.
// - How to work with return types and use them in variables.

fn main() {
    let result = square(5);
    println!("The square of 5 is {}", result);

    let result = square(13);
    println!("The square of 13 is {}", result);
}

// This function takes a number and returns its square.
// The return value is *implicit* — no 'return' keyword is used.
fn square(number: i32) -> i32 {
    number * number  // Last expression is automatically returned
}


fn main() {
    let val: i32 = -15;

    // `abs()` returns the absolute value of a number
    println!("Absolute value: {}", val.abs());

    let empty_space = "     my content    ";

    // `trim()` removes leading and trailing whitespace from a string slice
    println!("Trimmed content: '{}'", empty_space.trim());

    // `pow()` raises the number to the power of the given exponent
    println!("val squared: {}", val.pow(2));
    println!("val cubed: {}", val.pow(3));
}

// In this example, you'll learn how to declare and use floating-point numbers in Rust.
// Rust has two floating-point types: `f32` (32-bit) and `f64` (64-bit, default).
// You'll also see how to format output and use float methods like `.floor()`, `.ceil()`, and `.round()`.

fn main() {
    // f64 is the default type for floating-point numbers
    let pi: f64 = 3.1415926535897932384;
    println!("The full value of pi (f64) is: {pi}");

    // Display pi with only 4 decimal places
    println!("The current value of pi (4 decimal places): {:.4}", pi);

    // A smaller, less precise floating-point type
    let e: f32 = 2.71828;
    println!("The value of e (f32) is: {}", e);

    // Demonstrating common float methods
    println!("floor(pi): {}", pi.floor());   // Largest integer <= pi
    println!("ceil(pi): {}", pi.ceil());     // Smallest integer >= pi
    println!("round(pi): {}", pi.round());   // Nearest integer

    // Arithmetic with floats
    let radius = 5.0;
    let area = pi * radius * radius;
    println!("Area of circle: {}", area);
}

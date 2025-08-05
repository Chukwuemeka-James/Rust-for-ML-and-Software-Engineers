// In this example, you'll learn how to work with Boolean values in Rust.
// Booleans are either `true` or `false` and are used for making decisions.
// You'll also see how Boolean values can be the result of comparisons or method calls.

fn main() {
    // Boolean literals
    let is_active: bool = true;
    let is_admin: bool = false;

    println!("Is active: {}", is_active);
    println!("Is admin: {}", is_admin);

    // Boolean from a comparison expression
    let age: i32 = 28;
    let is_young = age < 35; // true, because 28 < 35

    println!("Is young: {}", is_young);

    // Boolean from method calls on integers
    println!("Is age positive? {}", age.is_positive());
    println!("Is age negative? {}", age.is_negative());
}

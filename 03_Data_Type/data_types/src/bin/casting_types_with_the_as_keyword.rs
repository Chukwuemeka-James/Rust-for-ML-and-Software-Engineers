// In this example, you'll learn how to convert values from one type to another using the `as` keyword in Rust.
// Rust requires explicit casting between different types to avoid unintended behavior.

fn main() {
    // Original integer value (i32 by default)
    let miles_away = 50;

    // Casting to a signed 8-bit integer (i8)
    let miles_away_i8 = miles_away as i8;

    // Casting to an unsigned 8-bit integer (u8)
    let miles_away_u8: u8 = miles_away as u8;

    println!("miles_away as i8: {miles_away_i8}");
    println!("miles_away as u8: {miles_away_u8}");
    println!("------------------------------------");

    // Floating-point value (f64 by default)
    let miles_away: f64 = 100.329032;

    // Casting from f64 to f32 (narrowing conversion, potential precision loss)
    let miles_away_f32 = miles_away as f32;

    // Casting from f64 to i32 (fractional part is discarded)
    let miles_away_int = miles_away as i32;

    println!("miles_away as f32: {miles_away_f32}");
    println!("miles_away as i32: {miles_away_int}");
}

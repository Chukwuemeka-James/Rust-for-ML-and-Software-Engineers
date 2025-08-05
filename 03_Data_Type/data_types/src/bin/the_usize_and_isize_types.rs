// In this example, you'll learn about the `usize` and `isize` types in Rust.
// These are pointer-sized integers — their size depends on the architecture of your computer (e.g., 64-bit or 32-bit).
// `usize` is unsigned and commonly used for indexing collections; `isize` is signed and used when negative values are needed.

fn main() {
    // `usize` is typically used for indexing arrays or collections, and it's always non-negative
    let days: usize = 55;

    // `isize` is a signed integer with the same size as `usize`, useful when values may be negative
    let years: isize = -15_000;

    // Print the actual values
    println!("days (usize): {}", days);
    println!("years (isize): {}", years);
}

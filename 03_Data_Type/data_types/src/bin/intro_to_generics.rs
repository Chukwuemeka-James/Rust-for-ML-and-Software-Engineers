// In this example, you'll get an introduction to **Generics** in Rust.
// Generics let us write code that works with many different types while ensuring type safety.
// Here, we use the generic `Range<T>` type from the standard library, which can represent a range of values for different types (like `i32` or `char`).
// This is meant to show that the same data structure (`Range<T>`) can work with multiple data types, thanks to Rust's generics system.

fn main() {
    // std::ops = Standard Library Operations Module

    let month_days: std::ops::Range<i32> = 1..31; // Range from 1 to 30 (i32 type)
    let letters: std::ops::Range<char> = 'b'..'f'; // Range from 'b' to 'e' (char type)

    // You can use these ranges in loops or inspect their values
    for day in month_days {
        print!("{day} ");
    }

    println!();

    for letter in letters {
        print!("{letter} ");
    }
    println!()
}

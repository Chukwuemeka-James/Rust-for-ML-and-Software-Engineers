// In this example, you'll learn how to use augmented assignment operators in Rust.
// These operators combine arithmetic with assignment (like +=, -=, *=, /=)
// and provide a concise way to update variable values without repeating the variable name.

fn main() {
    let mut year = 2025;

    // Add 1 to `year` (year = year + 1)
    year += 1;
    println!("The new year is {year}");

    // Subtract 5 from `year`
    year -= 5;
    println!("The new year is {year}");

    // Multiply `year` by 2
    year *= 2;
    println!("The new year is {year}");

    // Divide `year` by 4
    year /= 4;
    println!("The new year is {year}");
}

// In this example, you'll learn how to print different types of values in Rust.
// You'll also see how to use debug formatting (`{:?}` and `{:#?}`) and the `dbg!` macro
// to inspect arrays and other values during development.

fn main() {
    let seasons = ["Spring", "Summer", "Fall", "Winter"];

    // Print an integer
    println!("{}", 5);

    // Print a floating-point number
    println!("{}", 3.14);

    // Print a boolean
    println!("{}", true);

    // Pretty-print the array using debug formatting with indentation
    println!("{:#?}", seasons);

    // Compact debug print of the array (no indentation)
    println!("{:?}", seasons);

    // Use the `dbg!` macro to print both the variable name and value (along with file + line info)
    dbg!(seasons);
}

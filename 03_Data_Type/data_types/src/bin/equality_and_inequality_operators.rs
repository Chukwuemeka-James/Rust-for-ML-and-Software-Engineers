// In this example, you'll learn how to use equality (==) and inequality (!=) operators in Rust.
// These operators let you compare values of the same type, such as strings, numbers, and booleans.
// Rust enforces type safety, so only values of the same type can be compared directly.

fn main() {
    // String comparisons (case-sensitive and whitespace-sensitive)
    println!("{}", "Coke" == "Pepsi");   // false
    println!("{}", "Coke" != "Pepsi");   // true
    println!("{}", "Coke" == "coke");    // false (case matters)
    println!("{}", "Coke" == "Coke ");   // false (extra space)
    println!("{}", "Coke" == "Coke");    // true

    // Integer comparisons
    println!("{}", 13 == 13);            // true
    println!("{}", 13 != 13);            // false

    // Floating-point comparisons
    println!("{}", 26.1 == 26.1);        // true
    println!("{}", 26.1 == 26.14);       // false

    // Mixed integer and float comparison (explicit cast required)
    println!("{}", 13 == 13.1 as i32);   // true (13.1 -> 13)

    // Boolean comparisons
    println!("{}", true == true);        // true
    println!("{}", false == false);      // true
    println!("{}", true != false);       // true
}

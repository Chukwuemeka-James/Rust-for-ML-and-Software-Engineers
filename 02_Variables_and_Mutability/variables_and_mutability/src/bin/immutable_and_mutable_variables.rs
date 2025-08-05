fn main() {
    // Immutable variable
    let x = 10;
    println!("Immutable variable x: {x}");

    // Uncommenting the next line will cause a compile-time error:
    // x = 20; // Error: cannot assign twice to immutable variable

    // Mutable variable
    let mut y = 5;
    println!("Mutable variable y before change: {}y");

    y = 15; // Reassigning is allowed because y is mutable
    println!("Mutable variable y after change: {}y");

}

// In this example, you'll learn how to declare and work with arrays in Rust.
// You'll see how to define arrays with specific types and lengths, how to get their size,
// and how to create an empty array.

fn main() {
    // An array of six integers (type i32)
    let numbers: [i32; 6] = [4, 8, 15, 16, 23, 42];
    println!("Number of integers: {}", numbers.len());

    // An array of string slices representing apple types
    let apples: [&str; 3] = ["Granny Smith", "McIntosh", "Red Delicious"];
    println!("Number of apple types: {}", apples.len());

    // Debug-print the whole array using {:?}
    println!("Apple types: {:?}", apples);

    // An empty array of floating-point numbers (f64)
    let currency_rates: [f64; 0] = [];

    // You can't access elements of an empty array, but it still has a type and length
    println!("Currency rates array length: {}", currency_rates.len());
}

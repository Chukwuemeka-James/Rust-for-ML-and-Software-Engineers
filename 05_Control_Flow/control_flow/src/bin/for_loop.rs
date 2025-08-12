// Demonstrates `for` loops, useful for iterating over ranges, arrays, and vectors.

fn main() {
    // Loop over a range
    for num in 1..5 { // 1 to 4
        println!("Number: {num}");
    }

    // Loop over an array
    let fruits = ["apple", "banana", "cherry"];
    for fruit in fruits.iter() {
        println!("I like {fruit}");
    }

    // Loop with index
    for (index, fruit) in fruits.iter().enumerate() {
        println!("Fruit {index}: {fruit}");
    }
}

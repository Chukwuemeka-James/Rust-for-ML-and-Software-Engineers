fn main() {
    // Declare an immutable variable for the number of apples
    let apples = 50;

    // Declare another immutable variable for the number of oranges
    // Here, we are also demonstrating that you can perform arithmetic at assignment
    let oranges = 14 + 6;

    // Combine apples and oranges to get the total number of fruits
    let fruits = apples + oranges;

    // Print out the results using named variables directly
    println!("Apples: {apples}");
    println!("Oranges: {oranges}");
    println!("Total fruits: {fruits}");
}

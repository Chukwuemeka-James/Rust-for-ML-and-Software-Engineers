fn main() {
    // Declare the number of apples
    let apples = 50;

    // Declare the number of oranges and perform arithmetic during assignment
    let oranges = 14 + 6;

    // Calculate the total number of fruits
    let fruits = apples + oranges;

    // Print using named variable capture for cleaner formatting
    println!(
        "This year, my garden has {0} apples and {1} oranges. I can't believe I have {0} apples!", apples, oranges
    );

    // Optionally print the total fruits as well
    println!("In total, I have {fruits} fruits in my garden.");
}

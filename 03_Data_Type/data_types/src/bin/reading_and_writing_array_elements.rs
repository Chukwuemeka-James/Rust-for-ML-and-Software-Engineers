// In this example, you'll learn how to read and update elements in a Rust array using index access.
// Arrays are fixed-size collections, and you can access elements using square brackets like in many other languages.

fn main() {
    // Create a mutable array of 4 seasons
    let mut seasons = ["Spring", "Summer", "Fall", "Winter"];

    // Read and print the element at index 2 ("Fall")
    println!("Original third season: {}", seasons[2]);

    // Update the element at index 2 to "Autumn"
    seasons[2] = "Autumn";

    // Print the updated element
    println!("Updated third season: {}", seasons[2]);

    // Optional: print the full array after update using **Debug Trait: println!("{:?}" value)**
    println!("All seasons: {:?}", seasons);
}
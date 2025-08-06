fn main() {
    // This is the entry point of the program
    open_store();         // Call to function that prints opening message
    bake_pizza();         // Call to function that bakes a pizza
    swim_in_profit();     // Function called multiple times to show reuse
}

// Define a function to simulate opening a store
fn open_store() {
    println!("Opening my pizza store");
}

// Define a function to simulate baking pizza
fn bake_pizza() {
    println!("Baking a pizza");
}

// Define a function that shows we're making money!
fn swim_in_profit() {
    println!("So much profit, so little time");
}


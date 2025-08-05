// Create a type alias for clarity and better readability
type Minutes = u32;

fn main() {
    // Define workout durations using the Minutes alias
    let warm_up: Minutes = 10;
    let main_workout: Minutes = 30;
    let cool_down: Minutes = 5;

    // Calculate the total workout time
    let total_time: Minutes = warm_up + main_workout + cool_down;

    // Print the results using named placeholders
    println!("Warm-up takes {warm_up} minutes.");
    println!("Main workout takes {main_workout} minutes.");
    println!("Cool-down takes {cool_down} minutes.");
    println!("In total, your workout will take {total_time} minutes!");
}

/*
This example demonstrates the use of a type alias in Rust. By creating a custom alias for a base type (e.g. type Minutes = u32), 
you can give semantic meaning to your variables. Although Minutes is still a u32 under the hood, 
using an alias makes your code easier to read and understand by expressing intent clearly.
*/
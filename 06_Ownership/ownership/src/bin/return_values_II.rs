fn main() {
    // Create a new empty String for the meal
    let mut current_meal = String::new();
    
    // Pass ownership of current_meal into add_flour
    // and receive the updated String back
    current_meal = add_flour(current_meal);

    // Example of how other steps could be added:
    // current_meal = add_sugar(current_meal);
    // add_salt(); // This could modify state differently
}

fn add_flour(mut meal: String) -> String {
    // Take ownership of meal (String) and make it mutable
    meal.push_str("Add flour"); // Append text to the String
    meal // Return ownership of the updated String to the caller
}

/*
Learning points for students:**

1. **Ownership transfer into a function** – `current_meal` is moved into `add_flour` since it’s passed by value.
2. **Mutable parameter** – Inside `add_flour`, the parameter is declared `mut` so we can change the String’s content.
3. **Returning updated data** – Ownership is returned to `main` so the updated value can be used further.
4. **Why not borrow?** – This example teaches ownership passing rather than borrowing; borrowing would look different (`&mut`).
5. **Function chaining** – This structure supports chaining multiple steps, each consuming and returning the meal.
*/
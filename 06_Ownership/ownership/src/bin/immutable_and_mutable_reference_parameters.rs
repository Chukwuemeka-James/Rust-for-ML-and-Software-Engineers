fn main() {
    // Create a new, empty String that will hold meal preparation steps.
    let mut current_meal = String::new();

    // Pass a mutable reference to allow modification of the String in place.
    add_flour(&mut current_meal);

    // Pass an immutable reference to only read (not modify) the String.
    show_my_meal(&current_meal);
}

// Accepts a mutable reference so it can modify the original String in main.
fn add_flour(meal: &mut String) {
    meal.push_str("Add flour");
}

// Accepts an immutable reference so it can read the String without taking ownership.
fn show_my_meal(meal: &String) {
    println!("Meal steps: {meal}");
}

/* 
**Learning points for students:**

1. **Mutable vs. immutable references** – You can borrow a value mutably when you need to change it, and immutably when you only need to read it.
2. **No ownership transfer** – Because we use references (`&String` or `&mut String`), the original `String` stays owned by `main`.
3. **Borrow checker rules** – At any point in a scope, you can have **either** multiple immutable references or one mutable reference, but not both at the same time.
*/
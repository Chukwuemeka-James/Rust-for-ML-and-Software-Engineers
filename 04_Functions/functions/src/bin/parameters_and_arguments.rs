// At the end of this class, students will learn:
// - How to define and call functions with parameters and arguments.
// - How to use different data types (e.g., &str, i32) in functions.
// - How Rust handles string formatting inside println! macros.


fn main() {
    open_store("Brooklyn");
    bake_pizza(20, "pepperoni");
    swim_in_profit();
    cloth_maker("Smauel", 5);
}

// Function with a string parameter to specify store location
fn open_store(neighborhood: &str) {
    println!("Opening my pizza store in {}", neighborhood);
}

// Function with two parameters: a number and a topping
fn bake_pizza(number: i32, topping: &str) {
    println!("Baking {} {} pizzas", number, topping);
}

// Function with no parameters, called multiple times
fn swim_in_profit() {
    println!("So much money, so little time");
}

fn cloth_maker(name: &str, counts: i64){
    println!("Hello {name} we have made {counts} of your cloths as you requested");
}
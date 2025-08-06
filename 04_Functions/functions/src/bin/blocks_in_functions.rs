// At the end of this lesson, you will learn:
// - That blocks in Rust (i.e., code inside `{}`) can return values.
// - How block expressions can be used in variable assignments and functions.
// - That the last expression in a block determines the block’s return value.


fn main() {
    let multiplier = 3;

    // This block acts like an expression that returns a value
    let calculation = {
        let value = 5 + 4;
        value * multiplier  // Last expression is returned implicitly
    };

    println!("Result from block in main: {}", calculation);

    // Demonstrating block expression inside a function
    let bonus = bonus_points(10);
    println!("Bonus points earned: {}", bonus);
}

// Function using a block to compute return value
fn bonus_points(base: i32) -> i32 {
    let points = {
        let factor = 2;
        base * factor  // Implicit return from block
    };

    points + 5  // Final value returned from function
}


// In this example, you'll learn how to use ranges and iterate over them in Rust.
// You'll see how `..` (exclusive) and `..=` (inclusive) work with numbers and characters.
// You'll also loop through arrays using a `for` loop.

fn main() {
    // Exclusive range: goes from 1 up to (but not including) 31
    let month_days = 1..31;
    println!("Exclusive range (1..31): {:?}", month_days);

    // Inclusive range: goes from 1 to 31, including 31
    let month_days = 1..=31;
    println!("Inclusive range (1..=31): {:?}", month_days);

    // Iterate through the inclusive range and print each day number
    for number in month_days {
        print!("{number} ");
    }
    println!(); // just to add a newline

    // Range of characters from 'a' to 'e' (excludes 'f')
    let letters = 'a'..'f';
    for letter in letters {
        println!("Letter: {}", letter);
    }

    // Looping through an array of strings
    let colors = ["Red", "Green", "Yellow"];
    for color in colors {
        println!("{} is a great color!", color);
    }
}

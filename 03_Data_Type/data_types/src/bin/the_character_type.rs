// In this example, you'll learn how to use the `char` type in Rust, which represents a single Unicode character.
// You'll also explore built-in methods like `.is_alphabetic()`, `.is_uppercase()`, and `.is_lowercase()`
// to analyze character properties — even for emojis and special symbols.

fn main() {
    // A regular ASCII alphabetic character
    let first_initial = 'B';

    // A Unicode emoji character
    let emoji = '🎧';

    // Check if each character is alphabetic (letters only)
    println!(
        "Alphabetic? B: {}, 🎧: {}", first_initial.is_alphabetic(), emoji.is_alphabetic()
    );

    // Check if each character is uppercase
    println!(
        "Uppercase? B: {}, 🎧: {}", first_initial.is_uppercase(), emoji.is_uppercase()
    );

    // Check if each character is lowercase
    println!(
        "Lowercase? B: {}, 🎧: {}", first_initial.is_lowercase(), emoji.is_lowercase()
    );
}

// Demonstrates the basic `loop` keyword in Rust.
// A `loop` runs forever unless you explicitly stop it using `break`.

fn main() {
    let mut counter = 0;

    loop {
        println!("Counter: {counter}");
        counter += 1;

        if counter == 5 {
            println!("Breaking out of the loop.");
            break; // stops the infinite loop
        }
    }

    println!("Loop ended.");
}

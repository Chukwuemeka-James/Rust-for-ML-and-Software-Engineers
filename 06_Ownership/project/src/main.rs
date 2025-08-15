// Import the standard input/output library for reading user input
use std::io;

// Reads a line of input from the user and returns it as a trimmed String
fn read_input(prompt: &str) -> String {
    println!("{prompt}");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input"); // Handle any error if reading fails
    input.trim().to_string() // Remove whitespace/newlines and return as String
}

// Reads input from the user and ensures it is a valid floating-point number (f64)
fn read_f64(prompt: &str) -> f64 {
    loop {
        let input = read_input(prompt); // Ask user for input
        match input.parse::<f64>() { // Try to parse input to f64: Convert number in str format to f64
            Ok(val) => return val, // Return value if parsing succeeds
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

// Deposits money into the account if the amount is positive
fn deposit(balance: &mut f64, amount: f64) {
    if amount > 0.0 {
        *balance += amount; // Dereference balance and add amount
        println!("Deposit successful. New balance: ${:.2}", *balance);
    } else {
        println!("Cannot deposit a negative amount.");
    }
}

// Withdraws money from the account if sufficient funds are available
fn withdraw(balance: &mut f64, amount: f64) {
    if amount <= *balance {
        *balance -= amount; // Dereference balance and subtract amount
        println!("Withdrawal successful. New balance: ${:.2}", *balance);
    } else {
        println!("Insufficient funds. Your balance is ${:.2}", *balance);
    }
}

// Displays the account owner’s name and current balance
fn display(owner: &str, balance: f64) {
    println!("\nAccount Holder: {}", owner);
    println!("Current Balance: ${:.2}\n", balance);
}

fn main() {
    println!("Welcome to Rust Bank!");

    // Ask for account owner's name
    let owner_name = read_input("Enter your name:");
    let mut balance: f64 = 0.0; // Initialize balance to zero

    loop {
        // Show menu options
        println!("\nChoose an option:");
        println!("1 Deposit");
        println!("2 Withdraw");
        println!("3 Display Balance");
        println!("4 Exit");

        let choice = read_input("Enter your choice:"); // Read user choice

        match choice.as_str() {
            // Deposit flow
            "1" => {
                let amount = read_f64("Enter amount to deposit:");
                deposit(&mut balance, amount);
            }
            // Withdraw flow
            "2" => {
                let amount = read_f64("Enter amount to withdraw:");
                withdraw(&mut balance, amount);
            }
            // Display balance
            "3" => display(&owner_name, balance),
            // Exit program
            "4" => {
                println!("Thank you for banking with us!");
                break;
            }
            // Invalid menu option
            _ => println!("Invalid option. Please try again."),
        }
    }
}

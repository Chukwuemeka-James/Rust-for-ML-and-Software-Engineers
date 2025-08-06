fn main() {
    // Initialize a mutable account balance
    let mut account_balance: f64 = 100.0;

    println!("Welcome to the Simple Banking System!");
    println!("Your initial balance is: ${}", account_balance);

    // Deposit money
    account_balance = deposit(account_balance, 50.0);
    check_balance(account_balance);

    // Withdraw money
    account_balance = withdraw(account_balance, 30.0);
    check_balance(account_balance);

    // Attempt to withdraw more than the balance
    account_balance = withdraw(account_balance, 200.0);
    check_balance(account_balance);
}

// Function to deposit money
fn deposit(balance: f64, amount: f64) -> f64 {
    let new_balance = balance + amount;
    println!("Deposited ${}. New balance is: ${}", amount, new_balance);
    new_balance
}

// Function to withdraw money
fn withdraw(balance: f64, amount: f64) -> f64 {
    if amount <= balance {
        let new_balance = balance - amount;
        println!("Withdrew ${}. New balance is: ${}", amount, new_balance);
        new_balance
    } else {
        println!("Error: Insufficient funds! Current balance: ${}", balance);
        balance
    }
}

// Function to check the balance
fn check_balance(balance: f64) {
    println!("Current balance: ${}", balance);
}

// Define a currency type for clarity
type Currency = f64;

// Define constants
const MONTHLY_INCOME: Currency = 2000.0;
const EMERGENCY_FUND: Currency = 100.0;

fn main() {
    // Define fixed expenses
    let rent: Currency = 800.0;
    let groceries: Currency = 250.0;
    let transport: Currency = 100.0;

    // Calculate total expenses
    let total_expenses = rent + groceries + transport;

    // Shadow monthly income to compute remaining balance
    let balance = MONTHLY_INCOME - total_expenses;

    {
        // Inner scope to demonstrate scope and shadowing
        let balance = balance - EMERGENCY_FUND;
        println!("Balance after emergency fund deduction (in inner scope): ${balance:.2}");
    }

    // Final budget summary
    println!("\n====== Static Budget Summary ======");
    println!("Monthly Income: ${MONTHLY_INCOME}");
    println!("Rent: ${rent}, Groceries: ${groceries}, Transport: ${transport}");
    println!("Total Expenses: ${total_expenses}");
    println!("Remaining Balance: ${balance}");
    println!("(Emergency Fund reserved: ${EMERGENCY_FUND})");
}


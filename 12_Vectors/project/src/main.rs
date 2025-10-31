// Mini Bank Account Simulator

use std::io;

#[derive(Clone)]
struct Customer {
    name: String,
    dob: String,
    password: String,
    balance: i32,
    transactions: Vec<String>,
}

fn main() {
    let mut customers = vec![
        Customer::new("alice", "1995-01-01", "pass123", 500),
        Customer::new("bob", "1990-04-12", "bobpass", 1000),
        Customer::new("charlie", "1999-07-22", "charlie1", 250),
        Customer::new("diana", "1988-02-05", "diana88", 3000),
        Customer::new("eve", "2001-11-30", "eve321", 200),
    ];

    println!("Welcome to the Mini Bank Account Simulator!");

    let current_user_index = authenticate_or_register(&mut customers);

    // Begin session
    loop {
        println!("\nWhat would you like to do?");
        println!("1. Deposit");
        println!("2. Withdraw");
        println!("3. Check Balance");
        println!("4. Transaction History");
        println!("5. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        match input {
            "1" => {
                println!("Enter amount to deposit:");
                let amount = read_amount();
                customers[current_user_index].balance += amount;
                customers[current_user_index]
                    .transactions
                    .push(format!("Deposited ${}", amount));
                println!("Deposited ${}", amount);
            }
            "2" => {
                println!("Enter amount to withdraw:");
                let amount = read_amount();
                if amount > customers[current_user_index].balance {
                    println!("Insufficient balance.");
                } else {
                    customers[current_user_index].balance -= amount;
                    customers[current_user_index]
                        .transactions
                        .push(format!("Withdrew ${}", amount));
                    println!("Withdrew ${}", amount);
                }
            }
            "3" => {
                let balance = customers[current_user_index].balance;
                println!("Your current balance is: ${}", balance);
            }
            "4" => {
                let tx = &customers[current_user_index].transactions;
                println!("Transaction History:");
                if tx.is_empty() {
                    println!("No transactions yet.");
                } else {
                    for (i, entry) in tx.iter().enumerate() {
                        println!("{}. {}", i + 1, entry);
                    }
                }
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option. Try again.");
            }
        }
    }
}

fn authenticate_or_register(customers: &mut Vec<Customer>) -> usize {
    println!("\nEnter your username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read username");
    let username = username.trim();

    // Check if user exists
    let user_index = customers.iter().position(|c| c.name == username);

    match user_index {
        Some(index) => {
            // Authenticate
            for _ in 0..3 {
                println!("Enter your password:");
                let mut password = String::new();
                io::stdin().read_line(&mut password).expect("Failed to read password");
                let password = password.trim();

                if password == customers[index].password {
                    println!("Login successful. Welcome back, {}!", customers[index].name);
                    return index;
                } else {
                    println!("Incorrect password.");
                }
            }
            println!("Too many failed attempts. Exiting.");
            std::process::exit(1);
        }
        None => {
            println!("Account not found. Would you like to register? (yes/no)");
            let mut answer = String::new();
            io::stdin().read_line(&mut answer).expect("Failed to read answer");

            if answer.trim().to_lowercase() == "yes" {
                println!("Enter your full name:");
                let mut new_name = String::new();
                io::stdin().read_line(&mut new_name).expect("Failed to read name");
                let new_name = new_name.trim().to_string();

                println!("Enter your date of birth (YYYY-MM-DD):");
                let mut dob = String::new();
                io::stdin().read_line(&mut dob).expect("Failed to read dob");
                let dob = dob.trim().to_string();

                println!("Create a password:");
                let mut new_password = String::new();
                io::stdin().read_line(&mut new_password).expect("Failed to read password");
                let new_password = new_password.trim().to_string();

                let new_customer = Customer::new(new_name.clone(), dob, new_password, 0);
                customers.push(new_customer);
                let new_index = customers.len() - 1;
                println!("Registration complete. Welcome, {}!", new_name);
                return new_index;
            } else {
                println!("Goodbye!");
                std::process::exit(1);
            }
        }
    }
}

fn read_amount() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    match input.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number.");
            0
        }
    }
}

impl Customer {
    fn new(name: &str, dob: String, password: String, balance: i32) -> Self {
        Self {
            name: name.to_string(),
            dob,
            password,
            balance,
            transactions: vec![],
        }
    }
}

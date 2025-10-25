use std::io;

#[derive(Debug)]
enum WalletAction {
    CheckBalance,
    AddFunds(f64),
    WithdrawFunds(f64),
    ViewHistory,
    Exit,
}

struct Wallet {
    balance: f64,
    history: Vec<String>,
}

impl Wallet {
    fn new() -> Self {
        Wallet {
            balance: 0.0,
            history: Vec::new(),
        }
    }

    fn check_balance(&self) {
        println!("Current Balance: ${:.2}", self.balance);
    }

    fn add_funds(&mut self, amount: f64) {
        self.balance += amount;
        self.history.push(format!("Added ${:.2}", amount));
        println!("Funds added successfully.");
    }

    fn withdraw_funds(&mut self, amount: f64) {
        if amount <= self.balance {
            self.balance -= amount;
            self.history.push(format!("Withdrew ${:.2}", amount));
            println!("Withdrawal successful.");
        } else {
            println!("Insufficient funds.");
        }
    }

    fn view_history(&self) {
        println!("Transaction History:");
        if self.history.is_empty() {
            println!("No transactions yet.");
        } else {
            for event in &self.history {
                println!("{}", event);
            }
        }
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    input.trim().to_string()
}

fn parse_action(input: &str) -> Option<WalletAction> {
    match input {
        "1" => Some(WalletAction::CheckBalance),
        "2" => {
            println!("Enter amount to add:");
            let amount = get_user_input().parse::<f64>().unwrap_or(0.0);
            Some(WalletAction::AddFunds(amount))
        }
        "3" => {
            println!("Enter amount to withdraw:");
            let amount = get_user_input().parse::<f64>().unwrap_or(0.0);
            Some(WalletAction::WithdrawFunds(amount))
        }
        "4" => Some(WalletAction::ViewHistory),
        "5" => Some(WalletAction::Exit),
        _ => None,
    }
}

fn main() {
    let mut wallet = Wallet::new();

    loop {
        println!("\n===== Digital Wallet Menu =====");
        println!("1. Check Balance");
        println!("2. Add Funds");
        println!("3. Withdraw Funds");
        println!("4. View History");
        println!("5. Exit");
        println!("Select an option (1-5):");

        let input = get_user_input();

        match parse_action(&input) {
            Some(WalletAction::CheckBalance) => wallet.check_balance(),
            Some(WalletAction::AddFunds(amount)) => wallet.add_funds(amount),
            Some(WalletAction::WithdrawFunds(amount)) => wallet.withdraw_funds(amount),
            Some(WalletAction::ViewHistory) => wallet.view_history(),
            Some(WalletAction::Exit) => {
                println!("Exiting Wallet. Goodbye!");
                break;
            }
            None => println!("Invalid input. Please try again."),
        }
    }
}

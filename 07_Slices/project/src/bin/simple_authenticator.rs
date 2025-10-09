use std::io;

fn main() {
    let correct_username = "admin";
    let correct_password = "secure123";

    let mut attempts = 0;

    while attempts < 3 {
        println!("Enter username:");
        let mut username = String::new();
        io::stdin().read_line(&mut username).expect("Failed to read input");
        let username = username.trim();

        println!("Enter password:");
        let mut password = String::new();
        io::stdin().read_line(&mut password).expect("Failed to read input");
        let password = password.trim();

        if username == correct_username {
            if password == correct_password {
                println!("Login successful. Welcome, {}!", username);
                break;
            } else if &password[..5.min(password.len())] == "secure" {
                println!("Partial match: Password starts correctly but is not fully correct.");
            } else {
                println!("Incorrect password.");
            }
        } else {
            println!("Invalid username.");
        }

        attempts += 1;

        if attempts < 3 {
            println!("Try again. Attempts remaining: {}", 3 - attempts);
        } else {
            println!("Too many failed attempts. Exiting.");
        }
    }
}

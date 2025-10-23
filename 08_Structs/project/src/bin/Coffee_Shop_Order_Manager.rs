use std::io;

#[derive(Debug)]
enum Size {
    Small,
    Medium,
    Large,
}

impl Size {
    fn from_str(input: &str) -> Option<Size> {
        match input.trim().to_lowercase().as_str() {
            "small" => Some(Size::Small),
            "medium" => Some(Size::Medium),
            "large" => Some(Size::Large),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct CoffeeOrder {
    coffee_type: String,
    size: Size,
    extras: Vec<String>,
    is_hot: bool,
}

impl CoffeeOrder {
    fn new(coffee_type: String, size: Size, extras: Vec<String>, is_hot: bool) -> CoffeeOrder {
        CoffeeOrder {
            coffee_type,
            size,
            extras,
            is_hot,
        }
    }

    fn display(&self) {
        println!("Order Details:");
        println!("  Coffee Type: {}", self.coffee_type);
        println!(
            "  Size: {}",
            match self.size {
                Size::Small => "Small",
                Size::Medium => "Medium",
                Size::Large => "Large",
            }
        );
        if self.extras.is_empty() {
            println!("  Extras: None");
        } else {
            println!("  Extras: {}", self.extras.join(", "));
        }
        println!("  Temperature: {}", if self.is_hot { "Hot" } else { "Cold" });
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn ask_yes_no(question: &str) -> bool {
    loop {
        println!("{} (yes/no):", question);
        let input = read_line();
        match input.to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Please answer 'yes' or 'no'."),
        }
    }
}

fn main() {
    let mut orders: Vec<CoffeeOrder> = Vec::new();

    loop {
        println!("\nCoffee Shop Order Manager");
        println!("1. Create new order");
        println!("2. View all orders");
        println!("3. Exit");
        println!("Enter your choice:");

        let choice = read_line();

        match choice.as_str() {
            "1" => {
                // Create new order
                println!("Enter coffee type (e.g., Latte, Espresso, Americano):");
                let coffee_type = read_line();

                // Get size
                let size = loop {
                    println!("Enter size (small, medium, large):");
                    let input = read_line();
                    if let Some(size) = Size::from_str(&input) {
                        break size;
                    } else {
                        println!("Invalid size. Try again.");
                    }
                };

                // Extras
                let mut extras = Vec::new();
                println!("Add extras? (milk, sugar, syrup, etc.)");
                while ask_yes_no("Add an extra?") {
                    println!("Enter extra:");
                    let extra = read_line();
                    if !extra.is_empty() {
                        extras.push(extra);
                    }
                }

                // Temperature
                let is_hot = ask_yes_no("Do you want it hot?");

                let order = CoffeeOrder::new(coffee_type, size, extras, is_hot);
                println!("Order created!");
                order.display();

                orders.push(order);
            }
            "2" => {
                // View orders
                if orders.is_empty() {
                    println!("No orders yet.");
                } else {
                    for (i, order) in orders.iter().enumerate() {
                        println!("\nOrder #{}", i + 1);
                        order.display();
                    }
                }
            }
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice, please select 1, 2, or 3.");
            }
        }
    }
}

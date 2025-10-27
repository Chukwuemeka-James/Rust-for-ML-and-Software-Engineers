use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)]
enum ProductCategory {
    Electronics,
    Groceries,
    Clothing,
    Other,
}

impl ProductCategory {
    fn from_input(input: &str) -> Self {
        match input.to_lowercase().as_str() {
            "electronics" => ProductCategory::Electronics,
            "groceries" => ProductCategory::Groceries,
            "clothing" => ProductCategory::Clothing,
            _ => ProductCategory::Other,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            ProductCategory::Electronics => "Electronics",
            ProductCategory::Groceries => "Groceries",
            ProductCategory::Clothing => "Clothing",
            ProductCategory::Other => "Other",
        }
    }
}

#[derive(Debug, Clone)]
struct Product {
    id: usize,
    name: String,
    quantity: u32,
    price: f64,
    category: ProductCategory,
}

impl Product {
    fn new(id: usize, name: String, quantity: u32, price: f64, category: ProductCategory) -> Self {
        Product {
            id,
            name,
            quantity,
            price,
            category,
        }
    }

    fn update_stock(&mut self, qty: i32) {
        let new_qty = (self.quantity as i32 + qty).max(0) as u32;
        self.quantity = new_qty;
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn print_menu() {
    println!("\n=== Inventory Management ===");
    println!("1. Add Product");
    println!("2. View Inventory");
    println!("3. Update Stock");
    println!("4. Delete Product");
    println!("5. Exit");
    print!("Choose an option: ");
    io::stdout().flush().unwrap();
}

fn main() {
    let mut inventory: HashMap<usize, Product> = HashMap::new();
    let mut next_id = 1;

    loop {
        print_menu();
        let choice = read_input("");

        match choice.as_str() {
            "1" => {
                let name = read_input("Enter product name: ");
                let qty_input = read_input("Enter quantity: ");
                let price_input = read_input("Enter price: ");
                let category_input = read_input("Enter category (Electronics/Groceries/Clothing/Other): ");

                let quantity = qty_input.parse::<u32>().unwrap_or(0);
                let price = price_input.parse::<f64>().unwrap_or(0.0);
                let category = ProductCategory::from_input(&category_input);

                let product = Product::new(next_id, name, quantity, price, category);
                inventory.insert(next_id, product);
                println!("✅ Product added with ID: {}", next_id);
                next_id += 1;
            }

            "2" => {
                println!("\n📦 Current Inventory:");
                if inventory.is_empty() {
                    println!("(No products found)");
                } else {
                    for product in inventory.values() {
                        println!(
                            "[{}] {} | Qty: {} | ${:.2} | Category: {}",
                            product.id,
                            product.name,
                            product.quantity,
                            product.price,
                            product.category.as_str()
                        );
                    }
                }
            }

            "3" => {
                let id_input = read_input("Enter product ID to update: ");
                let qty_input = read_input("Enter quantity to add/remove (use negative for removal): ");

                if let (Ok(id), Ok(qty_change)) = (id_input.parse::<usize>(), qty_input.parse::<i32>()) {
                    if let Some(product) = inventory.get_mut(&id) {
                        product.update_stock(qty_change);
                        println!("✅ Updated stock. New quantity: {}", product.quantity);
                    } else {
                        println!("⚠️ Product ID not found.");
                    }
                } else {
                    println!("❌ Invalid input.");
                }
            }

            "4" => {
                let id_input = read_input("Enter product ID to delete: ");
                if let Ok(id) = id_input.parse::<usize>() {
                    if inventory.remove(&id).is_some() {
                        println!("Product {} deleted.", id);
                    } else {
                        println!("⚠️ Product ID not found.");
                    }
                } else {
                    println!("❌ Invalid ID.");
                }
            }

            "5" => {
                println!("👋 Exiting Inventory System. Goodbye!");
                break;
            }

            _ => {
                println!("❌ Invalid option. Try again.");
            }
        }
    }
}

use std::io;

#[derive(Debug, PartialEq)]
enum Availability {
    Available,
    Borrowed,
}

impl Availability {
    fn to_string(&self) -> &str {
        match self {
            Availability::Available => "Available",
            Availability::Borrowed => "Borrowed",
        }
    }
}

#[derive(Debug)]
struct Book {
    id: usize,
    title: String,
    author: String,
    status: Availability,
}

impl Book {
    fn new(id: usize, title: String, author: String) -> Self {
        Book {
            id,
            title,
            author,
            status: Availability::Available,
        }
    }

    fn display(&self) {
        println!(
            "[{}] Book #{}: '{}' by {}",
            self.status.to_string(),
            self.id,
            self.title,
            self.author
        );
    }
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let mut books: Vec<Book> = Vec::new();
    let mut next_id = 1;

    loop {
        println!("\nLibrary Book Management System");
        println!("1. Add a new book");
        println!("2. List all books");
        println!("3. Borrow a book");
        println!("4. Return a book");
        println!("5. Exit");

        let choice = read_input("Enter your choice:");

        match choice.as_str() {
            "1" => {
                let title = read_input("Enter book title:");
                let author = read_input("Enter book author:");
                if title.is_empty() || author.is_empty() {
                    println!("Title and author cannot be empty.");
                    continue;
                }
                let book = Book::new(next_id, title, author);
                books.push(book);
                println!("Book added with ID {}", next_id);
                next_id += 1;
            }
            "2" => {
                if books.is_empty() {
                    println!("No books in the library.");
                } else {
                    println!("List of books:");
                    for book in &books {
                        book.display();
                    }
                }
            }
            "3" => {
                let id_str = read_input("Enter book ID to borrow:");
                let id: usize = match id_str.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID.");
                        continue;
                    }
                };
                let mut found = false;
                for book in &mut books {
                    if book.id == id {
                        found = true;
                        if book.status == Availability::Available {
                            book.status = Availability::Borrowed;
                            println!("You have borrowed '{}'.", book.title);
                        } else {
                            println!("Book '{}' is already borrowed.", book.title);
                        }
                        break;
                    }
                }
                if !found {
                    println!("Book with ID {} not found.", id);
                }
            }
            "4" => {
                let id_str = read_input("Enter book ID to return:");
                let id: usize = match id_str.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID.");
                        continue;
                    }
                };
                let mut found = false;
                for book in &mut books {
                    if book.id == id {
                        found = true;
                        if book.status == Availability::Borrowed {
                            book.status = Availability::Available;
                            println!("You have returned '{}'.", book.title);
                        } else {
                            println!("Book '{}' is already available.", book.title);
                        }
                        break;
                    }
                }
                if !found {
                    println!("Book with ID {} not found.", id);
                }
            }
            "5" => {
                println!("Exiting the system. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option. Please enter a number between 1 and 5.");
            }
        }
    }
}

// Mini Library Inventory System 

use std::io::{self, Write};

#[derive(Debug)]
enum BookStatus {
    Available,
    Borrowed,
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    year: u16,
    status: BookStatus,
}

impl Book {
    fn new(title: String, author: String, year: u16) -> Self {
        Self {
            title,
            author,
            year,
            status: BookStatus::Available,
        }
    }
}

enum LibraryAction {
    AddBook,
    ListBooks,
    BorrowBook(String),
    ReturnBook(String),
    Exit,
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Self {
        Self { books: Vec::new() }
    }

    fn add_book(&mut self, title: String, author: String, year: u16) {
        let book = Book::new(title, author, year);
        self.books.push(book);
        println!("Book added successfully.");
    }

    fn list_books(&self) {
        if self.books.is_empty() {
            println!("No books in the library.");
        } else {
            println!("Books in the library:");
            for (index, book) in self.books.iter().enumerate() {
                let status = match book.status {
                    BookStatus::Available => "Available",
                    BookStatus::Borrowed => "Borrowed",
                };
                println!(
                    "{}. '{}' by {}, {} [{}]",
                    index + 1,
                    book.title,
                    book.author,
                    book.year,
                    status
                );
            }
        }
    }

    fn borrow_book(&mut self, title: &str) {
        match self.books.iter_mut().find(|b| b.title == title) {
            Some(book) => match book.status {
                BookStatus::Available => {
                    book.status = BookStatus::Borrowed;
                    println!("You have borrowed '{}'.", book.title);
                }
                BookStatus::Borrowed => {
                    println!("Sorry, '{}' is already borrowed.", book.title);
                }
            },
            None => {
                println!("Book '{}' not found.", title);
            }
        }
    }

    fn return_book(&mut self, title: &str) {
        match self.books.iter_mut().find(|b| b.title == title) {
            Some(book) => match book.status {
                BookStatus::Borrowed => {
                    book.status = BookStatus::Available;
                    println!("You have returned '{}'.", book.title);
                }
                BookStatus::Available => {
                    println!("'{}' was not borrowed.", book.title);
                }
            },
            None => {
                println!("Book '{}' not found.", title);
            }
        }
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn parse_action(input: &str) -> Option<LibraryAction> {
    match input.to_lowercase().as_str() {
        "add" => Some(LibraryAction::AddBook),
        "list" => Some(LibraryAction::ListBooks),
        "borrow" => {
            let title = read_input("Enter the title of the book to borrow: ");
            Some(LibraryAction::BorrowBook(title))
        }
        "return" => {
            let title = read_input("Enter the title of the book to return: ");
            Some(LibraryAction::ReturnBook(title))
        }
        "exit" => Some(LibraryAction::Exit),
        _ => None,
    }
}

fn main() {
    let mut library = Library::new();

    println!("Welcome to the Mini Library Inventory System!");

    loop {
        println!("\nChoose an action: add, list, borrow, return, exit");
        let input = read_input("Your choice: ");

        if let Some(action) = parse_action(&input) {
            match action {
                LibraryAction::AddBook => {
                    let title = read_input("Enter book title: ");
                    let author = read_input("Enter author name: ");
                    let year_input = read_input("Enter year published: ");
                    let year = match year_input.parse::<u16>() {
                        Ok(y) => y,
                        Err(_) => {
                            println!("Invalid year entered. Please enter a valid number.");
                            continue;
                        }
                    };
                    library.add_book(title, author, year);
                }
                LibraryAction::ListBooks => {
                    library.list_books();
                }
                LibraryAction::BorrowBook(title) => {
                    library.borrow_book(&title);
                }
                LibraryAction::ReturnBook(title) => {
                    library.return_book(&title);
                }
                LibraryAction::Exit => {
                    println!("Exiting the system. Goodbye!");
                    break;
                }
            }
        } else {
            println!("Invalid action. Please try again.");
        }
    }
}

use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, Write};

fn main() {
    // Define a set of unauthorized words
    let unauthorized_words: HashSet<&str> = [
        "classified", "secret", "unauthorized", "restricted"
    ]
    .iter()
    .cloned()
    .collect();

    println!("Welcome to the Rust File Reader CLI Tool!");
    
    loop {
        // Ask user for input
        println!("\nMenu:");
        println!("1. Enter filename to analyze");
        println!("2. Exit");

        print!("Enter your choice (1 or 2): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                print!("Enter file name: ");
                io::stdout().flush().unwrap();

                let mut filename = String::new();
                io::stdin().read_line(&mut filename).unwrap();
                let filename = filename.trim();

                match fs::read_to_string(filename) {
                    Ok(contents) => {
                        if let Some(found_words) =
                            contains_unauthorized_words(&contents, &unauthorized_words)
                        {
                            println!(
                                "File contains unauthorized words: {:?}. Skipping analysis.",
                                found_words
                            );
                        } else {
                            analyze_text(&contents);
                        }
                    }
                    Err(e) => {
                        println!("Error reading file: {}", e);
                    }
                }
            }
            "2" => {
                println!("Exiting. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter 1 or 2.");
            }
        }
    }
}

// Analyze and display line, word, character count, and most frequent word
fn analyze_text(text: &str) {
    let lines = text.lines().count();
    let words: Vec<&str> = text
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
        .filter(|w| !w.is_empty())
        .collect();
    let characters = text.chars().count();

    let mut freq_map: HashMap<String, usize> = HashMap::new();
    for word in &words {
        *freq_map.entry(word.to_string()).or_insert(0) += 1;
    }

    let most_frequent_word = freq_map
        .iter()
        .max_by_key(|entry| entry.1)
        .map(|(word, count)| (word, count))
        .unwrap_or((&String::from("N/A"), &0));

    println!("\nFile Analysis Results:");
    println!("Lines: {}", lines);
    println!("Words: {}", words.len());
    println!("Characters: {}", characters);
    println!(
        "Most Frequent Word: '{}' ({} times)",
        most_frequent_word.0, most_frequent_word.1
    );
}

// Check for any unauthorized words and return the list if found
fn contains_unauthorized_words<'a>(
    text: &str,
    blacklist: &'a HashSet<&str>,
) -> Option<Vec<&'a str>> {
    let mut found = vec![];
    let lower_text = text.to_lowercase();

    for &word in blacklist {
        if lower_text.contains(word) {
            found.push(word);
        }
    }

    if found.is_empty() {
        None
    } else {
        Some(found)
    }
}

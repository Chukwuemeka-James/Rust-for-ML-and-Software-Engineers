use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run --bin text_analyzer <file_path>");
        return;
    }

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Failed to read the file. Make sure the path is correct.");

    analyze_text(&contents);
}

fn analyze_text(text: &str) {
    // Remove newlines and extra whitespace
    let cleaned_text = text.replace("\n", " ");
    let words: Vec<&str> = cleaned_text
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|w| !w.is_empty())
        .collect();

    let word_count = words.len();
    let character_count = cleaned_text.chars().filter(|c| !c.is_whitespace()).count();

    let first_5_chars = &cleaned_text.chars().take(5).collect::<String>();

    println!("Text Analysis Report:");
    println!("----------------------");
    println!("Total number of words: {}", word_count);
    println!("Total number of characters (excluding whitespace): {}", character_count);
    println!("First 5 characters: {}", first_5_chars);

    let mut frequency_map: HashMap<String, usize> = HashMap::new();

    for word in &words {
        let lowercase_word = word.to_lowercase();
        *frequency_map.entry(lowercase_word).or_insert(0) += 1;
    }

    println!("\nWord Frequencies:");
    for (word, count) in &frequency_map {
        println!("{:<15} : {}", word, count);
    }

    // Identify most frequent word
    if let Some((most_frequent_word, max_count)) = frequency_map
        .iter()
        .max_by_key(|entry| entry.1)
    {
        println!("\nMost Frequent Word: '{}' ({} times)", most_frequent_word, max_count);
    }
}

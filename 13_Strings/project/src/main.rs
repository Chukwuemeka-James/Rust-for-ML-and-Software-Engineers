use std::io;

#[derive(Debug)]
struct SearchResult {
    content: String,
    score: usize,
}

fn main() {
    let documents = vec![
        String::from("Rust is fast and safe"),
        String::from("Python is great for data science"),
        String::from("Rust ownership model prevents bugs"),
        String::from("Learning programming takes time"),
        String::from("Data analysis with Rust is powerful"),
    ];

    println!("Enter your search query (you can use multiple words):");
    let query = get_input();

    let results = search_documents(&documents, &query);

    if results.is_empty() {
        println!("\nNo matching results found.");
    } else {
        println!("\n🔍 Search Results:");
        for result in results {
            println!("[{} match(es)] {}", result.score, result.content);
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_lowercase()
}

fn search_documents(docs: &Vec<String>, query: &str) -> Vec<SearchResult> {
    let keywords: Vec<&str> = query.split_whitespace().collect();
    let mut results: Vec<SearchResult> = Vec::new();

    for doc in docs {
        let lower_doc = doc.to_lowercase();
        let mut score = 0;
        let mut highlighted = doc.clone();

        for &keyword in &keywords {
            if lower_doc.contains(keyword) {
                score += lower_doc.matches(keyword).count();

                // Highlight keyword (case-insensitive)
                let word_parts: Vec<&str> = doc.split_whitespace().collect();
                let mut highlighted_words: Vec<String> = Vec::new();

                for word in word_parts {
                    if word.to_lowercase().contains(keyword) {
                        highlighted_words.push(format!("[{}]", word));
                    } else {
                        highlighted_words.push(word.to_string());
                    }
                }

                highlighted = highlighted_words.join(" ");
            }
        }

        if score > 0 {
            results.push(SearchResult {
                content: highlighted,
                score,
            });
        }
    }

    // Sort by score in descending order
    results.sort_by(|a, b| b.score.cmp(&a.score));
    results
}

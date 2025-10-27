use std::io;
use std::io::Write; // for flush()
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum TaskStatus {
    Pending,
    Completed,
}

#[derive(Debug, Clone)]
struct Task {
    id: usize,
    description: String,
    status: TaskStatus,
}

impl Task {
    fn new(id: usize, description: String) -> Self {
        Task {
            id,
            description,
            status: TaskStatus::Pending,
        }
    }

    fn mark_complete(&mut self) {
        self.status = TaskStatus::Completed;
    }
}

fn print_menu() {
    println!("\n=== Task Management CLI ===");
    println!("1. Add a task");
    println!("2. List all tasks");
    println!("3. Mark task as complete");
    println!("4. Delete a task");
    println!("5. Exit");
    print!("Choose an option: ");
    io::stdout().flush().unwrap();
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    let mut tasks: HashMap<usize, Task> = HashMap::new();
    let mut next_id = 1;

    loop {
        print_menu();

        let choice = read_input("");

        match choice.as_str() {
            "1" => {
                let desc = read_input("Enter task description: ");
                let task = Task::new(next_id, desc);
                tasks.insert(next_id, task);
                println!("✅ Task added with ID: {}", next_id);
                next_id += 1;
            }

            "2" => {
                println!("\nCurrent Tasks:");
                if tasks.is_empty() {
                    println!("(No tasks found)");
                } else {
                    for task in tasks.values() {
                        let status = match task.status {
                            TaskStatus::Pending => "Pending",
                            TaskStatus::Completed => "✅ Completed",
                        };
                        println!("[{}] {} - {}", task.id, task.description, status);
                    }
                }
            }

            "3" => {
                let id_str = read_input("Enter task ID to mark complete: ");
                if let Ok(id) = id_str.parse::<usize>() {
                    if let Some(task) = tasks.get_mut(&id) {
                        task.mark_complete();
                        println!("✅ Task {} marked as complete.", id);
                    } else {
                        println!("⚠️ Task ID not found.");
                    }
                } else {
                    println!("❌ Invalid ID.");
                }
            }

            "4" => {
                let id_str = read_input("Enter task ID to delete: ");
                if let Ok(id) = id_str.parse::<usize>() {
                    if tasks.remove(&id).is_some() {
                        println!("Task {} deleted.", id);
                    } else {
                        println!("⚠️ Task ID not found.");
                    }
                } else {
                    println!("❌ Invalid ID.");
                }
            }

            "5" => {
                println!("Exiting Task Manager. Goodbye!");
                break;
            }

            _ => {
                println!("❌ Invalid option. Please try again.");
            }
        }
    }
}

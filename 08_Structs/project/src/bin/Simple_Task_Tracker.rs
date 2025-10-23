use std::io;

#[derive(Debug)]
enum Status {
    Pending,
    Completed,
}

impl Status {
    fn from_str(input: &str) -> Option<Status> {
        match input.trim().to_lowercase().as_str() {
            "pending" => Some(Status::Pending),
            "completed" => Some(Status::Completed),
            _ => None,
        }
    }

    fn to_string(&self) -> &str {
        match self {
            Status::Pending => "Pending",
            Status::Completed => "Completed",
        }
    }
}

#[derive(Debug)]
struct Task {
    id: usize,
    description: String,
    status: Status,
}

impl Task {
    fn new(id: usize, description: String) -> Task {
        Task {
            id,
            description,
            status: Status::Pending,
        }
    }

    fn display(&self) {
        println!(
            "[{}] Task #{}: {}",
            self.status.to_string(),
            self.id,
            self.description
        );
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut next_id = 1;

    loop {
        println!("\nSimple Task Tracker");
        println!("1. Add a new task");
        println!("2. List all tasks");
        println!("3. Mark task as completed");
        println!("4. Delete a task");
        println!("5. Exit");
        println!("Enter your choice:");

        let choice = read_line();

        match choice.as_str() {
            "1" => {
                println!("Enter task description:");
                let description = read_line();
                if description.is_empty() {
                    println!("Task description cannot be empty.");
                    continue;
                }
                let task = Task::new(next_id, description);
                tasks.push(task);
                println!("Task added with ID {}", next_id);
                next_id += 1;
            }
            "2" => {
                if tasks.is_empty() {
                    println!("No tasks added yet.");
                } else {
                    println!("Listing all tasks:");
                    for task in &tasks {
                        task.display();
                    }
                }
            }
            "3" => {
                println!("Enter the ID of the task to mark as completed:");
                let id_str = read_line();
                let id: usize = match id_str.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID.");
                        continue;
                    }
                };

                let mut found = false;
                for task in &mut tasks {
                    if task.id == id {
                        task.status = Status::Completed;
                        println!("Task #{} marked as completed.", id);
                        found = true;
                        break;
                    }
                }
                if !found {
                    println!("Task with ID {} not found.", id);
                }
            }
            "4" => {
                println!("Enter the ID of the task to delete:");
                let id_str = read_line();
                let id: usize = match id_str.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID.");
                        continue;
                    }
                };

                let original_len = tasks.len();
                tasks.retain(|task| task.id != id);
                if tasks.len() < original_len {
                    println!("Task #{} deleted.", id);
                } else {
                    println!("Task with ID {} not found.", id);
                }
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice, please enter 1, 2, 3, 4, or 5.");
            }
        }
    }
}

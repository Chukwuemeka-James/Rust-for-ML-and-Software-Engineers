// In this example, you'll learn how to use tuples in Rust to store values of different types.
// You'll also see two ways to access tuple elements: by index and by destructuring into variables.

fn main() {
    // A tuple representing an employee: name, age, department
    let employee = ("Molly", 32, "Marketing");

    // Accessing tuple elements using dot notation (by index)
    let name_by_index = employee.0;
    let age_by_index = employee.1;
    let dept_by_index = employee.2;

    println!(
        "Using index access Name: {}, Age: {}, Department: {}", name_by_index, age_by_index, dept_by_index
    );

    // Destructuring the tuple into individual variables
    let (name, age, department) = employee;

    println!("Using destructuring → Name: {}, Age: {}, Department: {}", name, age, department);
}

// In this example, you'll learn how to use escape characters and raw string literals in Rust.
// You'll see how to format strings with newline `\n`, tab `\t`, double quotes `\"`, and raw strings for things like Windows file paths.

fn main() {
    // \n creates a new line in the output
    println!("Dear Amarachi,\nHow have you been?");

    // \t adds a horizontal tab (indentation)
    println!("\tOnce upon a time");

    // \" is used to include quotation marks inside a string
    println!("Amarachi said \"I love you music\"");

    // Raw strings (r"...") treat backslashes and quotes literally — great for Windows file paths
    let filepath = r"C:\My Documents\new\videos";

    // Use {} to print the value of `filepath`
    println!("File path: {}", filepath);
}
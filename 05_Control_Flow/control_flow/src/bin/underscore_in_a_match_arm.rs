fn main() {
    let number = 7;

    match number {
        1 => println!("You entered one."),
        2 => println!("You entered two."),
        3 => println!("You entered three."),
        _ => println!("You entered something else."), // _ matches ANY other value
    }
}

fn even_odd_name_checker(number: i32, name: &str) {
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number is {result}");

    // `to_lowercase()` returns a String, so we compare it with the original name
    let lowercase_assesment = if name == name.to_lowercase() { "accepted" } else { "rejected" };
    println!("The name {name} is {lowercase_assesment}");
}

fn main() {
    even_odd_name_checker(17, "Peter");
    even_odd_name_checker(100, "james");
}

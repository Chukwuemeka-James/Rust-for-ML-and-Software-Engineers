fn main() {
    let assesment: bool = true;

    match assesment{
        true => {
            println!("You passed");
        }
        false => {
            println!("You failed");
        }
    }

    let value : i32 = match assesment {
        true => 50,
        false => 39
    };
    println!("Your status is a reflection of your score {value}")
}
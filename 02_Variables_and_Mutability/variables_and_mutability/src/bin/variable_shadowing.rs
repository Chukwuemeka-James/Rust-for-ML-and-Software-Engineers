fn main(){
    let name : &str = "James";
    let mut year : i32 = 2010;
    let mut age : i32 = 10;
    println!("My name is {name}, it's the year {year} and I am {age} years old");

    year = 2020; 
    age = 20;
    println!("My name is {name}, it's the year {year} and I am {age} years old");

    year = 2025; 
    age = 25;
    println!("My name is {name}, it's the year {year} and I am {age} years old");
}
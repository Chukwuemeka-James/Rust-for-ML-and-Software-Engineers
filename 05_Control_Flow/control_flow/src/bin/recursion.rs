fn countdown(seconds: i32){
    if seconds == 1{
        println!("The end!");
    } else {
        println!("We still have {seconds} seconds to go!");
        countdown(seconds - 1);
    }
}

fn main(){
    countdown(5);
}
fn main (){
    let value = 8;

    match value {

        output if output % 2 == 0 => println!("{output} is even"),
        output if output % 2 != 0 => println!("{output} is odd"),
        _ => unreachable!(),
        
    }
}
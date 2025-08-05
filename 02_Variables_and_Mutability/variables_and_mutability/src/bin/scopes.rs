fn main() {
    // Outer scope
    let apples = 10;
    println!("Outer scope: I have {apples} apples.");

    {
        // Inner scope starts
        let oranges = 5;
        println!("Inner scope: I have {apples} apples and {oranges} oranges.");

        // Shadowing apples in the inner scope
        let apples = 20;
        println!("Inner scope after shadowing: I now have {apples} apples.");
    } // Inner scope ends, oranges and shadowed apples are dropped here

    // Back to outer scope: oranges is no longer accessible, and apples is the original one
    println!("Back to outer scope: I still have {apples} apples.");
}

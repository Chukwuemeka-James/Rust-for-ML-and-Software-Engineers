fn main() {
    let apples = 50;

    let oranges = 14 + 6;

    // Prefixing with "_" tells the compiler we intentionally won't use this variable
    let _fruits = apples + oranges;

    println!(
        "This year, my garden has {0} apples and {1} oranges. I can't believe I have {0} apples",
        apples, oranges
    );
}

fn main() {
    let oranges = String::from("Oranges");

    // Passing `oranges` to the function moves ownership.
    print_my_value(oranges); // let value = oranges;

    // ❌ Uncommenting below will cause a compile-time error
    // because `oranges` is no longer valid here.
    // println!("{oranges} is now invalid");
}

fn print_my_value(value: String) {
    // Ownership of `value` now belongs to this function.
    println!("Your value is {value}");
}

/*
Learning points

1. **Ownership transfer into a function** – When a value that doesn’t implement `Copy` (like `String`) is passed as a parameter, ownership moves to the function.
2. **Original variable becomes invalid** – After the move, the original variable (`oranges`) can no longer be used in the calling scope.
3. **No implicit copying** – Rust avoids hidden allocations or copies; the data is simply moved, making ownership explicit.
4. **Data safety** – This mechanism ensures that only one owner exists for the data at a time, preventing dangling references and unsafe memory use.
*/
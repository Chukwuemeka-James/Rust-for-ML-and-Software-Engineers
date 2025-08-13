fn main() {
    // Create a String on the heap and bind it to `burger`.
    let burger = String::from("Burger");

    // Pass `burger` into the function.
    // Ownership moves into `add_fries` because `meal` takes `String` by value.
    add_fries(burger);
}

fn add_fries(mut meal: String) {
    // `meal` is now the owner of the String data previously owned by `burger`.
    meal.push_str(" and Fries");
    println!("{meal}");
}

/*

**Learning points**

1. **Ownership transfer (move)** – Passing a `String` (non-`Copy` type) by value moves ownership to the function parameter.
2. **Original variable invalidation** – After `burger` is passed into `add_fries`, it can no longer be used in `main` because its ownership has been moved.
3. **Parameter mutability** – Even if the original variable was not mutable, the function can declare its parameter as `mut` to modify it after taking ownership.
4. **Heap data modification** – Using `.push_str()` on a mutable `String` changes its contents.
*/
fn main() {
    // Create a mutable String.
    let mut car = String::from("Red");

    // Attempt to create two mutable references at the same time.
    let ref1 = &mut car;
    let ref2 = &mut car; // ❌ This will cause a compile-time error.

    // println!("{ref2}");
}

/*
**Learning points

1. **Exclusive mutable borrow rule** – Rust allows **only one mutable reference** to a piece of data in a given scope at a time.
2. **Compile-time safety** – The second mutable borrow (`ref2`) causes a compile-time error to prevent **data races** and **undefined behavior**.
3. **Ownership vs Borrowing** – Even though we still own `car`, borrowing it mutably more than once in the same scope is forbidden.
4. **Safe concurrency** – This rule is a cornerstone of Rust’s memory safety guarantees without a garbage collector.
*/
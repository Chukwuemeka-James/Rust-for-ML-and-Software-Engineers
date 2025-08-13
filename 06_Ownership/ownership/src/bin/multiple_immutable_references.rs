fn main() {
    // Create a String on the heap and bind it to `car`.
    let car = String::from("Red");

    // Create two immutable references to `car`.
    let ref1 = &car;
    let ref2 = &car;

    // We can use both immutable references and the original variable because
    // immutable references allow shared read access without ownership transfer.
    println!("{ref1} and {ref2} and {}", &car);
}

/*
Learning points

1. **Immutable references (`&T`)** – You can create multiple immutable references to the same variable at the same time.
2. **No data mutation** – Immutable references do not allow modifying the referenced data.
3. **No conflict** – Multiple immutable borrows are allowed because they don’t create data races; they only read from the same data.
4. **Original owner usability** – While immutable references exist, you can still use the original variable for reading.
*/
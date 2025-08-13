fn main() {
    // 'ice_cream' is a string literal: stored in the program's binary (static memory), not on the heap
    let ice_cream = "Cookies and Cream";

    // Copy the value into 'dessert'
    // For string literals (&str), the "copy" is just copying the pointer + length (both live in static memory)
    let dessert = ice_cream;

    // Both 'ice_cream' and 'dessert' are valid and point to the same data in memory
    println!("{ice_cream} {dessert}.");
}

/*
Learning points

1. **String literals (`&str`)** – These are immutable slices of data stored in the program's binary.
2. **No move here** – Copying a `&str` is **cheap** because only a small fixed-size pointer and length are copied (stack data).
3. **Both variables valid** – Ownership rules aren’t violated since `&str` implements the `Copy` trait, meaning it’s duplicated automatically.
4. **Immutable by default** – You can’t change `"Cookies and Cream"` through either variable; it’s read-only.
5. **Contrast with `String`** – If we used `String::from("Cookies and Cream")` instead, this would trigger a **move** instead of a copy.
*/
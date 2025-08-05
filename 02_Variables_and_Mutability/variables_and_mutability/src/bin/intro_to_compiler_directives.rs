// This directive tells the Rust compiler not to warn about unused variables in this file.
#![allow(unused_variables)]

// Create a type alias for readability
type Meters = i32;

fn main() {
    // These variables are intentionally unused for demonstration
    let mile_race_length: Meters = 1600;
    let two_mile_race_length: Meters = 3200;

    // Normally, Rust would warn about unused variables here.
    // The compiler directive above prevents that warning.
}


/*
This example demonstrates how the #![allow(unused_variables)] compiler directive works in Rust. 
It prevents the compiler from generating warnings about variables that are declared but not used, 
which can be useful during learning, experimentation, or while incrementally building a program.
*/



/*
In Rust, a **compiler directive** (also called an *attribute*) is a special instruction that you give to the Rust compiler 
to change how it analyzes, compiles, or treats your code.

These directives are written using the `#![...]` or `#[...]` syntax:

* `#![...]` applies to the **entire file or crate**.
* `#[...]` applies to a **specific item** (function, variable, module, etc.).
*/
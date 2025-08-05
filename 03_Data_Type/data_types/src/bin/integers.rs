// In this example, you will learn how to use different integer types in Rust.
// You'll see the difference between signed and unsigned integers, and how to specify their bit sizes.
// We'll also print the values so you can observe how each type works.

fn main() {
    // i16: 16-bit signed integer, can hold both negative and positive values
    // Range: -32,768 to 32,767
    let sixteen_bit_signed: i16 = -32500;

    // u16: 16-bit unsigned integer, can only hold positive values
    // Range: 0 to 65,535
    let sixteen_bit_unsigned: u16 = 64000;

    // i32: 32-bit signed integer
    // Range: -2,147,483,648 to 2,147,483,647
    let thirty_two_bit_signed: i32 = -2147483648;

    // u32: 32-bit unsigned integer
    // Range: 0 to 4,294,967,295
    let thirty_two_bit_unsigned: u32 = 4294967295;

    // You can also specify the type directly in the value using a suffix
    let some_value = 20u16;

    // Print values to the console
    println!("16-bit signed: {sixteen_bit_signed}");
    println!("16-bit unsigned: {sixteen_bit_unsigned}");
    println!("32-bit signed: {thirty_two_bit_signed}");
    println!("32-bit unsigned: {thirty_two_bit_unsigned}");
    println!("Some value (u16): {some_value}");
}

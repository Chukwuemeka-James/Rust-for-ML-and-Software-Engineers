fn main() {
    // A stack-allocated integer
    let my_stack_value = 2;

    // Create an immutable reference to the stack value
    let my_integer_reference = &my_stack_value;
    println!("{}", my_integer_reference);

    // A heap-allocated String
    let my_heap_value = String::from("Toyota");

    // Create an immutable reference to the heap value
    let my_heap_reference = &my_heap_value;
    println!("{}", my_heap_reference)
}

/*
Key points

1. **Stack value** (`my_stack_value`) – small, fixed-size, stored directly on the stack.
2. **Heap value** (`my_heap_value`) – dynamically allocated, pointer stored on stack but data lives on heap.
3. Both types can be **borrowed immutably** using `&value`.
4. Dereferencing happens automatically in `println!` when using `{}`.
*/
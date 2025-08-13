fn main() {
    // A stack-allocated integer
    let my_stack_value = 2;
    // Immutable reference to a stack value
    let my_integer_reference = &my_stack_value;

    // A heap-allocated String
    let my_heap_value = String::from("Toyota");
    // Immutable reference to a heap value
    let my_heap_reference = &my_heap_value;
}

/*
Learning points

1. **Stack value reference** – `&my_stack_value` creates an immutable reference to a simple stack-allocated integer.
2. **Heap value reference** – `String` stores its data on the heap, but the variable `my_heap_value` (a pointer, length, and capacity) lives on the stack.
3. **Reference safety** – Rust ensures that both stack and heap references remain valid as long as the original variables are in scope.
4. **Drop order** – When the function ends, `my_heap_reference` and `my_integer_reference` are dropped first, but they don't free the actual data — the original variables (`my_stack_value`, `my_heap_value`) do that when they are dropped.
5. **Ownership vs Borrowing** – Both examples here are *borrowing* without taking ownership, so the original values remain usable after the reference is created.
*/
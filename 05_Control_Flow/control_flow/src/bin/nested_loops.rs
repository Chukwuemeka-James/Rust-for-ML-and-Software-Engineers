// Demonstrates nested loops and the use of labels to break specific loops.

fn main() {
    let mut outer_count = 0;

    'outer: loop {
        let mut inner_count = 0;

        loop {
            println!("Outer: {}, Inner: {}", outer_count, inner_count);
            inner_count += 1;

            if inner_count == 3 {
                break; // breaks only the inner loop
            }

            if outer_count == 2 {
                break 'outer; // breaks the outer loop
            }
        }

        outer_count += 1;
    }

    println!("All loops done!");
}

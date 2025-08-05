// In this example, you'll learn how to use the logical AND operator (`&&`) in Rust.
// This operator returns `true` only if **both** conditions are true.
// It's often used to combine boolean expressions in decision-making logic.

fn main() {
    let purchased_ticket = true;
    let plane_on_time = true;

    // The event can only happen if both the ticket is purchased AND the plane is on time
    let making_event = purchased_ticket && plane_on_time;
    let location = "Lagos";

    println!("It is {} that I will arrive {} as expected", making_event, location);
}

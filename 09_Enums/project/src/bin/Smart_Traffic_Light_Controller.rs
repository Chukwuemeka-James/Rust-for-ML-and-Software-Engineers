use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TrafficLight {
    fn next(&self) -> Self {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }

    fn duration(&self) -> Duration {
        match self {
            TrafficLight::Red => Duration::from_secs(5),
            TrafficLight::Green => Duration::from_secs(4),
            TrafficLight::Yellow => Duration::from_secs(2),
        }
    }

    fn display(&self) {
        match self {
            TrafficLight::Red => println!("🔴 Red Light - STOP"),
            TrafficLight::Green => println!("🟢 Green Light - GO"),
            TrafficLight::Yellow => println!("🟡 Yellow Light - SLOW DOWN"),
        }
    }
}

fn main() {
    let mut current_light = TrafficLight::Red;

    // Run for 10 cycles
    for cycle in 1..=10 {
        println!("\nCycle {}:", cycle);
        current_light.display();

        sleep(current_light.duration());
        current_light = current_light.next();
    }

    println!("\nSimulation ended after 10 cycles.");
}

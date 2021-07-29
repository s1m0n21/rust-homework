use std::time::Duration;

trait Timer {
    fn get_duration(&self) -> Duration;
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl Timer for TrafficLight {
    fn get_duration(&self) -> Duration {
        match self {
            TrafficLight::Red => Duration::from_secs(10),
            TrafficLight::Yellow => Duration::from_secs(5),
            TrafficLight::Green => Duration::from_secs(20),
        }
    }
}

fn main() {
    println!("Red = {:?}", TrafficLight::Red.get_duration());
    println!("Yellow = {:?}", TrafficLight::Yellow.get_duration());
    println!("Green = {:?}", TrafficLight::Green.get_duration());
}

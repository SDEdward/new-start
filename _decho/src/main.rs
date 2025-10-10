use rand::Rng; //use rand and io for usr input and random delay also thread and time for the delay
use std::thread;
use std::time::Duration;

fn main() {
    let duration = rand::rng().random_range(1..=10); //generate the delay

    println!("hi!");
    thread::sleep(Duration::from_secs(duration));
    println!("hi but {duration} secs later");
}

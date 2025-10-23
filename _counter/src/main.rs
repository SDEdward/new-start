use std::io;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("Enter countdown duration in seconds:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let secs: u64 = input.trim().parse().unwrap_or(10); // fallback to 10
    let wait = Duration::from_secs(secs);

    countdown(wait).await;
}

async fn countdown(duration: Duration) {
    let mut remaining = duration.as_secs();

    while remaining > 0 {
        println!("{remaining} seconds remaining...");
        sleep(Duration::from_secs(1)).await;
        remaining -= 1;
    }

    println!("Time's up!");
}

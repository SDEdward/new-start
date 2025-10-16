use std::io;
use std::time::Duration;
use tokio::time::sleep;
#[tokio::main]
async fn main() {
    println!("HEllo");
    wait().await;
    println!("Done waiting!");
}

async fn wait() {
    let mut input = String::new();
    println!("How much to wait in ms?");

    loop {
        input.clear(); // Clear previous input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let millis: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Come on, insert a number!");
                continue;
            }
        };

        let duration = Duration::from_millis(millis);
        println!("Waiting for {millis} ms...");
        sleep(duration).await;
        break;
    }
}

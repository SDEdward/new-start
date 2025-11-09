use rand::Rng;
use std::io;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut players: Vec<String> = Vec::new();
    println!("Enter player names (type 'done' to finish):");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let name = input.trim();
        if name.eq_ignore_ascii_case("done") {
            break;
        }

        players.push(name.to_string());
    }

    println!("Players: {}", players.join(", "));
    dares(players);
}

fn dares(players: Vec<String>) {
    let dares = [
        "Sing a song loudly",
        "Do 10 jumping jacks",
        "Tell a funny story",
        "Dance for 30 seconds",
        "Imitate a celebrity",
    ];

    let mut used = Vec::new();
    loop {
        let mut chosen = &players[rand::rng().random_range(0..players.len())];
        loop {
            if used.contains(chosen) {
                chosen = &players[rand::rng().random_range(0..players.len())];
                continue;
            } else {
                break;
            }
        }
        println!("Spinning...");
        println!("Landed on: {:?}", chosen);

        used.push(chosen.to_string());
        if used.len() == players.len() {
            used.clear();
        }

        sleep(Duration::from_secs(1));
    }
}

use rand::rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut select = String::new();
    println!("Select a game:");
    println!("1: Blackjack");
    println!("2: Slots");
    println!("3: Roulette (bad)");
    select.clear();
    io::stdin()
        .read_line(&mut select)
        .expect("Err: unable to read line; what");
    let select: u8 = match select.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number between 1 and 3.");
            return;
        }
    };
    match select {
        1 => {
            bj();
        }
        2 => {
            slot();
        }
        3 => {
            roulette();
        }
        _ => {
            println!("how the fuck do u fuck this up?");
        }
    }
}

fn bj() {
    let mut choice = String::new();

    loop {
        println!();
        println!("Welcome to blackjack!!!!");
        println!("Say 1 to play or 2 to go back");

        choice.clear();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter 1 or 2.");
                continue;
            }
        };

        match choice {
            1 => {
                println!();
                bjgame();
            }
            2 => {
                println!("Returning to main menu...\n");
                main();
            }
            _ => {
                println!("Invalid choice. Please enter 1 or 2.");
            }
        }
    }
}
fn bjgame() {
    let mut dealer: u8 = rand::rng().gen_range(2..=21);
    let mut player: u8 = rand::rng().gen_range(2..=21);
}
fn slot() {}

fn roulette() {}

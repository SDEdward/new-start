use rand::Rng;
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
    let mut dealer: u8 = rand::rng().random_range(2..=21);
    let mut player: u8 = rand::rng().random_range(2..=21);
    dealer -= 1;
    let dealer_deduction: u8 = rand::rng().random_range(1..=dealer);
    dealer += 1;
    let fake_dealer: u8 = dealer - dealer_deduction;

    println!("Dealer has {fake_dealer} showing (1 card hidden)");
    println!("You have {player}\n");
    println!("Do you:");
    println!("1: Stand");
    println!("2: Hit");

    loop {
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Idk how u made this panic man wtf!?!??");

        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num, // remember that num here can be anythiung ( its a temporary variable )
            Err(_) => {
                println!("NUMBER MASON NUMBER!!!!! (less than 256 btw)");
                continue;
            }
        };

        match choice {
            1 => {
                println!("You stand with {player}. Dealer reveals hidden card...");
                println!("Dealer's total is {dealer}.");
                // todo Add win/loss logic here
            }
            2 => {
                let hit_card: u8 = rand::rng().random_range(1..=11);
                player += hit_card;
                println!("You drew a {hit_card}. Your total is now {player}.");
                if player > 21 {
                    println!("You bust! Dealer wins.");
                    // todo enter to go back to the bj main menu
                } else {
                    // todo same as the comment above
                }
            }
            _ => {
                println!("Invalid choice. Try 1 or 2.");
                continue;
            }
        }
    }
}

fn slot() {}

fn roulette() {}

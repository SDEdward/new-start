use rand::Rng;
// use std::cmp::Ordering;
use std::io;

fn main() {
    let mut select = String::new();
    println!("Select a game:");
    println!("1: Blackjack");
    println!("2: Slots (bad)");
    println!("3: Roulette (bad)");
    println!("4: Exit");
    select.clear();
    io::stdin()
        .read_line(&mut select)
        .expect("Err: unable to read line; what");
    let select: u8 = match select.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number between 1 and 4.");
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
        4 => {
            // exits cuz nothing happens
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
                println!("\nYou stand with {player}. Dealer reveals hidden card...");
                println!("Dealer's new total is {dealer}. ({dealer_deduction} was hidden)\n");
                if dealer > 21 {
                    println!("Dealer busted! You win\n");
                    enter_to_go_back_bj();
                } else if dealer > player {
                    println!("Dealer wins!\n");
                    enter_to_go_back_bj();
                } else if player == dealer {
                    println!("Push! Noone loses or wins!\n");
                    enter_to_go_back_bj();
                } else if player > dealer {
                    loop {
                        // println!("The dealer shows the hidden card and then hits...");
                        // println!("He was hiding {dealer_deduction} ({dealer} in total)\n");
                        let hit_dealer: u8 = rand::rng().random_range(1..=11);
                        dealer += hit_dealer;
                        println!("The dealer draws a {hit_dealer}, he now has {dealer}");

                        if dealer > 21 {
                            println!("Dealer busts! You win!\n");
                            enter_to_go_back_bj();
                        } else if dealer > player {
                            println!("Dealer gets more then you! You lose!\n");
                            enter_to_go_back_bj();
                        } else if dealer == player {
                            println!("Push! Noone loses or wins!");
                            enter_to_go_back_bj();
                        } else {
                            let mut dummy = String::new();
                            println!("Dealer grabs another card... (Enter to continue)\n");
                            io::stdin().read_line(&mut dummy).expect("What how?");
                            continue;
                        }
                    }
                }
            }

            2 => {
                let hit_card: u8 = rand::rng().random_range(1..=11);
                player += hit_card;
                println!("\nYou drew a {hit_card}. Your total is now {player}.");
                if player > 21 {
                    println!("You bust! Dealer wins.\n");
                    enter_to_go_back_bj();
                } else {
                    println!("You are still in the game! Do you:");
                    println!("1: Stand");
                    println!("2: Hit");
                }
            }

            _ => {
                println!("Invalid choice. Try 1 or 2.");
                continue;
            }
        }
    }
}
fn enter_to_go_back_bj() {
    println!("Press enter to go back...");
    let mut dummy = String::new();
    io::stdin()
        .read_line(&mut dummy)
        .expect("Dude come on just press enter");
    bj();
}

fn slot() {}

fn roulette() {}

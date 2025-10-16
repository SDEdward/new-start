/* unfinished yet... actually if i get lazy it is lol */

use rand::Rng;
use std::io;
use std::thread::sleep; // sadly tokio wants async and thats overcomplycated so were using std sleep
use std::time::Duration;

fn main() {
    let mut select = String::new();

    println!(r#"HELLO PRESS ANYTHING TO PLAY!!!!!!!!! (or "q" to quit)"#); // r#..# here makes me able to use "" and () in the string

    // read_line returns a Result; if it fails, this crashes with a message (which is... intense)
    io::stdin().read_line(&mut select).expect("kill yourself");

    // the !matches! is a cleaner way to check if input is "q" ( not matches (function) )
    if !matches!(select.trim(), "q") {
        pretext();
    }
}

fn pretext() {
    let mut select = String::new();
    println!("\nSelect a game:");
    println!("1: Blackjack");
    println!("2: Slots (bad)");
    println!("3: Roulette (bad)");
    println!("4: Quit");

    loop {
        select.clear(); // clean the string cuz it itz loopin

        io::stdin()
            .read_line(&mut select)
            .expect("Err: unable to read line; what");

        // try to turn input into a number (u8)
        let select: u8 = match select.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("1, 2 or 3. ONE, TWO OR THREE");
                continue;
            }
        };

        match select {
            1 => {
                bj(); // blackjack
            }
            2 => {
                slot(); // slots
            }
            3 => {
                roulette(); // roulette
            }
            4 => {
                break; // quit
            }
            _ => {
                // if it's not 1–4, roast and retry
                println!("how the fuck do u fuck this up?");
                continue;
            }
        }
    }
}

// ? remake this using objects and shit??\
fn bj() {
    let mut choice = String::new();

    loop {
        println!();
        println!("Welcome to blackjack!!!!");
        println!("Say 1 to play or 2 to go back");

        choice.clear();

        io::stdin().read_line(&mut choice).expect("come on");

        // try to turn input into a number (u8)
        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("1 or 2.");
                continue;
            }
        };

        match choice {
            1 => {
                println!();
                bjgame(); // start blackjack game
            }
            2 => {
                println!("Returning to main menu...\n");
                pretext(); // go back to game select
            }
            _ => {
                println!("grrrr >:c");
            }
        }
    }
}

fn bjgame() {
    // dealer and player both get random starting cards between 2 and 21
    let mut dealer: u8 = rand::rng().random_range(2..=21);
    let mut player: u8 = rand::rng().random_range(2..=21);

    dealer -= 1; // temporarily lower dealer's total to hide a card
    let dealer_deduction: u8 = rand::rng().random_range(1..=dealer); // this is the hidden card
    dealer += 1; // restore dealer's full total
    let fake_dealer: u8 = dealer - dealer_deduction; // what the player sees

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

        // try to turn input into a number (u8)
        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("NUMBER MASON NUMBER!!!!! (less than 256 btw)");
                continue;
            }
        };

        match choice {
            1 => {
                // player stands, dealer reveals hidden card
                println!("\nYou stand with {player}. Dealer reveals hidden card...");
                println!("Dealer's new total is {dealer}. ({dealer_deduction} was hidden)\n");

                // check who wins or if dealer busts
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
                    // dealer hits until outcome is decided
                    loop {
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
                            // dealer still behind, draw again
                            let mut dummy = String::new();
                            println!("Dealer grabs another card... (Enter to continue)\n");
                            io::stdin().read_line(&mut dummy).expect("What how?");
                            continue;
                        }
                    }
                }
            }

            2 => {
                // player hits
                let hit_card: u8 = rand::rng().random_range(1..=11);
                player += hit_card;
                println!("\nYou drew a {hit_card}. Your total is now {player}.");

                // check if player busts
                if player > 21 {
                    println!("You bust! Dealer wins.\n");
                    enter_to_go_back_bj();
                } else {
                    // still alive, ask again
                    println!("You are still in the game! Do you:");
                    println!("1: Stand");
                    println!("2: Hit");
                }
            }

            _ => {
                // anything other than 1 or 2
                println!("dumbass");
                continue;
            }
        }
    }
}

fn enter_to_go_back_bj() {
    println!("Press enter to go back...");
    let mut dummy = String::new();

    // waits for user to press enter, then sends them back to blackjack menu
    io::stdin()
        .read_line(&mut dummy)
        .expect("Dude come on just press enter");

    bj();
}

// todo: finish
fn slot() {
    let mut choice = String::new();

    loop {
        println!();
        println!("Welcome to slots!1!11!!");
        println!("Say anything to play or q to go back");

        choice.clear();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "q" => {
                println!("Returning to main menu...\n");
                pretext();
                break;
            }
            _ => {
                slotsgame();
            }
        }
    }
}

fn slotsgame() {
    let icons = ["🍒", "🍋", "🔔", "💎", "🍉", "🎰", "⭐", "🃏"];
    let spin_time = Duration::from_millis(150);
    let mut rng = rand::rng();

    println!("The slots spin...\n");

    let mut slot1 = "";
    let mut slot2 = "";
    let mut slot3 = "";

    // Simulate spinning all three slots together
    for _ in 0..15 {
        slot1 = icons[rng.random_range(0..icons.len())];
        slot2 = icons[rng.random_range(0..icons.len())];
        slot3 = icons[rng.random_range(0..icons.len())];

        print!("\r[{slot1}] [{slot2}] [{slot3}]");
        io::Write::flush(&mut io::stdout()).expect("Flush failed");
        sleep(spin_time);
    }

    println!(); // Move to next line after final spin

    // Result evaluation
    if slot1 == slot2 && slot2 == slot3 {
        println!("You win big!!!! wow!!");
    } else if slot1 == slot2 || slot2 == slot3 || slot1 == slot3 {
        println!("You win good enough ig...");
    } else {
        println!("You lost lol");
    }

    //enter_to_continue_slots();
}

/* fn enter_to_continue_slots() {
    let mut dummy = String::new();
    println!("\nPress enter to continue...");
    io::stdin().read_line(&mut dummy).expect("what?!?!?");
    slot();
} */

// * idk if u can make this better but u should try
fn roulette() {
    let mut choice = String::new();

    loop {
        println!();
        println!("Welcome to roulette!!!!!!!");
        println!("Say 1 to play or 2 to go back");

        choice.clear();
        io::stdin().read_line(&mut choice).expect("1 or 2.");

        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("dumbass ts not 1 or 2");
                continue;
            }
        };

        match choice {
            1 => {
                println!();
                roulettegame();
            }
            2 => {
                println!("Returning to main menu...\n");
                pretext();
            }
            _ => {
                println!("1 or 2 dumbass");
            }
        }
    }
}

fn roulettegame() {
    let mut bet = String::new();

    println!("Choose your bet by entering the number:\n");

    println!("Bet 1: Single number");
    println!("Bet 2: Red");
    println!("Bet 3: Black");
    println!("Bet 4: Even");
    println!("Bet 5: Odd");
    println!("Bet 6: Zero");
    println!("Bet 7: 1-18");
    println!("Bet 8: 19-36");
    println!("Bet 9: 1-12");
    println!("Bet 10: 13-24");
    println!("Bet 11: 25-36");
    println!("Bet 12: Column 1");
    println!("Bet 13: Column 2");
    println!("Bet 14: Column 3");

    io::stdin().read_line(&mut bet).expect("say that again?");

    let choice: u8 = match bet.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("im bored of writing errors");
            return;
        }
    };
    bet.clear();
    println!();
    loop {
        match choice {
            1 => {
                let mut numb = String::new();
                println!("\nYou chose: Single Number. Enter a number between 1 and 36:");
                io::stdin().read_line(&mut numb).expect("what did i say");
                let numb: u8 = match numb.trim().parse() {
                    Ok(num) if (1..36).contains(&num) => {
                        println!("Bet placed on: {num}");
                        num
                    }
                    _ => {
                        println!("dude.");
                        continue;
                    }
                };
                spin_roulette(choice, numb);
                break;
            }
            2 => {
                println!("Bet placed on: Red");
                spin_roulette(choice, 0);
                break;
            }
            3 => {
                println!("Bet placed on: Black");
                spin_roulette(choice, 0);
                break;
            }
            4 => {
                println!("Bet placed on: Even");
                spin_roulette(choice, 0);
                break;
            }
            5 => {
                println!("Bet placed on: Odd");
                spin_roulette(choice, 0);
                break;
            }
            6 => {
                println!("Bet placed on: Zero");
                spin_roulette(choice, 0);
                break;
            }
            7 => {
                println!("Bet placed on: 1-18");
                spin_roulette(choice, 0);
                break;
            }
            8 => {
                println!("Bet placed on: 19-36");
                spin_roulette(choice, 0);
                break;
            }
            9 => {
                println!("Bet placed on: 1-12");
                spin_roulette(choice, 0);
                break;
            }
            10 => {
                println!("Bet placed on: 13-24");
                spin_roulette(choice, 0);
                break;
            }
            11 => {
                println!("Bet placed on: 25-36");
                spin_roulette(choice, 0);
                break;
            }
            12 => {
                println!("Bet placed on: 1st Column");
                spin_roulette(choice, 0);
                break;
            }
            13 => {
                println!("Bet placed on: 2nd Column");
                spin_roulette(choice, 0);
                break;
            }
            14 => {
                println!("Bet placed on: 3rd Column");
                spin_roulette(choice, 0);
                break;
            }
            _ => {
                println!("not a specified number");
            }
        }
    }
}

// todo remake the comments (ingame) in this part so that theyre randomised!!!
fn spin_roulette(bet: u8, numb: u8) {
    let mut next = String::new();
    let red: [u8; 18] = [
        1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36,
    ];
    let black: [u8; 18] = [
        2, 4, 6, 8, 10, 11, 13, 15, 17, 20, 22, 24, 26, 28, 29, 31, 33, 35,
    ];
    let column1: [u8; 12] = [1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34];
    let column2: [u8; 12] = [2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32, 35];
    let column3: [u8; 12] = [3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36];
    let spinned: u8 = rand::rng().random_range(0..=36);
    println!("The roulette spins... (enter to continue)");
    io::stdin().read_line(&mut next).expect("FUCK YOU");
    if red.contains(&spinned) {
        println!("It lands on... {spinned}, red!");
    } else if black.contains(&spinned) {
        println!("It lands on... {spinned}, black!");
    } else {
        println!("It lands on... {spinned}!");
    }

    match bet {
        1 => {
            if numb == spinned && (1..=36).contains(&numb) {
                println!("YOU WON WITH A SINGLE NUMBER. WHAT?!?!?");
                enter_to_continue_roulette();
            } else {
                println!("booohooooo you lost dumbass");
                enter_to_continue_roulette();
            }
        }
        2 => {
            if red.contains(&spinned) {
                println!("put it all on red and won it all");
                enter_to_continue_roulette();
            } else {
                println!("hehe no money for you");
                enter_to_continue_roulette();
            }
        }
        3 => {
            if black.contains(&spinned) {
                println!("put it all on black and won!!!");
                enter_to_continue_roulette();
            } else {
                println!("lost all of the money!!!!");
                enter_to_continue_roulette();
            }
        }
        4 => {
            if spinned % 2 == 0 {
                println!("good job but even isnt that spectacular");
                enter_to_continue_roulette();
            } else {
                println!("u lost on even; come on it was 50/50!");
                enter_to_continue_roulette();
            }
        }
        5 => {
            if spinned % 2 != 0 {
                println!("hip hip horray you won odd!!");
                enter_to_continue_roulette();
            } else {
                println!("you lost in odd. fuck theese loss messages");
                enter_to_continue_roulette();
            }
        }
        6 => {
            if spinned == 0 {
                println!("YOU WON WITH 0???!?!?!? STOP CHEATING!??!?!?!?");
                enter_to_continue_roulette();
            } else {
                println!("did u actually expect to win?");
                enter_to_continue_roulette();
            }
        }
        7 => {
            if (1..=18).contains(&spinned) {
                println!("good job ill write better texts tommorow i swear");
                enter_to_continue_roulette();
            } else {
                println!("you lost.");
                enter_to_continue_roulette();
            }
        }
        8 => {
            if (19..=36).contains(&spinned) {
                println!("good fucking job; you won!!");
                enter_to_continue_roulette();
            } else {
                println!("ok enough losing");
                enter_to_continue_roulette();
            }
        }
        9 => {
            if (1..=12).contains(&spinned) {
                println!("u are basic but u won!!");
                enter_to_continue_roulette();
            } else {
                println!("losses, losses, people make losees");
                enter_to_continue_roulette();
            }
        }
        10 => {
            if (13..=24).contains(&spinned) {
                println!("idek what to tell u anymore but u won!");
                enter_to_continue_roulette();
            } else {
                println!("YOU WONN!!!!! jk u lost lol");
                enter_to_continue_roulette();
            }
        }
        11 => {
            if (25..=36).contains(&spinned) {
                println!("you lost... JK YOU WON!!!!");
                enter_to_continue_roulette();
            } else {
                println!("time to go to the amanet to get more gambling money!!! (you lost)");
                enter_to_continue_roulette();
            }
        }
        12 => {
            if column1.contains(&spinned) {
                println!("omg column 1 soooooo interesting... you did win tho!");
                enter_to_continue_roulette();
            } else {
                println!("time to flip the table ig... (u lost)");
                enter_to_continue_roulette();
            }
        }
        13 => {
            if column2.contains(&spinned) {
                println!("its 2 am when im writing this and im sleep deprived but you won!!!");
                enter_to_continue_roulette();
            } else {
                println!("FUCK THIS RIGGED FUCK FUCK SHIT GAME!!!!!!! (u lost)");
                enter_to_continue_roulette();
            }
        }
        14 => {
            if column3.contains(&spinned) {
                println!("finally ican get some sleep but good job u won!");
                enter_to_continue_roulette();
            } else {
                println!("good job... you lost!");
                enter_to_continue_roulette();
            }
        }
        _ => println!(
            "if im pretty sure this is unreachable cuz theres 2 chacks adn if the first one fails this one should to but you do you ig?",
        ),
    }
}

fn enter_to_continue_roulette() {
    let mut dummy = String::new();
    println!("\nPress enter to continue...");
    io::stdin().read_line(&mut dummy).expect("what?!?!?");
    roulette();
}

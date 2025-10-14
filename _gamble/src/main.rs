/* unfinished yet, also dont forget to fix the clippy warning */

use rand::Rng;
use std::io;

fn main() {
    let mut select = String::new();
    println!(r#"HELLO PRESS ANYTHING TO PLAY!!!!!!!!! (or "q" to quit)"#); // r# at end and start make it a raw sting and enable ""
    io::stdin().read_line(&mut select).expect("kill yourself");
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
        select.clear();
        io::stdin()
            .read_line(&mut select)
            .expect("Err: unable to read line; what");
        let select: u8 = match select.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("1, 2 or 3. ONE, TWO OR THREE");
                continue;
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
                break;
            }
            _ => {
                println!("how the fuck do u fuck this up?");
                continue;
            }
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
                pretext();
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

fn slot() {
    println!("sorry not finished actuyally not even started rn");
    pretext();
}

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
                        return;
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

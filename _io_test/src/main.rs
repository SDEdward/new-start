use std::io;

fn main() {
    let mut action = String::new();
    let mut list = String::new();
    let mut mistakes = 0;
    let mut cleared = false;

    loop {
        println!("Select an action:");
        println!("1: View list");
        println!("2: Edit list");
        println!("3: Exit \n");

        action.clear(); // idrk what this is chatgpt added it and it works fine
        io::stdin().read_line(&mut action).expect("ascii please \n");
        let action = action.trim(); // trim action

        match action {
            //see if action is any of theese values
            "1" => {
                if !list.trim().is_empty() {
                    println!("\nYour list:\n{list}\n"); //if trimmed list is not empty then print the list
                } else {
                    println!("\nYour list is empty\n"); // if it is empty then say that it is
                }
                wait_for_enter();
            }
            "2" => {
                println!("\nEnter new list item (or type 'back' to return or 'cls' to clear):");
                let mut item = String::new();
                io::stdin()
                    .read_line(&mut item)
                    .expect("Failed to read line\n");
                let item = item.trim(); //makes, reads and trims the new item

                if item == "back" {
                    println!();
                    continue; // if the user types back then continue (programming teaches you to not trust language)
                } else if item == "cls" {
                    list.clear();
                    cleared = true;
                } else if !list.is_empty() {
                    list.push('\n'); // if the list isnt empty then adds a newline (notice its a push)
                }
                if !cleared {
                    list.push_str(item); // here then it adds the item (notice its a push_str)
                    println!("\nItem added to the list.\n");
                } else {
                    cleared = false;
                    println!();
                    continue;
                }
            }

            "3" => break, // if its 3 then it exits
            _ => {
                if mistakes == 0 {
                    println!("\nyou fucking retard i hope this was a mistake\n");
                    mistakes += 1;
                    wait_for_enter();
                } else if mistakes == 1 {
                    println!("\nagain? what, are you in a fucking hurry?\n");
                    mistakes += 1;
                    wait_for_enter();
                } else if mistakes == 2 {
                    println!("\n3 mistakes, u are a fucking mistake like the ones that u make\n");
                    mistakes += 1;
                    wait_for_enter();
                } else if (3..=8).contains(&mistakes) {
                    mistakes += 1;
                    println!("\nfuck you, {mistakes} mistakes\n");
                    wait_for_enter();
                } else if 9 == mistakes || mistakes < 9 {
                    println!(
                        "\n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n \n STOP FUCKING SPAMMING.\n"
                    ); // "clears screen" and yells (this is easier the ncatually clearing the screen cuz os's decided to not be the same)
                    break; // the program gets annoyed and kills itself lol
                }
            } // if the user says anything else then they get flamed the worse the more mistakes thay make
        }
    }
}

fn wait_for_enter() {
    println!("Press enter to continue...");
    let mut dummy = String::new();
    io::stdin()
        .read_line(&mut dummy)
        .expect("Failed to read line somehow, just press enter istg"); //all this does here is just make a "enter to continue"
}

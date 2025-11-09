mod tea;
use std::io;
fn main() {
    //inputting::
    let mut a = String::new(); // !! AT FIRST, WHEN INPUTTING TO A VARIABLE, WE GOTTA MAKE IT A STRING TO STORE THE INPUT !!
    //let mut a: u8 = a.trim().parse().expect("Please type a valid number!"); //this will crash if not a number

    let a: u8 = loop {
        a.clear(); // clear previous input
        println!("Please enter a number:");

        io::stdin().read_line(&mut a).expect("Failed to read line");

        match a.trim().parse() {
            Ok(num) => break num, // return the parsed number
            Err(_) => {
                println!("That wasn’t a valid number!");
                continue;
            }
        };
    };
    println!("{a}");
}

// * he wants an if, for, while, loop, match

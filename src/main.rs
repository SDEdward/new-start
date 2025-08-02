use std::cmp::Ordering; //makes +, -, and = available for comparison
use std::io; //input/output library

use rand::Rng; //random number generation

fn main() {
    print!("\n"); //prints a newline to the console
    println!("Guess the number! (quit by guessing correctly or by Ctrl+C)"); //prints to the console with a newline
    println!("Please enter your guess between 1 and 100."); //prints to the console with a newline
    print!("\n"); //prints a newline to the console

    let secret_number = //makes a variable called secret_number
     rand::thread_rng() //calls the random number generator
        .gen_range(1..=100); //makes rand::thread_rng() generate a random number between 1 and 100

    loop {
        println!("Please input your guess."); // prints to the console with a newline

        let mut guess = String::new(); //makes guess mutable and makes it a new String

        io::stdin() //calls the standard input function
            .read_line(&mut guess) //reads guess with a mutable reference
            .expect("Failed to read line"); //if it fails to read the line, it will panic with the message "Failed to read line" (expect gets either Ok or Err, if it gets Err it will panic with the message, otherwise it will continue)

        let guess: u32 = //makes guess a u32 (unsigned 32-bit integer)
         match guess.trim() //trims guess
         .parse() { //tries to parse guess
            Ok(num) => num, //if it is successful, it returns num
            Err(_) => continue, //if it fails, it continues the loop (ignores the rest of the code in the loop and starts over)
        };

        print!("\n"); //prints a newline to the console
        println!("You guessed: {guess}"); // prints the guess to the console

        match guess.cmp(&secret_number) {
            //compares guess and secret_number (cmp is a method that compares two values)
            Ordering::Less => {
                println!("Too small!");
                print!("\n");
            } //if guess is less than secret_number then prints "Too small!"
            Ordering::Greater => {
                println!("Too big!");
                print!("\n");
            } //if guess is greater than secret_number then prints "Too big!"
            Ordering::Equal => {
                println!("You win!");
                break;
            } //if guess is equal to secret_number then prints "You win!" also breaks the loop
        }
    }
    print!("\n"); //prints a newline to the console
    println!("The secret number was: {secret_number}"); // prints the secret number to the console in the end
}

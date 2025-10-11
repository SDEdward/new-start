/* say either "Hello world!" or "Bid farewell cruel world!" */

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng(); // Create a random number generator
    let two = "world!";
    let three = rng.gen_range(1..=2); // Generate a random number in the range 1 to 2

    let one = if three == 1 {
        "Hello"
    } else {
        "Bid farewell cruel"
    };

    println!("{one} {two}");
}

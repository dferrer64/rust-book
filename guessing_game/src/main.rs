// Source: The Rust Programming Language
// Notes: I took this program from The Book and I added my notes as comments

use rand::Rng; // needed to generate a random number
use std::cmp::Ordering; // needed for determining if a guess was too low, right, or too high
use std::io; // needed for reading from stdin

fn main() {
    // print a prompt
    println!("Guess the number!");

    // generate a random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // loop until the right answer is given
    loop {
        // print a prompt
        println!("Please input your guess.");

        // init variable for reading input
        let mut guess = String::new();

        // read input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // handle non-number inputs
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // print input
        println!("You guessed: {guess}");

        // print result
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
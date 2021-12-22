use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    // Create a secret number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Please input your guess.");

        // Save guess into a variable.
        let mut guess = String::new();

        // "read_line" takes input from the user and appends it to the reference of "guess" (which is made mutable).
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert guess into a number, if it fails, print a message and continue (the message prints that we should use only numbers, but we're actually catching all errors with "_"), if it works, return the number.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please use only numbers.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // Compare the guess to the reference of secret number.
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

// This brings the "rand" crate that we put in "Cargo.toml".
use rand::Rng;
// This brings the "io" library from the standard library (std).
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    // This generates a random number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess.");

        // Here we're storing the user input:
        // "let" is used to create a variable, and "mut" makes the variable mutable (since variables are immutable by default in Rust)
        // "String::new" is an instance of a string, the "String" is a string type and "new" is an associated function.
        let mut guess = String::new();

        // Here we call the "stdin" function from the "io" module that we imported before.
        io::stdin()
            // Call read_line method, which takes input from the user and appends it to the reference string (which has to be changed to be mutable), but also returns a value (io::Result), which can either be "Ok" or "Err".
            .read_line(&mut guess)
            // If this instance of "io:Result" is an "Err", "expect" will cause the program to crash and show the message.
            // If it returns an "Ok" value, expect will take the return value that "Ok" is holding and return just that value to you so you can use it. In this case, that value is the number of bytes in what the user entered into standard input.
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}

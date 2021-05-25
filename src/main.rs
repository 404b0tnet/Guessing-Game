use std::{io, u32};            // allows input and output
use rand::Rng;          // allows random number generation
use std::cmp::Ordering; // allows ordering and comparing

fn main() {
    // output to terminal
    println!("Guess the number!");

    // random number between 1 - 100
    // alternatively could have use .gen_range(1..=100)
    let secret_number = rand::thread_rng().gen_range(1..101);

    // outputs the secret number
    println!("The secret number is {}", secret_number);

    loop {
        // output to terminal
        println!("Please input your guess.");

        // create variable to store input
        let mut guess = String::new();

        // read line from terminal and store into "guess"
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        
        // convert guess from String to 32-bit number
        // trim removes white space and newlines
        // parse converts string to number
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number.");
        
        // compares guess to secret number
        // need to compare same types
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        
        // outputs to terminal what the user inputted
        println!("You guessed: {}", guess);

    }   // end loop

}   // end main

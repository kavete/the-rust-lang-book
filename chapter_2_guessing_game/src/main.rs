use rand::random_range;

use std::{cmp::Ordering, io};

fn main() {
    println!("Number guessing game");

    // Generate a random number between 0 and 100(inclusive)
    let random_number: u8 = random_range(0..=100);
    loop {
        println!("Guess a number between 1 and 100");

        // Create the variable to hold the users guess
        let mut guess = String::new();

        // Read the line panicking if unable to read_line
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read the line");

        // Remove the new line character and any other unnecessary spaces
        // Parse the string to u8 (throws an error if unable to)
        // Continue the loop if we could not parse into a number or into a u8 in this case
        // Inputing values greater than 255 will also produce an Err Result, we therefore continue
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess > 100 {
            continue;
        }

        println!("You guessed: {}", guess);

        match guess.cmp(&random_number) {
            Ordering::Less => {
                println!("Too small");
            }
            Ordering::Greater => {
                println!("Too big");
            }

            Ordering::Equal => {
                println!("Got it");
                break;
            }
        }
    }
}

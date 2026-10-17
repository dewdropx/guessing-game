use std::io; // standard io crate
use std::io::Write; // for flush method
use std::cmp::Ordering; // for comparing values
use rand::prelude::*; // for random number generation

fn main() {
    println!("GUESS THE NUMBER GAME!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        print!("guess a number bw 1 and 100: ");
        io::stdout().flush().expect("failed flush");

        let mut guess = String::new();

        // read input from user
        io::stdin().read_line(&mut guess).expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too large"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
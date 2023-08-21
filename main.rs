use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Welcome to number guessing game");

    let actual_number = rand::thread_rng().gen_range(1..=100); // Any thing from 1 -> 100 (start..=end)

    println!("Your task is to guess a number which is randomly generated using rand crate\n ");
    
    loop {
        println!("Input your guess: ");
    
        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Unable to read input!");
    
        // let guess: u32 = guess.trim().parse().expect("Input Mismatch!");
        
        let guess: u32 = match guess.trim().parse() {       // Program doesn't panics when Input Mismatch
            Ok(num) => num,     // Ignores anything which is not a number 
            Err(_) => continue,
        };
        
        println!("Your Guess: {guess}");
    
        match guess.cmp(&actual_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Guessed the number correctly!");
                break;
            }
        }
    }
}
